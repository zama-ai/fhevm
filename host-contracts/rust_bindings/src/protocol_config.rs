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
    struct KmsNode {
        address txSenderAddress;
        address signerAddress;
        string ipAddress;
        string storageUrl;
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
    error KeyManagementRequestInFlight();
    error KmsNodeNullSigner();
    error KmsNodeNullTxSender();
    error KmsSignerAlreadyRegistered(address signer);
    error KmsTxSenderAlreadyRegistered(address txSender);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event Initialized(uint64 version);
    event KmsContextDestroyed(uint256 indexed kmsContextId);
    event NewKmsContext(uint256 indexed kmsContextId, KmsNode[] kmsNodes, IProtocolConfig.KmsThresholds thresholds);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function defineNewKmsContext(KmsNode[] memory kmsNodes, IProtocolConfig.KmsThresholds memory thresholds) external;
    function destroyKmsContext(uint256 kmsContextId) external;
    function getCurrentKmsContextId() external view returns (uint256);
    function getKmsGenThreshold() external view returns (uint256);
    function getKmsNodeForContext(uint256 kmsContextId, address txSender) external view returns (KmsNode memory);
    function getKmsNodesForContext(uint256 kmsContextId) external view returns (KmsNode[] memory);
    function getKmsSignersForContext(uint256 kmsContextId) external view returns (address[] memory);
    function getMpcThreshold() external view returns (uint256);
    function getPublicDecryptionThreshold() external view returns (uint256);
    function getUserDecryptionThreshold() external view returns (uint256);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy(KmsNode[] memory initialKmsNodes, IProtocolConfig.KmsThresholds memory initialThresholds) external;
    function initializeFromMigration(uint256 existingContextId, KmsNode[] memory existingKmsNodes, IProtocolConfig.KmsThresholds memory existingThresholds) external;
    function isKmsSignerForContext(uint256 kmsContextId, address signer) external view returns (bool);
    function isKmsTxSenderForContext(uint256 kmsContextId, address txSender) external view returns (bool);
    function isValidKmsContext(uint256 kmsContextId) external view returns (bool);
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
    "name": "KeyManagementRequestInFlight",
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051613a2f620001eb5f395f8181611589015281816115de01526118800152613a2f5ff3fe608060405260043610610113575f3560e01c80639447cfd41161009f578063bf9b16c811610063578063bf9b16c814610399578063c0ae64f7146103d5578063c2b42986146103fd578063d8f8392b14610427578063f9c670c31461044f57610113565b80639447cfd4146102b7578063976f3eb9146102f3578063a92c75cb1461031d578063ad3cb1cc14610345578063b4722bc41461036f57610113565b806346c5bbbd116100e657806346c5bbbd146101d15780634f1ef2861461020d57806352d1902d14610229578063556ecafa146102535780635bff76d91461027b57610113565b80630d8e6e2c1461011757806326cf5def146101415780632a3889981461016b57806331ff41c814610195575b5f80fd5b348015610122575f80fd5b5061012b61048b565b60405161013891906126c2565b60405180910390f35b34801561014c575f80fd5b50610155610506565b60405161016291906126fa565b60405180910390f35b348015610176575f80fd5b5061017f61052f565b60405161018c91906126fa565b60405180910390f35b3480156101a0575f80fd5b506101bb60048036038101906101b691906127a8565b610558565b6040516101c891906128a4565b60405180910390f35b3480156101dc575f80fd5b506101f760048036038101906101f291906127a8565b61079a565b60405161020491906128de565b60405180910390f35b61022760048036038101906102229190612a23565b61080e565b005b348015610234575f80fd5b5061023d61082d565b60405161024a9190612a95565b60405180910390f35b34801561025e575f80fd5b5061027960048036038101906102749190612b2d565b61085e565b005b348015610286575f80fd5b506102a1600480360381019061029c9190612b9e565b610af9565b6040516102ae9190612c71565b60405180910390f35b3480156102c2575f80fd5b506102dd60048036038101906102d891906127a8565b610ba7565b6040516102ea91906128de565b60405180910390f35b3480156102fe575f80fd5b50610307610c1b565b60405161031491906126fa565b60405180910390f35b348015610328575f80fd5b50610343600480360381019061033e9190612c91565b610c2c565b005b348015610350575f80fd5b50610359610de5565b60405161036691906126c2565b60405180910390f35b34801561037a575f80fd5b50610383610e1e565b60405161039091906126fa565b60405180910390f35b3480156103a4575f80fd5b506103bf60048036038101906103ba9190612b9e565b610e47565b6040516103cc91906128de565b60405180910390f35b3480156103e0575f80fd5b506103fb60048036038101906103f69190612b9e565b610e58565b005b348015610408575f80fd5b50610411611040565b60405161041e91906126fa565b60405180910390f35b348015610432575f80fd5b5061044d60048036038101906104489190612c91565b611069565b005b34801561045a575f80fd5b5061047560048036038101906104709190612b9e565b611201565b6040516104829190612e10565b60405180910390f35b60606040518060400160405280600e81526020017f50726f746f636f6c436f6e6669670000000000000000000000000000000000008152506104cc5f611449565b6104d66001611449565b6104df5f611449565b6040516020016104f29493929190612efe565b604051602081830303815290604052905090565b5f80610510611513565b9050806009015f825f015481526020019081526020015f205491505090565b5f80610539611513565b9050806006015f825f015481526020019081526020015f205491505090565b6105606125e6565b6105698361153a565b610571611513565b6004015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461068290612f89565b80601f01602080910402602001604051908101604052809291908181526020018280546106ae90612f89565b80156106f95780601f106106d0576101008083540402835291602001916106f9565b820191905f5260205f20905b8154815290600101906020018083116106dc57829003601f168201915b5050505050815260200160038201805461071290612f89565b80601f016020809104026020016040519081016040528092919081815260200182805461073e90612f89565b80156107895780601f1061076057610100808354040283529160200191610789565b820191905f5260205f20905b81548152906001019060200180831161076c57829003601f168201915b505050505081525050905092915050565b5f6107a48361153a565b6107ac611513565b6002015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b610816611587565b61081f8261166d565b6108298282611760565b5050565b5f61083661187e565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108bb573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108df9190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461094e57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016109459190613007565b60405180910390fd5b60025f610959611905565b9050805f0160089054906101000a900460ff16806109a157508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156109d8576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550600160f86007901b610a2f919061304d565b861015610a7357856040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401610a6a91906126fa565b60405180910390fd5b5f610a7c611513565b9050600187610a8b9190613080565b815f0181905550610a9d86868661192c565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610ae991906130d5565b60405180910390a1505050505050565b6060610b048261153a565b610b0c611513565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610b9b57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610b52575b50505050509050919050565b5f610bb18361153a565b610bb9611513565b6003015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f610c24611513565b5f0154905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c89573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cad9190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610d1c57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610d139190613007565b60405180910390fd5b73216be43148db537beddbc268163deb1a802b555373ffffffffffffffffffffffffffffffffffffffff1663735b12636040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d79573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d9d9190613118565b15610dd4576040517f8e2bfd9000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610ddf83838361192c565b50505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80610e28611513565b9050806008015f825f015481526020019081526020015f205491505090565b5f610e5182611f25565b9050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610eb5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ed99190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f4857336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610f3f9190613007565b60405180910390fd5b5f610f51611513565b9050805f01548203610f9a57816040517f4595fce2000000000000000000000000000000000000000000000000000000008152600401610f9191906126fa565b60405180910390fd5b610fa382611f25565b610fe457816040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401610fdb91906126fa565b60405180910390fd5b600181600a015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550817fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f060405160405180910390a25050565b5f8061104a611513565b9050806007015f825f015481526020019081526020015f205491505090565b6001611073611fa8565b67ffffffffffffffff16146110b4576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f6110bf611905565b9050805f0160089054906101000a900460ff168061110757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b1561113e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f61118c611513565b905060f86007901b815f01819055506111a686868661192c565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516111f291906130d5565b60405180910390a15050505050565b606061120c8261153a565b611214611513565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b8282101561143e578382905f5260205f2090600402016040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461131f90612f89565b80601f016020809104026020016040519081016040528092919081815260200182805461134b90612f89565b80156113965780601f1061136d57610100808354040283529160200191611396565b820191905f5260205f20905b81548152906001019060200180831161137957829003601f168201915b505050505081526020016003820180546113af90612f89565b80601f01602080910402602001604051908101604052809291908181526020018280546113db90612f89565b80156114265780601f106113fd57610100808354040283529160200191611426565b820191905f5260205f20905b81548152906001019060200180831161140957829003601f168201915b50505050508152505081526020019060010190611245565b505050509050919050565b60605f600161145784611fcc565b0190505f8167ffffffffffffffff811115611475576114746128ff565b5b6040519080825280601f01601f1916602001820160405280156114a75781602001600182028036833780820191505090505b5090505f82602001820190505b600115611508578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816114fd576114fc613143565b5b0494505f85036114b4575b819350505050919050565b5f7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200905090565b61154381611f25565b61158457806040517f77ddbe8100000000000000000000000000000000000000000000000000000000815260040161157b91906126fa565b60405180910390fd5b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061163457507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1661161b61211d565b73ffffffffffffffffffffffffffffffffffffffff1614155b1561166b576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116ca573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116ee9190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461175d57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016117549190613007565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156117c857506040513d601f19601f820116820180604052508101906117c5919061319a565b60015b61180957816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016118009190613007565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461186f57806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016118669190612a95565b60405180910390fd5b6118798383612170565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614611903576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f808484905003611969576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61197682858590506121e2565b5f61197f611513565b9050805f015f8154611990906131c5565b91905081905591505f5b85859050811015611e7157368686838181106119b9576119b861320c565b5b90506020028101906119cb9190613245565b90505f73ffffffffffffffffffffffffffffffffffffffff16815f0160208101906119f6919061326c565b73ffffffffffffffffffffffffffffffffffffffff1603611a43576040517f8466804a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff16816020016020810190611a6d919061326c565b73ffffffffffffffffffffffffffffffffffffffff1603611aba576040517f2deccf4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826002015f8581526020019081526020015f205f825f016020810190611ae0919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611b7957805f016020810190611b3d919061326c565b6040517fd18c4ff0000000000000000000000000000000000000000000000000000000008152600401611b709190613007565b60405180910390fd5b826003015f8581526020019081526020015f205f826020016020810190611ba0919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611c3a57806020016020810190611bfe919061326c565b6040517ff51af6bb000000000000000000000000000000000000000000000000000000008152600401611c319190613007565b60405180910390fd5b826001015f8581526020019081526020015f2081908060018154018082558091505060019003905f5260205f2090600402015f909190919091508181611c8091906136c7565b50506001836002015f8681526020019081526020015f205f835f016020810190611caa919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506001836003015f8681526020019081526020015f205f836020016020810190611d22919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080836004015f8681526020019081526020015f205f835f016020810190611d98919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181611ddd91906136c7565b905050826005015f8581526020019081526020015f20816020016020810190611e06919061326c565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050808060010191505061199a565b50825f0135816006015f8481526020019081526020015f20819055508260200135816007015f8481526020019081526020015f20819055508260400135816008015f8481526020019081526020015f20819055508260600135816009015f8481526020019081526020015f2081905550817fe5296a8184d19a5fd24548749ea3c435b69ad26f12ca0afa1e8efef592368bf2868686604051611f1593929190613969565b60405180910390a2509392505050565b5f80611f2f611513565b9050600160f86007901b611f43919061304d565b8310158015611f555750805f01548311155b8015611f7757505f816001015f8581526020019081526020015f208054905014155b8015611fa0575080600a015f8481526020019081526020015f205f9054906101000a900460ff16155b915050919050565b5f611fb1611905565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612028577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161201e5761201d613143565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612065576d04ee2d6d415b85acef8100000000838161205b5761205a613143565b5b0492506020810190505b662386f26fc10000831061209457662386f26fc10000838161208a57612089613143565b5b0492506010810190505b6305f5e10083106120bd576305f5e10083816120b3576120b2613143565b5b0492506008810190505b61271083106120e25761271083816120d8576120d7613143565b5b0492506004810190505b6064831061210557606483816120fb576120fa613143565b5b0492506002810190505b600a8310612114576001810190505b80915050919050565b5f6121497f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6122f5565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612179826122fe565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156121d5576121cf82826123c7565b506121de565b6121dd612447565b5b5050565b6122256040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250835f013583612483565b6122696040518060400160405280600e81526020017f7573657244656372797074696f6e000000000000000000000000000000000000815250836020013583612483565b6122ad6040518060400160405280600681526020017f6b6d7347656e0000000000000000000000000000000000000000000000000000815250836040013583612483565b6122f16040518060400160405280600381526020017f6d70630000000000000000000000000000000000000000000000000000000000815250836060013583612483565b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b0361235957806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016123509190613007565b60405180910390fd5b806123857f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6122f5565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516123f091906139dd565b5f60405180830381855af49150503d805f8114612428576040519150601f19603f3d011682016040523d82523d5f602084013e61242d565b606091505b509150915061243d858383612515565b9250505092915050565b5f341115612481576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f82036124c757826040517f36bfb60e0000000000000000000000000000000000000000000000000000000081526004016124be91906126c2565b60405180910390fd5b80821115612510578282826040517fcaa814a3000000000000000000000000000000000000000000000000000000008152600401612507939291906139f3565b60405180910390fd5b505050565b60608261252a57612525826125a2565b61259a565b5f825114801561255057505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561259257836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016125899190613007565b60405180910390fd5b81905061259b565b5b9392505050565b5f815111156125b45780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561266f578082015181840152602081019050612654565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61269482612638565b61269e8185612642565b93506126ae818560208601612652565b6126b78161267a565b840191505092915050565b5f6020820190508181035f8301526126da818461268a565b905092915050565b5f819050919050565b6126f4816126e2565b82525050565b5f60208201905061270d5f8301846126eb565b92915050565b5f604051905090565b5f80fd5b5f80fd5b61272d816126e2565b8114612737575f80fd5b50565b5f8135905061274881612724565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6127778261274e565b9050919050565b6127878161276d565b8114612791575f80fd5b50565b5f813590506127a28161277e565b92915050565b5f80604083850312156127be576127bd61271c565b5b5f6127cb8582860161273a565b92505060206127dc85828601612794565b9150509250929050565b6127ef8161276d565b82525050565b5f82825260208201905092915050565b5f61280f82612638565b61281981856127f5565b9350612829818560208601612652565b6128328161267a565b840191505092915050565b5f608083015f8301516128525f8601826127e6565b50602083015161286560208601826127e6565b506040830151848203604086015261287d8282612805565b915050606083015184820360608601526128978282612805565b9150508091505092915050565b5f6020820190508181035f8301526128bc818461283d565b905092915050565b5f8115159050919050565b6128d8816128c4565b82525050565b5f6020820190506128f15f8301846128cf565b92915050565b5f80fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6129358261267a565b810181811067ffffffffffffffff82111715612954576129536128ff565b5b80604052505050565b5f612966612713565b9050612972828261292c565b919050565b5f67ffffffffffffffff821115612991576129906128ff565b5b61299a8261267a565b9050602081019050919050565b828183375f83830152505050565b5f6129c76129c284612977565b61295d565b9050828152602081018484840111156129e3576129e26128fb565b5b6129ee8482856129a7565b509392505050565b5f82601f830112612a0a57612a096128f7565b5b8135612a1a8482602086016129b5565b91505092915050565b5f8060408385031215612a3957612a3861271c565b5b5f612a4685828601612794565b925050602083013567ffffffffffffffff811115612a6757612a66612720565b5b612a73858286016129f6565b9150509250929050565b5f819050919050565b612a8f81612a7d565b82525050565b5f602082019050612aa85f830184612a86565b92915050565b5f80fd5b5f80fd5b5f8083601f840112612acb57612aca6128f7565b5b8235905067ffffffffffffffff811115612ae857612ae7612aae565b5b602083019150836020820283011115612b0457612b03612ab2565b5b9250929050565b5f80fd5b5f60808284031215612b2457612b23612b0b565b5b81905092915050565b5f805f8060c08587031215612b4557612b4461271c565b5b5f612b528782880161273a565b945050602085013567ffffffffffffffff811115612b7357612b72612720565b5b612b7f87828801612ab6565b93509350506040612b9287828801612b0f565b91505092959194509250565b5f60208284031215612bb357612bb261271c565b5b5f612bc08482850161273a565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f612bfd83836127e6565b60208301905092915050565b5f602082019050919050565b5f612c1f82612bc9565b612c298185612bd3565b9350612c3483612be3565b805f5b83811015612c64578151612c4b8882612bf2565b9750612c5683612c09565b925050600181019050612c37565b5085935050505092915050565b5f6020820190508181035f830152612c898184612c15565b905092915050565b5f805f60a08486031215612ca857612ca761271c565b5b5f84013567ffffffffffffffff811115612cc557612cc4612720565b5b612cd186828701612ab6565b93509350506020612ce486828701612b0f565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f608083015f830151612d2c5f8601826127e6565b506020830151612d3f60208601826127e6565b5060408301518482036040860152612d578282612805565b91505060608301518482036060860152612d718282612805565b9150508091505092915050565b5f612d898383612d17565b905092915050565b5f602082019050919050565b5f612da782612cee565b612db18185612cf8565b935083602082028501612dc385612d08565b805f5b85811015612dfe5784840389528151612ddf8582612d7e565b9450612dea83612d91565b925060208a01995050600181019050612dc6565b50829750879550505050505092915050565b5f6020820190508181035f830152612e288184612d9d565b905092915050565b5f81905092915050565b5f612e4482612638565b612e4e8185612e30565b9350612e5e818560208601612652565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f612e9e600283612e30565b9150612ea982612e6a565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f612ee8600183612e30565b9150612ef382612eb4565b600182019050919050565b5f612f098287612e3a565b9150612f1482612e92565b9150612f208286612e3a565b9150612f2b82612edc565b9150612f378285612e3a565b9150612f4282612edc565b9150612f4e8284612e3a565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680612fa057607f821691505b602082108103612fb357612fb2612f5c565b5b50919050565b5f81519050612fc78161277e565b92915050565b5f60208284031215612fe257612fe161271c565b5b5f612fef84828501612fb9565b91505092915050565b6130018161276d565b82525050565b5f60208201905061301a5f830184612ff8565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613057826126e2565b9150613062836126e2565b925082820190508082111561307a57613079613020565b5b92915050565b5f61308a826126e2565b9150613095836126e2565b92508282039050818111156130ad576130ac613020565b5b92915050565b5f67ffffffffffffffff82169050919050565b6130cf816130b3565b82525050565b5f6020820190506130e85f8301846130c6565b92915050565b6130f7816128c4565b8114613101575f80fd5b50565b5f81519050613112816130ee565b92915050565b5f6020828403121561312d5761312c61271c565b5b5f61313a84828501613104565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b61317981612a7d565b8114613183575f80fd5b50565b5f8151905061319481613170565b92915050565b5f602082840312156131af576131ae61271c565b5b5f6131bc84828501613186565b91505092915050565b5f6131cf826126e2565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361320157613200613020565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016080038336030381126132605761325f613239565b5b80830191505092915050565b5f602082840312156132815761328061271c565b5b5f61328e84828501612794565b91505092915050565b5f81356132a38161277e565b80915050919050565b5f815f1b9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff6132d6846132ac565b9350801983169250808416831791505092915050565b5f819050919050565b5f61330f61330a6133058461274e565b6132ec565b61274e565b9050919050565b5f613320826132f5565b9050919050565b5f61333182613316565b9050919050565b5f819050919050565b61334a82613327565b61335d61335682613338565b83546132b7565b8255505050565b5f80833560016020038436030381126133805761337f613239565b5b80840192508235915067ffffffffffffffff8211156133a2576133a161323d565b5b6020830192506001820236038313156133be576133bd613241565b5b509250929050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261342c7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826133f1565b61343686836133f1565b95508019841693508086168417925050509392505050565b5f61346861346361345e846126e2565b6132ec565b6126e2565b9050919050565b5f819050919050565b6134818361344e565b61349561348d8261346f565b8484546133fd565b825550505050565b5f90565b6134a961349d565b6134b4818484613478565b505050565b5b818110156134d7576134cc5f826134a1565b6001810190506134ba565b5050565b601f82111561351c576134ed816133d0565b6134f6846133e2565b81016020851015613505578190505b613519613511856133e2565b8301826134b9565b50505b505050565b5f82821c905092915050565b5f61353c5f1984600802613521565b1980831691505092915050565b5f613554838361352d565b9150826002028217905092915050565b61356e83836133c6565b67ffffffffffffffff811115613587576135866128ff565b5b6135918254612f89565b61359c8282856134db565b5f601f8311600181146135c9575f84156135b7578287013590505b6135c18582613549565b865550613628565b601f1984166135d7866133d0565b5f5b828110156135fe578489013582556001820191506020850194506020810190506135d9565b8683101561361b5784890135613617601f89168261352d565b8355505b6001600288020188555050505b50505050505050565b61363c838383613564565b505050565b5f81015f83018061365181613297565b905061365d8184613341565b50505060018101602083018061367281613297565b905061367e8184613341565b50505060028101604083016136938185613364565b61369e818386613631565b5050505060038101606083016136b48185613364565b6136bf818386613631565b505050505050565b6136d18282613641565b5050565b5f819050919050565b5f6136ec6020840184612794565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f808335600160200384360303811261371c5761371b6136fc565b5b83810192508235915060208301925067ffffffffffffffff821115613744576137436136f4565b5b60018202360383131561375a576137596136f8565b5b509250929050565b5f61376d83856127f5565b935061377a8385846129a7565b6137838361267a565b840190509392505050565b5f6080830161379f5f8401846136de565b6137ab5f8601826127e6565b506137b960208401846136de565b6137c660208601826127e6565b506137d46040840184613700565b85830360408701526137e7838284613762565b925050506137f86060840184613700565b858303606087015261380b838284613762565b925050508091505092915050565b5f613824838361378e565b905092915050565b5f82356001608003833603038112613847576138466136fc565b5b82810191505092915050565b5f602082019050919050565b5f61386a8385612cf8565b93508360208402850161387c846136d5565b805f5b878110156138bf578484038952613896828461382c565b6138a08582613819565b94506138ab83613853565b925060208a0199505060018101905061387f565b50829750879450505050509392505050565b5f6138df602084018461273a565b905092915050565b6138f0816126e2565b82525050565b608082016139065f8301836138d1565b6139125f8501826138e7565b5061392060208301836138d1565b61392d60208501826138e7565b5061393b60408301836138d1565b61394860408501826138e7565b5061395660608301836138d1565b61396360608501826138e7565b50505050565b5f60a0820190508181035f83015261398281858761385f565b905061399160208301846138f6565b949350505050565b5f81519050919050565b5f81905092915050565b5f6139b782613999565b6139c181856139a3565b93506139d1818560208601612652565b80840191505092915050565b5f6139e882846139ad565b915081905092915050565b5f6060820190508181035f830152613a0b818661268a565b9050613a1a60208301856126eb565b613a2760408301846126eb565b94935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80Qa:/b\0\x01\xEB_9_\x81\x81a\x15\x89\x01R\x81\x81a\x15\xDE\x01Ra\x18\x80\x01Ra:/_\xF3\xFE`\x80`@R`\x046\x10a\x01\x13W_5`\xE0\x1C\x80c\x94G\xCF\xD4\x11a\0\x9FW\x80c\xBF\x9B\x16\xC8\x11a\0cW\x80c\xBF\x9B\x16\xC8\x14a\x03\x99W\x80c\xC0\xAEd\xF7\x14a\x03\xD5W\x80c\xC2\xB4)\x86\x14a\x03\xFDW\x80c\xD8\xF89+\x14a\x04'W\x80c\xF9\xC6p\xC3\x14a\x04OWa\x01\x13V[\x80c\x94G\xCF\xD4\x14a\x02\xB7W\x80c\x97o>\xB9\x14a\x02\xF3W\x80c\xA9,u\xCB\x14a\x03\x1DW\x80c\xAD<\xB1\xCC\x14a\x03EW\x80c\xB4r+\xC4\x14a\x03oWa\x01\x13V[\x80cF\xC5\xBB\xBD\x11a\0\xE6W\x80cF\xC5\xBB\xBD\x14a\x01\xD1W\x80cO\x1E\xF2\x86\x14a\x02\rW\x80cR\xD1\x90-\x14a\x02)W\x80cUn\xCA\xFA\x14a\x02SW\x80c[\xFFv\xD9\x14a\x02{Wa\x01\x13V[\x80c\r\x8En,\x14a\x01\x17W\x80c&\xCF]\xEF\x14a\x01AW\x80c*8\x89\x98\x14a\x01kW\x80c1\xFFA\xC8\x14a\x01\x95W[_\x80\xFD[4\x80\x15a\x01\"W_\x80\xFD[Pa\x01+a\x04\x8BV[`@Qa\x018\x91\x90a&\xC2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01LW_\x80\xFD[Pa\x01Ua\x05\x06V[`@Qa\x01b\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01vW_\x80\xFD[Pa\x01\x7Fa\x05/V[`@Qa\x01\x8C\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA0W_\x80\xFD[Pa\x01\xBB`\x04\x806\x03\x81\x01\x90a\x01\xB6\x91\x90a'\xA8V[a\x05XV[`@Qa\x01\xC8\x91\x90a(\xA4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDCW_\x80\xFD[Pa\x01\xF7`\x04\x806\x03\x81\x01\x90a\x01\xF2\x91\x90a'\xA8V[a\x07\x9AV[`@Qa\x02\x04\x91\x90a(\xDEV[`@Q\x80\x91\x03\x90\xF3[a\x02'`\x04\x806\x03\x81\x01\x90a\x02\"\x91\x90a*#V[a\x08\x0EV[\0[4\x80\x15a\x024W_\x80\xFD[Pa\x02=a\x08-V[`@Qa\x02J\x91\x90a*\x95V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02^W_\x80\xFD[Pa\x02y`\x04\x806\x03\x81\x01\x90a\x02t\x91\x90a+-V[a\x08^V[\0[4\x80\x15a\x02\x86W_\x80\xFD[Pa\x02\xA1`\x04\x806\x03\x81\x01\x90a\x02\x9C\x91\x90a+\x9EV[a\n\xF9V[`@Qa\x02\xAE\x91\x90a,qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xC2W_\x80\xFD[Pa\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a'\xA8V[a\x0B\xA7V[`@Qa\x02\xEA\x91\x90a(\xDEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xFEW_\x80\xFD[Pa\x03\x07a\x0C\x1BV[`@Qa\x03\x14\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03(W_\x80\xFD[Pa\x03C`\x04\x806\x03\x81\x01\x90a\x03>\x91\x90a,\x91V[a\x0C,V[\0[4\x80\x15a\x03PW_\x80\xFD[Pa\x03Ya\r\xE5V[`@Qa\x03f\x91\x90a&\xC2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03zW_\x80\xFD[Pa\x03\x83a\x0E\x1EV[`@Qa\x03\x90\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xA4W_\x80\xFD[Pa\x03\xBF`\x04\x806\x03\x81\x01\x90a\x03\xBA\x91\x90a+\x9EV[a\x0EGV[`@Qa\x03\xCC\x91\x90a(\xDEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE0W_\x80\xFD[Pa\x03\xFB`\x04\x806\x03\x81\x01\x90a\x03\xF6\x91\x90a+\x9EV[a\x0EXV[\0[4\x80\x15a\x04\x08W_\x80\xFD[Pa\x04\x11a\x10@V[`@Qa\x04\x1E\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x042W_\x80\xFD[Pa\x04M`\x04\x806\x03\x81\x01\x90a\x04H\x91\x90a,\x91V[a\x10iV[\0[4\x80\x15a\x04ZW_\x80\xFD[Pa\x04u`\x04\x806\x03\x81\x01\x90a\x04p\x91\x90a+\x9EV[a\x12\x01V[`@Qa\x04\x82\x91\x90a.\x10V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FProtocolConfig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\xCC_a\x14IV[a\x04\xD6`\x01a\x14IV[a\x04\xDF_a\x14IV[`@Q` \x01a\x04\xF2\x94\x93\x92\x91\x90a.\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x80a\x05\x10a\x15\x13V[\x90P\x80`\t\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_\x80a\x059a\x15\x13V[\x90P\x80`\x06\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[a\x05`a%\xE6V[a\x05i\x83a\x15:V[a\x05qa\x15\x13V[`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x06\x82\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xAE\x90a/\x89V[\x80\x15a\x06\xF9W\x80`\x1F\x10a\x06\xD0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xF9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x07\x12\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07>\x90a/\x89V[\x80\x15a\x07\x89W\x80`\x1F\x10a\x07`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x89V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x92\x91PPV[_a\x07\xA4\x83a\x15:V[a\x07\xACa\x15\x13V[`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[a\x08\x16a\x15\x87V[a\x08\x1F\x82a\x16mV[a\x08)\x82\x82a\x17`V[PPV[_a\x086a\x18~V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xBBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDF\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tNW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\tE\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[`\x02_a\tYa\x19\x05V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\t\xA1WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\t\xD8W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\xF8`\x07\x90\x1Ba\n/\x91\x90a0MV[\x86\x10\x15a\nsW\x85`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nj\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[_a\n|a\x15\x13V[\x90P`\x01\x87a\n\x8B\x91\x90a0\x80V[\x81_\x01\x81\x90UPa\n\x9D\x86\x86\x86a\x19,V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\n\xE9\x91\x90a0\xD5V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[``a\x0B\x04\x82a\x15:V[a\x0B\x0Ca\x15\x13V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x9BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0BRW[PPPPP\x90P\x91\x90PV[_a\x0B\xB1\x83a\x15:V[a\x0B\xB9a\x15\x13V[`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x0C$a\x15\x13V[_\x01T\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x89W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAD\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r\x1CW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x13\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[s!k\xE41H\xDBS{\xED\xDB\xC2h\x16=\xEB\x1A\x80+USs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cs[\x12c`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\ryW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x9D\x91\x90a1\x18V[\x15a\r\xD4W`@Q\x7F\x8E+\xFD\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xDF\x83\x83\x83a\x19,V[PPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x0E(a\x15\x13V[\x90P\x80`\x08\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x0EQ\x82a\x1F%V[\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD9\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0FHW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F?\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[_a\x0FQa\x15\x13V[\x90P\x80_\x01T\x82\x03a\x0F\x9AW\x81`@Q\x7FE\x95\xFC\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x91\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[a\x0F\xA3\x82a\x1F%V[a\x0F\xE4W\x81`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xDB\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0`@Q`@Q\x80\x91\x03\x90\xA2PPV[_\x80a\x10Ja\x15\x13V[\x90P\x80`\x07\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[`\x01a\x10sa\x1F\xA8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10\xB4W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\x10\xBFa\x19\x05V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x11\x07WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x11>W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x11\x8Ca\x15\x13V[\x90P`\xF8`\x07\x90\x1B\x81_\x01\x81\x90UPa\x11\xA6\x86\x86\x86a\x19,V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x11\xF2\x91\x90a0\xD5V[`@Q\x80\x91\x03\x90\xA1PPPPPV[``a\x12\x0C\x82a\x15:V[a\x12\x14a\x15\x13V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x14>W\x83\x82\x90_R` _ \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x13\x1F\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13K\x90a/\x89V[\x80\x15a\x13\x96W\x80`\x1F\x10a\x13mWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\x96V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13yW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x13\xAF\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xDB\x90a/\x89V[\x80\x15a\x14&W\x80`\x1F\x10a\x13\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12EV[PPPP\x90P\x91\x90PV[``_`\x01a\x14W\x84a\x1F\xCCV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14uWa\x14ta(\xFFV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xA7W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a\x15\x08W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x14\xFDWa\x14\xFCa1CV[[\x04\x94P_\x85\x03a\x14\xB4W[\x81\x93PPPP\x91\x90PV[_\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90P\x90V[a\x15C\x81a\x1F%V[a\x15\x84W\x80`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15{\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x164WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x16\x1Ba!\x1DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x16kW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xEE\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17]W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17T\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x17\xC8WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC5\x91\x90a1\x9AV[`\x01[a\x18\tW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\0\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a\x18oW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18f\x91\x90a*\x95V[`@Q\x80\x91\x03\x90\xFD[a\x18y\x83\x83a!pV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\x03W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_\x80\x84\x84\x90P\x03a\x19iW`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19v\x82\x85\x85\x90Pa!\xE2V[_a\x19\x7Fa\x15\x13V[\x90P\x80_\x01_\x81Ta\x19\x90\x90a1\xC5V[\x91\x90P\x81\x90U\x91P_[\x85\x85\x90P\x81\x10\x15a\x1EqW6\x86\x86\x83\x81\x81\x10a\x19\xB9Wa\x19\xB8a2\x0CV[[\x90P` \x02\x81\x01\x90a\x19\xCB\x91\x90a2EV[\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01` \x81\x01\x90a\x19\xF6\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1ACW`@Q\x7F\x84f\x80J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01` \x81\x01\x90a\x1Am\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1A\xBAW`@Q\x7F-\xEC\xCFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82_\x01` \x81\x01\x90a\x1A\xE0\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1ByW\x80_\x01` \x81\x01\x90a\x1B=\x91\x90a2lV[`@Q\x7F\xD1\x8CO\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Bp\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x82`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82` \x01` \x81\x01\x90a\x1B\xA0\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1C:W\x80` \x01` \x81\x01\x90a\x1B\xFE\x91\x90a2lV[`@Q\x7F\xF5\x1A\xF6\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C1\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x04\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x1C\x80\x91\x90a6\xC7V[PP`\x01\x83`\x02\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a\x1C\xAA\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x83`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83` \x01` \x81\x01\x90a\x1D\"\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x83`\x04\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a\x1D\x98\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x1D\xDD\x91\x90a6\xC7V[\x90PP\x82`\x05\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81` \x01` \x81\x01\x90a\x1E\x06\x91\x90a2lV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP\x80\x80`\x01\x01\x91PPa\x19\x9AV[P\x82_\x015\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82` \x015\x81`\x07\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`@\x015\x81`\x08\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82``\x015\x81`\t\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x7F\xE5)j\x81\x84\xD1\x9A_\xD2EHt\x9E\xA3\xC45\xB6\x9A\xD2o\x12\xCA\n\xFA\x1E\x8E\xFE\xF5\x926\x8B\xF2\x86\x86\x86`@Qa\x1F\x15\x93\x92\x91\x90a9iV[`@Q\x80\x91\x03\x90\xA2P\x93\x92PPPV[_\x80a\x1F/a\x15\x13V[\x90P`\x01`\xF8`\x07\x90\x1Ba\x1FC\x91\x90a0MV[\x83\x10\x15\x80\x15a\x1FUWP\x80_\x01T\x83\x11\x15[\x80\x15a\x1FwWP_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15[\x80\x15a\x1F\xA0WP\x80`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x91PP\x91\x90PV[_a\x1F\xB1a\x19\x05V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a (Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a \x1EWa \x1Da1CV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a eWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a [Wa Za1CV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a \x94Wf#\x86\xF2o\xC1\0\0\x83\x81a \x8AWa \x89a1CV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a \xBDWc\x05\xF5\xE1\0\x83\x81a \xB3Wa \xB2a1CV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a \xE2Wa'\x10\x83\x81a \xD8Wa \xD7a1CV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a!\x05W`d\x83\x81a \xFBWa \xFAa1CV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a!\x14W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a!I\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\"\xF5V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a!y\x82a\"\xFEV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a!\xD5Wa!\xCF\x82\x82a#\xC7V[Pa!\xDEV[a!\xDDa$GV[[PPV[a\"%`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83_\x015\x83a$\x83V[a\"i`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83` \x015\x83a$\x83V[a\"\xAD`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83`@\x015\x83a$\x83V[a\"\xF1`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83``\x015\x83a$\x83V[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a#YW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#P\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x80a#\x85\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\"\xF5V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa#\xF0\x91\x90a9\xDDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a$(W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a$-V[``\x91P[P\x91P\x91Pa$=\x85\x83\x83a%\x15V[\x92PPP\x92\x91PPV[_4\x11\x15a$\x81W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x82\x03a$\xC7W\x82`@Q\x7F6\xBF\xB6\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xBE\x91\x90a&\xC2V[`@Q\x80\x91\x03\x90\xFD[\x80\x82\x11\x15a%\x10W\x82\x82\x82`@Q\x7F\xCA\xA8\x14\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\x07\x93\x92\x91\x90a9\xF3V[`@Q\x80\x91\x03\x90\xFD[PPPV[``\x82a%*Wa%%\x82a%\xA2V[a%\x9AV[_\x82Q\x14\x80\x15a%PWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a%\x92W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\x89\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa%\x9BV[[\x93\x92PPPV[_\x81Q\x11\x15a%\xB4W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a&oW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa&TV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a&\x94\x82a&8V[a&\x9E\x81\x85a&BV[\x93Pa&\xAE\x81\x85` \x86\x01a&RV[a&\xB7\x81a&zV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra&\xDA\x81\x84a&\x8AV[\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a&\xF4\x81a&\xE2V[\x82RPPV[_` \x82\x01\x90Pa'\r_\x83\x01\x84a&\xEBV[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[a'-\x81a&\xE2V[\x81\x14a'7W_\x80\xFD[PV[_\x815\x90Pa'H\x81a'$V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a'w\x82a'NV[\x90P\x91\x90PV[a'\x87\x81a'mV[\x81\x14a'\x91W_\x80\xFD[PV[_\x815\x90Pa'\xA2\x81a'~V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a'\xBEWa'\xBDa'\x1CV[[_a'\xCB\x85\x82\x86\x01a':V[\x92PP` a'\xDC\x85\x82\x86\x01a'\x94V[\x91PP\x92P\x92\x90PV[a'\xEF\x81a'mV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a(\x0F\x82a&8V[a(\x19\x81\x85a'\xF5V[\x93Pa()\x81\x85` \x86\x01a&RV[a(2\x81a&zV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa(R_\x86\x01\x82a'\xE6V[P` \x83\x01Qa(e` \x86\x01\x82a'\xE6V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra(}\x82\x82a(\x05V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra(\x97\x82\x82a(\x05V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra(\xBC\x81\x84a(=V[\x90P\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a(\xD8\x81a(\xC4V[\x82RPPV[_` \x82\x01\x90Pa(\xF1_\x83\x01\x84a(\xCFV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a)5\x82a&zV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a)TWa)Sa(\xFFV[[\x80`@RPPPV[_a)fa'\x13V[\x90Pa)r\x82\x82a),V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\x91Wa)\x90a(\xFFV[[a)\x9A\x82a&zV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a)\xC7a)\xC2\x84a)wV[a)]V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a)\xE3Wa)\xE2a(\xFBV[[a)\xEE\x84\x82\x85a)\xA7V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a*\nWa*\ta(\xF7V[[\x815a*\x1A\x84\x82` \x86\x01a)\xB5V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a*9Wa*8a'\x1CV[[_a*F\x85\x82\x86\x01a'\x94V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*gWa*fa' V[[a*s\x85\x82\x86\x01a)\xF6V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a*\x8F\x81a*}V[\x82RPPV[_` \x82\x01\x90Pa*\xA8_\x83\x01\x84a*\x86V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a*\xCBWa*\xCAa(\xF7V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xE8Wa*\xE7a*\xAEV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a+\x04Wa+\x03a*\xB2V[[\x92P\x92\x90PV[_\x80\xFD[_`\x80\x82\x84\x03\x12\x15a+$Wa+#a+\x0BV[[\x81\x90P\x92\x91PPV[_\x80_\x80`\xC0\x85\x87\x03\x12\x15a+EWa+Da'\x1CV[[_a+R\x87\x82\x88\x01a':V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+sWa+ra' V[[a+\x7F\x87\x82\x88\x01a*\xB6V[\x93P\x93PP`@a+\x92\x87\x82\x88\x01a+\x0FV[\x91PP\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a+\xB3Wa+\xB2a'\x1CV[[_a+\xC0\x84\x82\x85\x01a':V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a+\xFD\x83\x83a'\xE6V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a,\x1F\x82a+\xC9V[a,)\x81\x85a+\xD3V[\x93Pa,4\x83a+\xE3V[\x80_[\x83\x81\x10\x15a,dW\x81Qa,K\x88\x82a+\xF2V[\x97Pa,V\x83a,\tV[\x92PP`\x01\x81\x01\x90Pa,7V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra,\x89\x81\x84a,\x15V[\x90P\x92\x91PPV[_\x80_`\xA0\x84\x86\x03\x12\x15a,\xA8Wa,\xA7a'\x1CV[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xC5Wa,\xC4a' V[[a,\xD1\x86\x82\x87\x01a*\xB6V[\x93P\x93PP` a,\xE4\x86\x82\x87\x01a+\x0FV[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`\x80\x83\x01_\x83\x01Qa-,_\x86\x01\x82a'\xE6V[P` \x83\x01Qa-?` \x86\x01\x82a'\xE6V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra-W\x82\x82a(\x05V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra-q\x82\x82a(\x05V[\x91PP\x80\x91PP\x92\x91PPV[_a-\x89\x83\x83a-\x17V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a-\xA7\x82a,\xEEV[a-\xB1\x81\x85a,\xF8V[\x93P\x83` \x82\x02\x85\x01a-\xC3\x85a-\x08V[\x80_[\x85\x81\x10\x15a-\xFEW\x84\x84\x03\x89R\x81Qa-\xDF\x85\x82a-~V[\x94Pa-\xEA\x83a-\x91V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa-\xC6V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra.(\x81\x84a-\x9DV[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a.D\x82a&8V[a.N\x81\x85a.0V[\x93Pa.^\x81\x85` \x86\x01a&RV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a.\x9E`\x02\x83a.0V[\x91Pa.\xA9\x82a.jV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a.\xE8`\x01\x83a.0V[\x91Pa.\xF3\x82a.\xB4V[`\x01\x82\x01\x90P\x91\x90PV[_a/\t\x82\x87a.:V[\x91Pa/\x14\x82a.\x92V[\x91Pa/ \x82\x86a.:V[\x91Pa/+\x82a.\xDCV[\x91Pa/7\x82\x85a.:V[\x91Pa/B\x82a.\xDCV[\x91Pa/N\x82\x84a.:V[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a/\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a/\xB3Wa/\xB2a/\\V[[P\x91\x90PV[_\x81Q\x90Pa/\xC7\x81a'~V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a/\xE2Wa/\xE1a'\x1CV[[_a/\xEF\x84\x82\x85\x01a/\xB9V[\x91PP\x92\x91PPV[a0\x01\x81a'mV[\x82RPPV[_` \x82\x01\x90Pa0\x1A_\x83\x01\x84a/\xF8V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a0W\x82a&\xE2V[\x91Pa0b\x83a&\xE2V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a0zWa0ya0 V[[\x92\x91PPV[_a0\x8A\x82a&\xE2V[\x91Pa0\x95\x83a&\xE2V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a0\xADWa0\xACa0 V[[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a0\xCF\x81a0\xB3V[\x82RPPV[_` \x82\x01\x90Pa0\xE8_\x83\x01\x84a0\xC6V[\x92\x91PPV[a0\xF7\x81a(\xC4V[\x81\x14a1\x01W_\x80\xFD[PV[_\x81Q\x90Pa1\x12\x81a0\xEEV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a1-Wa1,a'\x1CV[[_a1:\x84\x82\x85\x01a1\x04V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[a1y\x81a*}V[\x81\x14a1\x83W_\x80\xFD[PV[_\x81Q\x90Pa1\x94\x81a1pV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a1\xAFWa1\xAEa'\x1CV[[_a1\xBC\x84\x82\x85\x01a1\x86V[\x91PP\x92\x91PPV[_a1\xCF\x82a&\xE2V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a2\x01Wa2\0a0 V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12a2`Wa2_a29V[[\x80\x83\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a2\x81Wa2\x80a'\x1CV[[_a2\x8E\x84\x82\x85\x01a'\x94V[\x91PP\x92\x91PPV[_\x815a2\xA3\x81a'~V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa2\xD6\x84a2\xACV[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a3\x0Fa3\na3\x05\x84a'NV[a2\xECV[a'NV[\x90P\x91\x90PV[_a3 \x82a2\xF5V[\x90P\x91\x90PV[_a31\x82a3\x16V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a3J\x82a3'V[a3]a3V\x82a38V[\x83Ta2\xB7V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a3\x80Wa3\x7Fa29V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a3\xA2Wa3\xA1a2=V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15a3\xBEWa3\xBDa2AV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a4,\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a3\xF1V[a46\x86\x83a3\xF1V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_a4ha4ca4^\x84a&\xE2V[a2\xECV[a&\xE2V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a4\x81\x83a4NV[a4\x95a4\x8D\x82a4oV[\x84\x84Ta3\xFDV[\x82UPPPPV[_\x90V[a4\xA9a4\x9DV[a4\xB4\x81\x84\x84a4xV[PPPV[[\x81\x81\x10\x15a4\xD7Wa4\xCC_\x82a4\xA1V[`\x01\x81\x01\x90Pa4\xBAV[PPV[`\x1F\x82\x11\x15a5\x1CWa4\xED\x81a3\xD0V[a4\xF6\x84a3\xE2V[\x81\x01` \x85\x10\x15a5\x05W\x81\x90P[a5\x19a5\x11\x85a3\xE2V[\x83\x01\x82a4\xB9V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a5<_\x19\x84`\x08\x02a5!V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a5T\x83\x83a5-V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a5n\x83\x83a3\xC6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x87Wa5\x86a(\xFFV[[a5\x91\x82Ta/\x89V[a5\x9C\x82\x82\x85a4\xDBV[_`\x1F\x83\x11`\x01\x81\x14a5\xC9W_\x84\x15a5\xB7W\x82\x87\x015\x90P[a5\xC1\x85\x82a5IV[\x86UPa6(V[`\x1F\x19\x84\x16a5\xD7\x86a3\xD0V[_[\x82\x81\x10\x15a5\xFEW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa5\xD9V[\x86\x83\x10\x15a6\x1BW\x84\x89\x015a6\x17`\x1F\x89\x16\x82a5-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[a6<\x83\x83\x83a5dV[PPPV[_\x81\x01_\x83\x01\x80a6Q\x81a2\x97V[\x90Pa6]\x81\x84a3AV[PPP`\x01\x81\x01` \x83\x01\x80a6r\x81a2\x97V[\x90Pa6~\x81\x84a3AV[PPP`\x02\x81\x01`@\x83\x01a6\x93\x81\x85a3dV[a6\x9E\x81\x83\x86a61V[PPPP`\x03\x81\x01``\x83\x01a6\xB4\x81\x85a3dV[a6\xBF\x81\x83\x86a61V[PPPPPPV[a6\xD1\x82\x82a6AV[PPV[_\x81\x90P\x91\x90PV[_a6\xEC` \x84\x01\x84a'\x94V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a7\x1CWa7\x1Ba6\xFCV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a7DWa7Ca6\xF4V[[`\x01\x82\x026\x03\x83\x13\x15a7ZWa7Ya6\xF8V[[P\x92P\x92\x90PV[_a7m\x83\x85a'\xF5V[\x93Pa7z\x83\x85\x84a)\xA7V[a7\x83\x83a&zV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x83\x01a7\x9F_\x84\x01\x84a6\xDEV[a7\xAB_\x86\x01\x82a'\xE6V[Pa7\xB9` \x84\x01\x84a6\xDEV[a7\xC6` \x86\x01\x82a'\xE6V[Pa7\xD4`@\x84\x01\x84a7\0V[\x85\x83\x03`@\x87\x01Ra7\xE7\x83\x82\x84a7bV[\x92PPPa7\xF8``\x84\x01\x84a7\0V[\x85\x83\x03``\x87\x01Ra8\x0B\x83\x82\x84a7bV[\x92PPP\x80\x91PP\x92\x91PPV[_a8$\x83\x83a7\x8EV[\x90P\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12a8GWa8Fa6\xFCV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a8j\x83\x85a,\xF8V[\x93P\x83` \x84\x02\x85\x01a8|\x84a6\xD5V[\x80_[\x87\x81\x10\x15a8\xBFW\x84\x84\x03\x89Ra8\x96\x82\x84a8,V[a8\xA0\x85\x82a8\x19V[\x94Pa8\xAB\x83a8SV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa8\x7FV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_a8\xDF` \x84\x01\x84a':V[\x90P\x92\x91PPV[a8\xF0\x81a&\xE2V[\x82RPPV[`\x80\x82\x01a9\x06_\x83\x01\x83a8\xD1V[a9\x12_\x85\x01\x82a8\xE7V[Pa9 ` \x83\x01\x83a8\xD1V[a9-` \x85\x01\x82a8\xE7V[Pa9;`@\x83\x01\x83a8\xD1V[a9H`@\x85\x01\x82a8\xE7V[Pa9V``\x83\x01\x83a8\xD1V[a9c``\x85\x01\x82a8\xE7V[PPPPV[_`\xA0\x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x82\x81\x85\x87a8_V[\x90Pa9\x91` \x83\x01\x84a8\xF6V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a9\xB7\x82a9\x99V[a9\xC1\x81\x85a9\xA3V[\x93Pa9\xD1\x81\x85` \x86\x01a&RV[\x80\x84\x01\x91PP\x92\x91PPV[_a9\xE8\x82\x84a9\xADV[\x91P\x81\x90P\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Ra:\x0B\x81\x86a&\x8AV[\x90Pa:\x1A` \x83\x01\x85a&\xEBV[a:'`@\x83\x01\x84a&\xEBV[\x94\x93PPPPV",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610113575f3560e01c80639447cfd41161009f578063bf9b16c811610063578063bf9b16c814610399578063c0ae64f7146103d5578063c2b42986146103fd578063d8f8392b14610427578063f9c670c31461044f57610113565b80639447cfd4146102b7578063976f3eb9146102f3578063a92c75cb1461031d578063ad3cb1cc14610345578063b4722bc41461036f57610113565b806346c5bbbd116100e657806346c5bbbd146101d15780634f1ef2861461020d57806352d1902d14610229578063556ecafa146102535780635bff76d91461027b57610113565b80630d8e6e2c1461011757806326cf5def146101415780632a3889981461016b57806331ff41c814610195575b5f80fd5b348015610122575f80fd5b5061012b61048b565b60405161013891906126c2565b60405180910390f35b34801561014c575f80fd5b50610155610506565b60405161016291906126fa565b60405180910390f35b348015610176575f80fd5b5061017f61052f565b60405161018c91906126fa565b60405180910390f35b3480156101a0575f80fd5b506101bb60048036038101906101b691906127a8565b610558565b6040516101c891906128a4565b60405180910390f35b3480156101dc575f80fd5b506101f760048036038101906101f291906127a8565b61079a565b60405161020491906128de565b60405180910390f35b61022760048036038101906102229190612a23565b61080e565b005b348015610234575f80fd5b5061023d61082d565b60405161024a9190612a95565b60405180910390f35b34801561025e575f80fd5b5061027960048036038101906102749190612b2d565b61085e565b005b348015610286575f80fd5b506102a1600480360381019061029c9190612b9e565b610af9565b6040516102ae9190612c71565b60405180910390f35b3480156102c2575f80fd5b506102dd60048036038101906102d891906127a8565b610ba7565b6040516102ea91906128de565b60405180910390f35b3480156102fe575f80fd5b50610307610c1b565b60405161031491906126fa565b60405180910390f35b348015610328575f80fd5b50610343600480360381019061033e9190612c91565b610c2c565b005b348015610350575f80fd5b50610359610de5565b60405161036691906126c2565b60405180910390f35b34801561037a575f80fd5b50610383610e1e565b60405161039091906126fa565b60405180910390f35b3480156103a4575f80fd5b506103bf60048036038101906103ba9190612b9e565b610e47565b6040516103cc91906128de565b60405180910390f35b3480156103e0575f80fd5b506103fb60048036038101906103f69190612b9e565b610e58565b005b348015610408575f80fd5b50610411611040565b60405161041e91906126fa565b60405180910390f35b348015610432575f80fd5b5061044d60048036038101906104489190612c91565b611069565b005b34801561045a575f80fd5b5061047560048036038101906104709190612b9e565b611201565b6040516104829190612e10565b60405180910390f35b60606040518060400160405280600e81526020017f50726f746f636f6c436f6e6669670000000000000000000000000000000000008152506104cc5f611449565b6104d66001611449565b6104df5f611449565b6040516020016104f29493929190612efe565b604051602081830303815290604052905090565b5f80610510611513565b9050806009015f825f015481526020019081526020015f205491505090565b5f80610539611513565b9050806006015f825f015481526020019081526020015f205491505090565b6105606125e6565b6105698361153a565b610571611513565b6004015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f206040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461068290612f89565b80601f01602080910402602001604051908101604052809291908181526020018280546106ae90612f89565b80156106f95780601f106106d0576101008083540402835291602001916106f9565b820191905f5260205f20905b8154815290600101906020018083116106dc57829003601f168201915b5050505050815260200160038201805461071290612f89565b80601f016020809104026020016040519081016040528092919081815260200182805461073e90612f89565b80156107895780601f1061076057610100808354040283529160200191610789565b820191905f5260205f20905b81548152906001019060200180831161076c57829003601f168201915b505050505081525050905092915050565b5f6107a48361153a565b6107ac611513565b6002015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b610816611587565b61081f8261166d565b6108298282611760565b5050565b5f61083661187e565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108bb573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108df9190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461094e57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016109459190613007565b60405180910390fd5b60025f610959611905565b9050805f0160089054906101000a900460ff16806109a157508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156109d8576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550600160f86007901b610a2f919061304d565b861015610a7357856040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401610a6a91906126fa565b60405180910390fd5b5f610a7c611513565b9050600187610a8b9190613080565b815f0181905550610a9d86868661192c565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610ae991906130d5565b60405180910390a1505050505050565b6060610b048261153a565b610b0c611513565b6005015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020018280548015610b9b57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610b52575b50505050509050919050565b5f610bb18361153a565b610bb9611513565b6003015f8481526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16905092915050565b5f610c24611513565b5f0154905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610c89573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cad9190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610d1c57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610d139190613007565b60405180910390fd5b73216be43148db537beddbc268163deb1a802b555373ffffffffffffffffffffffffffffffffffffffff1663735b12636040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d79573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d9d9190613118565b15610dd4576040517f8e2bfd9000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610ddf83838361192c565b50505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b5f80610e28611513565b9050806008015f825f015481526020019081526020015f205491505090565b5f610e5182611f25565b9050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610eb5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ed99190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f4857336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610f3f9190613007565b60405180910390fd5b5f610f51611513565b9050805f01548203610f9a57816040517f4595fce2000000000000000000000000000000000000000000000000000000008152600401610f9191906126fa565b60405180910390fd5b610fa382611f25565b610fe457816040517f77ddbe81000000000000000000000000000000000000000000000000000000008152600401610fdb91906126fa565b60405180910390fd5b600181600a015f8481526020019081526020015f205f6101000a81548160ff021916908315150217905550817fda075d09198d207e3a918d4b8dfc87df2d60a00be703fd39eaac90962da0b7f060405160405180910390a25050565b5f8061104a611513565b9050806007015f825f015481526020019081526020015f205491505090565b6001611073611fa8565b67ffffffffffffffff16146110b4576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60025f6110bf611905565b9050805f0160089054906101000a900460ff168061110757508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b1561113e576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f61118c611513565b905060f86007901b815f01819055506111a686868661192c565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516111f291906130d5565b60405180910390a15050505050565b606061120c8261153a565b611214611513565b6001015f8381526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b8282101561143e578382905f5260205f2090600402016040518060800160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001600182015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200160028201805461131f90612f89565b80601f016020809104026020016040519081016040528092919081815260200182805461134b90612f89565b80156113965780601f1061136d57610100808354040283529160200191611396565b820191905f5260205f20905b81548152906001019060200180831161137957829003601f168201915b505050505081526020016003820180546113af90612f89565b80601f01602080910402602001604051908101604052809291908181526020018280546113db90612f89565b80156114265780601f106113fd57610100808354040283529160200191611426565b820191905f5260205f20905b81548152906001019060200180831161140957829003601f168201915b50505050508152505081526020019060010190611245565b505050509050919050565b60605f600161145784611fcc565b0190505f8167ffffffffffffffff811115611475576114746128ff565b5b6040519080825280601f01601f1916602001820160405280156114a75781602001600182028036833780820191505090505b5090505f82602001820190505b600115611508578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816114fd576114fc613143565b5b0494505f85036114b4575b819350505050919050565b5f7f80f3585af86806c5774303b06c1ee640aa83b6ef3e45df49bb26c8524500c200905090565b61154381611f25565b61158457806040517f77ddbe8100000000000000000000000000000000000000000000000000000000815260040161157b91906126fa565b60405180910390fd5b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061163457507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1661161b61211d565b73ffffffffffffffffffffffffffffffffffffffff1614155b1561166b576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156116ca573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116ee9190612fcd565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461175d57336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016117549190613007565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156117c857506040513d601f19601f820116820180604052508101906117c5919061319a565b60015b61180957816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016118009190613007565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b811461186f57806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016118669190612a95565b60405180910390fd5b6118798383612170565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614611903576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f808484905003611969576040517f068c8d4000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61197682858590506121e2565b5f61197f611513565b9050805f015f8154611990906131c5565b91905081905591505f5b85859050811015611e7157368686838181106119b9576119b861320c565b5b90506020028101906119cb9190613245565b90505f73ffffffffffffffffffffffffffffffffffffffff16815f0160208101906119f6919061326c565b73ffffffffffffffffffffffffffffffffffffffff1603611a43576040517f8466804a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f73ffffffffffffffffffffffffffffffffffffffff16816020016020810190611a6d919061326c565b73ffffffffffffffffffffffffffffffffffffffff1603611aba576040517f2deccf4d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b826002015f8581526020019081526020015f205f825f016020810190611ae0919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611b7957805f016020810190611b3d919061326c565b6040517fd18c4ff0000000000000000000000000000000000000000000000000000000008152600401611b709190613007565b60405180910390fd5b826003015f8581526020019081526020015f205f826020016020810190611ba0919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611c3a57806020016020810190611bfe919061326c565b6040517ff51af6bb000000000000000000000000000000000000000000000000000000008152600401611c319190613007565b60405180910390fd5b826001015f8581526020019081526020015f2081908060018154018082558091505060019003905f5260205f2090600402015f909190919091508181611c8091906136c7565b50506001836002015f8681526020019081526020015f205f835f016020810190611caa919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055506001836003015f8681526020019081526020015f205f836020016020810190611d22919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555080836004015f8681526020019081526020015f205f835f016020810190611d98919061326c565b73ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f208181611ddd91906136c7565b905050826005015f8581526020019081526020015f20816020016020810190611e06919061326c565b908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050808060010191505061199a565b50825f0135816006015f8481526020019081526020015f20819055508260200135816007015f8481526020019081526020015f20819055508260400135816008015f8481526020019081526020015f20819055508260600135816009015f8481526020019081526020015f2081905550817fe5296a8184d19a5fd24548749ea3c435b69ad26f12ca0afa1e8efef592368bf2868686604051611f1593929190613969565b60405180910390a2509392505050565b5f80611f2f611513565b9050600160f86007901b611f43919061304d565b8310158015611f555750805f01548311155b8015611f7757505f816001015f8581526020019081526020015f208054905014155b8015611fa0575080600a015f8481526020019081526020015f205f9054906101000a900460ff16155b915050919050565b5f611fb1611905565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612028577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161201e5761201d613143565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612065576d04ee2d6d415b85acef8100000000838161205b5761205a613143565b5b0492506020810190505b662386f26fc10000831061209457662386f26fc10000838161208a57612089613143565b5b0492506010810190505b6305f5e10083106120bd576305f5e10083816120b3576120b2613143565b5b0492506008810190505b61271083106120e25761271083816120d8576120d7613143565b5b0492506004810190505b6064831061210557606483816120fb576120fa613143565b5b0492506002810190505b600a8310612114576001810190505b80915050919050565b5f6121497f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6122f5565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612179826122fe565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156121d5576121cf82826123c7565b506121de565b6121dd612447565b5b5050565b6122256040518060400160405280601081526020017f7075626c696344656372797074696f6e00000000000000000000000000000000815250835f013583612483565b6122696040518060400160405280600e81526020017f7573657244656372797074696f6e000000000000000000000000000000000000815250836020013583612483565b6122ad6040518060400160405280600681526020017f6b6d7347656e0000000000000000000000000000000000000000000000000000815250836040013583612483565b6122f16040518060400160405280600381526020017f6d70630000000000000000000000000000000000000000000000000000000000815250836060013583612483565b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b0361235957806040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016123509190613007565b60405180910390fd5b806123857f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6122f5565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516123f091906139dd565b5f60405180830381855af49150503d805f8114612428576040519150601f19603f3d011682016040523d82523d5f602084013e61242d565b606091505b509150915061243d858383612515565b9250505092915050565b5f341115612481576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f82036124c757826040517f36bfb60e0000000000000000000000000000000000000000000000000000000081526004016124be91906126c2565b60405180910390fd5b80821115612510578282826040517fcaa814a3000000000000000000000000000000000000000000000000000000008152600401612507939291906139f3565b60405180910390fd5b505050565b60608261252a57612525826125a2565b61259a565b5f825114801561255057505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561259257836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016125899190613007565b60405180910390fd5b81905061259b565b5b9392505050565b5f815111156125b45780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180608001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff16815260200160608152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b8381101561266f578082015181840152602081019050612654565b5f8484015250505050565b5f601f19601f8301169050919050565b5f61269482612638565b61269e8185612642565b93506126ae818560208601612652565b6126b78161267a565b840191505092915050565b5f6020820190508181035f8301526126da818461268a565b905092915050565b5f819050919050565b6126f4816126e2565b82525050565b5f60208201905061270d5f8301846126eb565b92915050565b5f604051905090565b5f80fd5b5f80fd5b61272d816126e2565b8114612737575f80fd5b50565b5f8135905061274881612724565b92915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6127778261274e565b9050919050565b6127878161276d565b8114612791575f80fd5b50565b5f813590506127a28161277e565b92915050565b5f80604083850312156127be576127bd61271c565b5b5f6127cb8582860161273a565b92505060206127dc85828601612794565b9150509250929050565b6127ef8161276d565b82525050565b5f82825260208201905092915050565b5f61280f82612638565b61281981856127f5565b9350612829818560208601612652565b6128328161267a565b840191505092915050565b5f608083015f8301516128525f8601826127e6565b50602083015161286560208601826127e6565b506040830151848203604086015261287d8282612805565b915050606083015184820360608601526128978282612805565b9150508091505092915050565b5f6020820190508181035f8301526128bc818461283d565b905092915050565b5f8115159050919050565b6128d8816128c4565b82525050565b5f6020820190506128f15f8301846128cf565b92915050565b5f80fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6129358261267a565b810181811067ffffffffffffffff82111715612954576129536128ff565b5b80604052505050565b5f612966612713565b9050612972828261292c565b919050565b5f67ffffffffffffffff821115612991576129906128ff565b5b61299a8261267a565b9050602081019050919050565b828183375f83830152505050565b5f6129c76129c284612977565b61295d565b9050828152602081018484840111156129e3576129e26128fb565b5b6129ee8482856129a7565b509392505050565b5f82601f830112612a0a57612a096128f7565b5b8135612a1a8482602086016129b5565b91505092915050565b5f8060408385031215612a3957612a3861271c565b5b5f612a4685828601612794565b925050602083013567ffffffffffffffff811115612a6757612a66612720565b5b612a73858286016129f6565b9150509250929050565b5f819050919050565b612a8f81612a7d565b82525050565b5f602082019050612aa85f830184612a86565b92915050565b5f80fd5b5f80fd5b5f8083601f840112612acb57612aca6128f7565b5b8235905067ffffffffffffffff811115612ae857612ae7612aae565b5b602083019150836020820283011115612b0457612b03612ab2565b5b9250929050565b5f80fd5b5f60808284031215612b2457612b23612b0b565b5b81905092915050565b5f805f8060c08587031215612b4557612b4461271c565b5b5f612b528782880161273a565b945050602085013567ffffffffffffffff811115612b7357612b72612720565b5b612b7f87828801612ab6565b93509350506040612b9287828801612b0f565b91505092959194509250565b5f60208284031215612bb357612bb261271c565b5b5f612bc08482850161273a565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f612bfd83836127e6565b60208301905092915050565b5f602082019050919050565b5f612c1f82612bc9565b612c298185612bd3565b9350612c3483612be3565b805f5b83811015612c64578151612c4b8882612bf2565b9750612c5683612c09565b925050600181019050612c37565b5085935050505092915050565b5f6020820190508181035f830152612c898184612c15565b905092915050565b5f805f60a08486031215612ca857612ca761271c565b5b5f84013567ffffffffffffffff811115612cc557612cc4612720565b5b612cd186828701612ab6565b93509350506020612ce486828701612b0f565b9150509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f608083015f830151612d2c5f8601826127e6565b506020830151612d3f60208601826127e6565b5060408301518482036040860152612d578282612805565b91505060608301518482036060860152612d718282612805565b9150508091505092915050565b5f612d898383612d17565b905092915050565b5f602082019050919050565b5f612da782612cee565b612db18185612cf8565b935083602082028501612dc385612d08565b805f5b85811015612dfe5784840389528151612ddf8582612d7e565b9450612dea83612d91565b925060208a01995050600181019050612dc6565b50829750879550505050505092915050565b5f6020820190508181035f830152612e288184612d9d565b905092915050565b5f81905092915050565b5f612e4482612638565b612e4e8185612e30565b9350612e5e818560208601612652565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f612e9e600283612e30565b9150612ea982612e6a565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f612ee8600183612e30565b9150612ef382612eb4565b600182019050919050565b5f612f098287612e3a565b9150612f1482612e92565b9150612f208286612e3a565b9150612f2b82612edc565b9150612f378285612e3a565b9150612f4282612edc565b9150612f4e8284612e3a565b915081905095945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680612fa057607f821691505b602082108103612fb357612fb2612f5c565b5b50919050565b5f81519050612fc78161277e565b92915050565b5f60208284031215612fe257612fe161271c565b5b5f612fef84828501612fb9565b91505092915050565b6130018161276d565b82525050565b5f60208201905061301a5f830184612ff8565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613057826126e2565b9150613062836126e2565b925082820190508082111561307a57613079613020565b5b92915050565b5f61308a826126e2565b9150613095836126e2565b92508282039050818111156130ad576130ac613020565b5b92915050565b5f67ffffffffffffffff82169050919050565b6130cf816130b3565b82525050565b5f6020820190506130e85f8301846130c6565b92915050565b6130f7816128c4565b8114613101575f80fd5b50565b5f81519050613112816130ee565b92915050565b5f6020828403121561312d5761312c61271c565b5b5f61313a84828501613104565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b61317981612a7d565b8114613183575f80fd5b50565b5f8151905061319481613170565b92915050565b5f602082840312156131af576131ae61271c565b5b5f6131bc84828501613186565b91505092915050565b5f6131cf826126e2565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361320157613200613020565b5b600182019050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f823560016080038336030381126132605761325f613239565b5b80830191505092915050565b5f602082840312156132815761328061271c565b5b5f61328e84828501612794565b91505092915050565b5f81356132a38161277e565b80915050919050565b5f815f1b9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff6132d6846132ac565b9350801983169250808416831791505092915050565b5f819050919050565b5f61330f61330a6133058461274e565b6132ec565b61274e565b9050919050565b5f613320826132f5565b9050919050565b5f61333182613316565b9050919050565b5f819050919050565b61334a82613327565b61335d61335682613338565b83546132b7565b8255505050565b5f80833560016020038436030381126133805761337f613239565b5b80840192508235915067ffffffffffffffff8211156133a2576133a161323d565b5b6020830192506001820236038313156133be576133bd613241565b5b509250929050565b5f82905092915050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f6008830261342c7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff826133f1565b61343686836133f1565b95508019841693508086168417925050509392505050565b5f61346861346361345e846126e2565b6132ec565b6126e2565b9050919050565b5f819050919050565b6134818361344e565b61349561348d8261346f565b8484546133fd565b825550505050565b5f90565b6134a961349d565b6134b4818484613478565b505050565b5b818110156134d7576134cc5f826134a1565b6001810190506134ba565b5050565b601f82111561351c576134ed816133d0565b6134f6846133e2565b81016020851015613505578190505b613519613511856133e2565b8301826134b9565b50505b505050565b5f82821c905092915050565b5f61353c5f1984600802613521565b1980831691505092915050565b5f613554838361352d565b9150826002028217905092915050565b61356e83836133c6565b67ffffffffffffffff811115613587576135866128ff565b5b6135918254612f89565b61359c8282856134db565b5f601f8311600181146135c9575f84156135b7578287013590505b6135c18582613549565b865550613628565b601f1984166135d7866133d0565b5f5b828110156135fe578489013582556001820191506020850194506020810190506135d9565b8683101561361b5784890135613617601f89168261352d565b8355505b6001600288020188555050505b50505050505050565b61363c838383613564565b505050565b5f81015f83018061365181613297565b905061365d8184613341565b50505060018101602083018061367281613297565b905061367e8184613341565b50505060028101604083016136938185613364565b61369e818386613631565b5050505060038101606083016136b48185613364565b6136bf818386613631565b505050505050565b6136d18282613641565b5050565b5f819050919050565b5f6136ec6020840184612794565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f808335600160200384360303811261371c5761371b6136fc565b5b83810192508235915060208301925067ffffffffffffffff821115613744576137436136f4565b5b60018202360383131561375a576137596136f8565b5b509250929050565b5f61376d83856127f5565b935061377a8385846129a7565b6137838361267a565b840190509392505050565b5f6080830161379f5f8401846136de565b6137ab5f8601826127e6565b506137b960208401846136de565b6137c660208601826127e6565b506137d46040840184613700565b85830360408701526137e7838284613762565b925050506137f86060840184613700565b858303606087015261380b838284613762565b925050508091505092915050565b5f613824838361378e565b905092915050565b5f82356001608003833603038112613847576138466136fc565b5b82810191505092915050565b5f602082019050919050565b5f61386a8385612cf8565b93508360208402850161387c846136d5565b805f5b878110156138bf578484038952613896828461382c565b6138a08582613819565b94506138ab83613853565b925060208a0199505060018101905061387f565b50829750879450505050509392505050565b5f6138df602084018461273a565b905092915050565b6138f0816126e2565b82525050565b608082016139065f8301836138d1565b6139125f8501826138e7565b5061392060208301836138d1565b61392d60208501826138e7565b5061393b60408301836138d1565b61394860408501826138e7565b5061395660608301836138d1565b61396360608501826138e7565b50505050565b5f60a0820190508181035f83015261398281858761385f565b905061399160208301846138f6565b949350505050565b5f81519050919050565b5f81905092915050565b5f6139b782613999565b6139c181856139a3565b93506139d1818560208601612652565b80840191505092915050565b5f6139e882846139ad565b915081905092915050565b5f6060820190508181035f830152613a0b818661268a565b9050613a1a60208301856126eb565b613a2760408301846126eb565b94935050505056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x13W_5`\xE0\x1C\x80c\x94G\xCF\xD4\x11a\0\x9FW\x80c\xBF\x9B\x16\xC8\x11a\0cW\x80c\xBF\x9B\x16\xC8\x14a\x03\x99W\x80c\xC0\xAEd\xF7\x14a\x03\xD5W\x80c\xC2\xB4)\x86\x14a\x03\xFDW\x80c\xD8\xF89+\x14a\x04'W\x80c\xF9\xC6p\xC3\x14a\x04OWa\x01\x13V[\x80c\x94G\xCF\xD4\x14a\x02\xB7W\x80c\x97o>\xB9\x14a\x02\xF3W\x80c\xA9,u\xCB\x14a\x03\x1DW\x80c\xAD<\xB1\xCC\x14a\x03EW\x80c\xB4r+\xC4\x14a\x03oWa\x01\x13V[\x80cF\xC5\xBB\xBD\x11a\0\xE6W\x80cF\xC5\xBB\xBD\x14a\x01\xD1W\x80cO\x1E\xF2\x86\x14a\x02\rW\x80cR\xD1\x90-\x14a\x02)W\x80cUn\xCA\xFA\x14a\x02SW\x80c[\xFFv\xD9\x14a\x02{Wa\x01\x13V[\x80c\r\x8En,\x14a\x01\x17W\x80c&\xCF]\xEF\x14a\x01AW\x80c*8\x89\x98\x14a\x01kW\x80c1\xFFA\xC8\x14a\x01\x95W[_\x80\xFD[4\x80\x15a\x01\"W_\x80\xFD[Pa\x01+a\x04\x8BV[`@Qa\x018\x91\x90a&\xC2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01LW_\x80\xFD[Pa\x01Ua\x05\x06V[`@Qa\x01b\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01vW_\x80\xFD[Pa\x01\x7Fa\x05/V[`@Qa\x01\x8C\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA0W_\x80\xFD[Pa\x01\xBB`\x04\x806\x03\x81\x01\x90a\x01\xB6\x91\x90a'\xA8V[a\x05XV[`@Qa\x01\xC8\x91\x90a(\xA4V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDCW_\x80\xFD[Pa\x01\xF7`\x04\x806\x03\x81\x01\x90a\x01\xF2\x91\x90a'\xA8V[a\x07\x9AV[`@Qa\x02\x04\x91\x90a(\xDEV[`@Q\x80\x91\x03\x90\xF3[a\x02'`\x04\x806\x03\x81\x01\x90a\x02\"\x91\x90a*#V[a\x08\x0EV[\0[4\x80\x15a\x024W_\x80\xFD[Pa\x02=a\x08-V[`@Qa\x02J\x91\x90a*\x95V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02^W_\x80\xFD[Pa\x02y`\x04\x806\x03\x81\x01\x90a\x02t\x91\x90a+-V[a\x08^V[\0[4\x80\x15a\x02\x86W_\x80\xFD[Pa\x02\xA1`\x04\x806\x03\x81\x01\x90a\x02\x9C\x91\x90a+\x9EV[a\n\xF9V[`@Qa\x02\xAE\x91\x90a,qV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xC2W_\x80\xFD[Pa\x02\xDD`\x04\x806\x03\x81\x01\x90a\x02\xD8\x91\x90a'\xA8V[a\x0B\xA7V[`@Qa\x02\xEA\x91\x90a(\xDEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xFEW_\x80\xFD[Pa\x03\x07a\x0C\x1BV[`@Qa\x03\x14\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03(W_\x80\xFD[Pa\x03C`\x04\x806\x03\x81\x01\x90a\x03>\x91\x90a,\x91V[a\x0C,V[\0[4\x80\x15a\x03PW_\x80\xFD[Pa\x03Ya\r\xE5V[`@Qa\x03f\x91\x90a&\xC2V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03zW_\x80\xFD[Pa\x03\x83a\x0E\x1EV[`@Qa\x03\x90\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xA4W_\x80\xFD[Pa\x03\xBF`\x04\x806\x03\x81\x01\x90a\x03\xBA\x91\x90a+\x9EV[a\x0EGV[`@Qa\x03\xCC\x91\x90a(\xDEV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xE0W_\x80\xFD[Pa\x03\xFB`\x04\x806\x03\x81\x01\x90a\x03\xF6\x91\x90a+\x9EV[a\x0EXV[\0[4\x80\x15a\x04\x08W_\x80\xFD[Pa\x04\x11a\x10@V[`@Qa\x04\x1E\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x042W_\x80\xFD[Pa\x04M`\x04\x806\x03\x81\x01\x90a\x04H\x91\x90a,\x91V[a\x10iV[\0[4\x80\x15a\x04ZW_\x80\xFD[Pa\x04u`\x04\x806\x03\x81\x01\x90a\x04p\x91\x90a+\x9EV[a\x12\x01V[`@Qa\x04\x82\x91\x90a.\x10V[`@Q\x80\x91\x03\x90\xF3[```@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FProtocolConfig\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\xCC_a\x14IV[a\x04\xD6`\x01a\x14IV[a\x04\xDF_a\x14IV[`@Q` \x01a\x04\xF2\x94\x93\x92\x91\x90a.\xFEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x80a\x05\x10a\x15\x13V[\x90P\x80`\t\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_\x80a\x059a\x15\x13V[\x90P\x80`\x06\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[a\x05`a%\xE6V[a\x05i\x83a\x15:V[a\x05qa\x15\x13V[`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x06\x82\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xAE\x90a/\x89V[\x80\x15a\x06\xF9W\x80`\x1F\x10a\x06\xD0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xF9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xDCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x07\x12\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07>\x90a/\x89V[\x80\x15a\x07\x89W\x80`\x1F\x10a\x07`Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x89V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07lW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x92\x91PPV[_a\x07\xA4\x83a\x15:V[a\x07\xACa\x15\x13V[`\x02\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[a\x08\x16a\x15\x87V[a\x08\x1F\x82a\x16mV[a\x08)\x82\x82a\x17`V[PPV[_a\x086a\x18~V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xBBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xDF\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\tNW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\tE\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[`\x02_a\tYa\x19\x05V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\t\xA1WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\t\xD8W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01`\xF8`\x07\x90\x1Ba\n/\x91\x90a0MV[\x86\x10\x15a\nsW\x85`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nj\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[_a\n|a\x15\x13V[\x90P`\x01\x87a\n\x8B\x91\x90a0\x80V[\x81_\x01\x81\x90UPa\n\x9D\x86\x86\x86a\x19,V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\n\xE9\x91\x90a0\xD5V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[``a\x0B\x04\x82a\x15:V[a\x0B\x0Ca\x15\x13V[`\x05\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x9BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0BRW[PPPPP\x90P\x91\x90PV[_a\x0B\xB1\x83a\x15:V[a\x0B\xB9a\x15\x13V[`\x03\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x92\x91PPV[_a\x0C$a\x15\x13V[_\x01T\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x89W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xAD\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\r\x1CW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r\x13\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[s!k\xE41H\xDBS{\xED\xDB\xC2h\x16=\xEB\x1A\x80+USs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cs[\x12c`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\ryW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x9D\x91\x90a1\x18V[\x15a\r\xD4W`@Q\x7F\x8E+\xFD\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xDF\x83\x83\x83a\x19,V[PPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[_\x80a\x0E(a\x15\x13V[\x90P\x80`\x08\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[_a\x0EQ\x82a\x1F%V[\x90P\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD9\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0FHW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F?\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[_a\x0FQa\x15\x13V[\x90P\x80_\x01T\x82\x03a\x0F\x9AW\x81`@Q\x7FE\x95\xFC\xE2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\x91\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[a\x0F\xA3\x82a\x1F%V[a\x0F\xE4W\x81`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xDB\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x81\x7F\xDA\x07]\t\x19\x8D ~:\x91\x8DK\x8D\xFC\x87\xDF-`\xA0\x0B\xE7\x03\xFD9\xEA\xAC\x90\x96-\xA0\xB7\xF0`@Q`@Q\x80\x91\x03\x90\xA2PPV[_\x80a\x10Ja\x15\x13V[\x90P\x80`\x07\x01_\x82_\x01T\x81R` \x01\x90\x81R` \x01_ T\x91PP\x90V[`\x01a\x10sa\x1F\xA8V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10\xB4W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02_a\x10\xBFa\x19\x05V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x11\x07WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x11>W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a\x11\x8Ca\x15\x13V[\x90P`\xF8`\x07\x90\x1B\x81_\x01\x81\x90UPa\x11\xA6\x86\x86\x86a\x19,V[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x11\xF2\x91\x90a0\xD5V[`@Q\x80\x91\x03\x90\xA1PPPPPV[``a\x12\x0C\x82a\x15:V[a\x12\x14a\x15\x13V[`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x14>W\x83\x82\x90_R` _ \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01`\x02\x82\x01\x80Ta\x13\x1F\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13K\x90a/\x89V[\x80\x15a\x13\x96W\x80`\x1F\x10a\x13mWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\x96V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13yW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x03\x82\x01\x80Ta\x13\xAF\x90a/\x89V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\xDB\x90a/\x89V[\x80\x15a\x14&W\x80`\x1F\x10a\x13\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12EV[PPPP\x90P\x91\x90PV[``_`\x01a\x14W\x84a\x1F\xCCV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14uWa\x14ta(\xFFV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xA7W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a\x15\x08W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x14\xFDWa\x14\xFCa1CV[[\x04\x94P_\x85\x03a\x14\xB4W[\x81\x93PPPP\x91\x90PV[_\x7F\x80\xF3XZ\xF8h\x06\xC5wC\x03\xB0l\x1E\xE6@\xAA\x83\xB6\xEF>E\xDFI\xBB&\xC8RE\0\xC2\0\x90P\x90V[a\x15C\x81a\x1F%V[a\x15\x84W\x80`@Q\x7Fw\xDD\xBE\x81\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15{\x91\x90a&\xFAV[`@Q\x80\x91\x03\x90\xFD[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x164WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x16\x1Ba!\x1DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x16kW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xEE\x91\x90a/\xCDV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x17]W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17T\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x17\xC8WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xC5\x91\x90a1\x9AV[`\x01[a\x18\tW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\0\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a\x18oW\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18f\x91\x90a*\x95V[`@Q\x80\x91\x03\x90\xFD[a\x18y\x83\x83a!pV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x19\x03W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_\x80\x84\x84\x90P\x03a\x19iW`@Q\x7F\x06\x8C\x8D@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x19v\x82\x85\x85\x90Pa!\xE2V[_a\x19\x7Fa\x15\x13V[\x90P\x80_\x01_\x81Ta\x19\x90\x90a1\xC5V[\x91\x90P\x81\x90U\x91P_[\x85\x85\x90P\x81\x10\x15a\x1EqW6\x86\x86\x83\x81\x81\x10a\x19\xB9Wa\x19\xB8a2\x0CV[[\x90P` \x02\x81\x01\x90a\x19\xCB\x91\x90a2EV[\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01` \x81\x01\x90a\x19\xF6\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1ACW`@Q\x7F\x84f\x80J\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01` \x81\x01\x90a\x1Am\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x1A\xBAW`@Q\x7F-\xEC\xCFM\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82_\x01` \x81\x01\x90a\x1A\xE0\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1ByW\x80_\x01` \x81\x01\x90a\x1B=\x91\x90a2lV[`@Q\x7F\xD1\x8CO\xF0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1Bp\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x82`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82` \x01` \x81\x01\x90a\x1B\xA0\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1C:W\x80` \x01` \x81\x01\x90a\x1B\xFE\x91\x90a2lV[`@Q\x7F\xF5\x1A\xF6\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C1\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x04\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x1C\x80\x91\x90a6\xC7V[PP`\x01\x83`\x02\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a\x1C\xAA\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP`\x01\x83`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83` \x01` \x81\x01\x90a\x1D\"\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x80\x83`\x04\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x83_\x01` \x81\x01\x90a\x1D\x98\x91\x90a2lV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81a\x1D\xDD\x91\x90a6\xC7V[\x90PP\x82`\x05\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81` \x01` \x81\x01\x90a\x1E\x06\x91\x90a2lV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP\x80\x80`\x01\x01\x91PPa\x19\x9AV[P\x82_\x015\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82` \x015\x81`\x07\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82`@\x015\x81`\x08\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x82``\x015\x81`\t\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x81\x7F\xE5)j\x81\x84\xD1\x9A_\xD2EHt\x9E\xA3\xC45\xB6\x9A\xD2o\x12\xCA\n\xFA\x1E\x8E\xFE\xF5\x926\x8B\xF2\x86\x86\x86`@Qa\x1F\x15\x93\x92\x91\x90a9iV[`@Q\x80\x91\x03\x90\xA2P\x93\x92PPPV[_\x80a\x1F/a\x15\x13V[\x90P`\x01`\xF8`\x07\x90\x1Ba\x1FC\x91\x90a0MV[\x83\x10\x15\x80\x15a\x1FUWP\x80_\x01T\x83\x11\x15[\x80\x15a\x1FwWP_\x81`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15[\x80\x15a\x1F\xA0WP\x80`\n\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x91PP\x91\x90PV[_a\x1F\xB1a\x19\x05V[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a (Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a \x1EWa \x1Da1CV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a eWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a [Wa Za1CV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a \x94Wf#\x86\xF2o\xC1\0\0\x83\x81a \x8AWa \x89a1CV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a \xBDWc\x05\xF5\xE1\0\x83\x81a \xB3Wa \xB2a1CV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a \xE2Wa'\x10\x83\x81a \xD8Wa \xD7a1CV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a!\x05W`d\x83\x81a \xFBWa \xFAa1CV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a!\x14W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a!I\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\"\xF5V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a!y\x82a\"\xFEV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a!\xD5Wa!\xCF\x82\x82a#\xC7V[Pa!\xDEV[a!\xDDa$GV[[PPV[a\"%`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FpublicDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83_\x015\x83a$\x83V[a\"i`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01\x7FuserDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83` \x015\x83a$\x83V[a\"\xAD`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01\x7FkmsGen\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83`@\x015\x83a$\x83V[a\"\xF1`@Q\x80`@\x01`@R\x80`\x03\x81R` \x01\x7Fmpc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x83``\x015\x83a$\x83V[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a#YW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a#P\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x80a#\x85\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba\"\xF5V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa#\xF0\x91\x90a9\xDDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a$(W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a$-V[``\x91P[P\x91P\x91Pa$=\x85\x83\x83a%\x15V[\x92PPP\x92\x91PPV[_4\x11\x15a$\x81W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x82\x03a$\xC7W\x82`@Q\x7F6\xBF\xB6\x0E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xBE\x91\x90a&\xC2V[`@Q\x80\x91\x03\x90\xFD[\x80\x82\x11\x15a%\x10W\x82\x82\x82`@Q\x7F\xCA\xA8\x14\xA3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\x07\x93\x92\x91\x90a9\xF3V[`@Q\x80\x91\x03\x90\xFD[PPPV[``\x82a%*Wa%%\x82a%\xA2V[a%\x9AV[_\x82Q\x14\x80\x15a%PWP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a%\x92W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\x89\x91\x90a0\x07V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa%\x9BV[[\x93\x92PPPV[_\x81Q\x11\x15a%\xB4W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01``\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a&oW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa&TV[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a&\x94\x82a&8V[a&\x9E\x81\x85a&BV[\x93Pa&\xAE\x81\x85` \x86\x01a&RV[a&\xB7\x81a&zV[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra&\xDA\x81\x84a&\x8AV[\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[a&\xF4\x81a&\xE2V[\x82RPPV[_` \x82\x01\x90Pa'\r_\x83\x01\x84a&\xEBV[\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[a'-\x81a&\xE2V[\x81\x14a'7W_\x80\xFD[PV[_\x815\x90Pa'H\x81a'$V[\x92\x91PPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a'w\x82a'NV[\x90P\x91\x90PV[a'\x87\x81a'mV[\x81\x14a'\x91W_\x80\xFD[PV[_\x815\x90Pa'\xA2\x81a'~V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a'\xBEWa'\xBDa'\x1CV[[_a'\xCB\x85\x82\x86\x01a':V[\x92PP` a'\xDC\x85\x82\x86\x01a'\x94V[\x91PP\x92P\x92\x90PV[a'\xEF\x81a'mV[\x82RPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a(\x0F\x82a&8V[a(\x19\x81\x85a'\xF5V[\x93Pa()\x81\x85` \x86\x01a&RV[a(2\x81a&zV[\x84\x01\x91PP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa(R_\x86\x01\x82a'\xE6V[P` \x83\x01Qa(e` \x86\x01\x82a'\xE6V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra(}\x82\x82a(\x05V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra(\x97\x82\x82a(\x05V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra(\xBC\x81\x84a(=V[\x90P\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a(\xD8\x81a(\xC4V[\x82RPPV[_` \x82\x01\x90Pa(\xF1_\x83\x01\x84a(\xCFV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a)5\x82a&zV[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a)TWa)Sa(\xFFV[[\x80`@RPPPV[_a)fa'\x13V[\x90Pa)r\x82\x82a),V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\x91Wa)\x90a(\xFFV[[a)\x9A\x82a&zV[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a)\xC7a)\xC2\x84a)wV[a)]V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a)\xE3Wa)\xE2a(\xFBV[[a)\xEE\x84\x82\x85a)\xA7V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a*\nWa*\ta(\xF7V[[\x815a*\x1A\x84\x82` \x86\x01a)\xB5V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a*9Wa*8a'\x1CV[[_a*F\x85\x82\x86\x01a'\x94V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*gWa*fa' V[[a*s\x85\x82\x86\x01a)\xF6V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a*\x8F\x81a*}V[\x82RPPV[_` \x82\x01\x90Pa*\xA8_\x83\x01\x84a*\x86V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a*\xCBWa*\xCAa(\xF7V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a*\xE8Wa*\xE7a*\xAEV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a+\x04Wa+\x03a*\xB2V[[\x92P\x92\x90PV[_\x80\xFD[_`\x80\x82\x84\x03\x12\x15a+$Wa+#a+\x0BV[[\x81\x90P\x92\x91PPV[_\x80_\x80`\xC0\x85\x87\x03\x12\x15a+EWa+Da'\x1CV[[_a+R\x87\x82\x88\x01a':V[\x94PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+sWa+ra' V[[a+\x7F\x87\x82\x88\x01a*\xB6V[\x93P\x93PP`@a+\x92\x87\x82\x88\x01a+\x0FV[\x91PP\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a+\xB3Wa+\xB2a'\x1CV[[_a+\xC0\x84\x82\x85\x01a':V[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a+\xFD\x83\x83a'\xE6V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a,\x1F\x82a+\xC9V[a,)\x81\x85a+\xD3V[\x93Pa,4\x83a+\xE3V[\x80_[\x83\x81\x10\x15a,dW\x81Qa,K\x88\x82a+\xF2V[\x97Pa,V\x83a,\tV[\x92PP`\x01\x81\x01\x90Pa,7V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra,\x89\x81\x84a,\x15V[\x90P\x92\x91PPV[_\x80_`\xA0\x84\x86\x03\x12\x15a,\xA8Wa,\xA7a'\x1CV[[_\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\xC5Wa,\xC4a' V[[a,\xD1\x86\x82\x87\x01a*\xB6V[\x93P\x93PP` a,\xE4\x86\x82\x87\x01a+\x0FV[\x91PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_`\x80\x83\x01_\x83\x01Qa-,_\x86\x01\x82a'\xE6V[P` \x83\x01Qa-?` \x86\x01\x82a'\xE6V[P`@\x83\x01Q\x84\x82\x03`@\x86\x01Ra-W\x82\x82a(\x05V[\x91PP``\x83\x01Q\x84\x82\x03``\x86\x01Ra-q\x82\x82a(\x05V[\x91PP\x80\x91PP\x92\x91PPV[_a-\x89\x83\x83a-\x17V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a-\xA7\x82a,\xEEV[a-\xB1\x81\x85a,\xF8V[\x93P\x83` \x82\x02\x85\x01a-\xC3\x85a-\x08V[\x80_[\x85\x81\x10\x15a-\xFEW\x84\x84\x03\x89R\x81Qa-\xDF\x85\x82a-~V[\x94Pa-\xEA\x83a-\x91V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa-\xC6V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra.(\x81\x84a-\x9DV[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a.D\x82a&8V[a.N\x81\x85a.0V[\x93Pa.^\x81\x85` \x86\x01a&RV[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a.\x9E`\x02\x83a.0V[\x91Pa.\xA9\x82a.jV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a.\xE8`\x01\x83a.0V[\x91Pa.\xF3\x82a.\xB4V[`\x01\x82\x01\x90P\x91\x90PV[_a/\t\x82\x87a.:V[\x91Pa/\x14\x82a.\x92V[\x91Pa/ \x82\x86a.:V[\x91Pa/+\x82a.\xDCV[\x91Pa/7\x82\x85a.:V[\x91Pa/B\x82a.\xDCV[\x91Pa/N\x82\x84a.:V[\x91P\x81\x90P\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a/\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a/\xB3Wa/\xB2a/\\V[[P\x91\x90PV[_\x81Q\x90Pa/\xC7\x81a'~V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a/\xE2Wa/\xE1a'\x1CV[[_a/\xEF\x84\x82\x85\x01a/\xB9V[\x91PP\x92\x91PPV[a0\x01\x81a'mV[\x82RPPV[_` \x82\x01\x90Pa0\x1A_\x83\x01\x84a/\xF8V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a0W\x82a&\xE2V[\x91Pa0b\x83a&\xE2V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a0zWa0ya0 V[[\x92\x91PPV[_a0\x8A\x82a&\xE2V[\x91Pa0\x95\x83a&\xE2V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a0\xADWa0\xACa0 V[[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a0\xCF\x81a0\xB3V[\x82RPPV[_` \x82\x01\x90Pa0\xE8_\x83\x01\x84a0\xC6V[\x92\x91PPV[a0\xF7\x81a(\xC4V[\x81\x14a1\x01W_\x80\xFD[PV[_\x81Q\x90Pa1\x12\x81a0\xEEV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a1-Wa1,a'\x1CV[[_a1:\x84\x82\x85\x01a1\x04V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[a1y\x81a*}V[\x81\x14a1\x83W_\x80\xFD[PV[_\x81Q\x90Pa1\x94\x81a1pV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a1\xAFWa1\xAEa'\x1CV[[_a1\xBC\x84\x82\x85\x01a1\x86V[\x91PP\x92\x91PPV[_a1\xCF\x82a&\xE2V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03a2\x01Wa2\0a0 V[[`\x01\x82\x01\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12a2`Wa2_a29V[[\x80\x83\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a2\x81Wa2\x80a'\x1CV[[_a2\x8E\x84\x82\x85\x01a'\x94V[\x91PP\x92\x91PPV[_\x815a2\xA3\x81a'~V[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFa2\xD6\x84a2\xACV[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a3\x0Fa3\na3\x05\x84a'NV[a2\xECV[a'NV[\x90P\x91\x90PV[_a3 \x82a2\xF5V[\x90P\x91\x90PV[_a31\x82a3\x16V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a3J\x82a3'V[a3]a3V\x82a38V[\x83Ta2\xB7V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a3\x80Wa3\x7Fa29V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a3\xA2Wa3\xA1a2=V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15a3\xBEWa3\xBDa2AV[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a4,\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a3\xF1V[a46\x86\x83a3\xF1V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_a4ha4ca4^\x84a&\xE2V[a2\xECV[a&\xE2V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a4\x81\x83a4NV[a4\x95a4\x8D\x82a4oV[\x84\x84Ta3\xFDV[\x82UPPPPV[_\x90V[a4\xA9a4\x9DV[a4\xB4\x81\x84\x84a4xV[PPPV[[\x81\x81\x10\x15a4\xD7Wa4\xCC_\x82a4\xA1V[`\x01\x81\x01\x90Pa4\xBAV[PPV[`\x1F\x82\x11\x15a5\x1CWa4\xED\x81a3\xD0V[a4\xF6\x84a3\xE2V[\x81\x01` \x85\x10\x15a5\x05W\x81\x90P[a5\x19a5\x11\x85a3\xE2V[\x83\x01\x82a4\xB9V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a5<_\x19\x84`\x08\x02a5!V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a5T\x83\x83a5-V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a5n\x83\x83a3\xC6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x87Wa5\x86a(\xFFV[[a5\x91\x82Ta/\x89V[a5\x9C\x82\x82\x85a4\xDBV[_`\x1F\x83\x11`\x01\x81\x14a5\xC9W_\x84\x15a5\xB7W\x82\x87\x015\x90P[a5\xC1\x85\x82a5IV[\x86UPa6(V[`\x1F\x19\x84\x16a5\xD7\x86a3\xD0V[_[\x82\x81\x10\x15a5\xFEW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa5\xD9V[\x86\x83\x10\x15a6\x1BW\x84\x89\x015a6\x17`\x1F\x89\x16\x82a5-V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[a6<\x83\x83\x83a5dV[PPPV[_\x81\x01_\x83\x01\x80a6Q\x81a2\x97V[\x90Pa6]\x81\x84a3AV[PPP`\x01\x81\x01` \x83\x01\x80a6r\x81a2\x97V[\x90Pa6~\x81\x84a3AV[PPP`\x02\x81\x01`@\x83\x01a6\x93\x81\x85a3dV[a6\x9E\x81\x83\x86a61V[PPPP`\x03\x81\x01``\x83\x01a6\xB4\x81\x85a3dV[a6\xBF\x81\x83\x86a61V[PPPPPPV[a6\xD1\x82\x82a6AV[PPV[_\x81\x90P\x91\x90PV[_a6\xEC` \x84\x01\x84a'\x94V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12a7\x1CWa7\x1Ba6\xFCV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a7DWa7Ca6\xF4V[[`\x01\x82\x026\x03\x83\x13\x15a7ZWa7Ya6\xF8V[[P\x92P\x92\x90PV[_a7m\x83\x85a'\xF5V[\x93Pa7z\x83\x85\x84a)\xA7V[a7\x83\x83a&zV[\x84\x01\x90P\x93\x92PPPV[_`\x80\x83\x01a7\x9F_\x84\x01\x84a6\xDEV[a7\xAB_\x86\x01\x82a'\xE6V[Pa7\xB9` \x84\x01\x84a6\xDEV[a7\xC6` \x86\x01\x82a'\xE6V[Pa7\xD4`@\x84\x01\x84a7\0V[\x85\x83\x03`@\x87\x01Ra7\xE7\x83\x82\x84a7bV[\x92PPPa7\xF8``\x84\x01\x84a7\0V[\x85\x83\x03``\x87\x01Ra8\x0B\x83\x82\x84a7bV[\x92PPP\x80\x91PP\x92\x91PPV[_a8$\x83\x83a7\x8EV[\x90P\x92\x91PPV[_\x825`\x01`\x80\x03\x836\x03\x03\x81\x12a8GWa8Fa6\xFCV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a8j\x83\x85a,\xF8V[\x93P\x83` \x84\x02\x85\x01a8|\x84a6\xD5V[\x80_[\x87\x81\x10\x15a8\xBFW\x84\x84\x03\x89Ra8\x96\x82\x84a8,V[a8\xA0\x85\x82a8\x19V[\x94Pa8\xAB\x83a8SV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa8\x7FV[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_a8\xDF` \x84\x01\x84a':V[\x90P\x92\x91PPV[a8\xF0\x81a&\xE2V[\x82RPPV[`\x80\x82\x01a9\x06_\x83\x01\x83a8\xD1V[a9\x12_\x85\x01\x82a8\xE7V[Pa9 ` \x83\x01\x83a8\xD1V[a9-` \x85\x01\x82a8\xE7V[Pa9;`@\x83\x01\x83a8\xD1V[a9H`@\x85\x01\x82a8\xE7V[Pa9V``\x83\x01\x83a8\xD1V[a9c``\x85\x01\x82a8\xE7V[PPPPV[_`\xA0\x82\x01\x90P\x81\x81\x03_\x83\x01Ra9\x82\x81\x85\x87a8_V[\x90Pa9\x91` \x83\x01\x84a8\xF6V[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a9\xB7\x82a9\x99V[a9\xC1\x81\x85a9\xA3V[\x93Pa9\xD1\x81\x85` \x86\x01a&RV[\x80\x84\x01\x91PP\x92\x91PPV[_a9\xE8\x82\x84a9\xADV[\x91P\x81\x90P\x92\x91PPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Ra:\x0B\x81\x86a&\x8AV[\x90Pa:\x1A` \x83\x01\x85a&\xEBV[a:'`@\x83\x01\x84a&\xEBV[\x94\x93PPPPV",
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
    /**Custom error with signature `KeyManagementRequestInFlight()` and selector `0x8e2bfd90`.
```solidity
error KeyManagementRequestInFlight();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyManagementRequestInFlight;
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
        impl ::core::convert::From<KeyManagementRequestInFlight>
        for UnderlyingRustTuple<'_> {
            fn from(value: KeyManagementRequestInFlight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KeyManagementRequestInFlight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyManagementRequestInFlight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyManagementRequestInFlight()";
            const SELECTOR: [u8; 4] = [142u8, 43u8, 253u8, 144u8];
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
    ///Container for all the [`ProtocolConfig`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum ProtocolConfigCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        defineNewKmsContext(defineNewKmsContextCall),
        #[allow(missing_docs)]
        destroyKmsContext(destroyKmsContextCall),
        #[allow(missing_docs)]
        getCurrentKmsContextId(getCurrentKmsContextIdCall),
        #[allow(missing_docs)]
        getKmsGenThreshold(getKmsGenThresholdCall),
        #[allow(missing_docs)]
        getKmsNodeForContext(getKmsNodeForContextCall),
        #[allow(missing_docs)]
        getKmsNodesForContext(getKmsNodesForContextCall),
        #[allow(missing_docs)]
        getKmsSignersForContext(getKmsSignersForContextCall),
        #[allow(missing_docs)]
        getMpcThreshold(getMpcThresholdCall),
        #[allow(missing_docs)]
        getPublicDecryptionThreshold(getPublicDecryptionThresholdCall),
        #[allow(missing_docs)]
        getUserDecryptionThreshold(getUserDecryptionThresholdCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        initializeFromMigration(initializeFromMigrationCall),
        #[allow(missing_docs)]
        isKmsSignerForContext(isKmsSignerForContextCall),
        #[allow(missing_docs)]
        isKmsTxSenderForContext(isKmsTxSenderForContextCall),
        #[allow(missing_docs)]
        isValidKmsContext(isValidKmsContextCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
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
            [38u8, 207u8, 93u8, 239u8],
            [42u8, 56u8, 137u8, 152u8],
            [49u8, 255u8, 65u8, 200u8],
            [70u8, 197u8, 187u8, 189u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [85u8, 110u8, 202u8, 250u8],
            [91u8, 255u8, 118u8, 217u8],
            [148u8, 71u8, 207u8, 212u8],
            [151u8, 111u8, 62u8, 185u8],
            [169u8, 44u8, 117u8, 203u8],
            [173u8, 60u8, 177u8, 204u8],
            [180u8, 114u8, 43u8, 196u8],
            [191u8, 155u8, 22u8, 200u8],
            [192u8, 174u8, 100u8, 247u8],
            [194u8, 180u8, 41u8, 134u8],
            [216u8, 248u8, 57u8, 43u8],
            [249u8, 198u8, 112u8, 195u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigCalls {
        const NAME: &'static str = "ProtocolConfigCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 19usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defineNewKmsContext(_) => {
                    <defineNewKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::destroyKmsContext(_) => {
                    <destroyKmsContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentKmsContextId(_) => {
                    <getCurrentKmsContextIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsGenThreshold(_) => {
                    <getKmsGenThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsNodeForContext(_) => {
                    <getKmsNodeForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsNodesForContext(_) => {
                    <getKmsNodesForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKmsSignersForContext(_) => {
                    <getKmsSignersForContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMpcThreshold(_) => {
                    <getMpcThresholdCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::initializeFromMigration(_) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::defineNewKmsContext(inner) => {
                    <defineNewKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::destroyKmsContext(inner) => {
                    <destroyKmsContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::initializeFromMigration(inner) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::defineNewKmsContext(inner) => {
                    <defineNewKmsContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initializeFromMigration(inner) => {
                    <initializeFromMigrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`ProtocolConfig`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigErrors {
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
        KeyManagementRequestInFlight(KeyManagementRequestInFlight),
        #[allow(missing_docs)]
        KmsNodeNullSigner(KmsNodeNullSigner),
        #[allow(missing_docs)]
        KmsNodeNullTxSender(KmsNodeNullTxSender),
        #[allow(missing_docs)]
        KmsSignerAlreadyRegistered(KmsSignerAlreadyRegistered),
        #[allow(missing_docs)]
        KmsTxSenderAlreadyRegistered(KmsTxSenderAlreadyRegistered),
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
            [33u8, 191u8, 218u8, 16u8],
            [45u8, 236u8, 207u8, 77u8],
            [54u8, 191u8, 182u8, 14u8],
            [69u8, 149u8, 252u8, 226u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 221u8, 190u8, 129u8],
            [132u8, 102u8, 128u8, 74u8],
            [142u8, 43u8, 253u8, 144u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [202u8, 168u8, 20u8, 163u8],
            [209u8, 140u8, 79u8, 240u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [245u8, 26u8, 246u8, 187u8],
            [249u8, 46u8, 232u8, 169u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProtocolConfigErrors {
        const NAME: &'static str = "ProtocolConfigErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
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
                Self::KeyManagementRequestInFlight(_) => {
                    <KeyManagementRequestInFlight as alloy_sol_types::SolError>::SELECTOR
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
                    fn KeyManagementRequestInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KeyManagementRequestInFlight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(ProtocolConfigErrors::KeyManagementRequestInFlight)
                    }
                    KeyManagementRequestInFlight
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
                    fn KeyManagementRequestInFlight(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ProtocolConfigErrors> {
                        <KeyManagementRequestInFlight as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ProtocolConfigErrors::KeyManagementRequestInFlight)
                    }
                    KeyManagementRequestInFlight
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
                Self::KeyManagementRequestInFlight(inner) => {
                    <KeyManagementRequestInFlight as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::KeyManagementRequestInFlight(inner) => {
                    <KeyManagementRequestInFlight as alloy_sol_types::SolError>::abi_encode_raw(
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
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ProtocolConfigEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KmsContextDestroyed(KmsContextDestroyed),
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
        const COUNT: usize = 4usize;
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
                    <KmsContextDestroyed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KmsContextDestroyed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KmsContextDestroyed)
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
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KmsContextDestroyed(inner) => {
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
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KmsContextDestroyed(inner) => {
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
