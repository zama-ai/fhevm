///Module containing a contract's types and functions.
/**

```solidity
library IProtocolConfig {
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
library IProtocolConfig {
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
    struct CoprocessorContext {
        uint64 gwStartBlock;
        uint64 activatedAtBlock;
        bool destroyed;
        string softwareVersion;
        ChainUpgradeWindow[] chainUpgradeWindows;
    }
    struct KmsNode {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
    }

    error AddressEmptyCode(address target);
    error CurrentKmsContextCannotBeDestroyed(uint256 kmsContextId);
    error DuplicateChainId(uint64 chainId);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyChainUpgradeWindows();
    error EmptyKmsNodes();
    error EmptySoftwareVersion();
    error FailedCall();
    error InvalidBlockWindow(uint64 chainId, uint64 startBlock, uint64 endBlock);
    error InvalidCoprocessorContext(uint256 coprocessorContextId);
    error InvalidHighThreshold(string thresholdName, uint256 threshold, uint256 nodeCount);
    error InvalidInitialization();
    error InvalidKmsContext(uint256 kmsContextId);
    error InvalidNullThreshold(string thresholdName);
    error KmsNodeNullSigner();
    error KmsNodeNullTxSender();
    error KmsSignerAlreadyRegistered(address signer);
    error KmsSignerSetExceedsProofFormatLimit(uint256 signerCount, uint256 maxAllowed);
    error KmsTxSenderAlreadyRegistered(address txSender);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error ThresholdExceedsProofFormatLimit(string thresholdName, uint256 threshold, uint256 maxAllowed);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error ZeroChainId();
    error ZeroGwStartBlock();

    event CoprocessorContextDestroyed(uint256 indexed coprocessorContextId);
    event Initialized(uint64 version);
    event KmsContextDestroyed(uint256 indexed kmsContextId);
    event NewCoprocessorContext(uint256 indexed coprocessorContextId, string softwareVersion, ChainUpgradeWindow[] chainUpgradeWindows, uint64 gwStartBlock);
    event NewKmsContext(uint256 indexed kmsContextId, KmsNode[] kmsNodes, IProtocolConfig.KmsThresholds thresholds);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function defineNewCoprocessorContext(string memory softwareVersion, ChainUpgradeWindow[] memory chainUpgradeWindows, uint64 gwStartBlock) external;
    function defineNewKmsContext(KmsNode[] memory kmsNodes, IProtocolConfig.KmsThresholds memory thresholds) external;
    function destroyCoprocessorContext(uint256 coprocessorContextId) external;
    function destroyKmsContext(uint256 kmsContextId) external;
    function getCoprocessorContext(uint256 coprocessorContextId) external view returns (CoprocessorContext memory);
    function getCurrentCoprocessorContextId() external view returns (uint256);
    function getCurrentKmsContextId() external view returns (uint256);
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
    function initializeFromEmptyProxy(KmsNode[] memory initialKmsNodes, IProtocolConfig.KmsThresholds memory initialThresholds) external;
    function initializeFromMigration(uint256 existingContextId, KmsNode[] memory existingKmsNodes, IProtocolConfig.KmsThresholds memory existingThresholds) external;
    function isKmsSigner(address signer) external view returns (bool);
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);
    function isValidCoprocessorContext(uint256 coprocessorContextId) external view returns (bool);
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);
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
    "name": "defineNewCoprocessorContext",
    "inputs": [
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
    "name": "defineNewKmsContext",
    "inputs": [
      {
        "name": "kmsNodes",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "destroyCoprocessorContext",
    "inputs": [
      {
        "name": "coprocessorContextId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getCoprocessorContext",
    "inputs": [
      {
        "name": "coprocessorContextId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct CoprocessorContext",
        "components": [
          {
            "name": "gwStartBlock",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "activatedAtBlock",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "destroyed",
            "type": "bool",
            "internalType": "bool"
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
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentCoprocessorContextId",
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "initializeFromMigration",
    "inputs": [
      {
        "name": "existingContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "existingKmsNodes",
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
        "name": "existingThresholds",
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
    "name": "isValidCoprocessorContext",
    "inputs": [
      {
        "name": "coprocessorContextId",
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
    "name": "CoprocessorContextDestroyed",
    "inputs": [
      {
        "name": "coprocessorContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
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
    "name": "NewCoprocessorContext",
    "inputs": [
      {
        "name": "coprocessorContextId",
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
    "name": "NewKmsContext",
    "inputs": [
      {
        "name": "kmsContextId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
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
    "name": "InvalidCoprocessorContext",
    "inputs": [
      {
        "name": "coprocessorContextId",
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
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051615025620001eb5f395f818161241e01528181612473015261271501526150255ff3fe6080604052600436106101d7575f3560e01c80637eaac8f211610101578063bf9b16c811610094578063d740e40211610063578063d740e402146106f9578063d8f8392b14610721578063f76ca57714610749578063f9c670c314610771576101d7565b8063bf9b16c81461062f578063c0ae64f71461066b578063c2b4298614610693578063c3aaaa5a146106bd576101d7565b8063a92c75cb116100d0578063a92c75cb1461059d578063ad3cb1cc146105c5578063b4722bc4146105ef578063bac22bb814610619576101d7565b80637eaac8f2146104d15780639447cfd4146104fb578063976f3eb9146105375780639a7860e014610561576101d7565b806331ff41c8116101795780634f1ef286116101485780634f1ef2861461042757806352d1902d14610443578063556ecafa1461046d5780635bff76d914610495576101d7565b806331ff41c81461033757806341ad069c1461037357806346c5bbbd146103af57806347e82295146103eb576101d7565b8063203d0114116101b5578063203d01141461026b57806326cf5def146102a7578063281e8bfe146102d15780632a3889981461030d576101d7565b80630d8e6e2c146101db5780630e1887c914610205578063170a298114610241575b5f80fd5b3480156101e6575f80fd5b506101ef6107ad565b6040516101fc919061363b565b60405180910390f35b348015610210575f80fd5b5061022b6004803603810190610226919061369f565b610828565b60405161023891906136e4565b60405180910390f35b34801561024c575f80fd5b50610255610839565b604051610262919061370c565b60405180910390f35b348015610276575f80fd5b50610291600480360381019061028c919061377f565b61084b565b60405161029e91906136e4565b60405180910390f35b3480156102b2575f80fd5b506102bb6108bd565b6040516102c8919061370c565b60405180910390f35b3480156102dc575f80fd5b506102f760048036038101906102f2919061369f565b6108e6565b604051610304919061370c565b60405180910390f35b348015610318575f80fd5b50610321610912565b60405161032e919061370c565b60405180910390f35b348015610342575f80fd5b5061035d600480360381019061035891906137aa565b61093b565b60405161036a91906138a6565b60405180910390f35b34801561037e575f80fd5b506103996004803603810190610394919061369f565b610b7d565b6040516103a6919061370c565b60405180910390f35b3480156103ba575f80fd5b506103d560048036038101906103d091906137aa565b610ba9565b6040516103e291906136e4565b60405180910390f35b3480156103f6575f80fd5b50610411600480360381019061040c919061369f565b610c1d565b60405161041e919061370c565b60405180910390f35b610441600480360381019061043c91906139f2565b610c49565b005b34801561044e575f80fd5b50610457610c68565b6040516104649190613a64565b60405180910390f35b348015610478575f80fd5b50610493600480360381019061048e9190613afc565b610c99565b005b3480156104a0575f80fd5b506104bb60048036038101906104b6919061369f565b610e9d565b6040516104c89190613c15565b60405180910390f35b3480156104dc575f80fd5b506104e5610f4b565b6040516104f29190613c15565b60405180910390f35b348015610506575f80fd5b50610521600480360381019061051c91906137aa565b610ff6565b60405161052e91906136e4565b60405180910390f35b348015610542575f80fd5b5061054b61106a565b604051610558919061370c565b60405180910390f35b34801561056c575f80fd5b506105876004803603810190610582919061369f565b61107b565b6040516105949190613dc8565b60405180910390f35b3480156105a8575f80fd5b506105c360048036038101906105be9190613de8565b6112ee565b005b3480156105d0575f80fd5b506105d96113ef565b6040516105e6919061363b565b60405180910390f35b3480156105fa575f80fd5b50610603611428565b604051610610919061370c565b60405180910390f35b348015610624575f80fd5b5061062d611451565b005b34801561063a575f80fd5b506106556004803603810190610650919061369f565b61158b565b60405161066291906136e4565b60405180910390f35b348015610676575f80fd5b50610691600480360381019061068c919061369f565b61159c565b005b34801561069e575f80fd5b506106a7611784565b6040516106b4919061370c565b60405180910390f35b3480156106c8575f80fd5b506106e360048036038101906106de919061369f565b6117ad565b6040516106f0919061370c565b60405180910390f35b348015610704575f80fd5b5061071f600480360381019061071a919061369f565b6117d9565b005b34801561072c575f80fd5b5061074760048036038101906107429190613de8565b611978565b005b348015610754575f80fd5b5061076f600480360381019061076a9190613f19565b611b1e565b005b34801561077c575f80fd5b506107976004803603810190610792919061369f565b612019565b6040516107a491906140cc565b60405180910390f35b60606040518060400160405280600e81526020017f50726f746f636f6c436f6e6669670000000000000000000000000000000000008152506107ee5f612261565b6107f86001612261565b6108015f612261565b60405160200161081494939291906141ba565b604051602081830303815290604052905090565b5f6108328261232b565b9050919050565b5f6108426123a8565b600b0154905090565b5f806108556123a8565b9050806003015f825f015481526020019081526020015f205f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806108c76123a8565b9050806009015f825f015481526020019081526020015f205491505090565b5f6108f0826123cf565b6108f86123a8565b6007015f8381526020019081526020015f20549050919050565b5f8061091c6123a8565b9050806006015f825f015481526020019081526020015f205491505090565b61094361351d565b61094c836123cf565b6109546123a8565b6004015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054610a6590614245565b80601f0160208091040260200160405190810160405280929190818152602001828054610a9190614245565b8015610adc5780601f10610ab357610100808354040283529160200191610adc565b820191905f5260205f20905b815481529060010190602001808311610abf57829003601f168201915b50505050508152602001600382018054610af590614245565b80601f0160208091040260200160405190810160405280929190818152602001828054610b2190614245565b8015610b6c5780601f10610b4357610100808354040283529160200191610b6c565b820191905f5260205f20905b815481529060010190602001808311610b4f57829003601f168201915b505050505081525050905092915050565b5f610b87826123cf565b610b8f6123a8565b6008015f8381526020019081526020015f20549050919050565b5f610bb3836123cf565b610bbb6123a8565b6002015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f610c27826123cf565b610c2f6123a8565b6009015f8381526020019081526020015f20549050919050565b610c5161241c565b610c5a82612502565b610c6482826125f5565b5050565b5f610c71612713565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b6001610ca361279a565b67ffffffffffffffff1614610ce4576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610cef6127be565b9050805f0160089054906101000a900460ff1680610d3757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610d6e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550600160f86007901b610dc591906142a2565b861015610e0957856040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401610e00919061370c565b60405180910390fd5b5f610e126123a8565b9050600187610e2191906142d5565b815f018190555060f86009901b81600b0181905550610e418686866127e5565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610e8d9190614317565b60405180910390a1505050505050565b6060610ea8826123cf565b610eb06123a8565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610f3f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610ef6575b50505050509050919050565b60605f610f566123a8565b9050806005015f825f015481526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610feb57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610fa2575b505050505091505090565b5f611000836123cf565b6110086123a8565b6003015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f6110736123a8565b5f0154905090565b61108361356f565b61108c8261232b565b6110cd57816040517f9797c3ff0000000000000000000000000000000000000000000000000000000081526004016110c4919061370c565b60405180910390fd5b6110d56123a8565b600c015f8381526020019081526020015f206040518060a00160405290815f82015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160089054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160109054906101000a900460ff1615151515815260200160018201805461117d90614245565b80601f01602080910402602001604051908101604052809291908181526020018280546111a990614245565b80156111f45780601f106111cb576101008083540402835291602001916111f4565b820191905f5260205f20905b8154815290600101906020018083116111d757829003601f168201915b5050505050815260200160028201805480602002602001604051908101604052809291908181526020015f905b828210156112df578382905f5260205f20016040518060600160405290815f82015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160089054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160109054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff168152505081526020019060010190611221565b50505050815250509050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561134b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061136f9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146113de57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016113d5919061437e565b60405180910390fd5b6113e98383836127e5565b50505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806114326123a8565b9050806008015f825f015481526020019081526020015f205491505090565b60035f61145c6127be565b9050805f0160089054906101000a900460ff16806114a457508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156114db576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555060f86009901b61152e6123a8565b600b01819055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d28260405161157f9190614317565b60405180910390a15050565b5f61159582612e31565b9050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115f9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061161d9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461168c57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611683919061437e565b60405180910390fd5b5f6116956123a8565b9050805f015482036116de57816040517f4595fce20000000000000000000000000000000000000000000000000000000081526004016116d5919061370c565b60405180910390fd5b6116e782612e31565b61172857816040517f77ddbe8100000000000000000000000000000000000000000000000000000000815260040161171f919061370c565b60405180910390fd5b600181600a015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550817fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f060405160405180910390a25050565b5f8061178e6123a8565b9050806007015f825f015481526020019081526020015f205491505090565b5f6117b7826123cf565b6117bf6123a8565b6006015f8381526020019081526020015f20549050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611836573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061185a9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146118c957336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016118c0919061437e565b60405180910390fd5b6118d28161232b565b61191357806040517f9797c3ff00000000000000000000000000000000000000000000000000000000815260040161190a919061370c565b60405180910390fd5b600161191d6123a8565b600c015f8381526020019081526020015f205f0160106101000a81548160ff021916908315150217905550807f6ed5f2c759f9fa25b478511dae2aa768dc993e9d04ab15f9c2519f075c4725d360405160405180910390a250565b600161198261279a565b67ffffffffffffffff16146119c3576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f6119ce6127be565b9050805f0160089054906101000a900460ff1680611a1657508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15611a4d576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f611a9b6123a8565b905060f86007901b815f018190555060f86009901b81600b0181905550611ac38686866127e5565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051611b0f9190614317565b60405180910390a15050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b7b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b9f9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611c0e57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611c05919061437e565b60405180910390fd5b5f8585905003611c4a576040517fb548914700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8383905003611c86576040517fbe50504400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8167ffffffffffffffff1603611cc9576040517f17d3e94800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b83839050811015611ec55736848483818110611cea57611ce9614397565b5b90506060020190505f815f016020810190611d0591906143c4565b67ffffffffffffffff1603611d46576040517fc84885d400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b806040016020810190611d5991906143c4565b67ffffffffffffffff16816020016020810190611d7691906143c4565b67ffffffffffffffff161115611dfc57805f016020810190611d9891906143c4565b816020016020810190611dab91906143c4565b826040016020810190611dbe91906143c4565b6040517ff219dc0e000000000000000000000000000000000000000000000000000000008152600401611df3939291906143ef565b60405180910390fd5b5f5b82811015611eb657815f016020810190611e1891906143c4565b67ffffffffffffffff16868683818110611e3557611e34614397565b5b9050606002015f016020810190611e4c91906143c4565b67ffffffffffffffff1603611ea957815f016020810190611e6d91906143c4565b6040517f6c67e470000000000000000000000000000000000000000000000000000000008152600401611ea09190614317565b60405180910390fd5b8080600101915050611dfe565b50508080600101915050611ccb565b505f611ecf6123a8565b90505f81600b015f8154611ee290614424565b91905081905590505f82600c015f8381526020019081526020015f2090508787826001019182611f13929190614612565b5083815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff16021790555043815f0160086101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f5b86869050811015611fce5781600201878783818110611f8b57611f8a614397565b5b905060600201908060018154018082558091505060019003905f5260205f20015f909190919091508181611fbf91906148a0565b50508080600101915050611f69565b50817f595d10949fcf822de17e89ebc302566ed150171ff414fe14d92b78a6d3aecce889898989896040516120079594939291906149e9565b60405180910390a25050505050505050565b6060612024826123cf565b61202c6123a8565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015612256578382905f5260205f2090600402016040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461213790614245565b80601f016020809104026020016040519081016040528092919081815260200182805461216390614245565b80156121ae5780601f10612185576101008083540402835291602001916121ae565b820191905f5260205f20905b81548152906001019060200180831161219157829003601f168201915b505050505081526020016003820180546121c790614245565b80601f01602080910402602001604051908101604052809291908181526020018280546121f390614245565b801561223e5780601f106122155761010080835404028352916020019161223e565b820191905f5260205f20905b81548152906001019060200180831161222157829003601f168201915b5050505050815250508152602001906001019061205d565b505050509050919050565b60605f600161226f84612eb4565b0190505f8167ffffffffffffffff81111561228d5761228c6138ce565b5b6040519080825280601f01601f1916602001820160405280156122bf5781602001600182028036833780820191505090505b5090505f82602001820190505b600115612320578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161231557612314614a30565b5b0494505f85036122cc575b819350505050919050565b5f806123356123a8565b90505f81600c015f8581526020019081526020015f209050600160f86009901b61235f91906142a2565b8410158015612372575081600b01548411155b801561238557505f816002018054905014155b801561239f5750805f0160109054906101000a900460ff16155b92505050919050565b5f7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200905090565b6123d881612e31565b61241957806040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401612410919061370c565b60405180910390fd5b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614806124c957507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166124b0613005565b73ffffffffffffffffffffffffffffffffffffffff1614155b15612500576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561255f573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125839190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146125f257336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016125e9919061437e565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561265d57506040513d601f19601f8201168201806040525081019061265a9190614a87565b60015b61269e57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401612695919061437e565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461270457806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016126fb9190613a64565b60405180910390fd5b61270e8383613058565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612798576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6127a36127be565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f808484905003612822576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60ff8016848490501115612875578383905060ff80166040517f16a7277800000000000000000000000000000000000000000000000000000000815260040161286c929190614ab2565b60405180910390fd5b61288282858590506130ca565b5f61288b6123a8565b9050805f015f815461289c90614424565b91905081905591505f5b85859050811015612d7d57368686838181106128c5576128c4614397565b5b90506020028101906128d79190614ae5565b90505f73ffffffffffffffffffffffffffffffffffffffff16815f016020810190612902919061377f565b73ffffffffffffffffffffffffffffffffffffffff160361294f576040517f8466804a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff16816020016020810190612979919061377f565b73ffffffffffffffffffffffffffffffffffffffff16036129c6576040517f2deccf4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826002015f8581526020019081526020015f205f825f0160208101906129ec919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612a8557805f016020810190612a49919061377f565b6040517fd18c4ff0000000000000000000000000000000000000000000000000000000008152600401612a7c919061437e565b60405180910390fd5b826003015f8581526020019081526020015f205f826020016020810190612aac919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612b4657806020016020810190612b0a919061377f565b6040517ff51af6bb000000000000000000000000000000000000000000000000000000008152600401612b3d919061437e565b60405180910390fd5b826001015f8581526020019081526020015f2081908060018154018082558091505060019003905f5260205f2090600402015f909190919091508181612b8c9190614cbd565b50506001836002015f8681526020019081526020015f205f835f016020810190612bb6919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506001836003015f8681526020019081526020015f205f836020016020810190612c2e919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080836004015f8681526020019081526020015f205f835f016020810190612ca4919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181612ce99190614cbd565b905050826005015f8581526020019081526020015f20816020016020810190612d12919061377f565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505080806001019150506128a6565b50825f0135816006015f8481526020019081526020015f20819055508260200135816007015f8481526020019081526020015f20819055508260400135816008015f8481526020019081526020015f20819055508260600135816009015f8481526020019081526020015f2081905550817fe5296a8184d19a5fd24548749ea3c435b69ad26f12ca0afa1e8efef592368bf2868686604051612e2193929190614f5f565b60405180910390a2509392505050565b5f80612e3b6123a8565b9050600160f86007901b612e4f91906142a2565b8310158015612e615750805f01548311155b8015612e8357505f816001015f8581526020019081526020015f208054905014155b8015612eac575080600a015f8481526020019081526020015f205f9054906101000a900460ff16155b915050919050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612f10577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381612f0657612f05614a30565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612f4d576d04ee2d6d415b85acef81000000008381612f4357612f42614a30565b5b0492506020810190505b662386f26fc100008310612f7c57662386f26fc100008381612f7257612f71614a30565b5b0492506010810190505b6305f5e1008310612fa5576305f5e1008381612f9b57612f9a614a30565b5b0492506008810190505b6127108310612fca576127108381612fc057612fbf614a30565b5b0492506004810190505b60648310612fed5760648381612fe357612fe2614a30565b5b0492506002810190505b600a8310612ffc576001810190505b80915050919050565b5f6130317f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6131dd565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b613061826131e6565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156130bd576130b782826132af565b506130c6565b6130c561332f565b5b5050565b61310d6040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250835f01358361336b565b6131516040518060400160405280600e81526020017f7573657244656372797074696f6e00000000000000000000000000000000000081525083602001358361336b565b6131956040518060400160405280600681526020017f6b6d7347656e000000000000000000000000000000000000000000000000000081525083604001358361336b565b6131d96040518060400160405280600381526020017f6d7063000000000000000000000000000000000000000000000000000000000081525083606001358361336b565b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b0361324157806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613238919061437e565b60405180910390fd5b8061326d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6131dd565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516132d89190614fd3565b5f60405180830381855af49150503d805f8114613310576040519150601f19603f3d011682016040523d82523d5f602084013e613315565b606091505b509150915061332585838361344c565b9250505092915050565b5f341115613369576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f82036133af57826040517f36bfb60e0000000000000000000000000000000000000000000000000000000081526004016133a6919061363b565b60405180910390fd5b60ff80168211156133fe57828260ff80166040517f22ba52db0000000000000000000000000000000000000000000000000000000081526004016133f593929190614fe9565b60405180910390fd5b80821115613447578282826040517fcaa814a300000000000000000000000000000000000000000000000000000000815260040161343e93929190614fe9565b60405180910390fd5b505050565b6060826134615761345c826134d9565b6134d1565b5f825114801561348757505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156134c957836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016134c0919061437e565b60405180910390fd5b8190506134d2565b5b9392505050565b5f815111156134eb5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b6040518060a001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f1515815260200160608152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b838110156135e85780820151818401526020810190506135cd565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61360d826135b1565b61361781856135bb565b93506136278185602086016135cb565b613630816135f3565b840191505092915050565b5f6020820190508181035f8301526136538184613603565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61367e8161366c565b8114613688575f80fd5b50565b5f8135905061369981613675565b92915050565b5f602082840312156136b4576136b3613664565b5b5f6136c18482850161368b565b91505092915050565b5f8115159050919050565b6136de816136ca565b82525050565b5f6020820190506136f75f8301846136d5565b92915050565b6137068161366c565b82525050565b5f60208201905061371f5f8301846136fd565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61374e82613725565b9050919050565b61375e81613744565b8114613768575f80fd5b50565b5f8135905061377981613755565b92915050565b5f6020828403121561379457613793613664565b5b5f6137a18482850161376b565b91505092915050565b5f80604083850312156137c0576137bf613664565b5b5f6137cd8582860161368b565b92505060206137de8582860161376b565b9150509250929050565b6137f181613744565b82525050565b5f82825260208201905092915050565b5f613811826135b1565b61381b81856137f7565b935061382b8185602086016135cb565b613834816135f3565b840191505092915050565b5f608083015f8301516138545f8601826137e8565b50602083015161386760208601826137e8565b506040830151848203604086015261387f8282613807565b915050606083015184820360608601526138998282613807565b9150508091505092915050565b5f6020820190508181035f8301526138be818461383f565b905092915050565b5f80fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613904826135f3565b810181811067ffffffffffffffff82111715613923576139226138ce565b5b80604052505050565b5f61393561365b565b905061394182826138fb565b919050565b5f67ffffffffffffffff8211156139605761395f6138ce565b5b613969826135f3565b9050602081019050919050565b828183375f83830152505050565b5f61399661399184613946565b61392c565b9050828152602081018484840111156139b2576139b16138ca565b5b6139bd848285613976565b509392505050565b5f82601f8301126139d9576139d86138c6565b5b81356139e9848260208601613984565b91505092915050565b5f8060408385031215613a0857613a07613664565b5b5f613a158582860161376b565b925050602083013567ffffffffffffffff811115613a3657613a35613668565b5b613a42858286016139c5565b9150509250929050565b5f819050919050565b613a5e81613a4c565b82525050565b5f602082019050613a775f830184613a55565b92915050565b5f80fd5b5f80fd5b5f8083601f840112613a9a57613a996138c6565b5b8235905067ffffffffffffffff811115613ab757613ab6613a7d565b5b602083019150836020820283011115613ad357613ad2613a81565b5b9250929050565b5f80fd5b5f60808284031215613af357613af2613ada565b5b81905092915050565b5f805f8060c08587031215613b1457613b13613664565b5b5f613b218782880161368b565b945050602085013567ffffffffffffffff811115613b4257613b41613668565b5b613b4e87828801613a85565b93509350506040613b6187828801613ade565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f613ba183836137e8565b60208301905092915050565b5f602082019050919050565b5f613bc382613b6d565b613bcd8185613b77565b9350613bd883613b87565b805f5b83811015613c08578151613bef8882613b96565b9750613bfa83613bad565b925050600181019050613bdb565b5085935050505092915050565b5f6020820190508181035f830152613c2d8184613bb9565b905092915050565b5f67ffffffffffffffff82169050919050565b613c5181613c35565b82525050565b613c60816136ca565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b606082015f820151613ca35f850182613c48565b506020820151613cb66020850182613c48565b506040820151613cc96040850182613c48565b50505050565b5f613cda8383613c8f565b60608301905092915050565b5f602082019050919050565b5f613cfc82613c66565b613d068185613c70565b9350613d1183613c80565b805f5b83811015613d41578151613d288882613ccf565b9750613d3383613ce6565b925050600181019050613d14565b5085935050505092915050565b5f60a083015f830151613d635f860182613c48565b506020830151613d766020860182613c48565b506040830151613d896040860182613c57565b5060608301518482036060860152613da18282613807565b91505060808301518482036080860152613dbb8282613cf2565b9150508091505092915050565b5f6020820190508181035f830152613de08184613d4e565b905092915050565b5f805f60a08486031215613dff57613dfe613664565b5b5f84013567ffffffffffffffff811115613e1c57613e1b613668565b5b613e2886828701613a85565b93509350506020613e3b86828701613ade565b9150509250925092565b5f8083601f840112613e5a57613e596138c6565b5b8235905067ffffffffffffffff811115613e7757613e76613a7d565b5b602083019150836001820283011115613e9357613e92613a81565b5b9250929050565b5f8083601f840112613eaf57613eae6138c6565b5b8235905067ffffffffffffffff811115613ecc57613ecb613a7d565b5b602083019150836060820283011115613ee857613ee7613a81565b5b9250929050565b613ef881613c35565b8114613f02575f80fd5b50565b5f81359050613f1381613eef565b92915050565b5f805f805f60608688031215613f3257613f31613664565b5b5f86013567ffffffffffffffff811115613f4f57613f4e613668565b5b613f5b88828901613e45565b9550955050602086013567ffffffffffffffff811115613f7e57613f7d613668565b5b613f8a88828901613e9a565b93509350506040613f9d88828901613f05565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f608083015f830151613fe85f8601826137e8565b506020830151613ffb60208601826137e8565b50604083015184820360408601526140138282613807565b9150506060830151848203606086015261402d8282613807565b9150508091505092915050565b5f6140458383613fd3565b905092915050565b5f602082019050919050565b5f61406382613faa565b61406d8185613fb4565b93508360208202850161407f85613fc4565b805f5b858110156140ba578484038952815161409b858261403a565b94506140a68361404d565b925060208a01995050600181019050614082565b50829750879550505050505092915050565b5f6020820190508181035f8301526140e48184614059565b905092915050565b5f81905092915050565b5f614100826135b1565b61410a81856140ec565b935061411a8185602086016135cb565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61415a6002836140ec565b915061416582614126565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6141a46001836140ec565b91506141af82614170565b600182019050919050565b5f6141c582876140f6565b91506141d08261414e565b91506141dc82866140f6565b91506141e782614198565b91506141f382856140f6565b91506141fe82614198565b915061420a82846140f6565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061425c57607f821691505b60208210810361426f5761426e614218565b5b50919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6142ac8261366c565b91506142b78361366c565b92508282019050808211156142cf576142ce614275565b5b92915050565b5f6142df8261366c565b91506142ea8361366c565b925082820390508181111561430257614301614275565b5b92915050565b61431181613c35565b82525050565b5f60208201905061432a5f830184614308565b92915050565b5f8151905061433e81613755565b92915050565b5f6020828403121561435957614358613664565b5b5f61436684828501614330565b91505092915050565b61437881613744565b82525050565b5f6020820190506143915f83018461436f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f602082840312156143d9576143d8613664565b5b5f6143e684828501613f05565b91505092915050565b5f6060820190506144025f830186614308565b61440f6020830185614308565b61441c6040830184614308565b949350505050565b5f61442e8261366c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036144605761445f614275565b5b600182019050919050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026144d17fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82614496565b6144db8683614496565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61451661451161450c8461366c565b6144f3565b61366c565b9050919050565b5f819050919050565b61452f836144fc565b61454361453b8261451d565b8484546144a2565b825550505050565b5f90565b61455761454b565b614562818484614526565b505050565b5b818110156145855761457a5f8261454f565b600181019050614568565b5050565b601f8211156145ca5761459b81614475565b6145a484614487565b810160208510156145b3578190505b6145c76145bf85614487565b830182614567565b50505b505050565b5f82821c905092915050565b5f6145ea5f19846008026145cf565b1980831691505092915050565b5f61460283836145db565b9150826002028217905092915050565b61461c838361446b565b67ffffffffffffffff811115614635576146346138ce565b5b61463f8254614245565b61464a828285614589565b5f601f831160018114614677575f8415614665578287013590505b61466f85826145f7565b8655506146d6565b601f19841661468586614475565b5f5b828110156146ac57848901358255600182019150602085019450602081019050614687565b868310156146c957848901356146c5601f8916826145db565b8355505b6001600288020188555050505b50505050505050565b5f81356146eb81613eef565b80915050919050565b5f815f1b9050919050565b5f67ffffffffffffffff614712846146f4565b9350801983169250808416831791505092915050565b5f61474261473d61473884613c35565b6144f3565b613c35565b9050919050565b5f819050919050565b61475b82614728565b61476e61476782614749565b83546146ff565b8255505050565b5f8160401b9050919050565b5f6fffffffffffffffff000000000000000061479c84614775565b9350801983169250808416831791505092915050565b6147bb82614728565b6147ce6147c782614749565b8354614781565b8255505050565b5f8160801b9050919050565b5f77ffffffffffffffff00000000000000000000000000000000614804846147d5565b9350801983169250808416831791505092915050565b61482382614728565b61483661482f82614749565b83546147e1565b8255505050565b5f81015f83018061484d816146df565b90506148598184614752565b5050505f8101602083018061486d816146df565b905061487981846147b2565b5050505f8101604083018061488d816146df565b9050614899818461481a565b5050505050565b6148aa828261483d565b5050565b5f6148b983856135bb565b93506148c6838584613976565b6148cf836135f3565b840190509392505050565b5f82825260208201905092915050565b5f819050919050565b5f6149016020840184613f05565b905092915050565b606082016149195f8301836148f3565b6149255f850182613c48565b5061493360208301836148f3565b6149406020850182613c48565b5061494e60408301836148f3565b61495b6040850182613c48565b50505050565b5f61496c8383614909565b60608301905092915050565b5f82905092915050565b5f606082019050919050565b5f61499983856148da565b93506149a4826148ea565b805f5b858110156149dc576149b98284614978565b6149c38882614961565b97506149ce83614982565b9250506001810190506149a7565b5085925050509392505050565b5f6060820190508181035f830152614a028187896148ae565b90508181036020830152614a1781858761498e565b9050614a266040830184614308565b9695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b614a6681613a4c565b8114614a70575f80fd5b50565b5f81519050614a8181614a5d565b92915050565b5f60208284031215614a9c57614a9b613664565b5b5f614aa984828501614a73565b91505092915050565b5f604082019050614ac55f8301856136fd565b614ad260208301846136fd565b9392505050565b5f80fd5b5f80fd5b5f80fd5b5f82356001608003833603038112614b0057614aff614ad9565b5b80830191505092915050565b5f8135614b1881613755565b80915050919050565b5f73ffffffffffffffffffffffffffffffffffffffff614b40846146f4565b9350801983169250808416831791505092915050565b5f614b70614b6b614b6684613725565b6144f3565b613725565b9050919050565b5f614b8182614b56565b9050919050565b5f614b9282614b77565b9050919050565b5f819050919050565b614bab82614b88565b614bbe614bb782614b99565b8354614b21565b8255505050565b5f8083356001602003843603038112614be157614be0614ad9565b5b80840192508235915067ffffffffffffffff821115614c0357614c02614add565b5b602083019250600182023603831315614c1f57614c1e614ae1565b5b509250929050565b614c32838383614612565b505050565b5f81015f830180614c4781614b0c565b9050614c538184614ba2565b505050600181016020830180614c6881614b0c565b9050614c748184614ba2565b5050506002810160408301614c898185614bc5565b614c94818386614c27565b505050506003810160608301614caa8185614bc5565b614cb5818386614c27565b505050505050565b614cc78282614c37565b5050565b5f819050919050565b5f614ce2602084018461376b565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112614d1257614d11614cf2565b5b83810192508235915060208301925067ffffffffffffffff821115614d3a57614d39614cea565b5b600182023603831315614d5057614d4f614cee565b5b509250929050565b5f614d6383856137f7565b9350614d70838584613976565b614d79836135f3565b840190509392505050565b5f60808301614d955f840184614cd4565b614da15f8601826137e8565b50614daf6020840184614cd4565b614dbc60208601826137e8565b50614dca6040840184614cf6565b8583036040870152614ddd838284614d58565b92505050614dee6060840184614cf6565b8583036060870152614e01838284614d58565b925050508091505092915050565b5f614e1a8383614d84565b905092915050565b5f82356001608003833603038112614e3d57614e3c614cf2565b5b82810191505092915050565b5f602082019050919050565b5f614e608385613fb4565b935083602084028501614e7284614ccb565b805f5b87811015614eb5578484038952614e8c8284614e22565b614e968582614e0f565b9450614ea183614e49565b925060208a01995050600181019050614e75565b50829750879450505050509392505050565b5f614ed5602084018461368b565b905092915050565b614ee68161366c565b82525050565b60808201614efc5f830183614ec7565b614f085f850182614edd565b50614f166020830183614ec7565b614f236020850182614edd565b50614f316040830183614ec7565b614f3e6040850182614edd565b50614f4c6060830183614ec7565b614f596060850182614edd565b50505050565b5f60a0820190508181035f830152614f78818587614e55565b9050614f876020830184614eec565b949350505050565b5f81519050919050565b5f81905092915050565b5f614fad82614f8f565b614fb78185614f99565b9350614fc78185602086016135cb565b80840191505092915050565b5f614fde8284614fa3565b915081905092915050565b5f6060820190508181035f8301526150018186613603565b905061501060208301856136fd565b61501d60408301846136fd565b94935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80QaP%b\0\x01\xEB_9_\x81\x81a$\x1E\x01R\x81\x81a$s\x01Ra'\x15\x01RaP%_\xF3\xFE`\x80`@R`\x046\x10a\x01\xD7W_5`\xE0\x1C\x80c~\xAA\xC8\xF2\x11a\x01\x01W\x80c\xBF\x9B\x16\xC8\x11a\0\x94W\x80c\xD7@\xE4\x02\x11a\0cW\x80c\xD7@\xE4\x02\x14a\x06\xF9W\x80c\xD8\xF89+\x14a\x07!W\x80c\xF7l\xA5w\x14a\x07IW\x80c\xF9\xC6p\xC3\x14a\x07qWa\x01\xD7V[\x80c\xBF\x9B\x16\xC8\x14a\x06/W\x80c\xC0\xAEd\xF7\x14a\x06kW\x80c\xC2\xB4)\x86\x14a\x06\x93W\x80c\xC3\xAA\xAAZ\x14a\x06\xBDWa\x01\xD7V[\x80c\xA9,u\xCB\x11a\0\xD0W\x80c\xA9,u\xCB\x14a\x05\x9DW\x80c\xAD<\xB1\xCC\x14a\x05\xC5W\x80c\xB4r+\xC4\x14a\x05\xEFW\x80c\xBA\xC2+\xB8\x14a\x06\x19Wa\x01\xD7V[\x80c~\xAA\xC8\xF2\x14a\x04\xD1W\x80c\x94G\xCF\xD4\x14a\x04\xFBW\x80c\x97o>\xB9\x14a\x057W\x80c\x9Ax`\xE0\x14a\x05aWa\x01\xD7V[\x80c1\xFFA\xC8\x11a\x01yW\x80cO\x1E\xF2\x86\x11a\x01HW\x80cO\x1E\xF2\x86\x14a\x04'W\x80cR\xD1\x90-\x14a\x04CW\x80cUn\xCA\xFA\x14a\x04mW\x80c[\xFFv\xD9\x14a\x04\x95Wa\x01\xD7V[\x80c1\xFFA\xC8\x14a\x037W\x80cA\xAD\x06\x9C\x14a\x03sW\x80cF\xC5\xBB\xBD\x14a\x03\xAFW\x80cG\xE8\"\x95\x14a\x03\xEBWa\x01\xD7V[\x80c =\x01\x14\x11a\x01\xB5W\x80c =\x01\x14\x14a\x02kW\x80c&\xCF]\xEF\x14a\x02\xA7W\x80c(\x1E\x8B\xFE\x14a\x02\xD1W\x80c*8\x89\x98\x14a\x03\rWa\x01\xD7V[\x80c\r\x8En,\x14a\x01\xDBW\x80c\x0E\x18\x87\xC9\x14a\x02\x05W\x80c\x17\n)\x81\x14a\x02AW[_\x80\xFD[4\x80\x15a\x01\xE6W_\x80\xFD[Pa\x01\xEFa\x07\xADV[`@Qa\x01\xFC\x91\x90a6;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x10W_\x80\xFD[Pa\x02+`\x04\x806\x03\x81\x01\x90a\x02&\x91\x90a6\x9FV[a\x08(V[`@Qa\x028\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02LW_\x80\xFD[Pa\x02Ua\x089V[`@Qa\x02b\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02vW_\x80\xFD[Pa\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a7\x7FV[a\x08KV[`@Qa\x02\x9E\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB2W_\x80\xFD[Pa\x02\xBBa\x08\xBDV[`@Qa\x02\xC8\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xDCW_\x80\xFD[Pa\x02\xF7`\x04\x806\x03\x81\x01\x90a\x02\xF2\x91\x90a6\x9FV[a\x08\xE6V[`@Qa\x03\x04\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x18W_\x80\xFD[Pa\x03!a\t\x12V[`@Qa\x03.\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03BW_\x80\xFD[Pa\x03]`\x04\x806\x03\x81\x01\x90a\x03X\x91\x90a7\xAAV[a\t;V[`@Qa\x03j\x91\x90a8\xA6V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03~W_\x80\xFD[Pa\x03\x99`\x04\x806\x03\x81\x01\x90a\x03\x94\x91\x90a6\x9FV[a\x0B}V[`@Qa\x03\xA6\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xBAW_\x80\xFD[Pa\x03\xD5`\x04\x806\x03\x81\x01\x90a\x03\xD0\x91\x90a7\xAAV[a\x0B\xA9V[`@Qa\x03\xE2\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xF6W_\x80\xFD[Pa\x04\x11`\x04\x806\x03\x81\x01\x90a\x04\x0C\x91\x90a6\x9FV[a\x0C\x1DV[`@Qa\x04\x1E\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[a\x04A`\x04\x806\x03\x81\x01\x90a\x04<\x91\x90a9\xF2V[a\x0CIV[\0[4\x80\x15a\x04NW_\x80\xFD[Pa\x04Wa\x0ChV[`@Qa\x04d\x91\x90a:dV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04xW_\x80\xFD[Pa\x04\x93`\x04\x806\x03\x81\x01\x90a\x04\x8E\x91\x90a:\xFCV[a\x0C\x99V[\0[4\x80\x15a\x04\xA0W_\x80\xFD[Pa\x04\xBB`\x04\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a6\x9FV[a\x0E\x9DV[`@Qa\x04\xC8\x91\x90a<\x15V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xDCW_\x80\xFD[Pa\x04\xE5a\x0FKV[`@Qa\x04\xF2\x91\x90a<\x15V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x06W_\x80\xFD[Pa\x05!`\x04\x806\x03\x81\x01\x90a\x05\x1C\x91\x90a7\xAAV[a\x0F\xF6V[`@Qa\x05.\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05BW_\x80\xFD[Pa\x05Ka\x10jV[`@Qa\x05X\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05lW_\x80\xFD[Pa\x05\x87`\x04\x806\x03\x81\x01\x90a\x05\x82\x91\x90a6\x9FV[a\x10{V[`@Qa\x05\x94\x91\x90a=\xC8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xA8W_\x80\xFD[Pa\x05\xC3`\x04\x806\x03\x81\x01\x90a\x05\xBE\x91\x90a=\xE8V[a\x12\xEEV[\0[4\x80\x15a\x05\xD0W_\x80\xFD[Pa\x05\xD9a\x13\xEFV[`@Qa\x05\xE6\x91\x90a6;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xFAW_\x80\xFD[Pa\x06\x03a\x14(V[`@Qa\x06\x10\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06$W_\x80\xFD[Pa\x06-a\x14QV[\0[4\x80\x15a\x06:W_\x80\xFD[Pa\x06U`\x04\x806\x03\x81\x01\x90a\x06P\x91\x90a6\x9FV[a\x15\x8BV[`@Qa\x06b\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06vW_\x80\xFD[Pa\x06\x91`\x04\x806\x03\x81\x01\x90a\x06\x8C\x91\x90a6\x9FV[a\x15\x9CV[\0[4\x80\x15a\x06\x9EW_\x80\xFD[Pa\x06\xA7a\x17\x84V[`@Qa\x06\xB4\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xC8W_\x80\xFD[Pa\x06\xE3`\x04\x806\x03\x81\x01\x90a\x06\xDE\x91\x90a6\x9FV[a\x17\xADV[`@Qa\x06\xF0\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x04W_\x80\xFD[Pa\x07\x1F`\x04\x806\x03\x81\x01\x90a\x07\x1A\x91\x90a6\x9FV[a\x17\xD9V[\0[4\x80\x15a\x07,W_\x80\xFD[Pa\x07G`\x04\x806\x03\x81\x01\x90a\x07B\x91\x90a=\xE8V[a\x19xV[\0[4\x80\x15a\x07TW_\x80\xFD[Pa\x07o`\x04\x806\x03\x81\x01\x90a\x07j\x91\x90a?\x19V[a\x1B\x1EV[\0[4\x80\x15a\x07|W_\x80\xFD[Pa\x07\x97`\x04\x806\x03\x81\x01\x90a\x07\x92\x91\x90a6\x9FV[a \x19V[`@Qa\x07\xA4\x91\x90a@\xCCV[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FProtocolConfig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x07\xEE_a\"aV[a\x07\xF8`\x01a\"aV[a\x08\x01_a\"aV[`@Q` \x01a\x08\x14\x94\x93\x92\x91\x90aA\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_a\x082\x82a#+V[\x90P\x91\x90PV[_a\x08Ba#\xA8V[`\x0B\x01T\x90P\x90V[_\x80a\x08Ua#\xA8V[\x90P\x80`\x03\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ _\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x08\xC7a#\xA8V[\x90P\x80`\t\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x08\xF0\x82a#\xCFV[a\x08\xF8a#\xA8V[`\x07\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_\x80a\t\x1Ca#\xA8V[\x90P\x80`\x06\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[a\tCa5\x1DV[a\tL\x83a#\xCFV[a\tTa#\xA8V[`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\ne\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x91\x90aBEV[\x80\x15a\n\xDCW\x80`\x1F\x10a\n\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xDCV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xBFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\n\xF5\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B!\x90aBEV[\x80\x15a\x0BlW\x80`\x1F\x10a\x0BCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BlV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BOW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x92\x91PPV[_a\x0B\x87\x82a#\xCFV[a\x0B\x8Fa#\xA8V[`\x08\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_a\x0B\xB3\x83a#\xCFV[a\x0B\xBBa#\xA8V[`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x0C'\x82a#\xCFV[a\x0C/a#\xA8V[`\t\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[a\x0CQa$\x1CV[a\x0CZ\x82a%\x02V[a\x0Cd\x82\x82a%\xF5V[PPV[_a\x0Cqa'\x13V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[`\x01a\x0C\xA3a'\x9AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xE4W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x0C\xEFa'\xBEV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\r7WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\rnW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\xF8`\x07\x90\x1Ba\r\xC5\x91\x90aB\xA2V[\x86\x10\x15a\x0E\tW\x85`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\0\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[_a\x0E\x12a#\xA8V[\x90P`\x01\x87a\x0E!\x91\x90aB\xD5V[\x81_\x01\x81\x90UP`\xF8`\t\x90\x1B\x81`\x0B\x01\x81\x90UPa\x0EA\x86\x86\x86a'\xE5V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0E\x8D\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[``a\x0E\xA8\x82a#\xCFV[a\x0E\xB0a#\xA8V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0F?W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\xF6W[PPPPP\x90P\x91\x90PV[``_a\x0FVa#\xA8V[\x90P\x80`\x05\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0F\xEBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0F\xA2W[PPPPP\x91PP\x90V[_a\x10\0\x83a#\xCFV[a\x10\x08a#\xA8V[`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x10sa#\xA8V[_\x01T\x90P\x90V[a\x10\x83a5oV[a\x10\x8C\x82a#+V[a\x10\xCDW\x81`@Q\x7F\x97\x97\xC3\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xC4\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[a\x10\xD5a#\xA8V[`\x0C\x01_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x15\x15\x81R` \x01`\x01\x82\x01\x80Ta\x11}\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xA9\x90aBEV[\x80\x15a\x11\xF4W\x80`\x1F\x10a\x11\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xF4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xD7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x12\xDFW\x83\x82\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12!V[PPPP\x81RPP\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13KW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13o\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\xDEW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xD5\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[a\x13\xE9\x83\x83\x83a'\xE5V[PPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x142a#\xA8V[\x90P\x80`\x08\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[`\x03_a\x14\\a'\xBEV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x14\xA4WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x14\xDBW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\xF8`\t\x90\x1Ba\x15.a#\xA8V[`\x0B\x01\x81\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x15\x7F\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xA1PPV[_a\x15\x95\x82a.1V[\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xF9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x1D\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\x8CW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x83\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[_a\x16\x95a#\xA8V[\x90P\x80_\x01T\x82\x03a\x16\xDEW\x81`@Q\x7FE\x95\xFC\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xD5\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[a\x16\xE7\x82a.1V[a\x17(W\x81`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x1F\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0`@Q`@Q\x80\x91\x03\x90\xA2PPV[_\x80a\x17\x8Ea#\xA8V[\x90P\x80`\x07\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x17\xB7\x82a#\xCFV[a\x17\xBFa#\xA8V[`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x186W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18Z\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xC9W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xC0\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[a\x18\xD2\x81a#+V[a\x19\x13W\x80`@Q\x7F\x97\x97\xC3\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\n\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[`\x01a\x19\x1Da#\xA8V[`\x0C\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x01`\x10a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x7Fn\xD5\xF2\xC7Y\xF9\xFA%\xB4xQ\x1D\xAE*\xA7h\xDC\x99>\x9D\x04\xAB\x15\xF9\xC2Q\x9F\x07\\G%\xD3`@Q`@Q\x80\x91\x03\x90\xA2PV[`\x01a\x19\x82a'\x9AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\xC3W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x19\xCEa'\xBEV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x1A\x16WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1AMW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x1A\x9Ba#\xA8V[\x90P`\xF8`\x07\x90\x1B\x81_\x01\x81\x90UP`\xF8`\t\x90\x1B\x81`\x0B\x01\x81\x90UPa\x1A\xC3\x86\x86\x86a'\xE5V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x1B\x0F\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xA1PPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B{W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x9F\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1C\x0EW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x05\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x1CJW`@Q\x7F\xB5H\x91G\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x83\x90P\x03a\x1C\x86W`@Q\x7F\xBEPPD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1C\xC9W`@Q\x7F\x17\xD3\xE9H\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x83\x90P\x81\x10\x15a\x1E\xC5W6\x84\x84\x83\x81\x81\x10a\x1C\xEAWa\x1C\xE9aC\x97V[[\x90P``\x02\x01\x90P_\x81_\x01` \x81\x01\x90a\x1D\x05\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1DFW`@Q\x7F\xC8H\x85\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01` \x81\x01\x90a\x1DY\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01` \x81\x01\x90a\x1Dv\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\xFCW\x80_\x01` \x81\x01\x90a\x1D\x98\x91\x90aC\xC4V[\x81` \x01` \x81\x01\x90a\x1D\xAB\x91\x90aC\xC4V[\x82`@\x01` \x81\x01\x90a\x1D\xBE\x91\x90aC\xC4V[`@Q\x7F\xF2\x19\xDC\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xF3\x93\x92\x91\x90aC\xEFV[`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x1E\xB6W\x81_\x01` \x81\x01\x90a\x1E\x18\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x86\x83\x81\x81\x10a\x1E5Wa\x1E4aC\x97V[[\x90P``\x02\x01_\x01` \x81\x01\x90a\x1EL\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1E\xA9W\x81_\x01` \x81\x01\x90a\x1Em\x91\x90aC\xC4V[`@Q\x7Flg\xE4p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\xA0\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x1D\xFEV[PP\x80\x80`\x01\x01\x91PPa\x1C\xCBV[P_a\x1E\xCFa#\xA8V[\x90P_\x81`\x0B\x01_\x81Ta\x1E\xE2\x90aD$V[\x91\x90P\x81\x90U\x90P_\x82`\x0C\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x87\x87\x82`\x01\x01\x91\x82a\x1F\x13\x92\x91\x90aF\x12V[P\x83\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPC\x81_\x01`\x08a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_[\x86\x86\x90P\x81\x10\x15a\x1F\xCEW\x81`\x02\x01\x87\x87\x83\x81\x81\x10a\x1F\x8BWa\x1F\x8AaC\x97V[[\x90P``\x02\x01\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x1F\xBF\x91\x90aH\xA0V[PP\x80\x80`\x01\x01\x91PPa\x1FiV[P\x81\x7FY]\x10\x94\x9F\xCF\x82-\xE1~\x89\xEB\xC3\x02Vn\xD1P\x17\x1F\xF4\x14\xFE\x14\xD9+x\xA6\xD3\xAE\xCC\xE8\x89\x89\x89\x89\x89`@Qa \x07\x95\x94\x93\x92\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[``a $\x82a#\xCFV[a ,a#\xA8V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\"VW\x83\x82\x90_R` _ \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta!7\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!c\x90aBEV[\x80\x15a!\xAEW\x80`\x1F\x10a!\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta!\xC7\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\xF3\x90aBEV[\x80\x15a\">W\x80`\x1F\x10a\"\x15Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\">V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"!W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a ]V[PPPP\x90P\x91\x90PV[``_`\x01a\"o\x84a.\xB4V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x8DWa\"\x8Ca8\xCEV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\"\xBFW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a# W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#\x15Wa#\x14aJ0V[[\x04\x94P_\x85\x03a\"\xCCW[\x81\x93PPPP\x91\x90PV[_\x80a#5a#\xA8V[\x90P_\x81`\x0C\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\xF8`\t\x90\x1Ba#_\x91\x90aB\xA2V[\x84\x10\x15\x80\x15a#rWP\x81`\x0B\x01T\x84\x11\x15[\x80\x15a#\x85WP_\x81`\x02\x01\x80T\x90P\x14\x15[\x80\x15a#\x9FWP\x80_\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x92PPP\x91\x90PV[_\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90P\x90V[a#\xD8\x81a.1V[a$\x19W\x80`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x10\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a$\xC9WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a$\xB0a0\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a%\0W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%_W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x83\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a%\xF2W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xE9\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a&]WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&Z\x91\x90aJ\x87V[`\x01[a&\x9EW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x95\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a'\x04W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xFB\x91\x90a:dV[`@Q\x80\x91\x03\x90\xFD[a'\x0E\x83\x83a0XV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'\x98W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a'\xA3a'\xBEV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_\x80\x84\x84\x90P\x03a(\"W`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x84\x84\x90P\x11\x15a(uW\x83\x83\x90P`\xFF\x80\x16`@Q\x7F\x16\xA7'x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(l\x92\x91\x90aJ\xB2V[`@Q\x80\x91\x03\x90\xFD[a(\x82\x82\x85\x85\x90Pa0\xCAV[_a(\x8Ba#\xA8V[\x90P\x80_\x01_\x81Ta(\x9C\x90aD$V[\x91\x90P\x81\x90U\x91P_[\x85\x85\x90P\x81\x10\x15a-}W6\x86\x86\x83\x81\x81\x10a(\xC5Wa(\xC4aC\x97V[[\x90P` \x02\x81\x01\x90a(\xD7\x91\x90aJ\xE5V[\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01` \x81\x01\x90a)\x02\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a)OW`@Q\x7F\x84f\x80J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01` \x81\x01\x90a)y\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a)\xC6W`@Q\x7F-\xEC\xCFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82_\x01` \x81\x01\x90a)\xEC\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a*\x85W\x80_\x01` \x81\x01\x90a*I\x91\x90a7\x7FV[`@Q\x7F\xD1\x8CO\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*|\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x82`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82` \x01` \x81\x01\x90a*\xAC\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a+FW\x80` \x01` \x81\x01\x90a+\n\x91\x90a7\x7FV[`@Q\x7F\xF5\x1A\xF6\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+=\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x04\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a+\x8C\x91\x90aL\xBDV[PP`\x01\x83`\x02\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a+\xB6\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x83`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83` \x01` \x81\x01\x90a,.\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x83`\x04\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a,\xA4\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a,\xE9\x91\x90aL\xBDV[\x90PP\x82`\x05\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81` \x01` \x81\x01\x90a-\x12\x91\x90a7\x7FV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP\x80\x80`\x01\x01\x91PPa(\xA6V[P\x82_\x015\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82` \x015\x81`\x07\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`@\x015\x81`\x08\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82``\x015\x81`\t\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x7F\xE5)j\x81\x84\xD1\x9A_\xD2EHt\x9E\xA3\xC45\xB6\x9A\xD2o\x12\xCA\n\xFA\x1E\x8E\xFE\xF5\x926\x8B\xF2\x86\x86\x86`@Qa.!\x93\x92\x91\x90aO_V[`@Q\x80\x91\x03\x90\xA2P\x93\x92PPPV[_\x80a.;a#\xA8V[\x90P`\x01`\xF8`\x07\x90\x1Ba.O\x91\x90aB\xA2V[\x83\x10\x15\x80\x15a.aWP\x80_\x01T\x83\x11\x15[\x80\x15a.\x83WP_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15[\x80\x15a.\xACWP\x80`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x91PP\x91\x90PV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a/\x10Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a/\x06Wa/\x05aJ0V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a/MWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a/CWa/BaJ0V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a/|Wf#\x86\xF2o\xC1\0\0\x83\x81a/rWa/qaJ0V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a/\xA5Wc\x05\xF5\xE1\0\x83\x81a/\x9BWa/\x9AaJ0V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a/\xCAWa'\x10\x83\x81a/\xC0Wa/\xBFaJ0V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a/\xEDW`d\x83\x81a/\xE3Wa/\xE2aJ0V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a/\xFCW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba1\xDDV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a0a\x82a1\xE6V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a0\xBDWa0\xB7\x82\x82a2\xAFV[Pa0\xC6V[a0\xC5a3/V[[PPV[a1\r`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83_\x015\x83a3kV[a1Q`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83` \x015\x83a3kV[a1\x95`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83`@\x015\x83a3kV[a1\xD9`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83``\x015\x83a3kV[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a2AW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a28\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x80a2m\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba1\xDDV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa2\xD8\x91\x90aO\xD3V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a3\x10W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a3\x15V[``\x91P[P\x91P\x91Pa3%\x85\x83\x83a4LV[\x92PPP\x92\x91PPV[_4\x11\x15a3iW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x82\x03a3\xAFW\x82`@Q\x7F6\xBF\xB6\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\xA6\x91\x90a6;V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x82\x11\x15a3\xFEW\x82\x82`\xFF\x80\x16`@Q\x7F\"\xBAR\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\xF5\x93\x92\x91\x90aO\xE9V[`@Q\x80\x91\x03\x90\xFD[\x80\x82\x11\x15a4GW\x82\x82\x82`@Q\x7F\xCA\xA8\x14\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4>\x93\x92\x91\x90aO\xE9V[`@Q\x80\x91\x03\x90\xFD[PPPV[``\x82a4aWa4\\\x82a4\xD9V[a4\xD1V[_\x82Q\x14\x80\x15a4\x87WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a4\xC9W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xC0\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa4\xD2V[[\x93\x92PPPV[_\x81Q\x11\x15a4\xEBW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a5\xE8W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa5\xCDV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a6\r\x82a5\xB1V[a6\x17\x81\x85a5\xBBV[\x93Pa6'\x81\x85` \x86\x01a5\xCBV[a60\x81a5\xF3V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra6S\x81\x84a6\x03V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a6~\x81a6lV[\x81\x14a6\x88W_\x80\xFD[PV[_\x815\x90Pa6\x99\x81a6uV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a6\xB4Wa6\xB3a6dV[[_a6\xC1\x84\x82\x85\x01a6\x8BV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a6\xDE\x81a6\xCAV[\x82RPPV[_` \x82\x01\x90Pa6\xF7_\x83\x01\x84a6\xD5V[\x92\x91PPV[a7\x06\x81a6lV[\x82RPPV[_` \x82\x01\x90Pa7\x1F_\x83\x01\x84a6\xFDV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a7N\x82a7%V[\x90P\x91\x90PV[a7^\x81a7DV[\x81\x14a7hW_\x80\xFD[PV[_\x815\x90Pa7y\x81a7UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7\x94Wa7\x93a6dV[[_a7\xA1\x84\x82\x85\x01a7kV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a7\xC0Wa7\xBFa6dV[[_a7\xCD\x85\x82\x86\x01a6\x8BV[\x92PP` a7\xDE\x85\x82\x86\x01a7kV[\x91PP\x92P\x92\x90PV[a7\xF1\x81a7DV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a8\x11\x82a5\xB1V[a8\x1B\x81\x85a7\xF7V[\x93Pa8+\x81\x85` \x86\x01a5\xCBV[a84\x81a5\xF3V[\x84\x01\x91PP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa8T_\x86\x01\x82a7\xE8V[P` \x83\x01Qa8g` \x86\x01\x82a7\xE8V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra8\x7F\x82\x82a8\x07V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra8\x99\x82\x82a8\x07V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xBE\x81\x84a8?V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a9\x04\x82a5\xF3V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a9#Wa9\"a8\xCEV[[\x80`@RPPPV[_a95a6[V[\x90Pa9A\x82\x82a8\xFBV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a9`Wa9_a8\xCEV[[a9i\x82a5\xF3V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a9\x96a9\x91\x84a9FV[a9,V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a9\xB2Wa9\xB1a8\xCAV[[a9\xBD\x84\x82\x85a9vV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a9\xD9Wa9\xD8a8\xC6V[[\x815a9\xE9\x84\x82` \x86\x01a9\x84V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a:\x08Wa:\x07a6dV[[_a:\x15\x85\x82\x86\x01a7kV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:6Wa:5a6hV[[a:B\x85\x82\x86\x01a9\xC5V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a:^\x81a:LV[\x82RPPV[_` \x82\x01\x90Pa:w_\x83\x01\x84a:UV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a:\x9AWa:\x99a8\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xB7Wa:\xB6a:}V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a:\xD3Wa:\xD2a:\x81V[[\x92P\x92\x90PV[_\x80\xFD[_`\x80\x82\x84\x03\x12\x15a:\xF3Wa:\xF2a:\xDAV[[\x81\x90P\x92\x91PPV[_\x80_\x80`\xC0\x85\x87\x03\x12\x15a;\x14Wa;\x13a6dV[[_a;!\x87\x82\x88\x01a6\x8BV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;BWa;Aa6hV[[a;N\x87\x82\x88\x01a:\x85V[\x93P\x93PP`@a;a\x87\x82\x88\x01a:\xDEV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a;\xA1\x83\x83a7\xE8V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a;\xC3\x82a;mV[a;\xCD\x81\x85a;wV[\x93Pa;\xD8\x83a;\x87V[\x80_[\x83\x81\x10\x15a<\x08W\x81Qa;\xEF\x88\x82a;\x96V[\x97Pa;\xFA\x83a;\xADV[\x92PP`\x01\x81\x01\x90Pa;\xDBV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<-\x81\x84a;\xB9V[\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a<Q\x81a<5V[\x82RPPV[a<`\x81a6\xCAV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[``\x82\x01_\x82\x01Qa<\xA3_\x85\x01\x82a<HV[P` \x82\x01Qa<\xB6` \x85\x01\x82a<HV[P`@\x82\x01Qa<\xC9`@\x85\x01\x82a<HV[PPPPV[_a<\xDA\x83\x83a<\x8FV[``\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a<\xFC\x82a<fV[a=\x06\x81\x85a<pV[\x93Pa=\x11\x83a<\x80V[\x80_[\x83\x81\x10\x15a=AW\x81Qa=(\x88\x82a<\xCFV[\x97Pa=3\x83a<\xE6V[\x92PP`\x01\x81\x01\x90Pa=\x14V[P\x85\x93PPPP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa=c_\x86\x01\x82a<HV[P` \x83\x01Qa=v` \x86\x01\x82a<HV[P`@\x83\x01Qa=\x89`@\x86\x01\x82a<WV[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra=\xA1\x82\x82a8\x07V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra=\xBB\x82\x82a<\xF2V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=\xE0\x81\x84a=NV[\x90P\x92\x91PPV[_\x80_`\xA0\x84\x86\x03\x12\x15a=\xFFWa=\xFEa6dV[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x1CWa>\x1Ba6hV[[a>(\x86\x82\x87\x01a:\x85V[\x93P\x93PP` a>;\x86\x82\x87\x01a:\xDEV[\x91PP\x92P\x92P\x92V[_\x80\x83`\x1F\x84\x01\x12a>ZWa>Ya8\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>wWa>va:}V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a>\x93Wa>\x92a:\x81V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a>\xAFWa>\xAEa8\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xCCWa>\xCBa:}V[[` \x83\x01\x91P\x83``\x82\x02\x83\x01\x11\x15a>\xE8Wa>\xE7a:\x81V[[\x92P\x92\x90PV[a>\xF8\x81a<5V[\x81\x14a?\x02W_\x80\xFD[PV[_\x815\x90Pa?\x13\x81a>\xEFV[\x92\x91PPV[_\x80_\x80_``\x86\x88\x03\x12\x15a?2Wa?1a6dV[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?OWa?Na6hV[[a?[\x88\x82\x89\x01a>EV[\x95P\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?~Wa?}a6hV[[a?\x8A\x88\x82\x89\x01a>\x9AV[\x93P\x93PP`@a?\x9D\x88\x82\x89\x01a?\x05V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`\x80\x83\x01_\x83\x01Qa?\xE8_\x86\x01\x82a7\xE8V[P` \x83\x01Qa?\xFB` \x86\x01\x82a7\xE8V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra@\x13\x82\x82a8\x07V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra@-\x82\x82a8\x07V[\x91PP\x80\x91PP\x92\x91PPV[_a@E\x83\x83a?\xD3V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@c\x82a?\xAAV[a@m\x81\x85a?\xB4V[\x93P\x83` \x82\x02\x85\x01a@\x7F\x85a?\xC4V[\x80_[\x85\x81\x10\x15a@\xBAW\x84\x84\x03\x89R\x81Qa@\x9B\x85\x82a@:V[\x94Pa@\xA6\x83a@MV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa@\x82V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\xE4\x81\x84a@YV[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aA\0\x82a5\xB1V[aA\n\x81\x85a@\xECV[\x93PaA\x1A\x81\x85` \x86\x01a5\xCBV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aAZ`\x02\x83a@\xECV[\x91PaAe\x82aA&V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aA\xA4`\x01\x83a@\xECV[\x91PaA\xAF\x82aApV[`\x01\x82\x01\x90P\x91\x90PV[_aA\xC5\x82\x87a@\xF6V[\x91PaA\xD0\x82aANV[\x91PaA\xDC\x82\x86a@\xF6V[\x91PaA\xE7\x82aA\x98V[\x91PaA\xF3\x82\x85a@\xF6V[\x91PaA\xFE\x82aA\x98V[\x91PaB\n\x82\x84a@\xF6V[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aB\\W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aBoWaBnaB\x18V[[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aB\xAC\x82a6lV[\x91PaB\xB7\x83a6lV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aB\xCFWaB\xCEaBuV[[\x92\x91PPV[_aB\xDF\x82a6lV[\x91PaB\xEA\x83a6lV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aC\x02WaC\x01aBuV[[\x92\x91PPV[aC\x11\x81a<5V[\x82RPPV[_` \x82\x01\x90PaC*_\x83\x01\x84aC\x08V[\x92\x91PPV[_\x81Q\x90PaC>\x81a7UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aCYWaCXa6dV[[_aCf\x84\x82\x85\x01aC0V[\x91PP\x92\x91PPV[aCx\x81a7DV[\x82RPPV[_` \x82\x01\x90PaC\x91_\x83\x01\x84aCoV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aC\xD9WaC\xD8a6dV[[_aC\xE6\x84\x82\x85\x01a?\x05V[\x91PP\x92\x91PPV[_``\x82\x01\x90PaD\x02_\x83\x01\x86aC\x08V[aD\x0F` \x83\x01\x85aC\x08V[aD\x1C`@\x83\x01\x84aC\x08V[\x94\x93PPPPV[_aD.\x82a6lV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aD`WaD_aBuV[[`\x01\x82\x01\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aD\xD1\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aD\x96V[aD\xDB\x86\x83aD\x96V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aE\x16aE\x11aE\x0C\x84a6lV[aD\xF3V[a6lV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aE/\x83aD\xFCV[aECaE;\x82aE\x1DV[\x84\x84TaD\xA2V[\x82UPPPPV[_\x90V[aEWaEKV[aEb\x81\x84\x84aE&V[PPPV[[\x81\x81\x10\x15aE\x85WaEz_\x82aEOV[`\x01\x81\x01\x90PaEhV[PPV[`\x1F\x82\x11\x15aE\xCAWaE\x9B\x81aDuV[aE\xA4\x84aD\x87V[\x81\x01` \x85\x10\x15aE\xB3W\x81\x90P[aE\xC7aE\xBF\x85aD\x87V[\x83\x01\x82aEgV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aE\xEA_\x19\x84`\x08\x02aE\xCFV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aF\x02\x83\x83aE\xDBV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aF\x1C\x83\x83aDkV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF5WaF4a8\xCEV[[aF?\x82TaBEV[aFJ\x82\x82\x85aE\x89V[_`\x1F\x83\x11`\x01\x81\x14aFwW_\x84\x15aFeW\x82\x87\x015\x90P[aFo\x85\x82aE\xF7V[\x86UPaF\xD6V[`\x1F\x19\x84\x16aF\x85\x86aDuV[_[\x82\x81\x10\x15aF\xACW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaF\x87V[\x86\x83\x10\x15aF\xC9W\x84\x89\x015aF\xC5`\x1F\x89\x16\x82aE\xDBV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x815aF\xEB\x81a>\xEFV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaG\x12\x84aF\xF4V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aGBaG=aG8\x84a<5V[aD\xF3V[a<5V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aG[\x82aG(V[aGnaGg\x82aGIV[\x83TaF\xFFV[\x82UPPPV[_\x81`@\x1B\x90P\x91\x90PV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0aG\x9C\x84aGuV[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[aG\xBB\x82aG(V[aG\xCEaG\xC7\x82aGIV[\x83TaG\x81V[\x82UPPPV[_\x81`\x80\x1B\x90P\x91\x90PV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aH\x04\x84aG\xD5V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[aH#\x82aG(V[aH6aH/\x82aGIV[\x83TaG\xE1V[\x82UPPPV[_\x81\x01_\x83\x01\x80aHM\x81aF\xDFV[\x90PaHY\x81\x84aGRV[PPP_\x81\x01` \x83\x01\x80aHm\x81aF\xDFV[\x90PaHy\x81\x84aG\xB2V[PPP_\x81\x01`@\x83\x01\x80aH\x8D\x81aF\xDFV[\x90PaH\x99\x81\x84aH\x1AV[PPPPPV[aH\xAA\x82\x82aH=V[PPV[_aH\xB9\x83\x85a5\xBBV[\x93PaH\xC6\x83\x85\x84a9vV[aH\xCF\x83a5\xF3V[\x84\x01\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aI\x01` \x84\x01\x84a?\x05V[\x90P\x92\x91PPV[``\x82\x01aI\x19_\x83\x01\x83aH\xF3V[aI%_\x85\x01\x82a<HV[PaI3` \x83\x01\x83aH\xF3V[aI@` \x85\x01\x82a<HV[PaIN`@\x83\x01\x83aH\xF3V[aI[`@\x85\x01\x82a<HV[PPPPV[_aIl\x83\x83aI\tV[``\x83\x01\x90P\x92\x91PPV[_\x82\x90P\x92\x91PPV[_``\x82\x01\x90P\x91\x90PV[_aI\x99\x83\x85aH\xDAV[\x93PaI\xA4\x82aH\xEAV[\x80_[\x85\x81\x10\x15aI\xDCWaI\xB9\x82\x84aIxV[aI\xC3\x88\x82aIaV[\x97PaI\xCE\x83aI\x82V[\x92PP`\x01\x81\x01\x90PaI\xA7V[P\x85\x92PPP\x93\x92PPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaJ\x02\x81\x87\x89aH\xAEV[\x90P\x81\x81\x03` \x83\x01RaJ\x17\x81\x85\x87aI\x8EV[\x90PaJ&`@\x83\x01\x84aC\x08V[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[aJf\x81a:LV[\x81\x14aJpW_\x80\xFD[PV[_\x81Q\x90PaJ\x81\x81aJ]V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aJ\x9CWaJ\x9Ba6dV[[_aJ\xA9\x84\x82\x85\x01aJsV[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaJ\xC5_\x83\x01\x85a6\xFDV[aJ\xD2` \x83\x01\x84a6\xFDV[\x93\x92PPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12aK\0WaJ\xFFaJ\xD9V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815aK\x18\x81a7UV[\x80\x91PP\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaK@\x84aF\xF4V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aKpaKkaKf\x84a7%V[aD\xF3V[a7%V[\x90P\x91\x90PV[_aK\x81\x82aKVV[\x90P\x91\x90PV[_aK\x92\x82aKwV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aK\xAB\x82aK\x88V[aK\xBEaK\xB7\x82aK\x99V[\x83TaK!V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aK\xE1WaK\xE0aJ\xD9V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aL\x03WaL\x02aJ\xDDV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aL\x1FWaL\x1EaJ\xE1V[[P\x92P\x92\x90PV[aL2\x83\x83\x83aF\x12V[PPPV[_\x81\x01_\x83\x01\x80aLG\x81aK\x0CV[\x90PaLS\x81\x84aK\xA2V[PPP`\x01\x81\x01` \x83\x01\x80aLh\x81aK\x0CV[\x90PaLt\x81\x84aK\xA2V[PPP`\x02\x81\x01`@\x83\x01aL\x89\x81\x85aK\xC5V[aL\x94\x81\x83\x86aL'V[PPPP`\x03\x81\x01``\x83\x01aL\xAA\x81\x85aK\xC5V[aL\xB5\x81\x83\x86aL'V[PPPPPPV[aL\xC7\x82\x82aL7V[PPV[_\x81\x90P\x91\x90PV[_aL\xE2` \x84\x01\x84a7kV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aM\x12WaM\x11aL\xF2V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aM:WaM9aL\xEAV[[`\x01\x82\x026\x03\x83\x13\x15aMPWaMOaL\xEEV[[P\x92P\x92\x90PV[_aMc\x83\x85a7\xF7V[\x93PaMp\x83\x85\x84a9vV[aMy\x83a5\xF3V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x83\x01aM\x95_\x84\x01\x84aL\xD4V[aM\xA1_\x86\x01\x82a7\xE8V[PaM\xAF` \x84\x01\x84aL\xD4V[aM\xBC` \x86\x01\x82a7\xE8V[PaM\xCA`@\x84\x01\x84aL\xF6V[\x85\x83\x03`@\x87\x01RaM\xDD\x83\x82\x84aMXV[\x92PPPaM\xEE``\x84\x01\x84aL\xF6V[\x85\x83\x03``\x87\x01RaN\x01\x83\x82\x84aMXV[\x92PPP\x80\x91PP\x92\x91PPV[_aN\x1A\x83\x83aM\x84V[\x90P\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12aN=WaN<aL\xF2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aN`\x83\x85a?\xB4V[\x93P\x83` \x84\x02\x85\x01aNr\x84aL\xCBV[\x80_[\x87\x81\x10\x15aN\xB5W\x84\x84\x03\x89RaN\x8C\x82\x84aN\"V[aN\x96\x85\x82aN\x0FV[\x94PaN\xA1\x83aNIV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaNuV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_aN\xD5` \x84\x01\x84a6\x8BV[\x90P\x92\x91PPV[aN\xE6\x81a6lV[\x82RPPV[`\x80\x82\x01aN\xFC_\x83\x01\x83aN\xC7V[aO\x08_\x85\x01\x82aN\xDDV[PaO\x16` \x83\x01\x83aN\xC7V[aO#` \x85\x01\x82aN\xDDV[PaO1`@\x83\x01\x83aN\xC7V[aO>`@\x85\x01\x82aN\xDDV[PaOL``\x83\x01\x83aN\xC7V[aOY``\x85\x01\x82aN\xDDV[PPPPV[_`\xA0\x82\x01\x90P\x81\x81\x03_\x83\x01RaOx\x81\x85\x87aNUV[\x90PaO\x87` \x83\x01\x84aN\xECV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_aO\xAD\x82aO\x8FV[aO\xB7\x81\x85aO\x99V[\x93PaO\xC7\x81\x85` \x86\x01a5\xCBV[\x80\x84\x01\x91PP\x92\x91PPV[_aO\xDE\x82\x84aO\xA3V[\x91P\x81\x90P\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaP\x01\x81\x86a6\x03V[\x90PaP\x10` \x83\x01\x85a6\xFDV[aP\x1D`@\x83\x01\x84a6\xFDV[\x94\x93PPPPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101d7575f3560e01c80637eaac8f211610101578063bf9b16c811610094578063d740e40211610063578063d740e402146106f9578063d8f8392b14610721578063f76ca57714610749578063f9c670c314610771576101d7565b8063bf9b16c81461062f578063c0ae64f71461066b578063c2b4298614610693578063c3aaaa5a146106bd576101d7565b8063a92c75cb116100d0578063a92c75cb1461059d578063ad3cb1cc146105c5578063b4722bc4146105ef578063bac22bb814610619576101d7565b80637eaac8f2146104d15780639447cfd4146104fb578063976f3eb9146105375780639a7860e014610561576101d7565b806331ff41c8116101795780634f1ef286116101485780634f1ef2861461042757806352d1902d14610443578063556ecafa1461046d5780635bff76d914610495576101d7565b806331ff41c81461033757806341ad069c1461037357806346c5bbbd146103af57806347e82295146103eb576101d7565b8063203d0114116101b5578063203d01141461026b57806326cf5def146102a7578063281e8bfe146102d15780632a3889981461030d576101d7565b80630d8e6e2c146101db5780630e1887c914610205578063170a298114610241575b5f80fd5b3480156101e6575f80fd5b506101ef6107ad565b6040516101fc919061363b565b60405180910390f35b348015610210575f80fd5b5061022b6004803603810190610226919061369f565b610828565b60405161023891906136e4565b60405180910390f35b34801561024c575f80fd5b50610255610839565b604051610262919061370c565b60405180910390f35b348015610276575f80fd5b50610291600480360381019061028c919061377f565b61084b565b60405161029e91906136e4565b60405180910390f35b3480156102b2575f80fd5b506102bb6108bd565b6040516102c8919061370c565b60405180910390f35b3480156102dc575f80fd5b506102f760048036038101906102f2919061369f565b6108e6565b604051610304919061370c565b60405180910390f35b348015610318575f80fd5b50610321610912565b60405161032e919061370c565b60405180910390f35b348015610342575f80fd5b5061035d600480360381019061035891906137aa565b61093b565b60405161036a91906138a6565b60405180910390f35b34801561037e575f80fd5b506103996004803603810190610394919061369f565b610b7d565b6040516103a6919061370c565b60405180910390f35b3480156103ba575f80fd5b506103d560048036038101906103d091906137aa565b610ba9565b6040516103e291906136e4565b60405180910390f35b3480156103f6575f80fd5b50610411600480360381019061040c919061369f565b610c1d565b60405161041e919061370c565b60405180910390f35b610441600480360381019061043c91906139f2565b610c49565b005b34801561044e575f80fd5b50610457610c68565b6040516104649190613a64565b60405180910390f35b348015610478575f80fd5b50610493600480360381019061048e9190613afc565b610c99565b005b3480156104a0575f80fd5b506104bb60048036038101906104b6919061369f565b610e9d565b6040516104c89190613c15565b60405180910390f35b3480156104dc575f80fd5b506104e5610f4b565b6040516104f29190613c15565b60405180910390f35b348015610506575f80fd5b50610521600480360381019061051c91906137aa565b610ff6565b60405161052e91906136e4565b60405180910390f35b348015610542575f80fd5b5061054b61106a565b604051610558919061370c565b60405180910390f35b34801561056c575f80fd5b506105876004803603810190610582919061369f565b61107b565b6040516105949190613dc8565b60405180910390f35b3480156105a8575f80fd5b506105c360048036038101906105be9190613de8565b6112ee565b005b3480156105d0575f80fd5b506105d96113ef565b6040516105e6919061363b565b60405180910390f35b3480156105fa575f80fd5b50610603611428565b604051610610919061370c565b60405180910390f35b348015610624575f80fd5b5061062d611451565b005b34801561063a575f80fd5b506106556004803603810190610650919061369f565b61158b565b60405161066291906136e4565b60405180910390f35b348015610676575f80fd5b50610691600480360381019061068c919061369f565b61159c565b005b34801561069e575f80fd5b506106a7611784565b6040516106b4919061370c565b60405180910390f35b3480156106c8575f80fd5b506106e360048036038101906106de919061369f565b6117ad565b6040516106f0919061370c565b60405180910390f35b348015610704575f80fd5b5061071f600480360381019061071a919061369f565b6117d9565b005b34801561072c575f80fd5b5061074760048036038101906107429190613de8565b611978565b005b348015610754575f80fd5b5061076f600480360381019061076a9190613f19565b611b1e565b005b34801561077c575f80fd5b506107976004803603810190610792919061369f565b612019565b6040516107a491906140cc565b60405180910390f35b60606040518060400160405280600e81526020017f50726f746f636f6c436f6e6669670000000000000000000000000000000000008152506107ee5f612261565b6107f86001612261565b6108015f612261565b60405160200161081494939291906141ba565b604051602081830303815290604052905090565b5f6108328261232b565b9050919050565b5f6108426123a8565b600b0154905090565b5f806108556123a8565b9050806003015f825f015481526020019081526020015f205f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806108c76123a8565b9050806009015f825f015481526020019081526020015f205491505090565b5f6108f0826123cf565b6108f86123a8565b6007015f8381526020019081526020015f20549050919050565b5f8061091c6123a8565b9050806006015f825f015481526020019081526020015f205491505090565b61094361351d565b61094c836123cf565b6109546123a8565b6004015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600282018054610a6590614245565b80601f0160208091040260200160405190810160405280929190818152602001828054610a9190614245565b8015610adc5780601f10610ab357610100808354040283529160200191610adc565b820191905f5260205f20905b815481529060010190602001808311610abf57829003601f168201915b50505050508152602001600382018054610af590614245565b80601f0160208091040260200160405190810160405280929190818152602001828054610b2190614245565b8015610b6c5780601f10610b4357610100808354040283529160200191610b6c565b820191905f5260205f20905b815481529060010190602001808311610b4f57829003601f168201915b505050505081525050905092915050565b5f610b87826123cf565b610b8f6123a8565b6008015f8381526020019081526020015f20549050919050565b5f610bb3836123cf565b610bbb6123a8565b6002015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f610c27826123cf565b610c2f6123a8565b6009015f8381526020019081526020015f20549050919050565b610c5161241c565b610c5a82612502565b610c6482826125f5565b5050565b5f610c71612713565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b6001610ca361279a565b67ffffffffffffffff1614610ce4576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610cef6127be565b9050805f0160089054906101000a900460ff1680610d3757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610d6e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550600160f86007901b610dc591906142a2565b861015610e0957856040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401610e00919061370c565b60405180910390fd5b5f610e126123a8565b9050600187610e2191906142d5565b815f018190555060f86009901b81600b0181905550610e418686866127e5565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610e8d9190614317565b60405180910390a1505050505050565b6060610ea8826123cf565b610eb06123a8565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610f3f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610ef6575b50505050509050919050565b60605f610f566123a8565b9050806005015f825f015481526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610feb57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610fa2575b505050505091505090565b5f611000836123cf565b6110086123a8565b6003015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f6110736123a8565b5f0154905090565b61108361356f565b61108c8261232b565b6110cd57816040517f9797c3ff0000000000000000000000000000000000000000000000000000000081526004016110c4919061370c565b60405180910390fd5b6110d56123a8565b600c015f8381526020019081526020015f206040518060a00160405290815f82015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160089054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160109054906101000a900460ff1615151515815260200160018201805461117d90614245565b80601f01602080910402602001604051908101604052809291908181526020018280546111a990614245565b80156111f45780601f106111cb576101008083540402835291602001916111f4565b820191905f5260205f20905b8154815290600101906020018083116111d757829003601f168201915b5050505050815260200160028201805480602002602001604051908101604052809291908181526020015f905b828210156112df578382905f5260205f20016040518060600160405290815f82015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160089054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff1681526020015f820160109054906101000a900467ffffffffffffffff1667ffffffffffffffff1667ffffffffffffffff168152505081526020019060010190611221565b50505050815250509050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561134b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061136f9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146113de57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016113d5919061437e565b60405180910390fd5b6113e98383836127e5565b50505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f806114326123a8565b9050806008015f825f015481526020019081526020015f205491505090565b60035f61145c6127be565b9050805f0160089054906101000a900460ff16806114a457508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156114db576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555060f86009901b61152e6123a8565b600b01819055505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d28260405161157f9190614317565b60405180910390a15050565b5f61159582612e31565b9050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156115f9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061161d9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461168c57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611683919061437e565b60405180910390fd5b5f6116956123a8565b9050805f015482036116de57816040517f4595fce20000000000000000000000000000000000000000000000000000000081526004016116d5919061370c565b60405180910390fd5b6116e782612e31565b61172857816040517f77ddbe8100000000000000000000000000000000000000000000000000000000815260040161171f919061370c565b60405180910390fd5b600181600a015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550817fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f060405160405180910390a25050565b5f8061178e6123a8565b9050806007015f825f015481526020019081526020015f205491505090565b5f6117b7826123cf565b6117bf6123a8565b6006015f8381526020019081526020015f20549050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611836573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061185a9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146118c957336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016118c0919061437e565b60405180910390fd5b6118d28161232b565b61191357806040517f9797c3ff00000000000000000000000000000000000000000000000000000000815260040161190a919061370c565b60405180910390fd5b600161191d6123a8565b600c015f8381526020019081526020015f205f0160106101000a81548160ff021916908315150217905550807f6ed5f2c759f9fa25b478511dae2aa768dc993e9d04ab15f9c2519f075c4725d360405160405180910390a250565b600161198261279a565b67ffffffffffffffff16146119c3576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f6119ce6127be565b9050805f0160089054906101000a900460ff1680611a1657508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15611a4d576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f611a9b6123a8565b905060f86007901b815f018190555060f86009901b81600b0181905550611ac38686866127e5565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051611b0f9190614317565b60405180910390a15050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611b7b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b9f9190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611c0e57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611c05919061437e565b60405180910390fd5b5f8585905003611c4a576040517fb548914700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8383905003611c86576040517fbe50504400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8167ffffffffffffffff1603611cc9576040517f17d3e94800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b83839050811015611ec55736848483818110611cea57611ce9614397565b5b90506060020190505f815f016020810190611d0591906143c4565b67ffffffffffffffff1603611d46576040517fc84885d400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b806040016020810190611d5991906143c4565b67ffffffffffffffff16816020016020810190611d7691906143c4565b67ffffffffffffffff161115611dfc57805f016020810190611d9891906143c4565b816020016020810190611dab91906143c4565b826040016020810190611dbe91906143c4565b6040517ff219dc0e000000000000000000000000000000000000000000000000000000008152600401611df3939291906143ef565b60405180910390fd5b5f5b82811015611eb657815f016020810190611e1891906143c4565b67ffffffffffffffff16868683818110611e3557611e34614397565b5b9050606002015f016020810190611e4c91906143c4565b67ffffffffffffffff1603611ea957815f016020810190611e6d91906143c4565b6040517f6c67e470000000000000000000000000000000000000000000000000000000008152600401611ea09190614317565b60405180910390fd5b8080600101915050611dfe565b50508080600101915050611ccb565b505f611ecf6123a8565b90505f81600b015f8154611ee290614424565b91905081905590505f82600c015f8381526020019081526020015f2090508787826001019182611f13929190614612565b5083815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff16021790555043815f0160086101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055505f5b86869050811015611fce5781600201878783818110611f8b57611f8a614397565b5b905060600201908060018154018082558091505060019003905f5260205f20015f909190919091508181611fbf91906148a0565b50508080600101915050611f69565b50817f595d10949fcf822de17e89ebc302566ed150171ff414fe14d92b78a6d3aecce889898989896040516120079594939291906149e9565b60405180910390a25050505050505050565b6060612024826123cf565b61202c6123a8565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015612256578382905f5260205f2090600402016040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461213790614245565b80601f016020809104026020016040519081016040528092919081815260200182805461216390614245565b80156121ae5780601f10612185576101008083540402835291602001916121ae565b820191905f5260205f20905b81548152906001019060200180831161219157829003601f168201915b505050505081526020016003820180546121c790614245565b80601f01602080910402602001604051908101604052809291908181526020018280546121f390614245565b801561223e5780601f106122155761010080835404028352916020019161223e565b820191905f5260205f20905b81548152906001019060200180831161222157829003601f168201915b5050505050815250508152602001906001019061205d565b505050509050919050565b60605f600161226f84612eb4565b0190505f8167ffffffffffffffff81111561228d5761228c6138ce565b5b6040519080825280601f01601f1916602001820160405280156122bf5781602001600182028036833780820191505090505b5090505f82602001820190505b600115612320578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161231557612314614a30565b5b0494505f85036122cc575b819350505050919050565b5f806123356123a8565b90505f81600c015f8581526020019081526020015f209050600160f86009901b61235f91906142a2565b8410158015612372575081600b01548411155b801561238557505f816002018054905014155b801561239f5750805f0160109054906101000a900460ff16155b92505050919050565b5f7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200905090565b6123d881612e31565b61241957806040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401612410919061370c565b60405180910390fd5b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614806124c957507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166124b0613005565b73ffffffffffffffffffffffffffffffffffffffff1614155b15612500576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561255f573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906125839190614344565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146125f257336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016125e9919061437e565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561265d57506040513d601f19601f8201168201806040525081019061265a9190614a87565b60015b61269e57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401612695919061437e565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461270457806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016126fb9190613a64565b60405180910390fd5b61270e8383613058565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614612798576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6127a36127be565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f808484905003612822576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60ff8016848490501115612875578383905060ff80166040517f16a7277800000000000000000000000000000000000000000000000000000000815260040161286c929190614ab2565b60405180910390fd5b61288282858590506130ca565b5f61288b6123a8565b9050805f015f815461289c90614424565b91905081905591505f5b85859050811015612d7d57368686838181106128c5576128c4614397565b5b90506020028101906128d79190614ae5565b90505f73ffffffffffffffffffffffffffffffffffffffff16815f016020810190612902919061377f565b73ffffffffffffffffffffffffffffffffffffffff160361294f576040517f8466804a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff16816020016020810190612979919061377f565b73ffffffffffffffffffffffffffffffffffffffff16036129c6576040517f2deccf4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826002015f8581526020019081526020015f205f825f0160208101906129ec919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612a8557805f016020810190612a49919061377f565b6040517fd18c4ff0000000000000000000000000000000000000000000000000000000008152600401612a7c919061437e565b60405180910390fd5b826003015f8581526020019081526020015f205f826020016020810190612aac919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615612b4657806020016020810190612b0a919061377f565b6040517ff51af6bb000000000000000000000000000000000000000000000000000000008152600401612b3d919061437e565b60405180910390fd5b826001015f8581526020019081526020015f2081908060018154018082558091505060019003905f5260205f2090600402015f909190919091508181612b8c9190614cbd565b50506001836002015f8681526020019081526020015f205f835f016020810190612bb6919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506001836003015f8681526020019081526020015f205f836020016020810190612c2e919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080836004015f8681526020019081526020015f205f835f016020810190612ca4919061377f565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181612ce99190614cbd565b905050826005015f8581526020019081526020015f20816020016020810190612d12919061377f565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505080806001019150506128a6565b50825f0135816006015f8481526020019081526020015f20819055508260200135816007015f8481526020019081526020015f20819055508260400135816008015f8481526020019081526020015f20819055508260600135816009015f8481526020019081526020015f2081905550817fe5296a8184d19a5fd24548749ea3c435b69ad26f12ca0afa1e8efef592368bf2868686604051612e2193929190614f5f565b60405180910390a2509392505050565b5f80612e3b6123a8565b9050600160f86007901b612e4f91906142a2565b8310158015612e615750805f01548311155b8015612e8357505f816001015f8581526020019081526020015f208054905014155b8015612eac575080600a015f8481526020019081526020015f205f9054906101000a900460ff16155b915050919050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612f10577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381612f0657612f05614a30565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612f4d576d04ee2d6d415b85acef81000000008381612f4357612f42614a30565b5b0492506020810190505b662386f26fc100008310612f7c57662386f26fc100008381612f7257612f71614a30565b5b0492506010810190505b6305f5e1008310612fa5576305f5e1008381612f9b57612f9a614a30565b5b0492506008810190505b6127108310612fca576127108381612fc057612fbf614a30565b5b0492506004810190505b60648310612fed5760648381612fe357612fe2614a30565b5b0492506002810190505b600a8310612ffc576001810190505b80915050919050565b5f6130317f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6131dd565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b613061826131e6565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156130bd576130b782826132af565b506130c6565b6130c561332f565b5b5050565b61310d6040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250835f01358361336b565b6131516040518060400160405280600e81526020017f7573657244656372797074696f6e00000000000000000000000000000000000081525083602001358361336b565b6131956040518060400160405280600681526020017f6b6d7347656e000000000000000000000000000000000000000000000000000081525083604001358361336b565b6131d96040518060400160405280600381526020017f6d7063000000000000000000000000000000000000000000000000000000000081525083606001358361336b565b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b0361324157806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613238919061437e565b60405180910390fd5b8061326d7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6131dd565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516132d89190614fd3565b5f60405180830381855af49150503d805f8114613310576040519150601f19603f3d011682016040523d82523d5f602084013e613315565b606091505b509150915061332585838361344c565b9250505092915050565b5f341115613369576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f82036133af57826040517f36bfb60e0000000000000000000000000000000000000000000000000000000081526004016133a6919061363b565b60405180910390fd5b60ff80168211156133fe57828260ff80166040517f22ba52db0000000000000000000000000000000000000000000000000000000081526004016133f593929190614fe9565b60405180910390fd5b80821115613447578282826040517fcaa814a300000000000000000000000000000000000000000000000000000000815260040161343e93929190614fe9565b60405180910390fd5b505050565b6060826134615761345c826134d9565b6134d1565b5f825114801561348757505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156134c957836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016134c0919061437e565b60405180910390fd5b8190506134d2565b5b9392505050565b5f815111156134eb5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b6040518060a001604052805f67ffffffffffffffff1681526020015f67ffffffffffffffff1681526020015f1515815260200160608152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b838110156135e85780820151818401526020810190506135cd565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61360d826135b1565b61361781856135bb565b93506136278185602086016135cb565b613630816135f3565b840191505092915050565b5f6020820190508181035f8301526136538184613603565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f819050919050565b61367e8161366c565b8114613688575f80fd5b50565b5f8135905061369981613675565b92915050565b5f602082840312156136b4576136b3613664565b5b5f6136c18482850161368b565b91505092915050565b5f8115159050919050565b6136de816136ca565b82525050565b5f6020820190506136f75f8301846136d5565b92915050565b6137068161366c565b82525050565b5f60208201905061371f5f8301846136fd565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61374e82613725565b9050919050565b61375e81613744565b8114613768575f80fd5b50565b5f8135905061377981613755565b92915050565b5f6020828403121561379457613793613664565b5b5f6137a18482850161376b565b91505092915050565b5f80604083850312156137c0576137bf613664565b5b5f6137cd8582860161368b565b92505060206137de8582860161376b565b9150509250929050565b6137f181613744565b82525050565b5f82825260208201905092915050565b5f613811826135b1565b61381b81856137f7565b935061382b8185602086016135cb565b613834816135f3565b840191505092915050565b5f608083015f8301516138545f8601826137e8565b50602083015161386760208601826137e8565b506040830151848203604086015261387f8282613807565b915050606083015184820360608601526138998282613807565b9150508091505092915050565b5f6020820190508181035f8301526138be818461383f565b905092915050565b5f80fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b613904826135f3565b810181811067ffffffffffffffff82111715613923576139226138ce565b5b80604052505050565b5f61393561365b565b905061394182826138fb565b919050565b5f67ffffffffffffffff8211156139605761395f6138ce565b5b613969826135f3565b9050602081019050919050565b828183375f83830152505050565b5f61399661399184613946565b61392c565b9050828152602081018484840111156139b2576139b16138ca565b5b6139bd848285613976565b509392505050565b5f82601f8301126139d9576139d86138c6565b5b81356139e9848260208601613984565b91505092915050565b5f8060408385031215613a0857613a07613664565b5b5f613a158582860161376b565b925050602083013567ffffffffffffffff811115613a3657613a35613668565b5b613a42858286016139c5565b9150509250929050565b5f819050919050565b613a5e81613a4c565b82525050565b5f602082019050613a775f830184613a55565b92915050565b5f80fd5b5f80fd5b5f8083601f840112613a9a57613a996138c6565b5b8235905067ffffffffffffffff811115613ab757613ab6613a7d565b5b602083019150836020820283011115613ad357613ad2613a81565b5b9250929050565b5f80fd5b5f60808284031215613af357613af2613ada565b5b81905092915050565b5f805f8060c08587031215613b1457613b13613664565b5b5f613b218782880161368b565b945050602085013567ffffffffffffffff811115613b4257613b41613668565b5b613b4e87828801613a85565b93509350506040613b6187828801613ade565b91505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f613ba183836137e8565b60208301905092915050565b5f602082019050919050565b5f613bc382613b6d565b613bcd8185613b77565b9350613bd883613b87565b805f5b83811015613c08578151613bef8882613b96565b9750613bfa83613bad565b925050600181019050613bdb565b5085935050505092915050565b5f6020820190508181035f830152613c2d8184613bb9565b905092915050565b5f67ffffffffffffffff82169050919050565b613c5181613c35565b82525050565b613c60816136ca565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b606082015f820151613ca35f850182613c48565b506020820151613cb66020850182613c48565b506040820151613cc96040850182613c48565b50505050565b5f613cda8383613c8f565b60608301905092915050565b5f602082019050919050565b5f613cfc82613c66565b613d068185613c70565b9350613d1183613c80565b805f5b83811015613d41578151613d288882613ccf565b9750613d3383613ce6565b925050600181019050613d14565b5085935050505092915050565b5f60a083015f830151613d635f860182613c48565b506020830151613d766020860182613c48565b506040830151613d896040860182613c57565b5060608301518482036060860152613da18282613807565b91505060808301518482036080860152613dbb8282613cf2565b9150508091505092915050565b5f6020820190508181035f830152613de08184613d4e565b905092915050565b5f805f60a08486031215613dff57613dfe613664565b5b5f84013567ffffffffffffffff811115613e1c57613e1b613668565b5b613e2886828701613a85565b93509350506020613e3b86828701613ade565b9150509250925092565b5f8083601f840112613e5a57613e596138c6565b5b8235905067ffffffffffffffff811115613e7757613e76613a7d565b5b602083019150836001820283011115613e9357613e92613a81565b5b9250929050565b5f8083601f840112613eaf57613eae6138c6565b5b8235905067ffffffffffffffff811115613ecc57613ecb613a7d565b5b602083019150836060820283011115613ee857613ee7613a81565b5b9250929050565b613ef881613c35565b8114613f02575f80fd5b50565b5f81359050613f1381613eef565b92915050565b5f805f805f60608688031215613f3257613f31613664565b5b5f86013567ffffffffffffffff811115613f4f57613f4e613668565b5b613f5b88828901613e45565b9550955050602086013567ffffffffffffffff811115613f7e57613f7d613668565b5b613f8a88828901613e9a565b93509350506040613f9d88828901613f05565b9150509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f608083015f830151613fe85f8601826137e8565b506020830151613ffb60208601826137e8565b50604083015184820360408601526140138282613807565b9150506060830151848203606086015261402d8282613807565b9150508091505092915050565b5f6140458383613fd3565b905092915050565b5f602082019050919050565b5f61406382613faa565b61406d8185613fb4565b93508360208202850161407f85613fc4565b805f5b858110156140ba578484038952815161409b858261403a565b94506140a68361404d565b925060208a01995050600181019050614082565b50829750879550505050505092915050565b5f6020820190508181035f8301526140e48184614059565b905092915050565b5f81905092915050565b5f614100826135b1565b61410a81856140ec565b935061411a8185602086016135cb565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f61415a6002836140ec565b915061416582614126565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6141a46001836140ec565b91506141af82614170565b600182019050919050565b5f6141c582876140f6565b91506141d08261414e565b91506141dc82866140f6565b91506141e782614198565b91506141f382856140f6565b91506141fe82614198565b915061420a82846140f6565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061425c57607f821691505b60208210810361426f5761426e614218565b5b50919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6142ac8261366c565b91506142b78361366c565b92508282019050808211156142cf576142ce614275565b5b92915050565b5f6142df8261366c565b91506142ea8361366c565b925082820390508181111561430257614301614275565b5b92915050565b61431181613c35565b82525050565b5f60208201905061432a5f830184614308565b92915050565b5f8151905061433e81613755565b92915050565b5f6020828403121561435957614358613664565b5b5f61436684828501614330565b91505092915050565b61437881613744565b82525050565b5f6020820190506143915f83018461436f565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f602082840312156143d9576143d8613664565b5b5f6143e684828501613f05565b91505092915050565b5f6060820190506144025f830186614308565b61440f6020830185614308565b61441c6040830184614308565b949350505050565b5f61442e8261366c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036144605761445f614275565b5b600182019050919050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026144d17fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82614496565b6144db8683614496565b95508019841693508086168417925050509392505050565b5f819050919050565b5f61451661451161450c8461366c565b6144f3565b61366c565b9050919050565b5f819050919050565b61452f836144fc565b61454361453b8261451d565b8484546144a2565b825550505050565b5f90565b61455761454b565b614562818484614526565b505050565b5b818110156145855761457a5f8261454f565b600181019050614568565b5050565b601f8211156145ca5761459b81614475565b6145a484614487565b810160208510156145b3578190505b6145c76145bf85614487565b830182614567565b50505b505050565b5f82821c905092915050565b5f6145ea5f19846008026145cf565b1980831691505092915050565b5f61460283836145db565b9150826002028217905092915050565b61461c838361446b565b67ffffffffffffffff811115614635576146346138ce565b5b61463f8254614245565b61464a828285614589565b5f601f831160018114614677575f8415614665578287013590505b61466f85826145f7565b8655506146d6565b601f19841661468586614475565b5f5b828110156146ac57848901358255600182019150602085019450602081019050614687565b868310156146c957848901356146c5601f8916826145db565b8355505b6001600288020188555050505b50505050505050565b5f81356146eb81613eef565b80915050919050565b5f815f1b9050919050565b5f67ffffffffffffffff614712846146f4565b9350801983169250808416831791505092915050565b5f61474261473d61473884613c35565b6144f3565b613c35565b9050919050565b5f819050919050565b61475b82614728565b61476e61476782614749565b83546146ff565b8255505050565b5f8160401b9050919050565b5f6fffffffffffffffff000000000000000061479c84614775565b9350801983169250808416831791505092915050565b6147bb82614728565b6147ce6147c782614749565b8354614781565b8255505050565b5f8160801b9050919050565b5f77ffffffffffffffff00000000000000000000000000000000614804846147d5565b9350801983169250808416831791505092915050565b61482382614728565b61483661482f82614749565b83546147e1565b8255505050565b5f81015f83018061484d816146df565b90506148598184614752565b5050505f8101602083018061486d816146df565b905061487981846147b2565b5050505f8101604083018061488d816146df565b9050614899818461481a565b5050505050565b6148aa828261483d565b5050565b5f6148b983856135bb565b93506148c6838584613976565b6148cf836135f3565b840190509392505050565b5f82825260208201905092915050565b5f819050919050565b5f6149016020840184613f05565b905092915050565b606082016149195f8301836148f3565b6149255f850182613c48565b5061493360208301836148f3565b6149406020850182613c48565b5061494e60408301836148f3565b61495b6040850182613c48565b50505050565b5f61496c8383614909565b60608301905092915050565b5f82905092915050565b5f606082019050919050565b5f61499983856148da565b93506149a4826148ea565b805f5b858110156149dc576149b98284614978565b6149c38882614961565b97506149ce83614982565b9250506001810190506149a7565b5085925050509392505050565b5f6060820190508181035f830152614a028187896148ae565b90508181036020830152614a1781858761498e565b9050614a266040830184614308565b9695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b614a6681613a4c565b8114614a70575f80fd5b50565b5f81519050614a8181614a5d565b92915050565b5f60208284031215614a9c57614a9b613664565b5b5f614aa984828501614a73565b91505092915050565b5f604082019050614ac55f8301856136fd565b614ad260208301846136fd565b9392505050565b5f80fd5b5f80fd5b5f80fd5b5f82356001608003833603038112614b0057614aff614ad9565b5b80830191505092915050565b5f8135614b1881613755565b80915050919050565b5f73ffffffffffffffffffffffffffffffffffffffff614b40846146f4565b9350801983169250808416831791505092915050565b5f614b70614b6b614b6684613725565b6144f3565b613725565b9050919050565b5f614b8182614b56565b9050919050565b5f614b9282614b77565b9050919050565b5f819050919050565b614bab82614b88565b614bbe614bb782614b99565b8354614b21565b8255505050565b5f8083356001602003843603038112614be157614be0614ad9565b5b80840192508235915067ffffffffffffffff821115614c0357614c02614add565b5b602083019250600182023603831315614c1f57614c1e614ae1565b5b509250929050565b614c32838383614612565b505050565b5f81015f830180614c4781614b0c565b9050614c538184614ba2565b505050600181016020830180614c6881614b0c565b9050614c748184614ba2565b5050506002810160408301614c898185614bc5565b614c94818386614c27565b505050506003810160608301614caa8185614bc5565b614cb5818386614c27565b505050505050565b614cc78282614c37565b5050565b5f819050919050565b5f614ce2602084018461376b565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f8083356001602003843603038112614d1257614d11614cf2565b5b83810192508235915060208301925067ffffffffffffffff821115614d3a57614d39614cea565b5b600182023603831315614d5057614d4f614cee565b5b509250929050565b5f614d6383856137f7565b9350614d70838584613976565b614d79836135f3565b840190509392505050565b5f60808301614d955f840184614cd4565b614da15f8601826137e8565b50614daf6020840184614cd4565b614dbc60208601826137e8565b50614dca6040840184614cf6565b8583036040870152614ddd838284614d58565b92505050614dee6060840184614cf6565b8583036060870152614e01838284614d58565b925050508091505092915050565b5f614e1a8383614d84565b905092915050565b5f82356001608003833603038112614e3d57614e3c614cf2565b5b82810191505092915050565b5f602082019050919050565b5f614e608385613fb4565b935083602084028501614e7284614ccb565b805f5b87811015614eb5578484038952614e8c8284614e22565b614e968582614e0f565b9450614ea183614e49565b925060208a01995050600181019050614e75565b50829750879450505050509392505050565b5f614ed5602084018461368b565b905092915050565b614ee68161366c565b82525050565b60808201614efc5f830183614ec7565b614f085f850182614edd565b50614f166020830183614ec7565b614f236020850182614edd565b50614f316040830183614ec7565b614f3e6040850182614edd565b50614f4c6060830183614ec7565b614f596060850182614edd565b50505050565b5f60a0820190508181035f830152614f78818587614e55565b9050614f876020830184614eec565b949350505050565b5f81519050919050565b5f81905092915050565b5f614fad82614f8f565b614fb78185614f99565b9350614fc78185602086016135cb565b80840191505092915050565b5f614fde8284614fa3565b915081905092915050565b5f6060820190508181035f8301526150018186613603565b905061501060208301856136fd565b61501d60408301846136fd565b94935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xD7W_5`\xE0\x1C\x80c~\xAA\xC8\xF2\x11a\x01\x01W\x80c\xBF\x9B\x16\xC8\x11a\0\x94W\x80c\xD7@\xE4\x02\x11a\0cW\x80c\xD7@\xE4\x02\x14a\x06\xF9W\x80c\xD8\xF89+\x14a\x07!W\x80c\xF7l\xA5w\x14a\x07IW\x80c\xF9\xC6p\xC3\x14a\x07qWa\x01\xD7V[\x80c\xBF\x9B\x16\xC8\x14a\x06/W\x80c\xC0\xAEd\xF7\x14a\x06kW\x80c\xC2\xB4)\x86\x14a\x06\x93W\x80c\xC3\xAA\xAAZ\x14a\x06\xBDWa\x01\xD7V[\x80c\xA9,u\xCB\x11a\0\xD0W\x80c\xA9,u\xCB\x14a\x05\x9DW\x80c\xAD<\xB1\xCC\x14a\x05\xC5W\x80c\xB4r+\xC4\x14a\x05\xEFW\x80c\xBA\xC2+\xB8\x14a\x06\x19Wa\x01\xD7V[\x80c~\xAA\xC8\xF2\x14a\x04\xD1W\x80c\x94G\xCF\xD4\x14a\x04\xFBW\x80c\x97o>\xB9\x14a\x057W\x80c\x9Ax`\xE0\x14a\x05aWa\x01\xD7V[\x80c1\xFFA\xC8\x11a\x01yW\x80cO\x1E\xF2\x86\x11a\x01HW\x80cO\x1E\xF2\x86\x14a\x04'W\x80cR\xD1\x90-\x14a\x04CW\x80cUn\xCA\xFA\x14a\x04mW\x80c[\xFFv\xD9\x14a\x04\x95Wa\x01\xD7V[\x80c1\xFFA\xC8\x14a\x037W\x80cA\xAD\x06\x9C\x14a\x03sW\x80cF\xC5\xBB\xBD\x14a\x03\xAFW\x80cG\xE8\"\x95\x14a\x03\xEBWa\x01\xD7V[\x80c =\x01\x14\x11a\x01\xB5W\x80c =\x01\x14\x14a\x02kW\x80c&\xCF]\xEF\x14a\x02\xA7W\x80c(\x1E\x8B\xFE\x14a\x02\xD1W\x80c*8\x89\x98\x14a\x03\rWa\x01\xD7V[\x80c\r\x8En,\x14a\x01\xDBW\x80c\x0E\x18\x87\xC9\x14a\x02\x05W\x80c\x17\n)\x81\x14a\x02AW[_\x80\xFD[4\x80\x15a\x01\xE6W_\x80\xFD[Pa\x01\xEFa\x07\xADV[`@Qa\x01\xFC\x91\x90a6;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x10W_\x80\xFD[Pa\x02+`\x04\x806\x03\x81\x01\x90a\x02&\x91\x90a6\x9FV[a\x08(V[`@Qa\x028\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02LW_\x80\xFD[Pa\x02Ua\x089V[`@Qa\x02b\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02vW_\x80\xFD[Pa\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90a7\x7FV[a\x08KV[`@Qa\x02\x9E\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xB2W_\x80\xFD[Pa\x02\xBBa\x08\xBDV[`@Qa\x02\xC8\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xDCW_\x80\xFD[Pa\x02\xF7`\x04\x806\x03\x81\x01\x90a\x02\xF2\x91\x90a6\x9FV[a\x08\xE6V[`@Qa\x03\x04\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x18W_\x80\xFD[Pa\x03!a\t\x12V[`@Qa\x03.\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03BW_\x80\xFD[Pa\x03]`\x04\x806\x03\x81\x01\x90a\x03X\x91\x90a7\xAAV[a\t;V[`@Qa\x03j\x91\x90a8\xA6V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03~W_\x80\xFD[Pa\x03\x99`\x04\x806\x03\x81\x01\x90a\x03\x94\x91\x90a6\x9FV[a\x0B}V[`@Qa\x03\xA6\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xBAW_\x80\xFD[Pa\x03\xD5`\x04\x806\x03\x81\x01\x90a\x03\xD0\x91\x90a7\xAAV[a\x0B\xA9V[`@Qa\x03\xE2\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xF6W_\x80\xFD[Pa\x04\x11`\x04\x806\x03\x81\x01\x90a\x04\x0C\x91\x90a6\x9FV[a\x0C\x1DV[`@Qa\x04\x1E\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[a\x04A`\x04\x806\x03\x81\x01\x90a\x04<\x91\x90a9\xF2V[a\x0CIV[\0[4\x80\x15a\x04NW_\x80\xFD[Pa\x04Wa\x0ChV[`@Qa\x04d\x91\x90a:dV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04xW_\x80\xFD[Pa\x04\x93`\x04\x806\x03\x81\x01\x90a\x04\x8E\x91\x90a:\xFCV[a\x0C\x99V[\0[4\x80\x15a\x04\xA0W_\x80\xFD[Pa\x04\xBB`\x04\x806\x03\x81\x01\x90a\x04\xB6\x91\x90a6\x9FV[a\x0E\x9DV[`@Qa\x04\xC8\x91\x90a<\x15V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xDCW_\x80\xFD[Pa\x04\xE5a\x0FKV[`@Qa\x04\xF2\x91\x90a<\x15V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x06W_\x80\xFD[Pa\x05!`\x04\x806\x03\x81\x01\x90a\x05\x1C\x91\x90a7\xAAV[a\x0F\xF6V[`@Qa\x05.\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05BW_\x80\xFD[Pa\x05Ka\x10jV[`@Qa\x05X\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05lW_\x80\xFD[Pa\x05\x87`\x04\x806\x03\x81\x01\x90a\x05\x82\x91\x90a6\x9FV[a\x10{V[`@Qa\x05\x94\x91\x90a=\xC8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xA8W_\x80\xFD[Pa\x05\xC3`\x04\x806\x03\x81\x01\x90a\x05\xBE\x91\x90a=\xE8V[a\x12\xEEV[\0[4\x80\x15a\x05\xD0W_\x80\xFD[Pa\x05\xD9a\x13\xEFV[`@Qa\x05\xE6\x91\x90a6;V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xFAW_\x80\xFD[Pa\x06\x03a\x14(V[`@Qa\x06\x10\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06$W_\x80\xFD[Pa\x06-a\x14QV[\0[4\x80\x15a\x06:W_\x80\xFD[Pa\x06U`\x04\x806\x03\x81\x01\x90a\x06P\x91\x90a6\x9FV[a\x15\x8BV[`@Qa\x06b\x91\x90a6\xE4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06vW_\x80\xFD[Pa\x06\x91`\x04\x806\x03\x81\x01\x90a\x06\x8C\x91\x90a6\x9FV[a\x15\x9CV[\0[4\x80\x15a\x06\x9EW_\x80\xFD[Pa\x06\xA7a\x17\x84V[`@Qa\x06\xB4\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xC8W_\x80\xFD[Pa\x06\xE3`\x04\x806\x03\x81\x01\x90a\x06\xDE\x91\x90a6\x9FV[a\x17\xADV[`@Qa\x06\xF0\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x04W_\x80\xFD[Pa\x07\x1F`\x04\x806\x03\x81\x01\x90a\x07\x1A\x91\x90a6\x9FV[a\x17\xD9V[\0[4\x80\x15a\x07,W_\x80\xFD[Pa\x07G`\x04\x806\x03\x81\x01\x90a\x07B\x91\x90a=\xE8V[a\x19xV[\0[4\x80\x15a\x07TW_\x80\xFD[Pa\x07o`\x04\x806\x03\x81\x01\x90a\x07j\x91\x90a?\x19V[a\x1B\x1EV[\0[4\x80\x15a\x07|W_\x80\xFD[Pa\x07\x97`\x04\x806\x03\x81\x01\x90a\x07\x92\x91\x90a6\x9FV[a \x19V[`@Qa\x07\xA4\x91\x90a@\xCCV[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FProtocolConfig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x07\xEE_a\"aV[a\x07\xF8`\x01a\"aV[a\x08\x01_a\"aV[`@Q` \x01a\x08\x14\x94\x93\x92\x91\x90aA\xBAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_a\x082\x82a#+V[\x90P\x91\x90PV[_a\x08Ba#\xA8V[`\x0B\x01T\x90P\x90V[_\x80a\x08Ua#\xA8V[\x90P\x80`\x03\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ _\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x08\xC7a#\xA8V[\x90P\x80`\t\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x08\xF0\x82a#\xCFV[a\x08\xF8a#\xA8V[`\x07\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_\x80a\t\x1Ca#\xA8V[\x90P\x80`\x06\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[a\tCa5\x1DV[a\tL\x83a#\xCFV[a\tTa#\xA8V[`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\ne\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x91\x90aBEV[\x80\x15a\n\xDCW\x80`\x1F\x10a\n\xB3Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\xDCV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\xBFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\n\xF5\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B!\x90aBEV[\x80\x15a\x0BlW\x80`\x1F\x10a\x0BCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0BlV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0BOW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x92\x91PPV[_a\x0B\x87\x82a#\xCFV[a\x0B\x8Fa#\xA8V[`\x08\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[_a\x0B\xB3\x83a#\xCFV[a\x0B\xBBa#\xA8V[`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x0C'\x82a#\xCFV[a\x0C/a#\xA8V[`\t\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[a\x0CQa$\x1CV[a\x0CZ\x82a%\x02V[a\x0Cd\x82\x82a%\xF5V[PPV[_a\x0Cqa'\x13V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[`\x01a\x0C\xA3a'\x9AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C\xE4W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x0C\xEFa'\xBEV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\r7WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\rnW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\xF8`\x07\x90\x1Ba\r\xC5\x91\x90aB\xA2V[\x86\x10\x15a\x0E\tW\x85`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\0\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[_a\x0E\x12a#\xA8V[\x90P`\x01\x87a\x0E!\x91\x90aB\xD5V[\x81_\x01\x81\x90UP`\xF8`\t\x90\x1B\x81`\x0B\x01\x81\x90UPa\x0EA\x86\x86\x86a'\xE5V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0E\x8D\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[``a\x0E\xA8\x82a#\xCFV[a\x0E\xB0a#\xA8V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0F?W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\xF6W[PPPPP\x90P\x91\x90PV[``_a\x0FVa#\xA8V[\x90P\x80`\x05\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0F\xEBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0F\xA2W[PPPPP\x91PP\x90V[_a\x10\0\x83a#\xCFV[a\x10\x08a#\xA8V[`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x10sa#\xA8V[_\x01T\x90P\x90V[a\x10\x83a5oV[a\x10\x8C\x82a#+V[a\x10\xCDW\x81`@Q\x7F\x97\x97\xC3\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xC4\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[a\x10\xD5a#\xA8V[`\x0C\x01_\x83\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x15\x15\x15\x81R` \x01`\x01\x82\x01\x80Ta\x11}\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x11\xA9\x90aBEV[\x80\x15a\x11\xF4W\x80`\x1F\x10a\x11\xCBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x11\xF4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x11\xD7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x12\xDFW\x83\x82\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x10\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12!V[PPPP\x81RPP\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13KW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13o\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x13\xDEW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\xD5\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[a\x13\xE9\x83\x83\x83a'\xE5V[PPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x142a#\xA8V[\x90P\x80`\x08\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[`\x03_a\x14\\a'\xBEV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x14\xA4WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x14\xDBW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\xF8`\t\x90\x1Ba\x15.a#\xA8V[`\x0B\x01\x81\x90UP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x15\x7F\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xA1PPV[_a\x15\x95\x82a.1V[\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xF9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x1D\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x16\x8CW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x83\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[_a\x16\x95a#\xA8V[\x90P\x80_\x01T\x82\x03a\x16\xDEW\x81`@Q\x7FE\x95\xFC\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\xD5\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[a\x16\xE7\x82a.1V[a\x17(W\x81`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\x1F\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0`@Q`@Q\x80\x91\x03\x90\xA2PPV[_\x80a\x17\x8Ea#\xA8V[\x90P\x80`\x07\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x17\xB7\x82a#\xCFV[a\x17\xBFa#\xA8V[`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x186W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18Z\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xC9W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xC0\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[a\x18\xD2\x81a#+V[a\x19\x13W\x80`@Q\x7F\x97\x97\xC3\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19\n\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[`\x01a\x19\x1Da#\xA8V[`\x0C\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x01`\x10a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x7Fn\xD5\xF2\xC7Y\xF9\xFA%\xB4xQ\x1D\xAE*\xA7h\xDC\x99>\x9D\x04\xAB\x15\xF9\xC2Q\x9F\x07\\G%\xD3`@Q`@Q\x80\x91\x03\x90\xA2PV[`\x01a\x19\x82a'\x9AV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\xC3W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x19\xCEa'\xBEV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x1A\x16WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1AMW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x1A\x9Ba#\xA8V[\x90P`\xF8`\x07\x90\x1B\x81_\x01\x81\x90UP`\xF8`\t\x90\x1B\x81`\x0B\x01\x81\x90UPa\x1A\xC3\x86\x86\x86a'\xE5V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x1B\x0F\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xA1PPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B{W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x9F\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1C\x0EW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x05\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x1CJW`@Q\x7F\xB5H\x91G\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x83\x90P\x03a\x1C\x86W`@Q\x7F\xBEPPD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1C\xC9W`@Q\x7F\x17\xD3\xE9H\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x83\x90P\x81\x10\x15a\x1E\xC5W6\x84\x84\x83\x81\x81\x10a\x1C\xEAWa\x1C\xE9aC\x97V[[\x90P``\x02\x01\x90P_\x81_\x01` \x81\x01\x90a\x1D\x05\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1DFW`@Q\x7F\xC8H\x85\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`@\x01` \x81\x01\x90a\x1DY\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01` \x81\x01\x90a\x1Dv\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1D\xFCW\x80_\x01` \x81\x01\x90a\x1D\x98\x91\x90aC\xC4V[\x81` \x01` \x81\x01\x90a\x1D\xAB\x91\x90aC\xC4V[\x82`@\x01` \x81\x01\x90a\x1D\xBE\x91\x90aC\xC4V[`@Q\x7F\xF2\x19\xDC\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xF3\x93\x92\x91\x90aC\xEFV[`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x1E\xB6W\x81_\x01` \x81\x01\x90a\x1E\x18\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x86\x83\x81\x81\x10a\x1E5Wa\x1E4aC\x97V[[\x90P``\x02\x01_\x01` \x81\x01\x90a\x1EL\x91\x90aC\xC4V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1E\xA9W\x81_\x01` \x81\x01\x90a\x1Em\x91\x90aC\xC4V[`@Q\x7Flg\xE4p\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1E\xA0\x91\x90aC\x17V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa\x1D\xFEV[PP\x80\x80`\x01\x01\x91PPa\x1C\xCBV[P_a\x1E\xCFa#\xA8V[\x90P_\x81`\x0B\x01_\x81Ta\x1E\xE2\x90aD$V[\x91\x90P\x81\x90U\x90P_\x82`\x0C\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x87\x87\x82`\x01\x01\x91\x82a\x1F\x13\x92\x91\x90aF\x12V[P\x83\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPC\x81_\x01`\x08a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_[\x86\x86\x90P\x81\x10\x15a\x1F\xCEW\x81`\x02\x01\x87\x87\x83\x81\x81\x10a\x1F\x8BWa\x1F\x8AaC\x97V[[\x90P``\x02\x01\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x1F\xBF\x91\x90aH\xA0V[PP\x80\x80`\x01\x01\x91PPa\x1FiV[P\x81\x7FY]\x10\x94\x9F\xCF\x82-\xE1~\x89\xEB\xC3\x02Vn\xD1P\x17\x1F\xF4\x14\xFE\x14\xD9+x\xA6\xD3\xAE\xCC\xE8\x89\x89\x89\x89\x89`@Qa \x07\x95\x94\x93\x92\x91\x90aI\xE9V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[``a $\x82a#\xCFV[a ,a#\xA8V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\"VW\x83\x82\x90_R` _ \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta!7\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!c\x90aBEV[\x80\x15a!\xAEW\x80`\x1F\x10a!\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta!\xC7\x90aBEV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!\xF3\x90aBEV[\x80\x15a\">W\x80`\x1F\x10a\"\x15Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\">V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"!W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a ]V[PPPP\x90P\x91\x90PV[``_`\x01a\"o\x84a.\xB4V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x8DWa\"\x8Ca8\xCEV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\"\xBFW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a# W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#\x15Wa#\x14aJ0V[[\x04\x94P_\x85\x03a\"\xCCW[\x81\x93PPPP\x91\x90PV[_\x80a#5a#\xA8V[\x90P_\x81`\x0C\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x90P`\x01`\xF8`\t\x90\x1Ba#_\x91\x90aB\xA2V[\x84\x10\x15\x80\x15a#rWP\x81`\x0B\x01T\x84\x11\x15[\x80\x15a#\x85WP_\x81`\x02\x01\x80T\x90P\x14\x15[\x80\x15a#\x9FWP\x80_\x01`\x10\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x92PPP\x91\x90PV[_\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90P\x90V[a#\xD8\x81a.1V[a$\x19W\x80`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x10\x91\x90a7\x0CV[`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a$\xC9WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a$\xB0a0\x05V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a%\0W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%_W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\x83\x91\x90aCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a%\xF2W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xE9\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a&]WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&Z\x91\x90aJ\x87V[`\x01[a&\x9EW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\x95\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a'\x04W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xFB\x91\x90a:dV[`@Q\x80\x91\x03\x90\xFD[a'\x0E\x83\x83a0XV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'\x98W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a'\xA3a'\xBEV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_\x80\x84\x84\x90P\x03a(\"W`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x84\x84\x90P\x11\x15a(uW\x83\x83\x90P`\xFF\x80\x16`@Q\x7F\x16\xA7'x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(l\x92\x91\x90aJ\xB2V[`@Q\x80\x91\x03\x90\xFD[a(\x82\x82\x85\x85\x90Pa0\xCAV[_a(\x8Ba#\xA8V[\x90P\x80_\x01_\x81Ta(\x9C\x90aD$V[\x91\x90P\x81\x90U\x91P_[\x85\x85\x90P\x81\x10\x15a-}W6\x86\x86\x83\x81\x81\x10a(\xC5Wa(\xC4aC\x97V[[\x90P` \x02\x81\x01\x90a(\xD7\x91\x90aJ\xE5V[\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01` \x81\x01\x90a)\x02\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a)OW`@Q\x7F\x84f\x80J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01` \x81\x01\x90a)y\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a)\xC6W`@Q\x7F-\xEC\xCFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82_\x01` \x81\x01\x90a)\xEC\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a*\x85W\x80_\x01` \x81\x01\x90a*I\x91\x90a7\x7FV[`@Q\x7F\xD1\x8CO\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*|\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x82`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82` \x01` \x81\x01\x90a*\xAC\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a+FW\x80` \x01` \x81\x01\x90a+\n\x91\x90a7\x7FV[`@Q\x7F\xF5\x1A\xF6\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+=\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x04\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a+\x8C\x91\x90aL\xBDV[PP`\x01\x83`\x02\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a+\xB6\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x83`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83` \x01` \x81\x01\x90a,.\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x83`\x04\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a,\xA4\x91\x90a7\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a,\xE9\x91\x90aL\xBDV[\x90PP\x82`\x05\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81` \x01` \x81\x01\x90a-\x12\x91\x90a7\x7FV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP\x80\x80`\x01\x01\x91PPa(\xA6V[P\x82_\x015\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82` \x015\x81`\x07\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`@\x015\x81`\x08\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82``\x015\x81`\t\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x7F\xE5)j\x81\x84\xD1\x9A_\xD2EHt\x9E\xA3\xC45\xB6\x9A\xD2o\x12\xCA\n\xFA\x1E\x8E\xFE\xF5\x926\x8B\xF2\x86\x86\x86`@Qa.!\x93\x92\x91\x90aO_V[`@Q\x80\x91\x03\x90\xA2P\x93\x92PPPV[_\x80a.;a#\xA8V[\x90P`\x01`\xF8`\x07\x90\x1Ba.O\x91\x90aB\xA2V[\x83\x10\x15\x80\x15a.aWP\x80_\x01T\x83\x11\x15[\x80\x15a.\x83WP_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15[\x80\x15a.\xACWP\x80`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x91PP\x91\x90PV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a/\x10Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a/\x06Wa/\x05aJ0V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a/MWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a/CWa/BaJ0V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a/|Wf#\x86\xF2o\xC1\0\0\x83\x81a/rWa/qaJ0V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a/\xA5Wc\x05\xF5\xE1\0\x83\x81a/\x9BWa/\x9AaJ0V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a/\xCAWa'\x10\x83\x81a/\xC0Wa/\xBFaJ0V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a/\xEDW`d\x83\x81a/\xE3Wa/\xE2aJ0V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a/\xFCW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a01\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba1\xDDV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a0a\x82a1\xE6V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a0\xBDWa0\xB7\x82\x82a2\xAFV[Pa0\xC6V[a0\xC5a3/V[[PPV[a1\r`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83_\x015\x83a3kV[a1Q`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83` \x015\x83a3kV[a1\x95`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83`@\x015\x83a3kV[a1\xD9`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83``\x015\x83a3kV[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a2AW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a28\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x80a2m\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba1\xDDV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa2\xD8\x91\x90aO\xD3V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a3\x10W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a3\x15V[``\x91P[P\x91P\x91Pa3%\x85\x83\x83a4LV[\x92PPP\x92\x91PPV[_4\x11\x15a3iW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x82\x03a3\xAFW\x82`@Q\x7F6\xBF\xB6\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\xA6\x91\x90a6;V[`@Q\x80\x91\x03\x90\xFD[`\xFF\x80\x16\x82\x11\x15a3\xFEW\x82\x82`\xFF\x80\x16`@Q\x7F\"\xBAR\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\xF5\x93\x92\x91\x90aO\xE9V[`@Q\x80\x91\x03\x90\xFD[\x80\x82\x11\x15a4GW\x82\x82\x82`@Q\x7F\xCA\xA8\x14\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4>\x93\x92\x91\x90aO\xE9V[`@Q\x80\x91\x03\x90\xFD[PPPV[``\x82a4aWa4\\\x82a4\xD9V[a4\xD1V[_\x82Q\x14\x80\x15a4\x87WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a4\xC9W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xC0\x91\x90aC~V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa4\xD2V[[\x93\x92PPPV[_\x81Q\x11\x15a4\xEBW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x15\x15\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a5\xE8W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa5\xCDV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a6\r\x82a5\xB1V[a6\x17\x81\x85a5\xBBV[\x93Pa6'\x81\x85` \x86\x01a5\xCBV[a60\x81a5\xF3V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra6S\x81\x84a6\x03V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_\x81\x90P\x91\x90PV[a6~\x81a6lV[\x81\x14a6\x88W_\x80\xFD[PV[_\x815\x90Pa6\x99\x81a6uV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a6\xB4Wa6\xB3a6dV[[_a6\xC1\x84\x82\x85\x01a6\x8BV[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a6\xDE\x81a6\xCAV[\x82RPPV[_` \x82\x01\x90Pa6\xF7_\x83\x01\x84a6\xD5V[\x92\x91PPV[a7\x06\x81a6lV[\x82RPPV[_` \x82\x01\x90Pa7\x1F_\x83\x01\x84a6\xFDV[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a7N\x82a7%V[\x90P\x91\x90PV[a7^\x81a7DV[\x81\x14a7hW_\x80\xFD[PV[_\x815\x90Pa7y\x81a7UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a7\x94Wa7\x93a6dV[[_a7\xA1\x84\x82\x85\x01a7kV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a7\xC0Wa7\xBFa6dV[[_a7\xCD\x85\x82\x86\x01a6\x8BV[\x92PP` a7\xDE\x85\x82\x86\x01a7kV[\x91PP\x92P\x92\x90PV[a7\xF1\x81a7DV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a8\x11\x82a5\xB1V[a8\x1B\x81\x85a7\xF7V[\x93Pa8+\x81\x85` \x86\x01a5\xCBV[a84\x81a5\xF3V[\x84\x01\x91PP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa8T_\x86\x01\x82a7\xE8V[P` \x83\x01Qa8g` \x86\x01\x82a7\xE8V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra8\x7F\x82\x82a8\x07V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra8\x99\x82\x82a8\x07V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xBE\x81\x84a8?V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a9\x04\x82a5\xF3V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a9#Wa9\"a8\xCEV[[\x80`@RPPPV[_a95a6[V[\x90Pa9A\x82\x82a8\xFBV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a9`Wa9_a8\xCEV[[a9i\x82a5\xF3V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a9\x96a9\x91\x84a9FV[a9,V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a9\xB2Wa9\xB1a8\xCAV[[a9\xBD\x84\x82\x85a9vV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a9\xD9Wa9\xD8a8\xC6V[[\x815a9\xE9\x84\x82` \x86\x01a9\x84V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a:\x08Wa:\x07a6dV[[_a:\x15\x85\x82\x86\x01a7kV[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:6Wa:5a6hV[[a:B\x85\x82\x86\x01a9\xC5V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a:^\x81a:LV[\x82RPPV[_` \x82\x01\x90Pa:w_\x83\x01\x84a:UV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a:\x9AWa:\x99a8\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\xB7Wa:\xB6a:}V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a:\xD3Wa:\xD2a:\x81V[[\x92P\x92\x90PV[_\x80\xFD[_`\x80\x82\x84\x03\x12\x15a:\xF3Wa:\xF2a:\xDAV[[\x81\x90P\x92\x91PPV[_\x80_\x80`\xC0\x85\x87\x03\x12\x15a;\x14Wa;\x13a6dV[[_a;!\x87\x82\x88\x01a6\x8BV[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;BWa;Aa6hV[[a;N\x87\x82\x88\x01a:\x85V[\x93P\x93PP`@a;a\x87\x82\x88\x01a:\xDEV[\x91PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a;\xA1\x83\x83a7\xE8V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a;\xC3\x82a;mV[a;\xCD\x81\x85a;wV[\x93Pa;\xD8\x83a;\x87V[\x80_[\x83\x81\x10\x15a<\x08W\x81Qa;\xEF\x88\x82a;\x96V[\x97Pa;\xFA\x83a;\xADV[\x92PP`\x01\x81\x01\x90Pa;\xDBV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra<-\x81\x84a;\xB9V[\x90P\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a<Q\x81a<5V[\x82RPPV[a<`\x81a6\xCAV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[``\x82\x01_\x82\x01Qa<\xA3_\x85\x01\x82a<HV[P` \x82\x01Qa<\xB6` \x85\x01\x82a<HV[P`@\x82\x01Qa<\xC9`@\x85\x01\x82a<HV[PPPPV[_a<\xDA\x83\x83a<\x8FV[``\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a<\xFC\x82a<fV[a=\x06\x81\x85a<pV[\x93Pa=\x11\x83a<\x80V[\x80_[\x83\x81\x10\x15a=AW\x81Qa=(\x88\x82a<\xCFV[\x97Pa=3\x83a<\xE6V[\x92PP`\x01\x81\x01\x90Pa=\x14V[P\x85\x93PPPP\x92\x91PPV[_`\xA0\x83\x01_\x83\x01Qa=c_\x86\x01\x82a<HV[P` \x83\x01Qa=v` \x86\x01\x82a<HV[P`@\x83\x01Qa=\x89`@\x86\x01\x82a<WV[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra=\xA1\x82\x82a8\x07V[\x91PP`\x80\x83\x01Q\x84\x82\x03`\x80\x86\x01Ra=\xBB\x82\x82a<\xF2V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra=\xE0\x81\x84a=NV[\x90P\x92\x91PPV[_\x80_`\xA0\x84\x86\x03\x12\x15a=\xFFWa=\xFEa6dV[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x1CWa>\x1Ba6hV[[a>(\x86\x82\x87\x01a:\x85V[\x93P\x93PP` a>;\x86\x82\x87\x01a:\xDEV[\x91PP\x92P\x92P\x92V[_\x80\x83`\x1F\x84\x01\x12a>ZWa>Ya8\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>wWa>va:}V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a>\x93Wa>\x92a:\x81V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a>\xAFWa>\xAEa8\xC6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xCCWa>\xCBa:}V[[` \x83\x01\x91P\x83``\x82\x02\x83\x01\x11\x15a>\xE8Wa>\xE7a:\x81V[[\x92P\x92\x90PV[a>\xF8\x81a<5V[\x81\x14a?\x02W_\x80\xFD[PV[_\x815\x90Pa?\x13\x81a>\xEFV[\x92\x91PPV[_\x80_\x80_``\x86\x88\x03\x12\x15a?2Wa?1a6dV[[_\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?OWa?Na6hV[[a?[\x88\x82\x89\x01a>EV[\x95P\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a?~Wa?}a6hV[[a?\x8A\x88\x82\x89\x01a>\x9AV[\x93P\x93PP`@a?\x9D\x88\x82\x89\x01a?\x05V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`\x80\x83\x01_\x83\x01Qa?\xE8_\x86\x01\x82a7\xE8V[P` \x83\x01Qa?\xFB` \x86\x01\x82a7\xE8V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra@\x13\x82\x82a8\x07V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra@-\x82\x82a8\x07V[\x91PP\x80\x91PP\x92\x91PPV[_a@E\x83\x83a?\xD3V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@c\x82a?\xAAV[a@m\x81\x85a?\xB4V[\x93P\x83` \x82\x02\x85\x01a@\x7F\x85a?\xC4V[\x80_[\x85\x81\x10\x15a@\xBAW\x84\x84\x03\x89R\x81Qa@\x9B\x85\x82a@:V[\x94Pa@\xA6\x83a@MV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa@\x82V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra@\xE4\x81\x84a@YV[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_aA\0\x82a5\xB1V[aA\n\x81\x85a@\xECV[\x93PaA\x1A\x81\x85` \x86\x01a5\xCBV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aAZ`\x02\x83a@\xECV[\x91PaAe\x82aA&V[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aA\xA4`\x01\x83a@\xECV[\x91PaA\xAF\x82aApV[`\x01\x82\x01\x90P\x91\x90PV[_aA\xC5\x82\x87a@\xF6V[\x91PaA\xD0\x82aANV[\x91PaA\xDC\x82\x86a@\xF6V[\x91PaA\xE7\x82aA\x98V[\x91PaA\xF3\x82\x85a@\xF6V[\x91PaA\xFE\x82aA\x98V[\x91PaB\n\x82\x84a@\xF6V[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aB\\W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aBoWaBnaB\x18V[[P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aB\xAC\x82a6lV[\x91PaB\xB7\x83a6lV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aB\xCFWaB\xCEaBuV[[\x92\x91PPV[_aB\xDF\x82a6lV[\x91PaB\xEA\x83a6lV[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aC\x02WaC\x01aBuV[[\x92\x91PPV[aC\x11\x81a<5V[\x82RPPV[_` \x82\x01\x90PaC*_\x83\x01\x84aC\x08V[\x92\x91PPV[_\x81Q\x90PaC>\x81a7UV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aCYWaCXa6dV[[_aCf\x84\x82\x85\x01aC0V[\x91PP\x92\x91PPV[aCx\x81a7DV[\x82RPPV[_` \x82\x01\x90PaC\x91_\x83\x01\x84aCoV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aC\xD9WaC\xD8a6dV[[_aC\xE6\x84\x82\x85\x01a?\x05V[\x91PP\x92\x91PPV[_``\x82\x01\x90PaD\x02_\x83\x01\x86aC\x08V[aD\x0F` \x83\x01\x85aC\x08V[aD\x1C`@\x83\x01\x84aC\x08V[\x94\x93PPPPV[_aD.\x82a6lV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aD`WaD_aBuV[[`\x01\x82\x01\x90P\x91\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aD\xD1\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aD\x96V[aD\xDB\x86\x83aD\x96V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aE\x16aE\x11aE\x0C\x84a6lV[aD\xF3V[a6lV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aE/\x83aD\xFCV[aECaE;\x82aE\x1DV[\x84\x84TaD\xA2V[\x82UPPPPV[_\x90V[aEWaEKV[aEb\x81\x84\x84aE&V[PPPV[[\x81\x81\x10\x15aE\x85WaEz_\x82aEOV[`\x01\x81\x01\x90PaEhV[PPV[`\x1F\x82\x11\x15aE\xCAWaE\x9B\x81aDuV[aE\xA4\x84aD\x87V[\x81\x01` \x85\x10\x15aE\xB3W\x81\x90P[aE\xC7aE\xBF\x85aD\x87V[\x83\x01\x82aEgV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aE\xEA_\x19\x84`\x08\x02aE\xCFV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aF\x02\x83\x83aE\xDBV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aF\x1C\x83\x83aDkV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF5WaF4a8\xCEV[[aF?\x82TaBEV[aFJ\x82\x82\x85aE\x89V[_`\x1F\x83\x11`\x01\x81\x14aFwW_\x84\x15aFeW\x82\x87\x015\x90P[aFo\x85\x82aE\xF7V[\x86UPaF\xD6V[`\x1F\x19\x84\x16aF\x85\x86aDuV[_[\x82\x81\x10\x15aF\xACW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaF\x87V[\x86\x83\x10\x15aF\xC9W\x84\x89\x015aF\xC5`\x1F\x89\x16\x82aE\xDBV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x815aF\xEB\x81a>\xEFV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaG\x12\x84aF\xF4V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aGBaG=aG8\x84a<5V[aD\xF3V[a<5V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aG[\x82aG(V[aGnaGg\x82aGIV[\x83TaF\xFFV[\x82UPPPV[_\x81`@\x1B\x90P\x91\x90PV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0aG\x9C\x84aGuV[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[aG\xBB\x82aG(V[aG\xCEaG\xC7\x82aGIV[\x83TaG\x81V[\x82UPPPV[_\x81`\x80\x1B\x90P\x91\x90PV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aH\x04\x84aG\xD5V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[aH#\x82aG(V[aH6aH/\x82aGIV[\x83TaG\xE1V[\x82UPPPV[_\x81\x01_\x83\x01\x80aHM\x81aF\xDFV[\x90PaHY\x81\x84aGRV[PPP_\x81\x01` \x83\x01\x80aHm\x81aF\xDFV[\x90PaHy\x81\x84aG\xB2V[PPP_\x81\x01`@\x83\x01\x80aH\x8D\x81aF\xDFV[\x90PaH\x99\x81\x84aH\x1AV[PPPPPV[aH\xAA\x82\x82aH=V[PPV[_aH\xB9\x83\x85a5\xBBV[\x93PaH\xC6\x83\x85\x84a9vV[aH\xCF\x83a5\xF3V[\x84\x01\x90P\x93\x92PPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aI\x01` \x84\x01\x84a?\x05V[\x90P\x92\x91PPV[``\x82\x01aI\x19_\x83\x01\x83aH\xF3V[aI%_\x85\x01\x82a<HV[PaI3` \x83\x01\x83aH\xF3V[aI@` \x85\x01\x82a<HV[PaIN`@\x83\x01\x83aH\xF3V[aI[`@\x85\x01\x82a<HV[PPPPV[_aIl\x83\x83aI\tV[``\x83\x01\x90P\x92\x91PPV[_\x82\x90P\x92\x91PPV[_``\x82\x01\x90P\x91\x90PV[_aI\x99\x83\x85aH\xDAV[\x93PaI\xA4\x82aH\xEAV[\x80_[\x85\x81\x10\x15aI\xDCWaI\xB9\x82\x84aIxV[aI\xC3\x88\x82aIaV[\x97PaI\xCE\x83aI\x82V[\x92PP`\x01\x81\x01\x90PaI\xA7V[P\x85\x92PPP\x93\x92PPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaJ\x02\x81\x87\x89aH\xAEV[\x90P\x81\x81\x03` \x83\x01RaJ\x17\x81\x85\x87aI\x8EV[\x90PaJ&`@\x83\x01\x84aC\x08V[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[aJf\x81a:LV[\x81\x14aJpW_\x80\xFD[PV[_\x81Q\x90PaJ\x81\x81aJ]V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aJ\x9CWaJ\x9Ba6dV[[_aJ\xA9\x84\x82\x85\x01aJsV[\x91PP\x92\x91PPV[_`@\x82\x01\x90PaJ\xC5_\x83\x01\x85a6\xFDV[aJ\xD2` \x83\x01\x84a6\xFDV[\x93\x92PPPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12aK\0WaJ\xFFaJ\xD9V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815aK\x18\x81a7UV[\x80\x91PP\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFaK@\x84aF\xF4V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_aKpaKkaKf\x84a7%V[aD\xF3V[a7%V[\x90P\x91\x90PV[_aK\x81\x82aKVV[\x90P\x91\x90PV[_aK\x92\x82aKwV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aK\xAB\x82aK\x88V[aK\xBEaK\xB7\x82aK\x99V[\x83TaK!V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aK\xE1WaK\xE0aJ\xD9V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aL\x03WaL\x02aJ\xDDV[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15aL\x1FWaL\x1EaJ\xE1V[[P\x92P\x92\x90PV[aL2\x83\x83\x83aF\x12V[PPPV[_\x81\x01_\x83\x01\x80aLG\x81aK\x0CV[\x90PaLS\x81\x84aK\xA2V[PPP`\x01\x81\x01` \x83\x01\x80aLh\x81aK\x0CV[\x90PaLt\x81\x84aK\xA2V[PPP`\x02\x81\x01`@\x83\x01aL\x89\x81\x85aK\xC5V[aL\x94\x81\x83\x86aL'V[PPPP`\x03\x81\x01``\x83\x01aL\xAA\x81\x85aK\xC5V[aL\xB5\x81\x83\x86aL'V[PPPPPPV[aL\xC7\x82\x82aL7V[PPV[_\x81\x90P\x91\x90PV[_aL\xE2` \x84\x01\x84a7kV[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aM\x12WaM\x11aL\xF2V[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aM:WaM9aL\xEAV[[`\x01\x82\x026\x03\x83\x13\x15aMPWaMOaL\xEEV[[P\x92P\x92\x90PV[_aMc\x83\x85a7\xF7V[\x93PaMp\x83\x85\x84a9vV[aMy\x83a5\xF3V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x83\x01aM\x95_\x84\x01\x84aL\xD4V[aM\xA1_\x86\x01\x82a7\xE8V[PaM\xAF` \x84\x01\x84aL\xD4V[aM\xBC` \x86\x01\x82a7\xE8V[PaM\xCA`@\x84\x01\x84aL\xF6V[\x85\x83\x03`@\x87\x01RaM\xDD\x83\x82\x84aMXV[\x92PPPaM\xEE``\x84\x01\x84aL\xF6V[\x85\x83\x03``\x87\x01RaN\x01\x83\x82\x84aMXV[\x92PPP\x80\x91PP\x92\x91PPV[_aN\x1A\x83\x83aM\x84V[\x90P\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12aN=WaN<aL\xF2V[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aN`\x83\x85a?\xB4V[\x93P\x83` \x84\x02\x85\x01aNr\x84aL\xCBV[\x80_[\x87\x81\x10\x15aN\xB5W\x84\x84\x03\x89RaN\x8C\x82\x84aN\"V[aN\x96\x85\x82aN\x0FV[\x94PaN\xA1\x83aNIV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaNuV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_aN\xD5` \x84\x01\x84a6\x8BV[\x90P\x92\x91PPV[aN\xE6\x81a6lV[\x82RPPV[`\x80\x82\x01aN\xFC_\x83\x01\x83aN\xC7V[aO\x08_\x85\x01\x82aN\xDDV[PaO\x16` \x83\x01\x83aN\xC7V[aO#` \x85\x01\x82aN\xDDV[PaO1`@\x83\x01\x83aN\xC7V[aO>`@\x85\x01\x82aN\xDDV[PaOL``\x83\x01\x83aN\xC7V[aOY``\x85\x01\x82aN\xDDV[PPPPV[_`\xA0\x82\x01\x90P\x81\x81\x03_\x83\x01RaOx\x81\x85\x87aNUV[\x90PaO\x87` \x83\x01\x84aN\xECV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_aO\xAD\x82aO\x8FV[aO\xB7\x81\x85aO\x99V[\x93PaO\xC7\x81\x85` \x86\x01a5\xCBV[\x80\x84\x01\x91PP\x92\x91PPV[_aO\xDE\x82\x84aO\xA3V[\x91P\x81\x90P\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01RaP\x01\x81\x86a6\x03V[\x90PaP\x10` \x83\x01\x85a6\xFDV[aP\x1D`@\x83\x01\x84a6\xFDV[\x94\x93PPPPV",
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
struct CoprocessorContext { uint64 gwStartBlock; uint64 activatedAtBlock; bool destroyed; string softwareVersion; ChainUpgradeWindow[] chainUpgradeWindows; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CoprocessorContext {
        #[allow(missing_docs)]
        pub gwStartBlock: u64,
        #[allow(missing_docs)]
        pub activatedAtBlock: u64,
        #[allow(missing_docs)]
        pub destroyed: bool,
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainUpgradeWindows: alloy::sol_types::private::Vec<
            <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<ChainUpgradeWindow>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            u64,
            bool,
            alloy::sol_types::private::String,
            alloy::sol_types::private::Vec<
                <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<CoprocessorContext> for UnderlyingRustTuple<'_> {
            fn from(value: CoprocessorContext) -> Self {
                (
                    value.gwStartBlock,
                    value.activatedAtBlock,
                    value.destroyed,
                    value.softwareVersion,
                    value.chainUpgradeWindows,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CoprocessorContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    gwStartBlock: tuple.0,
                    activatedAtBlock: tuple.1,
                    destroyed: tuple.2,
                    softwareVersion: tuple.3,
                    chainUpgradeWindows: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CoprocessorContext {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CoprocessorContext {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.gwStartBlock),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.activatedAtBlock),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.destroyed,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.softwareVersion,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        ChainUpgradeWindow,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainUpgradeWindows),
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
        impl alloy_sol_types::SolType for CoprocessorContext {
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
        impl alloy_sol_types::SolStruct for CoprocessorContext {
            const NAME: &'static str = "CoprocessorContext";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CoprocessorContext(uint64 gwStartBlock,uint64 activatedAtBlock,bool destroyed,string softwareVersion,ChainUpgradeWindow[] chainUpgradeWindows)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <ChainUpgradeWindow as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <ChainUpgradeWindow as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.gwStartBlock)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.activatedAtBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.destroyed,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.softwareVersion,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        ChainUpgradeWindow,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.chainUpgradeWindows,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CoprocessorContext {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.gwStartBlock,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activatedAtBlock,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.destroyed,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.softwareVersion,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        ChainUpgradeWindow,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainUpgradeWindows,
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
                    &rust.gwStartBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activatedAtBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.destroyed,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.softwareVersion,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    ChainUpgradeWindow,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.chainUpgradeWindows,
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
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Custom error with signature `InvalidCoprocessorContext(uint256)` and selector `0x9797c3ff`.
```solidity
error InvalidCoprocessorContext(uint256 coprocessorContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCoprocessorContext {
        #[allow(missing_docs)]
        pub coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
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
        impl ::core::convert::From<InvalidCoprocessorContext>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCoprocessorContext) -> Self {
                (value.coprocessorContextId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidCoprocessorContext {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    coprocessorContextId: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCoprocessorContext {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCoprocessorContext(uint256)";
            const SELECTOR: [u8; 4] = [151u8, 151u8, 195u8, 255u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessorContextId),
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
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CoprocessorContextDestroyed(uint256)` and selector `0x6ed5f2c759f9fa25b478511dae2aa768dc993e9d04ab15f9c2519f075c4725d3`.
```solidity
event CoprocessorContextDestroyed(uint256 indexed coprocessorContextId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CoprocessorContextDestroyed {
        #[allow(missing_docs)]
        pub coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for CoprocessorContextDestroyed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "CoprocessorContextDestroyed(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                110u8, 213u8, 242u8, 199u8, 89u8, 249u8, 250u8, 37u8, 180u8, 120u8, 81u8,
                29u8, 174u8, 42u8, 167u8, 104u8, 220u8, 153u8, 62u8, 157u8, 4u8, 171u8,
                21u8, 249u8, 194u8, 81u8, 159u8, 7u8, 92u8, 71u8, 37u8, 211u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    coprocessorContextId: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.coprocessorContextId.clone())
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
                    &self.coprocessorContextId,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CoprocessorContextDestroyed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CoprocessorContextDestroyed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CoprocessorContextDestroyed,
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
    /**Event with signature `NewCoprocessorContext(uint256,string,(uint64,uint64,uint64)[],uint64)` and selector `0x595d10949fcf822de17e89ebc302566ed150171ff414fe14d92b78a6d3aecce8`.
```solidity
event NewCoprocessorContext(uint256 indexed coprocessorContextId, string softwareVersion, ChainUpgradeWindow[] chainUpgradeWindows, uint64 gwStartBlock);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewCoprocessorContext {
        #[allow(missing_docs)]
        pub coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NewCoprocessorContext {
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
            const SIGNATURE: &'static str = "NewCoprocessorContext(uint256,string,(uint64,uint64,uint64)[],uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                89u8, 93u8, 16u8, 148u8, 159u8, 207u8, 130u8, 45u8, 225u8, 126u8, 137u8,
                235u8, 195u8, 2u8, 86u8, 110u8, 209u8, 80u8, 23u8, 31u8, 244u8, 20u8,
                254u8, 20u8, 217u8, 43u8, 120u8, 166u8, 211u8, 174u8, 204u8, 232u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    coprocessorContextId: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.coprocessorContextId.clone())
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
                    &self.coprocessorContextId,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewCoprocessorContext {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewCoprocessorContext> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewCoprocessorContext) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NewKmsContext(uint256,(address,address,string,string)[],(uint256,uint256,uint256,uint256))` and selector `0xe5296a8184d19a5fd24548749ea3c435b69ad26f12ca0afa1e8efef592368bf2`.
```solidity
event NewKmsContext(uint256 indexed kmsContextId, KmsNode[] kmsNodes, IProtocolConfig.KmsThresholds thresholds);
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
        pub kmsContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<KmsNode>,
                IProtocolConfig::KmsThresholds,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "NewKmsContext(uint256,(address,address,string,string)[],(uint256,uint256,uint256,uint256))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                229u8, 41u8, 106u8, 129u8, 132u8, 209u8, 154u8, 95u8, 210u8, 69u8, 72u8,
                116u8, 158u8, 163u8, 196u8, 53u8, 182u8, 154u8, 210u8, 111u8, 18u8,
                202u8, 10u8, 250u8, 30u8, 142u8, 254u8, 245u8, 146u8, 54u8, 139u8, 242u8,
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
                    kmsNodes: data.0,
                    thresholds: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodes),
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
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
    /**Function with signature `defineNewCoprocessorContext(string,(uint64,uint64,uint64)[],uint64)` and selector `0xf76ca577`.
```solidity
function defineNewCoprocessorContext(string memory softwareVersion, ChainUpgradeWindow[] memory chainUpgradeWindows, uint64 gwStartBlock) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewCoprocessorContextCall {
        #[allow(missing_docs)]
        pub softwareVersion: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub chainUpgradeWindows: alloy::sol_types::private::Vec<
            <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub gwStartBlock: u64,
    }
    ///Container type for the return parameters of the [`defineNewCoprocessorContext(string,(uint64,uint64,uint64)[],uint64)`](defineNewCoprocessorContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewCoprocessorContextReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<ChainUpgradeWindow>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<defineNewCoprocessorContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewCoprocessorContextCall) -> Self {
                    (
                        value.softwareVersion,
                        value.chainUpgradeWindows,
                        value.gwStartBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewCoprocessorContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        softwareVersion: tuple.0,
                        chainUpgradeWindows: tuple.1,
                        gwStartBlock: tuple.2,
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
            impl ::core::convert::From<defineNewCoprocessorContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewCoprocessorContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewCoprocessorContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl defineNewCoprocessorContextReturn {
            fn _tokenize(
                &self,
            ) -> <defineNewCoprocessorContextCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defineNewCoprocessorContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<ChainUpgradeWindow>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defineNewCoprocessorContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defineNewCoprocessorContext(string,(uint64,uint64,uint64)[],uint64)";
            const SELECTOR: [u8; 4] = [247u8, 108u8, 165u8, 119u8];
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
                defineNewCoprocessorContextReturn::_tokenize(ret)
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
    /**Function with signature `defineNewKmsContext((address,address,string,string)[],(uint256,uint256,uint256,uint256))` and selector `0xa92c75cb`.
```solidity
function defineNewKmsContext(KmsNode[] memory kmsNodes, IProtocolConfig.KmsThresholds memory thresholds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewKmsContextCall {
        #[allow(missing_docs)]
        pub kmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`defineNewKmsContext((address,address,string,string)[],(uint256,uint256,uint256,uint256))`](defineNewKmsContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewKmsContextReturn {}
    #[allow(
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
                IProtocolConfig::KmsThresholds,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<defineNewKmsContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewKmsContextCall) -> Self {
                    (value.kmsNodes, value.thresholds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewKmsContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        kmsNodes: tuple.0,
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
            impl ::core::convert::From<defineNewKmsContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewKmsContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewKmsContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl defineNewKmsContextReturn {
            fn _tokenize(
                &self,
            ) -> <defineNewKmsContextCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defineNewKmsContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<KmsNode>,
                IProtocolConfig::KmsThresholds,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defineNewKmsContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defineNewKmsContext((address,address,string,string)[],(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [169u8, 44u8, 117u8, 203u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodes),
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.thresholds,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                defineNewKmsContextReturn::_tokenize(ret)
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
    /**Function with signature `destroyCoprocessorContext(uint256)` and selector `0xd740e402`.
```solidity
function destroyCoprocessorContext(uint256 coprocessorContextId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct destroyCoprocessorContextCall {
        #[allow(missing_docs)]
        pub coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`destroyCoprocessorContext(uint256)`](destroyCoprocessorContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct destroyCoprocessorContextReturn {}
    #[allow(
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
            impl ::core::convert::From<destroyCoprocessorContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: destroyCoprocessorContextCall) -> Self {
                    (value.coprocessorContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for destroyCoprocessorContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        coprocessorContextId: tuple.0,
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
            impl ::core::convert::From<destroyCoprocessorContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: destroyCoprocessorContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for destroyCoprocessorContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl destroyCoprocessorContextReturn {
            fn _tokenize(
                &self,
            ) -> <destroyCoprocessorContextCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for destroyCoprocessorContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = destroyCoprocessorContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "destroyCoprocessorContext(uint256)";
            const SELECTOR: [u8; 4] = [215u8, 64u8, 228u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessorContextId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                destroyCoprocessorContextReturn::_tokenize(ret)
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
    /**Function with signature `getCoprocessorContext(uint256)` and selector `0x9a7860e0`.
```solidity
function getCoprocessorContext(uint256 coprocessorContextId) external view returns (CoprocessorContext memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorContextCall {
        #[allow(missing_docs)]
        pub coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCoprocessorContext(uint256)`](getCoprocessorContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCoprocessorContextReturn {
        #[allow(missing_docs)]
        pub _0: <CoprocessorContext as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCoprocessorContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorContextCall) -> Self {
                    (value.coprocessorContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        coprocessorContextId: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (CoprocessorContext,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <CoprocessorContext as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCoprocessorContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCoprocessorContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCoprocessorContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCoprocessorContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <CoprocessorContext as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (CoprocessorContext,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCoprocessorContext(uint256)";
            const SELECTOR: [u8; 4] = [154u8, 120u8, 96u8, 224u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessorContextId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<CoprocessorContext as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCoprocessorContextReturn = r.into();
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
                        let r: getCoprocessorContextReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentCoprocessorContextId()` and selector `0x170a2981`.
```solidity
function getCurrentCoprocessorContextId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentCoprocessorContextIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentCoprocessorContextId()`](getCurrentCoprocessorContextIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentCoprocessorContextIdReturn {
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
            impl ::core::convert::From<getCurrentCoprocessorContextIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentCoprocessorContextIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentCoprocessorContextIdCall {
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
            impl ::core::convert::From<getCurrentCoprocessorContextIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentCoprocessorContextIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentCoprocessorContextIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentCoprocessorContextIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentCoprocessorContextId()";
            const SELECTOR: [u8; 4] = [23u8, 10u8, 41u8, 129u8];
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
                        let r: getCurrentCoprocessorContextIdReturn = r.into();
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
                        let r: getCurrentCoprocessorContextIdReturn = r.into();
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
    /**Function with signature `initializeFromEmptyProxy((address,address,string,string)[],(uint256,uint256,uint256,uint256))` and selector `0xd8f8392b`.
```solidity
function initializeFromEmptyProxy(KmsNode[] memory initialKmsNodes, IProtocolConfig.KmsThresholds memory initialThresholds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyCall {
        #[allow(missing_docs)]
        pub initialKmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub initialThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initializeFromEmptyProxy((address,address,string,string)[],(uint256,uint256,uint256,uint256))`](initializeFromEmptyProxyCall) function.
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
                alloy::sol_types::sol_data::Array<KmsNode>,
                IProtocolConfig::KmsThresholds,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <KmsNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeFromEmptyProxyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromEmptyProxyCall) -> Self {
                    (value.initialKmsNodes, value.initialThresholds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromEmptyProxyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialKmsNodes: tuple.0,
                        initialThresholds: tuple.1,
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
                alloy::sol_types::sol_data::Array<KmsNode>,
                IProtocolConfig::KmsThresholds,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromEmptyProxyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromEmptyProxy((address,address,string,string)[],(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [216u8, 248u8, 57u8, 43u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.initialKmsNodes),
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.initialThresholds,
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
    /**Function with signature `initializeFromMigration(uint256,(address,address,string,string)[],(uint256,uint256,uint256,uint256))` and selector `0x556ecafa`.
```solidity
function initializeFromMigration(uint256 existingContextId, KmsNode[] memory existingKmsNodes, IProtocolConfig.KmsThresholds memory existingThresholds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromMigrationCall {
        #[allow(missing_docs)]
        pub existingContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub existingKmsNodes: alloy::sol_types::private::Vec<
            <KmsNode as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub existingThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initializeFromMigration(uint256,(address,address,string,string)[],(uint256,uint256,uint256,uint256))`](initializeFromMigrationCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNode>,
                IProtocolConfig::KmsThresholds,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <KmsNode as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeFromMigrationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeFromMigrationCall) -> Self {
                    (
                        value.existingContextId,
                        value.existingKmsNodes,
                        value.existingThresholds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromMigrationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        existingContextId: tuple.0,
                        existingKmsNodes: tuple.1,
                        existingThresholds: tuple.2,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<KmsNode>,
                IProtocolConfig::KmsThresholds,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromMigrationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromMigration(uint256,(address,address,string,string)[],(uint256,uint256,uint256,uint256))";
            const SELECTOR: [u8; 4] = [85u8, 110u8, 202u8, 250u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.existingContextId),
                    <alloy::sol_types::sol_data::Array<
                        KmsNode,
                    > as alloy_sol_types::SolType>::tokenize(&self.existingKmsNodes),
                    <IProtocolConfig::KmsThresholds as alloy_sol_types::SolType>::tokenize(
                        &self.existingThresholds,
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
    /**Function with signature `isValidCoprocessorContext(uint256)` and selector `0x0e1887c9`.
```solidity
function isValidCoprocessorContext(uint256 coprocessorContextId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidCoprocessorContextCall {
        #[allow(missing_docs)]
        pub coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isValidCoprocessorContext(uint256)`](isValidCoprocessorContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidCoprocessorContextReturn {
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
            impl ::core::convert::From<isValidCoprocessorContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidCoprocessorContextCall) -> Self {
                    (value.coprocessorContextId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidCoprocessorContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        coprocessorContextId: tuple.0,
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
            impl ::core::convert::From<isValidCoprocessorContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidCoprocessorContextReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidCoprocessorContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidCoprocessorContextCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidCoprocessorContext(uint256)";
            const SELECTOR: [u8; 4] = [14u8, 24u8, 135u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessorContextId),
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
                        let r: isValidCoprocessorContextReturn = r.into();
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
                        let r: isValidCoprocessorContextReturn = r.into();
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        defineNewCoprocessorContext(defineNewCoprocessorContextCall),
        #[allow(missing_docs)]
        defineNewKmsContext(defineNewKmsContextCall),
        #[allow(missing_docs)]
        destroyCoprocessorContext(destroyCoprocessorContextCall),
        #[allow(missing_docs)]
        destroyKmsContext(destroyKmsContextCall),
        #[allow(missing_docs)]
        getCoprocessorContext(getCoprocessorContextCall),
        #[allow(missing_docs)]
        getCurrentCoprocessorContextId(getCurrentCoprocessorContextIdCall),
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
        initializeFromMigration(initializeFromMigrationCall),
        #[allow(missing_docs)]
        isKmsSigner(isKmsSignerCall),
        #[allow(missing_docs)]
        isKmsSignerForContext(isKmsSignerForContextCall),
        #[allow(missing_docs)]
        isKmsTxSenderForContext(isKmsTxSenderForContextCall),
        #[allow(missing_docs)]
        isValidCoprocessorContext(isValidCoprocessorContextCall),
        #[allow(missing_docs)]
        isValidKmsContext(isValidKmsContextCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV3(reinitializeV3Call),
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
            [13u8, 142u8, 110u8, 44u8],
            [14u8, 24u8, 135u8, 201u8],
            [23u8, 10u8, 41u8, 129u8],
            [32u8, 61u8, 1u8, 20u8],
            [38u8, 207u8, 93u8, 239u8],
            [40u8, 30u8, 139u8, 254u8],
            [42u8, 56u8, 137u8, 152u8],
            [49u8, 255u8, 65u8, 200u8],
            [65u8, 173u8, 6u8, 156u8],
            [70u8, 197u8, 187u8, 189u8],
            [71u8, 232u8, 34u8, 149u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [85u8, 110u8, 202u8, 250u8],
            [91u8, 255u8, 118u8, 217u8],
            [126u8, 170u8, 200u8, 242u8],
            [148u8, 71u8, 207u8, 212u8],
            [151u8, 111u8, 62u8, 185u8],
            [154u8, 120u8, 96u8, 224u8],
            [169u8, 44u8, 117u8, 203u8],
            [173u8, 60u8, 177u8, 204u8],
            [180u8, 114u8, 43u8, 196u8],
            [186u8, 194u8, 43u8, 184u8],
            [191u8, 155u8, 22u8, 200u8],
            [192u8, 174u8, 100u8, 247u8],
            [194u8, 180u8, 41u8, 134u8],
            [195u8, 170u8, 170u8, 90u8],
            [215u8, 64u8, 228u8, 2u8],
            [216u8, 248u8, 57u8, 43u8],
            [247u8, 108u8, 165u8, 119u8],
            [249u8, 198u8, 112u8, 195u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigCalls {
        const NAME: &'static str = "ProtocolConfigCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 31usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defineNewCoprocessorContext(_) => {
                    <defineNewCoprocessorContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defineNewKmsContext(_) => {
                    <defineNewKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::destroyCoprocessorContext(_) => {
                    <destroyCoprocessorContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::destroyKmsContext(_) => {
                    <destroyKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCoprocessorContext(_) => {
                    <getCoprocessorContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentCoprocessorContextId(_) => {
                    <getCurrentCoprocessorContextIdCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::initializeFromMigration(_) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isValidCoprocessorContext(_) => {
                    <isValidCoprocessorContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidKmsContext(_) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn isValidCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::isValidCoprocessorContext)
                    }
                    isValidCoprocessorContext
                },
                {
                    fn getCurrentCoprocessorContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentCoprocessorContextIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentCoprocessorContextId)
                    }
                    getCurrentCoprocessorContextId
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
                    fn initializeFromMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::initializeFromMigration)
                    }
                    initializeFromMigration
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
                    fn getCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCoprocessorContext)
                    }
                    getCoprocessorContext
                },
                {
                    fn defineNewKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewKmsContext)
                    }
                    defineNewKmsContext
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
                    fn destroyCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::destroyCoprocessorContext)
                    }
                    destroyCoprocessorContext
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
                    fn defineNewCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewCoprocessorContext)
                    }
                    defineNewCoprocessorContext
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
                    fn isValidCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <isValidCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::isValidCoprocessorContext)
                    }
                    isValidCoprocessorContext
                },
                {
                    fn getCurrentCoprocessorContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCurrentCoprocessorContextIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCurrentCoprocessorContextId)
                    }
                    getCurrentCoprocessorContextId
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
                    fn initializeFromMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::initializeFromMigration)
                    }
                    initializeFromMigration
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
                    fn getCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <getCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::getCoprocessorContext)
                    }
                    getCoprocessorContext
                },
                {
                    fn defineNewKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewKmsContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewKmsContext)
                    }
                    defineNewKmsContext
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
                    fn destroyCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <destroyCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::destroyCoprocessorContext)
                    }
                    destroyCoprocessorContext
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
                    fn defineNewCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigCalls> {
                        <defineNewCoprocessorContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigCalls::defineNewCoprocessorContext)
                    }
                    defineNewCoprocessorContext
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
                Self::defineNewCoprocessorContext(inner) => {
                    <defineNewCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::defineNewKmsContext(inner) => {
                    <defineNewKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::destroyCoprocessorContext(inner) => {
                    <destroyCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::destroyKmsContext(inner) => {
                    <destroyKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCoprocessorContext(inner) => {
                    <getCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentCoprocessorContextId(inner) => {
                    <getCurrentCoprocessorContextIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::initializeFromMigration(inner) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isValidCoprocessorContext(inner) => {
                    <isValidCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isValidKmsContext(inner) => {
                    <isValidKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::defineNewCoprocessorContext(inner) => {
                    <defineNewCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::defineNewKmsContext(inner) => {
                    <defineNewKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::destroyCoprocessorContext(inner) => {
                    <destroyCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getCoprocessorContext(inner) => {
                    <getCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentCoprocessorContextId(inner) => {
                    <getCurrentCoprocessorContextIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initializeFromMigration(inner) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isValidCoprocessorContext(inner) => {
                    <isValidCoprocessorContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CurrentKmsContextCannotBeDestroyed(CurrentKmsContextCannotBeDestroyed),
        #[allow(missing_docs)]
        DuplicateChainId(DuplicateChainId),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        EmptyChainUpgradeWindows(EmptyChainUpgradeWindows),
        #[allow(missing_docs)]
        EmptyKmsNodes(EmptyKmsNodes),
        #[allow(missing_docs)]
        EmptySoftwareVersion(EmptySoftwareVersion),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidBlockWindow(InvalidBlockWindow),
        #[allow(missing_docs)]
        InvalidCoprocessorContext(InvalidCoprocessorContext),
        #[allow(missing_docs)]
        InvalidHighThreshold(InvalidHighThreshold),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidKmsContext(InvalidKmsContext),
        #[allow(missing_docs)]
        InvalidNullThreshold(InvalidNullThreshold),
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
            [23u8, 211u8, 233u8, 72u8],
            [33u8, 191u8, 218u8, 16u8],
            [34u8, 186u8, 82u8, 219u8],
            [45u8, 236u8, 207u8, 77u8],
            [54u8, 191u8, 182u8, 14u8],
            [69u8, 149u8, 252u8, 226u8],
            [76u8, 156u8, 140u8, 227u8],
            [108u8, 103u8, 228u8, 112u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 221u8, 190u8, 129u8],
            [132u8, 102u8, 128u8, 74u8],
            [151u8, 151u8, 195u8, 255u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [181u8, 72u8, 145u8, 71u8],
            [190u8, 80u8, 80u8, 68u8],
            [200u8, 72u8, 133u8, 212u8],
            [202u8, 168u8, 20u8, 163u8],
            [209u8, 140u8, 79u8, 240u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [242u8, 25u8, 220u8, 14u8],
            [245u8, 26u8, 246u8, 187u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigErrors {
        const NAME: &'static str = "ProtocolConfigErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentKmsContextCannotBeDestroyed(_) => {
                    <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DuplicateChainId(_) => {
                    <DuplicateChainId as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyKmsNodes(_) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptySoftwareVersion(_) => {
                    <EmptySoftwareVersion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBlockWindow(_) => {
                    <InvalidBlockWindow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCoprocessorContext(_) => {
                    <InvalidCoprocessorContext as alloy_sol_types::SolError>::SELECTOR
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
                    fn InvalidCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidCoprocessorContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidCoprocessorContext)
                    }
                    InvalidCoprocessorContext
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
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ProtocolConfigErrors::FailedCall)
                    }
                    FailedCall
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
                    fn InvalidCoprocessorContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <InvalidCoprocessorContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::InvalidCoprocessorContext)
                    }
                    InvalidCoprocessorContext
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
                Self::DuplicateChainId(inner) => {
                    <DuplicateChainId as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyKmsNodes(inner) => {
                    <EmptyKmsNodes as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptySoftwareVersion(inner) => {
                    <EmptySoftwareVersion as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidCoprocessorContext(inner) => {
                    <InvalidCoprocessorContext as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidNullThreshold(inner) => {
                    <InvalidNullThreshold as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::CurrentKmsContextCannotBeDestroyed(inner) => {
                    <CurrentKmsContextCannotBeDestroyed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidBlockWindow(inner) => {
                    <InvalidBlockWindow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCoprocessorContext(inner) => {
                    <InvalidCoprocessorContext as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`ProtocolConfig`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigEvents {
        #[allow(missing_docs)]
        CoprocessorContextDestroyed(CoprocessorContextDestroyed),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KmsContextDestroyed(KmsContextDestroyed),
        #[allow(missing_docs)]
        NewCoprocessorContext(NewCoprocessorContext),
        #[allow(missing_docs)]
        NewKmsContext(NewKmsContext),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
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
                89u8, 93u8, 16u8, 148u8, 159u8, 207u8, 130u8, 45u8, 225u8, 126u8, 137u8,
                235u8, 195u8, 2u8, 86u8, 110u8, 209u8, 80u8, 23u8, 31u8, 244u8, 20u8,
                254u8, 20u8, 217u8, 43u8, 120u8, 166u8, 211u8, 174u8, 204u8, 232u8,
            ],
            [
                110u8, 213u8, 242u8, 199u8, 89u8, 249u8, 250u8, 37u8, 180u8, 120u8, 81u8,
                29u8, 174u8, 42u8, 167u8, 104u8, 220u8, 153u8, 62u8, 157u8, 4u8, 171u8,
                21u8, 249u8, 194u8, 81u8, 159u8, 7u8, 92u8, 71u8, 37u8, 211u8,
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
                218u8, 7u8, 93u8, 9u8, 25u8, 141u8, 32u8, 126u8, 58u8, 145u8, 141u8,
                75u8, 141u8, 252u8, 135u8, 223u8, 45u8, 96u8, 160u8, 11u8, 231u8, 3u8,
                253u8, 57u8, 234u8, 172u8, 144u8, 150u8, 45u8, 160u8, 183u8, 240u8,
            ],
            [
                229u8, 41u8, 106u8, 129u8, 132u8, 209u8, 154u8, 95u8, 210u8, 69u8, 72u8,
                116u8, 158u8, 163u8, 196u8, 53u8, 182u8, 154u8, 210u8, 111u8, 18u8,
                202u8, 10u8, 250u8, 30u8, 142u8, 254u8, 245u8, 146u8, 54u8, 139u8, 242u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ProtocolConfigEvents {
        const NAME: &'static str = "ProtocolConfigEvents";
        const COUNT: usize = 6usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <CoprocessorContextDestroyed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CoprocessorContextDestroyed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CoprocessorContextDestroyed)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
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
                    <NewCoprocessorContext as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NewCoprocessorContext as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewCoprocessorContext)
                }
                Some(<NewKmsContext as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewKmsContext as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewKmsContext)
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
                Self::CoprocessorContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KmsContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewCoprocessorContext(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewKmsContext(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CoprocessorContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KmsContextDestroyed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewCoprocessorContext(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewKmsContext(inner) => {
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
        ///Creates a new call builder for the [`defineNewCoprocessorContext`] function.
        pub fn defineNewCoprocessorContext(
            &self,
            softwareVersion: alloy::sol_types::private::String,
            chainUpgradeWindows: alloy::sol_types::private::Vec<
                <ChainUpgradeWindow as alloy::sol_types::SolType>::RustType,
            >,
            gwStartBlock: u64,
        ) -> alloy_contract::SolCallBuilder<&P, defineNewCoprocessorContextCall, N> {
            self.call_builder(
                &defineNewCoprocessorContextCall {
                    softwareVersion,
                    chainUpgradeWindows,
                    gwStartBlock,
                },
            )
        }
        ///Creates a new call builder for the [`defineNewKmsContext`] function.
        pub fn defineNewKmsContext(
            &self,
            kmsNodes: alloy::sol_types::private::Vec<
                <KmsNode as alloy::sol_types::SolType>::RustType,
            >,
            thresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, defineNewKmsContextCall, N> {
            self.call_builder(
                &defineNewKmsContextCall {
                    kmsNodes,
                    thresholds,
                },
            )
        }
        ///Creates a new call builder for the [`destroyCoprocessorContext`] function.
        pub fn destroyCoprocessorContext(
            &self,
            coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, destroyCoprocessorContextCall, N> {
            self.call_builder(
                &destroyCoprocessorContextCall {
                    coprocessorContextId,
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
        ///Creates a new call builder for the [`getCoprocessorContext`] function.
        pub fn getCoprocessorContext(
            &self,
            coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCoprocessorContextCall, N> {
            self.call_builder(
                &getCoprocessorContextCall {
                    coprocessorContextId,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentCoprocessorContextId`] function.
        pub fn getCurrentCoprocessorContextId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentCoprocessorContextIdCall, N> {
            self.call_builder(&getCurrentCoprocessorContextIdCall)
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
            initialKmsNodes: alloy::sol_types::private::Vec<
                <KmsNode as alloy::sol_types::SolType>::RustType,
            >,
            initialThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(
                &initializeFromEmptyProxyCall {
                    initialKmsNodes,
                    initialThresholds,
                },
            )
        }
        ///Creates a new call builder for the [`initializeFromMigration`] function.
        pub fn initializeFromMigration(
            &self,
            existingContextId: alloy::sol_types::private::primitives::aliases::U256,
            existingKmsNodes: alloy::sol_types::private::Vec<
                <KmsNode as alloy::sol_types::SolType>::RustType,
            >,
            existingThresholds: <IProtocolConfig::KmsThresholds as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromMigrationCall, N> {
            self.call_builder(
                &initializeFromMigrationCall {
                    existingContextId,
                    existingKmsNodes,
                    existingThresholds,
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
        ///Creates a new call builder for the [`isValidCoprocessorContext`] function.
        pub fn isValidCoprocessorContext(
            &self,
            coprocessorContextId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isValidCoprocessorContextCall, N> {
            self.call_builder(
                &isValidCoprocessorContextCall {
                    coprocessorContextId,
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
        ///Creates a new event filter for the [`CoprocessorContextDestroyed`] event.
        pub fn CoprocessorContextDestroyed_filter(
            &self,
        ) -> alloy_contract::Event<&P, CoprocessorContextDestroyed, N> {
            self.event_filter::<CoprocessorContextDestroyed>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`KmsContextDestroyed`] event.
        pub fn KmsContextDestroyed_filter(
            &self,
        ) -> alloy_contract::Event<&P, KmsContextDestroyed, N> {
            self.event_filter::<KmsContextDestroyed>()
        }
        ///Creates a new event filter for the [`NewCoprocessorContext`] event.
        pub fn NewCoprocessorContext_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewCoprocessorContext, N> {
            self.event_filter::<NewCoprocessorContext>()
        }
        ///Creates a new event filter for the [`NewKmsContext`] event.
        pub fn NewKmsContext_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewKmsContext, N> {
            self.event_filter::<NewKmsContext>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
