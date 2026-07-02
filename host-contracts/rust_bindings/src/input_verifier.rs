///Module containing a contract's types and functions.
/**

```solidity
library FHEVMExecutor {
    struct ContextUserInputs { address userAddress; address contractAddress; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod FHEVMExecutor {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ContextUserInputs { address userAddress; address contractAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContextUserInputs {
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub contractAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ContextUserInputs> for UnderlyingRustTuple<'_> {
            fn from(value: ContextUserInputs) -> Self {
                (value.userAddress, value.contractAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContextUserInputs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    userAddress: tuple.0,
                    contractAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ContextUserInputs {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ContextUserInputs {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.contractAddress,
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
        impl alloy_sol_types::SolType for ContextUserInputs {
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
        impl alloy_sol_types::SolStruct for ContextUserInputs {
            const NAME: &'static str = "ContextUserInputs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ContextUserInputs(address userAddress,address contractAddress)",
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
                            &self.userAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.contractAddress,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ContextUserInputs {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.userAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.contractAddress,
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
                    &rust.userAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.contractAddress,
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
    /**Creates a new wrapper around an on-chain [`FHEVMExecutor`](self) contract instance.

See the [wrapper's documentation](`FHEVMExecutorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> FHEVMExecutorInstance<P, N> {
        FHEVMExecutorInstance::<P, N>::new(address, provider)
    }
    /**A [`FHEVMExecutor`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`FHEVMExecutor`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct FHEVMExecutorInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for FHEVMExecutorInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("FHEVMExecutorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FHEVMExecutorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`FHEVMExecutor`](self) contract instance.

See the [wrapper's documentation](`FHEVMExecutorInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> FHEVMExecutorInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> FHEVMExecutorInstance<P, N> {
            FHEVMExecutorInstance {
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
    > FHEVMExecutorInstance<P, N> {
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
    > FHEVMExecutorInstance<P, N> {
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
library FHEVMExecutor {
    struct ContextUserInputs {
        address userAddress;
        address contractAddress;
    }
}

interface InputVerifier {
    error AddressEmptyCode(address target);
    error CoprocessorAlreadySigner();
    error CoprocessorSignerNull();
    error DeserializingInputProofFail();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyInputProof();
    error FailedCall();
    error InvalidChainId();
    error InvalidHandleVersion();
    error InvalidIndex();
    error InvalidInitialization();
    error InvalidInputHandle();
    error InvalidSigner(address signerRecovered);
    error NotASigner();
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error SignatureThresholdNotReached(uint256 numSignatures);
    error SignaturesVerificationFailed();
    error SignersSetIsEmpty();
    error ThresholdIsAboveNumberOfSigners();
    error ThresholdIsNull();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error ZeroSignature();

    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event NewContextSet(address[] newSignersSet, uint256 newThreshold);
    event Upgraded(address indexed implementation);

    constructor();

    function EIP712_INPUT_VERIFICATION_TYPE() external view returns (string memory);
    function EIP712_INPUT_VERIFICATION_TYPEHASH() external view returns (bytes32);
    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function cleanTransientStorage() external;
    function defineNewContext(address[] memory newSignersSet, uint256 newThreshold) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getCoprocessorSigners() external view returns (address[] memory);
    function getHandleVersion() external pure returns (uint8);
    function getThreshold() external view returns (uint256);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy(address verifyingContractSource, uint64 chainIDSource, address[] memory initialSigners, uint256 initialThreshold) external;
    function isSigner(address account) external view returns (bool);
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV2(address[] memory newSignersSet, uint256 threshold) external;
    function setThreshold(uint256 threshold) external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function verifyInput(FHEVMExecutor.ContextUserInputs memory context, bytes32 inputHandle, bytes memory inputProof) external returns (bytes32);
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
    "name": "EIP712_INPUT_VERIFICATION_TYPE",
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
    "name": "EIP712_INPUT_VERIFICATION_TYPEHASH",
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
    "name": "cleanTransientStorage",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "defineNewContext",
    "inputs": [
      {
        "name": "newSignersSet",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "newThreshold",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "getHandleVersion",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getThreshold",
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
        "name": "verifyingContractSource",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "chainIDSource",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "initialSigners",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "initialThreshold",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isSigner",
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
        "name": "newSignersSet",
        "type": "address[]",
        "internalType": "address[]"
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
    "name": "setThreshold",
    "inputs": [
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
    "type": "function",
    "name": "verifyInput",
    "inputs": [
      {
        "name": "context",
        "type": "tuple",
        "internalType": "struct FHEVMExecutor.ContextUserInputs",
        "components": [
          {
            "name": "userAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "inputHandle",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "inputProof",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "NewContextSet",
    "inputs": [
      {
        "name": "newSignersSet",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      },
      {
        "name": "newThreshold",
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
    "name": "CoprocessorAlreadySigner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CoprocessorSignerNull",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DeserializingInputProofFail",
    "inputs": []
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
    "name": "EmptyInputProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidChainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidHandleVersion",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInputHandle",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSigner",
    "inputs": [
      {
        "name": "signerRecovered",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotASigner",
    "inputs": []
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
    "name": "SignatureThresholdNotReached",
    "inputs": [
      {
        "name": "numSignatures",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "SignaturesVerificationFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignersSetIsEmpty",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ThresholdIsAboveNumberOfSigners",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ThresholdIsNull",
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
    "name": "ZeroSignature",
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
pub mod InputVerifier {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b6080516129bd620001005f395f818161127f015281816112a8015261148b01526129bd5ff3fe6080604052600436106100fa575f3560e01c806384b0196e11610092578063ad3cb1cc11610062578063ad3cb1cc1461026a578063da53c47d1461029a578063e6317df5146102b9578063e75235b8146102d8578063e7d9e4071461030b575f80fd5b806384b0196e146101ef5780638b218123146102165780639164d0ae1461022a578063960bfe041461024b575f80fd5b806354130ccd116100cd57806354130ccd146101735780635eed7675146101875780637a297f4b146101a65780637df73e27146101c0575f80fd5b80630d8e6e2c146100fe57806335334c23146101285780634f1ef2861461013e57806352d1902d14610151575b5f80fd5b348015610109575f80fd5b5061011261032a565b60405161011f91906120e9565b60405180910390f35b348015610133575f80fd5b5061013c610395565b005b61013c61014c3660046121be565b6103bf565b34801561015c575f80fd5b506101656103de565b60405190815260200161011f565b34801561017e575f80fd5b506101126103f9565b348015610192575f80fd5b5061013c6101a136600461220a565b610415565b3480156101b1575f80fd5b506040515f815260200161011f565b3480156101cb575f80fd5b506101df6101da3660046122a4565b6105a3565b604051901515815260200161011f565b3480156101fa575f80fd5b506102036105cc565b60405161011f97969594939291906122bf565b348015610221575f80fd5b5061016561069a565b348015610235575f80fd5b5061023e6106bd565b60405161011f9190612399565b348015610256575f80fd5b5061013c6102653660046123ab565b61072c565b348015610275575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156102a5575f80fd5b5061013c6102b43660046123c2565b610849565b3480156102c4575f80fd5b506101656102d3366004612474565b610b66565b3480156102e3575f80fd5b507f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0254610165565b348015610316575f80fd5b5061013c6103253660046123c2565b611119565b60606040518060400160405280600d81526020016c24b7383aba2b32b934b334b2b960991b81525061035b5f6111e5565b61036560026111e5565b61036e5f6111e5565b6040516020016103819493929190612516565b604051602081830303815290604052905090565b5f5c5f805d600190810190805b828110156103ba57805c5f825d5f815d5081016103a2565b505050565b6103c7611274565b6103d08261131a565b6103da82826113c4565b5050565b5f6103e7611480565b505f8051602061295d83398151915290565b6040518060a00160405280607f81526020016128de607f913981565b5f8051602061299d833981519152546001600160401b03166001600160401b031660011461045657604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061299d833981519152805460039190600160401b900460ff168061048c575080546001600160401b03808416911610155b156104aa5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b178155604080518082018252601181527024b7383aba2b32b934b334b1b0ba34b7b760791b602080830191909152825180840190935260018352603160f81b908301526105169189896114c9565b6105538585808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250879250610849915050565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050505050565b6001600160a01b03165f9081525f805160206128be833981519152602052604090205460ff1690565b5f60608082808083815f8051602061297d83398151915280549091501580156105f757506001810154155b6106405760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064015b60405180910390fd5b6106486114e3565b61065061159a565b60049290920154604080515f80825260208201909252600f60f81b9c939b509399506001600160401b03600160a01b83041698506001600160a01b03909116965094509092509050565b6040518060a00160405280607f81526020016128de607f91398051906020012081565b60605f5f805160206128be8339815191526001810180546040805160208084028201810190925282815293945083018282801561072157602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610703575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561077c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107a09190612593565b6001600160a01b0316336001600160a01b0316146107d35760405163021bfda160e41b8152336004820152602401610637565b6107dc816115d8565b6040515f805160206128be833981519152907f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea9061083d907f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace019085906125ae565b60405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610899573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108bd9190612593565b6001600160a01b0316336001600160a01b0316146108f05760405163021bfda160e41b8152336004820152602401610637565b81515f81900361091357604051631286e95160e01b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace018054604080516020808402820181019092528281525f805160206128be833981519152935f93919290919083018282801561099657602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610978575b505083519394505f925050505b81811015610a3a575f845f015f8584815181106109c2576109c2612609565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f6101000a81548160ff02191690831515021790555083600101805480610a1257610a1261261d565b5f8281526020902081015f1990810180546001600160a01b03191690550190556001016109a3565b505f5b84811015610b1b575f878281518110610a5857610a58612609565b602002602001015190505f6001600160a01b0316816001600160a01b031603610a94576040516304069ca760e21b815260040160405180910390fd5b6001600160a01b0381165f9081526020869052604090205460ff1615610acd5760405163572de7c960e11b815260040160405180910390fd5b6001600160a01b03165f818152602086815260408220805460ff1916600190811790915587810180548083018255908452919092200180546001600160a01b03191690921790915501610a3d565b50610b25856115d8565b7f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8686604051610b56929190612631565b60405180910390a1505050505050565b5f805f610b7b84875f0151886020015161164f565b90925090506001600160401b03601086901c16468114610bae57604051633d23e4d160e11b815260040160405180910390fd5b8560ff605082901c16846110335786515f819003610bdf576040516359240e8b60e11b815260040160405180910390fd5b5f885f81518110610bf257610bf2612609565b602001015160f81c60f81b60f81c60ff1690505f89600181518110610c1957610c19612609565b016020015160f81c90508382111580610c32575060fe84115b15610c50576040516363df817160e01b815260040160405180910390fd5b5f610c5c826041612666565b610c67846020612666565b610c7290600261267d565b610c7c919061267d565b905080841015610c9f57604051631817ecd760e01b815260040160405180910390fd5b5f836001600160401b03811115610cb857610cb861210f565b604051908082528060200260200182016040528015610ce1578160200160208202803683370190505b5090505f5b84811015610d4157602081028d016022015160ff811615610d1a576040516317df86d560e21b815260040160405180910390fd5b80838381518110610d2d57610d2d612609565b602090810291909101015250600101610ce6565b505f836001600160401b03811115610d5b57610d5b61210f565b604051908082528060200260200182016040528015610d8e57816020015b6060815260200190600190039081610d795790505b5090505f5b84811015610e8557604080516041808252608082019092529060208201818036833701905050828281518110610dcb57610dcb612609565b60200260200101819052505f5b6041811015610e7c578e81610dee846041612666565b610df98a6020612666565b610e0490600261267d565b610e0e919061267d565b610e18919061267d565b81518110610e2857610e28612609565b602001015160f81c60f81b838381518110610e4557610e45612609565b60200260200101518281518110610e5e57610e5e612609565b60200101906001600160f81b03191690815f1a905350600101610dd8565b50600101610d93565b50610ec76040518060a00160405280606081526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f8152602001606081525090565b82815f01819052508f5f015181602001906001600160a01b031690816001600160a01b0316815250508f6020015181604001906001600160a01b031690816001600160a01b031681525050468160600181815250505f848f51610f2a9190612690565b9050806001600160401b03811115610f4457610f4461210f565b6040519080825280601f01601f191660200182016040528015610f6e576020820181803683370190505b5060808301525f5b81811015610fd8578f610f89828861267d565b81518110610f9957610f99612609565b602001015160f81c60f81b83608001518281518110610fba57610fba612609565b60200101906001600160f81b03191690815f1a905350600101610f76565b50610fe3828461168f565b610fec8c6116c2565b838981518110610ffe57610ffe612609565b60200260200101515f1c8a1461102657604051624b1bf160e31b815260040160405180910390fd5b505050505050505061110b565b5f875f8151811061104657611046612609565b016020015160f81c9050818111158061105f575060fe82115b1561107d576040516363df817160e01b815260040160405180910390fd5b5f805b60208110156110e85761109481601f612690565b61109f906008612666565b8a826110ac876020612666565b6110b790600261267d565b6110c1919061267d565b815181106110d1576110d1612609565b016020015160f81c901b9190911790600101611080565b5083811461110857604051624b1bf160e31b815260040160405180910390fd5b50505b5093505050505b9392505050565b5f8051602061299d833981519152805460039190600160401b900460ff168061114f575080546001600160401b03808416911610155b1561116d5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781556111988484610849565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050565b60605f6111f1836116d5565b60010190505f816001600160401b0381111561120f5761120f61210f565b6040519080825280601f01601f191660200182016040528015611239576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461124357509392505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806112fa57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166112ee5f8051602061295d833981519152546001600160a01b031690565b6001600160a01b031614155b156113185760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561136a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061138e9190612593565b6001600160a01b0316336001600160a01b0316146113c15760405163021bfda160e41b8152336004820152602401610637565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561141e575060408051601f3d908101601f1916820190925261141b918101906126a3565b60015b61144657604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610637565b5f8051602061295d833981519152811461147657604051632a87526960e21b815260048101829052602401610637565b6103ba83836117ad565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146113185760405163703e46dd60e11b815260040160405180910390fd5b6114d1611802565b6114dd84848484611838565b50505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360280546060915f8051602061297d83398151915291611521906126ba565b80601f016020809104026020016040519081016040528092919081815260200182805461154d906126ba565b80156107215780601f1061156f57610100808354040283529160200191610721565b820191905f5260205f20905b81548152906001019060200180831161157b5750939695505050505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360380546060915f8051602061297d83398151915291611521906126ba565b805f036115f857604051630151f13160e71b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace01545f805160206128be83398151915290821115611649576040516335194e6360e01b815260040160405180910390fd5b60020155565b5f805f80848688604051602001611668939291906126f2565b60408051808303601f190181529190528051602090910120805c9890975095505050505050565b5f611699836118e6565b90506116a581836119bd565b6103ba57604051634b506ccd60e01b815260040160405180910390fd5b6001815d60015f5c0181815d805f5d5050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106117135772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef8100000000831061173f576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc10000831061175d57662386f26fc10000830492506010015b6305f5e1008310611775576305f5e100830492506008015b612710831061178957612710830492506004015b6064831061179b576064830492506002015b600a83106117a7576001015b92915050565b6117b682611b60565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156117fa576103ba8282611bc3565b6103da611c35565b5f8051602061299d83398151915254600160401b900460ff1661131857604051631afcd79f60e31b815260040160405180910390fd5b611840611802565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360480546001600160a01b0384166001600160e01b031990911617600160a01b6001600160401b038416021790555f8051602061297d8339815191527fe910845fd818f61127c84f3586776436a37dead33625056c65162537e33736026118c68682612782565b50600381016118d58582612782565b505f80825560019091015550505050565b5f6117a76040518060a00160405280607f81526020016128de607f913980516020918201208451604051919261191c9201612841565b6040516020818303038152906040528051906020012084602001518560400151866060015187608001516040516020016119569190612876565b60408051601f198184030181528282528051602091820120908301979097528101949094526001600160a01b0392831660608501529116608083015260a082015260c081019190915260e00160405160208183030381529060405280519060200120611c54565b80515f908082036119e157604051635985192160e11b815260040160405180910390fd5b5f611a0a7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace025490565b905080821015611a3057604051632e57ff8360e21b815260048101839052602401610637565b5f826001600160401b03811115611a4957611a4961210f565b604051908082528060200260200182016040528015611a72578160200160208202803683370190505b5090505f805b84811015611b49575f611aa489898481518110611a9757611a97612609565b6020026020010151611c80565b9050611aaf816105a3565b611ad75760405163bf18af4360e01b81526001600160a01b0382166004820152602401610637565b805c611b205780848481518110611af057611af0612609565b6001600160a01b039092166020928302919091019091015282611b1281612891565b935050611b20816001611c94565b848310611b4057611b318484611c9b565b600196505050505050506117a7565b50600101611a78565b50611b548282611c9b565b505f9695505050505050565b806001600160a01b03163b5f03611b9557604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610637565b5f8051602061295d83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051611bdf9190612876565b5f60405180830381855af49150503d805f8114611c17576040519150601f19603f3d011682016040523d82523d5f602084013e611c1c565b606091505b5091509150611c2c858383611cd0565b95945050505050565b34156113185760405163b398979f60e01b815260040160405180910390fd5b5f6117a7611c60611d2c565b8360405161190160f01b8152600281019290925260228201526042902090565b5f80611c8c8484611d3a565b949350505050565b80825d5050565b5f5b818110156103ba57611cc8838281518110611cba57611cba612609565b60200260200101515f611c94565b600101611c9d565b606082611ce557611ce082611d62565b611112565b8151158015611cfc57506001600160a01b0384163b155b15611d2557604051639996b31560e01b81526001600160a01b0385166004820152602401610637565b5080611112565b5f611d35611d8b565b905090565b5f805f80611d488686611e29565b925092509250611d588282611e72565b5090949350505050565b805115611d725780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5f8051602061297d8339815191527f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611dc3611f2a565b611dcb611f92565b60048401546040805160208101959095528401929092526060830152600160a01b81046001600160401b031660808301526001600160a01b031660a082015260c0016040516020818303038152906040528051906020012091505090565b5f805f8351604103611e60576020840151604085015160608601515f1a611e5288828585611fd4565b955095509550505050611e6b565b505081515f91506002905b9250925092565b5f826003811115611e8557611e856128a9565b03611e8e575050565b6001826003811115611ea257611ea26128a9565b03611ec05760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115611ed457611ed46128a9565b03611ef55760405163fce698f760e01b815260048101829052602401610637565b6003826003811115611f0957611f096128a9565b036103da576040516335e2f38360e21b815260048101829052602401610637565b5f5f8051602061297d83398151915281611f426114e3565b805190915015611f5a57805160209091012092915050565b81548015611f69579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f5f8051602061297d83398151915281611faa61159a565b805190915015611fc257805160209091012092915050565b60018201548015611f69579392505050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561200d57505f91506003905082612092565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561205e573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661208957505f925060019150829050612092565b92505f91508190505b9450945094915050565b5f5b838110156120b657818101518382015260200161209e565b50505f910152565b5f81518084526120d581602086016020860161209c565b601f01601f19169290920160200192915050565b602081525f61111260208301846120be565b6001600160a01b03811681146113c1575f80fd5b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b038111828210171561214b5761214b61210f565b604052919050565b5f82601f830112612162575f80fd5b81356001600160401b0381111561217b5761217b61210f565b61218e601f8201601f1916602001612123565b8181528460208386010111156121a2575f80fd5b816020850160208301375f918101602001919091529392505050565b5f80604083850312156121cf575f80fd5b82356121da816120fb565b915060208301356001600160401b038111156121f4575f80fd5b61220085828601612153565b9150509250929050565b5f805f805f6080868803121561221e575f80fd5b8535612229816120fb565b945060208601356001600160401b038082168214612245575f80fd5b9094506040870135908082111561225a575f80fd5b818801915088601f83011261226d575f80fd5b81358181111561227b575f80fd5b8960208260051b850101111561228f575f80fd5b96999598505060200195606001359392505050565b5f602082840312156122b4575f80fd5b8135611112816120fb565b60ff60f81b881681525f602060e060208401526122df60e084018a6120be565b83810360408501526122f1818a6120be565b606085018990526001600160a01b038816608086015260a0850187905284810360c0860152855180825260208088019350909101905f5b8181101561234457835183529284019291840191600101612328565b50909c9b505050505050505050505050565b5f815180845260208085019450602084015f5b8381101561238e5781516001600160a01b031687529582019590820190600101612369565b509495945050505050565b602081525f6111126020830184612356565b5f602082840312156123bb575f80fd5b5035919050565b5f80604083850312156123d3575f80fd5b82356001600160401b03808211156123e9575f80fd5b818501915085601f8301126123fc575f80fd5b81356020828211156124105761241061210f565b8160051b9250612421818401612123565b828152928401810192818101908985111561243a575f80fd5b948201945b848610156124645785359350612454846120fb565b838252948201949082019061243f565b9997909101359750505050505050565b5f805f8385036080811215612487575f80fd5b6040811215612494575f80fd5b50604051604081016001600160401b0382821081831117156124b8576124b861210f565b81604052863591506124c9826120fb565b9082526020860135906124db826120fb565b8160208401528295506040870135945060608701359250808311156124fe575f80fd5b505061250c86828701612153565b9150509250925092565b5f8551612527818460208a0161209c565b61103b60f11b9083019081528551612546816002840160208a0161209c565b808201915050601760f91b806002830152855161256a816003850160208a0161209c565b6003920191820152835161258581600484016020880161209c565b016004019695505050505050565b5f602082840312156125a3575f80fd5b8151611112816120fb565b5f6040820160408352808554808352606085019150865f526020925060205f205f5b828110156125f55781546001600160a01b0316845292840192600191820191016125d0565b505050602093909301939093525092915050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52603160045260245ffd5b604081525f6126436040830185612356565b90508260208301529392505050565b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176117a7576117a7612652565b808201808211156117a7576117a7612652565b818103818111156117a7576117a7612652565b5f602082840312156126b3575f80fd5b5051919050565b600181811c908216806126ce57607f821691505b6020821081036126ec57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f6bffffffffffffffffffffffff19808660601b168352808560601b16601484015250825161272881602885016020870161209c565b91909101602801949350505050565b601f8211156103ba57805f5260205f20601f840160051c8101602085101561275c5750805b601f840160051c820191505b8181101561277b575f8155600101612768565b5050505050565b81516001600160401b0381111561279b5761279b61210f565b6127af816127a984546126ba565b84612737565b602080601f8311600181146127e2575f84156127cb5750858301515b5f19600386901b1c1916600185901b178555612839565b5f85815260208120601f198616915b82811015612810578886015182559484019460019091019084016127f1565b508582101561282d57878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b81515f9082906020808601845b8381101561286a5781518552938201939082019060010161284e565b50929695505050505050565b5f825161288781846020870161209c565b9190910192915050565b5f600182016128a2576128a2612652565b5060010190565b634e487b7160e01b5f52602160045260245ffdfe3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0043697068657274657874566572696669636174696f6e28627974657333325b5d20637448616e646c65732c616464726573732075736572416464726573732c6164647265737320636f6e7472616374416464726573732c75696e7432353620636f6e7472616374436861696e49642c62797465732065787472614461746129360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbce910845fd818f61127c84f3586776436a37dead33625056c65162537e3373600f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa)\xBDb\0\x01\0_9_\x81\x81a\x12\x7F\x01R\x81\x81a\x12\xA8\x01Ra\x14\x8B\x01Ra)\xBD_\xF3\xFE`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80c\x84\xB0\x19n\x11a\0\x92W\x80c\xAD<\xB1\xCC\x11a\0bW\x80c\xAD<\xB1\xCC\x14a\x02jW\x80c\xDAS\xC4}\x14a\x02\x9AW\x80c\xE61}\xF5\x14a\x02\xB9W\x80c\xE7R5\xB8\x14a\x02\xD8W\x80c\xE7\xD9\xE4\x07\x14a\x03\x0BW_\x80\xFD[\x80c\x84\xB0\x19n\x14a\x01\xEFW\x80c\x8B!\x81#\x14a\x02\x16W\x80c\x91d\xD0\xAE\x14a\x02*W\x80c\x96\x0B\xFE\x04\x14a\x02KW_\x80\xFD[\x80cT\x13\x0C\xCD\x11a\0\xCDW\x80cT\x13\x0C\xCD\x14a\x01sW\x80c^\xEDvu\x14a\x01\x87W\x80cz)\x7FK\x14a\x01\xA6W\x80c}\xF7>'\x14a\x01\xC0W_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c53L#\x14a\x01(W\x80cO\x1E\xF2\x86\x14a\x01>W\x80cR\xD1\x90-\x14a\x01QW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03*V[`@Qa\x01\x1F\x91\x90a \xE9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01<a\x03\x95V[\0[a\x01<a\x01L6`\x04a!\xBEV[a\x03\xBFV[4\x80\x15a\x01\\W_\x80\xFD[Pa\x01ea\x03\xDEV[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01~W_\x80\xFD[Pa\x01\x12a\x03\xF9V[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01<a\x01\xA16`\x04a\"\nV[a\x04\x15V[4\x80\x15a\x01\xB1W_\x80\xFD[P`@Q_\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xCBW_\x80\xFD[Pa\x01\xDFa\x01\xDA6`\x04a\"\xA4V[a\x05\xA3V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xFAW_\x80\xFD[Pa\x02\x03a\x05\xCCV[`@Qa\x01\x1F\x97\x96\x95\x94\x93\x92\x91\x90a\"\xBFV[4\x80\x15a\x02!W_\x80\xFD[Pa\x01ea\x06\x9AV[4\x80\x15a\x025W_\x80\xFD[Pa\x02>a\x06\xBDV[`@Qa\x01\x1F\x91\x90a#\x99V[4\x80\x15a\x02VW_\x80\xFD[Pa\x01<a\x02e6`\x04a#\xABV[a\x07,V[4\x80\x15a\x02uW_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x02\xA5W_\x80\xFD[Pa\x01<a\x02\xB46`\x04a#\xC2V[a\x08IV[4\x80\x15a\x02\xC4W_\x80\xFD[Pa\x01ea\x02\xD36`\x04a$tV[a\x0BfV[4\x80\x15a\x02\xE3W_\x80\xFD[P\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02Ta\x01eV[4\x80\x15a\x03\x16W_\x80\xFD[Pa\x01<a\x03%6`\x04a#\xC2V[a\x11\x19V[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01l$\xB78:\xBA+2\xB94\xB34\xB2\xB9`\x99\x1B\x81RPa\x03[_a\x11\xE5V[a\x03e`\x02a\x11\xE5V[a\x03n_a\x11\xE5V[`@Q` \x01a\x03\x81\x94\x93\x92\x91\x90a%\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\\_\x80]`\x01\x90\x81\x01\x90\x80[\x82\x81\x10\x15a\x03\xBAW\x80\\_\x82]_\x81]P\x81\x01a\x03\xA2V[PPPV[a\x03\xC7a\x12tV[a\x03\xD0\x82a\x13\x1AV[a\x03\xDA\x82\x82a\x13\xC4V[PPV[_a\x03\xE7a\x14\x80V[P_\x80Q` a)]\x839\x81Q\x91R\x90V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a(\xDE`\x7F\x919\x81V[_\x80Q` a)\x9D\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x04VW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` a)\x9D\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x04\x8CWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x04\xAAW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\x11\x81Rp$\xB78:\xBA+2\xB94\xB34\xB1\xB0\xBA4\xB7\xB7`y\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\x05\x16\x91\x89\x89a\x14\xC9V[a\x05S\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa\x08I\x91PPV[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a(\xBE\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x90V[_``\x80\x82\x80\x80\x83\x81_\x80Q` a)}\x839\x81Q\x91R\x80T\x90\x91P\x15\x80\x15a\x05\xF7WP`\x01\x81\x01T\x15[a\x06@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x06Ha\x14\xE3V[a\x06Pa\x15\x9AV[`\x04\x92\x90\x92\x01T`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x93\x99P`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x83\x04\x16\x98P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x96P\x94P\x90\x92P\x90PV[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a(\xDE`\x7F\x919\x80Q\x90` \x01 \x81V[``__\x80Q` a(\xBE\x839\x81Q\x91R`\x01\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a\x07!W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\x03W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07|W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA0\x91\x90a%\x93V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xD3W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x067V[a\x07\xDC\x81a\x15\xD8V[`@Q_\x80Q` a(\xBE\x839\x81Q\x91R\x90\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x90a\x08=\x90\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x90\x85\x90a%\xAEV[`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x99W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBD\x91\x90a%\x93V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xF0W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x067V[\x81Q_\x81\x90\x03a\t\x13W`@Qc\x12\x86\xE9Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R_\x80Q` a(\xBE\x839\x81Q\x91R\x93_\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\t\x96W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\txW[PP\x83Q\x93\x94P_\x92PPP[\x81\x81\x10\x15a\n:W_\x84_\x01_\x85\x84\x81Q\x81\x10a\t\xC2Wa\t\xC2a&\tV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x01\x80T\x80a\n\x12Wa\n\x12a&\x1DV[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\t\xA3V[P_[\x84\x81\x10\x15a\x0B\x1BW_\x87\x82\x81Q\x81\x10a\nXWa\nXa&\tV[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\n\x94W`@Qc\x04\x06\x9C\xA7`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x86\x90R`@\x90 T`\xFF\x16\x15a\n\xCDW`@QcW-\xE7\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R` \x86\x81R`@\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x87\x81\x01\x80T\x80\x83\x01\x82U\x90\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x92\x17\x90\x91U\x01a\n=V[Pa\x0B%\x85a\x15\xD8V[\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x86\x86`@Qa\x0BV\x92\x91\x90a&1V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80_a\x0B{\x84\x87_\x01Q\x88` \x01Qa\x16OV[\x90\x92P\x90P`\x01`\x01`@\x1B\x03`\x10\x86\x90\x1C\x16F\x81\x14a\x0B\xAEW`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\xFF`P\x82\x90\x1C\x16\x84a\x103W\x86Q_\x81\x90\x03a\x0B\xDFW`@QcY$\x0E\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88_\x81Q\x81\x10a\x0B\xF2Wa\x0B\xF2a&\tV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P_\x89`\x01\x81Q\x81\x10a\x0C\x19Wa\x0C\x19a&\tV[\x01` \x01Q`\xF8\x1C\x90P\x83\x82\x11\x15\x80a\x0C2WP`\xFE\x84\x11[\x15a\x0CPW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\\\x82`Aa&fV[a\x0Cg\x84` a&fV[a\x0Cr\x90`\x02a&}V[a\x0C|\x91\x90a&}V[\x90P\x80\x84\x10\x15a\x0C\x9FW`@Qc\x18\x17\xEC\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xB8Wa\x0C\xB8a!\x0FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xE1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\rAW` \x81\x02\x8D\x01`\"\x01Q`\xFF\x81\x16\x15a\r\x1AW`@Qc\x17\xDF\x86\xD5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x83\x81Q\x81\x10a\r-Wa\r-a&\tV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C\xE6V[P_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\r[Wa\r[a!\x0FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x8EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\ryW\x90P[P\x90P_[\x84\x81\x10\x15a\x0E\x85W`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01\x81\x806\x837\x01\x90PP\x82\x82\x81Q\x81\x10a\r\xCBWa\r\xCBa&\tV[` \x02` \x01\x01\x81\x90RP_[`A\x81\x10\x15a\x0E|W\x8E\x81a\r\xEE\x84`Aa&fV[a\r\xF9\x8A` a&fV[a\x0E\x04\x90`\x02a&}V[a\x0E\x0E\x91\x90a&}V[a\x0E\x18\x91\x90a&}V[\x81Q\x81\x10a\x0E(Wa\x0E(a&\tV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x0EEWa\x0EEa&\tV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x0E^Wa\x0E^a&\tV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\r\xD8V[P`\x01\x01a\r\x93V[Pa\x0E\xC7`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x82\x81_\x01\x81\x90RP\x8F_\x01Q\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8F` \x01Q\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPF\x81``\x01\x81\x81RPP_\x84\x8FQa\x0F*\x91\x90a&\x90V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0FDWa\x0FDa!\x0FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0FnW` \x82\x01\x81\x806\x837\x01\x90P[P`\x80\x83\x01R_[\x81\x81\x10\x15a\x0F\xD8W\x8Fa\x0F\x89\x82\x88a&}V[\x81Q\x81\x10a\x0F\x99Wa\x0F\x99a&\tV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x80\x01Q\x82\x81Q\x81\x10a\x0F\xBAWa\x0F\xBAa&\tV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x0FvV[Pa\x0F\xE3\x82\x84a\x16\x8FV[a\x0F\xEC\x8Ca\x16\xC2V[\x83\x89\x81Q\x81\x10a\x0F\xFEWa\x0F\xFEa&\tV[` \x02` \x01\x01Q_\x1C\x8A\x14a\x10&W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPa\x11\x0BV[_\x87_\x81Q\x81\x10a\x10FWa\x10Fa&\tV[\x01` \x01Q`\xF8\x1C\x90P\x81\x81\x11\x15\x80a\x10_WP`\xFE\x82\x11[\x15a\x10}W`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[` \x81\x10\x15a\x10\xE8Wa\x10\x94\x81`\x1Fa&\x90V[a\x10\x9F\x90`\x08a&fV[\x8A\x82a\x10\xAC\x87` a&fV[a\x10\xB7\x90`\x02a&}V[a\x10\xC1\x91\x90a&}V[\x81Q\x81\x10a\x10\xD1Wa\x10\xD1a&\tV[\x01` \x01Q`\xF8\x1C\x90\x1B\x91\x90\x91\x17\x90`\x01\x01a\x10\x80V[P\x83\x81\x14a\x11\x08W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP[P\x93PPPP[\x93\x92PPPV[_\x80Q` a)\x9D\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x11OWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x11mW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81Ua\x11\x98\x84\x84a\x08IV[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[``_a\x11\xF1\x83a\x16\xD5V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x0FWa\x12\x0Fa!\x0FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x129W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12CWP\x93\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x12\xFAWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x12\xEE_\x80Q` a)]\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x13\x18W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13jW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x8E\x91\x90a%\x93V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xC1W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x067V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x14\x1EWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x14\x1B\x91\x81\x01\x90a&\xA3V[`\x01[a\x14FW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x067V[_\x80Q` a)]\x839\x81Q\x91R\x81\x14a\x14vW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x067V[a\x03\xBA\x83\x83a\x17\xADV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13\x18W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xD1a\x18\x02V[a\x14\xDD\x84\x84\x84\x84a\x188V[PPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02\x80T``\x91_\x80Q` a)}\x839\x81Q\x91R\x91a\x15!\x90a&\xBAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15M\x90a&\xBAV[\x80\x15a\x07!W\x80`\x1F\x10a\x15oWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07!V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15{WP\x93\x96\x95PPPPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x03\x80T``\x91_\x80Q` a)}\x839\x81Q\x91R\x91a\x15!\x90a&\xBAV[\x80_\x03a\x15\xF8W`@Qc\x01Q\xF11`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01T_\x80Q` a(\xBE\x839\x81Q\x91R\x90\x82\x11\x15a\x16IW`@Qc5\x19Nc`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x01UV[_\x80_\x80\x84\x86\x88`@Q` \x01a\x16h\x93\x92\x91\x90a&\xF2V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x80\\\x98\x90\x97P\x95PPPPPPV[_a\x16\x99\x83a\x18\xE6V[\x90Pa\x16\xA5\x81\x83a\x19\xBDV[a\x03\xBAW`@QcKPl\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81]`\x01_\\\x01\x81\x81]\x80_]PPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x17\x13Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x17?Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x17]Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x17uWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x17\x89Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x17\x9BW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x17\xA7W`\x01\x01[\x92\x91PPV[a\x17\xB6\x82a\x1B`V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x17\xFAWa\x03\xBA\x82\x82a\x1B\xC3V[a\x03\xDAa\x1C5V[_\x80Q` a)\x9D\x839\x81Q\x91RT`\x01`@\x1B\x90\x04`\xFF\x16a\x13\x18W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18@a\x18\x02V[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x04\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1B`\x01`\x01`@\x1B\x03\x84\x16\x02\x17\x90U_\x80Q` a)}\x839\x81Q\x91R\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02a\x18\xC6\x86\x82a'\x82V[P`\x03\x81\x01a\x18\xD5\x85\x82a'\x82V[P_\x80\x82U`\x01\x90\x91\x01UPPPPV[_a\x17\xA7`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a(\xDE`\x7F\x919\x80Q` \x91\x82\x01 \x84Q`@Q\x91\x92a\x19\x1C\x92\x01a(AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a\x19V\x91\x90a(vV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16``\x85\x01R\x91\x16`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1CTV[\x80Q_\x90\x80\x82\x03a\x19\xE1W`@QcY\x85\x19!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1A\n\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02T\x90V[\x90P\x80\x82\x10\x15a\x1A0W`@Qc.W\xFF\x83`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x067V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AIWa\x1AIa!\x0FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1ArW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x84\x81\x10\x15a\x1BIW_a\x1A\xA4\x89\x89\x84\x81Q\x81\x10a\x1A\x97Wa\x1A\x97a&\tV[` \x02` \x01\x01Qa\x1C\x80V[\x90Pa\x1A\xAF\x81a\x05\xA3V[a\x1A\xD7W`@Qc\xBF\x18\xAFC`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x067V[\x80\\a\x1B W\x80\x84\x84\x81Q\x81\x10a\x1A\xF0Wa\x1A\xF0a&\tV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1B\x12\x81a(\x91V[\x93PPa\x1B \x81`\x01a\x1C\x94V[\x84\x83\x10a\x1B@Wa\x1B1\x84\x84a\x1C\x9BV[`\x01\x96PPPPPPPa\x17\xA7V[P`\x01\x01a\x1AxV[Pa\x1BT\x82\x82a\x1C\x9BV[P_\x96\x95PPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1B\x95W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x067V[_\x80Q` a)]\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1B\xDF\x91\x90a(vV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1C\x17W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1C\x1CV[``\x91P[P\x91P\x91Pa\x1C,\x85\x83\x83a\x1C\xD0V[\x95\x94PPPPPV[4\x15a\x13\x18W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x17\xA7a\x1C`a\x1D,V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80a\x1C\x8C\x84\x84a\x1D:V[\x94\x93PPPPV[\x80\x82]PPV[_[\x81\x81\x10\x15a\x03\xBAWa\x1C\xC8\x83\x82\x81Q\x81\x10a\x1C\xBAWa\x1C\xBAa&\tV[` \x02` \x01\x01Q_a\x1C\x94V[`\x01\x01a\x1C\x9DV[``\x82a\x1C\xE5Wa\x1C\xE0\x82a\x1DbV[a\x11\x12V[\x81Q\x15\x80\x15a\x1C\xFCWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1D%W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x067V[P\x80a\x11\x12V[_a\x1D5a\x1D\x8BV[\x90P\x90V[_\x80_\x80a\x1DH\x86\x86a\x1E)V[\x92P\x92P\x92Pa\x1DX\x82\x82a\x1ErV[P\x90\x94\x93PPPPV[\x80Q\x15a\x1DrW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x80Q` a)}\x839\x81Q\x91R\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1D\xC3a\x1F*V[a\x1D\xCBa\x1F\x92V[`\x04\x84\x01T`@\x80Q` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R`\x01`\xA0\x1B\x81\x04`\x01`\x01`@\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x90V[_\x80_\x83Q`A\x03a\x1E`W` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x1ER\x88\x82\x85\x85a\x1F\xD4V[\x95P\x95P\x95PPPPa\x1EkV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x1E\x85Wa\x1E\x85a(\xA9V[\x03a\x1E\x8EWPPV[`\x01\x82`\x03\x81\x11\x15a\x1E\xA2Wa\x1E\xA2a(\xA9V[\x03a\x1E\xC0W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x1E\xD4Wa\x1E\xD4a(\xA9V[\x03a\x1E\xF5W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x067V[`\x03\x82`\x03\x81\x11\x15a\x1F\tWa\x1F\ta(\xA9V[\x03a\x03\xDAW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x067V[__\x80Q` a)}\x839\x81Q\x91R\x81a\x1FBa\x14\xE3V[\x80Q\x90\x91P\x15a\x1FZW\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15a\x1FiW\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[__\x80Q` a)}\x839\x81Q\x91R\x81a\x1F\xAAa\x15\x9AV[\x80Q\x90\x91P\x15a\x1F\xC2W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15a\x1FiW\x93\x92PPPV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a \rWP_\x91P`\x03\x90P\x82a \x92V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a ^W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a \x89WP_\x92P`\x01\x91P\x82\x90Pa \x92V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_[\x83\x81\x10\x15a \xB6W\x81\x81\x01Q\x83\x82\x01R` \x01a \x9EV[PP_\x91\x01RV[_\x81Q\x80\x84Ra \xD5\x81` \x86\x01` \x86\x01a \x9CV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x11\x12` \x83\x01\x84a \xBEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\xC1W_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!KWa!Ka!\x0FV[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a!bW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!{Wa!{a!\x0FV[a!\x8E`\x1F\x82\x01`\x1F\x19\x16` \x01a!#V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a!\xA2W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a!\xCFW_\x80\xFD[\x825a!\xDA\x81a \xFBV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xF4W_\x80\xFD[a\"\0\x85\x82\x86\x01a!SV[\x91PP\x92P\x92\x90PV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a\"\x1EW_\x80\xFD[\x855a\")\x81a \xFBV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x16\x82\x14a\"EW_\x80\xFD[\x90\x94P`@\x87\x015\x90\x80\x82\x11\x15a\"ZW_\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\"mW_\x80\xFD[\x815\x81\x81\x11\x15a\"{W_\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"\x8FW_\x80\xFD[\x96\x99\x95\x98PP` \x01\x95``\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\"\xB4W_\x80\xFD[\x815a\x11\x12\x81a \xFBV[`\xFF`\xF8\x1B\x88\x16\x81R_` `\xE0` \x84\x01Ra\"\xDF`\xE0\x84\x01\x8Aa \xBEV[\x83\x81\x03`@\x85\x01Ra\"\xF1\x81\x8Aa \xBEV[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R` \x80\x88\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a#DW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a#(V[P\x90\x9C\x9BPPPPPPPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a#\x8EW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a#iV[P\x94\x95\x94PPPPPV[` \x81R_a\x11\x12` \x83\x01\x84a#VV[_` \x82\x84\x03\x12\x15a#\xBBW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a#\xD3W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#\xE9W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a#\xFCW_\x80\xFD[\x815` \x82\x82\x11\x15a$\x10Wa$\x10a!\x0FV[\x81`\x05\x1B\x92Pa$!\x81\x84\x01a!#V[\x82\x81R\x92\x84\x01\x81\x01\x92\x81\x81\x01\x90\x89\x85\x11\x15a$:W_\x80\xFD[\x94\x82\x01\x94[\x84\x86\x10\x15a$dW\x855\x93Pa$T\x84a \xFBV[\x83\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$?V[\x99\x97\x90\x91\x015\x97PPPPPPPV[_\x80_\x83\x85\x03`\x80\x81\x12\x15a$\x87W_\x80\xFD[`@\x81\x12\x15a$\x94W_\x80\xFD[P`@Q`@\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a$\xB8Wa$\xB8a!\x0FV[\x81`@R\x865\x91Pa$\xC9\x82a \xFBV[\x90\x82R` \x86\x015\x90a$\xDB\x82a \xFBV[\x81` \x84\x01R\x82\x95P`@\x87\x015\x94P``\x87\x015\x92P\x80\x83\x11\x15a$\xFEW_\x80\xFD[PPa%\x0C\x86\x82\x87\x01a!SV[\x91PP\x92P\x92P\x92V[_\x85Qa%'\x81\x84` \x8A\x01a \x9CV[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa%F\x81`\x02\x84\x01` \x8A\x01a \x9CV[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa%j\x81`\x03\x85\x01` \x8A\x01a \x9CV[`\x03\x92\x01\x91\x82\x01R\x83Qa%\x85\x81`\x04\x84\x01` \x88\x01a \x9CV[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a%\xA3W_\x80\xFD[\x81Qa\x11\x12\x81a \xFBV[_`@\x82\x01`@\x83R\x80\x85T\x80\x83R``\x85\x01\x91P\x86_R` \x92P` _ _[\x82\x81\x10\x15a%\xF5W\x81T`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92`\x01\x91\x82\x01\x91\x01a%\xD0V[PPP` \x93\x90\x93\x01\x93\x90\x93RP\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`@\x81R_a&C`@\x83\x01\x85a#VV[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x17\xA7Wa\x17\xA7a&RV[\x80\x82\x01\x80\x82\x11\x15a\x17\xA7Wa\x17\xA7a&RV[\x81\x81\x03\x81\x81\x11\x15a\x17\xA7Wa\x17\xA7a&RV[_` \x82\x84\x03\x12\x15a&\xB3W_\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a&\xCEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a&\xECWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x86``\x1B\x16\x83R\x80\x85``\x1B\x16`\x14\x84\x01RP\x82Qa'(\x81`(\x85\x01` \x87\x01a \x9CV[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[`\x1F\x82\x11\x15a\x03\xBAW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a'\\WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'{W_\x81U`\x01\x01a'hV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x9BWa'\x9Ba!\x0FV[a'\xAF\x81a'\xA9\x84Ta&\xBAV[\x84a'7V[` \x80`\x1F\x83\x11`\x01\x81\x14a'\xE2W_\x84\x15a'\xCBWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua(9V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a(\x10W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a'\xF1V[P\x85\x82\x10\x15a(-W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a(jW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a(NV[P\x92\x96\x95PPPPPPV[_\x82Qa(\x87\x81\x84` \x87\x01a \x9CV[\x91\x90\x91\x01\x92\x91PPV[_`\x01\x82\x01a(\xA2Wa(\xA2a&RV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD\xFE?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\0CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\0\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100fa575f3560e01c806384b0196e11610092578063ad3cb1cc11610062578063ad3cb1cc1461026a578063da53c47d1461029a578063e6317df5146102b9578063e75235b8146102d8578063e7d9e4071461030b575f80fd5b806384b0196e146101ef5780638b218123146102165780639164d0ae1461022a578063960bfe041461024b575f80fd5b806354130ccd116100cd57806354130ccd146101735780635eed7675146101875780637a297f4b146101a65780637df73e27146101c0575f80fd5b80630d8e6e2c146100fe57806335334c23146101285780634f1ef2861461013e57806352d1902d14610151575b5f80fd5b348015610109575f80fd5b5061011261032a565b60405161011f91906120e9565b60405180910390f35b348015610133575f80fd5b5061013c610395565b005b61013c61014c3660046121be565b6103bf565b34801561015c575f80fd5b506101656103de565b60405190815260200161011f565b34801561017e575f80fd5b506101126103f9565b348015610192575f80fd5b5061013c6101a136600461220a565b610415565b3480156101b1575f80fd5b506040515f815260200161011f565b3480156101cb575f80fd5b506101df6101da3660046122a4565b6105a3565b604051901515815260200161011f565b3480156101fa575f80fd5b506102036105cc565b60405161011f97969594939291906122bf565b348015610221575f80fd5b5061016561069a565b348015610235575f80fd5b5061023e6106bd565b60405161011f9190612399565b348015610256575f80fd5b5061013c6102653660046123ab565b61072c565b348015610275575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156102a5575f80fd5b5061013c6102b43660046123c2565b610849565b3480156102c4575f80fd5b506101656102d3366004612474565b610b66565b3480156102e3575f80fd5b507f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0254610165565b348015610316575f80fd5b5061013c6103253660046123c2565b611119565b60606040518060400160405280600d81526020016c24b7383aba2b32b934b334b2b960991b81525061035b5f6111e5565b61036560026111e5565b61036e5f6111e5565b6040516020016103819493929190612516565b604051602081830303815290604052905090565b5f5c5f805d600190810190805b828110156103ba57805c5f825d5f815d5081016103a2565b505050565b6103c7611274565b6103d08261131a565b6103da82826113c4565b5050565b5f6103e7611480565b505f8051602061295d83398151915290565b6040518060a00160405280607f81526020016128de607f913981565b5f8051602061299d833981519152546001600160401b03166001600160401b031660011461045657604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061299d833981519152805460039190600160401b900460ff168061048c575080546001600160401b03808416911610155b156104aa5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b178155604080518082018252601181527024b7383aba2b32b934b334b1b0ba34b7b760791b602080830191909152825180840190935260018352603160f81b908301526105169189896114c9565b6105538585808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250879250610849915050565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050505050565b6001600160a01b03165f9081525f805160206128be833981519152602052604090205460ff1690565b5f60608082808083815f8051602061297d83398151915280549091501580156105f757506001810154155b6106405760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064015b60405180910390fd5b6106486114e3565b61065061159a565b60049290920154604080515f80825260208201909252600f60f81b9c939b509399506001600160401b03600160a01b83041698506001600160a01b03909116965094509092509050565b6040518060a00160405280607f81526020016128de607f91398051906020012081565b60605f5f805160206128be8339815191526001810180546040805160208084028201810190925282815293945083018282801561072157602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610703575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561077c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107a09190612593565b6001600160a01b0316336001600160a01b0316146107d35760405163021bfda160e41b8152336004820152602401610637565b6107dc816115d8565b6040515f805160206128be833981519152907f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea9061083d907f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace019085906125ae565b60405180910390a15050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610899573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108bd9190612593565b6001600160a01b0316336001600160a01b0316146108f05760405163021bfda160e41b8152336004820152602401610637565b81515f81900361091357604051631286e95160e01b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace018054604080516020808402820181019092528281525f805160206128be833981519152935f93919290919083018282801561099657602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610978575b505083519394505f925050505b81811015610a3a575f845f015f8584815181106109c2576109c2612609565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f6101000a81548160ff02191690831515021790555083600101805480610a1257610a1261261d565b5f8281526020902081015f1990810180546001600160a01b03191690550190556001016109a3565b505f5b84811015610b1b575f878281518110610a5857610a58612609565b602002602001015190505f6001600160a01b0316816001600160a01b031603610a94576040516304069ca760e21b815260040160405180910390fd5b6001600160a01b0381165f9081526020869052604090205460ff1615610acd5760405163572de7c960e11b815260040160405180910390fd5b6001600160a01b03165f818152602086815260408220805460ff1916600190811790915587810180548083018255908452919092200180546001600160a01b03191690921790915501610a3d565b50610b25856115d8565b7f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8686604051610b56929190612631565b60405180910390a1505050505050565b5f805f610b7b84875f0151886020015161164f565b90925090506001600160401b03601086901c16468114610bae57604051633d23e4d160e11b815260040160405180910390fd5b8560ff605082901c16846110335786515f819003610bdf576040516359240e8b60e11b815260040160405180910390fd5b5f885f81518110610bf257610bf2612609565b602001015160f81c60f81b60f81c60ff1690505f89600181518110610c1957610c19612609565b016020015160f81c90508382111580610c32575060fe84115b15610c50576040516363df817160e01b815260040160405180910390fd5b5f610c5c826041612666565b610c67846020612666565b610c7290600261267d565b610c7c919061267d565b905080841015610c9f57604051631817ecd760e01b815260040160405180910390fd5b5f836001600160401b03811115610cb857610cb861210f565b604051908082528060200260200182016040528015610ce1578160200160208202803683370190505b5090505f5b84811015610d4157602081028d016022015160ff811615610d1a576040516317df86d560e21b815260040160405180910390fd5b80838381518110610d2d57610d2d612609565b602090810291909101015250600101610ce6565b505f836001600160401b03811115610d5b57610d5b61210f565b604051908082528060200260200182016040528015610d8e57816020015b6060815260200190600190039081610d795790505b5090505f5b84811015610e8557604080516041808252608082019092529060208201818036833701905050828281518110610dcb57610dcb612609565b60200260200101819052505f5b6041811015610e7c578e81610dee846041612666565b610df98a6020612666565b610e0490600261267d565b610e0e919061267d565b610e18919061267d565b81518110610e2857610e28612609565b602001015160f81c60f81b838381518110610e4557610e45612609565b60200260200101518281518110610e5e57610e5e612609565b60200101906001600160f81b03191690815f1a905350600101610dd8565b50600101610d93565b50610ec76040518060a00160405280606081526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f8152602001606081525090565b82815f01819052508f5f015181602001906001600160a01b031690816001600160a01b0316815250508f6020015181604001906001600160a01b031690816001600160a01b031681525050468160600181815250505f848f51610f2a9190612690565b9050806001600160401b03811115610f4457610f4461210f565b6040519080825280601f01601f191660200182016040528015610f6e576020820181803683370190505b5060808301525f5b81811015610fd8578f610f89828861267d565b81518110610f9957610f99612609565b602001015160f81c60f81b83608001518281518110610fba57610fba612609565b60200101906001600160f81b03191690815f1a905350600101610f76565b50610fe3828461168f565b610fec8c6116c2565b838981518110610ffe57610ffe612609565b60200260200101515f1c8a1461102657604051624b1bf160e31b815260040160405180910390fd5b505050505050505061110b565b5f875f8151811061104657611046612609565b016020015160f81c9050818111158061105f575060fe82115b1561107d576040516363df817160e01b815260040160405180910390fd5b5f805b60208110156110e85761109481601f612690565b61109f906008612666565b8a826110ac876020612666565b6110b790600261267d565b6110c1919061267d565b815181106110d1576110d1612609565b016020015160f81c901b9190911790600101611080565b5083811461110857604051624b1bf160e31b815260040160405180910390fd5b50505b5093505050505b9392505050565b5f8051602061299d833981519152805460039190600160401b900460ff168061114f575080546001600160401b03808416911610155b1561116d5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b1781556111988484610849565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050565b60605f6111f1836116d5565b60010190505f816001600160401b0381111561120f5761120f61210f565b6040519080825280601f01601f191660200182016040528015611239576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461124357509392505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806112fa57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166112ee5f8051602061295d833981519152546001600160a01b031690565b6001600160a01b031614155b156113185760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561136a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061138e9190612593565b6001600160a01b0316336001600160a01b0316146113c15760405163021bfda160e41b8152336004820152602401610637565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561141e575060408051601f3d908101601f1916820190925261141b918101906126a3565b60015b61144657604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610637565b5f8051602061295d833981519152811461147657604051632a87526960e21b815260048101829052602401610637565b6103ba83836117ad565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146113185760405163703e46dd60e11b815260040160405180910390fd5b6114d1611802565b6114dd84848484611838565b50505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360280546060915f8051602061297d83398151915291611521906126ba565b80601f016020809104026020016040519081016040528092919081815260200182805461154d906126ba565b80156107215780601f1061156f57610100808354040283529160200191610721565b820191905f5260205f20905b81548152906001019060200180831161157b5750939695505050505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360380546060915f8051602061297d83398151915291611521906126ba565b805f036115f857604051630151f13160e71b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace01545f805160206128be83398151915290821115611649576040516335194e6360e01b815260040160405180910390fd5b60020155565b5f805f80848688604051602001611668939291906126f2565b60408051808303601f190181529190528051602090910120805c9890975095505050505050565b5f611699836118e6565b90506116a581836119bd565b6103ba57604051634b506ccd60e01b815260040160405180910390fd5b6001815d60015f5c0181815d805f5d5050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106117135772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef8100000000831061173f576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc10000831061175d57662386f26fc10000830492506010015b6305f5e1008310611775576305f5e100830492506008015b612710831061178957612710830492506004015b6064831061179b576064830492506002015b600a83106117a7576001015b92915050565b6117b682611b60565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156117fa576103ba8282611bc3565b6103da611c35565b5f8051602061299d83398151915254600160401b900460ff1661131857604051631afcd79f60e31b815260040160405180910390fd5b611840611802565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360480546001600160a01b0384166001600160e01b031990911617600160a01b6001600160401b038416021790555f8051602061297d8339815191527fe910845fd818f61127c84f3586776436a37dead33625056c65162537e33736026118c68682612782565b50600381016118d58582612782565b505f80825560019091015550505050565b5f6117a76040518060a00160405280607f81526020016128de607f913980516020918201208451604051919261191c9201612841565b6040516020818303038152906040528051906020012084602001518560400151866060015187608001516040516020016119569190612876565b60408051601f198184030181528282528051602091820120908301979097528101949094526001600160a01b0392831660608501529116608083015260a082015260c081019190915260e00160405160208183030381529060405280519060200120611c54565b80515f908082036119e157604051635985192160e11b815260040160405180910390fd5b5f611a0a7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace025490565b905080821015611a3057604051632e57ff8360e21b815260048101839052602401610637565b5f826001600160401b03811115611a4957611a4961210f565b604051908082528060200260200182016040528015611a72578160200160208202803683370190505b5090505f805b84811015611b49575f611aa489898481518110611a9757611a97612609565b6020026020010151611c80565b9050611aaf816105a3565b611ad75760405163bf18af4360e01b81526001600160a01b0382166004820152602401610637565b805c611b205780848481518110611af057611af0612609565b6001600160a01b039092166020928302919091019091015282611b1281612891565b935050611b20816001611c94565b848310611b4057611b318484611c9b565b600196505050505050506117a7565b50600101611a78565b50611b548282611c9b565b505f9695505050505050565b806001600160a01b03163b5f03611b9557604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610637565b5f8051602061295d83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051611bdf9190612876565b5f60405180830381855af49150503d805f8114611c17576040519150601f19603f3d011682016040523d82523d5f602084013e611c1c565b606091505b5091509150611c2c858383611cd0565b95945050505050565b34156113185760405163b398979f60e01b815260040160405180910390fd5b5f6117a7611c60611d2c565b8360405161190160f01b8152600281019290925260228201526042902090565b5f80611c8c8484611d3a565b949350505050565b80825d5050565b5f5b818110156103ba57611cc8838281518110611cba57611cba612609565b60200260200101515f611c94565b600101611c9d565b606082611ce557611ce082611d62565b611112565b8151158015611cfc57506001600160a01b0384163b155b15611d2557604051639996b31560e01b81526001600160a01b0385166004820152602401610637565b5080611112565b5f611d35611d8b565b905090565b5f805f80611d488686611e29565b925092509250611d588282611e72565b5090949350505050565b805115611d725780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5f8051602061297d8339815191527f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611dc3611f2a565b611dcb611f92565b60048401546040805160208101959095528401929092526060830152600160a01b81046001600160401b031660808301526001600160a01b031660a082015260c0016040516020818303038152906040528051906020012091505090565b5f805f8351604103611e60576020840151604085015160608601515f1a611e5288828585611fd4565b955095509550505050611e6b565b505081515f91506002905b9250925092565b5f826003811115611e8557611e856128a9565b03611e8e575050565b6001826003811115611ea257611ea26128a9565b03611ec05760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115611ed457611ed46128a9565b03611ef55760405163fce698f760e01b815260048101829052602401610637565b6003826003811115611f0957611f096128a9565b036103da576040516335e2f38360e21b815260048101829052602401610637565b5f5f8051602061297d83398151915281611f426114e3565b805190915015611f5a57805160209091012092915050565b81548015611f69579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f5f8051602061297d83398151915281611faa61159a565b805190915015611fc257805160209091012092915050565b60018201548015611f69579392505050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561200d57505f91506003905082612092565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa15801561205e573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661208957505f925060019150829050612092565b92505f91508190505b9450945094915050565b5f5b838110156120b657818101518382015260200161209e565b50505f910152565b5f81518084526120d581602086016020860161209c565b601f01601f19169290920160200192915050565b602081525f61111260208301846120be565b6001600160a01b03811681146113c1575f80fd5b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b038111828210171561214b5761214b61210f565b604052919050565b5f82601f830112612162575f80fd5b81356001600160401b0381111561217b5761217b61210f565b61218e601f8201601f1916602001612123565b8181528460208386010111156121a2575f80fd5b816020850160208301375f918101602001919091529392505050565b5f80604083850312156121cf575f80fd5b82356121da816120fb565b915060208301356001600160401b038111156121f4575f80fd5b61220085828601612153565b9150509250929050565b5f805f805f6080868803121561221e575f80fd5b8535612229816120fb565b945060208601356001600160401b038082168214612245575f80fd5b9094506040870135908082111561225a575f80fd5b818801915088601f83011261226d575f80fd5b81358181111561227b575f80fd5b8960208260051b850101111561228f575f80fd5b96999598505060200195606001359392505050565b5f602082840312156122b4575f80fd5b8135611112816120fb565b60ff60f81b881681525f602060e060208401526122df60e084018a6120be565b83810360408501526122f1818a6120be565b606085018990526001600160a01b038816608086015260a0850187905284810360c0860152855180825260208088019350909101905f5b8181101561234457835183529284019291840191600101612328565b50909c9b505050505050505050505050565b5f815180845260208085019450602084015f5b8381101561238e5781516001600160a01b031687529582019590820190600101612369565b509495945050505050565b602081525f6111126020830184612356565b5f602082840312156123bb575f80fd5b5035919050565b5f80604083850312156123d3575f80fd5b82356001600160401b03808211156123e9575f80fd5b818501915085601f8301126123fc575f80fd5b81356020828211156124105761241061210f565b8160051b9250612421818401612123565b828152928401810192818101908985111561243a575f80fd5b948201945b848610156124645785359350612454846120fb565b838252948201949082019061243f565b9997909101359750505050505050565b5f805f8385036080811215612487575f80fd5b6040811215612494575f80fd5b50604051604081016001600160401b0382821081831117156124b8576124b861210f565b81604052863591506124c9826120fb565b9082526020860135906124db826120fb565b8160208401528295506040870135945060608701359250808311156124fe575f80fd5b505061250c86828701612153565b9150509250925092565b5f8551612527818460208a0161209c565b61103b60f11b9083019081528551612546816002840160208a0161209c565b808201915050601760f91b806002830152855161256a816003850160208a0161209c565b6003920191820152835161258581600484016020880161209c565b016004019695505050505050565b5f602082840312156125a3575f80fd5b8151611112816120fb565b5f6040820160408352808554808352606085019150865f526020925060205f205f5b828110156125f55781546001600160a01b0316845292840192600191820191016125d0565b505050602093909301939093525092915050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52603160045260245ffd5b604081525f6126436040830185612356565b90508260208301529392505050565b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176117a7576117a7612652565b808201808211156117a7576117a7612652565b818103818111156117a7576117a7612652565b5f602082840312156126b3575f80fd5b5051919050565b600181811c908216806126ce57607f821691505b6020821081036126ec57634e487b7160e01b5f52602260045260245ffd5b50919050565b5f6bffffffffffffffffffffffff19808660601b168352808560601b16601484015250825161272881602885016020870161209c565b91909101602801949350505050565b601f8211156103ba57805f5260205f20601f840160051c8101602085101561275c5750805b601f840160051c820191505b8181101561277b575f8155600101612768565b5050505050565b81516001600160401b0381111561279b5761279b61210f565b6127af816127a984546126ba565b84612737565b602080601f8311600181146127e2575f84156127cb5750858301515b5f19600386901b1c1916600185901b178555612839565b5f85815260208120601f198616915b82811015612810578886015182559484019460019091019084016127f1565b508582101561282d57878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b81515f9082906020808601845b8381101561286a5781518552938201939082019060010161284e565b50929695505050505050565b5f825161288781846020870161209c565b9190910192915050565b5f600182016128a2576128a2612652565b5060010190565b634e487b7160e01b5f52602160045260245ffdfe3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0043697068657274657874566572696669636174696f6e28627974657333325b5d20637448616e646c65732c616464726573732075736572416464726573732c6164647265737320636f6e7472616374416464726573732c75696e7432353620636f6e7472616374436861696e49642c62797465732065787472614461746129360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbce910845fd818f61127c84f3586776436a37dead33625056c65162537e3373600f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80c\x84\xB0\x19n\x11a\0\x92W\x80c\xAD<\xB1\xCC\x11a\0bW\x80c\xAD<\xB1\xCC\x14a\x02jW\x80c\xDAS\xC4}\x14a\x02\x9AW\x80c\xE61}\xF5\x14a\x02\xB9W\x80c\xE7R5\xB8\x14a\x02\xD8W\x80c\xE7\xD9\xE4\x07\x14a\x03\x0BW_\x80\xFD[\x80c\x84\xB0\x19n\x14a\x01\xEFW\x80c\x8B!\x81#\x14a\x02\x16W\x80c\x91d\xD0\xAE\x14a\x02*W\x80c\x96\x0B\xFE\x04\x14a\x02KW_\x80\xFD[\x80cT\x13\x0C\xCD\x11a\0\xCDW\x80cT\x13\x0C\xCD\x14a\x01sW\x80c^\xEDvu\x14a\x01\x87W\x80cz)\x7FK\x14a\x01\xA6W\x80c}\xF7>'\x14a\x01\xC0W_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c53L#\x14a\x01(W\x80cO\x1E\xF2\x86\x14a\x01>W\x80cR\xD1\x90-\x14a\x01QW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03*V[`@Qa\x01\x1F\x91\x90a \xE9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01<a\x03\x95V[\0[a\x01<a\x01L6`\x04a!\xBEV[a\x03\xBFV[4\x80\x15a\x01\\W_\x80\xFD[Pa\x01ea\x03\xDEV[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01~W_\x80\xFD[Pa\x01\x12a\x03\xF9V[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01<a\x01\xA16`\x04a\"\nV[a\x04\x15V[4\x80\x15a\x01\xB1W_\x80\xFD[P`@Q_\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xCBW_\x80\xFD[Pa\x01\xDFa\x01\xDA6`\x04a\"\xA4V[a\x05\xA3V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xFAW_\x80\xFD[Pa\x02\x03a\x05\xCCV[`@Qa\x01\x1F\x97\x96\x95\x94\x93\x92\x91\x90a\"\xBFV[4\x80\x15a\x02!W_\x80\xFD[Pa\x01ea\x06\x9AV[4\x80\x15a\x025W_\x80\xFD[Pa\x02>a\x06\xBDV[`@Qa\x01\x1F\x91\x90a#\x99V[4\x80\x15a\x02VW_\x80\xFD[Pa\x01<a\x02e6`\x04a#\xABV[a\x07,V[4\x80\x15a\x02uW_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x02\xA5W_\x80\xFD[Pa\x01<a\x02\xB46`\x04a#\xC2V[a\x08IV[4\x80\x15a\x02\xC4W_\x80\xFD[Pa\x01ea\x02\xD36`\x04a$tV[a\x0BfV[4\x80\x15a\x02\xE3W_\x80\xFD[P\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02Ta\x01eV[4\x80\x15a\x03\x16W_\x80\xFD[Pa\x01<a\x03%6`\x04a#\xC2V[a\x11\x19V[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01l$\xB78:\xBA+2\xB94\xB34\xB2\xB9`\x99\x1B\x81RPa\x03[_a\x11\xE5V[a\x03e`\x02a\x11\xE5V[a\x03n_a\x11\xE5V[`@Q` \x01a\x03\x81\x94\x93\x92\x91\x90a%\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\\_\x80]`\x01\x90\x81\x01\x90\x80[\x82\x81\x10\x15a\x03\xBAW\x80\\_\x82]_\x81]P\x81\x01a\x03\xA2V[PPPV[a\x03\xC7a\x12tV[a\x03\xD0\x82a\x13\x1AV[a\x03\xDA\x82\x82a\x13\xC4V[PPV[_a\x03\xE7a\x14\x80V[P_\x80Q` a)]\x839\x81Q\x91R\x90V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a(\xDE`\x7F\x919\x81V[_\x80Q` a)\x9D\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x04VW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` a)\x9D\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x04\x8CWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x04\xAAW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\x11\x81Rp$\xB78:\xBA+2\xB94\xB34\xB1\xB0\xBA4\xB7\xB7`y\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\x05\x16\x91\x89\x89a\x14\xC9V[a\x05S\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa\x08I\x91PPV[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a(\xBE\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x90V[_``\x80\x82\x80\x80\x83\x81_\x80Q` a)}\x839\x81Q\x91R\x80T\x90\x91P\x15\x80\x15a\x05\xF7WP`\x01\x81\x01T\x15[a\x06@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x06Ha\x14\xE3V[a\x06Pa\x15\x9AV[`\x04\x92\x90\x92\x01T`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x93\x99P`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x83\x04\x16\x98P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x96P\x94P\x90\x92P\x90PV[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a(\xDE`\x7F\x919\x80Q\x90` \x01 \x81V[``__\x80Q` a(\xBE\x839\x81Q\x91R`\x01\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a\x07!W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x07\x03W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07|W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xA0\x91\x90a%\x93V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xD3W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x067V[a\x07\xDC\x81a\x15\xD8V[`@Q_\x80Q` a(\xBE\x839\x81Q\x91R\x90\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x90a\x08=\x90\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x90\x85\x90a%\xAEV[`@Q\x80\x91\x03\x90\xA1PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x99W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xBD\x91\x90a%\x93V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08\xF0W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x067V[\x81Q_\x81\x90\x03a\t\x13W`@Qc\x12\x86\xE9Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R_\x80Q` a(\xBE\x839\x81Q\x91R\x93_\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\t\x96W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\txW[PP\x83Q\x93\x94P_\x92PPP[\x81\x81\x10\x15a\n:W_\x84_\x01_\x85\x84\x81Q\x81\x10a\t\xC2Wa\t\xC2a&\tV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x01\x80T\x80a\n\x12Wa\n\x12a&\x1DV[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\t\xA3V[P_[\x84\x81\x10\x15a\x0B\x1BW_\x87\x82\x81Q\x81\x10a\nXWa\nXa&\tV[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\n\x94W`@Qc\x04\x06\x9C\xA7`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x86\x90R`@\x90 T`\xFF\x16\x15a\n\xCDW`@QcW-\xE7\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R` \x86\x81R`@\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x87\x81\x01\x80T\x80\x83\x01\x82U\x90\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x92\x17\x90\x91U\x01a\n=V[Pa\x0B%\x85a\x15\xD8V[\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x86\x86`@Qa\x0BV\x92\x91\x90a&1V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80_a\x0B{\x84\x87_\x01Q\x88` \x01Qa\x16OV[\x90\x92P\x90P`\x01`\x01`@\x1B\x03`\x10\x86\x90\x1C\x16F\x81\x14a\x0B\xAEW`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\xFF`P\x82\x90\x1C\x16\x84a\x103W\x86Q_\x81\x90\x03a\x0B\xDFW`@QcY$\x0E\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88_\x81Q\x81\x10a\x0B\xF2Wa\x0B\xF2a&\tV[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P_\x89`\x01\x81Q\x81\x10a\x0C\x19Wa\x0C\x19a&\tV[\x01` \x01Q`\xF8\x1C\x90P\x83\x82\x11\x15\x80a\x0C2WP`\xFE\x84\x11[\x15a\x0CPW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\\\x82`Aa&fV[a\x0Cg\x84` a&fV[a\x0Cr\x90`\x02a&}V[a\x0C|\x91\x90a&}V[\x90P\x80\x84\x10\x15a\x0C\x9FW`@Qc\x18\x17\xEC\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\xB8Wa\x0C\xB8a!\x0FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xE1W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\rAW` \x81\x02\x8D\x01`\"\x01Q`\xFF\x81\x16\x15a\r\x1AW`@Qc\x17\xDF\x86\xD5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x83\x81Q\x81\x10a\r-Wa\r-a&\tV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C\xE6V[P_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\r[Wa\r[a!\x0FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x8EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\ryW\x90P[P\x90P_[\x84\x81\x10\x15a\x0E\x85W`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R\x90` \x82\x01\x81\x806\x837\x01\x90PP\x82\x82\x81Q\x81\x10a\r\xCBWa\r\xCBa&\tV[` \x02` \x01\x01\x81\x90RP_[`A\x81\x10\x15a\x0E|W\x8E\x81a\r\xEE\x84`Aa&fV[a\r\xF9\x8A` a&fV[a\x0E\x04\x90`\x02a&}V[a\x0E\x0E\x91\x90a&}V[a\x0E\x18\x91\x90a&}V[\x81Q\x81\x10a\x0E(Wa\x0E(a&\tV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x0EEWa\x0EEa&\tV[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x0E^Wa\x0E^a&\tV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\r\xD8V[P`\x01\x01a\r\x93V[Pa\x0E\xC7`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x82\x81_\x01\x81\x90RP\x8F_\x01Q\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8F` \x01Q\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPF\x81``\x01\x81\x81RPP_\x84\x8FQa\x0F*\x91\x90a&\x90V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0FDWa\x0FDa!\x0FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0FnW` \x82\x01\x81\x806\x837\x01\x90P[P`\x80\x83\x01R_[\x81\x81\x10\x15a\x0F\xD8W\x8Fa\x0F\x89\x82\x88a&}V[\x81Q\x81\x10a\x0F\x99Wa\x0F\x99a&\tV[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x80\x01Q\x82\x81Q\x81\x10a\x0F\xBAWa\x0F\xBAa&\tV[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x0FvV[Pa\x0F\xE3\x82\x84a\x16\x8FV[a\x0F\xEC\x8Ca\x16\xC2V[\x83\x89\x81Q\x81\x10a\x0F\xFEWa\x0F\xFEa&\tV[` \x02` \x01\x01Q_\x1C\x8A\x14a\x10&W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPa\x11\x0BV[_\x87_\x81Q\x81\x10a\x10FWa\x10Fa&\tV[\x01` \x01Q`\xF8\x1C\x90P\x81\x81\x11\x15\x80a\x10_WP`\xFE\x82\x11[\x15a\x10}W`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80[` \x81\x10\x15a\x10\xE8Wa\x10\x94\x81`\x1Fa&\x90V[a\x10\x9F\x90`\x08a&fV[\x8A\x82a\x10\xAC\x87` a&fV[a\x10\xB7\x90`\x02a&}V[a\x10\xC1\x91\x90a&}V[\x81Q\x81\x10a\x10\xD1Wa\x10\xD1a&\tV[\x01` \x01Q`\xF8\x1C\x90\x1B\x91\x90\x91\x17\x90`\x01\x01a\x10\x80V[P\x83\x81\x14a\x11\x08W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP[P\x93PPPP[\x93\x92PPPV[_\x80Q` a)\x9D\x839\x81Q\x91R\x80T`\x03\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x11OWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x11mW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81Ua\x11\x98\x84\x84a\x08IV[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPV[``_a\x11\xF1\x83a\x16\xD5V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x0FWa\x12\x0Fa!\x0FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x129W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12CWP\x93\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x12\xFAWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x12\xEE_\x80Q` a)]\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x13\x18W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13jW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x8E\x91\x90a%\x93V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x13\xC1W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x067V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x14\x1EWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x14\x1B\x91\x81\x01\x90a&\xA3V[`\x01[a\x14FW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x067V[_\x80Q` a)]\x839\x81Q\x91R\x81\x14a\x14vW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x067V[a\x03\xBA\x83\x83a\x17\xADV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13\x18W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xD1a\x18\x02V[a\x14\xDD\x84\x84\x84\x84a\x188V[PPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02\x80T``\x91_\x80Q` a)}\x839\x81Q\x91R\x91a\x15!\x90a&\xBAV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15M\x90a&\xBAV[\x80\x15a\x07!W\x80`\x1F\x10a\x15oWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07!V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15{WP\x93\x96\x95PPPPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x03\x80T``\x91_\x80Q` a)}\x839\x81Q\x91R\x91a\x15!\x90a&\xBAV[\x80_\x03a\x15\xF8W`@Qc\x01Q\xF11`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01T_\x80Q` a(\xBE\x839\x81Q\x91R\x90\x82\x11\x15a\x16IW`@Qc5\x19Nc`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x01UV[_\x80_\x80\x84\x86\x88`@Q` \x01a\x16h\x93\x92\x91\x90a&\xF2V[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x80\\\x98\x90\x97P\x95PPPPPPV[_a\x16\x99\x83a\x18\xE6V[\x90Pa\x16\xA5\x81\x83a\x19\xBDV[a\x03\xBAW`@QcKPl\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81]`\x01_\\\x01\x81\x81]\x80_]PPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x17\x13Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x17?Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x17]Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x17uWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x17\x89Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x17\x9BW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x17\xA7W`\x01\x01[\x92\x91PPV[a\x17\xB6\x82a\x1B`V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x17\xFAWa\x03\xBA\x82\x82a\x1B\xC3V[a\x03\xDAa\x1C5V[_\x80Q` a)\x9D\x839\x81Q\x91RT`\x01`@\x1B\x90\x04`\xFF\x16a\x13\x18W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18@a\x18\x02V[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x04\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1B`\x01`\x01`@\x1B\x03\x84\x16\x02\x17\x90U_\x80Q` a)}\x839\x81Q\x91R\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02a\x18\xC6\x86\x82a'\x82V[P`\x03\x81\x01a\x18\xD5\x85\x82a'\x82V[P_\x80\x82U`\x01\x90\x91\x01UPPPPV[_a\x17\xA7`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a(\xDE`\x7F\x919\x80Q` \x91\x82\x01 \x84Q`@Q\x91\x92a\x19\x1C\x92\x01a(AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a\x19V\x91\x90a(vV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16``\x85\x01R\x91\x16`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1CTV[\x80Q_\x90\x80\x82\x03a\x19\xE1W`@QcY\x85\x19!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1A\n\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02T\x90V[\x90P\x80\x82\x10\x15a\x1A0W`@Qc.W\xFF\x83`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x067V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AIWa\x1AIa!\x0FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1ArW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x84\x81\x10\x15a\x1BIW_a\x1A\xA4\x89\x89\x84\x81Q\x81\x10a\x1A\x97Wa\x1A\x97a&\tV[` \x02` \x01\x01Qa\x1C\x80V[\x90Pa\x1A\xAF\x81a\x05\xA3V[a\x1A\xD7W`@Qc\xBF\x18\xAFC`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x067V[\x80\\a\x1B W\x80\x84\x84\x81Q\x81\x10a\x1A\xF0Wa\x1A\xF0a&\tV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x1B\x12\x81a(\x91V[\x93PPa\x1B \x81`\x01a\x1C\x94V[\x84\x83\x10a\x1B@Wa\x1B1\x84\x84a\x1C\x9BV[`\x01\x96PPPPPPPa\x17\xA7V[P`\x01\x01a\x1AxV[Pa\x1BT\x82\x82a\x1C\x9BV[P_\x96\x95PPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1B\x95W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x067V[_\x80Q` a)]\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1B\xDF\x91\x90a(vV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1C\x17W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1C\x1CV[``\x91P[P\x91P\x91Pa\x1C,\x85\x83\x83a\x1C\xD0V[\x95\x94PPPPPV[4\x15a\x13\x18W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x17\xA7a\x1C`a\x1D,V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80a\x1C\x8C\x84\x84a\x1D:V[\x94\x93PPPPV[\x80\x82]PPV[_[\x81\x81\x10\x15a\x03\xBAWa\x1C\xC8\x83\x82\x81Q\x81\x10a\x1C\xBAWa\x1C\xBAa&\tV[` \x02` \x01\x01Q_a\x1C\x94V[`\x01\x01a\x1C\x9DV[``\x82a\x1C\xE5Wa\x1C\xE0\x82a\x1DbV[a\x11\x12V[\x81Q\x15\x80\x15a\x1C\xFCWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1D%W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x067V[P\x80a\x11\x12V[_a\x1D5a\x1D\x8BV[\x90P\x90V[_\x80_\x80a\x1DH\x86\x86a\x1E)V[\x92P\x92P\x92Pa\x1DX\x82\x82a\x1ErV[P\x90\x94\x93PPPPV[\x80Q\x15a\x1DrW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x80Q` a)}\x839\x81Q\x91R\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1D\xC3a\x1F*V[a\x1D\xCBa\x1F\x92V[`\x04\x84\x01T`@\x80Q` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R`\x01`\xA0\x1B\x81\x04`\x01`\x01`@\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x90V[_\x80_\x83Q`A\x03a\x1E`W` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x1ER\x88\x82\x85\x85a\x1F\xD4V[\x95P\x95P\x95PPPPa\x1EkV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x1E\x85Wa\x1E\x85a(\xA9V[\x03a\x1E\x8EWPPV[`\x01\x82`\x03\x81\x11\x15a\x1E\xA2Wa\x1E\xA2a(\xA9V[\x03a\x1E\xC0W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x1E\xD4Wa\x1E\xD4a(\xA9V[\x03a\x1E\xF5W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x067V[`\x03\x82`\x03\x81\x11\x15a\x1F\tWa\x1F\ta(\xA9V[\x03a\x03\xDAW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x067V[__\x80Q` a)}\x839\x81Q\x91R\x81a\x1FBa\x14\xE3V[\x80Q\x90\x91P\x15a\x1FZW\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15a\x1FiW\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[__\x80Q` a)}\x839\x81Q\x91R\x81a\x1F\xAAa\x15\x9AV[\x80Q\x90\x91P\x15a\x1F\xC2W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15a\x1FiW\x93\x92PPPV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a \rWP_\x91P`\x03\x90P\x82a \x92V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a ^W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a \x89WP_\x92P`\x01\x91P\x82\x90Pa \x92V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_[\x83\x81\x10\x15a \xB6W\x81\x81\x01Q\x83\x82\x01R` \x01a \x9EV[PP_\x91\x01RV[_\x81Q\x80\x84Ra \xD5\x81` \x86\x01` \x86\x01a \x9CV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x11\x12` \x83\x01\x84a \xBEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\xC1W_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a!KWa!Ka!\x0FV[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a!bW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!{Wa!{a!\x0FV[a!\x8E`\x1F\x82\x01`\x1F\x19\x16` \x01a!#V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a!\xA2W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a!\xCFW_\x80\xFD[\x825a!\xDA\x81a \xFBV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xF4W_\x80\xFD[a\"\0\x85\x82\x86\x01a!SV[\x91PP\x92P\x92\x90PV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a\"\x1EW_\x80\xFD[\x855a\")\x81a \xFBV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x16\x82\x14a\"EW_\x80\xFD[\x90\x94P`@\x87\x015\x90\x80\x82\x11\x15a\"ZW_\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\"mW_\x80\xFD[\x815\x81\x81\x11\x15a\"{W_\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"\x8FW_\x80\xFD[\x96\x99\x95\x98PP` \x01\x95``\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\"\xB4W_\x80\xFD[\x815a\x11\x12\x81a \xFBV[`\xFF`\xF8\x1B\x88\x16\x81R_` `\xE0` \x84\x01Ra\"\xDF`\xE0\x84\x01\x8Aa \xBEV[\x83\x81\x03`@\x85\x01Ra\"\xF1\x81\x8Aa \xBEV[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R` \x80\x88\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a#DW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a#(V[P\x90\x9C\x9BPPPPPPPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a#\x8EW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a#iV[P\x94\x95\x94PPPPPV[` \x81R_a\x11\x12` \x83\x01\x84a#VV[_` \x82\x84\x03\x12\x15a#\xBBW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a#\xD3W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#\xE9W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a#\xFCW_\x80\xFD[\x815` \x82\x82\x11\x15a$\x10Wa$\x10a!\x0FV[\x81`\x05\x1B\x92Pa$!\x81\x84\x01a!#V[\x82\x81R\x92\x84\x01\x81\x01\x92\x81\x81\x01\x90\x89\x85\x11\x15a$:W_\x80\xFD[\x94\x82\x01\x94[\x84\x86\x10\x15a$dW\x855\x93Pa$T\x84a \xFBV[\x83\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$?V[\x99\x97\x90\x91\x015\x97PPPPPPPV[_\x80_\x83\x85\x03`\x80\x81\x12\x15a$\x87W_\x80\xFD[`@\x81\x12\x15a$\x94W_\x80\xFD[P`@Q`@\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a$\xB8Wa$\xB8a!\x0FV[\x81`@R\x865\x91Pa$\xC9\x82a \xFBV[\x90\x82R` \x86\x015\x90a$\xDB\x82a \xFBV[\x81` \x84\x01R\x82\x95P`@\x87\x015\x94P``\x87\x015\x92P\x80\x83\x11\x15a$\xFEW_\x80\xFD[PPa%\x0C\x86\x82\x87\x01a!SV[\x91PP\x92P\x92P\x92V[_\x85Qa%'\x81\x84` \x8A\x01a \x9CV[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa%F\x81`\x02\x84\x01` \x8A\x01a \x9CV[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa%j\x81`\x03\x85\x01` \x8A\x01a \x9CV[`\x03\x92\x01\x91\x82\x01R\x83Qa%\x85\x81`\x04\x84\x01` \x88\x01a \x9CV[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a%\xA3W_\x80\xFD[\x81Qa\x11\x12\x81a \xFBV[_`@\x82\x01`@\x83R\x80\x85T\x80\x83R``\x85\x01\x91P\x86_R` \x92P` _ _[\x82\x81\x10\x15a%\xF5W\x81T`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92`\x01\x91\x82\x01\x91\x01a%\xD0V[PPP` \x93\x90\x93\x01\x93\x90\x93RP\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`@\x81R_a&C`@\x83\x01\x85a#VV[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x17\xA7Wa\x17\xA7a&RV[\x80\x82\x01\x80\x82\x11\x15a\x17\xA7Wa\x17\xA7a&RV[\x81\x81\x03\x81\x81\x11\x15a\x17\xA7Wa\x17\xA7a&RV[_` \x82\x84\x03\x12\x15a&\xB3W_\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a&\xCEW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a&\xECWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x86``\x1B\x16\x83R\x80\x85``\x1B\x16`\x14\x84\x01RP\x82Qa'(\x81`(\x85\x01` \x87\x01a \x9CV[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[`\x1F\x82\x11\x15a\x03\xBAW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a'\\WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a'{W_\x81U`\x01\x01a'hV[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x9BWa'\x9Ba!\x0FV[a'\xAF\x81a'\xA9\x84Ta&\xBAV[\x84a'7V[` \x80`\x1F\x83\x11`\x01\x81\x14a'\xE2W_\x84\x15a'\xCBWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua(9V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a(\x10W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a'\xF1V[P\x85\x82\x10\x15a(-W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a(jW\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a(NV[P\x92\x96\x95PPPPPPV[_\x82Qa(\x87\x81\x84` \x87\x01a \x9CV[\x91\x90\x91\x01\x92\x91PPV[_`\x01\x82\x01a(\xA2Wa(\xA2a&RV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD\xFE?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\0CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\0\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
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
    /**Custom error with signature `CoprocessorAlreadySigner()` and selector `0xae5bcf92`.
```solidity
error CoprocessorAlreadySigner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CoprocessorAlreadySigner;
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
        impl ::core::convert::From<CoprocessorAlreadySigner>
        for UnderlyingRustTuple<'_> {
            fn from(value: CoprocessorAlreadySigner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CoprocessorAlreadySigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CoprocessorAlreadySigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CoprocessorAlreadySigner()";
            const SELECTOR: [u8; 4] = [174u8, 91u8, 207u8, 146u8];
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
    /**Custom error with signature `CoprocessorSignerNull()` and selector `0x101a729c`.
```solidity
error CoprocessorSignerNull();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CoprocessorSignerNull;
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
        impl ::core::convert::From<CoprocessorSignerNull> for UnderlyingRustTuple<'_> {
            fn from(value: CoprocessorSignerNull) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CoprocessorSignerNull {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CoprocessorSignerNull {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CoprocessorSignerNull()";
            const SELECTOR: [u8; 4] = [16u8, 26u8, 114u8, 156u8];
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
    /**Custom error with signature `DeserializingInputProofFail()` and selector `0x1817ecd7`.
```solidity
error DeserializingInputProofFail();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeserializingInputProofFail;
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
        impl ::core::convert::From<DeserializingInputProofFail>
        for UnderlyingRustTuple<'_> {
            fn from(value: DeserializingInputProofFail) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DeserializingInputProofFail {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DeserializingInputProofFail {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DeserializingInputProofFail()";
            const SELECTOR: [u8; 4] = [24u8, 23u8, 236u8, 215u8];
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
    /**Custom error with signature `EmptyInputProof()` and selector `0xb2481d16`.
```solidity
error EmptyInputProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyInputProof;
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
        impl ::core::convert::From<EmptyInputProof> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyInputProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyInputProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyInputProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyInputProof()";
            const SELECTOR: [u8; 4] = [178u8, 72u8, 29u8, 22u8];
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
    /**Custom error with signature `InvalidChainId()` and selector `0x7a47c9a2`.
```solidity
error InvalidChainId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidChainId;
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
        impl ::core::convert::From<InvalidChainId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidChainId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidChainId()";
            const SELECTOR: [u8; 4] = [122u8, 71u8, 201u8, 162u8];
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
    /**Custom error with signature `InvalidHandleVersion()` and selector `0x5f7e1b54`.
```solidity
error InvalidHandleVersion();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidHandleVersion;
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
        impl ::core::convert::From<InvalidHandleVersion> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidHandleVersion) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidHandleVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidHandleVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidHandleVersion()";
            const SELECTOR: [u8; 4] = [95u8, 126u8, 27u8, 84u8];
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
    /**Custom error with signature `InvalidIndex()` and selector `0x63df8171`.
```solidity
error InvalidIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidIndex;
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
        impl ::core::convert::From<InvalidIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidIndex()";
            const SELECTOR: [u8; 4] = [99u8, 223u8, 129u8, 113u8];
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
    /**Custom error with signature `InvalidInputHandle()` and selector `0x0258df88`.
```solidity
error InvalidInputHandle();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInputHandle;
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
        impl ::core::convert::From<InvalidInputHandle> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidInputHandle) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidInputHandle {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidInputHandle {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidInputHandle()";
            const SELECTOR: [u8; 4] = [2u8, 88u8, 223u8, 136u8];
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
    /**Custom error with signature `InvalidSigner(address)` and selector `0xbf18af43`.
```solidity
error InvalidSigner(address signerRecovered);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSigner {
        #[allow(missing_docs)]
        pub signerRecovered: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<InvalidSigner> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSigner) -> Self {
                (value.signerRecovered,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signerRecovered: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSigner(address)";
            const SELECTOR: [u8; 4] = [191u8, 24u8, 175u8, 67u8];
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
                        &self.signerRecovered,
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
    /**Custom error with signature `NotASigner()` and selector `0xda0357f7`.
```solidity
error NotASigner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotASigner;
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
        impl ::core::convert::From<NotASigner> for UnderlyingRustTuple<'_> {
            fn from(value: NotASigner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotASigner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotASigner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotASigner()";
            const SELECTOR: [u8; 4] = [218u8, 3u8, 87u8, 247u8];
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
    /**Custom error with signature `SignatureThresholdNotReached(uint256)` and selector `0xb95ffe0c`.
```solidity
error SignatureThresholdNotReached(uint256 numSignatures);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureThresholdNotReached {
        #[allow(missing_docs)]
        pub numSignatures: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<SignatureThresholdNotReached>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureThresholdNotReached) -> Self {
                (value.numSignatures,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureThresholdNotReached {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { numSignatures: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureThresholdNotReached {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureThresholdNotReached(uint256)";
            const SELECTOR: [u8; 4] = [185u8, 95u8, 254u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.numSignatures),
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
    /**Custom error with signature `SignaturesVerificationFailed()` and selector `0x4b506ccd`.
```solidity
error SignaturesVerificationFailed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignaturesVerificationFailed;
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
        impl ::core::convert::From<SignaturesVerificationFailed>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignaturesVerificationFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignaturesVerificationFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignaturesVerificationFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignaturesVerificationFailed()";
            const SELECTOR: [u8; 4] = [75u8, 80u8, 108u8, 205u8];
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
    /**Custom error with signature `SignersSetIsEmpty()` and selector `0x1286e951`.
```solidity
error SignersSetIsEmpty();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignersSetIsEmpty;
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
        impl ::core::convert::From<SignersSetIsEmpty> for UnderlyingRustTuple<'_> {
            fn from(value: SignersSetIsEmpty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignersSetIsEmpty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignersSetIsEmpty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignersSetIsEmpty()";
            const SELECTOR: [u8; 4] = [18u8, 134u8, 233u8, 81u8];
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
    /**Custom error with signature `ThresholdIsAboveNumberOfSigners()` and selector `0x35194e63`.
```solidity
error ThresholdIsAboveNumberOfSigners();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ThresholdIsAboveNumberOfSigners;
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
        impl ::core::convert::From<ThresholdIsAboveNumberOfSigners>
        for UnderlyingRustTuple<'_> {
            fn from(value: ThresholdIsAboveNumberOfSigners) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ThresholdIsAboveNumberOfSigners {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ThresholdIsAboveNumberOfSigners {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ThresholdIsAboveNumberOfSigners()";
            const SELECTOR: [u8; 4] = [53u8, 25u8, 78u8, 99u8];
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
    /**Custom error with signature `ThresholdIsNull()` and selector `0xa8f89880`.
```solidity
error ThresholdIsNull();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ThresholdIsNull;
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
        impl ::core::convert::From<ThresholdIsNull> for UnderlyingRustTuple<'_> {
            fn from(value: ThresholdIsNull) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ThresholdIsNull {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ThresholdIsNull {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ThresholdIsNull()";
            const SELECTOR: [u8; 4] = [168u8, 248u8, 152u8, 128u8];
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
    /**Custom error with signature `ZeroSignature()` and selector `0xb30a3242`.
```solidity
error ZeroSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroSignature;
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
        impl ::core::convert::From<ZeroSignature> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroSignature()";
            const SELECTOR: [u8; 4] = [179u8, 10u8, 50u8, 66u8];
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
    /**Event with signature `NewContextSet(address[],uint256)` and selector `0x1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea`.
```solidity
event NewContextSet(address[] newSignersSet, uint256 newThreshold);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewContextSet {
        #[allow(missing_docs)]
        pub newSignersSet: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub newThreshold: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NewContextSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NewContextSet(address[],uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                29u8, 205u8, 126u8, 29u8, 233u8, 22u8, 173u8, 59u8, 224u8, 193u8, 9u8,
                121u8, 104u8, 2u8, 152u8, 153u8, 226u8, 231u8, 208u8, 25u8, 92u8, 250u8,
                105u8, 103u8, 225u8, 101u8, 32u8, 192u8, 232u8, 208u8, 124u8, 234u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newSignersSet: data.0,
                    newThreshold: data.1,
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.newSignersSet),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newThreshold),
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
        impl alloy_sol_types::private::IntoLogData for NewContextSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewContextSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewContextSet) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `EIP712_INPUT_VERIFICATION_TYPE()` and selector `0x54130ccd`.
```solidity
function EIP712_INPUT_VERIFICATION_TYPE() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIP712_INPUT_VERIFICATION_TYPECall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`EIP712_INPUT_VERIFICATION_TYPE()`](EIP712_INPUT_VERIFICATION_TYPECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIP712_INPUT_VERIFICATION_TYPEReturn {
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
            impl ::core::convert::From<EIP712_INPUT_VERIFICATION_TYPECall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EIP712_INPUT_VERIFICATION_TYPECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EIP712_INPUT_VERIFICATION_TYPECall {
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
            impl ::core::convert::From<EIP712_INPUT_VERIFICATION_TYPEReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EIP712_INPUT_VERIFICATION_TYPEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EIP712_INPUT_VERIFICATION_TYPEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EIP712_INPUT_VERIFICATION_TYPECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EIP712_INPUT_VERIFICATION_TYPE()";
            const SELECTOR: [u8; 4] = [84u8, 19u8, 12u8, 205u8];
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
                        let r: EIP712_INPUT_VERIFICATION_TYPEReturn = r.into();
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
                        let r: EIP712_INPUT_VERIFICATION_TYPEReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `EIP712_INPUT_VERIFICATION_TYPEHASH()` and selector `0x8b218123`.
```solidity
function EIP712_INPUT_VERIFICATION_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIP712_INPUT_VERIFICATION_TYPEHASHCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`EIP712_INPUT_VERIFICATION_TYPEHASH()`](EIP712_INPUT_VERIFICATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EIP712_INPUT_VERIFICATION_TYPEHASHReturn {
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
            impl ::core::convert::From<EIP712_INPUT_VERIFICATION_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: EIP712_INPUT_VERIFICATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EIP712_INPUT_VERIFICATION_TYPEHASHCall {
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
            impl ::core::convert::From<EIP712_INPUT_VERIFICATION_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: EIP712_INPUT_VERIFICATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for EIP712_INPUT_VERIFICATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for EIP712_INPUT_VERIFICATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EIP712_INPUT_VERIFICATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [139u8, 33u8, 129u8, 35u8];
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
                        let r: EIP712_INPUT_VERIFICATION_TYPEHASHReturn = r.into();
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
                        let r: EIP712_INPUT_VERIFICATION_TYPEHASHReturn = r.into();
                        r._0
                    })
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
    /**Function with signature `cleanTransientStorage()` and selector `0x35334c23`.
```solidity
function cleanTransientStorage() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cleanTransientStorageCall;
    ///Container type for the return parameters of the [`cleanTransientStorage()`](cleanTransientStorageCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cleanTransientStorageReturn {}
    #[allow(
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
            impl ::core::convert::From<cleanTransientStorageCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cleanTransientStorageCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cleanTransientStorageCall {
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
            impl ::core::convert::From<cleanTransientStorageReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cleanTransientStorageReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cleanTransientStorageReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl cleanTransientStorageReturn {
            fn _tokenize(
                &self,
            ) -> <cleanTransientStorageCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cleanTransientStorageCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cleanTransientStorageReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cleanTransientStorage()";
            const SELECTOR: [u8; 4] = [53u8, 51u8, 76u8, 35u8];
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
                cleanTransientStorageReturn::_tokenize(ret)
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
    /**Function with signature `defineNewContext(address[],uint256)` and selector `0xda53c47d`.
```solidity
function defineNewContext(address[] memory newSignersSet, uint256 newThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewContextCall {
        #[allow(missing_docs)]
        pub newSignersSet: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub newThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`defineNewContext(address[],uint256)`](defineNewContextCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defineNewContextReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defineNewContextCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewContextCall) -> Self {
                    (value.newSignersSet, value.newThreshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewContextCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newSignersSet: tuple.0,
                        newThreshold: tuple.1,
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
            impl ::core::convert::From<defineNewContextReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: defineNewContextReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defineNewContextReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl defineNewContextReturn {
            fn _tokenize(
                &self,
            ) -> <defineNewContextCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defineNewContextCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defineNewContextReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defineNewContext(address[],uint256)";
            const SELECTOR: [u8; 4] = [218u8, 83u8, 196u8, 125u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.newSignersSet),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newThreshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                defineNewContextReturn::_tokenize(ret)
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
    /**Function with signature `getHandleVersion()` and selector `0x7a297f4b`.
```solidity
function getHandleVersion() external pure returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHandleVersionCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getHandleVersion()`](getHandleVersionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHandleVersionReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<getHandleVersionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHandleVersionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHandleVersionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getHandleVersionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHandleVersionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHandleVersionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHandleVersionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHandleVersion()";
            const SELECTOR: [u8; 4] = [122u8, 41u8, 127u8, 75u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getHandleVersionReturn = r.into();
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
                        let r: getHandleVersionReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getThreshold()` and selector `0xe75235b8`.
```solidity
function getThreshold() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getThresholdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getThreshold()`](getThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getThresholdReturn {
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
            impl ::core::convert::From<getThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getThresholdCall {
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
            impl ::core::convert::From<getThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getThreshold()";
            const SELECTOR: [u8; 4] = [231u8, 82u8, 53u8, 184u8];
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
                        let r: getThresholdReturn = r.into();
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
                        let r: getThresholdReturn = r.into();
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
    /**Function with signature `initializeFromEmptyProxy(address,uint64,address[],uint256)` and selector `0x5eed7675`.
```solidity
function initializeFromEmptyProxy(address verifyingContractSource, uint64 chainIDSource, address[] memory initialSigners, uint256 initialThreshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeFromEmptyProxyCall {
        #[allow(missing_docs)]
        pub verifyingContractSource: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub chainIDSource: u64,
        #[allow(missing_docs)]
        pub initialSigners: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub initialThreshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initializeFromEmptyProxy(address,uint64,address[],uint256)`](initializeFromEmptyProxyCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u64,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                        value.verifyingContractSource,
                        value.chainIDSource,
                        value.initialSigners,
                        value.initialThreshold,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeFromEmptyProxyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        verifyingContractSource: tuple.0,
                        chainIDSource: tuple.1,
                        initialSigners: tuple.2,
                        initialThreshold: tuple.3,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeFromEmptyProxyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeFromEmptyProxy(address,uint64,address[],uint256)";
            const SELECTOR: [u8; 4] = [94u8, 237u8, 118u8, 117u8];
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
                        &self.verifyingContractSource,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainIDSource),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialSigners),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialThreshold),
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
    /**Function with signature `isSigner(address)` and selector `0x7df73e27`.
```solidity
function isSigner(address account) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isSignerCall {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isSigner(address)`](isSignerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isSignerReturn {
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
            impl ::core::convert::From<isSignerCall> for UnderlyingRustTuple<'_> {
                fn from(value: isSignerCall) -> Self {
                    (value.account,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSignerCall {
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
            impl ::core::convert::From<isSignerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isSignerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isSignerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isSignerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isSigner(address)";
            const SELECTOR: [u8; 4] = [125u8, 247u8, 62u8, 39u8];
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
                        let r: isSignerReturn = r.into();
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
                        let r: isSignerReturn = r.into();
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
    /**Function with signature `reinitializeV2(address[],uint256)` and selector `0xe7d9e407`.
```solidity
function reinitializeV2(address[] memory newSignersSet, uint256 threshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV2Call {
        #[allow(missing_docs)]
        pub newSignersSet: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`reinitializeV2(address[],uint256)`](reinitializeV2Call) function.
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value.newSignersSet, value.threshold)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newSignersSet: tuple.0,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV2(address[],uint256)";
            const SELECTOR: [u8; 4] = [231u8, 217u8, 228u8, 7u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.newSignersSet),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
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
    /**Function with signature `setThreshold(uint256)` and selector `0x960bfe04`.
```solidity
function setThreshold(uint256 threshold) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setThresholdCall {
        #[allow(missing_docs)]
        pub threshold: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setThreshold(uint256)`](setThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setThresholdReturn {}
    #[allow(
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
            impl ::core::convert::From<setThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: setThresholdCall) -> Self {
                    (value.threshold,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { threshold: tuple.0 }
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
            impl ::core::convert::From<setThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setThresholdReturn {
            fn _tokenize(
                &self,
            ) -> <setThresholdCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setThreshold(uint256)";
            const SELECTOR: [u8; 4] = [150u8, 11u8, 254u8, 4u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.threshold),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setThresholdReturn::_tokenize(ret)
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `verifyInput((address,address),bytes32,bytes)` and selector `0xe6317df5`.
```solidity
function verifyInput(FHEVMExecutor.ContextUserInputs memory context, bytes32 inputHandle, bytes memory inputProof) external returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyInputCall {
        #[allow(missing_docs)]
        pub context: <FHEVMExecutor::ContextUserInputs as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub inputHandle: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub inputProof: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyInput((address,address),bytes32,bytes)`](verifyInputCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyInputReturn {
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
            type UnderlyingSolTuple<'a> = (
                FHEVMExecutor::ContextUserInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <FHEVMExecutor::ContextUserInputs as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<verifyInputCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyInputCall) -> Self {
                    (value.context, value.inputHandle, value.inputProof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyInputCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        context: tuple.0,
                        inputHandle: tuple.1,
                        inputProof: tuple.2,
                    }
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
            impl ::core::convert::From<verifyInputReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyInputReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyInputReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyInputCall {
            type Parameters<'a> = (
                FHEVMExecutor::ContextUserInputs,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyInput((address,address),bytes32,bytes)";
            const SELECTOR: [u8; 4] = [230u8, 49u8, 125u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <FHEVMExecutor::ContextUserInputs as alloy_sol_types::SolType>::tokenize(
                        &self.context,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.inputHandle),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.inputProof,
                    ),
                )
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
                        let r: verifyInputReturn = r.into();
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
                        let r: verifyInputReturn = r.into();
                        r._0
                    })
            }
        }
    };
    ///Container for all the [`InputVerifier`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum InputVerifierCalls {
        #[allow(missing_docs)]
        EIP712_INPUT_VERIFICATION_TYPE(EIP712_INPUT_VERIFICATION_TYPECall),
        #[allow(missing_docs)]
        EIP712_INPUT_VERIFICATION_TYPEHASH(EIP712_INPUT_VERIFICATION_TYPEHASHCall),
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        cleanTransientStorage(cleanTransientStorageCall),
        #[allow(missing_docs)]
        defineNewContext(defineNewContextCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getCoprocessorSigners(getCoprocessorSignersCall),
        #[allow(missing_docs)]
        getHandleVersion(getHandleVersionCall),
        #[allow(missing_docs)]
        getThreshold(getThresholdCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isSigner(isSignerCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV2(reinitializeV2Call),
        #[allow(missing_docs)]
        setThreshold(setThresholdCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        verifyInput(verifyInputCall),
    }
    #[automatically_derived]
    impl InputVerifierCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 142u8, 110u8, 44u8],
            [53u8, 51u8, 76u8, 35u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [84u8, 19u8, 12u8, 205u8],
            [94u8, 237u8, 118u8, 117u8],
            [122u8, 41u8, 127u8, 75u8],
            [125u8, 247u8, 62u8, 39u8],
            [132u8, 176u8, 25u8, 110u8],
            [139u8, 33u8, 129u8, 35u8],
            [145u8, 100u8, 208u8, 174u8],
            [150u8, 11u8, 254u8, 4u8],
            [173u8, 60u8, 177u8, 204u8],
            [218u8, 83u8, 196u8, 125u8],
            [230u8, 49u8, 125u8, 245u8],
            [231u8, 82u8, 53u8, 184u8],
            [231u8, 217u8, 228u8, 7u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for InputVerifierCalls {
        const NAME: &'static str = "InputVerifierCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::EIP712_INPUT_VERIFICATION_TYPE(_) => {
                    <EIP712_INPUT_VERIFICATION_TYPECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::EIP712_INPUT_VERIFICATION_TYPEHASH(_) => {
                    <EIP712_INPUT_VERIFICATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cleanTransientStorage(_) => {
                    <cleanTransientStorageCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defineNewContext(_) => {
                    <defineNewContextCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCoprocessorSigners(_) => {
                    <getCoprocessorSignersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHandleVersion(_) => {
                    <getHandleVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getThreshold(_) => {
                    <getThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isSigner(_) => <isSignerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV2(_) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setThreshold(_) => {
                    <setThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyInput(_) => {
                    <verifyInputCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<InputVerifierCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn cleanTransientStorage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <cleanTransientStorageCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::cleanTransientStorage)
                    }
                    cleanTransientStorage
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn EIP712_INPUT_VERIFICATION_TYPE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <EIP712_INPUT_VERIFICATION_TYPECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::EIP712_INPUT_VERIFICATION_TYPE)
                    }
                    EIP712_INPUT_VERIFICATION_TYPE
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getHandleVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getHandleVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::getHandleVersion)
                    }
                    getHandleVersion
                },
                {
                    fn isSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <isSignerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(InputVerifierCalls::isSigner)
                    }
                    isSigner
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn EIP712_INPUT_VERIFICATION_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <EIP712_INPUT_VERIFICATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::EIP712_INPUT_VERIFICATION_TYPEHASH)
                    }
                    EIP712_INPUT_VERIFICATION_TYPEHASH
                },
                {
                    fn getCoprocessorSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::getCoprocessorSigners)
                    }
                    getCoprocessorSigners
                },
                {
                    fn setThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <setThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::setThreshold)
                    }
                    setThreshold
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn defineNewContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <defineNewContextCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::defineNewContext)
                    }
                    defineNewContext
                },
                {
                    fn verifyInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <verifyInputCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::verifyInput)
                    }
                    verifyInput
                },
                {
                    fn getThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::getThreshold)
                    }
                    getThreshold
                },
                {
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::reinitializeV2)
                    }
                    reinitializeV2
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
            ) -> alloy_sol_types::Result<InputVerifierCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn cleanTransientStorage(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <cleanTransientStorageCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::cleanTransientStorage)
                    }
                    cleanTransientStorage
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn EIP712_INPUT_VERIFICATION_TYPE(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <EIP712_INPUT_VERIFICATION_TYPECall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::EIP712_INPUT_VERIFICATION_TYPE)
                    }
                    EIP712_INPUT_VERIFICATION_TYPE
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getHandleVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getHandleVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::getHandleVersion)
                    }
                    getHandleVersion
                },
                {
                    fn isSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <isSignerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::isSigner)
                    }
                    isSigner
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn EIP712_INPUT_VERIFICATION_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <EIP712_INPUT_VERIFICATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::EIP712_INPUT_VERIFICATION_TYPEHASH)
                    }
                    EIP712_INPUT_VERIFICATION_TYPEHASH
                },
                {
                    fn getCoprocessorSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::getCoprocessorSigners)
                    }
                    getCoprocessorSigners
                },
                {
                    fn setThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <setThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::setThreshold)
                    }
                    setThreshold
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn defineNewContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <defineNewContextCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::defineNewContext)
                    }
                    defineNewContext
                },
                {
                    fn verifyInput(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <verifyInputCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::verifyInput)
                    }
                    verifyInput
                },
                {
                    fn getThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <getThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::getThreshold)
                    }
                    getThreshold
                },
                {
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::reinitializeV2)
                    }
                    reinitializeV2
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
                Self::EIP712_INPUT_VERIFICATION_TYPE(inner) => {
                    <EIP712_INPUT_VERIFICATION_TYPECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EIP712_INPUT_VERIFICATION_TYPEHASH(inner) => {
                    <EIP712_INPUT_VERIFICATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cleanTransientStorage(inner) => {
                    <cleanTransientStorageCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::defineNewContext(inner) => {
                    <defineNewContextCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCoprocessorSigners(inner) => {
                    <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHandleVersion(inner) => {
                    <getHandleVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getThreshold(inner) => {
                    <getThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isSigner(inner) => {
                    <isSignerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::setThreshold(inner) => {
                    <setThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyInput(inner) => {
                    <verifyInputCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::EIP712_INPUT_VERIFICATION_TYPE(inner) => {
                    <EIP712_INPUT_VERIFICATION_TYPECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EIP712_INPUT_VERIFICATION_TYPEHASH(inner) => {
                    <EIP712_INPUT_VERIFICATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UPGRADE_INTERFACE_VERSION(inner) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cleanTransientStorage(inner) => {
                    <cleanTransientStorageCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::defineNewContext(inner) => {
                    <defineNewContextCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getCoprocessorSigners(inner) => {
                    <getCoprocessorSignersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHandleVersion(inner) => {
                    <getHandleVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getThreshold(inner) => {
                    <getThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isSigner(inner) => {
                    <isSignerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setThreshold(inner) => {
                    <setThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::verifyInput(inner) => {
                    <verifyInputCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`InputVerifier`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum InputVerifierErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CoprocessorAlreadySigner(CoprocessorAlreadySigner),
        #[allow(missing_docs)]
        CoprocessorSignerNull(CoprocessorSignerNull),
        #[allow(missing_docs)]
        DeserializingInputProofFail(DeserializingInputProofFail),
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
        EmptyInputProof(EmptyInputProof),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidChainId(InvalidChainId),
        #[allow(missing_docs)]
        InvalidHandleVersion(InvalidHandleVersion),
        #[allow(missing_docs)]
        InvalidIndex(InvalidIndex),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidInputHandle(InvalidInputHandle),
        #[allow(missing_docs)]
        InvalidSigner(InvalidSigner),
        #[allow(missing_docs)]
        NotASigner(NotASigner),
        #[allow(missing_docs)]
        NotHostOwner(NotHostOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        SignatureThresholdNotReached(SignatureThresholdNotReached),
        #[allow(missing_docs)]
        SignaturesVerificationFailed(SignaturesVerificationFailed),
        #[allow(missing_docs)]
        SignersSetIsEmpty(SignersSetIsEmpty),
        #[allow(missing_docs)]
        ThresholdIsAboveNumberOfSigners(ThresholdIsAboveNumberOfSigners),
        #[allow(missing_docs)]
        ThresholdIsNull(ThresholdIsNull),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        ZeroSignature(ZeroSignature),
    }
    #[automatically_derived]
    impl InputVerifierErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [2u8, 88u8, 223u8, 136u8],
            [16u8, 26u8, 114u8, 156u8],
            [18u8, 134u8, 233u8, 81u8],
            [24u8, 23u8, 236u8, 215u8],
            [33u8, 191u8, 218u8, 16u8],
            [53u8, 25u8, 78u8, 99u8],
            [75u8, 80u8, 108u8, 205u8],
            [76u8, 156u8, 140u8, 227u8],
            [95u8, 126u8, 27u8, 84u8],
            [99u8, 223u8, 129u8, 113u8],
            [111u8, 79u8, 115u8, 31u8],
            [122u8, 71u8, 201u8, 162u8],
            [153u8, 150u8, 179u8, 21u8],
            [168u8, 248u8, 152u8, 128u8],
            [170u8, 29u8, 73u8, 164u8],
            [174u8, 91u8, 207u8, 146u8],
            [178u8, 72u8, 29u8, 22u8],
            [179u8, 10u8, 50u8, 66u8],
            [179u8, 152u8, 151u8, 159u8],
            [185u8, 95u8, 254u8, 12u8],
            [191u8, 24u8, 175u8, 67u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [218u8, 3u8, 87u8, 247u8],
            [224u8, 124u8, 141u8, 186u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for InputVerifierErrors {
        const NAME: &'static str = "InputVerifierErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CoprocessorAlreadySigner(_) => {
                    <CoprocessorAlreadySigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CoprocessorSignerNull(_) => {
                    <CoprocessorSignerNull as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DeserializingInputProofFail(_) => {
                    <DeserializingInputProofFail as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyInputProof(_) => {
                    <EmptyInputProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidChainId(_) => {
                    <InvalidChainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidHandleVersion(_) => {
                    <InvalidHandleVersion as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidIndex(_) => {
                    <InvalidIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInputHandle(_) => {
                    <InvalidInputHandle as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSigner(_) => {
                    <InvalidSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotASigner(_) => {
                    <NotASigner as alloy_sol_types::SolError>::SELECTOR
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
                Self::SignatureThresholdNotReached(_) => {
                    <SignatureThresholdNotReached as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignaturesVerificationFailed(_) => {
                    <SignaturesVerificationFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignersSetIsEmpty(_) => {
                    <SignersSetIsEmpty as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ThresholdIsAboveNumberOfSigners(_) => {
                    <ThresholdIsAboveNumberOfSigners as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ThresholdIsNull(_) => {
                    <ThresholdIsNull as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroSignature(_) => {
                    <ZeroSignature as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<InputVerifierErrors>] = &[
                {
                    fn InvalidInputHandle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidInputHandle as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidInputHandle)
                    }
                    InvalidInputHandle
                },
                {
                    fn CoprocessorSignerNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <CoprocessorSignerNull as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::CoprocessorSignerNull)
                    }
                    CoprocessorSignerNull
                },
                {
                    fn SignersSetIsEmpty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <SignersSetIsEmpty as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::SignersSetIsEmpty)
                    }
                    SignersSetIsEmpty
                },
                {
                    fn DeserializingInputProofFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <DeserializingInputProofFail as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::DeserializingInputProofFail)
                    }
                    DeserializingInputProofFail
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(InputVerifierErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn ThresholdIsAboveNumberOfSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ThresholdIsAboveNumberOfSigners as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ThresholdIsAboveNumberOfSigners)
                    }
                    ThresholdIsAboveNumberOfSigners
                },
                {
                    fn SignaturesVerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <SignaturesVerificationFailed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::SignaturesVerificationFailed)
                    }
                    SignaturesVerificationFailed
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn InvalidHandleVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidHandleVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidHandleVersion)
                    }
                    InvalidHandleVersion
                },
                {
                    fn InvalidIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidIndex as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(InputVerifierErrors::InvalidIndex)
                    }
                    InvalidIndex
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn ThresholdIsNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ThresholdIsNull as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ThresholdIsNull)
                    }
                    ThresholdIsNull
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn CoprocessorAlreadySigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <CoprocessorAlreadySigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::CoprocessorAlreadySigner)
                    }
                    CoprocessorAlreadySigner
                },
                {
                    fn EmptyInputProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <EmptyInputProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::EmptyInputProof)
                    }
                    EmptyInputProof
                },
                {
                    fn ZeroSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ZeroSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ZeroSignature)
                    }
                    ZeroSignature
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn SignatureThresholdNotReached(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <SignatureThresholdNotReached as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::SignatureThresholdNotReached)
                    }
                    SignatureThresholdNotReached
                },
                {
                    fn InvalidSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidSigner)
                    }
                    InvalidSigner
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(InputVerifierErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn NotASigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotASigner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(InputVerifierErrors::NotASigner)
                    }
                    NotASigner
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierErrors::ECDSAInvalidSignatureLength)
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
            ) -> alloy_sol_types::Result<InputVerifierErrors>] = &[
                {
                    fn InvalidInputHandle(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidInputHandle as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidInputHandle)
                    }
                    InvalidInputHandle
                },
                {
                    fn CoprocessorSignerNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <CoprocessorSignerNull as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::CoprocessorSignerNull)
                    }
                    CoprocessorSignerNull
                },
                {
                    fn SignersSetIsEmpty(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <SignersSetIsEmpty as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::SignersSetIsEmpty)
                    }
                    SignersSetIsEmpty
                },
                {
                    fn DeserializingInputProofFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <DeserializingInputProofFail as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::DeserializingInputProofFail)
                    }
                    DeserializingInputProofFail
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn ThresholdIsAboveNumberOfSigners(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ThresholdIsAboveNumberOfSigners as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ThresholdIsAboveNumberOfSigners)
                    }
                    ThresholdIsAboveNumberOfSigners
                },
                {
                    fn SignaturesVerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <SignaturesVerificationFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::SignaturesVerificationFailed)
                    }
                    SignaturesVerificationFailed
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn InvalidHandleVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidHandleVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidHandleVersion)
                    }
                    InvalidHandleVersion
                },
                {
                    fn InvalidIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidIndex)
                    }
                    InvalidIndex
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn ThresholdIsNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ThresholdIsNull as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ThresholdIsNull)
                    }
                    ThresholdIsNull
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn CoprocessorAlreadySigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <CoprocessorAlreadySigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::CoprocessorAlreadySigner)
                    }
                    CoprocessorAlreadySigner
                },
                {
                    fn EmptyInputProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <EmptyInputProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::EmptyInputProof)
                    }
                    EmptyInputProof
                },
                {
                    fn ZeroSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ZeroSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ZeroSignature)
                    }
                    ZeroSignature
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn SignatureThresholdNotReached(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <SignatureThresholdNotReached as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::SignatureThresholdNotReached)
                    }
                    SignatureThresholdNotReached
                },
                {
                    fn InvalidSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidSigner)
                    }
                    InvalidSigner
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn NotASigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <NotASigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::NotASigner)
                    }
                    NotASigner
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierErrors::ECDSAInvalidSignatureLength)
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
                Self::CoprocessorAlreadySigner(inner) => {
                    <CoprocessorAlreadySigner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CoprocessorSignerNull(inner) => {
                    <CoprocessorSignerNull as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DeserializingInputProofFail(inner) => {
                    <DeserializingInputProofFail as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyInputProof(inner) => {
                    <EmptyInputProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidHandleVersion(inner) => {
                    <InvalidHandleVersion as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidIndex(inner) => {
                    <InvalidIndex as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInputHandle(inner) => {
                    <InvalidInputHandle as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSigner(inner) => {
                    <InvalidSigner as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotASigner(inner) => {
                    <NotASigner as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::SignatureThresholdNotReached(inner) => {
                    <SignatureThresholdNotReached as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SignaturesVerificationFailed(inner) => {
                    <SignaturesVerificationFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SignersSetIsEmpty(inner) => {
                    <SignersSetIsEmpty as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ThresholdIsAboveNumberOfSigners(inner) => {
                    <ThresholdIsAboveNumberOfSigners as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ThresholdIsNull(inner) => {
                    <ThresholdIsNull as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ZeroSignature(inner) => {
                    <ZeroSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::CoprocessorAlreadySigner(inner) => {
                    <CoprocessorAlreadySigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CoprocessorSignerNull(inner) => {
                    <CoprocessorSignerNull as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DeserializingInputProofFail(inner) => {
                    <DeserializingInputProofFail as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyInputProof(inner) => {
                    <EmptyInputProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidHandleVersion(inner) => {
                    <InvalidHandleVersion as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidIndex(inner) => {
                    <InvalidIndex as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidInputHandle(inner) => {
                    <InvalidInputHandle as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSigner(inner) => {
                    <InvalidSigner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotASigner(inner) => {
                    <NotASigner as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::SignatureThresholdNotReached(inner) => {
                    <SignatureThresholdNotReached as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SignaturesVerificationFailed(inner) => {
                    <SignaturesVerificationFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SignersSetIsEmpty(inner) => {
                    <SignersSetIsEmpty as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ThresholdIsAboveNumberOfSigners(inner) => {
                    <ThresholdIsAboveNumberOfSigners as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ThresholdIsNull(inner) => {
                    <ThresholdIsNull as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ZeroSignature(inner) => {
                    <ZeroSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`InputVerifier`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum InputVerifierEvents {
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NewContextSet(NewContextSet),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl InputVerifierEvents {
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
                29u8, 205u8, 126u8, 29u8, 233u8, 22u8, 173u8, 59u8, 224u8, 193u8, 9u8,
                121u8, 104u8, 2u8, 152u8, 153u8, 226u8, 231u8, 208u8, 25u8, 92u8, 250u8,
                105u8, 103u8, 225u8, 101u8, 32u8, 192u8, 232u8, 208u8, 124u8, 234u8,
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
    impl alloy_sol_types::SolEventInterface for InputVerifierEvents {
        const NAME: &'static str = "InputVerifierEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
                Some(<NewContextSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewContextSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewContextSet)
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
    impl alloy_sol_types::private::IntoLogData for InputVerifierEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewContextSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewContextSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`InputVerifier`](self) contract instance.

See the [wrapper's documentation](`InputVerifierInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> InputVerifierInstance<P, N> {
        InputVerifierInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<InputVerifierInstance<P, N>>,
    > {
        InputVerifierInstance::<P, N>::deploy(provider)
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
        InputVerifierInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`InputVerifier`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`InputVerifier`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct InputVerifierInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for InputVerifierInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("InputVerifierInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > InputVerifierInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`InputVerifier`](self) contract instance.

See the [wrapper's documentation](`InputVerifierInstance`) for more details.*/
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
        ) -> alloy_contract::Result<InputVerifierInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> InputVerifierInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> InputVerifierInstance<P, N> {
            InputVerifierInstance {
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
    > InputVerifierInstance<P, N> {
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
        ///Creates a new call builder for the [`EIP712_INPUT_VERIFICATION_TYPE`] function.
        pub fn EIP712_INPUT_VERIFICATION_TYPE(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, EIP712_INPUT_VERIFICATION_TYPECall, N> {
            self.call_builder(&EIP712_INPUT_VERIFICATION_TYPECall)
        }
        ///Creates a new call builder for the [`EIP712_INPUT_VERIFICATION_TYPEHASH`] function.
        pub fn EIP712_INPUT_VERIFICATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            &P,
            EIP712_INPUT_VERIFICATION_TYPEHASHCall,
            N,
        > {
            self.call_builder(&EIP712_INPUT_VERIFICATION_TYPEHASHCall)
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall)
        }
        ///Creates a new call builder for the [`cleanTransientStorage`] function.
        pub fn cleanTransientStorage(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, cleanTransientStorageCall, N> {
            self.call_builder(&cleanTransientStorageCall)
        }
        ///Creates a new call builder for the [`defineNewContext`] function.
        pub fn defineNewContext(
            &self,
            newSignersSet: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            newThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, defineNewContextCall, N> {
            self.call_builder(
                &defineNewContextCall {
                    newSignersSet,
                    newThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`getCoprocessorSigners`] function.
        pub fn getCoprocessorSigners(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCoprocessorSignersCall, N> {
            self.call_builder(&getCoprocessorSignersCall)
        }
        ///Creates a new call builder for the [`getHandleVersion`] function.
        pub fn getHandleVersion(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getHandleVersionCall, N> {
            self.call_builder(&getHandleVersionCall)
        }
        ///Creates a new call builder for the [`getThreshold`] function.
        pub fn getThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getThresholdCall, N> {
            self.call_builder(&getThresholdCall)
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
            verifyingContractSource: alloy::sol_types::private::Address,
            chainIDSource: u64,
            initialSigners: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            initialThreshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(
                &initializeFromEmptyProxyCall {
                    verifyingContractSource,
                    chainIDSource,
                    initialSigners,
                    initialThreshold,
                },
            )
        }
        ///Creates a new call builder for the [`isSigner`] function.
        pub fn isSigner(
            &self,
            account: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isSignerCall, N> {
            self.call_builder(&isSignerCall { account })
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
            newSignersSet: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(
                &reinitializeV2Call {
                    newSignersSet,
                    threshold,
                },
            )
        }
        ///Creates a new call builder for the [`setThreshold`] function.
        pub fn setThreshold(
            &self,
            threshold: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setThresholdCall, N> {
            self.call_builder(&setThresholdCall { threshold })
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
        ///Creates a new call builder for the [`verifyInput`] function.
        pub fn verifyInput(
            &self,
            context: <FHEVMExecutor::ContextUserInputs as alloy::sol_types::SolType>::RustType,
            inputHandle: alloy::sol_types::private::FixedBytes<32>,
            inputProof: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, verifyInputCall, N> {
            self.call_builder(
                &verifyInputCall {
                    context,
                    inputHandle,
                    inputProof,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > InputVerifierInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
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
        ///Creates a new event filter for the [`NewContextSet`] event.
        pub fn NewContextSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewContextSet, N> {
            self.event_filter::<NewContextSet>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
