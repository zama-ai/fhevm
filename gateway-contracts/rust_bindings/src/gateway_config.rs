///Module containing a contract's types and functions.
/**

```solidity
library IGatewayConfig {
    struct Thresholds { uint256 mpcThreshold; uint256 publicDecryptionThreshold; uint256 userDecryptionThreshold; uint256 kmsGenThreshold; uint256 coprocessorThreshold; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IGatewayConfig {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Thresholds { uint256 mpcThreshold; uint256 publicDecryptionThreshold; uint256 userDecryptionThreshold; uint256 kmsGenThreshold; uint256 coprocessorThreshold; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Thresholds {
        #[allow(missing_docs)]
        pub mpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub publicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub coprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<Thresholds> for UnderlyingRustTuple<'_> {
            fn from(value: Thresholds) -> Self {
                (
                    value.mpcThreshold,
                    value.publicDecryptionThreshold,
                    value.userDecryptionThreshold,
                    value.kmsGenThreshold,
                    value.coprocessorThreshold,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Thresholds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    mpcThreshold: tuple.0,
                    publicDecryptionThreshold: tuple.1,
                    userDecryptionThreshold: tuple.2,
                    kmsGenThreshold: tuple.3,
                    coprocessorThreshold: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Thresholds {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Thresholds {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mpcThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.publicDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.userDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsGenThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessorThreshold),
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
        impl alloy_sol_types::SolType for Thresholds {
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
        impl alloy_sol_types::SolStruct for Thresholds {
            const NAME: &'static str = "Thresholds";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Thresholds(uint256 mpcThreshold,uint256 publicDecryptionThreshold,uint256 userDecryptionThreshold,uint256 kmsGenThreshold,uint256 coprocessorThreshold)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.mpcThreshold)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.publicDecryptionThreshold,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.userDecryptionThreshold,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.kmsGenThreshold,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.coprocessorThreshold,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Thresholds {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.mpcThreshold,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.publicDecryptionThreshold,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.userDecryptionThreshold,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.kmsGenThreshold,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.coprocessorThreshold,
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
                    &rust.mpcThreshold,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.publicDecryptionThreshold,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.userDecryptionThreshold,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.kmsGenThreshold,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.coprocessorThreshold,
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
    /**Creates a new wrapper around an on-chain [`IGatewayConfig`](self) contract instance.

See the [wrapper's documentation](`IGatewayConfigInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IGatewayConfigInstance<P, N> {
        IGatewayConfigInstance::<P, N>::new(address, provider)
    }
    /**A [`IGatewayConfig`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IGatewayConfig`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IGatewayConfigInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IGatewayConfigInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IGatewayConfigInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IGatewayConfigInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IGatewayConfig`](self) contract instance.

See the [wrapper's documentation](`IGatewayConfigInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IGatewayConfigInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IGatewayConfigInstance<P, N> {
            IGatewayConfigInstance {
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
    > IGatewayConfigInstance<P, N> {
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
    > IGatewayConfigInstance<P, N> {
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
library IGatewayConfig {
    struct Thresholds {
        uint256 mpcThreshold;
        uint256 publicDecryptionThreshold;
        uint256 userDecryptionThreshold;
        uint256 kmsGenThreshold;
        uint256 coprocessorThreshold;
    }
}

interface GatewayConfig {
    struct Coprocessor {
        address txSenderAddress;
        address signerAddress;
        string s3BucketUrl;
    }
    struct Custodian {
        address txSenderAddress;
        address signerAddress;
        bytes encryptionKey;
    }
    struct HostChain {
        uint256 chainId;
        address fhevmExecutorAddress;
        address aclAddress;
        string name;
        string website;
    }
    struct KmsNode {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
    }
    struct ProtocolMetadata {
        string name;
        string website;
    }

    error AddressEmptyCode(address target);
    error ChainIdNotUint64(uint256 chainId);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyCoprocessors();
    error EmptyCustodians();
    error EmptyKmsNodes();
    error FailedCall();
    error HostChainAlreadyRegistered(uint256 chainId);
    error InvalidHighCoprocessorThreshold(uint256 coprocessorThreshold, uint256 nCoprocessors);
    error InvalidHighKmsGenThreshold(uint256 kmsGenThreshold, uint256 nKmsNodes);
    error InvalidHighMpcThreshold(uint256 mpcThreshold, uint256 nKmsNodes);
    error InvalidHighPublicDecryptionThreshold(uint256 publicDecryptionThreshold, uint256 nKmsNodes);
    error InvalidHighUserDecryptionThreshold(uint256 userDecryptionThreshold, uint256 nKmsNodes);
    error InvalidInitialization();
    error InvalidNullChainId();
    error InvalidNullCoprocessorThreshold();
    error InvalidNullKmsGenThreshold();
    error InvalidNullPublicDecryptionThreshold();
    error InvalidNullUserDecryptionThreshold();
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotPauser(address account);
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event AddHostChain(HostChain hostChain);
    event InitializeGatewayConfig(ProtocolMetadata metadata, IGatewayConfig.Thresholds thresholds, KmsNode[] kmsNodes, Coprocessor[] coprocessors, Custodian[] custodians);
    event Initialized(uint64 version);
    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PauseAllGatewayContracts();
    event ReinitializeGatewayConfigV3(KmsNode[] newKmsNodes);
    event UnpauseAllGatewayContracts();
    event UpdateCoprocessorThreshold(uint256 newCoprocessorThreshold);
    event UpdateCoprocessors(Coprocessor[] newCoprocessors, uint256 newCoprocessorThreshold);
    event UpdateCustodians(Custodian[] newCustodians);
    event UpdateKmsGenThreshold(uint256 newKmsGenThreshold);
    event UpdateKmsNodes(KmsNode[] newKmsNodes, uint256 newMpcThreshold, uint256 newPublicDecryptionThreshold, uint256 newUserDecryptionThreshold, uint256 newKmsGenThreshold);
    event UpdateMpcThreshold(uint256 newMpcThreshold);
    event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);
    event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function acceptOwnership() external;
    function addHostChain(HostChain memory hostChain) external;
    function getCoprocessor(address coprocessorTxSenderAddress) external view returns (Coprocessor memory);
    function getCoprocessorMajorityThreshold() external view returns (uint256);
    function getCoprocessorSigners() external view returns (address[] memory);
    function getCoprocessorTxSenders() external view returns (address[] memory);
    function getCustodian(address custodianTxSenderAddress) external view returns (Custodian memory);
    function getCustodianSigners() external view returns (address[] memory);
    function getCustodianTxSenders() external view returns (address[] memory);
    function getHostChain(uint256 index) external view returns (HostChain memory);
    function getHostChains() external view returns (HostChain[] memory);
    function getKmsGenThreshold() external view returns (uint256);
    function getKmsNode(address kmsTxSenderAddress) external view returns (KmsNode memory);
    function getKmsSigners() external view returns (address[] memory);
    function getKmsTxSenders() external view returns (address[] memory);
    function getMpcThreshold() external view returns (uint256);
    function getProtocolMetadata() external view returns (ProtocolMetadata memory);
    function getPublicDecryptionThreshold() external view returns (uint256);
    function getUserDecryptionThreshold() external view returns (uint256);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy(ProtocolMetadata memory initialMetadata, IGatewayConfig.Thresholds memory initialThresholds, KmsNode[] memory initialKmsNodes, Coprocessor[] memory initialCoprocessors, Custodian[] memory initialCustodians) external;
    function isCoprocessorSigner(address signerAddress) external view returns (bool);
    function isCoprocessorTxSender(address txSenderAddress) external view returns (bool);
    function isCustodianSigner(address signerAddress) external view returns (bool);
    function isCustodianTxSender(address txSenderAddress) external view returns (bool);
    function isHostChainRegistered(uint256 chainId) external view returns (bool);
    function isKmsSigner(address signerAddress) external view returns (bool);
    function isKmsTxSender(address txSenderAddress) external view returns (bool);
    function isPauser(address account) external view returns (bool);
    function owner() external view returns (address);
    function pauseAllGatewayContracts() external;
    function pendingOwner() external view returns (address);
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV3(KmsNode[] memory newKmsNodes) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function unpauseAllGatewayContracts() external;
    function updateCoprocessorThreshold(uint256 newCoprocessorThreshold) external;
    function updateCoprocessors(Coprocessor[] memory newCoprocessors, uint256 newCoprocessorThreshold) external;
    function updateCustodians(Custodian[] memory newCustodians) external;
    function updateKmsGenThreshold(uint256 newKmsGenThreshold) external;
    function updateKmsNodes(KmsNode[] memory newKmsNodes, uint256 newMpcThreshold, uint256 newPublicDecryptionThreshold, uint256 newUserDecryptionThreshold, uint256 newKmsGenThreshold) external;
    function updateMpcThreshold(uint256 newMpcThreshold) external;
    function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external;
    function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external;
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
    "name": "acceptOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addHostChain",
    "inputs": [
      {
        "name": "hostChain",
        "type": "tuple",
        "internalType": "struct HostChain",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fhevmExecutorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "aclAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "website",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getCoprocessor",
    "inputs": [
      {
        "name": "coprocessorTxSenderAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Coprocessor",
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
            "name": "s3BucketUrl",
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
    "name": "getCoprocessorMajorityThreshold",
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
    "name": "getCoprocessorSigners",
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
    "name": "getCoprocessorTxSenders",
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
    "name": "getCustodian",
    "inputs": [
      {
        "name": "custodianTxSenderAddress",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Custodian",
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
            "name": "encryptionKey",
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
    "name": "getCustodianSigners",
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
    "name": "getCustodianTxSenders",
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
    "name": "getHostChain",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct HostChain",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fhevmExecutorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "aclAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "website",
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
    "name": "getHostChains",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct HostChain[]",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fhevmExecutorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "aclAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "website",
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
    "name": "getKmsNode",
    "inputs": [
      {
        "name": "kmsTxSenderAddress",
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
    "name": "getKmsTxSenders",
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
    "name": "getProtocolMetadata",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ProtocolMetadata",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "website",
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
        "name": "initialMetadata",
        "type": "tuple",
        "internalType": "struct ProtocolMetadata",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "website",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "initialThresholds",
        "type": "tuple",
        "internalType": "struct IGatewayConfig.Thresholds",
        "components": [
          {
            "name": "mpcThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "publicDecryptionThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "userDecryptionThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "kmsGenThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "coprocessorThreshold",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "initialKmsNodes",
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
      },
      {
        "name": "initialCoprocessors",
        "type": "tuple[]",
        "internalType": "struct Coprocessor[]",
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
            "name": "s3BucketUrl",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "initialCustodians",
        "type": "tuple[]",
        "internalType": "struct Custodian[]",
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
            "name": "encryptionKey",
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
    "name": "isCoprocessorSigner",
    "inputs": [
      {
        "name": "signerAddress",
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
    "name": "isCoprocessorTxSender",
    "inputs": [
      {
        "name": "txSenderAddress",
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
    "name": "isCustodianSigner",
    "inputs": [
      {
        "name": "signerAddress",
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
    "name": "isCustodianTxSender",
    "inputs": [
      {
        "name": "txSenderAddress",
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
    "name": "isHostChainRegistered",
    "inputs": [
      {
        "name": "chainId",
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
    "name": "isKmsSigner",
    "inputs": [
      {
        "name": "signerAddress",
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
    "name": "isKmsTxSender",
    "inputs": [
      {
        "name": "txSenderAddress",
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
    "name": "isPauser",
    "inputs": [
      {
        "name": "account",
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
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "pauseAllGatewayContracts",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pendingOwner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
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
    "inputs": [
      {
        "name": "newKmsNodes",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpauseAllGatewayContracts",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateCoprocessorThreshold",
    "inputs": [
      {
        "name": "newCoprocessorThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateCoprocessors",
    "inputs": [
      {
        "name": "newCoprocessors",
        "type": "tuple[]",
        "internalType": "struct Coprocessor[]",
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
            "name": "s3BucketUrl",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "newCoprocessorThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateCustodians",
    "inputs": [
      {
        "name": "newCustodians",
        "type": "tuple[]",
        "internalType": "struct Custodian[]",
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
            "name": "encryptionKey",
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
    "name": "updateKmsGenThreshold",
    "inputs": [
      {
        "name": "newKmsGenThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateKmsNodes",
    "inputs": [
      {
        "name": "newKmsNodes",
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
      },
      {
        "name": "newMpcThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newPublicDecryptionThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newUserDecryptionThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "newKmsGenThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateMpcThreshold",
    "inputs": [
      {
        "name": "newMpcThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updatePublicDecryptionThreshold",
    "inputs": [
      {
        "name": "newPublicDecryptionThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateUserDecryptionThreshold",
    "inputs": [
      {
        "name": "newUserDecryptionThreshold",
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
    "name": "AddHostChain",
    "inputs": [
      {
        "name": "hostChain",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct HostChain",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "fhevmExecutorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "aclAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "website",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "InitializeGatewayConfig",
    "inputs": [
      {
        "name": "metadata",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct ProtocolMetadata",
        "components": [
          {
            "name": "name",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "website",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "thresholds",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IGatewayConfig.Thresholds",
        "components": [
          {
            "name": "mpcThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "publicDecryptionThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "userDecryptionThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "kmsGenThreshold",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "coprocessorThreshold",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "kmsNodes",
        "type": "tuple[]",
        "indexed": false,
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
      },
      {
        "name": "coprocessors",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct Coprocessor[]",
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
            "name": "s3BucketUrl",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "custodians",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct Custodian[]",
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
            "name": "encryptionKey",
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
    "name": "OwnershipTransferStarted",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PauseAllGatewayContracts",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ReinitializeGatewayConfigV3",
    "inputs": [
      {
        "name": "newKmsNodes",
        "type": "tuple[]",
        "indexed": false,
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
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UnpauseAllGatewayContracts",
    "inputs": [],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateCoprocessorThreshold",
    "inputs": [
      {
        "name": "newCoprocessorThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateCoprocessors",
    "inputs": [
      {
        "name": "newCoprocessors",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct Coprocessor[]",
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
            "name": "s3BucketUrl",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "newCoprocessorThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateCustodians",
    "inputs": [
      {
        "name": "newCustodians",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct Custodian[]",
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
            "name": "encryptionKey",
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
    "name": "UpdateKmsGenThreshold",
    "inputs": [
      {
        "name": "newKmsGenThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateKmsNodes",
    "inputs": [
      {
        "name": "newKmsNodes",
        "type": "tuple[]",
        "indexed": false,
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
      },
      {
        "name": "newMpcThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newPublicDecryptionThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newUserDecryptionThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newKmsGenThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateMpcThreshold",
    "inputs": [
      {
        "name": "newMpcThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdatePublicDecryptionThreshold",
    "inputs": [
      {
        "name": "newPublicDecryptionThreshold",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateUserDecryptionThreshold",
    "inputs": [
      {
        "name": "newUserDecryptionThreshold",
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
    "name": "ChainIdNotUint64",
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
    "name": "EmptyCoprocessors",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyCustodians",
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
    "name": "HostChainAlreadyRegistered",
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
    "name": "InvalidHighCoprocessorThreshold",
    "inputs": [
      {
        "name": "coprocessorThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "nCoprocessors",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidHighKmsGenThreshold",
    "inputs": [
      {
        "name": "kmsGenThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "nKmsNodes",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidHighMpcThreshold",
    "inputs": [
      {
        "name": "mpcThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "nKmsNodes",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidHighPublicDecryptionThreshold",
    "inputs": [
      {
        "name": "publicDecryptionThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "nKmsNodes",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidHighUserDecryptionThreshold",
    "inputs": [
      {
        "name": "userDecryptionThreshold",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "nKmsNodes",
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
    "name": "InvalidNullChainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNullCoprocessorThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNullKmsGenThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNullPublicDecryptionThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNullUserDecryptionThreshold",
    "inputs": []
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
    "name": "NotPauser",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableInvalidOwner",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OwnableUnauthorizedAccount",
    "inputs": [
      {
        "name": "account",
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
pub mod GatewayConfig {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051616c5f620001eb5f395f81816136f60152818161374b01526139050152616c5f5ff3fe608060405260043610610287575f3560e01c8063798b58a611610159578063bff3aaba116100c0578063e30c397811610079578063e30c397814610975578063e3b2a8741461099f578063e5275eaf146109db578063eb843cf614610a17578063ef6997f914610a3f578063f2fde38b14610a7b57610287565b8063bff3aaba14610847578063c2b4298614610883578063c80b33ca146108ad578063cb5aa7e9146108d5578063d10f7ff914610911578063d5e16b7d1461094d57610287565b80639164d0ae116101125780639164d0ae146107615780639a5a3bc41461078b578063ad3cb1cc146107a1578063b4722bc4146107cb578063ba1f31d2146107f5578063bb59e3621461081f57610287565b8063798b58a61461067d57806379ba5097146106935780637eaac8f2146106a957806383bb2e57146106d3578063882d7dd3146106fb5780638da5cb5b1461073757610287565b80632dd3edfe116101fd57806353da9246116101b657806353da9246146105875780635bace7ff146105af5780636799ef52146105eb578063715018a6146106155780637420f3d41461062b578063772d2fe91461065557610287565b80632dd3edfe146104775780632e2d3a82146104b357806346fbf68e146104db57806348144c61146105175780634f1ef2861461054157806352d1902d1461055d57610287565b80632585bb651161024f5780632585bb651461036b57806326cf5def1461039557806327612cf6146103bf5780632a388998146103e75780632a8b9de9146104115780632b101c031461043b57610287565b8063013dc21e1461028b5780630724dd23146102b35780630d8e6e2c146102db5780631ea5bd4214610305578063203d01141461032f575b5f80fd5b348015610296575f80fd5b506102b160048036038101906102ac9190614b8d565b610aa3565b005b3480156102be575f80fd5b506102d960048036038101906102d49190614c0b565b610d3f565b005b3480156102e6575f80fd5b506102ef610d8a565b6040516102fc9190614cc0565b60405180910390f35b348015610310575f80fd5b50610319610e05565b6040516103269190614dc7565b60405180910390f35b34801561033a575f80fd5b5061035560048036038101906103509190614e11565b610e9e565b6040516103629190614e56565b60405180910390f35b348015610376575f80fd5b5061037f610efe565b60405161038c9190614ffb565b60405180910390f35b3480156103a0575f80fd5b506103a961113b565b6040516103b6919061502a565b60405180910390f35b3480156103ca575f80fd5b506103e560048036038101906103e09190615098565b611152565b005b3480156103f2575f80fd5b506103fb6112dc565b604051610408919061502a565b60405180910390f35b34801561041c575f80fd5b506104256112f3565b6040516104329190614dc7565b60405180910390f35b348015610446575f80fd5b50610461600480360381019061045c9190614e11565b61138c565b60405161046e9190614e56565b60405180910390f35b348015610482575f80fd5b5061049d60048036038101906104989190614e11565b6113ec565b6040516104aa9190614e56565b60405180910390f35b3480156104be575f80fd5b506104d960048036038101906104d49190614c0b565b61144c565b005b3480156104e6575f80fd5b5061050160048036038101906104fc9190614e11565b611497565b60405161050e9190614e56565b60405180910390f35b348015610522575f80fd5b5061052b61152b565b6040516105389190615124565b60405180910390f35b61055b6004803603810190610556919061526c565b611671565b005b348015610568575f80fd5b50610571611690565b60405161057e91906152de565b60405180910390f35b348015610592575f80fd5b506105ad60048036038101906105a891906152f7565b6116c1565b005b3480156105ba575f80fd5b506105d560048036038101906105d09190614e11565b61197c565b6040516105e29190614e56565b60405180910390f35b3480156105f6575f80fd5b506105ff6119dc565b60405161060c919061502a565b60405180910390f35b348015610620575f80fd5b506106296119f3565b005b348015610636575f80fd5b5061063f611a06565b60405161064c9190614dc7565b60405180910390f35b348015610660575f80fd5b5061067b60048036038101906106769190614c0b565b611a9f565b005b348015610688575f80fd5b50610691611aea565b005b34801561069e575f80fd5b506106a7611bfe565b005b3480156106b4575f80fd5b506106bd611c8c565b6040516106ca9190614dc7565b60405180910390f35b3480156106de575f80fd5b506106f960048036038101906106f491906153e2565b611d25565b005b348015610706575f80fd5b50610721600480360381019061071c9190614e11565b611fc5565b60405161072e9190614e56565b60405180910390f35b348015610742575f80fd5b5061074b612025565b604051610758919061544e565b60405180910390f35b34801561076c575f80fd5b5061077561205a565b6040516107829190614dc7565b60405180910390f35b348015610796575f80fd5b5061079f6120f3565b005b3480156107ac575f80fd5b506107b5612249565b6040516107c29190614cc0565b60405180910390f35b3480156107d6575f80fd5b506107df612282565b6040516107ec919061502a565b60405180910390f35b348015610800575f80fd5b50610809612299565b6040516108169190614dc7565b60405180910390f35b34801561082a575f80fd5b50610845600480360381019061084091906154a7565b612332565b005b348015610852575f80fd5b5061086d60048036038101906108689190614c0b565b612554565b60405161087a9190614e56565b60405180910390f35b34801561088e575f80fd5b50610897612588565b6040516108a4919061502a565b60405180910390f35b3480156108b8575f80fd5b506108d360048036038101906108ce91906155ba565b61259f565b005b3480156108e0575f80fd5b506108fb60048036038101906108f69190614e11565b61274d565b60405161090891906156a0565b60405180910390f35b34801561091c575f80fd5b5061093760048036038101906109329190614c0b565b6128eb565b604051610944919061573a565b60405180910390f35b348015610958575f80fd5b50610973600480360381019061096e9190614c0b565b612b06565b005b348015610980575f80fd5b50610989612b51565b604051610996919061544e565b60405180910390f35b3480156109aa575f80fd5b506109c560048036038101906109c09190614e11565b612b86565b6040516109d291906157c1565b60405180910390f35b3480156109e6575f80fd5b50610a0160048036038101906109fc9190614e11565b612db4565b604051610a0e9190614e56565b60405180910390f35b348015610a22575f80fd5b50610a3d6004803603810190610a389190614c0b565b612e14565b005b348015610a4a575f80fd5b50610a656004803603810190610a609190614e11565b612e5f565b604051610a72919061582e565b60405180910390f35b348015610a86575f80fd5b50610aa16004803603810190610a9c9190614e11565b612ffd565b005b610aab6130b6565b5f610ab461313d565b90505f816012018054905090505f5b81811015610cd7575f836014015f856012018481548110610ae757610ae661584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836015015f856013018481548110610b7a57610b7961584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550826011015f846012018381548110610c0c57610c0b61584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8082015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600182015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600282015f610cc8919061490e565b50508080600101915050610ac3565b50816012015f610ce7919061494b565b816013015f610cf6919061494b565b610d008484613164565b7f6cdc1aa76e1ebacd67c81be0dcf9603b5dfbeb4dd801ab214114acb536f110688484604051610d31929190615a63565b60405180910390a150505050565b610d476130b6565b610d50816134bb565b7f30c9b1d004f57eae3c6cc3a3752bcb4c8ea2e57c8241a782aa9b65fbc604ec5b81604051610d7f919061502a565b60405180910390a150565b60606040518060400160405280600d81526020017f47617465776179436f6e66696700000000000000000000000000000000000000815250610dcb5f61355f565b610dd5600361355f565b610dde5f61355f565b604051602001610df19493929190615b53565b604051602081830303815290604052905090565b60605f610e1061313d565b905080600d01805480602002602001604051908101604052809291908181526020018280548015610e9357602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610e4a575b505050505091505090565b5f80610ea861313d565b9050806003015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b60605f610f0961313d565b905080601001805480602002602001604051908101604052809291908181526020015f905b82821015611131578382905f5260205f2090600502016040518060a00160405290815f8201548152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160038201805461101290615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461103e90615bde565b80156110895780601f1061106057610100808354040283529160200191611089565b820191905f5260205f20905b81548152906001019060200180831161106c57829003601f168201915b505050505081526020016004820180546110a290615bde565b80601f01602080910402602001604051908101604052809291908181526020018280546110ce90615bde565b80156111195780601f106110f057610100808354040283529160200191611119565b820191905f5260205f20905b8154815290600101906020018083116110fc57829003601f168201915b50505050508152505081526020019060010190610f2e565b5050505091505090565b5f8061114561313d565b9050806007015491505090565b60045f61115d613629565b9050805f0160089054906101000a900460ff16806111a557508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156111dc576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f61122a61313d565b905061124a858583600701548460080154856009015486601601546116c1565b7f430929cffdf43b3681bb03eae5802745e335ec0159968d301e2a8e3d209eefb8858560405161127b929190615df8565b60405180910390a1505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516112ce9190615e3c565b60405180910390a150505050565b5f806112e661313d565b9050806008015491505090565b60605f6112fe61313d565b90508060120180548060200260200160405190810160405280929190818152602001828054801561138157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611338575b505050505091505090565b5f8061139661313d565b905080600b015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806113f661313d565b905080600a015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b6114546130b6565b61145d81613650565b7fe41802af725729adcb8c151e2937380a25c69155757e3af5d3979adab50358008160405161148c919061502a565b60405180910390a150565b5f73c3f9e1d27cd10402375b7cd237d57e0f4888c18973ffffffffffffffffffffffffffffffffffffffff166346fbf68e836040518263ffffffff1660e01b81526004016114e5919061544e565b602060405180830381865afa158015611500573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115249190615e7f565b9050919050565b611533614969565b5f61153c61313d565b9050805f016040518060400160405290815f8201805461155b90615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461158790615bde565b80156115d25780601f106115a9576101008083540402835291602001916115d2565b820191905f5260205f20905b8154815290600101906020018083116115b557829003601f168201915b505050505081526020016001820180546115eb90615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461161790615bde565b80156116625780601f1061163957610100808354040283529160200191611662565b820191905f5260205f20905b81548152906001019060200180831161164557829003601f168201915b50505050508152505091505090565b6116796136f4565b611682826137da565b61168c82826137e5565b5050565b5f611699613903565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b6116c96130b6565b5f6116d261313d565b90505f816005018054905090505f5b81811015611904575f836002015f8560050184815481106117055761170461584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836003015f8560060184815481106117985761179761584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550826004015f84600501838154811061182a5761182961584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8082015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600182015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600282015f6118e69190614983565b600382015f6118f59190614983565b505080806001019150506116e1565b50816005015f611914919061494b565b816006015f611923919061494b565b61193188888888888861398a565b7f25d1ea647128b56d47e64534cd0f5a86d3207f67b04895495b66dc0db87a0ca788888888888860405161196a96959493929190615eaa565b60405180910390a15050505050505050565b5f8061198661313d565b9050806014015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806119e661313d565b9050806017015491505090565b6119fb6130b6565b611a045f613d09565b565b60605f611a1161313d565b905080600501805480602002602001604051908101604052809291908181526020018280548015611a9457602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611a4b575b505050505091505090565b611aa76130b6565b611ab081613d46565b7f3571172a49e72d7724be384cdd59f4f21a216c70352ea59cb02543fc7630843781604051611adf919061502a565b60405180910390a150565b611af26130b6565b7387a5b1152aa51728258dbc1aa54b6a83dcd1d3dd73ffffffffffffffffffffffffffffffffffffffff16633f4ba83a6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611b4b575f80fd5b505af1158015611b5d573d5f803e3d5ffd5b505050507333e0c7a03d2b040b518580c365f4b3bde7cc4e6e73ffffffffffffffffffffffffffffffffffffffff16633f4ba83a6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611bba575f80fd5b505af1158015611bcc573d5f803e3d5ffd5b505050507fbe4f655daae0dbaef63a6b525cab2fa6ace4aa5b94b8834b241137cdfe73a5b060405160405180910390a1565b5f611c07613db0565b90508073ffffffffffffffffffffffffffffffffffffffff16611c28612b51565b73ffffffffffffffffffffffffffffffffffffffff1614611c8057806040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401611c77919061544e565b60405180910390fd5b611c8981613d09565b50565b60605f611c9761313d565b905080600601805480602002602001604051908101604052809291908181526020018280548015611d1a57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611cd1575b505050505091505090565b611d2d6130b6565b5f611d3661313d565b90505f81600d018054905090505f5b81811015611f59575f83600a015f85600d018481548110611d6957611d6861584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f83600b015f85600e018481548110611dfc57611dfb61584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555082600c015f84600d018381548110611e8e57611e8d61584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8082015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600182015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600282015f611f4a9190614983565b50508080600101915050611d45565b5081600d015f611f69919061494b565b81600e015f611f78919061494b565b611f83858585613db7565b7fffe20bdb855e514e94147702922690cf1da10bdd18bf1f6215027c93ac05d455858585604051611fb69392919061603c565b60405180910390a15050505050565b5f80611fcf61313d565b9050806015015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f8061202f614118565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b60605f61206561313d565b905080600e018054806020026020016040519081016040528092919081815260200182805480156120e857602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161209f575b505050505091505090565b6120fc33611497565b61213d57336040517f206a346e000000000000000000000000000000000000000000000000000000008152600401612134919061544e565b60405180910390fd5b7387a5b1152aa51728258dbc1aa54b6a83dcd1d3dd73ffffffffffffffffffffffffffffffffffffffff16638456cb596040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612196575f80fd5b505af11580156121a8573d5f803e3d5ffd5b505050507333e0c7a03d2b040b518580c365f4b3bde7cc4e6e73ffffffffffffffffffffffffffffffffffffffff16638456cb596040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612205575f80fd5b505af1158015612217573d5f803e3d5ffd5b505050507f13dbe8823219e226dd0525aeb071e1d2679f89382ba799f7f644867e65b6f3a660405160405180910390a1565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f8061228c61313d565b9050806016015491505090565b60605f6122a461313d565b90508060130180548060200260200160405190810160405280929190818152602001828054801561232757602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116122de575b505050505091505090565b600161233c61413f565b67ffffffffffffffff161461237d576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60045f612388613629565b9050805f0160089054906101000a900460ff16806123d057508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15612407576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061245c612457612025565b614163565b5f61246561313d565b90508a815f01818161247791906163a2565b90505061249789898c5f01358d602001358e604001358f6060013561398a565b6124a687878c60800135613db7565b6124b08585613164565b7fb2cbe65ea308bfe4b9431819a3168d544f46ba344b1e79f92f973fcff43aae3b8b8b8b8b8b8b8b8b6040516124ed9897969594939291906164a9565b60405180910390a1505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516125409190615e3c565b60405180910390a150505050505050505050565b5f8061255e61313d565b905080600f015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f8061259261313d565b9050806009015491505090565b6125a76130b6565b5f815f0135036125e3576040517f22f73fea00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f0135111561263857805f01356040517f4178de4200000000000000000000000000000000000000000000000000000000815260040161262f919061502a565b60405180910390fd5b5f61264161313d565b905080600f015f835f013581526020019081526020015f205f9054906101000a900460ff16156126ab57815f01356040517f96a568280000000000000000000000000000000000000000000000000000000081526004016126a2919061502a565b60405180910390fd5b8060100182908060018154018082558091505060019003905f5260205f2090600502015f9091909190915081816126e29190616702565b5050600181600f015f845f013581526020019081526020015f205f6101000a81548160ff0219169083151502179055507f66769341effd268fc4e9a9c8f27bfc968507b519b0ddb9b4ad3ded5f030168378260405161274191906167b6565b60405180910390a15050565b6127556149c0565b5f61275e61313d565b9050806011015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060600160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461286390615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461288f90615bde565b80156128da5780601f106128b1576101008083540402835291602001916128da565b820191905f5260205f20905b8154815290600101906020018083116128bd57829003601f168201915b505050505081525050915050919050565b6128f3614a0b565b5f6128fc61313d565b90508060100183815481106129145761291361584e565b5b905f5260205f2090600502016040518060a00160405290815f8201548152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016003820180546129ee90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612a1a90615bde565b8015612a655780601f10612a3c57610100808354040283529160200191612a65565b820191905f5260205f20905b815481529060010190602001808311612a4857829003601f168201915b50505050508152602001600482018054612a7e90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612aaa90615bde565b8015612af55780601f10612acc57610100808354040283529160200191612af5565b820191905f5260205f20905b815481529060010190602001808311612ad857829003601f168201915b505050505081525050915050919050565b612b0e6130b6565b612b1781614177565b7f7a2ef7dc89400a8ad92bb4ccf44d482624b40fe76b66977e85ed6a618e2e2fc781604051612b46919061502a565b60405180910390a150565b5f80612b5b61421b565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b612b8e614a63565b5f612b9761313d565b9050806004015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054612c9c90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612cc890615bde565b8015612d135780601f10612cea57610100808354040283529160200191612d13565b820191905f5260205f20905b815481529060010190602001808311612cf657829003601f168201915b50505050508152602001600382018054612d2c90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612d5890615bde565b8015612da35780601f10612d7a57610100808354040283529160200191612da3565b820191905f5260205f20905b815481529060010190602001808311612d8657829003601f168201915b505050505081525050915050919050565b5f80612dbe61313d565b9050806002015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b612e1c6130b6565b612e2581614242565b7f837e0a6528dadfa2dc792692c5182e52a9f5bbdeed7b2372927a26c69583961381604051612e54919061502a565b60405180910390a150565b612e67614ab5565b5f612e7061313d565b905080600c015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060600160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054612f7590615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612fa190615bde565b8015612fec5780601f10612fc357610100808354040283529160200191612fec565b820191905f5260205f20905b815481529060010190602001808311612fcf57829003601f168201915b505050505081525050915050919050565b6130056130b6565b5f61300e61421b565b905081815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff16613070612025565b73ffffffffffffffffffffffffffffffffffffffff167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a35050565b6130be613db0565b73ffffffffffffffffffffffffffffffffffffffff166130dc612025565b73ffffffffffffffffffffffffffffffffffffffff161461313b576130ff613db0565b6040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401613132919061544e565b60405180910390fd5b565b5f7f86d3070a8993f6b209bee6185186d38a07fce8bbd97c750d934451b72f35b400905090565b5f82829050036131a0576040517fcad1d53400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6131a961313d565b90505f5b838390508110156134b5578383828181106131cb576131ca61584e565b5b90506020028101906131dd91906167d6565b826011015f8686858181106131f5576131f461584e565b5b905060200281019061320791906167d6565b5f0160208101906132189190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20818161325d9190616a03565b905050816012018484838181106132775761327661584e565b5b905060200281019061328991906167d6565b5f01602081019061329a9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001826014015f8686858181106133115761331061584e565b5b905060200281019061332391906167d6565b5f0160208101906133349190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508160130184848381811061339a5761339961584e565b5b90506020028101906133ac91906167d6565b60200160208101906133be9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001826015015f8686858181106134355761343461584e565b5b905060200281019061344791906167d6565b60200160208101906134599190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080806001019150506131ad565b50505050565b5f6134c461313d565b90505f816006018054905090505f830361350a576040517f3ee5077400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156135515782816040517f0f69cbfc000000000000000000000000000000000000000000000000000000008152600401613548929190616a11565b60405180910390fd5b828260160181905550505050565b60605f600161356d846142e6565b0190505f8167ffffffffffffffff81111561358b5761358a615148565b5b6040519080825280601f01601f1916602001820160405280156135bd5781602001600182028036833780820191505090505b5090505f82602001820190505b60011561361e578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161361357613612616a38565b5b0494505f85036135ca575b819350505050919050565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f61365961313d565b90505f816006018054905090505f830361369f576040517fb1ae92ea00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156136e65782816040517f84208f230000000000000000000000000000000000000000000000000000000081526004016136dd929190616a11565b60405180910390fd5b828260080181905550505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614806137a157507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613788614437565b73ffffffffffffffffffffffffffffffffffffffff1614155b156137d8576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6137e26130b6565b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561384d57506040513d601f19601f8201168201806040525081019061384a9190616a8f565b60015b61388e57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613885919061544e565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146138f457806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016138eb91906152de565b60405180910390fd5b6138fe838361448a565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614613988576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f86869050036139c6576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6139cf61313d565b90505f5b87879050811015613cdb576001826002015f8a8a858181106139f8576139f761584e565b5b9050602002810190613a0a9190616aba565b5f016020810190613a1b9190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550878782818110613a7d57613a7c61584e565b5b9050602002810190613a8f9190616aba565b826004015f8a8a85818110613aa757613aa661584e565b5b9050602002810190613ab99190616aba565b5f016020810190613aca9190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181613b0f9190616b67565b90505081600501888883818110613b2957613b2861584e565b5b9050602002810190613b3b9190616aba565b5f016020810190613b4c9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001826003015f8a8a85818110613bc357613bc261584e565b5b9050602002810190613bd59190616aba565b6020016020810190613be79190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555081600601888883818110613c4d57613c4c61584e565b5b9050602002810190613c5f9190616aba565b6020016020810190613c719190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555080806001019150506139d3565b50613ce585613d46565b613cee84613650565b613cf783614242565b613d00826134bb565b50505050505050565b5f613d1261421b565b9050805f015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055613d42826144fc565b5050565b5f613d4f61313d565b90505f81600601805490509050808310613da25782816040517f907e6681000000000000000000000000000000000000000000000000000000008152600401613d99929190616a11565b60405180910390fd5b828260070181905550505050565b5f33905090565b5f8383905003613df3576040517f8af082ef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f613dfc61313d565b90505f5b8484905081101561410857600182600a015f878785818110613e2557613e2461584e565b5b9050602002810190613e379190616b75565b5f016020810190613e489190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550848482818110613eaa57613ea961584e565b5b9050602002810190613ebc9190616b75565b82600c015f878785818110613ed457613ed361584e565b5b9050602002810190613ee69190616b75565b5f016020810190613ef79190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181613f3c9190616c01565b90505081600d01858583818110613f5657613f5561584e565b5b9050602002810190613f689190616b75565b5f016020810190613f799190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600182600b015f878785818110613ff057613fef61584e565b5b90506020028101906140029190616b75565b60200160208101906140149190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555081600e0185858381811061407a5761407961584e565b5b905060200281019061408c9190616b75565b602001602081019061409e9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508080600101915050613e00565b5061411282614177565b50505050565b5f7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300905090565b5f614148613629565b5f015f9054906101000a900467ffffffffffffffff16905090565b61416b6145cd565b6141748161460d565b50565b5f61418061313d565b90505f81600e018054905090505f83036141c6576040517fb60d244100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8083111561420d5782816040517f97beabad000000000000000000000000000000000000000000000000000000008152600401614204929190616a11565b60405180910390fd5b828260170181905550505050565b5f7f237e158222e3e6968b72b9db0d8043aacf074ad9f650f0d1606b4d82ee432c00905090565b5f61424b61313d565b90505f816006018054905090505f8303614291576040517fe60a727100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156142d85782816040517fd2535e110000000000000000000000000000000000000000000000000000000081526004016142cf929190616a11565b60405180910390fd5b828260090181905550505050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310614342577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161433857614337616a38565b5b0492506040810190505b6d04ee2d6d415b85acef8100000000831061437f576d04ee2d6d415b85acef8100000000838161437557614374616a38565b5b0492506020810190505b662386f26fc1000083106143ae57662386f26fc1000083816143a4576143a3616a38565b5b0492506010810190505b6305f5e10083106143d7576305f5e10083816143cd576143cc616a38565b5b0492506008810190505b61271083106143fc5761271083816143f2576143f1616a38565b5b0492506004810190505b6064831061441f576064838161441557614414616a38565b5b0492506002810190505b600a831061442e576001810190505b80915050919050565b5f6144637f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614691565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b6144938261469a565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156144ef576144e98282614763565b506144f8565b6144f76147e3565b5b5050565b5f614505614118565b90505f815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905082825f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a3505050565b6145d561481f565b61460b576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6146156145cd565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614685575f6040517f1e4fbdf700000000000000000000000000000000000000000000000000000000815260040161467c919061544e565b60405180910390fd5b61468e81613d09565b50565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036146f557806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016146ec919061544e565b60405180910390fd5b806147217f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614691565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff168460405161478c9190616c49565b5f60405180830381855af49150503d805f81146147c4576040519150601f19603f3d011682016040523d82523d5f602084013e6147c9565b606091505b50915091506147d985838361483d565b9250505092915050565b5f34111561481d576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f614828613629565b5f0160089054906101000a900460ff16905090565b6060826148525761484d826148ca565b6148c2565b5f825114801561487857505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156148ba57836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016148b1919061544e565b60405180910390fd5b8190506148c3565b5b9392505050565b5f815111156148dc5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50805461491a90615bde565b5f825580601f1061492b5750614948565b601f0160209004905f5260205f20908101906149479190614b00565b5b50565b5080545f8255905f5260205f20908101906149669190614b00565b50565b604051806040016040528060608152602001606081525090565b50805461498f90615bde565b5f825580601f106149a057506149bd565b601f0160209004905f5260205f20908101906149bc9190614b00565b5b50565b60405180606001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff168152602001606081525090565b6040518060a001604052805f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b60405180606001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff168152602001606081525090565b5b80821115614b17575f815f905550600101614b01565b5090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112614b4d57614b4c614b2c565b5b8235905067ffffffffffffffff811115614b6a57614b69614b30565b5b602083019150836020820283011115614b8657614b85614b34565b5b9250929050565b5f8060208385031215614ba357614ba2614b24565b5b5f83013567ffffffffffffffff811115614bc057614bbf614b28565b5b614bcc85828601614b38565b92509250509250929050565b5f819050919050565b614bea81614bd8565b8114614bf4575f80fd5b50565b5f81359050614c0581614be1565b92915050565b5f60208284031215614c2057614c1f614b24565b5b5f614c2d84828501614bf7565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015614c6d578082015181840152602081019050614c52565b5f8484015250505050565b5f601f19601f8301169050919050565b5f614c9282614c36565b614c9c8185614c40565b9350614cac818560208601614c50565b614cb581614c78565b840191505092915050565b5f6020820190508181035f830152614cd88184614c88565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f614d3282614d09565b9050919050565b614d4281614d28565b82525050565b5f614d538383614d39565b60208301905092915050565b5f602082019050919050565b5f614d7582614ce0565b614d7f8185614cea565b9350614d8a83614cfa565b805f5b83811015614dba578151614da18882614d48565b9750614dac83614d5f565b925050600181019050614d8d565b5085935050505092915050565b5f6020820190508181035f830152614ddf8184614d6b565b905092915050565b614df081614d28565b8114614dfa575f80fd5b50565b5f81359050614e0b81614de7565b92915050565b5f60208284031215614e2657614e25614b24565b5b5f614e3384828501614dfd565b91505092915050565b5f8115159050919050565b614e5081614e3c565b82525050565b5f602082019050614e695f830184614e47565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b614ea181614bd8565b82525050565b5f82825260208201905092915050565b5f614ec182614c36565b614ecb8185614ea7565b9350614edb818560208601614c50565b614ee481614c78565b840191505092915050565b5f60a083015f830151614f045f860182614e98565b506020830151614f176020860182614d39565b506040830151614f2a6040860182614d39565b5060608301518482036060860152614f428282614eb7565b91505060808301518482036080860152614f5c8282614eb7565b9150508091505092915050565b5f614f748383614eef565b905092915050565b5f602082019050919050565b5f614f9282614e6f565b614f9c8185614e79565b935083602082028501614fae85614e89565b805f5b85811015614fe95784840389528151614fca8582614f69565b9450614fd583614f7c565b925060208a01995050600181019050614fb1565b50829750879550505050505092915050565b5f6020820190508181035f8301526150138184614f88565b905092915050565b61502481614bd8565b82525050565b5f60208201905061503d5f83018461501b565b92915050565b5f8083601f84011261505857615057614b2c565b5b8235905067ffffffffffffffff81111561507557615074614b30565b5b60208301915083602082028301111561509157615090614b34565b5b9250929050565b5f80602083850312156150ae576150ad614b24565b5b5f83013567ffffffffffffffff8111156150cb576150ca614b28565b5b6150d785828601615043565b92509250509250929050565b5f604083015f8301518482035f8601526150fd8282614eb7565b915050602083015184820360208601526151178282614eb7565b9150508091505092915050565b5f6020820190508181035f83015261513c81846150e3565b905092915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61517e82614c78565b810181811067ffffffffffffffff8211171561519d5761519c615148565b5b80604052505050565b5f6151af614b1b565b90506151bb8282615175565b919050565b5f67ffffffffffffffff8211156151da576151d9615148565b5b6151e382614c78565b9050602081019050919050565b828183375f83830152505050565b5f61521061520b846151c0565b6151a6565b90508281526020810184848401111561522c5761522b615144565b5b6152378482856151f0565b509392505050565b5f82601f83011261525357615252614b2c565b5b81356152638482602086016151fe565b91505092915050565b5f806040838503121561528257615281614b24565b5b5f61528f85828601614dfd565b925050602083013567ffffffffffffffff8111156152b0576152af614b28565b5b6152bc8582860161523f565b9150509250929050565b5f819050919050565b6152d8816152c6565b82525050565b5f6020820190506152f15f8301846152cf565b92915050565b5f805f805f8060a0878903121561531157615310614b24565b5b5f87013567ffffffffffffffff81111561532e5761532d614b28565b5b61533a89828a01615043565b9650965050602061534d89828a01614bf7565b945050604061535e89828a01614bf7565b935050606061536f89828a01614bf7565b925050608061538089828a01614bf7565b9150509295509295509295565b5f8083601f8401126153a2576153a1614b2c565b5b8235905067ffffffffffffffff8111156153bf576153be614b30565b5b6020830191508360208202830111156153db576153da614b34565b5b9250929050565b5f805f604084860312156153f9576153f8614b24565b5b5f84013567ffffffffffffffff81111561541657615415614b28565b5b6154228682870161538d565b9350935050602061543586828701614bf7565b9150509250925092565b61544881614d28565b82525050565b5f6020820190506154615f83018461543f565b92915050565b5f80fd5b5f604082840312156154805761547f615467565b5b81905092915050565b5f60a0828403121561549e5761549d615467565b5b81905092915050565b5f805f805f805f80610120898b0312156154c4576154c3614b24565b5b5f89013567ffffffffffffffff8111156154e1576154e0614b28565b5b6154ed8b828c0161546b565b98505060206154fe8b828c01615489565b97505060c089013567ffffffffffffffff81111561551f5761551e614b28565b5b61552b8b828c01615043565b965096505060e089013567ffffffffffffffff81111561554e5761554d614b28565b5b61555a8b828c0161538d565b945094505061010089013567ffffffffffffffff81111561557e5761557d614b28565b5b61558a8b828c01614b38565b92509250509295985092959890939650565b5f60a082840312156155b1576155b0615467565b5b81905092915050565b5f602082840312156155cf576155ce614b24565b5b5f82013567ffffffffffffffff8111156155ec576155eb614b28565b5b6155f88482850161559c565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f61562582615601565b61562f818561560b565b935061563f818560208601614c50565b61564881614c78565b840191505092915050565b5f606083015f8301516156685f860182614d39565b50602083015161567b6020860182614d39565b5060408301518482036040860152615693828261561b565b9150508091505092915050565b5f6020820190508181035f8301526156b88184615653565b905092915050565b5f60a083015f8301516156d55f860182614e98565b5060208301516156e86020860182614d39565b5060408301516156fb6040860182614d39565b50606083015184820360608601526157138282614eb7565b9150506080830151848203608086015261572d8282614eb7565b9150508091505092915050565b5f6020820190508181035f83015261575281846156c0565b905092915050565b5f608083015f83015161576f5f860182614d39565b5060208301516157826020860182614d39565b506040830151848203604086015261579a8282614eb7565b915050606083015184820360608601526157b48282614eb7565b9150508091505092915050565b5f6020820190508181035f8301526157d9818461575a565b905092915050565b5f606083015f8301516157f65f860182614d39565b5060208301516158096020860182614d39565b50604083015184820360408601526158218282614eb7565b9150508091505092915050565b5f6020820190508181035f83015261584681846157e1565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b5f819050919050565b5f6158a26020840184614dfd565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126158d2576158d16158b2565b5b83810192508235915060208301925067ffffffffffffffff8211156158fa576158f96158aa565b5b6001820236038313156159105761590f6158ae565b5b509250929050565b5f615923838561560b565b93506159308385846151f0565b61593983614c78565b840190509392505050565b5f606083016159555f840184615894565b6159615f860182614d39565b5061596f6020840184615894565b61597c6020860182614d39565b5061598a60408401846158b6565b858303604087015261599d838284615918565b925050508091505092915050565b5f6159b68383615944565b905092915050565b5f823560016060038336030381126159d9576159d86158b2565b5b82810191505092915050565b5f602082019050919050565b5f6159fc838561587b565b935083602084028501615a0e8461588b565b805f5b87811015615a51578484038952615a2882846159be565b615a3285826159ab565b9450615a3d836159e5565b925060208a01995050600181019050615a11565b50829750879450505050509392505050565b5f6020820190508181035f830152615a7c8184866159f1565b90509392505050565b5f81905092915050565b5f615a9982614c36565b615aa38185615a85565b9350615ab3818560208601614c50565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f615af3600283615a85565b9150615afe82615abf565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f615b3d600183615a85565b9150615b4882615b09565b600182019050919050565b5f615b5e8287615a8f565b9150615b6982615ae7565b9150615b758286615a8f565b9150615b8082615b31565b9150615b8c8285615a8f565b9150615b9782615b31565b9150615ba38284615a8f565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680615bf557607f821691505b602082108103615c0857615c07615bb1565b5b50919050565b5f82825260208201905092915050565b5f819050919050565b5f8083356001602003843603038112615c4357615c426158b2565b5b83810192508235915060208301925067ffffffffffffffff821115615c6b57615c6a6158aa565b5b600182023603831315615c8157615c806158ae565b5b509250929050565b5f615c948385614ea7565b9350615ca18385846151f0565b615caa83614c78565b840190509392505050565b5f60808301615cc65f840184615894565b615cd25f860182614d39565b50615ce06020840184615894565b615ced6020860182614d39565b50615cfb6040840184615c27565b8583036040870152615d0e838284615c89565b92505050615d1f6060840184615c27565b8583036060870152615d32838284615c89565b925050508091505092915050565b5f615d4b8383615cb5565b905092915050565b5f82356001608003833603038112615d6e57615d6d6158b2565b5b82810191505092915050565b5f602082019050919050565b5f615d918385615c0e565b935083602084028501615da384615c1e565b805f5b87811015615de6578484038952615dbd8284615d53565b615dc78582615d40565b9450615dd283615d7a565b925060208a01995050600181019050615da6565b50829750879450505050509392505050565b5f6020820190508181035f830152615e11818486615d86565b90509392505050565b5f67ffffffffffffffff82169050919050565b615e3681615e1a565b82525050565b5f602082019050615e4f5f830184615e2d565b92915050565b615e5e81614e3c565b8114615e68575f80fd5b50565b5f81519050615e7981615e55565b92915050565b5f60208284031215615e9457615e93614b24565b5b5f615ea184828501615e6b565b91505092915050565b5f60a0820190508181035f830152615ec381888a615d86565b9050615ed2602083018761501b565b615edf604083018661501b565b615eec606083018561501b565b615ef9608083018461501b565b979650505050505050565b5f82825260208201905092915050565b5f819050919050565b5f60608301615f2e5f840184615894565b615f3a5f860182614d39565b50615f486020840184615894565b615f556020860182614d39565b50615f636040840184615c27565b8583036040870152615f76838284615c89565b925050508091505092915050565b5f615f8f8383615f1d565b905092915050565b5f82356001606003833603038112615fb257615fb16158b2565b5b82810191505092915050565b5f602082019050919050565b5f615fd58385615f04565b935083602084028501615fe784615f14565b805f5b8781101561602a5784840389526160018284615f97565b61600b8582615f84565b945061601683615fbe565b925060208a01995050600181019050615fea565b50829750879450505050509392505050565b5f6040820190508181035f830152616055818587615fca565b9050616064602083018461501b565b949350505050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126160945761609361606c565b5b80840192508235915067ffffffffffffffff8211156160b6576160b5616070565b5b6020830192506001820236038313156160d2576160d1616074565b5b509250929050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026161407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82616105565b61614a8683616105565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61618561618061617b84614bd8565b616162565b614bd8565b9050919050565b5f819050919050565b61619e8361616b565b6161b26161aa8261618c565b848454616111565b825550505050565b5f90565b6161c66161ba565b6161d1818484616195565b505050565b5b818110156161f4576161e95f826161be565b6001810190506161d7565b5050565b601f8211156162395761620a816160e4565b616213846160f6565b81016020851015616222578190505b61623661622e856160f6565b8301826161d6565b50505b505050565b5f82821c905092915050565b5f6162595f198460080261623e565b1980831691505092915050565b5f616271838361624a565b9150826002028217905092915050565b61628b83836160da565b67ffffffffffffffff8111156162a4576162a3615148565b5b6162ae8254615bde565b6162b98282856161f8565b5f601f8311600181146162e6575f84156162d4578287013590505b6162de8582616266565b865550616345565b601f1984166162f4866160e4565b5f5b8281101561631b578489013582556001820191506020850194506020810190506162f6565b868310156163385784890135616334601f89168261624a565b8355505b6001600288020188555050505b50505050505050565b616359838383616281565b505050565b5f81015f830161636e8185616078565b61637981838661634e565b50505050600181016020830161638f8185616078565b61639a81838661634e565b505050505050565b6163ac828261635e565b5050565b5f604083016163c15f840184615c27565b8583035f8701526163d3838284615c89565b925050506163e46020840184615c27565b85830360208701526163f7838284615c89565b925050508091505092915050565b5f6164136020840184614bf7565b905092915050565b60a0820161642b5f830183616405565b6164375f850182614e98565b506164456020830183616405565b6164526020850182614e98565b506164606040830183616405565b61646d6040850182614e98565b5061647b6060830183616405565b6164886060850182614e98565b506164966080830183616405565b6164a36080850182614e98565b50505050565b5f610120820190508181035f8301526164c2818b6163b0565b90506164d1602083018a61641b565b81810360c08301526164e481888a615d86565b905081810360e08301526164f9818688615fca565b905081810361010083015261650f8184866159f1565b90509998505050505050505050565b5f813561652a81614be1565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61656984616533565b9350801983169250808416831791505092915050565b6165888261616b565b61659b6165948261618c565b835461653e565b8255505050565b5f81356165ae81614de7565b80915050919050565b5f73ffffffffffffffffffffffffffffffffffffffff6165d684616533565b9350801983169250808416831791505092915050565b5f6166066166016165fc84614d09565b616162565b614d09565b9050919050565b5f616617826165ec565b9050919050565b5f6166288261660d565b9050919050565b5f819050919050565b6166418261661e565b61665461664d8261662f565b83546165b7565b8255505050565b5f81015f83018061666b8161651e565b9050616677818461657f565b50505060018101602083018061668c816165a2565b90506166988184616638565b5050506002810160408301806166ad816165a2565b90506166b98184616638565b50505060038101606083016166ce8185616078565b6166d981838661634e565b5050505060048101608083016166ef8185616078565b6166fa81838661634e565b505050505050565b61670c828261665b565b5050565b5f60a083016167215f840184616405565b61672d5f860182614e98565b5061673b6020840184615894565b6167486020860182614d39565b506167566040840184615894565b6167636040860182614d39565b506167716060840184615c27565b8583036060870152616784838284615c89565b925050506167956080840184615c27565b85830360808701526167a8838284615c89565b925050508091505092915050565b5f6020820190508181035f8301526167ce8184616710565b905092915050565b5f823560016060038336030381126167f1576167f061606c565b5b80830191505092915050565b5f80833560016020038436030381126168195761681861606c565b5b80840192508235915067ffffffffffffffff82111561683b5761683a616070565b5b60208301925060018202360383131561685757616856616074565b5b509250929050565b5f82905092915050565b5f819050815f5260205f209050919050565b601f8211156168bc5761688d81616869565b616896846160f6565b810160208510156168a5578190505b6168b96168b1856160f6565b8301826161d6565b50505b505050565b6168cb838361685f565b67ffffffffffffffff8111156168e4576168e3615148565b5b6168ee8254615bde565b6168f982828561687b565b5f601f831160018114616926575f8415616914578287013590505b61691e8582616266565b865550616985565b601f19841661693486616869565b5f5b8281101561695b57848901358255600182019150602085019450602081019050616936565b868310156169785784890135616974601f89168261624a565b8355505b6001600288020188555050505b50505050505050565b6169998383836168c1565b505050565b5f81015f8301806169ae816165a2565b90506169ba8184616638565b5050506001810160208301806169cf816165a2565b90506169db8184616638565b50505060028101604083016169f081856167fd565b6169fb81838661698e565b505050505050565b616a0d828261699e565b5050565b5f604082019050616a245f83018561501b565b616a31602083018461501b565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b616a6e816152c6565b8114616a78575f80fd5b50565b5f81519050616a8981616a65565b92915050565b5f60208284031215616aa457616aa3614b24565b5b5f616ab184828501616a7b565b91505092915050565b5f82356001608003833603038112616ad557616ad461606c565b5b80830191505092915050565b5f81015f830180616af1816165a2565b9050616afd8184616638565b505050600181016020830180616b12816165a2565b9050616b1e8184616638565b5050506002810160408301616b338185616078565b616b3e81838661634e565b505050506003810160608301616b548185616078565b616b5f81838661634e565b505050505050565b616b718282616ae1565b5050565b5f82356001606003833603038112616b9057616b8f61606c565b5b80830191505092915050565b5f81015f830180616bac816165a2565b9050616bb88184616638565b505050600181016020830180616bcd816165a2565b9050616bd98184616638565b5050506002810160408301616bee8185616078565b616bf981838661634e565b505050505050565b616c0b8282616b9c565b5050565b5f81905092915050565b5f616c2382615601565b616c2d8185616c0f565b9350616c3d818560208601614c50565b80840191505092915050565b5f616c548284616c19565b91508190509291505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80Qal_b\0\x01\xEB_9_\x81\x81a6\xF6\x01R\x81\x81a7K\x01Ra9\x05\x01Ral__\xF3\xFE`\x80`@R`\x046\x10a\x02\x87W_5`\xE0\x1C\x80cy\x8BX\xA6\x11a\x01YW\x80c\xBF\xF3\xAA\xBA\x11a\0\xC0W\x80c\xE3\x0C9x\x11a\0yW\x80c\xE3\x0C9x\x14a\tuW\x80c\xE3\xB2\xA8t\x14a\t\x9FW\x80c\xE5'^\xAF\x14a\t\xDBW\x80c\xEB\x84<\xF6\x14a\n\x17W\x80c\xEFi\x97\xF9\x14a\n?W\x80c\xF2\xFD\xE3\x8B\x14a\n{Wa\x02\x87V[\x80c\xBF\xF3\xAA\xBA\x14a\x08GW\x80c\xC2\xB4)\x86\x14a\x08\x83W\x80c\xC8\x0B3\xCA\x14a\x08\xADW\x80c\xCBZ\xA7\xE9\x14a\x08\xD5W\x80c\xD1\x0F\x7F\xF9\x14a\t\x11W\x80c\xD5\xE1k}\x14a\tMWa\x02\x87V[\x80c\x91d\xD0\xAE\x11a\x01\x12W\x80c\x91d\xD0\xAE\x14a\x07aW\x80c\x9AZ;\xC4\x14a\x07\x8BW\x80c\xAD<\xB1\xCC\x14a\x07\xA1W\x80c\xB4r+\xC4\x14a\x07\xCBW\x80c\xBA\x1F1\xD2\x14a\x07\xF5W\x80c\xBBY\xE3b\x14a\x08\x1FWa\x02\x87V[\x80cy\x8BX\xA6\x14a\x06}W\x80cy\xBAP\x97\x14a\x06\x93W\x80c~\xAA\xC8\xF2\x14a\x06\xA9W\x80c\x83\xBB.W\x14a\x06\xD3W\x80c\x88-}\xD3\x14a\x06\xFBW\x80c\x8D\xA5\xCB[\x14a\x077Wa\x02\x87V[\x80c-\xD3\xED\xFE\x11a\x01\xFDW\x80cS\xDA\x92F\x11a\x01\xB6W\x80cS\xDA\x92F\x14a\x05\x87W\x80c[\xAC\xE7\xFF\x14a\x05\xAFW\x80cg\x99\xEFR\x14a\x05\xEBW\x80cqP\x18\xA6\x14a\x06\x15W\x80ct \xF3\xD4\x14a\x06+W\x80cw-/\xE9\x14a\x06UWa\x02\x87V[\x80c-\xD3\xED\xFE\x14a\x04wW\x80c.-:\x82\x14a\x04\xB3W\x80cF\xFB\xF6\x8E\x14a\x04\xDBW\x80cH\x14La\x14a\x05\x17W\x80cO\x1E\xF2\x86\x14a\x05AW\x80cR\xD1\x90-\x14a\x05]Wa\x02\x87V[\x80c%\x85\xBBe\x11a\x02OW\x80c%\x85\xBBe\x14a\x03kW\x80c&\xCF]\xEF\x14a\x03\x95W\x80c'a,\xF6\x14a\x03\xBFW\x80c*8\x89\x98\x14a\x03\xE7W\x80c*\x8B\x9D\xE9\x14a\x04\x11W\x80c+\x10\x1C\x03\x14a\x04;Wa\x02\x87V[\x80c\x01=\xC2\x1E\x14a\x02\x8BW\x80c\x07$\xDD#\x14a\x02\xB3W\x80c\r\x8En,\x14a\x02\xDBW\x80c\x1E\xA5\xBDB\x14a\x03\x05W\x80c =\x01\x14\x14a\x03/W[_\x80\xFD[4\x80\x15a\x02\x96W_\x80\xFD[Pa\x02\xB1`\x04\x806\x03\x81\x01\x90a\x02\xAC\x91\x90aK\x8DV[a\n\xA3V[\0[4\x80\x15a\x02\xBEW_\x80\xFD[Pa\x02\xD9`\x04\x806\x03\x81\x01\x90a\x02\xD4\x91\x90aL\x0BV[a\r?V[\0[4\x80\x15a\x02\xE6W_\x80\xFD[Pa\x02\xEFa\r\x8AV[`@Qa\x02\xFC\x91\x90aL\xC0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x10W_\x80\xFD[Pa\x03\x19a\x0E\x05V[`@Qa\x03&\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03:W_\x80\xFD[Pa\x03U`\x04\x806\x03\x81\x01\x90a\x03P\x91\x90aN\x11V[a\x0E\x9EV[`@Qa\x03b\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03vW_\x80\xFD[Pa\x03\x7Fa\x0E\xFEV[`@Qa\x03\x8C\x91\x90aO\xFBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xA0W_\x80\xFD[Pa\x03\xA9a\x11;V[`@Qa\x03\xB6\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xCAW_\x80\xFD[Pa\x03\xE5`\x04\x806\x03\x81\x01\x90a\x03\xE0\x91\x90aP\x98V[a\x11RV[\0[4\x80\x15a\x03\xF2W_\x80\xFD[Pa\x03\xFBa\x12\xDCV[`@Qa\x04\x08\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x1CW_\x80\xFD[Pa\x04%a\x12\xF3V[`@Qa\x042\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04FW_\x80\xFD[Pa\x04a`\x04\x806\x03\x81\x01\x90a\x04\\\x91\x90aN\x11V[a\x13\x8CV[`@Qa\x04n\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x82W_\x80\xFD[Pa\x04\x9D`\x04\x806\x03\x81\x01\x90a\x04\x98\x91\x90aN\x11V[a\x13\xECV[`@Qa\x04\xAA\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xBEW_\x80\xFD[Pa\x04\xD9`\x04\x806\x03\x81\x01\x90a\x04\xD4\x91\x90aL\x0BV[a\x14LV[\0[4\x80\x15a\x04\xE6W_\x80\xFD[Pa\x05\x01`\x04\x806\x03\x81\x01\x90a\x04\xFC\x91\x90aN\x11V[a\x14\x97V[`@Qa\x05\x0E\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\"W_\x80\xFD[Pa\x05+a\x15+V[`@Qa\x058\x91\x90aQ$V[`@Q\x80\x91\x03\x90\xF3[a\x05[`\x04\x806\x03\x81\x01\x90a\x05V\x91\x90aRlV[a\x16qV[\0[4\x80\x15a\x05hW_\x80\xFD[Pa\x05qa\x16\x90V[`@Qa\x05~\x91\x90aR\xDEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x92W_\x80\xFD[Pa\x05\xAD`\x04\x806\x03\x81\x01\x90a\x05\xA8\x91\x90aR\xF7V[a\x16\xC1V[\0[4\x80\x15a\x05\xBAW_\x80\xFD[Pa\x05\xD5`\x04\x806\x03\x81\x01\x90a\x05\xD0\x91\x90aN\x11V[a\x19|V[`@Qa\x05\xE2\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xF6W_\x80\xFD[Pa\x05\xFFa\x19\xDCV[`@Qa\x06\x0C\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06 W_\x80\xFD[Pa\x06)a\x19\xF3V[\0[4\x80\x15a\x066W_\x80\xFD[Pa\x06?a\x1A\x06V[`@Qa\x06L\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06`W_\x80\xFD[Pa\x06{`\x04\x806\x03\x81\x01\x90a\x06v\x91\x90aL\x0BV[a\x1A\x9FV[\0[4\x80\x15a\x06\x88W_\x80\xFD[Pa\x06\x91a\x1A\xEAV[\0[4\x80\x15a\x06\x9EW_\x80\xFD[Pa\x06\xA7a\x1B\xFEV[\0[4\x80\x15a\x06\xB4W_\x80\xFD[Pa\x06\xBDa\x1C\x8CV[`@Qa\x06\xCA\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xDEW_\x80\xFD[Pa\x06\xF9`\x04\x806\x03\x81\x01\x90a\x06\xF4\x91\x90aS\xE2V[a\x1D%V[\0[4\x80\x15a\x07\x06W_\x80\xFD[Pa\x07!`\x04\x806\x03\x81\x01\x90a\x07\x1C\x91\x90aN\x11V[a\x1F\xC5V[`@Qa\x07.\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07BW_\x80\xFD[Pa\x07Ka %V[`@Qa\x07X\x91\x90aTNV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07lW_\x80\xFD[Pa\x07ua ZV[`@Qa\x07\x82\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x96W_\x80\xFD[Pa\x07\x9Fa \xF3V[\0[4\x80\x15a\x07\xACW_\x80\xFD[Pa\x07\xB5a\"IV[`@Qa\x07\xC2\x91\x90aL\xC0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\xD6W_\x80\xFD[Pa\x07\xDFa\"\x82V[`@Qa\x07\xEC\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\0W_\x80\xFD[Pa\x08\ta\"\x99V[`@Qa\x08\x16\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08*W_\x80\xFD[Pa\x08E`\x04\x806\x03\x81\x01\x90a\x08@\x91\x90aT\xA7V[a#2V[\0[4\x80\x15a\x08RW_\x80\xFD[Pa\x08m`\x04\x806\x03\x81\x01\x90a\x08h\x91\x90aL\x0BV[a%TV[`@Qa\x08z\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\x8EW_\x80\xFD[Pa\x08\x97a%\x88V[`@Qa\x08\xA4\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\xB8W_\x80\xFD[Pa\x08\xD3`\x04\x806\x03\x81\x01\x90a\x08\xCE\x91\x90aU\xBAV[a%\x9FV[\0[4\x80\x15a\x08\xE0W_\x80\xFD[Pa\x08\xFB`\x04\x806\x03\x81\x01\x90a\x08\xF6\x91\x90aN\x11V[a'MV[`@Qa\t\x08\x91\x90aV\xA0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\x1CW_\x80\xFD[Pa\t7`\x04\x806\x03\x81\x01\x90a\t2\x91\x90aL\x0BV[a(\xEBV[`@Qa\tD\x91\x90aW:V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\tXW_\x80\xFD[Pa\ts`\x04\x806\x03\x81\x01\x90a\tn\x91\x90aL\x0BV[a+\x06V[\0[4\x80\x15a\t\x80W_\x80\xFD[Pa\t\x89a+QV[`@Qa\t\x96\x91\x90aTNV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xAAW_\x80\xFD[Pa\t\xC5`\x04\x806\x03\x81\x01\x90a\t\xC0\x91\x90aN\x11V[a+\x86V[`@Qa\t\xD2\x91\x90aW\xC1V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xE6W_\x80\xFD[Pa\n\x01`\x04\x806\x03\x81\x01\x90a\t\xFC\x91\x90aN\x11V[a-\xB4V[`@Qa\n\x0E\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\"W_\x80\xFD[Pa\n=`\x04\x806\x03\x81\x01\x90a\n8\x91\x90aL\x0BV[a.\x14V[\0[4\x80\x15a\nJW_\x80\xFD[Pa\ne`\x04\x806\x03\x81\x01\x90a\n`\x91\x90aN\x11V[a._V[`@Qa\nr\x91\x90aX.V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\x86W_\x80\xFD[Pa\n\xA1`\x04\x806\x03\x81\x01\x90a\n\x9C\x91\x90aN\x11V[a/\xFDV[\0[a\n\xABa0\xB6V[_a\n\xB4a1=V[\x90P_\x81`\x12\x01\x80T\x90P\x90P_[\x81\x81\x10\x15a\x0C\xD7W_\x83`\x14\x01_\x85`\x12\x01\x84\x81T\x81\x10a\n\xE7Wa\n\xE6aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x15\x01_\x85`\x13\x01\x84\x81T\x81\x10a\x0BzWa\x0ByaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82`\x11\x01_\x84`\x12\x01\x83\x81T\x81\x10a\x0C\x0CWa\x0C\x0BaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x80\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x01\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01_a\x0C\xC8\x91\x90aI\x0EV[PP\x80\x80`\x01\x01\x91PPa\n\xC3V[P\x81`\x12\x01_a\x0C\xE7\x91\x90aIKV[\x81`\x13\x01_a\x0C\xF6\x91\x90aIKV[a\r\0\x84\x84a1dV[\x7Fl\xDC\x1A\xA7n\x1E\xBA\xCDg\xC8\x1B\xE0\xDC\xF9`;]\xFB\xEBM\xD8\x01\xAB!A\x14\xAC\xB56\xF1\x10h\x84\x84`@Qa\r1\x92\x91\x90aZcV[`@Q\x80\x91\x03\x90\xA1PPPPV[a\rGa0\xB6V[a\rP\x81a4\xBBV[\x7F0\xC9\xB1\xD0\x04\xF5~\xAE<l\xC3\xA3u+\xCBL\x8E\xA2\xE5|\x82A\xA7\x82\xAA\x9Be\xFB\xC6\x04\xEC[\x81`@Qa\r\x7F\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FGatewayConfig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\r\xCB_a5_V[a\r\xD5`\x03a5_V[a\r\xDE_a5_V[`@Q` \x01a\r\xF1\x94\x93\x92\x91\x90a[SV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x0E\x10a1=V[\x90P\x80`\r\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\x93W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0EJW[PPPPP\x91PP\x90V[_\x80a\x0E\xA8a1=V[\x90P\x80`\x03\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[``_a\x0F\ta1=V[\x90P\x80`\x10\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x111W\x83\x82\x90_R` _ \x90`\x05\x02\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x03\x82\x01\x80Ta\x10\x12\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10>\x90a[\xDEV[\x80\x15a\x10\x89W\x80`\x1F\x10a\x10`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x89V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta\x10\xA2\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xCE\x90a[\xDEV[\x80\x15a\x11\x19W\x80`\x1F\x10a\x10\xF0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\x19V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0F.V[PPPP\x91PP\x90V[_\x80a\x11Ea1=V[\x90P\x80`\x07\x01T\x91PP\x90V[`\x04_a\x11]a6)V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x11\xA5WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x11\xDCW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x12*a1=V[\x90Pa\x12J\x85\x85\x83`\x07\x01T\x84`\x08\x01T\x85`\t\x01T\x86`\x16\x01Ta\x16\xC1V[\x7FC\t)\xCF\xFD\xF4;6\x81\xBB\x03\xEA\xE5\x80'E\xE35\xEC\x01Y\x96\x8D0\x1E*\x8E= \x9E\xEF\xB8\x85\x85`@Qa\x12{\x92\x91\x90a]\xF8V[`@Q\x80\x91\x03\x90\xA1P_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x12\xCE\x91\x90a^<V[`@Q\x80\x91\x03\x90\xA1PPPPV[_\x80a\x12\xE6a1=V[\x90P\x80`\x08\x01T\x91PP\x90V[``_a\x12\xFEa1=V[\x90P\x80`\x12\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x13\x81W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x138W[PPPPP\x91PP\x90V[_\x80a\x13\x96a1=V[\x90P\x80`\x0B\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x13\xF6a1=V[\x90P\x80`\n\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[a\x14Ta0\xB6V[a\x14]\x81a6PV[\x7F\xE4\x18\x02\xAFrW)\xAD\xCB\x8C\x15\x1E)78\n%\xC6\x91Uu~:\xF5\xD3\x97\x9A\xDA\xB5\x03X\0\x81`@Qa\x14\x8C\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[_s\xC3\xF9\xE1\xD2|\xD1\x04\x027[|\xD27\xD5~\x0FH\x88\xC1\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xFB\xF6\x8E\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xE5\x91\x90aTNV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15$\x91\x90a^\x7FV[\x90P\x91\x90PV[a\x153aIiV[_a\x15<a1=V[\x90P\x80_\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x15[\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x87\x90a[\xDEV[\x80\x15a\x15\xD2W\x80`\x1F\x10a\x15\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xD2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x15\xEB\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\x17\x90a[\xDEV[\x80\x15a\x16bW\x80`\x1F\x10a\x169Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16bV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x90V[a\x16ya6\xF4V[a\x16\x82\x82a7\xDAV[a\x16\x8C\x82\x82a7\xE5V[PPV[_a\x16\x99a9\x03V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[a\x16\xC9a0\xB6V[_a\x16\xD2a1=V[\x90P_\x81`\x05\x01\x80T\x90P\x90P_[\x81\x81\x10\x15a\x19\x04W_\x83`\x02\x01_\x85`\x05\x01\x84\x81T\x81\x10a\x17\x05Wa\x17\x04aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x03\x01_\x85`\x06\x01\x84\x81T\x81\x10a\x17\x98Wa\x17\x97aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82`\x04\x01_\x84`\x05\x01\x83\x81T\x81\x10a\x18*Wa\x18)aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x80\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x01\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01_a\x18\xE6\x91\x90aI\x83V[`\x03\x82\x01_a\x18\xF5\x91\x90aI\x83V[PP\x80\x80`\x01\x01\x91PPa\x16\xE1V[P\x81`\x05\x01_a\x19\x14\x91\x90aIKV[\x81`\x06\x01_a\x19#\x91\x90aIKV[a\x191\x88\x88\x88\x88\x88\x88a9\x8AV[\x7F%\xD1\xEAdq(\xB5mG\xE6E4\xCD\x0FZ\x86\xD3 \x7Fg\xB0H\x95I[f\xDC\r\xB8z\x0C\xA7\x88\x88\x88\x88\x88\x88`@Qa\x19j\x96\x95\x94\x93\x92\x91\x90a^\xAAV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x19\x86a1=V[\x90P\x80`\x14\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x19\xE6a1=V[\x90P\x80`\x17\x01T\x91PP\x90V[a\x19\xFBa0\xB6V[a\x1A\x04_a=\tV[V[``_a\x1A\x11a1=V[\x90P\x80`\x05\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1A\x94W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1AKW[PPPPP\x91PP\x90V[a\x1A\xA7a0\xB6V[a\x1A\xB0\x81a=FV[\x7F5q\x17*I\xE7-w$\xBE8L\xDDY\xF4\xF2\x1A!lp5.\xA5\x9C\xB0%C\xFCv0\x847\x81`@Qa\x1A\xDF\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[a\x1A\xF2a0\xB6V[s\x87\xA5\xB1\x15*\xA5\x17(%\x8D\xBC\x1A\xA5Kj\x83\xDC\xD1\xD3\xDDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c?K\xA8:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BKW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B]W=_\x80>=_\xFD[PPPPs3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNns\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c?K\xA8:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xBAW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xCCW=_\x80>=_\xFD[PPPP\x7F\xBEOe]\xAA\xE0\xDB\xAE\xF6:kR\\\xAB/\xA6\xAC\xE4\xAA[\x94\xB8\x83K$\x117\xCD\xFEs\xA5\xB0`@Q`@Q\x80\x91\x03\x90\xA1V[_a\x1C\x07a=\xB0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C(a+QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1C\x80W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Cw\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[a\x1C\x89\x81a=\tV[PV[``_a\x1C\x97a1=V[\x90P\x80`\x06\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1D\x1AW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1C\xD1W[PPPPP\x91PP\x90V[a\x1D-a0\xB6V[_a\x1D6a1=V[\x90P_\x81`\r\x01\x80T\x90P\x90P_[\x81\x81\x10\x15a\x1FYW_\x83`\n\x01_\x85`\r\x01\x84\x81T\x81\x10a\x1DiWa\x1DhaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x0B\x01_\x85`\x0E\x01\x84\x81T\x81\x10a\x1D\xFCWa\x1D\xFBaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82`\x0C\x01_\x84`\r\x01\x83\x81T\x81\x10a\x1E\x8EWa\x1E\x8DaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x80\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x01\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01_a\x1FJ\x91\x90aI\x83V[PP\x80\x80`\x01\x01\x91PPa\x1DEV[P\x81`\r\x01_a\x1Fi\x91\x90aIKV[\x81`\x0E\x01_a\x1Fx\x91\x90aIKV[a\x1F\x83\x85\x85\x85a=\xB7V[\x7F\xFF\xE2\x0B\xDB\x85^QN\x94\x14w\x02\x92&\x90\xCF\x1D\xA1\x0B\xDD\x18\xBF\x1Fb\x15\x02|\x93\xAC\x05\xD4U\x85\x85\x85`@Qa\x1F\xB6\x93\x92\x91\x90a`<V[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a\x1F\xCFa1=V[\x90P\x80`\x15\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a /aA\x18V[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[``_a ea1=V[\x90P\x80`\x0E\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a \xE8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a \x9FW[PPPPP\x91PP\x90V[a \xFC3a\x14\x97V[a!=W3`@Q\x7F j4n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!4\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[s\x87\xA5\xB1\x15*\xA5\x17(%\x8D\xBC\x1A\xA5Kj\x83\xDC\xD1\xD3\xDDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x84V\xCBY`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\x96W_\x80\xFD[PZ\xF1\x15\x80\x15a!\xA8W=_\x80>=_\xFD[PPPPs3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNns\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x84V\xCBY`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"\x05W_\x80\xFD[PZ\xF1\x15\x80\x15a\"\x17W=_\x80>=_\xFD[PPPP\x7F\x13\xDB\xE8\x822\x19\xE2&\xDD\x05%\xAE\xB0q\xE1\xD2g\x9F\x898+\xA7\x99\xF7\xF6D\x86~e\xB6\xF3\xA6`@Q`@Q\x80\x91\x03\x90\xA1V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\"\x8Ca1=V[\x90P\x80`\x16\x01T\x91PP\x90V[``_a\"\xA4a1=V[\x90P\x80`\x13\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a#'W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\"\xDEW[PPPPP\x91PP\x90V[`\x01a#<aA?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#}W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04_a#\x88a6)V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a#\xD0WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a$\x07W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa$\\a$Wa %V[aAcV[_a$ea1=V[\x90P\x8A\x81_\x01\x81\x81a$w\x91\x90ac\xA2V[\x90PPa$\x97\x89\x89\x8C_\x015\x8D` \x015\x8E`@\x015\x8F``\x015a9\x8AV[a$\xA6\x87\x87\x8C`\x80\x015a=\xB7V[a$\xB0\x85\x85a1dV[\x7F\xB2\xCB\xE6^\xA3\x08\xBF\xE4\xB9C\x18\x19\xA3\x16\x8DTOF\xBA4K\x1Ey\xF9/\x97?\xCF\xF4:\xAE;\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B`@Qa$\xED\x98\x97\x96\x95\x94\x93\x92\x91\x90ad\xA9V[`@Q\x80\x91\x03\x90\xA1P_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa%@\x91\x90a^<V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPV[_\x80a%^a1=V[\x90P\x80`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a%\x92a1=V[\x90P\x80`\t\x01T\x91PP\x90V[a%\xA7a0\xB6V[_\x81_\x015\x03a%\xE3W`@Q\x7F\"\xF7?\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x015\x11\x15a&8W\x80_\x015`@Q\x7FAx\xDEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&/\x91\x90aP*V[`@Q\x80\x91\x03\x90\xFD[_a&Aa1=V[\x90P\x80`\x0F\x01_\x83_\x015\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a&\xABW\x81_\x015`@Q\x7F\x96\xA5h(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xA2\x91\x90aP*V[`@Q\x80\x91\x03\x90\xFD[\x80`\x10\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x05\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a&\xE2\x91\x90ag\x02V[PP`\x01\x81`\x0F\x01_\x84_\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Ffv\x93A\xEF\xFD&\x8F\xC4\xE9\xA9\xC8\xF2{\xFC\x96\x85\x07\xB5\x19\xB0\xDD\xB9\xB4\xAD=\xED_\x03\x01h7\x82`@Qa'A\x91\x90ag\xB6V[`@Q\x80\x91\x03\x90\xA1PPV[a'UaI\xC0V[_a'^a1=V[\x90P\x80`\x11\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta(c\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x8F\x90a[\xDEV[\x80\x15a(\xDAW\x80`\x1F\x10a(\xB1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xDAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[a(\xF3aJ\x0BV[_a(\xFCa1=V[\x90P\x80`\x10\x01\x83\x81T\x81\x10a)\x14Wa)\x13aXNV[[\x90_R` _ \x90`\x05\x02\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x03\x82\x01\x80Ta)\xEE\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\x1A\x90a[\xDEV[\x80\x15a*eW\x80`\x1F\x10a*<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*eV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*HW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta*~\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xAA\x90a[\xDEV[\x80\x15a*\xF5W\x80`\x1F\x10a*\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[a+\x0Ea0\xB6V[a+\x17\x81aAwV[\x7Fz.\xF7\xDC\x89@\n\x8A\xD9+\xB4\xCC\xF4MH&$\xB4\x0F\xE7kf\x97~\x85\xEDja\x8E./\xC7\x81`@Qa+F\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[_\x80a+[aB\x1BV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[a+\x8EaJcV[_a+\x97a1=V[\x90P\x80`\x04\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta,\x9C\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,\xC8\x90a[\xDEV[\x80\x15a-\x13W\x80`\x1F\x10a,\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\x13V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,\xF6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta-,\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta-X\x90a[\xDEV[\x80\x15a-\xA3W\x80`\x1F\x10a-zWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\xA3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a-\x86W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[_\x80a-\xBEa1=V[\x90P\x80`\x02\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[a.\x1Ca0\xB6V[a.%\x81aBBV[\x7F\x83~\ne(\xDA\xDF\xA2\xDCy&\x92\xC5\x18.R\xA9\xF5\xBB\xDE\xED{#r\x92z&\xC6\x95\x83\x96\x13\x81`@Qa.T\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[a.gaJ\xB5V[_a.pa1=V[\x90P\x80`\x0C\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta/u\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xA1\x90a[\xDEV[\x80\x15a/\xECW\x80`\x1F\x10a/\xC3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a/\xECV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a/\xCFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[a0\x05a0\xB6V[_a0\x0EaB\x1BV[\x90P\x81\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a0pa %V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PPV[a0\xBEa=\xB0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a0\xDCa %V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a1;Wa0\xFFa=\xB0V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a12\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x86\xD3\x07\n\x89\x93\xF6\xB2\t\xBE\xE6\x18Q\x86\xD3\x8A\x07\xFC\xE8\xBB\xD9|u\r\x93DQ\xB7/5\xB4\0\x90P\x90V[_\x82\x82\x90P\x03a1\xA0W`@Q\x7F\xCA\xD1\xD54\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a1\xA9a1=V[\x90P_[\x83\x83\x90P\x81\x10\x15a4\xB5W\x83\x83\x82\x81\x81\x10a1\xCBWa1\xCAaXNV[[\x90P` \x02\x81\x01\x90a1\xDD\x91\x90ag\xD6V[\x82`\x11\x01_\x86\x86\x85\x81\x81\x10a1\xF5Wa1\xF4aXNV[[\x90P` \x02\x81\x01\x90a2\x07\x91\x90ag\xD6V[_\x01` \x81\x01\x90a2\x18\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a2]\x91\x90aj\x03V[\x90PP\x81`\x12\x01\x84\x84\x83\x81\x81\x10a2wWa2vaXNV[[\x90P` \x02\x81\x01\x90a2\x89\x91\x90ag\xD6V[_\x01` \x81\x01\x90a2\x9A\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x14\x01_\x86\x86\x85\x81\x81\x10a3\x11Wa3\x10aXNV[[\x90P` \x02\x81\x01\x90a3#\x91\x90ag\xD6V[_\x01` \x81\x01\x90a34\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`\x13\x01\x84\x84\x83\x81\x81\x10a3\x9AWa3\x99aXNV[[\x90P` \x02\x81\x01\x90a3\xAC\x91\x90ag\xD6V[` \x01` \x81\x01\x90a3\xBE\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x15\x01_\x86\x86\x85\x81\x81\x10a45Wa44aXNV[[\x90P` \x02\x81\x01\x90a4G\x91\x90ag\xD6V[` \x01` \x81\x01\x90a4Y\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa1\xADV[PPPPV[_a4\xC4a1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P_\x83\x03a5\nW`@Q\x7F>\xE5\x07t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a5QW\x82\x81`@Q\x7F\x0Fi\xCB\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5H\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x16\x01\x81\x90UPPPPV[``_`\x01a5m\x84aB\xE6V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x8BWa5\x8AaQHV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a5\xBDW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a6\x1EW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a6\x13Wa6\x12aj8V[[\x04\x94P_\x85\x03a5\xCAW[\x81\x93PPPP\x91\x90PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_a6Ya1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P_\x83\x03a6\x9FW`@Q\x7F\xB1\xAE\x92\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a6\xE6W\x82\x81`@Q\x7F\x84 \x8F#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a6\xDD\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x08\x01\x81\x90UPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a7\xA1WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a7\x88aD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a7\xD8W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a7\xE2a0\xB6V[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a8MWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8J\x91\x90aj\x8FV[`\x01[a8\x8EW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\x85\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a8\xF4W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\xEB\x91\x90aR\xDEV[`@Q\x80\x91\x03\x90\xFD[a8\xFE\x83\x83aD\x8AV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a9\x88W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x86\x86\x90P\x03a9\xC6W`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a9\xCFa1=V[\x90P_[\x87\x87\x90P\x81\x10\x15a<\xDBW`\x01\x82`\x02\x01_\x8A\x8A\x85\x81\x81\x10a9\xF8Wa9\xF7aXNV[[\x90P` \x02\x81\x01\x90a:\n\x91\x90aj\xBAV[_\x01` \x81\x01\x90a:\x1B\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x87\x87\x82\x81\x81\x10a:}Wa:|aXNV[[\x90P` \x02\x81\x01\x90a:\x8F\x91\x90aj\xBAV[\x82`\x04\x01_\x8A\x8A\x85\x81\x81\x10a:\xA7Wa:\xA6aXNV[[\x90P` \x02\x81\x01\x90a:\xB9\x91\x90aj\xBAV[_\x01` \x81\x01\x90a:\xCA\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a;\x0F\x91\x90akgV[\x90PP\x81`\x05\x01\x88\x88\x83\x81\x81\x10a;)Wa;(aXNV[[\x90P` \x02\x81\x01\x90a;;\x91\x90aj\xBAV[_\x01` \x81\x01\x90a;L\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x03\x01_\x8A\x8A\x85\x81\x81\x10a;\xC3Wa;\xC2aXNV[[\x90P` \x02\x81\x01\x90a;\xD5\x91\x90aj\xBAV[` \x01` \x81\x01\x90a;\xE7\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`\x06\x01\x88\x88\x83\x81\x81\x10a<MWa<LaXNV[[\x90P` \x02\x81\x01\x90a<_\x91\x90aj\xBAV[` \x01` \x81\x01\x90a<q\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa9\xD3V[Pa<\xE5\x85a=FV[a<\xEE\x84a6PV[a<\xF7\x83aBBV[a=\0\x82a4\xBBV[PPPPPPPV[_a=\x12aB\x1BV[\x90P\x80_\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua=B\x82aD\xFCV[PPV[_a=Oa1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P\x80\x83\x10a=\xA2W\x82\x81`@Q\x7F\x90~f\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\x99\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x07\x01\x81\x90UPPPPV[_3\x90P\x90V[_\x83\x83\x90P\x03a=\xF3W`@Q\x7F\x8A\xF0\x82\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a=\xFCa1=V[\x90P_[\x84\x84\x90P\x81\x10\x15aA\x08W`\x01\x82`\n\x01_\x87\x87\x85\x81\x81\x10a>%Wa>$aXNV[[\x90P` \x02\x81\x01\x90a>7\x91\x90akuV[_\x01` \x81\x01\x90a>H\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x84\x84\x82\x81\x81\x10a>\xAAWa>\xA9aXNV[[\x90P` \x02\x81\x01\x90a>\xBC\x91\x90akuV[\x82`\x0C\x01_\x87\x87\x85\x81\x81\x10a>\xD4Wa>\xD3aXNV[[\x90P` \x02\x81\x01\x90a>\xE6\x91\x90akuV[_\x01` \x81\x01\x90a>\xF7\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a?<\x91\x90al\x01V[\x90PP\x81`\r\x01\x85\x85\x83\x81\x81\x10a?VWa?UaXNV[[\x90P` \x02\x81\x01\x90a?h\x91\x90akuV[_\x01` \x81\x01\x90a?y\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x0B\x01_\x87\x87\x85\x81\x81\x10a?\xF0Wa?\xEFaXNV[[\x90P` \x02\x81\x01\x90a@\x02\x91\x90akuV[` \x01` \x81\x01\x90a@\x14\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`\x0E\x01\x85\x85\x83\x81\x81\x10a@zWa@yaXNV[[\x90P` \x02\x81\x01\x90a@\x8C\x91\x90akuV[` \x01` \x81\x01\x90a@\x9E\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa>\0V[PaA\x12\x82aAwV[PPPPV[_\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90P\x90V[_aAHa6)V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aAkaE\xCDV[aAt\x81aF\rV[PV[_aA\x80a1=V[\x90P_\x81`\x0E\x01\x80T\x90P\x90P_\x83\x03aA\xC6W`@Q\x7F\xB6\r$A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15aB\rW\x82\x81`@Q\x7F\x97\xBE\xAB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\x04\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x17\x01\x81\x90UPPPPV[_\x7F#~\x15\x82\"\xE3\xE6\x96\x8Br\xB9\xDB\r\x80C\xAA\xCF\x07J\xD9\xF6P\xF0\xD1`kM\x82\xEEC,\0\x90P\x90V[_aBKa1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P_\x83\x03aB\x91W`@Q\x7F\xE6\nrq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15aB\xD8W\x82\x81`@Q\x7F\xD2S^\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\xCF\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\t\x01\x81\x90UPPPPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aCBWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aC8WaC7aj8V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aC\x7FWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aCuWaCtaj8V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aC\xAEWf#\x86\xF2o\xC1\0\0\x83\x81aC\xA4WaC\xA3aj8V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aC\xD7Wc\x05\xF5\xE1\0\x83\x81aC\xCDWaC\xCCaj8V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aC\xFCWa'\x10\x83\x81aC\xF2WaC\xF1aj8V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aD\x1FW`d\x83\x81aD\x15WaD\x14aj8V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aD.W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_aDc\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaF\x91V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aD\x93\x82aF\x9AV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aD\xEFWaD\xE9\x82\x82aGcV[PaD\xF8V[aD\xF7aG\xE3V[[PPV[_aE\x05aA\x18V[\x90P_\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82\x82_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPPV[aE\xD5aH\x1FV[aF\x0BW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aF\x15aE\xCDV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aF\x85W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aF|\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[aF\x8E\x81a=\tV[PV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aF\xF5W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aF\xEC\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[\x80aG!\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaF\x91V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaG\x8C\x91\x90alIV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aG\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aG\xC9V[``\x91P[P\x91P\x91PaG\xD9\x85\x83\x83aH=V[\x92PPP\x92\x91PPV[_4\x11\x15aH\x1DW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aH(a6)V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[``\x82aHRWaHM\x82aH\xCAV[aH\xC2V[_\x82Q\x14\x80\x15aHxWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aH\xBAW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aH\xB1\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaH\xC3V[[\x93\x92PPPV[_\x81Q\x11\x15aH\xDCW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80TaI\x1A\x90a[\xDEV[_\x82U\x80`\x1F\x10aI+WPaIHV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90aIG\x91\x90aK\0V[[PV[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90aIf\x91\x90aK\0V[PV[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[P\x80TaI\x8F\x90a[\xDEV[_\x82U\x80`\x1F\x10aI\xA0WPaI\xBDV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90aI\xBC\x91\x90aK\0V[[PV[`@Q\x80``\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Q\x80``\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81RP\x90V[[\x80\x82\x11\x15aK\x17W_\x81_\x90UP`\x01\x01aK\x01V[P\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aKMWaKLaK,V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aKjWaKiaK0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aK\x86WaK\x85aK4V[[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15aK\xA3WaK\xA2aK$V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xC0WaK\xBFaK(V[[aK\xCC\x85\x82\x86\x01aK8V[\x92P\x92PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aK\xEA\x81aK\xD8V[\x81\x14aK\xF4W_\x80\xFD[PV[_\x815\x90PaL\x05\x81aK\xE1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aL WaL\x1FaK$V[[_aL-\x84\x82\x85\x01aK\xF7V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aLmW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaLRV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aL\x92\x82aL6V[aL\x9C\x81\x85aL@V[\x93PaL\xAC\x81\x85` \x86\x01aLPV[aL\xB5\x81aLxV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaL\xD8\x81\x84aL\x88V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aM2\x82aM\tV[\x90P\x91\x90PV[aMB\x81aM(V[\x82RPPV[_aMS\x83\x83aM9V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aMu\x82aL\xE0V[aM\x7F\x81\x85aL\xEAV[\x93PaM\x8A\x83aL\xFAV[\x80_[\x83\x81\x10\x15aM\xBAW\x81QaM\xA1\x88\x82aMHV[\x97PaM\xAC\x83aM_V[\x92PP`\x01\x81\x01\x90PaM\x8DV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM\xDF\x81\x84aMkV[\x90P\x92\x91PPV[aM\xF0\x81aM(V[\x81\x14aM\xFAW_\x80\xFD[PV[_\x815\x90PaN\x0B\x81aM\xE7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aN&WaN%aK$V[[_aN3\x84\x82\x85\x01aM\xFDV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[aNP\x81aN<V[\x82RPPV[_` \x82\x01\x90PaNi_\x83\x01\x84aNGV[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aN\xA1\x81aK\xD8V[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aN\xC1\x82aL6V[aN\xCB\x81\x85aN\xA7V[\x93PaN\xDB\x81\x85` \x86\x01aLPV[aN\xE4\x81aLxV[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01QaO\x04_\x86\x01\x82aN\x98V[P` \x83\x01QaO\x17` \x86\x01\x82aM9V[P`@\x83\x01QaO*`@\x86\x01\x82aM9V[P``\x83\x01Q\x84\x82\x03``\x86\x01RaOB\x82\x82aN\xB7V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01RaO\\\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_aOt\x83\x83aN\xEFV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aO\x92\x82aNoV[aO\x9C\x81\x85aNyV[\x93P\x83` \x82\x02\x85\x01aO\xAE\x85aN\x89V[\x80_[\x85\x81\x10\x15aO\xE9W\x84\x84\x03\x89R\x81QaO\xCA\x85\x82aOiV[\x94PaO\xD5\x83aO|V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaO\xB1V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaP\x13\x81\x84aO\x88V[\x90P\x92\x91PPV[aP$\x81aK\xD8V[\x82RPPV[_` \x82\x01\x90PaP=_\x83\x01\x84aP\x1BV[\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aPXWaPWaK,V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aPuWaPtaK0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aP\x91WaP\x90aK4V[[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15aP\xAEWaP\xADaK$V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xCBWaP\xCAaK(V[[aP\xD7\x85\x82\x86\x01aPCV[\x92P\x92PP\x92P\x92\x90PV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01RaP\xFD\x82\x82aN\xB7V[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01RaQ\x17\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaQ<\x81\x84aP\xE3V[\x90P\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aQ~\x82aLxV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aQ\x9DWaQ\x9CaQHV[[\x80`@RPPPV[_aQ\xAFaK\x1BV[\x90PaQ\xBB\x82\x82aQuV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQ\xDAWaQ\xD9aQHV[[aQ\xE3\x82aLxV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aR\x10aR\x0B\x84aQ\xC0V[aQ\xA6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aR,WaR+aQDV[[aR7\x84\x82\x85aQ\xF0V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aRSWaRRaK,V[[\x815aRc\x84\x82` \x86\x01aQ\xFEV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aR\x82WaR\x81aK$V[[_aR\x8F\x85\x82\x86\x01aM\xFDV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\xB0WaR\xAFaK(V[[aR\xBC\x85\x82\x86\x01aR?V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aR\xD8\x81aR\xC6V[\x82RPPV[_` \x82\x01\x90PaR\xF1_\x83\x01\x84aR\xCFV[\x92\x91PPV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15aS\x11WaS\x10aK$V[[_\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS.WaS-aK(V[[aS:\x89\x82\x8A\x01aPCV[\x96P\x96PP` aSM\x89\x82\x8A\x01aK\xF7V[\x94PP`@aS^\x89\x82\x8A\x01aK\xF7V[\x93PP``aSo\x89\x82\x8A\x01aK\xF7V[\x92PP`\x80aS\x80\x89\x82\x8A\x01aK\xF7V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80\x83`\x1F\x84\x01\x12aS\xA2WaS\xA1aK,V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xBFWaS\xBEaK0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aS\xDBWaS\xDAaK4V[[\x92P\x92\x90PV[_\x80_`@\x84\x86\x03\x12\x15aS\xF9WaS\xF8aK$V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\x16WaT\x15aK(V[[aT\"\x86\x82\x87\x01aS\x8DV[\x93P\x93PP` aT5\x86\x82\x87\x01aK\xF7V[\x91PP\x92P\x92P\x92V[aTH\x81aM(V[\x82RPPV[_` \x82\x01\x90PaTa_\x83\x01\x84aT?V[\x92\x91PPV[_\x80\xFD[_`@\x82\x84\x03\x12\x15aT\x80WaT\x7FaTgV[[\x81\x90P\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15aT\x9EWaT\x9DaTgV[[\x81\x90P\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01 \x89\x8B\x03\x12\x15aT\xC4WaT\xC3aK$V[[_\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xE1WaT\xE0aK(V[[aT\xED\x8B\x82\x8C\x01aTkV[\x98PP` aT\xFE\x8B\x82\x8C\x01aT\x89V[\x97PP`\xC0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\x1FWaU\x1EaK(V[[aU+\x8B\x82\x8C\x01aPCV[\x96P\x96PP`\xE0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aUNWaUMaK(V[[aUZ\x8B\x82\x8C\x01aS\x8DV[\x94P\x94PPa\x01\0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU~WaU}aK(V[[aU\x8A\x8B\x82\x8C\x01aK8V[\x92P\x92PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_`\xA0\x82\x84\x03\x12\x15aU\xB1WaU\xB0aTgV[[\x81\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15aU\xCFWaU\xCEaK$V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xECWaU\xEBaK(V[[aU\xF8\x84\x82\x85\x01aU\x9CV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aV%\x82aV\x01V[aV/\x81\x85aV\x0BV[\x93PaV?\x81\x85` \x86\x01aLPV[aVH\x81aLxV[\x84\x01\x91PP\x92\x91PPV[_``\x83\x01_\x83\x01QaVh_\x86\x01\x82aM9V[P` \x83\x01QaV{` \x86\x01\x82aM9V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaV\x93\x82\x82aV\x1BV[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaV\xB8\x81\x84aVSV[\x90P\x92\x91PPV[_`\xA0\x83\x01_\x83\x01QaV\xD5_\x86\x01\x82aN\x98V[P` \x83\x01QaV\xE8` \x86\x01\x82aM9V[P`@\x83\x01QaV\xFB`@\x86\x01\x82aM9V[P``\x83\x01Q\x84\x82\x03``\x86\x01RaW\x13\x82\x82aN\xB7V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01RaW-\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaWR\x81\x84aV\xC0V[\x90P\x92\x91PPV[_`\x80\x83\x01_\x83\x01QaWo_\x86\x01\x82aM9V[P` \x83\x01QaW\x82` \x86\x01\x82aM9V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaW\x9A\x82\x82aN\xB7V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01RaW\xB4\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xD9\x81\x84aWZV[\x90P\x92\x91PPV[_``\x83\x01_\x83\x01QaW\xF6_\x86\x01\x82aM9V[P` \x83\x01QaX\t` \x86\x01\x82aM9V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaX!\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaXF\x81\x84aW\xE1V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aX\xA2` \x84\x01\x84aM\xFDV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aX\xD2WaX\xD1aX\xB2V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX\xFAWaX\xF9aX\xAAV[[`\x01\x82\x026\x03\x83\x13\x15aY\x10WaY\x0FaX\xAEV[[P\x92P\x92\x90PV[_aY#\x83\x85aV\x0BV[\x93PaY0\x83\x85\x84aQ\xF0V[aY9\x83aLxV[\x84\x01\x90P\x93\x92PPPV[_``\x83\x01aYU_\x84\x01\x84aX\x94V[aYa_\x86\x01\x82aM9V[PaYo` \x84\x01\x84aX\x94V[aY|` \x86\x01\x82aM9V[PaY\x8A`@\x84\x01\x84aX\xB6V[\x85\x83\x03`@\x87\x01RaY\x9D\x83\x82\x84aY\x18V[\x92PPP\x80\x91PP\x92\x91PPV[_aY\xB6\x83\x83aYDV[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12aY\xD9WaY\xD8aX\xB2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aY\xFC\x83\x85aX{V[\x93P\x83` \x84\x02\x85\x01aZ\x0E\x84aX\x8BV[\x80_[\x87\x81\x10\x15aZQW\x84\x84\x03\x89RaZ(\x82\x84aY\xBEV[aZ2\x85\x82aY\xABV[\x94PaZ=\x83aY\xE5V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaZ\x11V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaZ|\x81\x84\x86aY\xF1V[\x90P\x93\x92PPPV[_\x81\x90P\x92\x91PPV[_aZ\x99\x82aL6V[aZ\xA3\x81\x85aZ\x85V[\x93PaZ\xB3\x81\x85` \x86\x01aLPV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aZ\xF3`\x02\x83aZ\x85V[\x91PaZ\xFE\x82aZ\xBFV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a[=`\x01\x83aZ\x85V[\x91Pa[H\x82a[\tV[`\x01\x82\x01\x90P\x91\x90PV[_a[^\x82\x87aZ\x8FV[\x91Pa[i\x82aZ\xE7V[\x91Pa[u\x82\x86aZ\x8FV[\x91Pa[\x80\x82a[1V[\x91Pa[\x8C\x82\x85aZ\x8FV[\x91Pa[\x97\x82a[1V[\x91Pa[\xA3\x82\x84aZ\x8FV[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a[\xF5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\\\x08Wa\\\x07a[\xB1V[[P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a\\CWa\\BaX\xB2V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\\kWa\\jaX\xAAV[[`\x01\x82\x026\x03\x83\x13\x15a\\\x81Wa\\\x80aX\xAEV[[P\x92P\x92\x90PV[_a\\\x94\x83\x85aN\xA7V[\x93Pa\\\xA1\x83\x85\x84aQ\xF0V[a\\\xAA\x83aLxV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x83\x01a\\\xC6_\x84\x01\x84aX\x94V[a\\\xD2_\x86\x01\x82aM9V[Pa\\\xE0` \x84\x01\x84aX\x94V[a\\\xED` \x86\x01\x82aM9V[Pa\\\xFB`@\x84\x01\x84a\\'V[\x85\x83\x03`@\x87\x01Ra]\x0E\x83\x82\x84a\\\x89V[\x92PPPa]\x1F``\x84\x01\x84a\\'V[\x85\x83\x03``\x87\x01Ra]2\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_a]K\x83\x83a\\\xB5V[\x90P\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12a]nWa]maX\xB2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a]\x91\x83\x85a\\\x0EV[\x93P\x83` \x84\x02\x85\x01a]\xA3\x84a\\\x1EV[\x80_[\x87\x81\x10\x15a]\xE6W\x84\x84\x03\x89Ra]\xBD\x82\x84a]SV[a]\xC7\x85\x82a]@V[\x94Pa]\xD2\x83a]zV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa]\xA6V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\x11\x81\x84\x86a]\x86V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a^6\x81a^\x1AV[\x82RPPV[_` \x82\x01\x90Pa^O_\x83\x01\x84a^-V[\x92\x91PPV[a^^\x81aN<V[\x81\x14a^hW_\x80\xFD[PV[_\x81Q\x90Pa^y\x81a^UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a^\x94Wa^\x93aK$V[[_a^\xA1\x84\x82\x85\x01a^kV[\x91PP\x92\x91PPV[_`\xA0\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\xC3\x81\x88\x8Aa]\x86V[\x90Pa^\xD2` \x83\x01\x87aP\x1BV[a^\xDF`@\x83\x01\x86aP\x1BV[a^\xEC``\x83\x01\x85aP\x1BV[a^\xF9`\x80\x83\x01\x84aP\x1BV[\x97\x96PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_``\x83\x01a_._\x84\x01\x84aX\x94V[a_:_\x86\x01\x82aM9V[Pa_H` \x84\x01\x84aX\x94V[a_U` \x86\x01\x82aM9V[Pa_c`@\x84\x01\x84a\\'V[\x85\x83\x03`@\x87\x01Ra_v\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_a_\x8F\x83\x83a_\x1DV[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12a_\xB2Wa_\xB1aX\xB2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a_\xD5\x83\x85a_\x04V[\x93P\x83` \x84\x02\x85\x01a_\xE7\x84a_\x14V[\x80_[\x87\x81\x10\x15a`*W\x84\x84\x03\x89Ra`\x01\x82\x84a_\x97V[a`\x0B\x85\x82a_\x84V[\x94Pa`\x16\x83a_\xBEV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa_\xEAV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra`U\x81\x85\x87a_\xCAV[\x90Pa`d` \x83\x01\x84aP\x1BV[\x94\x93PPPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a`\x94Wa`\x93a`lV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a`\xB6Wa`\xB5a`pV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15a`\xD2Wa`\xD1a`tV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aa@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aa\x05V[aaJ\x86\x83aa\x05V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aa\x85aa\x80aa{\x84aK\xD8V[aabV[aK\xD8V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aa\x9E\x83aakV[aa\xB2aa\xAA\x82aa\x8CV[\x84\x84Taa\x11V[\x82UPPPPV[_\x90V[aa\xC6aa\xBAV[aa\xD1\x81\x84\x84aa\x95V[PPPV[[\x81\x81\x10\x15aa\xF4Waa\xE9_\x82aa\xBEV[`\x01\x81\x01\x90Paa\xD7V[PPV[`\x1F\x82\x11\x15ab9Wab\n\x81a`\xE4V[ab\x13\x84a`\xF6V[\x81\x01` \x85\x10\x15ab\"W\x81\x90P[ab6ab.\x85a`\xF6V[\x83\x01\x82aa\xD6V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_abY_\x19\x84`\x08\x02ab>V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_abq\x83\x83abJV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[ab\x8B\x83\x83a`\xDAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab\xA4Wab\xA3aQHV[[ab\xAE\x82Ta[\xDEV[ab\xB9\x82\x82\x85aa\xF8V[_`\x1F\x83\x11`\x01\x81\x14ab\xE6W_\x84\x15ab\xD4W\x82\x87\x015\x90P[ab\xDE\x85\x82abfV[\x86UPacEV[`\x1F\x19\x84\x16ab\xF4\x86a`\xE4V[_[\x82\x81\x10\x15ac\x1BW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pab\xF6V[\x86\x83\x10\x15ac8W\x84\x89\x015ac4`\x1F\x89\x16\x82abJV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[acY\x83\x83\x83ab\x81V[PPPV[_\x81\x01_\x83\x01acn\x81\x85a`xV[acy\x81\x83\x86acNV[PPPP`\x01\x81\x01` \x83\x01ac\x8F\x81\x85a`xV[ac\x9A\x81\x83\x86acNV[PPPPPPV[ac\xAC\x82\x82ac^V[PPV[_`@\x83\x01ac\xC1_\x84\x01\x84a\\'V[\x85\x83\x03_\x87\x01Rac\xD3\x83\x82\x84a\\\x89V[\x92PPPac\xE4` \x84\x01\x84a\\'V[\x85\x83\x03` \x87\x01Rac\xF7\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_ad\x13` \x84\x01\x84aK\xF7V[\x90P\x92\x91PPV[`\xA0\x82\x01ad+_\x83\x01\x83ad\x05V[ad7_\x85\x01\x82aN\x98V[PadE` \x83\x01\x83ad\x05V[adR` \x85\x01\x82aN\x98V[Pad``@\x83\x01\x83ad\x05V[adm`@\x85\x01\x82aN\x98V[Pad{``\x83\x01\x83ad\x05V[ad\x88``\x85\x01\x82aN\x98V[Pad\x96`\x80\x83\x01\x83ad\x05V[ad\xA3`\x80\x85\x01\x82aN\x98V[PPPPV[_a\x01 \x82\x01\x90P\x81\x81\x03_\x83\x01Rad\xC2\x81\x8Bac\xB0V[\x90Pad\xD1` \x83\x01\x8Aad\x1BV[\x81\x81\x03`\xC0\x83\x01Rad\xE4\x81\x88\x8Aa]\x86V[\x90P\x81\x81\x03`\xE0\x83\x01Rad\xF9\x81\x86\x88a_\xCAV[\x90P\x81\x81\x03a\x01\0\x83\x01Rae\x0F\x81\x84\x86aY\xF1V[\x90P\x99\x98PPPPPPPPPV[_\x815ae*\x81aK\xE1V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaei\x84ae3V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[ae\x88\x82aakV[ae\x9Bae\x94\x82aa\x8CV[\x83Tae>V[\x82UPPPV[_\x815ae\xAE\x81aM\xE7V[\x80\x91PP\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFae\xD6\x84ae3V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_af\x06af\x01ae\xFC\x84aM\tV[aabV[aM\tV[\x90P\x91\x90PV[_af\x17\x82ae\xECV[\x90P\x91\x90PV[_af(\x82af\rV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[afA\x82af\x1EV[afTafM\x82af/V[\x83Tae\xB7V[\x82UPPPV[_\x81\x01_\x83\x01\x80afk\x81ae\x1EV[\x90Pafw\x81\x84ae\x7FV[PPP`\x01\x81\x01` \x83\x01\x80af\x8C\x81ae\xA2V[\x90Paf\x98\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01\x80af\xAD\x81ae\xA2V[\x90Paf\xB9\x81\x84af8V[PPP`\x03\x81\x01``\x83\x01af\xCE\x81\x85a`xV[af\xD9\x81\x83\x86acNV[PPPP`\x04\x81\x01`\x80\x83\x01af\xEF\x81\x85a`xV[af\xFA\x81\x83\x86acNV[PPPPPPV[ag\x0C\x82\x82af[V[PPV[_`\xA0\x83\x01ag!_\x84\x01\x84ad\x05V[ag-_\x86\x01\x82aN\x98V[Pag;` \x84\x01\x84aX\x94V[agH` \x86\x01\x82aM9V[PagV`@\x84\x01\x84aX\x94V[agc`@\x86\x01\x82aM9V[Pagq``\x84\x01\x84a\\'V[\x85\x83\x03``\x87\x01Rag\x84\x83\x82\x84a\\\x89V[\x92PPPag\x95`\x80\x84\x01\x84a\\'V[\x85\x83\x03`\x80\x87\x01Rag\xA8\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rag\xCE\x81\x84ag\x10V[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12ag\xF1Wag\xF0a`lV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ah\x19Wah\x18a`lV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ah;Wah:a`pV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15ahWWahVa`tV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ah\xBCWah\x8D\x81ahiV[ah\x96\x84a`\xF6V[\x81\x01` \x85\x10\x15ah\xA5W\x81\x90P[ah\xB9ah\xB1\x85a`\xF6V[\x83\x01\x82aa\xD6V[PP[PPPV[ah\xCB\x83\x83ah_V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\xE4Wah\xE3aQHV[[ah\xEE\x82Ta[\xDEV[ah\xF9\x82\x82\x85ah{V[_`\x1F\x83\x11`\x01\x81\x14ai&W_\x84\x15ai\x14W\x82\x87\x015\x90P[ai\x1E\x85\x82abfV[\x86UPai\x85V[`\x1F\x19\x84\x16ai4\x86ahiV[_[\x82\x81\x10\x15ai[W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pai6V[\x86\x83\x10\x15aixW\x84\x89\x015ait`\x1F\x89\x16\x82abJV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[ai\x99\x83\x83\x83ah\xC1V[PPPV[_\x81\x01_\x83\x01\x80ai\xAE\x81ae\xA2V[\x90Pai\xBA\x81\x84af8V[PPP`\x01\x81\x01` \x83\x01\x80ai\xCF\x81ae\xA2V[\x90Pai\xDB\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01ai\xF0\x81\x85ag\xFDV[ai\xFB\x81\x83\x86ai\x8EV[PPPPPPV[aj\r\x82\x82ai\x9EV[PPV[_`@\x82\x01\x90Paj$_\x83\x01\x85aP\x1BV[aj1` \x83\x01\x84aP\x1BV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[ajn\x81aR\xC6V[\x81\x14ajxW_\x80\xFD[PV[_\x81Q\x90Paj\x89\x81ajeV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aj\xA4Waj\xA3aK$V[[_aj\xB1\x84\x82\x85\x01aj{V[\x91PP\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12aj\xD5Waj\xD4a`lV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x81\x01_\x83\x01\x80aj\xF1\x81ae\xA2V[\x90Paj\xFD\x81\x84af8V[PPP`\x01\x81\x01` \x83\x01\x80ak\x12\x81ae\xA2V[\x90Pak\x1E\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01ak3\x81\x85a`xV[ak>\x81\x83\x86acNV[PPPP`\x03\x81\x01``\x83\x01akT\x81\x85a`xV[ak_\x81\x83\x86acNV[PPPPPPV[akq\x82\x82aj\xE1V[PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12ak\x90Wak\x8Fa`lV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x81\x01_\x83\x01\x80ak\xAC\x81ae\xA2V[\x90Pak\xB8\x81\x84af8V[PPP`\x01\x81\x01` \x83\x01\x80ak\xCD\x81ae\xA2V[\x90Pak\xD9\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01ak\xEE\x81\x85a`xV[ak\xF9\x81\x83\x86acNV[PPPPPPV[al\x0B\x82\x82ak\x9CV[PPV[_\x81\x90P\x92\x91PPV[_al#\x82aV\x01V[al-\x81\x85al\x0FV[\x93Pal=\x81\x85` \x86\x01aLPV[\x80\x84\x01\x91PP\x92\x91PPV[_alT\x82\x84al\x19V[\x91P\x81\x90P\x92\x91PPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610287575f3560e01c8063798b58a611610159578063bff3aaba116100c0578063e30c397811610079578063e30c397814610975578063e3b2a8741461099f578063e5275eaf146109db578063eb843cf614610a17578063ef6997f914610a3f578063f2fde38b14610a7b57610287565b8063bff3aaba14610847578063c2b4298614610883578063c80b33ca146108ad578063cb5aa7e9146108d5578063d10f7ff914610911578063d5e16b7d1461094d57610287565b80639164d0ae116101125780639164d0ae146107615780639a5a3bc41461078b578063ad3cb1cc146107a1578063b4722bc4146107cb578063ba1f31d2146107f5578063bb59e3621461081f57610287565b8063798b58a61461067d57806379ba5097146106935780637eaac8f2146106a957806383bb2e57146106d3578063882d7dd3146106fb5780638da5cb5b1461073757610287565b80632dd3edfe116101fd57806353da9246116101b657806353da9246146105875780635bace7ff146105af5780636799ef52146105eb578063715018a6146106155780637420f3d41461062b578063772d2fe91461065557610287565b80632dd3edfe146104775780632e2d3a82146104b357806346fbf68e146104db57806348144c61146105175780634f1ef2861461054157806352d1902d1461055d57610287565b80632585bb651161024f5780632585bb651461036b57806326cf5def1461039557806327612cf6146103bf5780632a388998146103e75780632a8b9de9146104115780632b101c031461043b57610287565b8063013dc21e1461028b5780630724dd23146102b35780630d8e6e2c146102db5780631ea5bd4214610305578063203d01141461032f575b5f80fd5b348015610296575f80fd5b506102b160048036038101906102ac9190614b8d565b610aa3565b005b3480156102be575f80fd5b506102d960048036038101906102d49190614c0b565b610d3f565b005b3480156102e6575f80fd5b506102ef610d8a565b6040516102fc9190614cc0565b60405180910390f35b348015610310575f80fd5b50610319610e05565b6040516103269190614dc7565b60405180910390f35b34801561033a575f80fd5b5061035560048036038101906103509190614e11565b610e9e565b6040516103629190614e56565b60405180910390f35b348015610376575f80fd5b5061037f610efe565b60405161038c9190614ffb565b60405180910390f35b3480156103a0575f80fd5b506103a961113b565b6040516103b6919061502a565b60405180910390f35b3480156103ca575f80fd5b506103e560048036038101906103e09190615098565b611152565b005b3480156103f2575f80fd5b506103fb6112dc565b604051610408919061502a565b60405180910390f35b34801561041c575f80fd5b506104256112f3565b6040516104329190614dc7565b60405180910390f35b348015610446575f80fd5b50610461600480360381019061045c9190614e11565b61138c565b60405161046e9190614e56565b60405180910390f35b348015610482575f80fd5b5061049d60048036038101906104989190614e11565b6113ec565b6040516104aa9190614e56565b60405180910390f35b3480156104be575f80fd5b506104d960048036038101906104d49190614c0b565b61144c565b005b3480156104e6575f80fd5b5061050160048036038101906104fc9190614e11565b611497565b60405161050e9190614e56565b60405180910390f35b348015610522575f80fd5b5061052b61152b565b6040516105389190615124565b60405180910390f35b61055b6004803603810190610556919061526c565b611671565b005b348015610568575f80fd5b50610571611690565b60405161057e91906152de565b60405180910390f35b348015610592575f80fd5b506105ad60048036038101906105a891906152f7565b6116c1565b005b3480156105ba575f80fd5b506105d560048036038101906105d09190614e11565b61197c565b6040516105e29190614e56565b60405180910390f35b3480156105f6575f80fd5b506105ff6119dc565b60405161060c919061502a565b60405180910390f35b348015610620575f80fd5b506106296119f3565b005b348015610636575f80fd5b5061063f611a06565b60405161064c9190614dc7565b60405180910390f35b348015610660575f80fd5b5061067b60048036038101906106769190614c0b565b611a9f565b005b348015610688575f80fd5b50610691611aea565b005b34801561069e575f80fd5b506106a7611bfe565b005b3480156106b4575f80fd5b506106bd611c8c565b6040516106ca9190614dc7565b60405180910390f35b3480156106de575f80fd5b506106f960048036038101906106f491906153e2565b611d25565b005b348015610706575f80fd5b50610721600480360381019061071c9190614e11565b611fc5565b60405161072e9190614e56565b60405180910390f35b348015610742575f80fd5b5061074b612025565b604051610758919061544e565b60405180910390f35b34801561076c575f80fd5b5061077561205a565b6040516107829190614dc7565b60405180910390f35b348015610796575f80fd5b5061079f6120f3565b005b3480156107ac575f80fd5b506107b5612249565b6040516107c29190614cc0565b60405180910390f35b3480156107d6575f80fd5b506107df612282565b6040516107ec919061502a565b60405180910390f35b348015610800575f80fd5b50610809612299565b6040516108169190614dc7565b60405180910390f35b34801561082a575f80fd5b50610845600480360381019061084091906154a7565b612332565b005b348015610852575f80fd5b5061086d60048036038101906108689190614c0b565b612554565b60405161087a9190614e56565b60405180910390f35b34801561088e575f80fd5b50610897612588565b6040516108a4919061502a565b60405180910390f35b3480156108b8575f80fd5b506108d360048036038101906108ce91906155ba565b61259f565b005b3480156108e0575f80fd5b506108fb60048036038101906108f69190614e11565b61274d565b60405161090891906156a0565b60405180910390f35b34801561091c575f80fd5b5061093760048036038101906109329190614c0b565b6128eb565b604051610944919061573a565b60405180910390f35b348015610958575f80fd5b50610973600480360381019061096e9190614c0b565b612b06565b005b348015610980575f80fd5b50610989612b51565b604051610996919061544e565b60405180910390f35b3480156109aa575f80fd5b506109c560048036038101906109c09190614e11565b612b86565b6040516109d291906157c1565b60405180910390f35b3480156109e6575f80fd5b50610a0160048036038101906109fc9190614e11565b612db4565b604051610a0e9190614e56565b60405180910390f35b348015610a22575f80fd5b50610a3d6004803603810190610a389190614c0b565b612e14565b005b348015610a4a575f80fd5b50610a656004803603810190610a609190614e11565b612e5f565b604051610a72919061582e565b60405180910390f35b348015610a86575f80fd5b50610aa16004803603810190610a9c9190614e11565b612ffd565b005b610aab6130b6565b5f610ab461313d565b90505f816012018054905090505f5b81811015610cd7575f836014015f856012018481548110610ae757610ae661584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836015015f856013018481548110610b7a57610b7961584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550826011015f846012018381548110610c0c57610c0b61584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8082015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600182015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600282015f610cc8919061490e565b50508080600101915050610ac3565b50816012015f610ce7919061494b565b816013015f610cf6919061494b565b610d008484613164565b7f6cdc1aa76e1ebacd67c81be0dcf9603b5dfbeb4dd801ab214114acb536f110688484604051610d31929190615a63565b60405180910390a150505050565b610d476130b6565b610d50816134bb565b7f30c9b1d004f57eae3c6cc3a3752bcb4c8ea2e57c8241a782aa9b65fbc604ec5b81604051610d7f919061502a565b60405180910390a150565b60606040518060400160405280600d81526020017f47617465776179436f6e66696700000000000000000000000000000000000000815250610dcb5f61355f565b610dd5600361355f565b610dde5f61355f565b604051602001610df19493929190615b53565b604051602081830303815290604052905090565b60605f610e1061313d565b905080600d01805480602002602001604051908101604052809291908181526020018280548015610e9357602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610e4a575b505050505091505090565b5f80610ea861313d565b9050806003015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b60605f610f0961313d565b905080601001805480602002602001604051908101604052809291908181526020015f905b82821015611131578382905f5260205f2090600502016040518060a00160405290815f8201548152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160038201805461101290615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461103e90615bde565b80156110895780601f1061106057610100808354040283529160200191611089565b820191905f5260205f20905b81548152906001019060200180831161106c57829003601f168201915b505050505081526020016004820180546110a290615bde565b80601f01602080910402602001604051908101604052809291908181526020018280546110ce90615bde565b80156111195780601f106110f057610100808354040283529160200191611119565b820191905f5260205f20905b8154815290600101906020018083116110fc57829003601f168201915b50505050508152505081526020019060010190610f2e565b5050505091505090565b5f8061114561313d565b9050806007015491505090565b60045f61115d613629565b9050805f0160089054906101000a900460ff16806111a557508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156111dc576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f61122a61313d565b905061124a858583600701548460080154856009015486601601546116c1565b7f430929cffdf43b3681bb03eae5802745e335ec0159968d301e2a8e3d209eefb8858560405161127b929190615df8565b60405180910390a1505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516112ce9190615e3c565b60405180910390a150505050565b5f806112e661313d565b9050806008015491505090565b60605f6112fe61313d565b90508060120180548060200260200160405190810160405280929190818152602001828054801561138157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611338575b505050505091505090565b5f8061139661313d565b905080600b015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806113f661313d565b905080600a015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b6114546130b6565b61145d81613650565b7fe41802af725729adcb8c151e2937380a25c69155757e3af5d3979adab50358008160405161148c919061502a565b60405180910390a150565b5f73c3f9e1d27cd10402375b7cd237d57e0f4888c18973ffffffffffffffffffffffffffffffffffffffff166346fbf68e836040518263ffffffff1660e01b81526004016114e5919061544e565b602060405180830381865afa158015611500573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115249190615e7f565b9050919050565b611533614969565b5f61153c61313d565b9050805f016040518060400160405290815f8201805461155b90615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461158790615bde565b80156115d25780601f106115a9576101008083540402835291602001916115d2565b820191905f5260205f20905b8154815290600101906020018083116115b557829003601f168201915b505050505081526020016001820180546115eb90615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461161790615bde565b80156116625780601f1061163957610100808354040283529160200191611662565b820191905f5260205f20905b81548152906001019060200180831161164557829003601f168201915b50505050508152505091505090565b6116796136f4565b611682826137da565b61168c82826137e5565b5050565b5f611699613903565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b6116c96130b6565b5f6116d261313d565b90505f816005018054905090505f5b81811015611904575f836002015f8560050184815481106117055761170461584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f836003015f8560060184815481106117985761179761584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550826004015f84600501838154811061182a5761182961584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8082015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600182015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600282015f6118e69190614983565b600382015f6118f59190614983565b505080806001019150506116e1565b50816005015f611914919061494b565b816006015f611923919061494b565b61193188888888888861398a565b7f25d1ea647128b56d47e64534cd0f5a86d3207f67b04895495b66dc0db87a0ca788888888888860405161196a96959493929190615eaa565b60405180910390a15050505050505050565b5f8061198661313d565b9050806014015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806119e661313d565b9050806017015491505090565b6119fb6130b6565b611a045f613d09565b565b60605f611a1161313d565b905080600501805480602002602001604051908101604052809291908181526020018280548015611a9457602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611a4b575b505050505091505090565b611aa76130b6565b611ab081613d46565b7f3571172a49e72d7724be384cdd59f4f21a216c70352ea59cb02543fc7630843781604051611adf919061502a565b60405180910390a150565b611af26130b6565b7387a5b1152aa51728258dbc1aa54b6a83dcd1d3dd73ffffffffffffffffffffffffffffffffffffffff16633f4ba83a6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611b4b575f80fd5b505af1158015611b5d573d5f803e3d5ffd5b505050507333e0c7a03d2b040b518580c365f4b3bde7cc4e6e73ffffffffffffffffffffffffffffffffffffffff16633f4ba83a6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015611bba575f80fd5b505af1158015611bcc573d5f803e3d5ffd5b505050507fbe4f655daae0dbaef63a6b525cab2fa6ace4aa5b94b8834b241137cdfe73a5b060405160405180910390a1565b5f611c07613db0565b90508073ffffffffffffffffffffffffffffffffffffffff16611c28612b51565b73ffffffffffffffffffffffffffffffffffffffff1614611c8057806040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401611c77919061544e565b60405180910390fd5b611c8981613d09565b50565b60605f611c9761313d565b905080600601805480602002602001604051908101604052809291908181526020018280548015611d1a57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611cd1575b505050505091505090565b611d2d6130b6565b5f611d3661313d565b90505f81600d018054905090505f5b81811015611f59575f83600a015f85600d018481548110611d6957611d6861584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f83600b015f85600e018481548110611dfc57611dfb61584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555082600c015f84600d018381548110611e8e57611e8d61584e565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f8082015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600182015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055600282015f611f4a9190614983565b50508080600101915050611d45565b5081600d015f611f69919061494b565b81600e015f611f78919061494b565b611f83858585613db7565b7fffe20bdb855e514e94147702922690cf1da10bdd18bf1f6215027c93ac05d455858585604051611fb69392919061603c565b60405180910390a15050505050565b5f80611fcf61313d565b9050806015015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f8061202f614118565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b60605f61206561313d565b905080600e018054806020026020016040519081016040528092919081815260200182805480156120e857602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161209f575b505050505091505090565b6120fc33611497565b61213d57336040517f206a346e000000000000000000000000000000000000000000000000000000008152600401612134919061544e565b60405180910390fd5b7387a5b1152aa51728258dbc1aa54b6a83dcd1d3dd73ffffffffffffffffffffffffffffffffffffffff16638456cb596040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612196575f80fd5b505af11580156121a8573d5f803e3d5ffd5b505050507333e0c7a03d2b040b518580c365f4b3bde7cc4e6e73ffffffffffffffffffffffffffffffffffffffff16638456cb596040518163ffffffff1660e01b81526004015f604051808303815f87803b158015612205575f80fd5b505af1158015612217573d5f803e3d5ffd5b505050507f13dbe8823219e226dd0525aeb071e1d2679f89382ba799f7f644867e65b6f3a660405160405180910390a1565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f8061228c61313d565b9050806016015491505090565b60605f6122a461313d565b90508060130180548060200260200160405190810160405280929190818152602001828054801561232757602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116122de575b505050505091505090565b600161233c61413f565b67ffffffffffffffff161461237d576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60045f612388613629565b9050805f0160089054906101000a900460ff16806123d057508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15612407576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061245c612457612025565b614163565b5f61246561313d565b90508a815f01818161247791906163a2565b90505061249789898c5f01358d602001358e604001358f6060013561398a565b6124a687878c60800135613db7565b6124b08585613164565b7fb2cbe65ea308bfe4b9431819a3168d544f46ba344b1e79f92f973fcff43aae3b8b8b8b8b8b8b8b8b6040516124ed9897969594939291906164a9565b60405180910390a1505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516125409190615e3c565b60405180910390a150505050505050505050565b5f8061255e61313d565b905080600f015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f8061259261313d565b9050806009015491505090565b6125a76130b6565b5f815f0135036125e3576040517f22f73fea00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f0135111561263857805f01356040517f4178de4200000000000000000000000000000000000000000000000000000000815260040161262f919061502a565b60405180910390fd5b5f61264161313d565b905080600f015f835f013581526020019081526020015f205f9054906101000a900460ff16156126ab57815f01356040517f96a568280000000000000000000000000000000000000000000000000000000081526004016126a2919061502a565b60405180910390fd5b8060100182908060018154018082558091505060019003905f5260205f2090600502015f9091909190915081816126e29190616702565b5050600181600f015f845f013581526020019081526020015f205f6101000a81548160ff0219169083151502179055507f66769341effd268fc4e9a9c8f27bfc968507b519b0ddb9b4ad3ded5f030168378260405161274191906167b6565b60405180910390a15050565b6127556149c0565b5f61275e61313d565b9050806011015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060600160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461286390615bde565b80601f016020809104026020016040519081016040528092919081815260200182805461288f90615bde565b80156128da5780601f106128b1576101008083540402835291602001916128da565b820191905f5260205f20905b8154815290600101906020018083116128bd57829003601f168201915b505050505081525050915050919050565b6128f3614a0b565b5f6128fc61313d565b90508060100183815481106129145761291361584e565b5b905f5260205f2090600502016040518060a00160405290815f8201548152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020016003820180546129ee90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612a1a90615bde565b8015612a655780601f10612a3c57610100808354040283529160200191612a65565b820191905f5260205f20905b815481529060010190602001808311612a4857829003601f168201915b50505050508152602001600482018054612a7e90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612aaa90615bde565b8015612af55780601f10612acc57610100808354040283529160200191612af5565b820191905f5260205f20905b815481529060010190602001808311612ad857829003601f168201915b505050505081525050915050919050565b612b0e6130b6565b612b1781614177565b7f7a2ef7dc89400a8ad92bb4ccf44d482624b40fe76b66977e85ed6a618e2e2fc781604051612b46919061502a565b60405180910390a150565b5f80612b5b61421b565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b612b8e614a63565b5f612b9761313d565b9050806004015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054612c9c90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612cc890615bde565b8015612d135780601f10612cea57610100808354040283529160200191612d13565b820191905f5260205f20905b815481529060010190602001808311612cf657829003601f168201915b50505050508152602001600382018054612d2c90615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612d5890615bde565b8015612da35780601f10612d7a57610100808354040283529160200191612da3565b820191905f5260205f20905b815481529060010190602001808311612d8657829003601f168201915b505050505081525050915050919050565b5f80612dbe61313d565b9050806002015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b612e1c6130b6565b612e2581614242565b7f837e0a6528dadfa2dc792692c5182e52a9f5bbdeed7b2372927a26c69583961381604051612e54919061502a565b60405180910390a150565b612e67614ab5565b5f612e7061313d565b905080600c015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060600160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054612f7590615bde565b80601f0160208091040260200160405190810160405280929190818152602001828054612fa190615bde565b8015612fec5780601f10612fc357610100808354040283529160200191612fec565b820191905f5260205f20905b815481529060010190602001808311612fcf57829003601f168201915b505050505081525050915050919050565b6130056130b6565b5f61300e61421b565b905081815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff16613070612025565b73ffffffffffffffffffffffffffffffffffffffff167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a35050565b6130be613db0565b73ffffffffffffffffffffffffffffffffffffffff166130dc612025565b73ffffffffffffffffffffffffffffffffffffffff161461313b576130ff613db0565b6040517f118cdaa7000000000000000000000000000000000000000000000000000000008152600401613132919061544e565b60405180910390fd5b565b5f7f86d3070a8993f6b209bee6185186d38a07fce8bbd97c750d934451b72f35b400905090565b5f82829050036131a0576040517fcad1d53400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6131a961313d565b90505f5b838390508110156134b5578383828181106131cb576131ca61584e565b5b90506020028101906131dd91906167d6565b826011015f8686858181106131f5576131f461584e565b5b905060200281019061320791906167d6565b5f0160208101906132189190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f20818161325d9190616a03565b905050816012018484838181106132775761327661584e565b5b905060200281019061328991906167d6565b5f01602081019061329a9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001826014015f8686858181106133115761331061584e565b5b905060200281019061332391906167d6565b5f0160208101906133349190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508160130184848381811061339a5761339961584e565b5b90506020028101906133ac91906167d6565b60200160208101906133be9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001826015015f8686858181106134355761343461584e565b5b905060200281019061344791906167d6565b60200160208101906134599190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080806001019150506131ad565b50505050565b5f6134c461313d565b90505f816006018054905090505f830361350a576040517f3ee5077400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156135515782816040517f0f69cbfc000000000000000000000000000000000000000000000000000000008152600401613548929190616a11565b60405180910390fd5b828260160181905550505050565b60605f600161356d846142e6565b0190505f8167ffffffffffffffff81111561358b5761358a615148565b5b6040519080825280601f01601f1916602001820160405280156135bd5781602001600182028036833780820191505090505b5090505f82602001820190505b60011561361e578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161361357613612616a38565b5b0494505f85036135ca575b819350505050919050565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f61365961313d565b90505f816006018054905090505f830361369f576040517fb1ae92ea00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156136e65782816040517f84208f230000000000000000000000000000000000000000000000000000000081526004016136dd929190616a11565b60405180910390fd5b828260080181905550505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614806137a157507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613788614437565b73ffffffffffffffffffffffffffffffffffffffff1614155b156137d8576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6137e26130b6565b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561384d57506040513d601f19601f8201168201806040525081019061384a9190616a8f565b60015b61388e57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613885919061544e565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146138f457806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016138eb91906152de565b60405180910390fd5b6138fe838361448a565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614613988576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f86869050036139c6576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6139cf61313d565b90505f5b87879050811015613cdb576001826002015f8a8a858181106139f8576139f761584e565b5b9050602002810190613a0a9190616aba565b5f016020810190613a1b9190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550878782818110613a7d57613a7c61584e565b5b9050602002810190613a8f9190616aba565b826004015f8a8a85818110613aa757613aa661584e565b5b9050602002810190613ab99190616aba565b5f016020810190613aca9190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181613b0f9190616b67565b90505081600501888883818110613b2957613b2861584e565b5b9050602002810190613b3b9190616aba565b5f016020810190613b4c9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506001826003015f8a8a85818110613bc357613bc261584e565b5b9050602002810190613bd59190616aba565b6020016020810190613be79190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555081600601888883818110613c4d57613c4c61584e565b5b9050602002810190613c5f9190616aba565b6020016020810190613c719190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555080806001019150506139d3565b50613ce585613d46565b613cee84613650565b613cf783614242565b613d00826134bb565b50505050505050565b5f613d1261421b565b9050805f015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff0219169055613d42826144fc565b5050565b5f613d4f61313d565b90505f81600601805490509050808310613da25782816040517f907e6681000000000000000000000000000000000000000000000000000000008152600401613d99929190616a11565b60405180910390fd5b828260070181905550505050565b5f33905090565b5f8383905003613df3576040517f8af082ef00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f613dfc61313d565b90505f5b8484905081101561410857600182600a015f878785818110613e2557613e2461584e565b5b9050602002810190613e379190616b75565b5f016020810190613e489190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550848482818110613eaa57613ea961584e565b5b9050602002810190613ebc9190616b75565b82600c015f878785818110613ed457613ed361584e565b5b9050602002810190613ee69190616b75565b5f016020810190613ef79190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181613f3c9190616c01565b90505081600d01858583818110613f5657613f5561584e565b5b9050602002810190613f689190616b75565b5f016020810190613f799190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550600182600b015f878785818110613ff057613fef61584e565b5b90506020028101906140029190616b75565b60200160208101906140149190614e11565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555081600e0185858381811061407a5761407961584e565b5b905060200281019061408c9190616b75565b602001602081019061409e9190614e11565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508080600101915050613e00565b5061411282614177565b50505050565b5f7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300905090565b5f614148613629565b5f015f9054906101000a900467ffffffffffffffff16905090565b61416b6145cd565b6141748161460d565b50565b5f61418061313d565b90505f81600e018054905090505f83036141c6576040517fb60d244100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8083111561420d5782816040517f97beabad000000000000000000000000000000000000000000000000000000008152600401614204929190616a11565b60405180910390fd5b828260170181905550505050565b5f7f237e158222e3e6968b72b9db0d8043aacf074ad9f650f0d1606b4d82ee432c00905090565b5f61424b61313d565b90505f816006018054905090505f8303614291576040517fe60a727100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156142d85782816040517fd2535e110000000000000000000000000000000000000000000000000000000081526004016142cf929190616a11565b60405180910390fd5b828260090181905550505050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310614342577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161433857614337616a38565b5b0492506040810190505b6d04ee2d6d415b85acef8100000000831061437f576d04ee2d6d415b85acef8100000000838161437557614374616a38565b5b0492506020810190505b662386f26fc1000083106143ae57662386f26fc1000083816143a4576143a3616a38565b5b0492506010810190505b6305f5e10083106143d7576305f5e10083816143cd576143cc616a38565b5b0492506008810190505b61271083106143fc5761271083816143f2576143f1616a38565b5b0492506004810190505b6064831061441f576064838161441557614414616a38565b5b0492506002810190505b600a831061442e576001810190505b80915050919050565b5f6144637f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614691565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b6144938261469a565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156144ef576144e98282614763565b506144f8565b6144f76147e3565b5b5050565b5f614505614118565b90505f815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905082825f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a3505050565b6145d561481f565b61460b576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6146156145cd565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614685575f6040517f1e4fbdf700000000000000000000000000000000000000000000000000000000815260040161467c919061544e565b60405180910390fd5b61468e81613d09565b50565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b036146f557806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016146ec919061544e565b60405180910390fd5b806147217f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614691565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff168460405161478c9190616c49565b5f60405180830381855af49150503d805f81146147c4576040519150601f19603f3d011682016040523d82523d5f602084013e6147c9565b606091505b50915091506147d985838361483d565b9250505092915050565b5f34111561481d576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f614828613629565b5f0160089054906101000a900460ff16905090565b6060826148525761484d826148ca565b6148c2565b5f825114801561487857505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156148ba57836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016148b1919061544e565b60405180910390fd5b8190506148c3565b5b9392505050565b5f815111156148dc5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50805461491a90615bde565b5f825580601f1061492b5750614948565b601f0160209004905f5260205f20908101906149479190614b00565b5b50565b5080545f8255905f5260205f20908101906149669190614b00565b50565b604051806040016040528060608152602001606081525090565b50805461498f90615bde565b5f825580601f106149a057506149bd565b601f0160209004905f5260205f20908101906149bc9190614b00565b5b50565b60405180606001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff168152602001606081525090565b6040518060a001604052805f81526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b60405180606001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff168152602001606081525090565b5b80821115614b17575f815f905550600101614b01565b5090565b5f604051905090565b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f80fd5b5f8083601f840112614b4d57614b4c614b2c565b5b8235905067ffffffffffffffff811115614b6a57614b69614b30565b5b602083019150836020820283011115614b8657614b85614b34565b5b9250929050565b5f8060208385031215614ba357614ba2614b24565b5b5f83013567ffffffffffffffff811115614bc057614bbf614b28565b5b614bcc85828601614b38565b92509250509250929050565b5f819050919050565b614bea81614bd8565b8114614bf4575f80fd5b50565b5f81359050614c0581614be1565b92915050565b5f60208284031215614c2057614c1f614b24565b5b5f614c2d84828501614bf7565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015614c6d578082015181840152602081019050614c52565b5f8484015250505050565b5f601f19601f8301169050919050565b5f614c9282614c36565b614c9c8185614c40565b9350614cac818560208601614c50565b614cb581614c78565b840191505092915050565b5f6020820190508181035f830152614cd88184614c88565b905092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f614d3282614d09565b9050919050565b614d4281614d28565b82525050565b5f614d538383614d39565b60208301905092915050565b5f602082019050919050565b5f614d7582614ce0565b614d7f8185614cea565b9350614d8a83614cfa565b805f5b83811015614dba578151614da18882614d48565b9750614dac83614d5f565b925050600181019050614d8d565b5085935050505092915050565b5f6020820190508181035f830152614ddf8184614d6b565b905092915050565b614df081614d28565b8114614dfa575f80fd5b50565b5f81359050614e0b81614de7565b92915050565b5f60208284031215614e2657614e25614b24565b5b5f614e3384828501614dfd565b91505092915050565b5f8115159050919050565b614e5081614e3c565b82525050565b5f602082019050614e695f830184614e47565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b614ea181614bd8565b82525050565b5f82825260208201905092915050565b5f614ec182614c36565b614ecb8185614ea7565b9350614edb818560208601614c50565b614ee481614c78565b840191505092915050565b5f60a083015f830151614f045f860182614e98565b506020830151614f176020860182614d39565b506040830151614f2a6040860182614d39565b5060608301518482036060860152614f428282614eb7565b91505060808301518482036080860152614f5c8282614eb7565b9150508091505092915050565b5f614f748383614eef565b905092915050565b5f602082019050919050565b5f614f9282614e6f565b614f9c8185614e79565b935083602082028501614fae85614e89565b805f5b85811015614fe95784840389528151614fca8582614f69565b9450614fd583614f7c565b925060208a01995050600181019050614fb1565b50829750879550505050505092915050565b5f6020820190508181035f8301526150138184614f88565b905092915050565b61502481614bd8565b82525050565b5f60208201905061503d5f83018461501b565b92915050565b5f8083601f84011261505857615057614b2c565b5b8235905067ffffffffffffffff81111561507557615074614b30565b5b60208301915083602082028301111561509157615090614b34565b5b9250929050565b5f80602083850312156150ae576150ad614b24565b5b5f83013567ffffffffffffffff8111156150cb576150ca614b28565b5b6150d785828601615043565b92509250509250929050565b5f604083015f8301518482035f8601526150fd8282614eb7565b915050602083015184820360208601526151178282614eb7565b9150508091505092915050565b5f6020820190508181035f83015261513c81846150e3565b905092915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61517e82614c78565b810181811067ffffffffffffffff8211171561519d5761519c615148565b5b80604052505050565b5f6151af614b1b565b90506151bb8282615175565b919050565b5f67ffffffffffffffff8211156151da576151d9615148565b5b6151e382614c78565b9050602081019050919050565b828183375f83830152505050565b5f61521061520b846151c0565b6151a6565b90508281526020810184848401111561522c5761522b615144565b5b6152378482856151f0565b509392505050565b5f82601f83011261525357615252614b2c565b5b81356152638482602086016151fe565b91505092915050565b5f806040838503121561528257615281614b24565b5b5f61528f85828601614dfd565b925050602083013567ffffffffffffffff8111156152b0576152af614b28565b5b6152bc8582860161523f565b9150509250929050565b5f819050919050565b6152d8816152c6565b82525050565b5f6020820190506152f15f8301846152cf565b92915050565b5f805f805f8060a0878903121561531157615310614b24565b5b5f87013567ffffffffffffffff81111561532e5761532d614b28565b5b61533a89828a01615043565b9650965050602061534d89828a01614bf7565b945050604061535e89828a01614bf7565b935050606061536f89828a01614bf7565b925050608061538089828a01614bf7565b9150509295509295509295565b5f8083601f8401126153a2576153a1614b2c565b5b8235905067ffffffffffffffff8111156153bf576153be614b30565b5b6020830191508360208202830111156153db576153da614b34565b5b9250929050565b5f805f604084860312156153f9576153f8614b24565b5b5f84013567ffffffffffffffff81111561541657615415614b28565b5b6154228682870161538d565b9350935050602061543586828701614bf7565b9150509250925092565b61544881614d28565b82525050565b5f6020820190506154615f83018461543f565b92915050565b5f80fd5b5f604082840312156154805761547f615467565b5b81905092915050565b5f60a0828403121561549e5761549d615467565b5b81905092915050565b5f805f805f805f80610120898b0312156154c4576154c3614b24565b5b5f89013567ffffffffffffffff8111156154e1576154e0614b28565b5b6154ed8b828c0161546b565b98505060206154fe8b828c01615489565b97505060c089013567ffffffffffffffff81111561551f5761551e614b28565b5b61552b8b828c01615043565b965096505060e089013567ffffffffffffffff81111561554e5761554d614b28565b5b61555a8b828c0161538d565b945094505061010089013567ffffffffffffffff81111561557e5761557d614b28565b5b61558a8b828c01614b38565b92509250509295985092959890939650565b5f60a082840312156155b1576155b0615467565b5b81905092915050565b5f602082840312156155cf576155ce614b24565b5b5f82013567ffffffffffffffff8111156155ec576155eb614b28565b5b6155f88482850161559c565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f61562582615601565b61562f818561560b565b935061563f818560208601614c50565b61564881614c78565b840191505092915050565b5f606083015f8301516156685f860182614d39565b50602083015161567b6020860182614d39565b5060408301518482036040860152615693828261561b565b9150508091505092915050565b5f6020820190508181035f8301526156b88184615653565b905092915050565b5f60a083015f8301516156d55f860182614e98565b5060208301516156e86020860182614d39565b5060408301516156fb6040860182614d39565b50606083015184820360608601526157138282614eb7565b9150506080830151848203608086015261572d8282614eb7565b9150508091505092915050565b5f6020820190508181035f83015261575281846156c0565b905092915050565b5f608083015f83015161576f5f860182614d39565b5060208301516157826020860182614d39565b506040830151848203604086015261579a8282614eb7565b915050606083015184820360608601526157b48282614eb7565b9150508091505092915050565b5f6020820190508181035f8301526157d9818461575a565b905092915050565b5f606083015f8301516157f65f860182614d39565b5060208301516158096020860182614d39565b50604083015184820360408601526158218282614eb7565b9150508091505092915050565b5f6020820190508181035f83015261584681846157e1565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f82825260208201905092915050565b5f819050919050565b5f6158a26020840184614dfd565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126158d2576158d16158b2565b5b83810192508235915060208301925067ffffffffffffffff8211156158fa576158f96158aa565b5b6001820236038313156159105761590f6158ae565b5b509250929050565b5f615923838561560b565b93506159308385846151f0565b61593983614c78565b840190509392505050565b5f606083016159555f840184615894565b6159615f860182614d39565b5061596f6020840184615894565b61597c6020860182614d39565b5061598a60408401846158b6565b858303604087015261599d838284615918565b925050508091505092915050565b5f6159b68383615944565b905092915050565b5f823560016060038336030381126159d9576159d86158b2565b5b82810191505092915050565b5f602082019050919050565b5f6159fc838561587b565b935083602084028501615a0e8461588b565b805f5b87811015615a51578484038952615a2882846159be565b615a3285826159ab565b9450615a3d836159e5565b925060208a01995050600181019050615a11565b50829750879450505050509392505050565b5f6020820190508181035f830152615a7c8184866159f1565b90509392505050565b5f81905092915050565b5f615a9982614c36565b615aa38185615a85565b9350615ab3818560208601614c50565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f615af3600283615a85565b9150615afe82615abf565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f615b3d600183615a85565b9150615b4882615b09565b600182019050919050565b5f615b5e8287615a8f565b9150615b6982615ae7565b9150615b758286615a8f565b9150615b8082615b31565b9150615b8c8285615a8f565b9150615b9782615b31565b9150615ba38284615a8f565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680615bf557607f821691505b602082108103615c0857615c07615bb1565b5b50919050565b5f82825260208201905092915050565b5f819050919050565b5f8083356001602003843603038112615c4357615c426158b2565b5b83810192508235915060208301925067ffffffffffffffff821115615c6b57615c6a6158aa565b5b600182023603831315615c8157615c806158ae565b5b509250929050565b5f615c948385614ea7565b9350615ca18385846151f0565b615caa83614c78565b840190509392505050565b5f60808301615cc65f840184615894565b615cd25f860182614d39565b50615ce06020840184615894565b615ced6020860182614d39565b50615cfb6040840184615c27565b8583036040870152615d0e838284615c89565b92505050615d1f6060840184615c27565b8583036060870152615d32838284615c89565b925050508091505092915050565b5f615d4b8383615cb5565b905092915050565b5f82356001608003833603038112615d6e57615d6d6158b2565b5b82810191505092915050565b5f602082019050919050565b5f615d918385615c0e565b935083602084028501615da384615c1e565b805f5b87811015615de6578484038952615dbd8284615d53565b615dc78582615d40565b9450615dd283615d7a565b925060208a01995050600181019050615da6565b50829750879450505050509392505050565b5f6020820190508181035f830152615e11818486615d86565b90509392505050565b5f67ffffffffffffffff82169050919050565b615e3681615e1a565b82525050565b5f602082019050615e4f5f830184615e2d565b92915050565b615e5e81614e3c565b8114615e68575f80fd5b50565b5f81519050615e7981615e55565b92915050565b5f60208284031215615e9457615e93614b24565b5b5f615ea184828501615e6b565b91505092915050565b5f60a0820190508181035f830152615ec381888a615d86565b9050615ed2602083018761501b565b615edf604083018661501b565b615eec606083018561501b565b615ef9608083018461501b565b979650505050505050565b5f82825260208201905092915050565b5f819050919050565b5f60608301615f2e5f840184615894565b615f3a5f860182614d39565b50615f486020840184615894565b615f556020860182614d39565b50615f636040840184615c27565b8583036040870152615f76838284615c89565b925050508091505092915050565b5f615f8f8383615f1d565b905092915050565b5f82356001606003833603038112615fb257615fb16158b2565b5b82810191505092915050565b5f602082019050919050565b5f615fd58385615f04565b935083602084028501615fe784615f14565b805f5b8781101561602a5784840389526160018284615f97565b61600b8582615f84565b945061601683615fbe565b925060208a01995050600181019050615fea565b50829750879450505050509392505050565b5f6040820190508181035f830152616055818587615fca565b9050616064602083018461501b565b949350505050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126160945761609361606c565b5b80840192508235915067ffffffffffffffff8211156160b6576160b5616070565b5b6020830192506001820236038313156160d2576160d1616074565b5b509250929050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026161407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82616105565b61614a8683616105565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61618561618061617b84614bd8565b616162565b614bd8565b9050919050565b5f819050919050565b61619e8361616b565b6161b26161aa8261618c565b848454616111565b825550505050565b5f90565b6161c66161ba565b6161d1818484616195565b505050565b5b818110156161f4576161e95f826161be565b6001810190506161d7565b5050565b601f8211156162395761620a816160e4565b616213846160f6565b81016020851015616222578190505b61623661622e856160f6565b8301826161d6565b50505b505050565b5f82821c905092915050565b5f6162595f198460080261623e565b1980831691505092915050565b5f616271838361624a565b9150826002028217905092915050565b61628b83836160da565b67ffffffffffffffff8111156162a4576162a3615148565b5b6162ae8254615bde565b6162b98282856161f8565b5f601f8311600181146162e6575f84156162d4578287013590505b6162de8582616266565b865550616345565b601f1984166162f4866160e4565b5f5b8281101561631b578489013582556001820191506020850194506020810190506162f6565b868310156163385784890135616334601f89168261624a565b8355505b6001600288020188555050505b50505050505050565b616359838383616281565b505050565b5f81015f830161636e8185616078565b61637981838661634e565b50505050600181016020830161638f8185616078565b61639a81838661634e565b505050505050565b6163ac828261635e565b5050565b5f604083016163c15f840184615c27565b8583035f8701526163d3838284615c89565b925050506163e46020840184615c27565b85830360208701526163f7838284615c89565b925050508091505092915050565b5f6164136020840184614bf7565b905092915050565b60a0820161642b5f830183616405565b6164375f850182614e98565b506164456020830183616405565b6164526020850182614e98565b506164606040830183616405565b61646d6040850182614e98565b5061647b6060830183616405565b6164886060850182614e98565b506164966080830183616405565b6164a36080850182614e98565b50505050565b5f610120820190508181035f8301526164c2818b6163b0565b90506164d1602083018a61641b565b81810360c08301526164e481888a615d86565b905081810360e08301526164f9818688615fca565b905081810361010083015261650f8184866159f1565b90509998505050505050505050565b5f813561652a81614be1565b80915050919050565b5f815f1b9050919050565b5f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61656984616533565b9350801983169250808416831791505092915050565b6165888261616b565b61659b6165948261618c565b835461653e565b8255505050565b5f81356165ae81614de7565b80915050919050565b5f73ffffffffffffffffffffffffffffffffffffffff6165d684616533565b9350801983169250808416831791505092915050565b5f6166066166016165fc84614d09565b616162565b614d09565b9050919050565b5f616617826165ec565b9050919050565b5f6166288261660d565b9050919050565b5f819050919050565b6166418261661e565b61665461664d8261662f565b83546165b7565b8255505050565b5f81015f83018061666b8161651e565b9050616677818461657f565b50505060018101602083018061668c816165a2565b90506166988184616638565b5050506002810160408301806166ad816165a2565b90506166b98184616638565b50505060038101606083016166ce8185616078565b6166d981838661634e565b5050505060048101608083016166ef8185616078565b6166fa81838661634e565b505050505050565b61670c828261665b565b5050565b5f60a083016167215f840184616405565b61672d5f860182614e98565b5061673b6020840184615894565b6167486020860182614d39565b506167566040840184615894565b6167636040860182614d39565b506167716060840184615c27565b8583036060870152616784838284615c89565b925050506167956080840184615c27565b85830360808701526167a8838284615c89565b925050508091505092915050565b5f6020820190508181035f8301526167ce8184616710565b905092915050565b5f823560016060038336030381126167f1576167f061606c565b5b80830191505092915050565b5f80833560016020038436030381126168195761681861606c565b5b80840192508235915067ffffffffffffffff82111561683b5761683a616070565b5b60208301925060018202360383131561685757616856616074565b5b509250929050565b5f82905092915050565b5f819050815f5260205f209050919050565b601f8211156168bc5761688d81616869565b616896846160f6565b810160208510156168a5578190505b6168b96168b1856160f6565b8301826161d6565b50505b505050565b6168cb838361685f565b67ffffffffffffffff8111156168e4576168e3615148565b5b6168ee8254615bde565b6168f982828561687b565b5f601f831160018114616926575f8415616914578287013590505b61691e8582616266565b865550616985565b601f19841661693486616869565b5f5b8281101561695b57848901358255600182019150602085019450602081019050616936565b868310156169785784890135616974601f89168261624a565b8355505b6001600288020188555050505b50505050505050565b6169998383836168c1565b505050565b5f81015f8301806169ae816165a2565b90506169ba8184616638565b5050506001810160208301806169cf816165a2565b90506169db8184616638565b50505060028101604083016169f081856167fd565b6169fb81838661698e565b505050505050565b616a0d828261699e565b5050565b5f604082019050616a245f83018561501b565b616a31602083018461501b565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b616a6e816152c6565b8114616a78575f80fd5b50565b5f81519050616a8981616a65565b92915050565b5f60208284031215616aa457616aa3614b24565b5b5f616ab184828501616a7b565b91505092915050565b5f82356001608003833603038112616ad557616ad461606c565b5b80830191505092915050565b5f81015f830180616af1816165a2565b9050616afd8184616638565b505050600181016020830180616b12816165a2565b9050616b1e8184616638565b5050506002810160408301616b338185616078565b616b3e81838661634e565b505050506003810160608301616b548185616078565b616b5f81838661634e565b505050505050565b616b718282616ae1565b5050565b5f82356001606003833603038112616b9057616b8f61606c565b5b80830191505092915050565b5f81015f830180616bac816165a2565b9050616bb88184616638565b505050600181016020830180616bcd816165a2565b9050616bd98184616638565b5050506002810160408301616bee8185616078565b616bf981838661634e565b505050505050565b616c0b8282616b9c565b5050565b5f81905092915050565b5f616c2382615601565b616c2d8185616c0f565b9350616c3d818560208601614c50565b80840191505092915050565b5f616c548284616c19565b91508190509291505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x87W_5`\xE0\x1C\x80cy\x8BX\xA6\x11a\x01YW\x80c\xBF\xF3\xAA\xBA\x11a\0\xC0W\x80c\xE3\x0C9x\x11a\0yW\x80c\xE3\x0C9x\x14a\tuW\x80c\xE3\xB2\xA8t\x14a\t\x9FW\x80c\xE5'^\xAF\x14a\t\xDBW\x80c\xEB\x84<\xF6\x14a\n\x17W\x80c\xEFi\x97\xF9\x14a\n?W\x80c\xF2\xFD\xE3\x8B\x14a\n{Wa\x02\x87V[\x80c\xBF\xF3\xAA\xBA\x14a\x08GW\x80c\xC2\xB4)\x86\x14a\x08\x83W\x80c\xC8\x0B3\xCA\x14a\x08\xADW\x80c\xCBZ\xA7\xE9\x14a\x08\xD5W\x80c\xD1\x0F\x7F\xF9\x14a\t\x11W\x80c\xD5\xE1k}\x14a\tMWa\x02\x87V[\x80c\x91d\xD0\xAE\x11a\x01\x12W\x80c\x91d\xD0\xAE\x14a\x07aW\x80c\x9AZ;\xC4\x14a\x07\x8BW\x80c\xAD<\xB1\xCC\x14a\x07\xA1W\x80c\xB4r+\xC4\x14a\x07\xCBW\x80c\xBA\x1F1\xD2\x14a\x07\xF5W\x80c\xBBY\xE3b\x14a\x08\x1FWa\x02\x87V[\x80cy\x8BX\xA6\x14a\x06}W\x80cy\xBAP\x97\x14a\x06\x93W\x80c~\xAA\xC8\xF2\x14a\x06\xA9W\x80c\x83\xBB.W\x14a\x06\xD3W\x80c\x88-}\xD3\x14a\x06\xFBW\x80c\x8D\xA5\xCB[\x14a\x077Wa\x02\x87V[\x80c-\xD3\xED\xFE\x11a\x01\xFDW\x80cS\xDA\x92F\x11a\x01\xB6W\x80cS\xDA\x92F\x14a\x05\x87W\x80c[\xAC\xE7\xFF\x14a\x05\xAFW\x80cg\x99\xEFR\x14a\x05\xEBW\x80cqP\x18\xA6\x14a\x06\x15W\x80ct \xF3\xD4\x14a\x06+W\x80cw-/\xE9\x14a\x06UWa\x02\x87V[\x80c-\xD3\xED\xFE\x14a\x04wW\x80c.-:\x82\x14a\x04\xB3W\x80cF\xFB\xF6\x8E\x14a\x04\xDBW\x80cH\x14La\x14a\x05\x17W\x80cO\x1E\xF2\x86\x14a\x05AW\x80cR\xD1\x90-\x14a\x05]Wa\x02\x87V[\x80c%\x85\xBBe\x11a\x02OW\x80c%\x85\xBBe\x14a\x03kW\x80c&\xCF]\xEF\x14a\x03\x95W\x80c'a,\xF6\x14a\x03\xBFW\x80c*8\x89\x98\x14a\x03\xE7W\x80c*\x8B\x9D\xE9\x14a\x04\x11W\x80c+\x10\x1C\x03\x14a\x04;Wa\x02\x87V[\x80c\x01=\xC2\x1E\x14a\x02\x8BW\x80c\x07$\xDD#\x14a\x02\xB3W\x80c\r\x8En,\x14a\x02\xDBW\x80c\x1E\xA5\xBDB\x14a\x03\x05W\x80c =\x01\x14\x14a\x03/W[_\x80\xFD[4\x80\x15a\x02\x96W_\x80\xFD[Pa\x02\xB1`\x04\x806\x03\x81\x01\x90a\x02\xAC\x91\x90aK\x8DV[a\n\xA3V[\0[4\x80\x15a\x02\xBEW_\x80\xFD[Pa\x02\xD9`\x04\x806\x03\x81\x01\x90a\x02\xD4\x91\x90aL\x0BV[a\r?V[\0[4\x80\x15a\x02\xE6W_\x80\xFD[Pa\x02\xEFa\r\x8AV[`@Qa\x02\xFC\x91\x90aL\xC0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x10W_\x80\xFD[Pa\x03\x19a\x0E\x05V[`@Qa\x03&\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03:W_\x80\xFD[Pa\x03U`\x04\x806\x03\x81\x01\x90a\x03P\x91\x90aN\x11V[a\x0E\x9EV[`@Qa\x03b\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03vW_\x80\xFD[Pa\x03\x7Fa\x0E\xFEV[`@Qa\x03\x8C\x91\x90aO\xFBV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xA0W_\x80\xFD[Pa\x03\xA9a\x11;V[`@Qa\x03\xB6\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xCAW_\x80\xFD[Pa\x03\xE5`\x04\x806\x03\x81\x01\x90a\x03\xE0\x91\x90aP\x98V[a\x11RV[\0[4\x80\x15a\x03\xF2W_\x80\xFD[Pa\x03\xFBa\x12\xDCV[`@Qa\x04\x08\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x1CW_\x80\xFD[Pa\x04%a\x12\xF3V[`@Qa\x042\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04FW_\x80\xFD[Pa\x04a`\x04\x806\x03\x81\x01\x90a\x04\\\x91\x90aN\x11V[a\x13\x8CV[`@Qa\x04n\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\x82W_\x80\xFD[Pa\x04\x9D`\x04\x806\x03\x81\x01\x90a\x04\x98\x91\x90aN\x11V[a\x13\xECV[`@Qa\x04\xAA\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xBEW_\x80\xFD[Pa\x04\xD9`\x04\x806\x03\x81\x01\x90a\x04\xD4\x91\x90aL\x0BV[a\x14LV[\0[4\x80\x15a\x04\xE6W_\x80\xFD[Pa\x05\x01`\x04\x806\x03\x81\x01\x90a\x04\xFC\x91\x90aN\x11V[a\x14\x97V[`@Qa\x05\x0E\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\"W_\x80\xFD[Pa\x05+a\x15+V[`@Qa\x058\x91\x90aQ$V[`@Q\x80\x91\x03\x90\xF3[a\x05[`\x04\x806\x03\x81\x01\x90a\x05V\x91\x90aRlV[a\x16qV[\0[4\x80\x15a\x05hW_\x80\xFD[Pa\x05qa\x16\x90V[`@Qa\x05~\x91\x90aR\xDEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x92W_\x80\xFD[Pa\x05\xAD`\x04\x806\x03\x81\x01\x90a\x05\xA8\x91\x90aR\xF7V[a\x16\xC1V[\0[4\x80\x15a\x05\xBAW_\x80\xFD[Pa\x05\xD5`\x04\x806\x03\x81\x01\x90a\x05\xD0\x91\x90aN\x11V[a\x19|V[`@Qa\x05\xE2\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xF6W_\x80\xFD[Pa\x05\xFFa\x19\xDCV[`@Qa\x06\x0C\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06 W_\x80\xFD[Pa\x06)a\x19\xF3V[\0[4\x80\x15a\x066W_\x80\xFD[Pa\x06?a\x1A\x06V[`@Qa\x06L\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06`W_\x80\xFD[Pa\x06{`\x04\x806\x03\x81\x01\x90a\x06v\x91\x90aL\x0BV[a\x1A\x9FV[\0[4\x80\x15a\x06\x88W_\x80\xFD[Pa\x06\x91a\x1A\xEAV[\0[4\x80\x15a\x06\x9EW_\x80\xFD[Pa\x06\xA7a\x1B\xFEV[\0[4\x80\x15a\x06\xB4W_\x80\xFD[Pa\x06\xBDa\x1C\x8CV[`@Qa\x06\xCA\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xDEW_\x80\xFD[Pa\x06\xF9`\x04\x806\x03\x81\x01\x90a\x06\xF4\x91\x90aS\xE2V[a\x1D%V[\0[4\x80\x15a\x07\x06W_\x80\xFD[Pa\x07!`\x04\x806\x03\x81\x01\x90a\x07\x1C\x91\x90aN\x11V[a\x1F\xC5V[`@Qa\x07.\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07BW_\x80\xFD[Pa\x07Ka %V[`@Qa\x07X\x91\x90aTNV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07lW_\x80\xFD[Pa\x07ua ZV[`@Qa\x07\x82\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x96W_\x80\xFD[Pa\x07\x9Fa \xF3V[\0[4\x80\x15a\x07\xACW_\x80\xFD[Pa\x07\xB5a\"IV[`@Qa\x07\xC2\x91\x90aL\xC0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\xD6W_\x80\xFD[Pa\x07\xDFa\"\x82V[`@Qa\x07\xEC\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\0W_\x80\xFD[Pa\x08\ta\"\x99V[`@Qa\x08\x16\x91\x90aM\xC7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08*W_\x80\xFD[Pa\x08E`\x04\x806\x03\x81\x01\x90a\x08@\x91\x90aT\xA7V[a#2V[\0[4\x80\x15a\x08RW_\x80\xFD[Pa\x08m`\x04\x806\x03\x81\x01\x90a\x08h\x91\x90aL\x0BV[a%TV[`@Qa\x08z\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\x8EW_\x80\xFD[Pa\x08\x97a%\x88V[`@Qa\x08\xA4\x91\x90aP*V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x08\xB8W_\x80\xFD[Pa\x08\xD3`\x04\x806\x03\x81\x01\x90a\x08\xCE\x91\x90aU\xBAV[a%\x9FV[\0[4\x80\x15a\x08\xE0W_\x80\xFD[Pa\x08\xFB`\x04\x806\x03\x81\x01\x90a\x08\xF6\x91\x90aN\x11V[a'MV[`@Qa\t\x08\x91\x90aV\xA0V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\x1CW_\x80\xFD[Pa\t7`\x04\x806\x03\x81\x01\x90a\t2\x91\x90aL\x0BV[a(\xEBV[`@Qa\tD\x91\x90aW:V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\tXW_\x80\xFD[Pa\ts`\x04\x806\x03\x81\x01\x90a\tn\x91\x90aL\x0BV[a+\x06V[\0[4\x80\x15a\t\x80W_\x80\xFD[Pa\t\x89a+QV[`@Qa\t\x96\x91\x90aTNV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xAAW_\x80\xFD[Pa\t\xC5`\x04\x806\x03\x81\x01\x90a\t\xC0\x91\x90aN\x11V[a+\x86V[`@Qa\t\xD2\x91\x90aW\xC1V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\t\xE6W_\x80\xFD[Pa\n\x01`\x04\x806\x03\x81\x01\x90a\t\xFC\x91\x90aN\x11V[a-\xB4V[`@Qa\n\x0E\x91\x90aNVV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\"W_\x80\xFD[Pa\n=`\x04\x806\x03\x81\x01\x90a\n8\x91\x90aL\x0BV[a.\x14V[\0[4\x80\x15a\nJW_\x80\xFD[Pa\ne`\x04\x806\x03\x81\x01\x90a\n`\x91\x90aN\x11V[a._V[`@Qa\nr\x91\x90aX.V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\n\x86W_\x80\xFD[Pa\n\xA1`\x04\x806\x03\x81\x01\x90a\n\x9C\x91\x90aN\x11V[a/\xFDV[\0[a\n\xABa0\xB6V[_a\n\xB4a1=V[\x90P_\x81`\x12\x01\x80T\x90P\x90P_[\x81\x81\x10\x15a\x0C\xD7W_\x83`\x14\x01_\x85`\x12\x01\x84\x81T\x81\x10a\n\xE7Wa\n\xE6aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x15\x01_\x85`\x13\x01\x84\x81T\x81\x10a\x0BzWa\x0ByaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82`\x11\x01_\x84`\x12\x01\x83\x81T\x81\x10a\x0C\x0CWa\x0C\x0BaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x80\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x01\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01_a\x0C\xC8\x91\x90aI\x0EV[PP\x80\x80`\x01\x01\x91PPa\n\xC3V[P\x81`\x12\x01_a\x0C\xE7\x91\x90aIKV[\x81`\x13\x01_a\x0C\xF6\x91\x90aIKV[a\r\0\x84\x84a1dV[\x7Fl\xDC\x1A\xA7n\x1E\xBA\xCDg\xC8\x1B\xE0\xDC\xF9`;]\xFB\xEBM\xD8\x01\xAB!A\x14\xAC\xB56\xF1\x10h\x84\x84`@Qa\r1\x92\x91\x90aZcV[`@Q\x80\x91\x03\x90\xA1PPPPV[a\rGa0\xB6V[a\rP\x81a4\xBBV[\x7F0\xC9\xB1\xD0\x04\xF5~\xAE<l\xC3\xA3u+\xCBL\x8E\xA2\xE5|\x82A\xA7\x82\xAA\x9Be\xFB\xC6\x04\xEC[\x81`@Qa\r\x7F\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FGatewayConfig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\r\xCB_a5_V[a\r\xD5`\x03a5_V[a\r\xDE_a5_V[`@Q` \x01a\r\xF1\x94\x93\x92\x91\x90a[SV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x0E\x10a1=V[\x90P\x80`\r\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0E\x93W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0EJW[PPPPP\x91PP\x90V[_\x80a\x0E\xA8a1=V[\x90P\x80`\x03\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[``_a\x0F\ta1=V[\x90P\x80`\x10\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x111W\x83\x82\x90_R` _ \x90`\x05\x02\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x03\x82\x01\x80Ta\x10\x12\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10>\x90a[\xDEV[\x80\x15a\x10\x89W\x80`\x1F\x10a\x10`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x89V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta\x10\xA2\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10\xCE\x90a[\xDEV[\x80\x15a\x11\x19W\x80`\x1F\x10a\x10\xF0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\x19V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0F.V[PPPP\x91PP\x90V[_\x80a\x11Ea1=V[\x90P\x80`\x07\x01T\x91PP\x90V[`\x04_a\x11]a6)V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x11\xA5WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x11\xDCW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x12*a1=V[\x90Pa\x12J\x85\x85\x83`\x07\x01T\x84`\x08\x01T\x85`\t\x01T\x86`\x16\x01Ta\x16\xC1V[\x7FC\t)\xCF\xFD\xF4;6\x81\xBB\x03\xEA\xE5\x80'E\xE35\xEC\x01Y\x96\x8D0\x1E*\x8E= \x9E\xEF\xB8\x85\x85`@Qa\x12{\x92\x91\x90a]\xF8V[`@Q\x80\x91\x03\x90\xA1P_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x12\xCE\x91\x90a^<V[`@Q\x80\x91\x03\x90\xA1PPPPV[_\x80a\x12\xE6a1=V[\x90P\x80`\x08\x01T\x91PP\x90V[``_a\x12\xFEa1=V[\x90P\x80`\x12\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x13\x81W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x138W[PPPPP\x91PP\x90V[_\x80a\x13\x96a1=V[\x90P\x80`\x0B\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x13\xF6a1=V[\x90P\x80`\n\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[a\x14Ta0\xB6V[a\x14]\x81a6PV[\x7F\xE4\x18\x02\xAFrW)\xAD\xCB\x8C\x15\x1E)78\n%\xC6\x91Uu~:\xF5\xD3\x97\x9A\xDA\xB5\x03X\0\x81`@Qa\x14\x8C\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[_s\xC3\xF9\xE1\xD2|\xD1\x04\x027[|\xD27\xD5~\x0FH\x88\xC1\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xFB\xF6\x8E\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xE5\x91\x90aTNV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15$\x91\x90a^\x7FV[\x90P\x91\x90PV[a\x153aIiV[_a\x15<a1=V[\x90P\x80_\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x15[\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x87\x90a[\xDEV[\x80\x15a\x15\xD2W\x80`\x1F\x10a\x15\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xD2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x15\xEB\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\x17\x90a[\xDEV[\x80\x15a\x16bW\x80`\x1F\x10a\x169Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16bV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x90V[a\x16ya6\xF4V[a\x16\x82\x82a7\xDAV[a\x16\x8C\x82\x82a7\xE5V[PPV[_a\x16\x99a9\x03V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[a\x16\xC9a0\xB6V[_a\x16\xD2a1=V[\x90P_\x81`\x05\x01\x80T\x90P\x90P_[\x81\x81\x10\x15a\x19\x04W_\x83`\x02\x01_\x85`\x05\x01\x84\x81T\x81\x10a\x17\x05Wa\x17\x04aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x03\x01_\x85`\x06\x01\x84\x81T\x81\x10a\x17\x98Wa\x17\x97aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82`\x04\x01_\x84`\x05\x01\x83\x81T\x81\x10a\x18*Wa\x18)aXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x80\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x01\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01_a\x18\xE6\x91\x90aI\x83V[`\x03\x82\x01_a\x18\xF5\x91\x90aI\x83V[PP\x80\x80`\x01\x01\x91PPa\x16\xE1V[P\x81`\x05\x01_a\x19\x14\x91\x90aIKV[\x81`\x06\x01_a\x19#\x91\x90aIKV[a\x191\x88\x88\x88\x88\x88\x88a9\x8AV[\x7F%\xD1\xEAdq(\xB5mG\xE6E4\xCD\x0FZ\x86\xD3 \x7Fg\xB0H\x95I[f\xDC\r\xB8z\x0C\xA7\x88\x88\x88\x88\x88\x88`@Qa\x19j\x96\x95\x94\x93\x92\x91\x90a^\xAAV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x19\x86a1=V[\x90P\x80`\x14\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x19\xE6a1=V[\x90P\x80`\x17\x01T\x91PP\x90V[a\x19\xFBa0\xB6V[a\x1A\x04_a=\tV[V[``_a\x1A\x11a1=V[\x90P\x80`\x05\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1A\x94W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1AKW[PPPPP\x91PP\x90V[a\x1A\xA7a0\xB6V[a\x1A\xB0\x81a=FV[\x7F5q\x17*I\xE7-w$\xBE8L\xDDY\xF4\xF2\x1A!lp5.\xA5\x9C\xB0%C\xFCv0\x847\x81`@Qa\x1A\xDF\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[a\x1A\xF2a0\xB6V[s\x87\xA5\xB1\x15*\xA5\x17(%\x8D\xBC\x1A\xA5Kj\x83\xDC\xD1\xD3\xDDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c?K\xA8:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BKW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B]W=_\x80>=_\xFD[PPPPs3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNns\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c?K\xA8:`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xBAW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xCCW=_\x80>=_\xFD[PPPP\x7F\xBEOe]\xAA\xE0\xDB\xAE\xF6:kR\\\xAB/\xA6\xAC\xE4\xAA[\x94\xB8\x83K$\x117\xCD\xFEs\xA5\xB0`@Q`@Q\x80\x91\x03\x90\xA1V[_a\x1C\x07a=\xB0V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1C(a+QV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1C\x80W\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Cw\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[a\x1C\x89\x81a=\tV[PV[``_a\x1C\x97a1=V[\x90P\x80`\x06\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1D\x1AW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1C\xD1W[PPPPP\x91PP\x90V[a\x1D-a0\xB6V[_a\x1D6a1=V[\x90P_\x81`\r\x01\x80T\x90P\x90P_[\x81\x81\x10\x15a\x1FYW_\x83`\n\x01_\x85`\r\x01\x84\x81T\x81\x10a\x1DiWa\x1DhaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x83`\x0B\x01_\x85`\x0E\x01\x84\x81T\x81\x10a\x1D\xFCWa\x1D\xFBaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82`\x0C\x01_\x84`\r\x01\x83\x81T\x81\x10a\x1E\x8EWa\x1E\x8DaXNV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x80\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x01\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U`\x02\x82\x01_a\x1FJ\x91\x90aI\x83V[PP\x80\x80`\x01\x01\x91PPa\x1DEV[P\x81`\r\x01_a\x1Fi\x91\x90aIKV[\x81`\x0E\x01_a\x1Fx\x91\x90aIKV[a\x1F\x83\x85\x85\x85a=\xB7V[\x7F\xFF\xE2\x0B\xDB\x85^QN\x94\x14w\x02\x92&\x90\xCF\x1D\xA1\x0B\xDD\x18\xBF\x1Fb\x15\x02|\x93\xAC\x05\xD4U\x85\x85\x85`@Qa\x1F\xB6\x93\x92\x91\x90a`<V[`@Q\x80\x91\x03\x90\xA1PPPPPV[_\x80a\x1F\xCFa1=V[\x90P\x80`\x15\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a /aA\x18V[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[``_a ea1=V[\x90P\x80`\x0E\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a \xE8W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a \x9FW[PPPPP\x91PP\x90V[a \xFC3a\x14\x97V[a!=W3`@Q\x7F j4n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!4\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[s\x87\xA5\xB1\x15*\xA5\x17(%\x8D\xBC\x1A\xA5Kj\x83\xDC\xD1\xD3\xDDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x84V\xCBY`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!\x96W_\x80\xFD[PZ\xF1\x15\x80\x15a!\xA8W=_\x80>=_\xFD[PPPPs3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNns\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x84V\xCBY`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"\x05W_\x80\xFD[PZ\xF1\x15\x80\x15a\"\x17W=_\x80>=_\xFD[PPPP\x7F\x13\xDB\xE8\x822\x19\xE2&\xDD\x05%\xAE\xB0q\xE1\xD2g\x9F\x898+\xA7\x99\xF7\xF6D\x86~e\xB6\xF3\xA6`@Q`@Q\x80\x91\x03\x90\xA1V[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\"\x8Ca1=V[\x90P\x80`\x16\x01T\x91PP\x90V[``_a\"\xA4a1=V[\x90P\x80`\x13\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a#'W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\"\xDEW[PPPPP\x91PP\x90V[`\x01a#<aA?V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a#}W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04_a#\x88a6)V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a#\xD0WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a$\x07W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa$\\a$Wa %V[aAcV[_a$ea1=V[\x90P\x8A\x81_\x01\x81\x81a$w\x91\x90ac\xA2V[\x90PPa$\x97\x89\x89\x8C_\x015\x8D` \x015\x8E`@\x015\x8F``\x015a9\x8AV[a$\xA6\x87\x87\x8C`\x80\x015a=\xB7V[a$\xB0\x85\x85a1dV[\x7F\xB2\xCB\xE6^\xA3\x08\xBF\xE4\xB9C\x18\x19\xA3\x16\x8DTOF\xBA4K\x1Ey\xF9/\x97?\xCF\xF4:\xAE;\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B`@Qa$\xED\x98\x97\x96\x95\x94\x93\x92\x91\x90ad\xA9V[`@Q\x80\x91\x03\x90\xA1P_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa%@\x91\x90a^<V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPV[_\x80a%^a1=V[\x90P\x80`\x0F\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a%\x92a1=V[\x90P\x80`\t\x01T\x91PP\x90V[a%\xA7a0\xB6V[_\x81_\x015\x03a%\xE3W`@Q\x7F\"\xF7?\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x015\x11\x15a&8W\x80_\x015`@Q\x7FAx\xDEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&/\x91\x90aP*V[`@Q\x80\x91\x03\x90\xFD[_a&Aa1=V[\x90P\x80`\x0F\x01_\x83_\x015\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a&\xABW\x81_\x015`@Q\x7F\x96\xA5h(\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xA2\x91\x90aP*V[`@Q\x80\x91\x03\x90\xFD[\x80`\x10\x01\x82\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x05\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a&\xE2\x91\x90ag\x02V[PP`\x01\x81`\x0F\x01_\x84_\x015\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7Ffv\x93A\xEF\xFD&\x8F\xC4\xE9\xA9\xC8\xF2{\xFC\x96\x85\x07\xB5\x19\xB0\xDD\xB9\xB4\xAD=\xED_\x03\x01h7\x82`@Qa'A\x91\x90ag\xB6V[`@Q\x80\x91\x03\x90\xA1PPV[a'UaI\xC0V[_a'^a1=V[\x90P\x80`\x11\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta(c\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\x8F\x90a[\xDEV[\x80\x15a(\xDAW\x80`\x1F\x10a(\xB1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(\xDAV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(\xBDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[a(\xF3aJ\x0BV[_a(\xFCa1=V[\x90P\x80`\x10\x01\x83\x81T\x81\x10a)\x14Wa)\x13aXNV[[\x90_R` _ \x90`\x05\x02\x01`@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x03\x82\x01\x80Ta)\xEE\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\x1A\x90a[\xDEV[\x80\x15a*eW\x80`\x1F\x10a*<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*eV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*HW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x04\x82\x01\x80Ta*~\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\xAA\x90a[\xDEV[\x80\x15a*\xF5W\x80`\x1F\x10a*\xCCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*\xF5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\xD8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[a+\x0Ea0\xB6V[a+\x17\x81aAwV[\x7Fz.\xF7\xDC\x89@\n\x8A\xD9+\xB4\xCC\xF4MH&$\xB4\x0F\xE7kf\x97~\x85\xEDja\x8E./\xC7\x81`@Qa+F\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[_\x80a+[aB\x1BV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[a+\x8EaJcV[_a+\x97a1=V[\x90P\x80`\x04\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta,\x9C\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,\xC8\x90a[\xDEV[\x80\x15a-\x13W\x80`\x1F\x10a,\xEAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\x13V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,\xF6W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta-,\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta-X\x90a[\xDEV[\x80\x15a-\xA3W\x80`\x1F\x10a-zWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\xA3V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a-\x86W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[_\x80a-\xBEa1=V[\x90P\x80`\x02\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[a.\x1Ca0\xB6V[a.%\x81aBBV[\x7F\x83~\ne(\xDA\xDF\xA2\xDCy&\x92\xC5\x18.R\xA9\xF5\xBB\xDE\xED{#r\x92z&\xC6\x95\x83\x96\x13\x81`@Qa.T\x91\x90aP*V[`@Q\x80\x91\x03\x90\xA1PV[a.gaJ\xB5V[_a.pa1=V[\x90P\x80`\x0C\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta/u\x90a[\xDEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xA1\x90a[\xDEV[\x80\x15a/\xECW\x80`\x1F\x10a/\xC3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a/\xECV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a/\xCFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x91PP\x91\x90PV[a0\x05a0\xB6V[_a0\x0EaB\x1BV[\x90P\x81\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a0pa %V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PPV[a0\xBEa=\xB0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a0\xDCa %V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a1;Wa0\xFFa=\xB0V[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a12\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x86\xD3\x07\n\x89\x93\xF6\xB2\t\xBE\xE6\x18Q\x86\xD3\x8A\x07\xFC\xE8\xBB\xD9|u\r\x93DQ\xB7/5\xB4\0\x90P\x90V[_\x82\x82\x90P\x03a1\xA0W`@Q\x7F\xCA\xD1\xD54\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a1\xA9a1=V[\x90P_[\x83\x83\x90P\x81\x10\x15a4\xB5W\x83\x83\x82\x81\x81\x10a1\xCBWa1\xCAaXNV[[\x90P` \x02\x81\x01\x90a1\xDD\x91\x90ag\xD6V[\x82`\x11\x01_\x86\x86\x85\x81\x81\x10a1\xF5Wa1\xF4aXNV[[\x90P` \x02\x81\x01\x90a2\x07\x91\x90ag\xD6V[_\x01` \x81\x01\x90a2\x18\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a2]\x91\x90aj\x03V[\x90PP\x81`\x12\x01\x84\x84\x83\x81\x81\x10a2wWa2vaXNV[[\x90P` \x02\x81\x01\x90a2\x89\x91\x90ag\xD6V[_\x01` \x81\x01\x90a2\x9A\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x14\x01_\x86\x86\x85\x81\x81\x10a3\x11Wa3\x10aXNV[[\x90P` \x02\x81\x01\x90a3#\x91\x90ag\xD6V[_\x01` \x81\x01\x90a34\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`\x13\x01\x84\x84\x83\x81\x81\x10a3\x9AWa3\x99aXNV[[\x90P` \x02\x81\x01\x90a3\xAC\x91\x90ag\xD6V[` \x01` \x81\x01\x90a3\xBE\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x15\x01_\x86\x86\x85\x81\x81\x10a45Wa44aXNV[[\x90P` \x02\x81\x01\x90a4G\x91\x90ag\xD6V[` \x01` \x81\x01\x90a4Y\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa1\xADV[PPPPV[_a4\xC4a1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P_\x83\x03a5\nW`@Q\x7F>\xE5\x07t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a5QW\x82\x81`@Q\x7F\x0Fi\xCB\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5H\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x16\x01\x81\x90UPPPPV[``_`\x01a5m\x84aB\xE6V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x8BWa5\x8AaQHV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a5\xBDW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a6\x1EW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a6\x13Wa6\x12aj8V[[\x04\x94P_\x85\x03a5\xCAW[\x81\x93PPPP\x91\x90PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_a6Ya1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P_\x83\x03a6\x9FW`@Q\x7F\xB1\xAE\x92\xEA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a6\xE6W\x82\x81`@Q\x7F\x84 \x8F#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a6\xDD\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x08\x01\x81\x90UPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a7\xA1WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a7\x88aD7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a7\xD8W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a7\xE2a0\xB6V[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a8MWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8J\x91\x90aj\x8FV[`\x01[a8\x8EW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\x85\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a8\xF4W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a8\xEB\x91\x90aR\xDEV[`@Q\x80\x91\x03\x90\xFD[a8\xFE\x83\x83aD\x8AV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a9\x88W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x86\x86\x90P\x03a9\xC6W`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a9\xCFa1=V[\x90P_[\x87\x87\x90P\x81\x10\x15a<\xDBW`\x01\x82`\x02\x01_\x8A\x8A\x85\x81\x81\x10a9\xF8Wa9\xF7aXNV[[\x90P` \x02\x81\x01\x90a:\n\x91\x90aj\xBAV[_\x01` \x81\x01\x90a:\x1B\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x87\x87\x82\x81\x81\x10a:}Wa:|aXNV[[\x90P` \x02\x81\x01\x90a:\x8F\x91\x90aj\xBAV[\x82`\x04\x01_\x8A\x8A\x85\x81\x81\x10a:\xA7Wa:\xA6aXNV[[\x90P` \x02\x81\x01\x90a:\xB9\x91\x90aj\xBAV[_\x01` \x81\x01\x90a:\xCA\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a;\x0F\x91\x90akgV[\x90PP\x81`\x05\x01\x88\x88\x83\x81\x81\x10a;)Wa;(aXNV[[\x90P` \x02\x81\x01\x90a;;\x91\x90aj\xBAV[_\x01` \x81\x01\x90a;L\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x03\x01_\x8A\x8A\x85\x81\x81\x10a;\xC3Wa;\xC2aXNV[[\x90P` \x02\x81\x01\x90a;\xD5\x91\x90aj\xBAV[` \x01` \x81\x01\x90a;\xE7\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`\x06\x01\x88\x88\x83\x81\x81\x10a<MWa<LaXNV[[\x90P` \x02\x81\x01\x90a<_\x91\x90aj\xBAV[` \x01` \x81\x01\x90a<q\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa9\xD3V[Pa<\xE5\x85a=FV[a<\xEE\x84a6PV[a<\xF7\x83aBBV[a=\0\x82a4\xBBV[PPPPPPPV[_a=\x12aB\x1BV[\x90P\x80_\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua=B\x82aD\xFCV[PPV[_a=Oa1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P\x80\x83\x10a=\xA2W\x82\x81`@Q\x7F\x90~f\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\x99\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x07\x01\x81\x90UPPPPV[_3\x90P\x90V[_\x83\x83\x90P\x03a=\xF3W`@Q\x7F\x8A\xF0\x82\xEF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a=\xFCa1=V[\x90P_[\x84\x84\x90P\x81\x10\x15aA\x08W`\x01\x82`\n\x01_\x87\x87\x85\x81\x81\x10a>%Wa>$aXNV[[\x90P` \x02\x81\x01\x90a>7\x91\x90akuV[_\x01` \x81\x01\x90a>H\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x84\x84\x82\x81\x81\x10a>\xAAWa>\xA9aXNV[[\x90P` \x02\x81\x01\x90a>\xBC\x91\x90akuV[\x82`\x0C\x01_\x87\x87\x85\x81\x81\x10a>\xD4Wa>\xD3aXNV[[\x90P` \x02\x81\x01\x90a>\xE6\x91\x90akuV[_\x01` \x81\x01\x90a>\xF7\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a?<\x91\x90al\x01V[\x90PP\x81`\r\x01\x85\x85\x83\x81\x81\x10a?VWa?UaXNV[[\x90P` \x02\x81\x01\x90a?h\x91\x90akuV[_\x01` \x81\x01\x90a?y\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x82`\x0B\x01_\x87\x87\x85\x81\x81\x10a?\xF0Wa?\xEFaXNV[[\x90P` \x02\x81\x01\x90a@\x02\x91\x90akuV[` \x01` \x81\x01\x90a@\x14\x91\x90aN\x11V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81`\x0E\x01\x85\x85\x83\x81\x81\x10a@zWa@yaXNV[[\x90P` \x02\x81\x01\x90a@\x8C\x91\x90akuV[` \x01` \x81\x01\x90a@\x9E\x91\x90aN\x11V[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x80\x80`\x01\x01\x91PPa>\0V[PaA\x12\x82aAwV[PPPPV[_\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90P\x90V[_aAHa6)V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aAkaE\xCDV[aAt\x81aF\rV[PV[_aA\x80a1=V[\x90P_\x81`\x0E\x01\x80T\x90P\x90P_\x83\x03aA\xC6W`@Q\x7F\xB6\r$A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15aB\rW\x82\x81`@Q\x7F\x97\xBE\xAB\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\x04\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\x17\x01\x81\x90UPPPPV[_\x7F#~\x15\x82\"\xE3\xE6\x96\x8Br\xB9\xDB\r\x80C\xAA\xCF\x07J\xD9\xF6P\xF0\xD1`kM\x82\xEEC,\0\x90P\x90V[_aBKa1=V[\x90P_\x81`\x06\x01\x80T\x90P\x90P_\x83\x03aB\x91W`@Q\x7F\xE6\nrq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15aB\xD8W\x82\x81`@Q\x7F\xD2S^\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aB\xCF\x92\x91\x90aj\x11V[`@Q\x80\x91\x03\x90\xFD[\x82\x82`\t\x01\x81\x90UPPPPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aCBWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aC8WaC7aj8V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aC\x7FWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aCuWaCtaj8V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aC\xAEWf#\x86\xF2o\xC1\0\0\x83\x81aC\xA4WaC\xA3aj8V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aC\xD7Wc\x05\xF5\xE1\0\x83\x81aC\xCDWaC\xCCaj8V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aC\xFCWa'\x10\x83\x81aC\xF2WaC\xF1aj8V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aD\x1FW`d\x83\x81aD\x15WaD\x14aj8V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aD.W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_aDc\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaF\x91V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aD\x93\x82aF\x9AV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aD\xEFWaD\xE9\x82\x82aGcV[PaD\xF8V[aD\xF7aG\xE3V[[PPV[_aE\x05aA\x18V[\x90P_\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82\x82_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPPV[aE\xD5aH\x1FV[aF\x0BW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aF\x15aE\xCDV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aF\x85W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aF|\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[aF\x8E\x81a=\tV[PV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aF\xF5W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aF\xEC\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[\x80aG!\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaF\x91V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaG\x8C\x91\x90alIV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aG\xC4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aG\xC9V[``\x91P[P\x91P\x91PaG\xD9\x85\x83\x83aH=V[\x92PPP\x92\x91PPV[_4\x11\x15aH\x1DW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aH(a6)V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[``\x82aHRWaHM\x82aH\xCAV[aH\xC2V[_\x82Q\x14\x80\x15aHxWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aH\xBAW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aH\xB1\x91\x90aTNV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaH\xC3V[[\x93\x92PPPV[_\x81Q\x11\x15aH\xDCW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80TaI\x1A\x90a[\xDEV[_\x82U\x80`\x1F\x10aI+WPaIHV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90aIG\x91\x90aK\0V[[PV[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90aIf\x91\x90aK\0V[PV[`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[P\x80TaI\x8F\x90a[\xDEV[_\x82U\x80`\x1F\x10aI\xA0WPaI\xBDV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90aI\xBC\x91\x90aK\0V[[PV[`@Q\x80``\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Q\x80``\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81RP\x90V[[\x80\x82\x11\x15aK\x17W_\x81_\x90UP`\x01\x01aK\x01V[P\x90V[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aKMWaKLaK,V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aKjWaKiaK0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aK\x86WaK\x85aK4V[[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15aK\xA3WaK\xA2aK$V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xC0WaK\xBFaK(V[[aK\xCC\x85\x82\x86\x01aK8V[\x92P\x92PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aK\xEA\x81aK\xD8V[\x81\x14aK\xF4W_\x80\xFD[PV[_\x815\x90PaL\x05\x81aK\xE1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aL WaL\x1FaK$V[[_aL-\x84\x82\x85\x01aK\xF7V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aLmW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaLRV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aL\x92\x82aL6V[aL\x9C\x81\x85aL@V[\x93PaL\xAC\x81\x85` \x86\x01aLPV[aL\xB5\x81aLxV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaL\xD8\x81\x84aL\x88V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aM2\x82aM\tV[\x90P\x91\x90PV[aMB\x81aM(V[\x82RPPV[_aMS\x83\x83aM9V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aMu\x82aL\xE0V[aM\x7F\x81\x85aL\xEAV[\x93PaM\x8A\x83aL\xFAV[\x80_[\x83\x81\x10\x15aM\xBAW\x81QaM\xA1\x88\x82aMHV[\x97PaM\xAC\x83aM_V[\x92PP`\x01\x81\x01\x90PaM\x8DV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM\xDF\x81\x84aMkV[\x90P\x92\x91PPV[aM\xF0\x81aM(V[\x81\x14aM\xFAW_\x80\xFD[PV[_\x815\x90PaN\x0B\x81aM\xE7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aN&WaN%aK$V[[_aN3\x84\x82\x85\x01aM\xFDV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[aNP\x81aN<V[\x82RPPV[_` \x82\x01\x90PaNi_\x83\x01\x84aNGV[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aN\xA1\x81aK\xD8V[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aN\xC1\x82aL6V[aN\xCB\x81\x85aN\xA7V[\x93PaN\xDB\x81\x85` \x86\x01aLPV[aN\xE4\x81aLxV[\x84\x01\x91PP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01QaO\x04_\x86\x01\x82aN\x98V[P` \x83\x01QaO\x17` \x86\x01\x82aM9V[P`@\x83\x01QaO*`@\x86\x01\x82aM9V[P``\x83\x01Q\x84\x82\x03``\x86\x01RaOB\x82\x82aN\xB7V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01RaO\\\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_aOt\x83\x83aN\xEFV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aO\x92\x82aNoV[aO\x9C\x81\x85aNyV[\x93P\x83` \x82\x02\x85\x01aO\xAE\x85aN\x89V[\x80_[\x85\x81\x10\x15aO\xE9W\x84\x84\x03\x89R\x81QaO\xCA\x85\x82aOiV[\x94PaO\xD5\x83aO|V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaO\xB1V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaP\x13\x81\x84aO\x88V[\x90P\x92\x91PPV[aP$\x81aK\xD8V[\x82RPPV[_` \x82\x01\x90PaP=_\x83\x01\x84aP\x1BV[\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aPXWaPWaK,V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aPuWaPtaK0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aP\x91WaP\x90aK4V[[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15aP\xAEWaP\xADaK$V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aP\xCBWaP\xCAaK(V[[aP\xD7\x85\x82\x86\x01aPCV[\x92P\x92PP\x92P\x92\x90PV[_`@\x83\x01_\x83\x01Q\x84\x82\x03_\x86\x01RaP\xFD\x82\x82aN\xB7V[\x91PP` \x83\x01Q\x84\x82\x03` \x86\x01RaQ\x17\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaQ<\x81\x84aP\xE3V[\x90P\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aQ~\x82aLxV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aQ\x9DWaQ\x9CaQHV[[\x80`@RPPPV[_aQ\xAFaK\x1BV[\x90PaQ\xBB\x82\x82aQuV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQ\xDAWaQ\xD9aQHV[[aQ\xE3\x82aLxV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aR\x10aR\x0B\x84aQ\xC0V[aQ\xA6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aR,WaR+aQDV[[aR7\x84\x82\x85aQ\xF0V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aRSWaRRaK,V[[\x815aRc\x84\x82` \x86\x01aQ\xFEV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aR\x82WaR\x81aK$V[[_aR\x8F\x85\x82\x86\x01aM\xFDV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aR\xB0WaR\xAFaK(V[[aR\xBC\x85\x82\x86\x01aR?V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aR\xD8\x81aR\xC6V[\x82RPPV[_` \x82\x01\x90PaR\xF1_\x83\x01\x84aR\xCFV[\x92\x91PPV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15aS\x11WaS\x10aK$V[[_\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS.WaS-aK(V[[aS:\x89\x82\x8A\x01aPCV[\x96P\x96PP` aSM\x89\x82\x8A\x01aK\xF7V[\x94PP`@aS^\x89\x82\x8A\x01aK\xF7V[\x93PP``aSo\x89\x82\x8A\x01aK\xF7V[\x92PP`\x80aS\x80\x89\x82\x8A\x01aK\xF7V[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80\x83`\x1F\x84\x01\x12aS\xA2WaS\xA1aK,V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xBFWaS\xBEaK0V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aS\xDBWaS\xDAaK4V[[\x92P\x92\x90PV[_\x80_`@\x84\x86\x03\x12\x15aS\xF9WaS\xF8aK$V[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\x16WaT\x15aK(V[[aT\"\x86\x82\x87\x01aS\x8DV[\x93P\x93PP` aT5\x86\x82\x87\x01aK\xF7V[\x91PP\x92P\x92P\x92V[aTH\x81aM(V[\x82RPPV[_` \x82\x01\x90PaTa_\x83\x01\x84aT?V[\x92\x91PPV[_\x80\xFD[_`@\x82\x84\x03\x12\x15aT\x80WaT\x7FaTgV[[\x81\x90P\x92\x91PPV[_`\xA0\x82\x84\x03\x12\x15aT\x9EWaT\x9DaTgV[[\x81\x90P\x92\x91PPV[_\x80_\x80_\x80_\x80a\x01 \x89\x8B\x03\x12\x15aT\xC4WaT\xC3aK$V[[_\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aT\xE1WaT\xE0aK(V[[aT\xED\x8B\x82\x8C\x01aTkV[\x98PP` aT\xFE\x8B\x82\x8C\x01aT\x89V[\x97PP`\xC0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\x1FWaU\x1EaK(V[[aU+\x8B\x82\x8C\x01aPCV[\x96P\x96PP`\xE0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aUNWaUMaK(V[[aUZ\x8B\x82\x8C\x01aS\x8DV[\x94P\x94PPa\x01\0\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU~WaU}aK(V[[aU\x8A\x8B\x82\x8C\x01aK8V[\x92P\x92PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[_`\xA0\x82\x84\x03\x12\x15aU\xB1WaU\xB0aTgV[[\x81\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15aU\xCFWaU\xCEaK$V[[_\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xECWaU\xEBaK(V[[aU\xF8\x84\x82\x85\x01aU\x9CV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aV%\x82aV\x01V[aV/\x81\x85aV\x0BV[\x93PaV?\x81\x85` \x86\x01aLPV[aVH\x81aLxV[\x84\x01\x91PP\x92\x91PPV[_``\x83\x01_\x83\x01QaVh_\x86\x01\x82aM9V[P` \x83\x01QaV{` \x86\x01\x82aM9V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaV\x93\x82\x82aV\x1BV[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaV\xB8\x81\x84aVSV[\x90P\x92\x91PPV[_`\xA0\x83\x01_\x83\x01QaV\xD5_\x86\x01\x82aN\x98V[P` \x83\x01QaV\xE8` \x86\x01\x82aM9V[P`@\x83\x01QaV\xFB`@\x86\x01\x82aM9V[P``\x83\x01Q\x84\x82\x03``\x86\x01RaW\x13\x82\x82aN\xB7V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01RaW-\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaWR\x81\x84aV\xC0V[\x90P\x92\x91PPV[_`\x80\x83\x01_\x83\x01QaWo_\x86\x01\x82aM9V[P` \x83\x01QaW\x82` \x86\x01\x82aM9V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaW\x9A\x82\x82aN\xB7V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01RaW\xB4\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xD9\x81\x84aWZV[\x90P\x92\x91PPV[_``\x83\x01_\x83\x01QaW\xF6_\x86\x01\x82aM9V[P` \x83\x01QaX\t` \x86\x01\x82aM9V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaX!\x82\x82aN\xB7V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaXF\x81\x84aW\xE1V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aX\xA2` \x84\x01\x84aM\xFDV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aX\xD2WaX\xD1aX\xB2V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aX\xFAWaX\xF9aX\xAAV[[`\x01\x82\x026\x03\x83\x13\x15aY\x10WaY\x0FaX\xAEV[[P\x92P\x92\x90PV[_aY#\x83\x85aV\x0BV[\x93PaY0\x83\x85\x84aQ\xF0V[aY9\x83aLxV[\x84\x01\x90P\x93\x92PPPV[_``\x83\x01aYU_\x84\x01\x84aX\x94V[aYa_\x86\x01\x82aM9V[PaYo` \x84\x01\x84aX\x94V[aY|` \x86\x01\x82aM9V[PaY\x8A`@\x84\x01\x84aX\xB6V[\x85\x83\x03`@\x87\x01RaY\x9D\x83\x82\x84aY\x18V[\x92PPP\x80\x91PP\x92\x91PPV[_aY\xB6\x83\x83aYDV[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12aY\xD9WaY\xD8aX\xB2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aY\xFC\x83\x85aX{V[\x93P\x83` \x84\x02\x85\x01aZ\x0E\x84aX\x8BV[\x80_[\x87\x81\x10\x15aZQW\x84\x84\x03\x89RaZ(\x82\x84aY\xBEV[aZ2\x85\x82aY\xABV[\x94PaZ=\x83aY\xE5V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaZ\x11V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaZ|\x81\x84\x86aY\xF1V[\x90P\x93\x92PPPV[_\x81\x90P\x92\x91PPV[_aZ\x99\x82aL6V[aZ\xA3\x81\x85aZ\x85V[\x93PaZ\xB3\x81\x85` \x86\x01aLPV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aZ\xF3`\x02\x83aZ\x85V[\x91PaZ\xFE\x82aZ\xBFV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a[=`\x01\x83aZ\x85V[\x91Pa[H\x82a[\tV[`\x01\x82\x01\x90P\x91\x90PV[_a[^\x82\x87aZ\x8FV[\x91Pa[i\x82aZ\xE7V[\x91Pa[u\x82\x86aZ\x8FV[\x91Pa[\x80\x82a[1V[\x91Pa[\x8C\x82\x85aZ\x8FV[\x91Pa[\x97\x82a[1V[\x91Pa[\xA3\x82\x84aZ\x8FV[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a[\xF5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\\\x08Wa\\\x07a[\xB1V[[P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a\\CWa\\BaX\xB2V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\\kWa\\jaX\xAAV[[`\x01\x82\x026\x03\x83\x13\x15a\\\x81Wa\\\x80aX\xAEV[[P\x92P\x92\x90PV[_a\\\x94\x83\x85aN\xA7V[\x93Pa\\\xA1\x83\x85\x84aQ\xF0V[a\\\xAA\x83aLxV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x83\x01a\\\xC6_\x84\x01\x84aX\x94V[a\\\xD2_\x86\x01\x82aM9V[Pa\\\xE0` \x84\x01\x84aX\x94V[a\\\xED` \x86\x01\x82aM9V[Pa\\\xFB`@\x84\x01\x84a\\'V[\x85\x83\x03`@\x87\x01Ra]\x0E\x83\x82\x84a\\\x89V[\x92PPPa]\x1F``\x84\x01\x84a\\'V[\x85\x83\x03``\x87\x01Ra]2\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_a]K\x83\x83a\\\xB5V[\x90P\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12a]nWa]maX\xB2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a]\x91\x83\x85a\\\x0EV[\x93P\x83` \x84\x02\x85\x01a]\xA3\x84a\\\x1EV[\x80_[\x87\x81\x10\x15a]\xE6W\x84\x84\x03\x89Ra]\xBD\x82\x84a]SV[a]\xC7\x85\x82a]@V[\x94Pa]\xD2\x83a]zV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa]\xA6V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\x11\x81\x84\x86a]\x86V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a^6\x81a^\x1AV[\x82RPPV[_` \x82\x01\x90Pa^O_\x83\x01\x84a^-V[\x92\x91PPV[a^^\x81aN<V[\x81\x14a^hW_\x80\xFD[PV[_\x81Q\x90Pa^y\x81a^UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a^\x94Wa^\x93aK$V[[_a^\xA1\x84\x82\x85\x01a^kV[\x91PP\x92\x91PPV[_`\xA0\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\xC3\x81\x88\x8Aa]\x86V[\x90Pa^\xD2` \x83\x01\x87aP\x1BV[a^\xDF`@\x83\x01\x86aP\x1BV[a^\xEC``\x83\x01\x85aP\x1BV[a^\xF9`\x80\x83\x01\x84aP\x1BV[\x97\x96PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_``\x83\x01a_._\x84\x01\x84aX\x94V[a_:_\x86\x01\x82aM9V[Pa_H` \x84\x01\x84aX\x94V[a_U` \x86\x01\x82aM9V[Pa_c`@\x84\x01\x84a\\'V[\x85\x83\x03`@\x87\x01Ra_v\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_a_\x8F\x83\x83a_\x1DV[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12a_\xB2Wa_\xB1aX\xB2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a_\xD5\x83\x85a_\x04V[\x93P\x83` \x84\x02\x85\x01a_\xE7\x84a_\x14V[\x80_[\x87\x81\x10\x15a`*W\x84\x84\x03\x89Ra`\x01\x82\x84a_\x97V[a`\x0B\x85\x82a_\x84V[\x94Pa`\x16\x83a_\xBEV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa_\xEAV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra`U\x81\x85\x87a_\xCAV[\x90Pa`d` \x83\x01\x84aP\x1BV[\x94\x93PPPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a`\x94Wa`\x93a`lV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a`\xB6Wa`\xB5a`pV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15a`\xD2Wa`\xD1a`tV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aa@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aa\x05V[aaJ\x86\x83aa\x05V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aa\x85aa\x80aa{\x84aK\xD8V[aabV[aK\xD8V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aa\x9E\x83aakV[aa\xB2aa\xAA\x82aa\x8CV[\x84\x84Taa\x11V[\x82UPPPPV[_\x90V[aa\xC6aa\xBAV[aa\xD1\x81\x84\x84aa\x95V[PPPV[[\x81\x81\x10\x15aa\xF4Waa\xE9_\x82aa\xBEV[`\x01\x81\x01\x90Paa\xD7V[PPV[`\x1F\x82\x11\x15ab9Wab\n\x81a`\xE4V[ab\x13\x84a`\xF6V[\x81\x01` \x85\x10\x15ab\"W\x81\x90P[ab6ab.\x85a`\xF6V[\x83\x01\x82aa\xD6V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_abY_\x19\x84`\x08\x02ab>V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_abq\x83\x83abJV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[ab\x8B\x83\x83a`\xDAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab\xA4Wab\xA3aQHV[[ab\xAE\x82Ta[\xDEV[ab\xB9\x82\x82\x85aa\xF8V[_`\x1F\x83\x11`\x01\x81\x14ab\xE6W_\x84\x15ab\xD4W\x82\x87\x015\x90P[ab\xDE\x85\x82abfV[\x86UPacEV[`\x1F\x19\x84\x16ab\xF4\x86a`\xE4V[_[\x82\x81\x10\x15ac\x1BW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pab\xF6V[\x86\x83\x10\x15ac8W\x84\x89\x015ac4`\x1F\x89\x16\x82abJV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[acY\x83\x83\x83ab\x81V[PPPV[_\x81\x01_\x83\x01acn\x81\x85a`xV[acy\x81\x83\x86acNV[PPPP`\x01\x81\x01` \x83\x01ac\x8F\x81\x85a`xV[ac\x9A\x81\x83\x86acNV[PPPPPPV[ac\xAC\x82\x82ac^V[PPV[_`@\x83\x01ac\xC1_\x84\x01\x84a\\'V[\x85\x83\x03_\x87\x01Rac\xD3\x83\x82\x84a\\\x89V[\x92PPPac\xE4` \x84\x01\x84a\\'V[\x85\x83\x03` \x87\x01Rac\xF7\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_ad\x13` \x84\x01\x84aK\xF7V[\x90P\x92\x91PPV[`\xA0\x82\x01ad+_\x83\x01\x83ad\x05V[ad7_\x85\x01\x82aN\x98V[PadE` \x83\x01\x83ad\x05V[adR` \x85\x01\x82aN\x98V[Pad``@\x83\x01\x83ad\x05V[adm`@\x85\x01\x82aN\x98V[Pad{``\x83\x01\x83ad\x05V[ad\x88``\x85\x01\x82aN\x98V[Pad\x96`\x80\x83\x01\x83ad\x05V[ad\xA3`\x80\x85\x01\x82aN\x98V[PPPPV[_a\x01 \x82\x01\x90P\x81\x81\x03_\x83\x01Rad\xC2\x81\x8Bac\xB0V[\x90Pad\xD1` \x83\x01\x8Aad\x1BV[\x81\x81\x03`\xC0\x83\x01Rad\xE4\x81\x88\x8Aa]\x86V[\x90P\x81\x81\x03`\xE0\x83\x01Rad\xF9\x81\x86\x88a_\xCAV[\x90P\x81\x81\x03a\x01\0\x83\x01Rae\x0F\x81\x84\x86aY\xF1V[\x90P\x99\x98PPPPPPPPPV[_\x815ae*\x81aK\xE1V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaei\x84ae3V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[ae\x88\x82aakV[ae\x9Bae\x94\x82aa\x8CV[\x83Tae>V[\x82UPPPV[_\x815ae\xAE\x81aM\xE7V[\x80\x91PP\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFae\xD6\x84ae3V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_af\x06af\x01ae\xFC\x84aM\tV[aabV[aM\tV[\x90P\x91\x90PV[_af\x17\x82ae\xECV[\x90P\x91\x90PV[_af(\x82af\rV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[afA\x82af\x1EV[afTafM\x82af/V[\x83Tae\xB7V[\x82UPPPV[_\x81\x01_\x83\x01\x80afk\x81ae\x1EV[\x90Pafw\x81\x84ae\x7FV[PPP`\x01\x81\x01` \x83\x01\x80af\x8C\x81ae\xA2V[\x90Paf\x98\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01\x80af\xAD\x81ae\xA2V[\x90Paf\xB9\x81\x84af8V[PPP`\x03\x81\x01``\x83\x01af\xCE\x81\x85a`xV[af\xD9\x81\x83\x86acNV[PPPP`\x04\x81\x01`\x80\x83\x01af\xEF\x81\x85a`xV[af\xFA\x81\x83\x86acNV[PPPPPPV[ag\x0C\x82\x82af[V[PPV[_`\xA0\x83\x01ag!_\x84\x01\x84ad\x05V[ag-_\x86\x01\x82aN\x98V[Pag;` \x84\x01\x84aX\x94V[agH` \x86\x01\x82aM9V[PagV`@\x84\x01\x84aX\x94V[agc`@\x86\x01\x82aM9V[Pagq``\x84\x01\x84a\\'V[\x85\x83\x03``\x87\x01Rag\x84\x83\x82\x84a\\\x89V[\x92PPPag\x95`\x80\x84\x01\x84a\\'V[\x85\x83\x03`\x80\x87\x01Rag\xA8\x83\x82\x84a\\\x89V[\x92PPP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rag\xCE\x81\x84ag\x10V[\x90P\x92\x91PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12ag\xF1Wag\xF0a`lV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ah\x19Wah\x18a`lV[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ah;Wah:a`pV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15ahWWahVa`tV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ah\xBCWah\x8D\x81ahiV[ah\x96\x84a`\xF6V[\x81\x01` \x85\x10\x15ah\xA5W\x81\x90P[ah\xB9ah\xB1\x85a`\xF6V[\x83\x01\x82aa\xD6V[PP[PPPV[ah\xCB\x83\x83ah_V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah\xE4Wah\xE3aQHV[[ah\xEE\x82Ta[\xDEV[ah\xF9\x82\x82\x85ah{V[_`\x1F\x83\x11`\x01\x81\x14ai&W_\x84\x15ai\x14W\x82\x87\x015\x90P[ai\x1E\x85\x82abfV[\x86UPai\x85V[`\x1F\x19\x84\x16ai4\x86ahiV[_[\x82\x81\x10\x15ai[W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pai6V[\x86\x83\x10\x15aixW\x84\x89\x015ait`\x1F\x89\x16\x82abJV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[ai\x99\x83\x83\x83ah\xC1V[PPPV[_\x81\x01_\x83\x01\x80ai\xAE\x81ae\xA2V[\x90Pai\xBA\x81\x84af8V[PPP`\x01\x81\x01` \x83\x01\x80ai\xCF\x81ae\xA2V[\x90Pai\xDB\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01ai\xF0\x81\x85ag\xFDV[ai\xFB\x81\x83\x86ai\x8EV[PPPPPPV[aj\r\x82\x82ai\x9EV[PPV[_`@\x82\x01\x90Paj$_\x83\x01\x85aP\x1BV[aj1` \x83\x01\x84aP\x1BV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[ajn\x81aR\xC6V[\x81\x14ajxW_\x80\xFD[PV[_\x81Q\x90Paj\x89\x81ajeV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aj\xA4Waj\xA3aK$V[[_aj\xB1\x84\x82\x85\x01aj{V[\x91PP\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12aj\xD5Waj\xD4a`lV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x81\x01_\x83\x01\x80aj\xF1\x81ae\xA2V[\x90Paj\xFD\x81\x84af8V[PPP`\x01\x81\x01` \x83\x01\x80ak\x12\x81ae\xA2V[\x90Pak\x1E\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01ak3\x81\x85a`xV[ak>\x81\x83\x86acNV[PPPP`\x03\x81\x01``\x83\x01akT\x81\x85a`xV[ak_\x81\x83\x86acNV[PPPPPPV[akq\x82\x82aj\xE1V[PPV[_\x825`\x01``\x03\x836\x03\x03\x81\x12ak\x90Wak\x8Fa`lV[[\x80\x83\x01\x91PP\x92\x91PPV[_\x81\x01_\x83\x01\x80ak\xAC\x81ae\xA2V[\x90Pak\xB8\x81\x84af8V[PPP`\x01\x81\x01` \x83\x01\x80ak\xCD\x81ae\xA2V[\x90Pak\xD9\x81\x84af8V[PPP`\x02\x81\x01`@\x83\x01ak\xEE\x81\x85a`xV[ak\xF9\x81\x83\x86acNV[PPPPPPV[al\x0B\x82\x82ak\x9CV[PPV[_\x81\x90P\x92\x91PPV[_al#\x82aV\x01V[al-\x81\x85al\x0FV[\x93Pal=\x81\x85` \x86\x01aLPV[\x80\x84\x01\x91PP\x92\x91PPV[_alT\x82\x84al\x19V[\x91P\x81\x90P\x92\x91PPV",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct Coprocessor { address txSenderAddress; address signerAddress; string s3BucketUrl; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Coprocessor {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub s3BucketUrl: alloy::sol_types::private::String,
    }
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
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<Coprocessor> for UnderlyingRustTuple<'_> {
            fn from(value: Coprocessor) -> Self {
                (value.txSenderAddress, value.signerAddress, value.s3BucketUrl)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Coprocessor {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txSenderAddress: tuple.0,
                    signerAddress: tuple.1,
                    s3BucketUrl: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Coprocessor {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Coprocessor {
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
                        &self.s3BucketUrl,
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
        impl alloy_sol_types::SolType for Coprocessor {
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
        impl alloy_sol_types::SolStruct for Coprocessor {
            const NAME: &'static str = "Coprocessor";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Coprocessor(address txSenderAddress,address signerAddress,string s3BucketUrl)",
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
                            &self.s3BucketUrl,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Coprocessor {
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
                        &rust.s3BucketUrl,
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
                    &rust.s3BucketUrl,
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
struct Custodian { address txSenderAddress; address signerAddress; bytes encryptionKey; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Custodian {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub encryptionKey: alloy::sol_types::private::Bytes,
    }
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
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<Custodian> for UnderlyingRustTuple<'_> {
            fn from(value: Custodian) -> Self {
                (value.txSenderAddress, value.signerAddress, value.encryptionKey)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Custodian {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    txSenderAddress: tuple.0,
                    signerAddress: tuple.1,
                    encryptionKey: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Custodian {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Custodian {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.txSenderAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signerAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.encryptionKey,
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
        impl alloy_sol_types::SolType for Custodian {
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
        impl alloy_sol_types::SolStruct for Custodian {
            const NAME: &'static str = "Custodian";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Custodian(address txSenderAddress,address signerAddress,bytes encryptionKey)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.encryptionKey,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Custodian {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.txSenderAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signerAddress,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.encryptionKey,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.encryptionKey,
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
struct HostChain { uint256 chainId; address fhevmExecutorAddress; address aclAddress; string name; string website; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HostChain {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub fhevmExecutorAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub aclAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub website: alloy::sol_types::private::String,
    }
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<HostChain> for UnderlyingRustTuple<'_> {
            fn from(value: HostChain) -> Self {
                (
                    value.chainId,
                    value.fhevmExecutorAddress,
                    value.aclAddress,
                    value.name,
                    value.website,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HostChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    fhevmExecutorAddress: tuple.1,
                    aclAddress: tuple.2,
                    name: tuple.3,
                    website: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for HostChain {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for HostChain {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.fhevmExecutorAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.aclAddress,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.website,
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
        impl alloy_sol_types::SolType for HostChain {
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
        impl alloy_sol_types::SolStruct for HostChain {
            const NAME: &'static str = "HostChain";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "HostChain(uint256 chainId,address fhevmExecutorAddress,address aclAddress,string name,string website)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.chainId)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.fhevmExecutorAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aclAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.name,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.website,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for HostChain {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fhevmExecutorAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aclAddress,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.name,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.website,
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
                    &rust.chainId,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.fhevmExecutorAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.aclAddress,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.name,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.website,
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
struct ProtocolMetadata { string name; string website; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ProtocolMetadata {
        #[allow(missing_docs)]
        pub name: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub website: alloy::sol_types::private::String,
    }
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
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<ProtocolMetadata> for UnderlyingRustTuple<'_> {
            fn from(value: ProtocolMetadata) -> Self {
                (value.name, value.website)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ProtocolMetadata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    name: tuple.0,
                    website: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ProtocolMetadata {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ProtocolMetadata {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.name,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.website,
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
        impl alloy_sol_types::SolType for ProtocolMetadata {
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
        impl alloy_sol_types::SolStruct for ProtocolMetadata {
            const NAME: &'static str = "ProtocolMetadata";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ProtocolMetadata(string name,string website)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.name,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.website,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ProtocolMetadata {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.name,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.website,
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
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.name,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.website,
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
    /**Custom error with signature `ChainIdNotUint64(uint256)` and selector `0x4178de42`.
```solidity
error ChainIdNotUint64(uint256 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainIdNotUint64 {
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
        impl ::core::convert::From<ChainIdNotUint64> for UnderlyingRustTuple<'_> {
            fn from(value: ChainIdNotUint64) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChainIdNotUint64 {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChainIdNotUint64 {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChainIdNotUint64(uint256)";
            const SELECTOR: [u8; 4] = [65u8, 120u8, 222u8, 66u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `EmptyCoprocessors()` and selector `0x8af082ef`.
```solidity
error EmptyCoprocessors();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCoprocessors;
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
        impl ::core::convert::From<EmptyCoprocessors> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCoprocessors) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyCoprocessors {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCoprocessors {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCoprocessors()";
            const SELECTOR: [u8; 4] = [138u8, 240u8, 130u8, 239u8];
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
    /**Custom error with signature `EmptyCustodians()` and selector `0xcad1d534`.
```solidity
error EmptyCustodians();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCustodians;
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
        impl ::core::convert::From<EmptyCustodians> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCustodians) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyCustodians {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCustodians {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCustodians()";
            const SELECTOR: [u8; 4] = [202u8, 209u8, 213u8, 52u8];
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
    /**Custom error with signature `HostChainAlreadyRegistered(uint256)` and selector `0x96a56828`.
```solidity
error HostChainAlreadyRegistered(uint256 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HostChainAlreadyRegistered {
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
        impl ::core::convert::From<HostChainAlreadyRegistered>
        for UnderlyingRustTuple<'_> {
            fn from(value: HostChainAlreadyRegistered) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for HostChainAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for HostChainAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "HostChainAlreadyRegistered(uint256)";
            const SELECTOR: [u8; 4] = [150u8, 165u8, 104u8, 40u8];
            #[inline]
            fn new<'a>(
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
    /**Custom error with signature `InvalidHighCoprocessorThreshold(uint256,uint256)` and selector `0x97beabad`.
```solidity
error InvalidHighCoprocessorThreshold(uint256 coprocessorThreshold, uint256 nCoprocessors);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHighCoprocessorThreshold {
        #[allow(missing_docs)]
        pub coprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nCoprocessors: alloy::sol_types::private::primitives::aliases::U256,
    }
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
        impl ::core::convert::From<InvalidHighCoprocessorThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHighCoprocessorThreshold) -> Self {
                (value.coprocessorThreshold, value.nCoprocessors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidHighCoprocessorThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    coprocessorThreshold: tuple.0,
                    nCoprocessors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHighCoprocessorThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHighCoprocessorThreshold(uint256,uint256)";
            const SELECTOR: [u8; 4] = [151u8, 190u8, 171u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessorThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nCoprocessors),
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
    /**Custom error with signature `InvalidHighKmsGenThreshold(uint256,uint256)` and selector `0x0f69cbfc`.
```solidity
error InvalidHighKmsGenThreshold(uint256 kmsGenThreshold, uint256 nKmsNodes);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHighKmsGenThreshold {
        #[allow(missing_docs)]
        pub kmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nKmsNodes: alloy::sol_types::private::primitives::aliases::U256,
    }
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
        impl ::core::convert::From<InvalidHighKmsGenThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHighKmsGenThreshold) -> Self {
                (value.kmsGenThreshold, value.nKmsNodes)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidHighKmsGenThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    kmsGenThreshold: tuple.0,
                    nKmsNodes: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHighKmsGenThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHighKmsGenThreshold(uint256,uint256)";
            const SELECTOR: [u8; 4] = [15u8, 105u8, 203u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsGenThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nKmsNodes),
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
    /**Custom error with signature `InvalidHighMpcThreshold(uint256,uint256)` and selector `0x907e6681`.
```solidity
error InvalidHighMpcThreshold(uint256 mpcThreshold, uint256 nKmsNodes);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHighMpcThreshold {
        #[allow(missing_docs)]
        pub mpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nKmsNodes: alloy::sol_types::private::primitives::aliases::U256,
    }
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
        impl ::core::convert::From<InvalidHighMpcThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHighMpcThreshold) -> Self {
                (value.mpcThreshold, value.nKmsNodes)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHighMpcThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    mpcThreshold: tuple.0,
                    nKmsNodes: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHighMpcThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHighMpcThreshold(uint256,uint256)";
            const SELECTOR: [u8; 4] = [144u8, 126u8, 102u8, 129u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.mpcThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nKmsNodes),
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
    /**Custom error with signature `InvalidHighPublicDecryptionThreshold(uint256,uint256)` and selector `0x84208f23`.
```solidity
error InvalidHighPublicDecryptionThreshold(uint256 publicDecryptionThreshold, uint256 nKmsNodes);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHighPublicDecryptionThreshold {
        #[allow(missing_docs)]
        pub publicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nKmsNodes: alloy::sol_types::private::primitives::aliases::U256,
    }
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
        impl ::core::convert::From<InvalidHighPublicDecryptionThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHighPublicDecryptionThreshold) -> Self {
                (value.publicDecryptionThreshold, value.nKmsNodes)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidHighPublicDecryptionThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    publicDecryptionThreshold: tuple.0,
                    nKmsNodes: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHighPublicDecryptionThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHighPublicDecryptionThreshold(uint256,uint256)";
            const SELECTOR: [u8; 4] = [132u8, 32u8, 143u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.publicDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nKmsNodes),
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
    /**Custom error with signature `InvalidHighUserDecryptionThreshold(uint256,uint256)` and selector `0xd2535e11`.
```solidity
error InvalidHighUserDecryptionThreshold(uint256 userDecryptionThreshold, uint256 nKmsNodes);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHighUserDecryptionThreshold {
        #[allow(missing_docs)]
        pub userDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub nKmsNodes: alloy::sol_types::private::primitives::aliases::U256,
    }
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
        impl ::core::convert::From<InvalidHighUserDecryptionThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHighUserDecryptionThreshold) -> Self {
                (value.userDecryptionThreshold, value.nKmsNodes)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidHighUserDecryptionThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    userDecryptionThreshold: tuple.0,
                    nKmsNodes: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHighUserDecryptionThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHighUserDecryptionThreshold(uint256,uint256)";
            const SELECTOR: [u8; 4] = [210u8, 83u8, 94u8, 17u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.userDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nKmsNodes),
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
    /**Custom error with signature `InvalidNullChainId()` and selector `0x22f73fea`.
```solidity
error InvalidNullChainId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullChainId;
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
        impl ::core::convert::From<InvalidNullChainId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullChainId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNullChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullChainId()";
            const SELECTOR: [u8; 4] = [34u8, 247u8, 63u8, 234u8];
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
    /**Custom error with signature `InvalidNullCoprocessorThreshold()` and selector `0xb60d2441`.
```solidity
error InvalidNullCoprocessorThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullCoprocessorThreshold;
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
        impl ::core::convert::From<InvalidNullCoprocessorThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullCoprocessorThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidNullCoprocessorThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullCoprocessorThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullCoprocessorThreshold()";
            const SELECTOR: [u8; 4] = [182u8, 13u8, 36u8, 65u8];
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
    /**Custom error with signature `InvalidNullKmsGenThreshold()` and selector `0x3ee50774`.
```solidity
error InvalidNullKmsGenThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullKmsGenThreshold;
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
        impl ::core::convert::From<InvalidNullKmsGenThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullKmsGenThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidNullKmsGenThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullKmsGenThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullKmsGenThreshold()";
            const SELECTOR: [u8; 4] = [62u8, 229u8, 7u8, 116u8];
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
    /**Custom error with signature `InvalidNullPublicDecryptionThreshold()` and selector `0xb1ae92ea`.
```solidity
error InvalidNullPublicDecryptionThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullPublicDecryptionThreshold;
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
        impl ::core::convert::From<InvalidNullPublicDecryptionThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullPublicDecryptionThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidNullPublicDecryptionThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullPublicDecryptionThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullPublicDecryptionThreshold()";
            const SELECTOR: [u8; 4] = [177u8, 174u8, 146u8, 234u8];
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
    /**Custom error with signature `InvalidNullUserDecryptionThreshold()` and selector `0xe60a7271`.
```solidity
error InvalidNullUserDecryptionThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullUserDecryptionThreshold;
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
        impl ::core::convert::From<InvalidNullUserDecryptionThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullUserDecryptionThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidNullUserDecryptionThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullUserDecryptionThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullUserDecryptionThreshold()";
            const SELECTOR: [u8; 4] = [230u8, 10u8, 114u8, 113u8];
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
    /**Custom error with signature `NotPauser(address)` and selector `0x206a346e`.
```solidity
error NotPauser(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotPauser {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
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
        impl ::core::convert::From<NotPauser> for UnderlyingRustTuple<'_> {
            fn from(value: NotPauser) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotPauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotPauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotPauser(address)";
            const SELECTOR: [u8; 4] = [32u8, 106u8, 52u8, 110u8];
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
                        &self.account,
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
    /**Custom error with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`.
```solidity
error OwnableInvalidOwner(address owner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableInvalidOwner {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
    }
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
        impl ::core::convert::From<OwnableInvalidOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OwnableInvalidOwner) -> Self {
                (value.owner,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OwnableInvalidOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { owner: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableInvalidOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableInvalidOwner(address)";
            const SELECTOR: [u8; 4] = [30u8, 79u8, 189u8, 247u8];
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
                        &self.owner,
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
    /**Custom error with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`.
```solidity
error OwnableUnauthorizedAccount(address account);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OwnableUnauthorizedAccount {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
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
        impl ::core::convert::From<OwnableUnauthorizedAccount>
        for UnderlyingRustTuple<'_> {
            fn from(value: OwnableUnauthorizedAccount) -> Self {
                (value.account,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OwnableUnauthorizedAccount {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { account: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OwnableUnauthorizedAccount {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OwnableUnauthorizedAccount(address)";
            const SELECTOR: [u8; 4] = [17u8, 140u8, 218u8, 167u8];
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
                        &self.account,
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
    /**Event with signature `AddHostChain((uint256,address,address,string,string))` and selector `0x66769341effd268fc4e9a9c8f27bfc968507b519b0ddb9b4ad3ded5f03016837`.
```solidity
event AddHostChain(HostChain hostChain);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AddHostChain {
        #[allow(missing_docs)]
        pub hostChain: <HostChain as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for AddHostChain {
            type DataTuple<'a> = (HostChain,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AddHostChain((uint256,address,address,string,string))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                102u8, 118u8, 147u8, 65u8, 239u8, 253u8, 38u8, 143u8, 196u8, 233u8,
                169u8, 200u8, 242u8, 123u8, 252u8, 150u8, 133u8, 7u8, 181u8, 25u8, 176u8,
                221u8, 185u8, 180u8, 173u8, 61u8, 237u8, 95u8, 3u8, 1u8, 104u8, 55u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { hostChain: data.0 }
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
                (<HostChain as alloy_sol_types::SolType>::tokenize(&self.hostChain),)
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
        impl alloy_sol_types::private::IntoLogData for AddHostChain {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AddHostChain> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AddHostChain) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `InitializeGatewayConfig((string,string),(uint256,uint256,uint256,uint256,uint256),(address,address,string,string)[],(address,address,string)[],(address,address,bytes)[])` and selector `0xb2cbe65ea308bfe4b9431819a3168d544f46ba344b1e79f92f973fcff43aae3b`.
```solidity
event InitializeGatewayConfig(ProtocolMetadata metadata, IGatewayConfig.Thresholds thresholds, KmsNode[] kmsNodes, Coprocessor[] coprocessors, Custodian[] custodians);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct InitializeGatewayConfig {
        #[allow(missing_docs)]
        pub metadata: <ProtocolMetadata as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub thresholds: <IGatewayConfig::Thresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub kmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub coprocessors: alloy::sol_types::private::Vec<
            <Coprocessor as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub custodians: alloy::sol_types::private::Vec<
            <Custodian as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for InitializeGatewayConfig {
            type DataTuple<'a> = (
                ProtocolMetadata,
                IGatewayConfig::Thresholds,
                alloy::sol_types::sol_data::Array<KmsNode>,
                alloy::sol_types::sol_data::Array<Coprocessor>,
                alloy::sol_types::sol_data::Array<Custodian>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "InitializeGatewayConfig((string,string),(uint256,uint256,uint256,uint256,uint256),(address,address,string,string)[],(address,address,string)[],(address,address,bytes)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8, 203u8, 230u8, 94u8, 163u8, 8u8, 191u8, 228u8, 185u8, 67u8, 24u8,
                25u8, 163u8, 22u8, 141u8, 84u8, 79u8, 70u8, 186u8, 52u8, 75u8, 30u8,
                121u8, 249u8, 47u8, 151u8, 63u8, 207u8, 244u8, 58u8, 174u8, 59u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    metadata: data.0,
                    thresholds: data.1,
                    kmsNodes: data.2,
                    coprocessors: data.3,
                    custodians: data.4,
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
                    <ProtocolMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.metadata,
                    ),
                    <IGatewayConfig::Thresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodes),
                    <alloy::sol_types::sol_data::Array<
                        Coprocessor,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessors),
                    <alloy::sol_types::sol_data::Array<
                        Custodian,
                    > as alloy_sol_types::SolType>::tokenize(&self.custodians),
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
        impl alloy_sol_types::private::IntoLogData for InitializeGatewayConfig {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&InitializeGatewayConfig> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &InitializeGatewayConfig,
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
    /**Event with signature `OwnershipTransferStarted(address,address)` and selector `0x38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e22700`.
```solidity
event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferStarted {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferStarted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferStarted(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                56u8, 209u8, 107u8, 140u8, 172u8, 34u8, 217u8, 159u8, 199u8, 193u8, 36u8,
                185u8, 205u8, 13u8, 226u8, 211u8, 250u8, 31u8, 174u8, 244u8, 32u8, 191u8,
                231u8, 145u8, 216u8, 195u8, 98u8, 215u8, 101u8, 226u8, 39u8, 0u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferStarted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferStarted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OwnershipTransferStarted,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PauseAllGatewayContracts()` and selector `0x13dbe8823219e226dd0525aeb071e1d2679f89382ba799f7f644867e65b6f3a6`.
```solidity
event PauseAllGatewayContracts();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PauseAllGatewayContracts;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for PauseAllGatewayContracts {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PauseAllGatewayContracts()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                19u8, 219u8, 232u8, 130u8, 50u8, 25u8, 226u8, 38u8, 221u8, 5u8, 37u8,
                174u8, 176u8, 113u8, 225u8, 210u8, 103u8, 159u8, 137u8, 56u8, 43u8,
                167u8, 153u8, 247u8, 246u8, 68u8, 134u8, 126u8, 101u8, 182u8, 243u8,
                166u8,
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
        impl alloy_sol_types::private::IntoLogData for PauseAllGatewayContracts {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PauseAllGatewayContracts> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PauseAllGatewayContracts,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ReinitializeGatewayConfigV3((address,address,string,string)[])` and selector `0x430929cffdf43b3681bb03eae5802745e335ec0159968d301e2a8e3d209eefb8`.
```solidity
event ReinitializeGatewayConfigV3(KmsNode[] newKmsNodes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ReinitializeGatewayConfigV3 {
        #[allow(missing_docs)]
        pub newKmsNodes: alloy::sol_types::private::Vec<
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ReinitializeGatewayConfigV3 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Array<KmsNode>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ReinitializeGatewayConfigV3((address,address,string,string)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                67u8, 9u8, 41u8, 207u8, 253u8, 244u8, 59u8, 54u8, 129u8, 187u8, 3u8,
                234u8, 229u8, 128u8, 39u8, 69u8, 227u8, 53u8, 236u8, 1u8, 89u8, 150u8,
                141u8, 48u8, 30u8, 42u8, 142u8, 61u8, 32u8, 158u8, 239u8, 184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newKmsNodes: data.0 }
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
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsNodes),
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
        impl alloy_sol_types::private::IntoLogData for ReinitializeGatewayConfigV3 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ReinitializeGatewayConfigV3> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ReinitializeGatewayConfigV3,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UnpauseAllGatewayContracts()` and selector `0xbe4f655daae0dbaef63a6b525cab2fa6ace4aa5b94b8834b241137cdfe73a5b0`.
```solidity
event UnpauseAllGatewayContracts();
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UnpauseAllGatewayContracts;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UnpauseAllGatewayContracts {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UnpauseAllGatewayContracts()";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                190u8, 79u8, 101u8, 93u8, 170u8, 224u8, 219u8, 174u8, 246u8, 58u8, 107u8,
                82u8, 92u8, 171u8, 47u8, 166u8, 172u8, 228u8, 170u8, 91u8, 148u8, 184u8,
                131u8, 75u8, 36u8, 17u8, 55u8, 205u8, 254u8, 115u8, 165u8, 176u8,
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
        impl alloy_sol_types::private::IntoLogData for UnpauseAllGatewayContracts {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UnpauseAllGatewayContracts> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UnpauseAllGatewayContracts,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdateCoprocessorThreshold(uint256)` and selector `0x7a2ef7dc89400a8ad92bb4ccf44d482624b40fe76b66977e85ed6a618e2e2fc7`.
```solidity
event UpdateCoprocessorThreshold(uint256 newCoprocessorThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateCoprocessorThreshold {
        #[allow(missing_docs)]
        pub newCoprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateCoprocessorThreshold {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateCoprocessorThreshold(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8, 46u8, 247u8, 220u8, 137u8, 64u8, 10u8, 138u8, 217u8, 43u8, 180u8,
                204u8, 244u8, 77u8, 72u8, 38u8, 36u8, 180u8, 15u8, 231u8, 107u8, 102u8,
                151u8, 126u8, 133u8, 237u8, 106u8, 97u8, 142u8, 46u8, 47u8, 199u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newCoprocessorThreshold: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newCoprocessorThreshold,
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
        impl alloy_sol_types::private::IntoLogData for UpdateCoprocessorThreshold {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateCoprocessorThreshold> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UpdateCoprocessorThreshold,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdateCoprocessors((address,address,string)[],uint256)` and selector `0xffe20bdb855e514e94147702922690cf1da10bdd18bf1f6215027c93ac05d455`.
```solidity
event UpdateCoprocessors(Coprocessor[] newCoprocessors, uint256 newCoprocessorThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateCoprocessors {
        #[allow(missing_docs)]
        pub newCoprocessors: alloy::sol_types::private::Vec<
            <Coprocessor as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub newCoprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateCoprocessors {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<Coprocessor>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateCoprocessors((address,address,string)[],uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                255u8, 226u8, 11u8, 219u8, 133u8, 94u8, 81u8, 78u8, 148u8, 20u8, 119u8,
                2u8, 146u8, 38u8, 144u8, 207u8, 29u8, 161u8, 11u8, 221u8, 24u8, 191u8,
                31u8, 98u8, 21u8, 2u8, 124u8, 147u8, 172u8, 5u8, 212u8, 85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newCoprocessors: data.0,
                    newCoprocessorThreshold: data.1,
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
                        Coprocessor,
                    > as alloy_sol_types::SolType>::tokenize(&self.newCoprocessors),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newCoprocessorThreshold,
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
        impl alloy_sol_types::private::IntoLogData for UpdateCoprocessors {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateCoprocessors> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateCoprocessors) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdateCustodians((address,address,bytes)[])` and selector `0x6cdc1aa76e1ebacd67c81be0dcf9603b5dfbeb4dd801ab214114acb536f11068`.
```solidity
event UpdateCustodians(Custodian[] newCustodians);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateCustodians {
        #[allow(missing_docs)]
        pub newCustodians: alloy::sol_types::private::Vec<
            <Custodian as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for UpdateCustodians {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Array<Custodian>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateCustodians((address,address,bytes)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                108u8, 220u8, 26u8, 167u8, 110u8, 30u8, 186u8, 205u8, 103u8, 200u8, 27u8,
                224u8, 220u8, 249u8, 96u8, 59u8, 93u8, 251u8, 235u8, 77u8, 216u8, 1u8,
                171u8, 33u8, 65u8, 20u8, 172u8, 181u8, 54u8, 241u8, 16u8, 104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newCustodians: data.0 }
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
                        Custodian,
                    > as alloy_sol_types::SolType>::tokenize(&self.newCustodians),
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
        impl alloy_sol_types::private::IntoLogData for UpdateCustodians {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateCustodians> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateCustodians) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdateKmsGenThreshold(uint256)` and selector `0x30c9b1d004f57eae3c6cc3a3752bcb4c8ea2e57c8241a782aa9b65fbc604ec5b`.
```solidity
event UpdateKmsGenThreshold(uint256 newKmsGenThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateKmsGenThreshold {
        #[allow(missing_docs)]
        pub newKmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateKmsGenThreshold {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateKmsGenThreshold(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                48u8, 201u8, 177u8, 208u8, 4u8, 245u8, 126u8, 174u8, 60u8, 108u8, 195u8,
                163u8, 117u8, 43u8, 203u8, 76u8, 142u8, 162u8, 229u8, 124u8, 130u8, 65u8,
                167u8, 130u8, 170u8, 155u8, 101u8, 251u8, 198u8, 4u8, 236u8, 91u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newKmsGenThreshold: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsGenThreshold),
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
        impl alloy_sol_types::private::IntoLogData for UpdateKmsGenThreshold {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateKmsGenThreshold> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateKmsGenThreshold) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdateKmsNodes((address,address,string,string)[],uint256,uint256,uint256,uint256)` and selector `0x25d1ea647128b56d47e64534cd0f5a86d3207f67b04895495b66dc0db87a0ca7`.
```solidity
event UpdateKmsNodes(KmsNode[] newKmsNodes, uint256 newMpcThreshold, uint256 newPublicDecryptionThreshold, uint256 newUserDecryptionThreshold, uint256 newKmsGenThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateKmsNodes {
        #[allow(missing_docs)]
        pub newKmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub newMpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newPublicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newUserDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newKmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateKmsNodes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<KmsNode>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateKmsNodes((address,address,string,string)[],uint256,uint256,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                37u8, 209u8, 234u8, 100u8, 113u8, 40u8, 181u8, 109u8, 71u8, 230u8, 69u8,
                52u8, 205u8, 15u8, 90u8, 134u8, 211u8, 32u8, 127u8, 103u8, 176u8, 72u8,
                149u8, 73u8, 91u8, 102u8, 220u8, 13u8, 184u8, 122u8, 12u8, 167u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newKmsNodes: data.0,
                    newMpcThreshold: data.1,
                    newPublicDecryptionThreshold: data.2,
                    newUserDecryptionThreshold: data.3,
                    newKmsGenThreshold: data.4,
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
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsNodes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMpcThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newPublicDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newUserDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsGenThreshold),
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
        impl alloy_sol_types::private::IntoLogData for UpdateKmsNodes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateKmsNodes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateKmsNodes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdateMpcThreshold(uint256)` and selector `0x3571172a49e72d7724be384cdd59f4f21a216c70352ea59cb02543fc76308437`.
```solidity
event UpdateMpcThreshold(uint256 newMpcThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateMpcThreshold {
        #[allow(missing_docs)]
        pub newMpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateMpcThreshold {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMpcThreshold(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                53u8, 113u8, 23u8, 42u8, 73u8, 231u8, 45u8, 119u8, 36u8, 190u8, 56u8,
                76u8, 221u8, 89u8, 244u8, 242u8, 26u8, 33u8, 108u8, 112u8, 53u8, 46u8,
                165u8, 156u8, 176u8, 37u8, 67u8, 252u8, 118u8, 48u8, 132u8, 55u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newMpcThreshold: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newMpcThreshold),
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
        impl alloy_sol_types::private::IntoLogData for UpdateMpcThreshold {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateMpcThreshold> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateMpcThreshold) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdatePublicDecryptionThreshold(uint256)` and selector `0xe41802af725729adcb8c151e2937380a25c69155757e3af5d3979adab5035800`.
```solidity
event UpdatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdatePublicDecryptionThreshold {
        #[allow(missing_docs)]
        pub newPublicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdatePublicDecryptionThreshold {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdatePublicDecryptionThreshold(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                228u8, 24u8, 2u8, 175u8, 114u8, 87u8, 41u8, 173u8, 203u8, 140u8, 21u8,
                30u8, 41u8, 55u8, 56u8, 10u8, 37u8, 198u8, 145u8, 85u8, 117u8, 126u8,
                58u8, 245u8, 211u8, 151u8, 154u8, 218u8, 181u8, 3u8, 88u8, 0u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newPublicDecryptionThreshold: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newPublicDecryptionThreshold,
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
        impl alloy_sol_types::private::IntoLogData for UpdatePublicDecryptionThreshold {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdatePublicDecryptionThreshold>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UpdatePublicDecryptionThreshold,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UpdateUserDecryptionThreshold(uint256)` and selector `0x837e0a6528dadfa2dc792692c5182e52a9f5bbdeed7b2372927a26c695839613`.
```solidity
event UpdateUserDecryptionThreshold(uint256 newUserDecryptionThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateUserDecryptionThreshold {
        #[allow(missing_docs)]
        pub newUserDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateUserDecryptionThreshold {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateUserDecryptionThreshold(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                131u8, 126u8, 10u8, 101u8, 40u8, 218u8, 223u8, 162u8, 220u8, 121u8, 38u8,
                146u8, 197u8, 24u8, 46u8, 82u8, 169u8, 245u8, 187u8, 222u8, 237u8, 123u8,
                35u8, 114u8, 146u8, 122u8, 38u8, 198u8, 149u8, 131u8, 150u8, 19u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newUserDecryptionThreshold: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newUserDecryptionThreshold,
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
        impl alloy_sol_types::private::IntoLogData for UpdateUserDecryptionThreshold {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateUserDecryptionThreshold> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UpdateUserDecryptionThreshold,
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
    /**Function with signature `acceptOwnership()` and selector `0x79ba5097`.
```solidity
function acceptOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipCall;
    ///Container type for the return parameters of the [`acceptOwnership()`](acceptOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipReturn {}
    #[allow(
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
            impl ::core::convert::From<acceptOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for acceptOwnershipCall {
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
            impl ::core::convert::From<acceptOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: acceptOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for acceptOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl acceptOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <acceptOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for acceptOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = acceptOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "acceptOwnership()";
            const SELECTOR: [u8; 4] = [121u8, 186u8, 80u8, 151u8];
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
                acceptOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `addHostChain((uint256,address,address,string,string))` and selector `0xc80b33ca`.
```solidity
function addHostChain(HostChain memory hostChain) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addHostChainCall {
        #[allow(missing_docs)]
        pub hostChain: <HostChain as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`addHostChain((uint256,address,address,string,string))`](addHostChainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addHostChainReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (HostChain,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HostChain as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addHostChainCall> for UnderlyingRustTuple<'_> {
                fn from(value: addHostChainCall) -> Self {
                    (value.hostChain,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addHostChainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { hostChain: tuple.0 }
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
            impl ::core::convert::From<addHostChainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addHostChainReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addHostChainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addHostChainReturn {
            fn _tokenize(
                &self,
            ) -> <addHostChainCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addHostChainCall {
            type Parameters<'a> = (HostChain,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addHostChainReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addHostChain((uint256,address,address,string,string))";
            const SELECTOR: [u8; 4] = [200u8, 11u8, 51u8, 202u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<HostChain as alloy_sol_types::SolType>::tokenize(&self.hostChain),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addHostChainReturn::_tokenize(ret)
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
    /**Function with signature `getCoprocessor(address)` and selector `0xef6997f9`.
```solidity
function getCoprocessor(address coprocessorTxSenderAddress) external view returns (Coprocessor memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorCall {
        #[allow(missing_docs)]
        pub coprocessorTxSenderAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCoprocessor(address)`](getCoprocessorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorReturn {
        #[allow(missing_docs)]
        pub _0: <Coprocessor as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCoprocessorCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorCall) -> Self {
                    (value.coprocessorTxSenderAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCoprocessorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        coprocessorTxSenderAddress: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Coprocessor,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Coprocessor as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCoprocessorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCoprocessorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Coprocessor as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Coprocessor,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCoprocessor(address)";
            const SELECTOR: [u8; 4] = [239u8, 105u8, 151u8, 249u8];
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
                        &self.coprocessorTxSenderAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Coprocessor as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCoprocessorReturn = r.into();
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
                        let r: getCoprocessorReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCoprocessorMajorityThreshold()` and selector `0x6799ef52`.
```solidity
function getCoprocessorMajorityThreshold() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorMajorityThresholdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCoprocessorMajorityThreshold()`](getCoprocessorMajorityThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorMajorityThresholdReturn {
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
            impl ::core::convert::From<getCoprocessorMajorityThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorMajorityThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorMajorityThresholdCall {
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
            impl ::core::convert::From<getCoprocessorMajorityThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorMajorityThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorMajorityThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCoprocessorMajorityThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCoprocessorMajorityThreshold()";
            const SELECTOR: [u8; 4] = [103u8, 153u8, 239u8, 82u8];
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
                        let r: getCoprocessorMajorityThresholdReturn = r.into();
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
                        let r: getCoprocessorMajorityThresholdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCoprocessorSigners()` and selector `0x9164d0ae`.
```solidity
function getCoprocessorSigners() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorSignersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCoprocessorSigners()`](getCoprocessorSignersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorSignersReturn {
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
            impl ::core::convert::From<getCoprocessorSignersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorSignersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorSignersCall {
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
            impl ::core::convert::From<getCoprocessorSignersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorSignersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorSignersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCoprocessorSignersCall {
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
            const SIGNATURE: &'static str = "getCoprocessorSigners()";
            const SELECTOR: [u8; 4] = [145u8, 100u8, 208u8, 174u8];
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
                        let r: getCoprocessorSignersReturn = r.into();
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
                        let r: getCoprocessorSignersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCoprocessorTxSenders()` and selector `0x1ea5bd42`.
```solidity
function getCoprocessorTxSenders() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorTxSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCoprocessorTxSenders()`](getCoprocessorTxSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorTxSendersReturn {
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
            impl ::core::convert::From<getCoprocessorTxSendersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorTxSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorTxSendersCall {
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
            impl ::core::convert::From<getCoprocessorTxSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorTxSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorTxSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCoprocessorTxSendersCall {
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
            const SIGNATURE: &'static str = "getCoprocessorTxSenders()";
            const SELECTOR: [u8; 4] = [30u8, 165u8, 189u8, 66u8];
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
                        let r: getCoprocessorTxSendersReturn = r.into();
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
                        let r: getCoprocessorTxSendersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCustodian(address)` and selector `0xcb5aa7e9`.
```solidity
function getCustodian(address custodianTxSenderAddress) external view returns (Custodian memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCustodianCall {
        #[allow(missing_docs)]
        pub custodianTxSenderAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCustodian(address)`](getCustodianCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCustodianReturn {
        #[allow(missing_docs)]
        pub _0: <Custodian as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCustodianCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCustodianCall) -> Self {
                    (value.custodianTxSenderAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCustodianCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        custodianTxSenderAddress: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Custodian,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Custodian as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCustodianReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCustodianReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCustodianReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCustodianCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <Custodian as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (Custodian,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCustodian(address)";
            const SELECTOR: [u8; 4] = [203u8, 90u8, 167u8, 233u8];
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
                        &self.custodianTxSenderAddress,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<Custodian as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCustodianReturn = r.into();
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
                        let r: getCustodianReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCustodianSigners()` and selector `0xba1f31d2`.
```solidity
function getCustodianSigners() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCustodianSignersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCustodianSigners()`](getCustodianSignersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCustodianSignersReturn {
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
            impl ::core::convert::From<getCustodianSignersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCustodianSignersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCustodianSignersCall {
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
            impl ::core::convert::From<getCustodianSignersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCustodianSignersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCustodianSignersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCustodianSignersCall {
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
            const SIGNATURE: &'static str = "getCustodianSigners()";
            const SELECTOR: [u8; 4] = [186u8, 31u8, 49u8, 210u8];
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
                        let r: getCustodianSignersReturn = r.into();
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
                        let r: getCustodianSignersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCustodianTxSenders()` and selector `0x2a8b9de9`.
```solidity
function getCustodianTxSenders() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCustodianTxSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCustodianTxSenders()`](getCustodianTxSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCustodianTxSendersReturn {
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
            impl ::core::convert::From<getCustodianTxSendersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCustodianTxSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCustodianTxSendersCall {
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
            impl ::core::convert::From<getCustodianTxSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCustodianTxSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCustodianTxSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCustodianTxSendersCall {
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
            const SIGNATURE: &'static str = "getCustodianTxSenders()";
            const SELECTOR: [u8; 4] = [42u8, 139u8, 157u8, 233u8];
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
                        let r: getCustodianTxSendersReturn = r.into();
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
                        let r: getCustodianTxSendersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getHostChain(uint256)` and selector `0xd10f7ff9`.
```solidity
function getHostChain(uint256 index) external view returns (HostChain memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHostChainCall {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getHostChain(uint256)`](getHostChainCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHostChainReturn {
        #[allow(missing_docs)]
        pub _0: <HostChain as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getHostChainCall> for UnderlyingRustTuple<'_> {
                fn from(value: getHostChainCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHostChainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (HostChain,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <HostChain as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHostChainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getHostChainReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHostChainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHostChainCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <HostChain as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (HostChain,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHostChain(uint256)";
            const SELECTOR: [u8; 4] = [209u8, 15u8, 127u8, 249u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<HostChain as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getHostChainReturn = r.into();
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
                        let r: getHostChainReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getHostChains()` and selector `0x2585bb65`.
```solidity
function getHostChains() external view returns (HostChain[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHostChainsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getHostChains()`](getHostChainsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHostChainsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <HostChain as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getHostChainsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getHostChainsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHostChainsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<HostChain>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <HostChain as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getHostChainsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getHostChainsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getHostChainsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHostChainsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <HostChain as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<HostChain>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHostChains()";
            const SELECTOR: [u8; 4] = [37u8, 133u8, 187u8, 101u8];
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
                        HostChain,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getHostChainsReturn = r.into();
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
                        let r: getHostChainsReturn = r.into();
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
    /**Function with signature `getKmsNode(address)` and selector `0xe3b2a874`.
```solidity
function getKmsNode(address kmsTxSenderAddress) external view returns (KmsNode memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsNodeCall {
        #[allow(missing_docs)]
        pub kmsTxSenderAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsNode(address)`](getKmsNodeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsNodeReturn {
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
            impl ::core::convert::From<getKmsNodeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKmsNodeCall) -> Self {
                    (value.kmsTxSenderAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKmsNodeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsTxSenderAddress: tuple.0,
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
            impl ::core::convert::From<getKmsNodeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getKmsNodeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKmsNodeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsNodeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <KmsNode as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (KmsNode,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKmsNode(address)";
            const SELECTOR: [u8; 4] = [227u8, 178u8, 168u8, 116u8];
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
                        &self.kmsTxSenderAddress,
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
                        let r: getKmsNodeReturn = r.into();
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
                        let r: getKmsNodeReturn = r.into();
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
    /**Function with signature `getKmsTxSenders()` and selector `0x7420f3d4`.
```solidity
function getKmsTxSenders() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsTxSendersCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKmsTxSenders()`](getKmsTxSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKmsTxSendersReturn {
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
            impl ::core::convert::From<getKmsTxSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKmsTxSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKmsTxSendersCall {
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
            impl ::core::convert::From<getKmsTxSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKmsTxSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKmsTxSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKmsTxSendersCall {
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
            const SIGNATURE: &'static str = "getKmsTxSenders()";
            const SELECTOR: [u8; 4] = [116u8, 32u8, 243u8, 212u8];
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
                        let r: getKmsTxSendersReturn = r.into();
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
                        let r: getKmsTxSendersReturn = r.into();
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
    /**Function with signature `getProtocolMetadata()` and selector `0x48144c61`.
```solidity
function getProtocolMetadata() external view returns (ProtocolMetadata memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getProtocolMetadataCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getProtocolMetadata()`](getProtocolMetadataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getProtocolMetadataReturn {
        #[allow(missing_docs)]
        pub _0: <ProtocolMetadata as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getProtocolMetadataCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getProtocolMetadataCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getProtocolMetadataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ProtocolMetadata,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ProtocolMetadata as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getProtocolMetadataReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getProtocolMetadataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getProtocolMetadataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getProtocolMetadataCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ProtocolMetadata as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ProtocolMetadata,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getProtocolMetadata()";
            const SELECTOR: [u8; 4] = [72u8, 20u8, 76u8, 97u8];
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
                (<ProtocolMetadata as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getProtocolMetadataReturn = r.into();
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
                        let r: getProtocolMetadataReturn = r.into();
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
    /**Function with signature `initializeFromEmptyProxy((string,string),(uint256,uint256,uint256,uint256,uint256),(address,address,string,string)[],(address,address,string)[],(address,address,bytes)[])` and selector `0xbb59e362`.
```solidity
function initializeFromEmptyProxy(ProtocolMetadata memory initialMetadata, IGatewayConfig.Thresholds memory initialThresholds, KmsNode[] memory initialKmsNodes, Coprocessor[] memory initialCoprocessors, Custodian[] memory initialCustodians) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyCall {
        #[allow(missing_docs)]
        pub initialMetadata: <ProtocolMetadata as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub initialThresholds: <IGatewayConfig::Thresholds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub initialKmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub initialCoprocessors: alloy::sol_types::private::Vec<
            <Coprocessor as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub initialCustodians: alloy::sol_types::private::Vec<
            <Custodian as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeFromEmptyProxy((string,string),(uint256,uint256,uint256,uint256,uint256),(address,address,string,string)[],(address,address,string)[],(address,address,bytes)[])`](initializeFromEmptyProxyCall) function.
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
                ProtocolMetadata,
                IGatewayConfig::Thresholds,
                alloy::sol_types::sol_data::Array<KmsNode>,
                alloy::sol_types::sol_data::Array<Coprocessor>,
                alloy::sol_types::sol_data::Array<Custodian>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ProtocolMetadata as alloy::sol_types::SolType>::RustType,
                <IGatewayConfig::Thresholds as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <KmsNode as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <Coprocessor as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <Custodian as alloy::sol_types::SolType>::RustType,
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
                        value.initialMetadata,
                        value.initialThresholds,
                        value.initialKmsNodes,
                        value.initialCoprocessors,
                        value.initialCustodians,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromEmptyProxyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialMetadata: tuple.0,
                        initialThresholds: tuple.1,
                        initialKmsNodes: tuple.2,
                        initialCoprocessors: tuple.3,
                        initialCustodians: tuple.4,
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
                ProtocolMetadata,
                IGatewayConfig::Thresholds,
                alloy::sol_types::sol_data::Array<KmsNode>,
                alloy::sol_types::sol_data::Array<Coprocessor>,
                alloy::sol_types::sol_data::Array<Custodian>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromEmptyProxyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromEmptyProxy((string,string),(uint256,uint256,uint256,uint256,uint256),(address,address,string,string)[],(address,address,string)[],(address,address,bytes)[])";
            const SELECTOR: [u8; 4] = [187u8, 89u8, 227u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ProtocolMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.initialMetadata,
                    ),
                    <IGatewayConfig::Thresholds as alloy_sol_types::SolType>::tokenize(
                        &self.initialThresholds,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialKmsNodes),
                    <alloy::sol_types::sol_data::Array<
                        Coprocessor,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialCoprocessors),
                    <alloy::sol_types::sol_data::Array<
                        Custodian,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialCustodians),
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
    /**Function with signature `isCoprocessorSigner(address)` and selector `0x2b101c03`.
```solidity
function isCoprocessorSigner(address signerAddress) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCoprocessorSignerCall {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCoprocessorSigner(address)`](isCoprocessorSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCoprocessorSignerReturn {
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
            impl ::core::convert::From<isCoprocessorSignerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCoprocessorSignerCall) -> Self {
                    (value.signerAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCoprocessorSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signerAddress: tuple.0 }
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
            impl ::core::convert::From<isCoprocessorSignerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCoprocessorSignerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCoprocessorSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCoprocessorSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCoprocessorSigner(address)";
            const SELECTOR: [u8; 4] = [43u8, 16u8, 28u8, 3u8];
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
                        let r: isCoprocessorSignerReturn = r.into();
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
                        let r: isCoprocessorSignerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isCoprocessorTxSender(address)` and selector `0x2dd3edfe`.
```solidity
function isCoprocessorTxSender(address txSenderAddress) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCoprocessorTxSenderCall {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCoprocessorTxSender(address)`](isCoprocessorTxSenderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCoprocessorTxSenderReturn {
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
            impl ::core::convert::From<isCoprocessorTxSenderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCoprocessorTxSenderCall) -> Self {
                    (value.txSenderAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCoprocessorTxSenderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { txSenderAddress: tuple.0 }
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
            impl ::core::convert::From<isCoprocessorTxSenderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCoprocessorTxSenderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCoprocessorTxSenderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCoprocessorTxSenderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCoprocessorTxSender(address)";
            const SELECTOR: [u8; 4] = [45u8, 211u8, 237u8, 254u8];
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
                        let r: isCoprocessorTxSenderReturn = r.into();
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
                        let r: isCoprocessorTxSenderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isCustodianSigner(address)` and selector `0x882d7dd3`.
```solidity
function isCustodianSigner(address signerAddress) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCustodianSignerCall {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCustodianSigner(address)`](isCustodianSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCustodianSignerReturn {
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
            impl ::core::convert::From<isCustodianSignerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCustodianSignerCall) -> Self {
                    (value.signerAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCustodianSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signerAddress: tuple.0 }
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
            impl ::core::convert::From<isCustodianSignerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCustodianSignerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCustodianSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCustodianSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCustodianSigner(address)";
            const SELECTOR: [u8; 4] = [136u8, 45u8, 125u8, 211u8];
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
                        let r: isCustodianSignerReturn = r.into();
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
                        let r: isCustodianSignerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isCustodianTxSender(address)` and selector `0x5bace7ff`.
```solidity
function isCustodianTxSender(address txSenderAddress) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCustodianTxSenderCall {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isCustodianTxSender(address)`](isCustodianTxSenderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isCustodianTxSenderReturn {
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
            impl ::core::convert::From<isCustodianTxSenderCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCustodianTxSenderCall) -> Self {
                    (value.txSenderAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCustodianTxSenderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { txSenderAddress: tuple.0 }
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
            impl ::core::convert::From<isCustodianTxSenderReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isCustodianTxSenderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isCustodianTxSenderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isCustodianTxSenderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isCustodianTxSender(address)";
            const SELECTOR: [u8; 4] = [91u8, 172u8, 231u8, 255u8];
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
                        let r: isCustodianTxSenderReturn = r.into();
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
                        let r: isCustodianTxSenderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isHostChainRegistered(uint256)` and selector `0xbff3aaba`.
```solidity
function isHostChainRegistered(uint256 chainId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isHostChainRegisteredCall {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isHostChainRegistered(uint256)`](isHostChainRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isHostChainRegisteredReturn {
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
            impl ::core::convert::From<isHostChainRegisteredCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isHostChainRegisteredCall) -> Self {
                    (value.chainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isHostChainRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainId: tuple.0 }
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
            impl ::core::convert::From<isHostChainRegisteredReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isHostChainRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isHostChainRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isHostChainRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isHostChainRegistered(uint256)";
            const SELECTOR: [u8; 4] = [191u8, 243u8, 170u8, 186u8];
            #[inline]
            fn new<'a>(
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
                        let r: isHostChainRegisteredReturn = r.into();
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
                        let r: isHostChainRegisteredReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isKmsSigner(address)` and selector `0x203d0114`.
```solidity
function isKmsSigner(address signerAddress) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsSignerCall {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
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
                    (value.signerAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isKmsSignerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signerAddress: tuple.0 }
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
                        &self.signerAddress,
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
    /**Function with signature `isKmsTxSender(address)` and selector `0xe5275eaf`.
```solidity
function isKmsTxSender(address txSenderAddress) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsTxSenderCall {
        #[allow(missing_docs)]
        pub txSenderAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isKmsTxSender(address)`](isKmsTxSenderCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKmsTxSenderReturn {
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
            impl ::core::convert::From<isKmsTxSenderCall> for UnderlyingRustTuple<'_> {
                fn from(value: isKmsTxSenderCall) -> Self {
                    (value.txSenderAddress,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isKmsTxSenderCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { txSenderAddress: tuple.0 }
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
            impl ::core::convert::From<isKmsTxSenderReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isKmsTxSenderReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isKmsTxSenderReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isKmsTxSenderCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isKmsTxSender(address)";
            const SELECTOR: [u8; 4] = [229u8, 39u8, 94u8, 175u8];
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
                        let r: isKmsTxSenderReturn = r.into();
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
                        let r: isKmsTxSenderReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isPauser(address)` and selector `0x46fbf68e`.
```solidity
function isPauser(address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPauserCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isPauser(address)`](isPauserCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPauserReturn {
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
            impl ::core::convert::From<isPauserCall> for UnderlyingRustTuple<'_> {
                fn from(value: isPauserCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPauserCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { account: tuple.0 }
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
            impl ::core::convert::From<isPauserReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isPauserReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isPauserReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isPauserCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isPauser(address)";
            const SELECTOR: [u8; 4] = [70u8, 251u8, 246u8, 142u8];
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
                        &self.account,
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
                        let r: isPauserReturn = r.into();
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
                        let r: isPauserReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: ownerReturn = r.into();
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
                        let r: ownerReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pauseAllGatewayContracts()` and selector `0x9a5a3bc4`.
```solidity
function pauseAllGatewayContracts() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllGatewayContractsCall;
    ///Container type for the return parameters of the [`pauseAllGatewayContracts()`](pauseAllGatewayContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllGatewayContractsReturn {}
    #[allow(
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
            impl ::core::convert::From<pauseAllGatewayContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllGatewayContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pauseAllGatewayContractsCall {
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
            impl ::core::convert::From<pauseAllGatewayContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllGatewayContractsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for pauseAllGatewayContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseAllGatewayContractsReturn {
            fn _tokenize(
                &self,
            ) -> <pauseAllGatewayContractsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllGatewayContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllGatewayContractsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAllGatewayContracts()";
            const SELECTOR: [u8; 4] = [154u8, 90u8, 59u8, 196u8];
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
                pauseAllGatewayContractsReturn::_tokenize(ret)
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
    /**Function with signature `pendingOwner()` and selector `0xe30c3978`.
```solidity
function pendingOwner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingOwnerCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`pendingOwner()`](pendingOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingOwnerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
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
            impl ::core::convert::From<pendingOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: pendingOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<pendingOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pendingOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pendingOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pendingOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pendingOwner()";
            const SELECTOR: [u8; 4] = [227u8, 12u8, 57u8, 120u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
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
                        let r: pendingOwnerReturn = r.into();
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
                        let r: pendingOwnerReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `reinitializeV3((address,address,string,string)[])` and selector `0x27612cf6`.
```solidity
function reinitializeV3(KmsNode[] memory newKmsNodes) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV3Call {
        #[allow(missing_docs)]
        pub newKmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`reinitializeV3((address,address,string,string)[])`](reinitializeV3Call) function.
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
            impl ::core::convert::From<reinitializeV3Call> for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV3Call) -> Self {
                    (value.newKmsNodes,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV3Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newKmsNodes: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Array<KmsNode>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV3Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV3((address,address,string,string)[])";
            const SELECTOR: [u8; 4] = [39u8, 97u8, 44u8, 246u8];
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
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsNodes),
                )
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
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
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
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
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
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
                renounceOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `unpauseAllGatewayContracts()` and selector `0x798b58a6`.
```solidity
function unpauseAllGatewayContracts() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseAllGatewayContractsCall;
    ///Container type for the return parameters of the [`unpauseAllGatewayContracts()`](unpauseAllGatewayContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseAllGatewayContractsReturn {}
    #[allow(
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
            impl ::core::convert::From<unpauseAllGatewayContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: unpauseAllGatewayContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unpauseAllGatewayContractsCall {
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
            impl ::core::convert::From<unpauseAllGatewayContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unpauseAllGatewayContractsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unpauseAllGatewayContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl unpauseAllGatewayContractsReturn {
            fn _tokenize(
                &self,
            ) -> <unpauseAllGatewayContractsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseAllGatewayContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseAllGatewayContractsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpauseAllGatewayContracts()";
            const SELECTOR: [u8; 4] = [121u8, 139u8, 88u8, 166u8];
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
                unpauseAllGatewayContractsReturn::_tokenize(ret)
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
    /**Function with signature `updateCoprocessorThreshold(uint256)` and selector `0xd5e16b7d`.
```solidity
function updateCoprocessorThreshold(uint256 newCoprocessorThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateCoprocessorThresholdCall {
        #[allow(missing_docs)]
        pub newCoprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateCoprocessorThreshold(uint256)`](updateCoprocessorThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateCoprocessorThresholdReturn {}
    #[allow(
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
            impl ::core::convert::From<updateCoprocessorThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateCoprocessorThresholdCall) -> Self {
                    (value.newCoprocessorThreshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateCoprocessorThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newCoprocessorThreshold: tuple.0,
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
            impl ::core::convert::From<updateCoprocessorThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateCoprocessorThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateCoprocessorThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateCoprocessorThresholdReturn {
            fn _tokenize(
                &self,
            ) -> <updateCoprocessorThresholdCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateCoprocessorThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateCoprocessorThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateCoprocessorThreshold(uint256)";
            const SELECTOR: [u8; 4] = [213u8, 225u8, 107u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newCoprocessorThreshold,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateCoprocessorThresholdReturn::_tokenize(ret)
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
    /**Function with signature `updateCoprocessors((address,address,string)[],uint256)` and selector `0x83bb2e57`.
```solidity
function updateCoprocessors(Coprocessor[] memory newCoprocessors, uint256 newCoprocessorThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateCoprocessorsCall {
        #[allow(missing_docs)]
        pub newCoprocessors: alloy::sol_types::private::Vec<
            <Coprocessor as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub newCoprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateCoprocessors((address,address,string)[],uint256)`](updateCoprocessorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateCoprocessorsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<Coprocessor>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <Coprocessor as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateCoprocessorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateCoprocessorsCall) -> Self {
                    (value.newCoprocessors, value.newCoprocessorThreshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateCoprocessorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newCoprocessors: tuple.0,
                        newCoprocessorThreshold: tuple.1,
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
            impl ::core::convert::From<updateCoprocessorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateCoprocessorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateCoprocessorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateCoprocessorsReturn {
            fn _tokenize(
                &self,
            ) -> <updateCoprocessorsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateCoprocessorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<Coprocessor>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateCoprocessorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateCoprocessors((address,address,string)[],uint256)";
            const SELECTOR: [u8; 4] = [131u8, 187u8, 46u8, 87u8];
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
                        Coprocessor,
                    > as alloy_sol_types::SolType>::tokenize(&self.newCoprocessors),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newCoprocessorThreshold,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateCoprocessorsReturn::_tokenize(ret)
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
    /**Function with signature `updateCustodians((address,address,bytes)[])` and selector `0x013dc21e`.
```solidity
function updateCustodians(Custodian[] memory newCustodians) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateCustodiansCall {
        #[allow(missing_docs)]
        pub newCustodians: alloy::sol_types::private::Vec<
            <Custodian as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`updateCustodians((address,address,bytes)[])`](updateCustodiansCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateCustodiansReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<Custodian>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <Custodian as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateCustodiansCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateCustodiansCall) -> Self {
                    (value.newCustodians,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateCustodiansCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newCustodians: tuple.0 }
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
            impl ::core::convert::From<updateCustodiansReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateCustodiansReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateCustodiansReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateCustodiansReturn {
            fn _tokenize(
                &self,
            ) -> <updateCustodiansCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateCustodiansCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Array<Custodian>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateCustodiansReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateCustodians((address,address,bytes)[])";
            const SELECTOR: [u8; 4] = [1u8, 61u8, 194u8, 30u8];
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
                        Custodian,
                    > as alloy_sol_types::SolType>::tokenize(&self.newCustodians),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateCustodiansReturn::_tokenize(ret)
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
    /**Function with signature `updateKmsGenThreshold(uint256)` and selector `0x0724dd23`.
```solidity
function updateKmsGenThreshold(uint256 newKmsGenThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKmsGenThresholdCall {
        #[allow(missing_docs)]
        pub newKmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateKmsGenThreshold(uint256)`](updateKmsGenThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKmsGenThresholdReturn {}
    #[allow(
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
            impl ::core::convert::From<updateKmsGenThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateKmsGenThresholdCall) -> Self {
                    (value.newKmsGenThreshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateKmsGenThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newKmsGenThreshold: tuple.0,
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
            impl ::core::convert::From<updateKmsGenThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateKmsGenThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateKmsGenThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateKmsGenThresholdReturn {
            fn _tokenize(
                &self,
            ) -> <updateKmsGenThresholdCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateKmsGenThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateKmsGenThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateKmsGenThreshold(uint256)";
            const SELECTOR: [u8; 4] = [7u8, 36u8, 221u8, 35u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsGenThreshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateKmsGenThresholdReturn::_tokenize(ret)
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
    /**Function with signature `updateKmsNodes((address,address,string,string)[],uint256,uint256,uint256,uint256)` and selector `0x53da9246`.
```solidity
function updateKmsNodes(KmsNode[] memory newKmsNodes, uint256 newMpcThreshold, uint256 newPublicDecryptionThreshold, uint256 newUserDecryptionThreshold, uint256 newKmsGenThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKmsNodesCall {
        #[allow(missing_docs)]
        pub newKmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub newMpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newPublicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newUserDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newKmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateKmsNodes((address,address,string,string)[],uint256,uint256,uint256,uint256)`](updateKmsNodesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateKmsNodesReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<KmsNode>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNode as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<updateKmsNodesCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateKmsNodesCall) -> Self {
                    (
                        value.newKmsNodes,
                        value.newMpcThreshold,
                        value.newPublicDecryptionThreshold,
                        value.newUserDecryptionThreshold,
                        value.newKmsGenThreshold,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateKmsNodesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newKmsNodes: tuple.0,
                        newMpcThreshold: tuple.1,
                        newPublicDecryptionThreshold: tuple.2,
                        newUserDecryptionThreshold: tuple.3,
                        newKmsGenThreshold: tuple.4,
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
            impl ::core::convert::From<updateKmsNodesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateKmsNodesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateKmsNodesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateKmsNodesReturn {
            fn _tokenize(
                &self,
            ) -> <updateKmsNodesCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateKmsNodesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<KmsNode>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateKmsNodesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateKmsNodes((address,address,string,string)[],uint256,uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [83u8, 218u8, 146u8, 70u8];
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
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsNodes),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMpcThreshold),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newPublicDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newUserDecryptionThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newKmsGenThreshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateKmsNodesReturn::_tokenize(ret)
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
    /**Function with signature `updateMpcThreshold(uint256)` and selector `0x772d2fe9`.
```solidity
function updateMpcThreshold(uint256 newMpcThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMpcThresholdCall {
        #[allow(missing_docs)]
        pub newMpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateMpcThreshold(uint256)`](updateMpcThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMpcThresholdReturn {}
    #[allow(
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
            impl ::core::convert::From<updateMpcThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMpcThresholdCall) -> Self {
                    (value.newMpcThreshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMpcThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newMpcThreshold: tuple.0 }
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
            impl ::core::convert::From<updateMpcThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMpcThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMpcThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateMpcThresholdReturn {
            fn _tokenize(
                &self,
            ) -> <updateMpcThresholdCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMpcThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMpcThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMpcThreshold(uint256)";
            const SELECTOR: [u8; 4] = [119u8, 45u8, 47u8, 233u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMpcThreshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateMpcThresholdReturn::_tokenize(ret)
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
    /**Function with signature `updatePublicDecryptionThreshold(uint256)` and selector `0x2e2d3a82`.
```solidity
function updatePublicDecryptionThreshold(uint256 newPublicDecryptionThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePublicDecryptionThresholdCall {
        #[allow(missing_docs)]
        pub newPublicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updatePublicDecryptionThreshold(uint256)`](updatePublicDecryptionThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updatePublicDecryptionThresholdReturn {}
    #[allow(
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
            impl ::core::convert::From<updatePublicDecryptionThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updatePublicDecryptionThresholdCall) -> Self {
                    (value.newPublicDecryptionThreshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updatePublicDecryptionThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPublicDecryptionThreshold: tuple.0,
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
            impl ::core::convert::From<updatePublicDecryptionThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updatePublicDecryptionThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updatePublicDecryptionThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updatePublicDecryptionThresholdReturn {
            fn _tokenize(
                &self,
            ) -> <updatePublicDecryptionThresholdCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updatePublicDecryptionThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updatePublicDecryptionThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updatePublicDecryptionThreshold(uint256)";
            const SELECTOR: [u8; 4] = [46u8, 45u8, 58u8, 130u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newPublicDecryptionThreshold,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updatePublicDecryptionThresholdReturn::_tokenize(ret)
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
    /**Function with signature `updateUserDecryptionThreshold(uint256)` and selector `0xeb843cf6`.
```solidity
function updateUserDecryptionThreshold(uint256 newUserDecryptionThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateUserDecryptionThresholdCall {
        #[allow(missing_docs)]
        pub newUserDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateUserDecryptionThreshold(uint256)`](updateUserDecryptionThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateUserDecryptionThresholdReturn {}
    #[allow(
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
            impl ::core::convert::From<updateUserDecryptionThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateUserDecryptionThresholdCall) -> Self {
                    (value.newUserDecryptionThreshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateUserDecryptionThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newUserDecryptionThreshold: tuple.0,
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
            impl ::core::convert::From<updateUserDecryptionThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateUserDecryptionThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateUserDecryptionThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateUserDecryptionThresholdReturn {
            fn _tokenize(
                &self,
            ) -> <updateUserDecryptionThresholdCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateUserDecryptionThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateUserDecryptionThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateUserDecryptionThreshold(uint256)";
            const SELECTOR: [u8; 4] = [235u8, 132u8, 60u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newUserDecryptionThreshold,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateUserDecryptionThresholdReturn::_tokenize(ret)
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
    ///Container for all the [`GatewayConfig`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum GatewayConfigCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        acceptOwnership(acceptOwnershipCall),
        #[allow(missing_docs)]
        addHostChain(addHostChainCall),
        #[allow(missing_docs)]
        getCoprocessor(getCoprocessorCall),
        #[allow(missing_docs)]
        getCoprocessorMajorityThreshold(getCoprocessorMajorityThresholdCall),
        #[allow(missing_docs)]
        getCoprocessorSigners(getCoprocessorSignersCall),
        #[allow(missing_docs)]
        getCoprocessorTxSenders(getCoprocessorTxSendersCall),
        #[allow(missing_docs)]
        getCustodian(getCustodianCall),
        #[allow(missing_docs)]
        getCustodianSigners(getCustodianSignersCall),
        #[allow(missing_docs)]
        getCustodianTxSenders(getCustodianTxSendersCall),
        #[allow(missing_docs)]
        getHostChain(getHostChainCall),
        #[allow(missing_docs)]
        getHostChains(getHostChainsCall),
        #[allow(missing_docs)]
        getKmsGenThreshold(getKmsGenThresholdCall),
        #[allow(missing_docs)]
        getKmsNode(getKmsNodeCall),
        #[allow(missing_docs)]
        getKmsSigners(getKmsSignersCall),
        #[allow(missing_docs)]
        getKmsTxSenders(getKmsTxSendersCall),
        #[allow(missing_docs)]
        getMpcThreshold(getMpcThresholdCall),
        #[allow(missing_docs)]
        getProtocolMetadata(getProtocolMetadataCall),
        #[allow(missing_docs)]
        getPublicDecryptionThreshold(getPublicDecryptionThresholdCall),
        #[allow(missing_docs)]
        getUserDecryptionThreshold(getUserDecryptionThresholdCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isCoprocessorSigner(isCoprocessorSignerCall),
        #[allow(missing_docs)]
        isCoprocessorTxSender(isCoprocessorTxSenderCall),
        #[allow(missing_docs)]
        isCustodianSigner(isCustodianSignerCall),
        #[allow(missing_docs)]
        isCustodianTxSender(isCustodianTxSenderCall),
        #[allow(missing_docs)]
        isHostChainRegistered(isHostChainRegisteredCall),
        #[allow(missing_docs)]
        isKmsSigner(isKmsSignerCall),
        #[allow(missing_docs)]
        isKmsTxSender(isKmsTxSenderCall),
        #[allow(missing_docs)]
        isPauser(isPauserCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pauseAllGatewayContracts(pauseAllGatewayContractsCall),
        #[allow(missing_docs)]
        pendingOwner(pendingOwnerCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV3(reinitializeV3Call),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpauseAllGatewayContracts(unpauseAllGatewayContractsCall),
        #[allow(missing_docs)]
        updateCoprocessorThreshold(updateCoprocessorThresholdCall),
        #[allow(missing_docs)]
        updateCoprocessors(updateCoprocessorsCall),
        #[allow(missing_docs)]
        updateCustodians(updateCustodiansCall),
        #[allow(missing_docs)]
        updateKmsGenThreshold(updateKmsGenThresholdCall),
        #[allow(missing_docs)]
        updateKmsNodes(updateKmsNodesCall),
        #[allow(missing_docs)]
        updateMpcThreshold(updateMpcThresholdCall),
        #[allow(missing_docs)]
        updatePublicDecryptionThreshold(updatePublicDecryptionThresholdCall),
        #[allow(missing_docs)]
        updateUserDecryptionThreshold(updateUserDecryptionThresholdCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    #[automatically_derived]
    impl GatewayConfigCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 61u8, 194u8, 30u8],
            [7u8, 36u8, 221u8, 35u8],
            [13u8, 142u8, 110u8, 44u8],
            [30u8, 165u8, 189u8, 66u8],
            [32u8, 61u8, 1u8, 20u8],
            [37u8, 133u8, 187u8, 101u8],
            [38u8, 207u8, 93u8, 239u8],
            [39u8, 97u8, 44u8, 246u8],
            [42u8, 56u8, 137u8, 152u8],
            [42u8, 139u8, 157u8, 233u8],
            [43u8, 16u8, 28u8, 3u8],
            [45u8, 211u8, 237u8, 254u8],
            [46u8, 45u8, 58u8, 130u8],
            [70u8, 251u8, 246u8, 142u8],
            [72u8, 20u8, 76u8, 97u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [83u8, 218u8, 146u8, 70u8],
            [91u8, 172u8, 231u8, 255u8],
            [103u8, 153u8, 239u8, 82u8],
            [113u8, 80u8, 24u8, 166u8],
            [116u8, 32u8, 243u8, 212u8],
            [119u8, 45u8, 47u8, 233u8],
            [121u8, 139u8, 88u8, 166u8],
            [121u8, 186u8, 80u8, 151u8],
            [126u8, 170u8, 200u8, 242u8],
            [131u8, 187u8, 46u8, 87u8],
            [136u8, 45u8, 125u8, 211u8],
            [141u8, 165u8, 203u8, 91u8],
            [145u8, 100u8, 208u8, 174u8],
            [154u8, 90u8, 59u8, 196u8],
            [173u8, 60u8, 177u8, 204u8],
            [180u8, 114u8, 43u8, 196u8],
            [186u8, 31u8, 49u8, 210u8],
            [187u8, 89u8, 227u8, 98u8],
            [191u8, 243u8, 170u8, 186u8],
            [194u8, 180u8, 41u8, 134u8],
            [200u8, 11u8, 51u8, 202u8],
            [203u8, 90u8, 167u8, 233u8],
            [209u8, 15u8, 127u8, 249u8],
            [213u8, 225u8, 107u8, 125u8],
            [227u8, 12u8, 57u8, 120u8],
            [227u8, 178u8, 168u8, 116u8],
            [229u8, 39u8, 94u8, 175u8],
            [235u8, 132u8, 60u8, 246u8],
            [239u8, 105u8, 151u8, 249u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GatewayConfigCalls {
        const NAME: &'static str = "GatewayConfigCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 47usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptOwnership(_) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addHostChain(_) => {
                    <addHostChainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCoprocessor(_) => {
                    <getCoprocessorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCoprocessorMajorityThreshold(_) => {
                    <getCoprocessorMajorityThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCoprocessorSigners(_) => {
                    <getCoprocessorSignersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCoprocessorTxSenders(_) => {
                    <getCoprocessorTxSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCustodian(_) => {
                    <getCustodianCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCustodianSigners(_) => {
                    <getCustodianSignersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCustodianTxSenders(_) => {
                    <getCustodianTxSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHostChain(_) => {
                    <getHostChainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHostChains(_) => {
                    <getHostChainsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsGenThreshold(_) => {
                    <getKmsGenThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsNode(_) => {
                    <getKmsNodeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsSigners(_) => {
                    <getKmsSignersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsTxSenders(_) => {
                    <getKmsTxSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMpcThreshold(_) => {
                    <getMpcThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getProtocolMetadata(_) => {
                    <getProtocolMetadataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPublicDecryptionThreshold(_) => {
                    <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserDecryptionThreshold(_) => {
                    <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCoprocessorSigner(_) => {
                    <isCoprocessorSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCoprocessorTxSender(_) => {
                    <isCoprocessorTxSenderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCustodianSigner(_) => {
                    <isCustodianSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isCustodianTxSender(_) => {
                    <isCustodianTxSenderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isHostChainRegistered(_) => {
                    <isHostChainRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isKmsSigner(_) => {
                    <isKmsSignerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isKmsTxSender(_) => {
                    <isKmsTxSenderCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isPauser(_) => <isPauserCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAllGatewayContracts(_) => {
                    <pauseAllGatewayContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pendingOwner(_) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV3(_) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpauseAllGatewayContracts(_) => {
                    <unpauseAllGatewayContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateCoprocessorThreshold(_) => {
                    <updateCoprocessorThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateCoprocessors(_) => {
                    <updateCoprocessorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateCustodians(_) => {
                    <updateCustodiansCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateKmsGenThreshold(_) => {
                    <updateKmsGenThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateKmsNodes(_) => {
                    <updateKmsNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateMpcThreshold(_) => {
                    <updateMpcThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updatePublicDecryptionThreshold(_) => {
                    <updatePublicDecryptionThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateUserDecryptionThreshold(_) => {
                    <updateUserDecryptionThresholdCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<GatewayConfigCalls>] = &[
                {
                    fn updateCustodians(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateCustodiansCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updateCustodians)
                    }
                    updateCustodians
                },
                {
                    fn updateKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updateKmsGenThreshold)
                    }
                    updateKmsGenThreshold
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn getCoprocessorTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessorTxSenders)
                    }
                    getCoprocessorTxSenders
                },
                {
                    fn isKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isKmsSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::isKmsSigner)
                    }
                    isKmsSigner
                },
                {
                    fn getHostChains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getHostChainsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getHostChains)
                    }
                    getHostChains
                },
                {
                    fn getMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getMpcThreshold)
                    }
                    getMpcThreshold
                },
                {
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::reinitializeV3)
                    }
                    reinitializeV3
                },
                {
                    fn getPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getPublicDecryptionThreshold)
                    }
                    getPublicDecryptionThreshold
                },
                {
                    fn getCustodianTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCustodianTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getCustodianTxSenders)
                    }
                    getCustodianTxSenders
                },
                {
                    fn isCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCoprocessorSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::isCoprocessorSigner)
                    }
                    isCoprocessorSigner
                },
                {
                    fn isCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCoprocessorTxSenderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::isCoprocessorTxSender)
                    }
                    isCoprocessorTxSender
                },
                {
                    fn updatePublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updatePublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updatePublicDecryptionThreshold)
                    }
                    updatePublicDecryptionThreshold
                },
                {
                    fn isPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isPauserCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(GatewayConfigCalls::isPauser)
                    }
                    isPauser
                },
                {
                    fn getProtocolMetadata(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getProtocolMetadataCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getProtocolMetadata)
                    }
                    getProtocolMetadata
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn updateKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateKmsNodesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updateKmsNodes)
                    }
                    updateKmsNodes
                },
                {
                    fn isCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCustodianTxSenderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::isCustodianTxSender)
                    }
                    isCustodianTxSender
                },
                {
                    fn getCoprocessorMajorityThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorMajorityThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessorMajorityThreshold)
                    }
                    getCoprocessorMajorityThreshold
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getKmsTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsTxSenders)
                    }
                    getKmsTxSenders
                },
                {
                    fn updateMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updateMpcThreshold)
                    }
                    updateMpcThreshold
                },
                {
                    fn unpauseAllGatewayContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <unpauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::unpauseAllGatewayContracts)
                    }
                    unpauseAllGatewayContracts
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn getKmsSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsSignersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsSigners)
                    }
                    getKmsSigners
                },
                {
                    fn updateCoprocessors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateCoprocessorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updateCoprocessors)
                    }
                    updateCoprocessors
                },
                {
                    fn isCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCustodianSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::isCustodianSigner)
                    }
                    isCustodianSigner
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(GatewayConfigCalls::owner)
                    }
                    owner
                },
                {
                    fn getCoprocessorSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessorSigners)
                    }
                    getCoprocessorSigners
                },
                {
                    fn pauseAllGatewayContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <pauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::pauseAllGatewayContracts)
                    }
                    pauseAllGatewayContracts
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsGenThreshold)
                    }
                    getKmsGenThreshold
                },
                {
                    fn getCustodianSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCustodianSignersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getCustodianSigners)
                    }
                    getCustodianSigners
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn isHostChainRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isHostChainRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::isHostChainRegistered)
                    }
                    isHostChainRegistered
                },
                {
                    fn getUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getUserDecryptionThreshold)
                    }
                    getUserDecryptionThreshold
                },
                {
                    fn addHostChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <addHostChainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::addHostChain)
                    }
                    addHostChain
                },
                {
                    fn getCustodian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCustodianCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getCustodian)
                    }
                    getCustodian
                },
                {
                    fn getHostChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getHostChainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getHostChain)
                    }
                    getHostChain
                },
                {
                    fn updateCoprocessorThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateCoprocessorThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updateCoprocessorThreshold)
                    }
                    updateCoprocessorThreshold
                },
                {
                    fn pendingOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <pendingOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::pendingOwner)
                    }
                    pendingOwner
                },
                {
                    fn getKmsNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsNodeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsNode)
                    }
                    getKmsNode
                },
                {
                    fn isKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isKmsTxSenderCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::isKmsTxSender)
                    }
                    isKmsTxSender
                },
                {
                    fn updateUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::updateUserDecryptionThreshold)
                    }
                    updateUserDecryptionThreshold
                },
                {
                    fn getCoprocessor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessor)
                    }
                    getCoprocessor
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigCalls::transferOwnership)
                    }
                    transferOwnership
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
            ) -> alloy_sol_types::Result<GatewayConfigCalls>] = &[
                {
                    fn updateCustodians(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateCustodiansCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updateCustodians)
                    }
                    updateCustodians
                },
                {
                    fn updateKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updateKmsGenThreshold)
                    }
                    updateKmsGenThreshold
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn getCoprocessorTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessorTxSenders)
                    }
                    getCoprocessorTxSenders
                },
                {
                    fn isKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isKmsSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isKmsSigner)
                    }
                    isKmsSigner
                },
                {
                    fn getHostChains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getHostChainsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getHostChains)
                    }
                    getHostChains
                },
                {
                    fn getMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getMpcThreshold)
                    }
                    getMpcThreshold
                },
                {
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::reinitializeV3)
                    }
                    reinitializeV3
                },
                {
                    fn getPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getPublicDecryptionThreshold)
                    }
                    getPublicDecryptionThreshold
                },
                {
                    fn getCustodianTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCustodianTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getCustodianTxSenders)
                    }
                    getCustodianTxSenders
                },
                {
                    fn isCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCoprocessorSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isCoprocessorSigner)
                    }
                    isCoprocessorSigner
                },
                {
                    fn isCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCoprocessorTxSenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isCoprocessorTxSender)
                    }
                    isCoprocessorTxSender
                },
                {
                    fn updatePublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updatePublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updatePublicDecryptionThreshold)
                    }
                    updatePublicDecryptionThreshold
                },
                {
                    fn isPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isPauserCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isPauser)
                    }
                    isPauser
                },
                {
                    fn getProtocolMetadata(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getProtocolMetadataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getProtocolMetadata)
                    }
                    getProtocolMetadata
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn updateKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateKmsNodesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updateKmsNodes)
                    }
                    updateKmsNodes
                },
                {
                    fn isCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCustodianTxSenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isCustodianTxSender)
                    }
                    isCustodianTxSender
                },
                {
                    fn getCoprocessorMajorityThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorMajorityThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessorMajorityThreshold)
                    }
                    getCoprocessorMajorityThreshold
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getKmsTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsTxSenders)
                    }
                    getKmsTxSenders
                },
                {
                    fn updateMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateMpcThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updateMpcThreshold)
                    }
                    updateMpcThreshold
                },
                {
                    fn unpauseAllGatewayContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <unpauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::unpauseAllGatewayContracts)
                    }
                    unpauseAllGatewayContracts
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn getKmsSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsSignersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsSigners)
                    }
                    getKmsSigners
                },
                {
                    fn updateCoprocessors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateCoprocessorsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updateCoprocessors)
                    }
                    updateCoprocessors
                },
                {
                    fn isCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isCustodianSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isCustodianSigner)
                    }
                    isCustodianSigner
                },
                {
                    fn owner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::owner)
                    }
                    owner
                },
                {
                    fn getCoprocessorSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessorSigners)
                    }
                    getCoprocessorSigners
                },
                {
                    fn pauseAllGatewayContracts(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <pauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::pauseAllGatewayContracts)
                    }
                    pauseAllGatewayContracts
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn getKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsGenThreshold)
                    }
                    getKmsGenThreshold
                },
                {
                    fn getCustodianSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCustodianSignersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getCustodianSigners)
                    }
                    getCustodianSigners
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn isHostChainRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isHostChainRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isHostChainRegistered)
                    }
                    isHostChainRegistered
                },
                {
                    fn getUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getUserDecryptionThreshold)
                    }
                    getUserDecryptionThreshold
                },
                {
                    fn addHostChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <addHostChainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::addHostChain)
                    }
                    addHostChain
                },
                {
                    fn getCustodian(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCustodianCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getCustodian)
                    }
                    getCustodian
                },
                {
                    fn getHostChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getHostChainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getHostChain)
                    }
                    getHostChain
                },
                {
                    fn updateCoprocessorThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateCoprocessorThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updateCoprocessorThreshold)
                    }
                    updateCoprocessorThreshold
                },
                {
                    fn pendingOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <pendingOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::pendingOwner)
                    }
                    pendingOwner
                },
                {
                    fn getKmsNode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getKmsNodeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getKmsNode)
                    }
                    getKmsNode
                },
                {
                    fn isKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <isKmsTxSenderCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::isKmsTxSender)
                    }
                    isKmsTxSender
                },
                {
                    fn updateUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <updateUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::updateUserDecryptionThreshold)
                    }
                    updateUserDecryptionThreshold
                },
                {
                    fn getCoprocessor(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <getCoprocessorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::getCoprocessor)
                    }
                    getCoprocessor
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigCalls::transferOwnership)
                    }
                    transferOwnership
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
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addHostChain(inner) => {
                    <addHostChainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCoprocessor(inner) => {
                    <getCoprocessorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCoprocessorMajorityThreshold(inner) => {
                    <getCoprocessorMajorityThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCoprocessorSigners(inner) => {
                    <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCoprocessorTxSenders(inner) => {
                    <getCoprocessorTxSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCustodian(inner) => {
                    <getCustodianCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCustodianSigners(inner) => {
                    <getCustodianSignersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCustodianTxSenders(inner) => {
                    <getCustodianTxSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHostChain(inner) => {
                    <getHostChainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHostChains(inner) => {
                    <getHostChainsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsGenThreshold(inner) => {
                    <getKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsNode(inner) => {
                    <getKmsNodeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getKmsSigners(inner) => {
                    <getKmsSignersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKmsTxSenders(inner) => {
                    <getKmsTxSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMpcThreshold(inner) => {
                    <getMpcThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getProtocolMetadata(inner) => {
                    <getProtocolMetadataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPublicDecryptionThreshold(inner) => {
                    <getPublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserDecryptionThreshold(inner) => {
                    <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isCoprocessorSigner(inner) => {
                    <isCoprocessorSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isCoprocessorTxSender(inner) => {
                    <isCoprocessorTxSenderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isCustodianSigner(inner) => {
                    <isCustodianSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isCustodianTxSender(inner) => {
                    <isCustodianTxSenderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isHostChainRegistered(inner) => {
                    <isHostChainRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isKmsSigner(inner) => {
                    <isKmsSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isKmsTxSender(inner) => {
                    <isKmsTxSenderCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isPauser(inner) => {
                    <isPauserCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAllGatewayContracts(inner) => {
                    <pauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pendingOwner(inner) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpauseAllGatewayContracts(inner) => {
                    <unpauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateCoprocessorThreshold(inner) => {
                    <updateCoprocessorThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateCoprocessors(inner) => {
                    <updateCoprocessorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateCustodians(inner) => {
                    <updateCustodiansCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateKmsGenThreshold(inner) => {
                    <updateKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateKmsNodes(inner) => {
                    <updateKmsNodesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateMpcThreshold(inner) => {
                    <updateMpcThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updatePublicDecryptionThreshold(inner) => {
                    <updatePublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateUserDecryptionThreshold(inner) => {
                    <updateUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::acceptOwnership(inner) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addHostChain(inner) => {
                    <addHostChainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCoprocessor(inner) => {
                    <getCoprocessorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCoprocessorMajorityThreshold(inner) => {
                    <getCoprocessorMajorityThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCoprocessorSigners(inner) => {
                    <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCoprocessorTxSenders(inner) => {
                    <getCoprocessorTxSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCustodian(inner) => {
                    <getCustodianCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCustodianSigners(inner) => {
                    <getCustodianSignersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCustodianTxSenders(inner) => {
                    <getCustodianTxSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHostChain(inner) => {
                    <getHostChainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHostChains(inner) => {
                    <getHostChainsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getKmsNode(inner) => {
                    <getKmsNodeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getKmsTxSenders(inner) => {
                    <getKmsTxSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getProtocolMetadata(inner) => {
                    <getProtocolMetadataCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getUserDecryptionThreshold(inner) => {
                    <getUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isCoprocessorSigner(inner) => {
                    <isCoprocessorSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isCoprocessorTxSender(inner) => {
                    <isCoprocessorTxSenderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isCustodianSigner(inner) => {
                    <isCustodianSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isCustodianTxSender(inner) => {
                    <isCustodianTxSenderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isHostChainRegistered(inner) => {
                    <isHostChainRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isKmsTxSender(inner) => {
                    <isKmsTxSenderCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isPauser(inner) => {
                    <isPauserCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAllGatewayContracts(inner) => {
                    <pauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pendingOwner(inner) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpauseAllGatewayContracts(inner) => {
                    <unpauseAllGatewayContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateCoprocessorThreshold(inner) => {
                    <updateCoprocessorThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateCoprocessors(inner) => {
                    <updateCoprocessorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateCustodians(inner) => {
                    <updateCustodiansCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateKmsGenThreshold(inner) => {
                    <updateKmsGenThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateKmsNodes(inner) => {
                    <updateKmsNodesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateMpcThreshold(inner) => {
                    <updateMpcThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updatePublicDecryptionThreshold(inner) => {
                    <updatePublicDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateUserDecryptionThreshold(inner) => {
                    <updateUserDecryptionThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`GatewayConfig`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum GatewayConfigErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        ChainIdNotUint64(ChainIdNotUint64),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        EmptyCoprocessors(EmptyCoprocessors),
        #[allow(missing_docs)]
        EmptyCustodians(EmptyCustodians),
        #[allow(missing_docs)]
        EmptyKmsNodes(EmptyKmsNodes),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        HostChainAlreadyRegistered(HostChainAlreadyRegistered),
        #[allow(missing_docs)]
        InvalidHighCoprocessorThreshold(InvalidHighCoprocessorThreshold),
        #[allow(missing_docs)]
        InvalidHighKmsGenThreshold(InvalidHighKmsGenThreshold),
        #[allow(missing_docs)]
        InvalidHighMpcThreshold(InvalidHighMpcThreshold),
        #[allow(missing_docs)]
        InvalidHighPublicDecryptionThreshold(InvalidHighPublicDecryptionThreshold),
        #[allow(missing_docs)]
        InvalidHighUserDecryptionThreshold(InvalidHighUserDecryptionThreshold),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidNullChainId(InvalidNullChainId),
        #[allow(missing_docs)]
        InvalidNullCoprocessorThreshold(InvalidNullCoprocessorThreshold),
        #[allow(missing_docs)]
        InvalidNullKmsGenThreshold(InvalidNullKmsGenThreshold),
        #[allow(missing_docs)]
        InvalidNullPublicDecryptionThreshold(InvalidNullPublicDecryptionThreshold),
        #[allow(missing_docs)]
        InvalidNullUserDecryptionThreshold(InvalidNullUserDecryptionThreshold),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        NotPauser(NotPauser),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    #[automatically_derived]
    impl GatewayConfigErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 140u8, 141u8, 64u8],
            [15u8, 105u8, 203u8, 252u8],
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [32u8, 106u8, 52u8, 110u8],
            [34u8, 247u8, 63u8, 234u8],
            [62u8, 229u8, 7u8, 116u8],
            [65u8, 120u8, 222u8, 66u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [132u8, 32u8, 143u8, 35u8],
            [138u8, 240u8, 130u8, 239u8],
            [144u8, 126u8, 102u8, 129u8],
            [150u8, 165u8, 104u8, 40u8],
            [151u8, 190u8, 171u8, 173u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [177u8, 174u8, 146u8, 234u8],
            [179u8, 152u8, 151u8, 159u8],
            [182u8, 13u8, 36u8, 65u8],
            [202u8, 209u8, 213u8, 52u8],
            [210u8, 83u8, 94u8, 17u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 10u8, 114u8, 113u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for GatewayConfigErrors {
        const NAME: &'static str = "GatewayConfigErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 27usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChainIdNotUint64(_) => {
                    <ChainIdNotUint64 as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCoprocessors(_) => {
                    <EmptyCoprocessors as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCustodians(_) => {
                    <EmptyCustodians as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyKmsNodes(_) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::HostChainAlreadyRegistered(_) => {
                    <HostChainAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHighCoprocessorThreshold(_) => {
                    <InvalidHighCoprocessorThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHighKmsGenThreshold(_) => {
                    <InvalidHighKmsGenThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHighMpcThreshold(_) => {
                    <InvalidHighMpcThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHighPublicDecryptionThreshold(_) => {
                    <InvalidHighPublicDecryptionThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHighUserDecryptionThreshold(_) => {
                    <InvalidHighUserDecryptionThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullChainId(_) => {
                    <InvalidNullChainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullCoprocessorThreshold(_) => {
                    <InvalidNullCoprocessorThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullKmsGenThreshold(_) => {
                    <InvalidNullKmsGenThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullPublicDecryptionThreshold(_) => {
                    <InvalidNullPublicDecryptionThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullUserDecryptionThreshold(_) => {
                    <InvalidNullUserDecryptionThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializingFromEmptyProxy(_) => {
                    <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotPauser(_) => <NotPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<GatewayConfigErrors>] = &[
                {
                    fn EmptyKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <EmptyKmsNodes as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::EmptyKmsNodes)
                    }
                    EmptyKmsNodes
                },
                {
                    fn InvalidHighKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighKmsGenThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighKmsGenThreshold)
                    }
                    InvalidHighKmsGenThreshold
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NotPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <NotPauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(GatewayConfigErrors::NotPauser)
                    }
                    NotPauser
                },
                {
                    fn InvalidNullChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullChainId)
                    }
                    InvalidNullChainId
                },
                {
                    fn InvalidNullKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullKmsGenThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullKmsGenThreshold)
                    }
                    InvalidNullKmsGenThreshold
                },
                {
                    fn ChainIdNotUint64(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <ChainIdNotUint64 as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::ChainIdNotUint64)
                    }
                    ChainIdNotUint64
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidHighPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                GatewayConfigErrors::InvalidHighPublicDecryptionThreshold,
                            )
                    }
                    InvalidHighPublicDecryptionThreshold
                },
                {
                    fn EmptyCoprocessors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <EmptyCoprocessors as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::EmptyCoprocessors)
                    }
                    EmptyCoprocessors
                },
                {
                    fn InvalidHighMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighMpcThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighMpcThreshold)
                    }
                    InvalidHighMpcThreshold
                },
                {
                    fn HostChainAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <HostChainAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::HostChainAlreadyRegistered)
                    }
                    HostChainAlreadyRegistered
                },
                {
                    fn InvalidHighCoprocessorThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighCoprocessorThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighCoprocessorThreshold)
                    }
                    InvalidHighCoprocessorThreshold
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn InvalidNullPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                GatewayConfigErrors::InvalidNullPublicDecryptionThreshold,
                            )
                    }
                    InvalidNullPublicDecryptionThreshold
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn InvalidNullCoprocessorThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullCoprocessorThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullCoprocessorThreshold)
                    }
                    InvalidNullCoprocessorThreshold
                },
                {
                    fn EmptyCustodians(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <EmptyCustodians as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::EmptyCustodians)
                    }
                    EmptyCustodians
                },
                {
                    fn InvalidHighUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighUserDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighUserDecryptionThreshold)
                    }
                    InvalidHighUserDecryptionThreshold
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(GatewayConfigErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn InvalidNullUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullUserDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullUserDecryptionThreshold)
                    }
                    InvalidNullUserDecryptionThreshold
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidInitialization)
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
            ) -> alloy_sol_types::Result<GatewayConfigErrors>] = &[
                {
                    fn EmptyKmsNodes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <EmptyKmsNodes as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::EmptyKmsNodes)
                    }
                    EmptyKmsNodes
                },
                {
                    fn InvalidHighKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighKmsGenThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighKmsGenThreshold)
                    }
                    InvalidHighKmsGenThreshold
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn NotPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <NotPauser as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::NotPauser)
                    }
                    NotPauser
                },
                {
                    fn InvalidNullChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullChainId)
                    }
                    InvalidNullChainId
                },
                {
                    fn InvalidNullKmsGenThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullKmsGenThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullKmsGenThreshold)
                    }
                    InvalidNullKmsGenThreshold
                },
                {
                    fn ChainIdNotUint64(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <ChainIdNotUint64 as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::ChainIdNotUint64)
                    }
                    ChainIdNotUint64
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidHighPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                GatewayConfigErrors::InvalidHighPublicDecryptionThreshold,
                            )
                    }
                    InvalidHighPublicDecryptionThreshold
                },
                {
                    fn EmptyCoprocessors(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <EmptyCoprocessors as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::EmptyCoprocessors)
                    }
                    EmptyCoprocessors
                },
                {
                    fn InvalidHighMpcThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighMpcThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighMpcThreshold)
                    }
                    InvalidHighMpcThreshold
                },
                {
                    fn HostChainAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <HostChainAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::HostChainAlreadyRegistered)
                    }
                    HostChainAlreadyRegistered
                },
                {
                    fn InvalidHighCoprocessorThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighCoprocessorThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighCoprocessorThreshold)
                    }
                    InvalidHighCoprocessorThreshold
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn InvalidNullPublicDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                GatewayConfigErrors::InvalidNullPublicDecryptionThreshold,
                            )
                    }
                    InvalidNullPublicDecryptionThreshold
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn InvalidNullCoprocessorThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullCoprocessorThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullCoprocessorThreshold)
                    }
                    InvalidNullCoprocessorThreshold
                },
                {
                    fn EmptyCustodians(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <EmptyCustodians as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::EmptyCustodians)
                    }
                    EmptyCustodians
                },
                {
                    fn InvalidHighUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidHighUserDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidHighUserDecryptionThreshold)
                    }
                    InvalidHighUserDecryptionThreshold
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn InvalidNullUserDecryptionThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidNullUserDecryptionThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidNullUserDecryptionThreshold)
                    }
                    InvalidNullUserDecryptionThreshold
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<GatewayConfigErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(GatewayConfigErrors::InvalidInitialization)
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
                Self::ChainIdNotUint64(inner) => {
                    <ChainIdNotUint64 as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyCoprocessors(inner) => {
                    <EmptyCoprocessors as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyCustodians(inner) => {
                    <EmptyCustodians as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyKmsNodes(inner) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::HostChainAlreadyRegistered(inner) => {
                    <HostChainAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHighCoprocessorThreshold(inner) => {
                    <InvalidHighCoprocessorThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHighKmsGenThreshold(inner) => {
                    <InvalidHighKmsGenThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHighMpcThreshold(inner) => {
                    <InvalidHighMpcThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHighPublicDecryptionThreshold(inner) => {
                    <InvalidHighPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHighUserDecryptionThreshold(inner) => {
                    <InvalidHighUserDecryptionThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullChainId(inner) => {
                    <InvalidNullChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullCoprocessorThreshold(inner) => {
                    <InvalidNullCoprocessorThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullKmsGenThreshold(inner) => {
                    <InvalidNullKmsGenThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullPublicDecryptionThreshold(inner) => {
                    <InvalidNullPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullUserDecryptionThreshold(inner) => {
                    <InvalidNullUserDecryptionThreshold as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotPauser(inner) => {
                    <NotPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ChainIdNotUint64(inner) => {
                    <ChainIdNotUint64 as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyCoprocessors(inner) => {
                    <EmptyCoprocessors as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyCustodians(inner) => {
                    <EmptyCustodians as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::HostChainAlreadyRegistered(inner) => {
                    <HostChainAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHighCoprocessorThreshold(inner) => {
                    <InvalidHighCoprocessorThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHighKmsGenThreshold(inner) => {
                    <InvalidHighKmsGenThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHighMpcThreshold(inner) => {
                    <InvalidHighMpcThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHighPublicDecryptionThreshold(inner) => {
                    <InvalidHighPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHighUserDecryptionThreshold(inner) => {
                    <InvalidHighUserDecryptionThreshold as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidNullChainId(inner) => {
                    <InvalidNullChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNullCoprocessorThreshold(inner) => {
                    <InvalidNullCoprocessorThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNullKmsGenThreshold(inner) => {
                    <InvalidNullKmsGenThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNullPublicDecryptionThreshold(inner) => {
                    <InvalidNullPublicDecryptionThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNullUserDecryptionThreshold(inner) => {
                    <InvalidNullUserDecryptionThreshold as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotPauser(inner) => {
                    <NotPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OwnableInvalidOwner(inner) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OwnableUnauthorizedAccount(inner) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`GatewayConfig`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum GatewayConfigEvents {
        #[allow(missing_docs)]
        AddHostChain(AddHostChain),
        #[allow(missing_docs)]
        InitializeGatewayConfig(InitializeGatewayConfig),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OwnershipTransferStarted(OwnershipTransferStarted),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        PauseAllGatewayContracts(PauseAllGatewayContracts),
        #[allow(missing_docs)]
        ReinitializeGatewayConfigV3(ReinitializeGatewayConfigV3),
        #[allow(missing_docs)]
        UnpauseAllGatewayContracts(UnpauseAllGatewayContracts),
        #[allow(missing_docs)]
        UpdateCoprocessorThreshold(UpdateCoprocessorThreshold),
        #[allow(missing_docs)]
        UpdateCoprocessors(UpdateCoprocessors),
        #[allow(missing_docs)]
        UpdateCustodians(UpdateCustodians),
        #[allow(missing_docs)]
        UpdateKmsGenThreshold(UpdateKmsGenThreshold),
        #[allow(missing_docs)]
        UpdateKmsNodes(UpdateKmsNodes),
        #[allow(missing_docs)]
        UpdateMpcThreshold(UpdateMpcThreshold),
        #[allow(missing_docs)]
        UpdatePublicDecryptionThreshold(UpdatePublicDecryptionThreshold),
        #[allow(missing_docs)]
        UpdateUserDecryptionThreshold(UpdateUserDecryptionThreshold),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl GatewayConfigEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                19u8, 219u8, 232u8, 130u8, 50u8, 25u8, 226u8, 38u8, 221u8, 5u8, 37u8,
                174u8, 176u8, 113u8, 225u8, 210u8, 103u8, 159u8, 137u8, 56u8, 43u8,
                167u8, 153u8, 247u8, 246u8, 68u8, 134u8, 126u8, 101u8, 182u8, 243u8,
                166u8,
            ],
            [
                37u8, 209u8, 234u8, 100u8, 113u8, 40u8, 181u8, 109u8, 71u8, 230u8, 69u8,
                52u8, 205u8, 15u8, 90u8, 134u8, 211u8, 32u8, 127u8, 103u8, 176u8, 72u8,
                149u8, 73u8, 91u8, 102u8, 220u8, 13u8, 184u8, 122u8, 12u8, 167u8,
            ],
            [
                48u8, 201u8, 177u8, 208u8, 4u8, 245u8, 126u8, 174u8, 60u8, 108u8, 195u8,
                163u8, 117u8, 43u8, 203u8, 76u8, 142u8, 162u8, 229u8, 124u8, 130u8, 65u8,
                167u8, 130u8, 170u8, 155u8, 101u8, 251u8, 198u8, 4u8, 236u8, 91u8,
            ],
            [
                53u8, 113u8, 23u8, 42u8, 73u8, 231u8, 45u8, 119u8, 36u8, 190u8, 56u8,
                76u8, 221u8, 89u8, 244u8, 242u8, 26u8, 33u8, 108u8, 112u8, 53u8, 46u8,
                165u8, 156u8, 176u8, 37u8, 67u8, 252u8, 118u8, 48u8, 132u8, 55u8,
            ],
            [
                56u8, 209u8, 107u8, 140u8, 172u8, 34u8, 217u8, 159u8, 199u8, 193u8, 36u8,
                185u8, 205u8, 13u8, 226u8, 211u8, 250u8, 31u8, 174u8, 244u8, 32u8, 191u8,
                231u8, 145u8, 216u8, 195u8, 98u8, 215u8, 101u8, 226u8, 39u8, 0u8,
            ],
            [
                67u8, 9u8, 41u8, 207u8, 253u8, 244u8, 59u8, 54u8, 129u8, 187u8, 3u8,
                234u8, 229u8, 128u8, 39u8, 69u8, 227u8, 53u8, 236u8, 1u8, 89u8, 150u8,
                141u8, 48u8, 30u8, 42u8, 142u8, 61u8, 32u8, 158u8, 239u8, 184u8,
            ],
            [
                102u8, 118u8, 147u8, 65u8, 239u8, 253u8, 38u8, 143u8, 196u8, 233u8,
                169u8, 200u8, 242u8, 123u8, 252u8, 150u8, 133u8, 7u8, 181u8, 25u8, 176u8,
                221u8, 185u8, 180u8, 173u8, 61u8, 237u8, 95u8, 3u8, 1u8, 104u8, 55u8,
            ],
            [
                108u8, 220u8, 26u8, 167u8, 110u8, 30u8, 186u8, 205u8, 103u8, 200u8, 27u8,
                224u8, 220u8, 249u8, 96u8, 59u8, 93u8, 251u8, 235u8, 77u8, 216u8, 1u8,
                171u8, 33u8, 65u8, 20u8, 172u8, 181u8, 54u8, 241u8, 16u8, 104u8,
            ],
            [
                122u8, 46u8, 247u8, 220u8, 137u8, 64u8, 10u8, 138u8, 217u8, 43u8, 180u8,
                204u8, 244u8, 77u8, 72u8, 38u8, 36u8, 180u8, 15u8, 231u8, 107u8, 102u8,
                151u8, 126u8, 133u8, 237u8, 106u8, 97u8, 142u8, 46u8, 47u8, 199u8,
            ],
            [
                131u8, 126u8, 10u8, 101u8, 40u8, 218u8, 223u8, 162u8, 220u8, 121u8, 38u8,
                146u8, 197u8, 24u8, 46u8, 82u8, 169u8, 245u8, 187u8, 222u8, 237u8, 123u8,
                35u8, 114u8, 146u8, 122u8, 38u8, 198u8, 149u8, 131u8, 150u8, 19u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                178u8, 203u8, 230u8, 94u8, 163u8, 8u8, 191u8, 228u8, 185u8, 67u8, 24u8,
                25u8, 163u8, 22u8, 141u8, 84u8, 79u8, 70u8, 186u8, 52u8, 75u8, 30u8,
                121u8, 249u8, 47u8, 151u8, 63u8, 207u8, 244u8, 58u8, 174u8, 59u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                190u8, 79u8, 101u8, 93u8, 170u8, 224u8, 219u8, 174u8, 246u8, 58u8, 107u8,
                82u8, 92u8, 171u8, 47u8, 166u8, 172u8, 228u8, 170u8, 91u8, 148u8, 184u8,
                131u8, 75u8, 36u8, 17u8, 55u8, 205u8, 254u8, 115u8, 165u8, 176u8,
            ],
            [
                199u8, 245u8, 5u8, 178u8, 243u8, 113u8, 174u8, 33u8, 117u8, 238u8, 73u8,
                19u8, 244u8, 73u8, 158u8, 31u8, 38u8, 51u8, 167u8, 181u8, 147u8, 99u8,
                33u8, 238u8, 209u8, 205u8, 174u8, 182u8, 17u8, 81u8, 129u8, 210u8,
            ],
            [
                228u8, 24u8, 2u8, 175u8, 114u8, 87u8, 41u8, 173u8, 203u8, 140u8, 21u8,
                30u8, 41u8, 55u8, 56u8, 10u8, 37u8, 198u8, 145u8, 85u8, 117u8, 126u8,
                58u8, 245u8, 211u8, 151u8, 154u8, 218u8, 181u8, 3u8, 88u8, 0u8,
            ],
            [
                255u8, 226u8, 11u8, 219u8, 133u8, 94u8, 81u8, 78u8, 148u8, 20u8, 119u8,
                2u8, 146u8, 38u8, 144u8, 207u8, 29u8, 161u8, 11u8, 221u8, 24u8, 191u8,
                31u8, 98u8, 21u8, 2u8, 124u8, 147u8, 172u8, 5u8, 212u8, 85u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for GatewayConfigEvents {
        const NAME: &'static str = "GatewayConfigEvents";
        const COUNT: usize = 17usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AddHostChain as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AddHostChain as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::AddHostChain)
                }
                Some(
                    <InitializeGatewayConfig as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <InitializeGatewayConfig as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::InitializeGatewayConfig)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferStarted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <PauseAllGatewayContracts as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PauseAllGatewayContracts as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PauseAllGatewayContracts)
                }
                Some(
                    <ReinitializeGatewayConfigV3 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ReinitializeGatewayConfigV3 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ReinitializeGatewayConfigV3)
                }
                Some(
                    <UnpauseAllGatewayContracts as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UnpauseAllGatewayContracts as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UnpauseAllGatewayContracts)
                }
                Some(
                    <UpdateCoprocessorThreshold as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateCoprocessorThreshold as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdateCoprocessorThreshold)
                }
                Some(
                    <UpdateCoprocessors as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateCoprocessors as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdateCoprocessors)
                }
                Some(<UpdateCustodians as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UpdateCustodians as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdateCustodians)
                }
                Some(
                    <UpdateKmsGenThreshold as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateKmsGenThreshold as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdateKmsGenThreshold)
                }
                Some(<UpdateKmsNodes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <UpdateKmsNodes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdateKmsNodes)
                }
                Some(
                    <UpdateMpcThreshold as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateMpcThreshold as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdateMpcThreshold)
                }
                Some(
                    <UpdatePublicDecryptionThreshold as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdatePublicDecryptionThreshold as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdatePublicDecryptionThreshold)
                }
                Some(
                    <UpdateUserDecryptionThreshold as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateUserDecryptionThreshold as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UpdateUserDecryptionThreshold)
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
    impl alloy_sol_types::private::IntoLogData for GatewayConfigEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddHostChain(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::InitializeGatewayConfig(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PauseAllGatewayContracts(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ReinitializeGatewayConfigV3(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UnpauseAllGatewayContracts(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateCoprocessorThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateCoprocessors(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateCustodians(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateKmsGenThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateKmsNodes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateMpcThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdatePublicDecryptionThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateUserDecryptionThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddHostChain(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::InitializeGatewayConfig(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PauseAllGatewayContracts(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ReinitializeGatewayConfigV3(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UnpauseAllGatewayContracts(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateCoprocessorThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateCoprocessors(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateCustodians(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateKmsGenThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateKmsNodes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateMpcThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdatePublicDecryptionThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateUserDecryptionThreshold(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`GatewayConfig`](self) contract instance.

See the [wrapper's documentation](`GatewayConfigInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> GatewayConfigInstance<P, N> {
        GatewayConfigInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<GatewayConfigInstance<P, N>>,
    > {
        GatewayConfigInstance::<P, N>::deploy(provider)
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
        GatewayConfigInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`GatewayConfig`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`GatewayConfig`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct GatewayConfigInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for GatewayConfigInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("GatewayConfigInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > GatewayConfigInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`GatewayConfig`](self) contract instance.

See the [wrapper's documentation](`GatewayConfigInstance`) for more details.*/
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
        ) -> alloy_contract::Result<GatewayConfigInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> GatewayConfigInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> GatewayConfigInstance<P, N> {
            GatewayConfigInstance {
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
    > GatewayConfigInstance<P, N> {
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
        ///Creates a new call builder for the [`acceptOwnership`] function.
        pub fn acceptOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, acceptOwnershipCall, N> {
            self.call_builder(&acceptOwnershipCall)
        }
        ///Creates a new call builder for the [`addHostChain`] function.
        pub fn addHostChain(
            &self,
            hostChain: <HostChain as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, addHostChainCall, N> {
            self.call_builder(&addHostChainCall { hostChain })
        }
        ///Creates a new call builder for the [`getCoprocessor`] function.
        pub fn getCoprocessor(
            &self,
            coprocessorTxSenderAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getCoprocessorCall, N> {
            self.call_builder(
                &getCoprocessorCall {
                    coprocessorTxSenderAddress,
                },
            )
        }
        ///Creates a new call builder for the [`getCoprocessorMajorityThreshold`] function.
        pub fn getCoprocessorMajorityThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCoprocessorMajorityThresholdCall, N> {
            self.call_builder(&getCoprocessorMajorityThresholdCall)
        }
        ///Creates a new call builder for the [`getCoprocessorSigners`] function.
        pub fn getCoprocessorSigners(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCoprocessorSignersCall, N> {
            self.call_builder(&getCoprocessorSignersCall)
        }
        ///Creates a new call builder for the [`getCoprocessorTxSenders`] function.
        pub fn getCoprocessorTxSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCoprocessorTxSendersCall, N> {
            self.call_builder(&getCoprocessorTxSendersCall)
        }
        ///Creates a new call builder for the [`getCustodian`] function.
        pub fn getCustodian(
            &self,
            custodianTxSenderAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getCustodianCall, N> {
            self.call_builder(
                &getCustodianCall {
                    custodianTxSenderAddress,
                },
            )
        }
        ///Creates a new call builder for the [`getCustodianSigners`] function.
        pub fn getCustodianSigners(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCustodianSignersCall, N> {
            self.call_builder(&getCustodianSignersCall)
        }
        ///Creates a new call builder for the [`getCustodianTxSenders`] function.
        pub fn getCustodianTxSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCustodianTxSendersCall, N> {
            self.call_builder(&getCustodianTxSendersCall)
        }
        ///Creates a new call builder for the [`getHostChain`] function.
        pub fn getHostChain(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getHostChainCall, N> {
            self.call_builder(&getHostChainCall { index })
        }
        ///Creates a new call builder for the [`getHostChains`] function.
        pub fn getHostChains(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getHostChainsCall, N> {
            self.call_builder(&getHostChainsCall)
        }
        ///Creates a new call builder for the [`getKmsGenThreshold`] function.
        pub fn getKmsGenThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsGenThresholdCall, N> {
            self.call_builder(&getKmsGenThresholdCall)
        }
        ///Creates a new call builder for the [`getKmsNode`] function.
        pub fn getKmsNode(
            &self,
            kmsTxSenderAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsNodeCall, N> {
            self.call_builder(
                &getKmsNodeCall {
                    kmsTxSenderAddress,
                },
            )
        }
        ///Creates a new call builder for the [`getKmsSigners`] function.
        pub fn getKmsSigners(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsSignersCall, N> {
            self.call_builder(&getKmsSignersCall)
        }
        ///Creates a new call builder for the [`getKmsTxSenders`] function.
        pub fn getKmsTxSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKmsTxSendersCall, N> {
            self.call_builder(&getKmsTxSendersCall)
        }
        ///Creates a new call builder for the [`getMpcThreshold`] function.
        pub fn getMpcThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getMpcThresholdCall, N> {
            self.call_builder(&getMpcThresholdCall)
        }
        ///Creates a new call builder for the [`getProtocolMetadata`] function.
        pub fn getProtocolMetadata(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getProtocolMetadataCall, N> {
            self.call_builder(&getProtocolMetadataCall)
        }
        ///Creates a new call builder for the [`getPublicDecryptionThreshold`] function.
        pub fn getPublicDecryptionThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getPublicDecryptionThresholdCall, N> {
            self.call_builder(&getPublicDecryptionThresholdCall)
        }
        ///Creates a new call builder for the [`getUserDecryptionThreshold`] function.
        pub fn getUserDecryptionThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getUserDecryptionThresholdCall, N> {
            self.call_builder(&getUserDecryptionThresholdCall)
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
            initialMetadata: <ProtocolMetadata as alloy::sol_types::SolType>::RustType,
            initialThresholds: <IGatewayConfig::Thresholds as alloy::sol_types::SolType>::RustType,
            initialKmsNodes: alloy::sol_types::private::Vec<
                <KmsNode as alloy::sol_types::SolType>::RustType,
            >,
            initialCoprocessors: alloy::sol_types::private::Vec<
                <Coprocessor as alloy::sol_types::SolType>::RustType,
            >,
            initialCustodians: alloy::sol_types::private::Vec<
                <Custodian as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(
                &initializeFromEmptyProxyCall {
                    initialMetadata,
                    initialThresholds,
                    initialKmsNodes,
                    initialCoprocessors,
                    initialCustodians,
                },
            )
        }
        ///Creates a new call builder for the [`isCoprocessorSigner`] function.
        pub fn isCoprocessorSigner(
            &self,
            signerAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isCoprocessorSignerCall, N> {
            self.call_builder(
                &isCoprocessorSignerCall {
                    signerAddress,
                },
            )
        }
        ///Creates a new call builder for the [`isCoprocessorTxSender`] function.
        pub fn isCoprocessorTxSender(
            &self,
            txSenderAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isCoprocessorTxSenderCall, N> {
            self.call_builder(
                &isCoprocessorTxSenderCall {
                    txSenderAddress,
                },
            )
        }
        ///Creates a new call builder for the [`isCustodianSigner`] function.
        pub fn isCustodianSigner(
            &self,
            signerAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isCustodianSignerCall, N> {
            self.call_builder(
                &isCustodianSignerCall {
                    signerAddress,
                },
            )
        }
        ///Creates a new call builder for the [`isCustodianTxSender`] function.
        pub fn isCustodianTxSender(
            &self,
            txSenderAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isCustodianTxSenderCall, N> {
            self.call_builder(
                &isCustodianTxSenderCall {
                    txSenderAddress,
                },
            )
        }
        ///Creates a new call builder for the [`isHostChainRegistered`] function.
        pub fn isHostChainRegistered(
            &self,
            chainId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isHostChainRegisteredCall, N> {
            self.call_builder(
                &isHostChainRegisteredCall {
                    chainId,
                },
            )
        }
        ///Creates a new call builder for the [`isKmsSigner`] function.
        pub fn isKmsSigner(
            &self,
            signerAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isKmsSignerCall, N> {
            self.call_builder(&isKmsSignerCall { signerAddress })
        }
        ///Creates a new call builder for the [`isKmsTxSender`] function.
        pub fn isKmsTxSender(
            &self,
            txSenderAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isKmsTxSenderCall, N> {
            self.call_builder(
                &isKmsTxSenderCall {
                    txSenderAddress,
                },
            )
        }
        ///Creates a new call builder for the [`isPauser`] function.
        pub fn isPauser(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isPauserCall, N> {
            self.call_builder(&isPauserCall { account })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`pauseAllGatewayContracts`] function.
        pub fn pauseAllGatewayContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, pauseAllGatewayContractsCall, N> {
            self.call_builder(&pauseAllGatewayContractsCall)
        }
        ///Creates a new call builder for the [`pendingOwner`] function.
        pub fn pendingOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, pendingOwnerCall, N> {
            self.call_builder(&pendingOwnerCall)
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
            newKmsNodes: alloy::sol_types::private::Vec<
                <KmsNode as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV3Call, N> {
            self.call_builder(&reinitializeV3Call { newKmsNodes })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpauseAllGatewayContracts`] function.
        pub fn unpauseAllGatewayContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, unpauseAllGatewayContractsCall, N> {
            self.call_builder(&unpauseAllGatewayContractsCall)
        }
        ///Creates a new call builder for the [`updateCoprocessorThreshold`] function.
        pub fn updateCoprocessorThreshold(
            &self,
            newCoprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateCoprocessorThresholdCall, N> {
            self.call_builder(
                &updateCoprocessorThresholdCall {
                    newCoprocessorThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`updateCoprocessors`] function.
        pub fn updateCoprocessors(
            &self,
            newCoprocessors: alloy::sol_types::private::Vec<
                <Coprocessor as alloy::sol_types::SolType>::RustType,
            >,
            newCoprocessorThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateCoprocessorsCall, N> {
            self.call_builder(
                &updateCoprocessorsCall {
                    newCoprocessors,
                    newCoprocessorThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`updateCustodians`] function.
        pub fn updateCustodians(
            &self,
            newCustodians: alloy::sol_types::private::Vec<
                <Custodian as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, updateCustodiansCall, N> {
            self.call_builder(
                &updateCustodiansCall {
                    newCustodians,
                },
            )
        }
        ///Creates a new call builder for the [`updateKmsGenThreshold`] function.
        pub fn updateKmsGenThreshold(
            &self,
            newKmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateKmsGenThresholdCall, N> {
            self.call_builder(
                &updateKmsGenThresholdCall {
                    newKmsGenThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`updateKmsNodes`] function.
        pub fn updateKmsNodes(
            &self,
            newKmsNodes: alloy::sol_types::private::Vec<
                <KmsNode as alloy::sol_types::SolType>::RustType,
            >,
            newMpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
            newPublicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
            newUserDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
            newKmsGenThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateKmsNodesCall, N> {
            self.call_builder(
                &updateKmsNodesCall {
                    newKmsNodes,
                    newMpcThreshold,
                    newPublicDecryptionThreshold,
                    newUserDecryptionThreshold,
                    newKmsGenThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`updateMpcThreshold`] function.
        pub fn updateMpcThreshold(
            &self,
            newMpcThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateMpcThresholdCall, N> {
            self.call_builder(
                &updateMpcThresholdCall {
                    newMpcThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`updatePublicDecryptionThreshold`] function.
        pub fn updatePublicDecryptionThreshold(
            &self,
            newPublicDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updatePublicDecryptionThresholdCall, N> {
            self.call_builder(
                &updatePublicDecryptionThresholdCall {
                    newPublicDecryptionThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`updateUserDecryptionThreshold`] function.
        pub fn updateUserDecryptionThreshold(
            &self,
            newUserDecryptionThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, updateUserDecryptionThresholdCall, N> {
            self.call_builder(
                &updateUserDecryptionThresholdCall {
                    newUserDecryptionThreshold,
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
    > GatewayConfigInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AddHostChain`] event.
        pub fn AddHostChain_filter(&self) -> alloy_contract::Event<&P, AddHostChain, N> {
            self.event_filter::<AddHostChain>()
        }
        ///Creates a new event filter for the [`InitializeGatewayConfig`] event.
        pub fn InitializeGatewayConfig_filter(
            &self,
        ) -> alloy_contract::Event<&P, InitializeGatewayConfig, N> {
            self.event_filter::<InitializeGatewayConfig>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OwnershipTransferStarted`] event.
        pub fn OwnershipTransferStarted_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferStarted, N> {
            self.event_filter::<OwnershipTransferStarted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PauseAllGatewayContracts`] event.
        pub fn PauseAllGatewayContracts_filter(
            &self,
        ) -> alloy_contract::Event<&P, PauseAllGatewayContracts, N> {
            self.event_filter::<PauseAllGatewayContracts>()
        }
        ///Creates a new event filter for the [`ReinitializeGatewayConfigV3`] event.
        pub fn ReinitializeGatewayConfigV3_filter(
            &self,
        ) -> alloy_contract::Event<&P, ReinitializeGatewayConfigV3, N> {
            self.event_filter::<ReinitializeGatewayConfigV3>()
        }
        ///Creates a new event filter for the [`UnpauseAllGatewayContracts`] event.
        pub fn UnpauseAllGatewayContracts_filter(
            &self,
        ) -> alloy_contract::Event<&P, UnpauseAllGatewayContracts, N> {
            self.event_filter::<UnpauseAllGatewayContracts>()
        }
        ///Creates a new event filter for the [`UpdateCoprocessorThreshold`] event.
        pub fn UpdateCoprocessorThreshold_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdateCoprocessorThreshold, N> {
            self.event_filter::<UpdateCoprocessorThreshold>()
        }
        ///Creates a new event filter for the [`UpdateCoprocessors`] event.
        pub fn UpdateCoprocessors_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdateCoprocessors, N> {
            self.event_filter::<UpdateCoprocessors>()
        }
        ///Creates a new event filter for the [`UpdateCustodians`] event.
        pub fn UpdateCustodians_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdateCustodians, N> {
            self.event_filter::<UpdateCustodians>()
        }
        ///Creates a new event filter for the [`UpdateKmsGenThreshold`] event.
        pub fn UpdateKmsGenThreshold_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdateKmsGenThreshold, N> {
            self.event_filter::<UpdateKmsGenThreshold>()
        }
        ///Creates a new event filter for the [`UpdateKmsNodes`] event.
        pub fn UpdateKmsNodes_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdateKmsNodes, N> {
            self.event_filter::<UpdateKmsNodes>()
        }
        ///Creates a new event filter for the [`UpdateMpcThreshold`] event.
        pub fn UpdateMpcThreshold_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdateMpcThreshold, N> {
            self.event_filter::<UpdateMpcThreshold>()
        }
        ///Creates a new event filter for the [`UpdatePublicDecryptionThreshold`] event.
        pub fn UpdatePublicDecryptionThreshold_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdatePublicDecryptionThreshold, N> {
            self.event_filter::<UpdatePublicDecryptionThreshold>()
        }
        ///Creates a new event filter for the [`UpdateUserDecryptionThreshold`] event.
        pub fn UpdateUserDecryptionThreshold_filter(
            &self,
        ) -> alloy_contract::Event<&P, UpdateUserDecryptionThreshold, N> {
            self.event_filter::<UpdateUserDecryptionThreshold>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
