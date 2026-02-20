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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b60805161426a620001eb5f395f818161193f015281816119940152611c36015261426a5ff3fe6080604052600436106100fd575f3560e01c806384b0196e11610094578063ad3cb1cc11610063578063ad3cb1cc146102eb578063da53c47d14610315578063e6317df51461033d578063e75235b814610379578063e7d9e407146103a3576100fd565b806384b0196e1461023f5780638b2181231461026f5780639164d0ae14610299578063960bfe04146102c3576100fd565b806354130ccd116100d057806354130ccd146101875780635eed7675146101b15780637a297f4b146101d95780637df73e2714610203576100fd565b80630d8e6e2c1461010157806335334c231461012b5780634f1ef2861461014157806352d1902d1461015d575b5f80fd5b34801561010c575f80fd5b506101156103cb565b6040516101229190612e60565b60405180910390f35b348015610136575f80fd5b5061013f610446565b005b61015b60048036038101906101569190613017565b610473565b005b348015610168575f80fd5b50610171610492565b60405161017e9190613089565b60405180910390f35b348015610192575f80fd5b5061019b6104c3565b6040516101a89190612e60565b60405180910390f35b3480156101bc575f80fd5b506101d760048036038101906101d2919061316f565b6104df565b005b3480156101e4575f80fd5b506101ed610714565b6040516101fa919061320e565b60405180910390f35b34801561020e575f80fd5b5061022960048036038101906102249190613227565b610718565b604051610236919061326c565b60405180910390f35b34801561024a575f80fd5b50610253610777565b6040516102669796959493929190613394565b60405180910390f35b34801561027a575f80fd5b506102836108c8565b6040516102909190613089565b60405180910390f35b3480156102a4575f80fd5b506102ad6108eb565b6040516102ba91906134cd565b60405180910390f35b3480156102ce575f80fd5b506102e960048036038101906102e491906134ed565b610984565b005b3480156102f6575f80fd5b506102ff610ac8565b60405161030c9190612e60565b60405180910390f35b348015610320575f80fd5b5061033b600480360381019061033691906135d8565b610b01565b005b348015610348575f80fd5b50610363600480360381019061035e91906136ad565b610fb5565b6040516103709190613089565b60405180910390f35b348015610384575f80fd5b5061038d61172b565b60405161039a9190613719565b60405180910390f35b3480156103ae575f80fd5b506103c960048036038101906103c491906135d8565b611742565b005b60606040518060400160405280600d81526020017f496e70757456657269666965720000000000000000000000000000000000000081525061040c5f611873565b6104166002611873565b61041f5f611873565b6040516020016104329493929190613800565b604051602081830303815290604052905090565b5f5c5f805d6001810160015b8181101561046e57805c5f825d5f815d50600181019050610452565b505050565b61047b61193d565b61048482611a23565b61048e8282611b16565b5050565b5f61049b611c34565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b6040518060a00160405280607f81526020016141eb607f913981565b60016104e9611cbb565b67ffffffffffffffff161461052a576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610535611cdf565b9050805f0160089054906101000a900460ff168061057d57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156105b4576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061066f6040518060400160405280601181526020017f496e707574566572696669636174696f6e0000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152508989611d06565b6106b98585808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505084610b01565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610703919061386d565b60405180910390a150505050505050565b5f90565b5f80610722611d20565b9050805f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f6060805f805f60605f610789611d47565b90505f801b815f01541480156107a457505f801b8160010154145b6107e3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016107da906138d0565b60405180910390fd5b6107eb611d6e565b6107f3611e0c565b8260040160149054906101000a900467ffffffffffffffff16836004015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f801b5f67ffffffffffffffff81111561084d5761084c612ef3565b5b60405190808252806020026020018201604052801561087b5781602001602082028036833780820191505090505b507f0f000000000000000000000000000000000000000000000000000000000000009594939291908367ffffffffffffffff16935097509750975097509750975097505090919293949596565b6040518060a00160405280607f81526020016141eb607f91398051906020012081565b60605f6108f6611d20565b90508060010180548060200260200160405190810160405280929190818152602001828054801561097957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610930575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109e1573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a059190613902565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610a7457336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610a6b919061392d565b60405180910390fd5b610a7d81611eaa565b5f610a86611d20565b90507f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8160010183604051610abc929190613a26565b60405180910390a15050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b5e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b829190613902565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610bf157336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610be8919061392d565b60405180910390fd5b5f825190505f8103610c2f576040517f1286e95100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610c38611d20565b90505f81600101805480602002602001604051908101604052809291908181526020018280548015610cbc57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610c73575b505050505090505f815190505f5b81811015610d94575f845f015f858481518110610cea57610ce9613a54565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555083600101805480610d5557610d54613a81565b5b600190038181905f5260205f20015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905590558080600101915050610cca565b505f5b84811015610f6a575f878281518110610db357610db2613a54565b5b602002602001015190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610e22576040517f101a729c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b845f015f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610ea4576040517fae5bcf9200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001855f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508460010181908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550508080600101915050610d97565b50610f7485611eaa565b7f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8686604051610fa5929190613aae565b60405180910390a1505050505050565b5f805f610fca84875f01518860200151611f3c565b915091505f601069ffffffffffffffffffff5f1b8716901c5f1c9050468167ffffffffffffffff1614611029576040517f7a47c9a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f865f1c90505f60506aff000000000000000000008316901c9050846115ee575f875190505f8103611087576040517fb2481d1600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f885f8151811061109b5761109a613a54565b5b602001015160f81c60f81b60f81c60ff1690505f896001815181106110c3576110c2613a54565b5b602001015160f81c60f81b60f81c60ff16905083821115806110e5575060fe84115b1561111c576040517f63df817100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81604161112a9190613b09565b8360206111379190613b09565b60026111439190613b4a565b61114d9190613b4a565b905080841015611189576040517f1817ecd700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8367ffffffffffffffff8111156111a4576111a3612ef3565b5b6040519080825280602002602001820160405280156111d25781602001602082028036833780820191505090505b5090505f5b8481101561125b575f602082026022018e015190505f60ff16815f1c60ff161461122d576040517f5f7e1b5400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8083838151811061124157611240613a54565b5b6020026020010181815250505080806001019150506111d7565b505f8367ffffffffffffffff81111561127757611276612ef3565b5b6040519080825280602002602001820160405280156112aa57816020015b60608152602001906001900390816112955790505b5090505f5b848110156113fc57604167ffffffffffffffff8111156112d2576112d1612ef3565b5b6040519080825280601f01601f1916602001820160405280156113045781602001600182028036833780820191505090505b5082828151811061131857611317613a54565b5b60200260200101819052505f5b60418110156113ee578e8183604161133d9190613b09565b89602061134a9190613b09565b60026113569190613b4a565b6113609190613b4a565b61136a9190613b4a565b8151811061137b5761137a613a54565b5b602001015160f81c60f81b83838151811061139957611398613a54565b5b602002602001015182815181106113b3576113b2613a54565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053508080600101915050611325565b5080806001019150506112af565b50611405612d7e565b82815f01819052508f5f0151816020019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508f60200151816040019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050468160600181815250505f848f5161149c9190613b7d565b90508067ffffffffffffffff8111156114b8576114b7612ef3565b5b6040519080825280601f01601f1916602001820160405280156114ea5781602001600182028036833780820191505090505b5082608001819052505f5b81811015611578578f818761150a9190613b4a565b8151811061151b5761151a613a54565b5b602001015160f81c60f81b8360800151828151811061153d5761153c613a54565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a90535080806001019150506114f5565b506115838284611f81565b61158c8c611fd2565b83898151811061159f5761159e613a54565b5b60200260200101515f1c8a146115e1576040517f0258df8800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050505050505061171a565b5f875f8151811061160257611601613a54565b5b602001015160f81c60f81b60f81c9050818160ff16111580611624575060fe82115b1561165b576040517f63df817100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805f90505b60208110156116dd5780601f6116779190613b7d565b60086116839190613b09565b8a826020876116929190613b09565b600261169e9190613b4a565b6116a89190613b4a565b815181106116b9576116b8613a54565b5b602001015160f81c60f81b60f81c60ff16901b821791508080600101915050611661565b50838114611717576040517f0258df8800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505b815f1b955050505050509392505050565b5f80611735611d20565b9050806002015491505090565b60035f61174d611cdf565b9050805f0160089054906101000a900460ff168061179557508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156117cc576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061181b8484610b01565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051611865919061386d565b60405180910390a150505050565b60605f600161188184611fe7565b0190505f8167ffffffffffffffff81111561189f5761189e612ef3565b5b6040519080825280601f01601f1916602001820160405280156118d15781602001600182028036833780820191505090505b5090505f82602001820190505b600115611932578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161192757611926613bb0565b5b0494505f85036118de575b819350505050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614806119ea57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166119d1612138565b73ffffffffffffffffffffffffffffffffffffffff1614155b15611a21576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a80573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611aa49190613902565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611b1357336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611b0a919061392d565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611b7e57506040513d601f19601f82011682018060405250810190611b7b9190613bf1565b60015b611bbf57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401611bb6919061392d565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114611c2557806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401611c1c9190613089565b60405180910390fd5b611c2f838361218b565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614611cb9576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f611cc4611cdf565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b611d0e6121fd565b611d1a8484848461223d565b50505050565b5f7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace00905090565b5f7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e3373600905090565b60605f611d79611d47565b9050806002018054611d8a90613c49565b80601f0160208091040260200160405190810160405280929190818152602001828054611db690613c49565b8015611e015780601f10611dd857610100808354040283529160200191611e01565b820191905f5260205f20905b815481529060010190602001808311611de457829003601f168201915b505050505091505090565b60605f611e17611d47565b9050806003018054611e2890613c49565b80601f0160208091040260200160405190810160405280929190818152602001828054611e5490613c49565b8015611e9f5780601f10611e7657610100808354040283529160200191611e9f565b820191905f5260205f20905b815481529060010190602001808311611e8257829003601f168201915b505050505091505090565b5f8103611ee3576040517fa8f8988000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f611eec611d20565b90508060010180549050821115611f2f576040517f35194e6300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8181600201819055505050565b5f805f80848688604051602001611f5593929190613d02565b604051602081830303815290604052805190602001209050805c91508181935093505050935093915050565b5f611f8b836122fd565b9050611f9781836123bc565b611fcd576040517f4b506ccd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050565b6001815d5f5c6001810182815d805f5d505050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612043577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161203957612038613bb0565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612080576d04ee2d6d415b85acef8100000000838161207657612075613bb0565b5b0492506020810190505b662386f26fc1000083106120af57662386f26fc1000083816120a5576120a4613bb0565b5b0492506010810190505b6305f5e10083106120d8576305f5e10083816120ce576120cd613bb0565b5b0492506008810190505b61271083106120fd5761271083816120f3576120f2613bb0565b5b0492506004810190505b60648310612120576064838161211657612115613bb0565b5b0492506002810190505b600a831061212f576001810190505b80915050919050565b5f6121647f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6125d2565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612194826125db565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156121f0576121ea82826126a4565b506121f9565b6121f8612724565b5b5050565b612205612760565b61223b576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6122456121fd565b5f61224e611d47565b905082816004015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550818160040160146101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550848160020190816122ce9190613ed7565b50838160030190816122e09190613ed7565b505f801b815f01819055505f801b81600101819055505050505050565b5f6123b56040518060a00160405280607f81526020016141eb607f913980519060200120835f01516040516020016123359190614057565b60405160208183030381529060405280519060200120846020015185604001518660600151876080015160405160200161236f919061406d565b6040516020818303038152906040528051906020012060405160200161239a96959493929190614083565b6040516020818303038152906040528051906020012061277e565b9050919050565b5f80825190505f81036123fb576040517fb30a324200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61240461172b565b90508082101561244b57816040517fb95ffe0c0000000000000000000000000000000000000000000000000000000081526004016124429190613719565b60405180910390fd5b5f8267ffffffffffffffff81111561246657612465612ef3565b5b6040519080825280602002602001820160405280156124945781602001602082028036833780820191505090505b5090505f805f90505b848110156125b9575f6124ca898984815181106124bd576124bc613a54565b5b6020026020010151612797565b90506124d581610718565b61251657806040517fbf18af4300000000000000000000000000000000000000000000000000000000815260040161250d919061392d565b60405180910390fd5b61251f816127af565b61258b578084848151811061253757612536613a54565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050828061257c906140e2565b93505061258a8160016127b9565b5b8483106125ab5761259c84846127c0565b600196505050505050506125cc565b50808060010191505061249d565b506125c482826127c0565b5f9450505050505b92915050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b0361263657806040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161262d919061392d565b60405180910390fd5b806126627f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6125d2565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516126cd919061406d565b5f60405180830381855af49150503d805f8114612705576040519150601f19603f3d011682016040523d82523d5f602084013e61270a565b606091505b509150915061271a858383612800565b9250505092915050565b5f34111561275e576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f612769611cdf565b5f0160089054906101000a900460ff16905090565b5f61279061278a61288d565b8361289b565b9050919050565b5f806127a384846128db565b90508091505092915050565b5f815c9050919050565b80825d5050565b5f5b818110156127fb576127ee8382815181106127e0576127df613a54565b5b60200260200101515f6127b9565b80806001019150506127c2565b505050565b6060826128155761281082612905565b612885565b5f825114801561283b57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561287d57836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401612874919061392d565b60405180910390fd5b819050612886565b5b9392505050565b5f612896612949565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f806128e986866129f3565b9250925092506128f98282612a48565b82935050505092915050565b5f815111156129175780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80612953611d47565b90507f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61297e612baa565b612986612c20565b8360040160149054906101000a900467ffffffffffffffff16846004015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516020016129d7959493929190614129565b6040516020818303038152906040528051906020012091505090565b5f805f6041845103612a33575f805f602087015192506040870151915060608701515f1a9050612a2588828585612c97565b955095509550505050612a41565b5f600285515f1b9250925092505b9250925092565b5f6003811115612a5b57612a5a61417a565b5b826003811115612a6e57612a6d61417a565b5b0315612ba65760016003811115612a8857612a8761417a565b5b826003811115612a9b57612a9a61417a565b5b03612ad2576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115612ae657612ae561417a565b5b826003811115612af957612af861417a565b5b03612b3d57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401612b349190613719565b60405180910390fd5b600380811115612b5057612b4f61417a565b5b826003811115612b6357612b6261417a565b5b03612ba557806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401612b9c9190613089565b60405180910390fd5b5b5050565b5f80612bb4611d47565b90505f612bbf611d6e565b90505f81511115612bdb57808051906020012092505050612c1d565b5f825f015490505f801b8114612bf657809350505050612c1d565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80612c2a611d47565b90505f612c35611e0c565b90505f81511115612c5157808051906020012092505050612c94565b5f826001015490505f801b8114612c6d57809350505050612c94565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115612cd3575f600385925092509250612d74565b5f6001888888886040515f8152602001604052604051612cf694939291906141a7565b6020604051602081039080840390855afa158015612d16573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603612d67575f60015f801b93509350935050612d74565b805f805f1b935093509350505b9450945094915050565b6040518060a00160405280606081526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f8152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015612e0d578082015181840152602081019050612df2565b5f8484015250505050565b5f601f19601f8301169050919050565b5f612e3282612dd6565b612e3c8185612de0565b9350612e4c818560208601612df0565b612e5581612e18565b840191505092915050565b5f6020820190508181035f830152612e788184612e28565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f612eba82612e91565b9050919050565b612eca81612eb0565b8114612ed4575f80fd5b50565b5f81359050612ee581612ec1565b92915050565b5f80fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b612f2982612e18565b810181811067ffffffffffffffff82111715612f4857612f47612ef3565b5b80604052505050565b5f612f5a612e80565b9050612f668282612f20565b919050565b5f67ffffffffffffffff821115612f8557612f84612ef3565b5b612f8e82612e18565b9050602081019050919050565b828183375f83830152505050565b5f612fbb612fb684612f6b565b612f51565b905082815260208101848484011115612fd757612fd6612eef565b5b612fe2848285612f9b565b509392505050565b5f82601f830112612ffe57612ffd612eeb565b5b813561300e848260208601612fa9565b91505092915050565b5f806040838503121561302d5761302c612e89565b5b5f61303a85828601612ed7565b925050602083013567ffffffffffffffff81111561305b5761305a612e8d565b5b61306785828601612fea565b9150509250929050565b5f819050919050565b61308381613071565b82525050565b5f60208201905061309c5f83018461307a565b92915050565b5f67ffffffffffffffff82169050919050565b6130be816130a2565b81146130c8575f80fd5b50565b5f813590506130d9816130b5565b92915050565b5f80fd5b5f80fd5b5f8083601f8401126130fc576130fb612eeb565b5b8235905067ffffffffffffffff811115613119576131186130df565b5b602083019150836020820283011115613135576131346130e3565b5b9250929050565b5f819050919050565b61314e8161313c565b8114613158575f80fd5b50565b5f8135905061316981613145565b92915050565b5f805f805f6080868803121561318857613187612e89565b5b5f61319588828901612ed7565b95505060206131a6888289016130cb565b945050604086013567ffffffffffffffff8111156131c7576131c6612e8d565b5b6131d3888289016130e7565b935093505060606131e68882890161315b565b9150509295509295909350565b5f60ff82169050919050565b613208816131f3565b82525050565b5f6020820190506132215f8301846131ff565b92915050565b5f6020828403121561323c5761323b612e89565b5b5f61324984828501612ed7565b91505092915050565b5f8115159050919050565b61326681613252565b82525050565b5f60208201905061327f5f83018461325d565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6132b981613285565b82525050565b6132c88161313c565b82525050565b6132d781612eb0565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61330f8161313c565b82525050565b5f6133208383613306565b60208301905092915050565b5f602082019050919050565b5f613342826132dd565b61334c81856132e7565b9350613357836132f7565b805f5b8381101561338757815161336e8882613315565b97506133798361332c565b92505060018101905061335a565b5085935050505092915050565b5f60e0820190506133a75f83018a6132b0565b81810360208301526133b98189612e28565b905081810360408301526133cd8188612e28565b90506133dc60608301876132bf565b6133e960808301866132ce565b6133f660a083018561307a565b81810360c08301526134088184613338565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61344881612eb0565b82525050565b5f613459838361343f565b60208301905092915050565b5f602082019050919050565b5f61347b82613416565b6134858185613420565b935061349083613430565b805f5b838110156134c05781516134a7888261344e565b97506134b283613465565b925050600181019050613493565b5085935050505092915050565b5f6020820190508181035f8301526134e58184613471565b905092915050565b5f6020828403121561350257613501612e89565b5b5f61350f8482850161315b565b91505092915050565b5f67ffffffffffffffff82111561353257613531612ef3565b5b602082029050602081019050919050565b5f61355561355084613518565b612f51565b90508083825260208201905060208402830185811115613578576135776130e3565b5b835b818110156135a1578061358d8882612ed7565b84526020840193505060208101905061357a565b5050509392505050565b5f82601f8301126135bf576135be612eeb565b5b81356135cf848260208601613543565b91505092915050565b5f80604083850312156135ee576135ed612e89565b5b5f83013567ffffffffffffffff81111561360b5761360a612e8d565b5b613617858286016135ab565b92505060206136288582860161315b565b9150509250929050565b5f80fd5b5f6040828403121561364b5761364a613632565b5b6136556040612f51565b90505f61366484828501612ed7565b5f83015250602061367784828501612ed7565b60208301525092915050565b61368c81613071565b8114613696575f80fd5b50565b5f813590506136a781613683565b92915050565b5f805f608084860312156136c4576136c3612e89565b5b5f6136d186828701613636565b93505060406136e286828701613699565b925050606084013567ffffffffffffffff81111561370357613702612e8d565b5b61370f86828701612fea565b9150509250925092565b5f60208201905061372c5f8301846132bf565b92915050565b5f81905092915050565b5f61374682612dd6565b6137508185613732565b9350613760818560208601612df0565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6137a0600283613732565b91506137ab8261376c565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6137ea600183613732565b91506137f5826137b6565b600182019050919050565b5f61380b828761373c565b915061381682613794565b9150613822828661373c565b915061382d826137de565b9150613839828561373c565b9150613844826137de565b9150613850828461373c565b915081905095945050505050565b613867816130a2565b82525050565b5f6020820190506138805f83018461385e565b92915050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6138ba601583612de0565b91506138c582613886565b602082019050919050565b5f6020820190508181035f8301526138e7816138ae565b9050919050565b5f815190506138fc81612ec1565b92915050565b5f6020828403121561391757613916612e89565b5b5f613924848285016138ee565b91505092915050565b5f6020820190506139405f8301846132ce565b92915050565b5f81549050919050565b5f819050815f5260205f209050919050565b5f815f1c9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61399e61399983613962565b61396d565b9050919050565b5f6139b0825461398c565b9050919050565b5f600182019050919050565b5f6139cd82613946565b6139d78185613420565b93506139e283613950565b805f5b83811015613a19576139f6826139a5565b613a00888261344e565b9750613a0b836139b7565b9250506001810190506139e5565b5085935050505092915050565b5f6040820190508181035f830152613a3e81856139c3565b9050613a4d60208301846132bf565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b5f6040820190508181035f830152613ac68185613471565b9050613ad560208301846132bf565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613b138261313c565b9150613b1e8361313c565b9250828202613b2c8161313c565b91508282048414831517613b4357613b42613adc565b5b5092915050565b5f613b548261313c565b9150613b5f8361313c565b9250828201905080821115613b7757613b76613adc565b5b92915050565b5f613b878261313c565b9150613b928361313c565b9250828203905081811115613baa57613ba9613adc565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f81519050613beb81613683565b92915050565b5f60208284031215613c0657613c05612e89565b5b5f613c1384828501613bdd565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680613c6057607f821691505b602082108103613c7357613c72613c1c565b5b50919050565b5f8160601b9050919050565b5f613c8f82613c79565b9050919050565b5f613ca082613c85565b9050919050565b613cb8613cb382612eb0565b613c96565b82525050565b5f81519050919050565b5f81905092915050565b5f613cdc82613cbe565b613ce68185613cc8565b9350613cf6818560208601612df0565b80840191505092915050565b5f613d0d8286613ca7565b601482019150613d1d8285613ca7565b601482019150613d2d8284613cd2565b9150819050949350505050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302613d967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82613d5b565b613da08683613d5b565b95508019841693508086168417925050509392505050565b5f819050919050565b5f613ddb613dd6613dd18461313c565b613db8565b61313c565b9050919050565b5f819050919050565b613df483613dc1565b613e08613e0082613de2565b848454613d67565b825550505050565b5f90565b613e1c613e10565b613e27818484613deb565b505050565b5b81811015613e4a57613e3f5f82613e14565b600181019050613e2d565b5050565b601f821115613e8f57613e6081613d3a565b613e6984613d4c565b81016020851015613e78578190505b613e8c613e8485613d4c565b830182613e2c565b50505b505050565b5f82821c905092915050565b5f613eaf5f1984600802613e94565b1980831691505092915050565b5f613ec78383613ea0565b9150826002028217905092915050565b613ee082612dd6565b67ffffffffffffffff811115613ef957613ef8612ef3565b5b613f038254613c49565b613f0e828285613e4e565b5f60209050601f831160018114613f3f575f8415613f2d578287015190505b613f378582613ebc565b865550613f9e565b601f198416613f4d86613d3a565b5f5b82811015613f7457848901518255600182019150602085019450602081019050613f4f565b86831015613f915784890151613f8d601f891682613ea0565b8355505b6001600288020188555050505b505050505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b613fd281613071565b82525050565b5f613fe38383613fc9565b60208301905092915050565b5f602082019050919050565b5f61400582613fa6565b61400f8185613fb0565b935061401a83613fba565b805f5b8381101561404a5781516140318882613fd8565b975061403c83613fef565b92505060018101905061401d565b5085935050505092915050565b5f6140628284613ffb565b915081905092915050565b5f6140788284613cd2565b915081905092915050565b5f60c0820190506140965f83018961307a565b6140a3602083018861307a565b6140b060408301876132ce565b6140bd60608301866132ce565b6140ca60808301856132bf565b6140d760a083018461307a565b979650505050505050565b5f6140ec8261313c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361411e5761411d613adc565b5b600182019050919050565b5f60a08201905061413c5f83018861307a565b614149602083018761307a565b614156604083018661307a565b614163606083018561385e565b61417060808301846132ce565b9695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f6080820190506141ba5f83018761307a565b6141c760208301866131ff565b6141d4604083018561307a565b6141e1606083018461307a565b9594505050505056fe43697068657274657874566572696669636174696f6e28627974657333325b5d20637448616e646c65732c616464726573732075736572416464726573732c6164647265737320636f6e7472616374416464726573732c75696e7432353620636f6e7472616374436861696e49642c62797465732065787472614461746129
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80QaBjb\0\x01\xEB_9_\x81\x81a\x19?\x01R\x81\x81a\x19\x94\x01Ra\x1C6\x01RaBj_\xF3\xFE`\x80`@R`\x046\x10a\0\xFDW_5`\xE0\x1C\x80c\x84\xB0\x19n\x11a\0\x94W\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x02\xEBW\x80c\xDAS\xC4}\x14a\x03\x15W\x80c\xE61}\xF5\x14a\x03=W\x80c\xE7R5\xB8\x14a\x03yW\x80c\xE7\xD9\xE4\x07\x14a\x03\xA3Wa\0\xFDV[\x80c\x84\xB0\x19n\x14a\x02?W\x80c\x8B!\x81#\x14a\x02oW\x80c\x91d\xD0\xAE\x14a\x02\x99W\x80c\x96\x0B\xFE\x04\x14a\x02\xC3Wa\0\xFDV[\x80cT\x13\x0C\xCD\x11a\0\xD0W\x80cT\x13\x0C\xCD\x14a\x01\x87W\x80c^\xEDvu\x14a\x01\xB1W\x80cz)\x7FK\x14a\x01\xD9W\x80c}\xF7>'\x14a\x02\x03Wa\0\xFDV[\x80c\r\x8En,\x14a\x01\x01W\x80c53L#\x14a\x01+W\x80cO\x1E\xF2\x86\x14a\x01AW\x80cR\xD1\x90-\x14a\x01]W[_\x80\xFD[4\x80\x15a\x01\x0CW_\x80\xFD[Pa\x01\x15a\x03\xCBV[`@Qa\x01\"\x91\x90a.`V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x016W_\x80\xFD[Pa\x01?a\x04FV[\0[a\x01[`\x04\x806\x03\x81\x01\x90a\x01V\x91\x90a0\x17V[a\x04sV[\0[4\x80\x15a\x01hW_\x80\xFD[Pa\x01qa\x04\x92V[`@Qa\x01~\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01\x9Ba\x04\xC3V[`@Qa\x01\xA8\x91\x90a.`V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xBCW_\x80\xFD[Pa\x01\xD7`\x04\x806\x03\x81\x01\x90a\x01\xD2\x91\x90a1oV[a\x04\xDFV[\0[4\x80\x15a\x01\xE4W_\x80\xFD[Pa\x01\xEDa\x07\x14V[`@Qa\x01\xFA\x91\x90a2\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x0EW_\x80\xFD[Pa\x02)`\x04\x806\x03\x81\x01\x90a\x02$\x91\x90a2'V[a\x07\x18V[`@Qa\x026\x91\x90a2lV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02JW_\x80\xFD[Pa\x02Sa\x07wV[`@Qa\x02f\x97\x96\x95\x94\x93\x92\x91\x90a3\x94V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02zW_\x80\xFD[Pa\x02\x83a\x08\xC8V[`@Qa\x02\x90\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA4W_\x80\xFD[Pa\x02\xADa\x08\xEBV[`@Qa\x02\xBA\x91\x90a4\xCDV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCEW_\x80\xFD[Pa\x02\xE9`\x04\x806\x03\x81\x01\x90a\x02\xE4\x91\x90a4\xEDV[a\t\x84V[\0[4\x80\x15a\x02\xF6W_\x80\xFD[Pa\x02\xFFa\n\xC8V[`@Qa\x03\x0C\x91\x90a.`V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03 W_\x80\xFD[Pa\x03;`\x04\x806\x03\x81\x01\x90a\x036\x91\x90a5\xD8V[a\x0B\x01V[\0[4\x80\x15a\x03HW_\x80\xFD[Pa\x03c`\x04\x806\x03\x81\x01\x90a\x03^\x91\x90a6\xADV[a\x0F\xB5V[`@Qa\x03p\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x84W_\x80\xFD[Pa\x03\x8Da\x17+V[`@Qa\x03\x9A\x91\x90a7\x19V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xAEW_\x80\xFD[Pa\x03\xC9`\x04\x806\x03\x81\x01\x90a\x03\xC4\x91\x90a5\xD8V[a\x17BV[\0[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FInputVerifier\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\x0C_a\x18sV[a\x04\x16`\x02a\x18sV[a\x04\x1F_a\x18sV[`@Q` \x01a\x042\x94\x93\x92\x91\x90a8\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\\_\x80]`\x01\x81\x01`\x01[\x81\x81\x10\x15a\x04nW\x80\\_\x82]_\x81]P`\x01\x81\x01\x90Pa\x04RV[PPPV[a\x04{a\x19=V[a\x04\x84\x82a\x1A#V[a\x04\x8E\x82\x82a\x1B\x16V[PPV[_a\x04\x9Ba\x1C4V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01aA\xEB`\x7F\x919\x81V[`\x01a\x04\xE9a\x1C\xBBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05*W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x055a\x1C\xDFV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x05}WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x05\xB4W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x06o`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7FInputVerification\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x89\x89a\x1D\x06V[a\x06\xB9\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x84a\x0B\x01V[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x07\x03\x91\x90a8mV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x90V[_\x80a\x07\"a\x1D V[\x90P\x80_\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_``\x80_\x80_``_a\x07\x89a\x1DGV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x07\xA4WP_\x80\x1B\x81`\x01\x01T\x14[a\x07\xE3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xDA\x90a8\xD0V[`@Q\x80\x91\x03\x90\xFD[a\x07\xEBa\x1DnV[a\x07\xF3a\x1E\x0CV[\x82`\x04\x01`\x14\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x04\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08MWa\x08La.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08{W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93P\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01aA\xEB`\x7F\x919\x80Q\x90` \x01 \x81V[``_a\x08\xF6a\x1D V[\x90P\x80`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tyW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t0W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x05\x91\x90a9\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\ntW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nk\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[a\n}\x81a\x1E\xAAV[_a\n\x86a\x1D V[\x90P\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x81`\x01\x01\x83`@Qa\n\xBC\x92\x91\x90a:&V[`@Q\x80\x91\x03\x90\xA1PPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x82\x91\x90a9\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B\xF1W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xE8\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[_\x82Q\x90P_\x81\x03a\x0C/W`@Q\x7F\x12\x86\xE9Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C8a\x1D V[\x90P_\x81`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0C\xBCW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0CsW[PPPPP\x90P_\x81Q\x90P_[\x81\x81\x10\x15a\r\x94W_\x84_\x01_\x85\x84\x81Q\x81\x10a\x0C\xEAWa\x0C\xE9a:TV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x01\x80T\x80a\rUWa\rTa:\x81V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x90U\x80\x80`\x01\x01\x91PPa\x0C\xCAV[P_[\x84\x81\x10\x15a\x0FjW_\x87\x82\x81Q\x81\x10a\r\xB3Wa\r\xB2a:TV[[` \x02` \x01\x01Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0E\"W`@Q\x7F\x10\x1Ar\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84_\x01_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0E\xA4W`@Q\x7F\xAE[\xCF\x92\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x84`\x01\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP\x80\x80`\x01\x01\x91PPa\r\x97V[Pa\x0Ft\x85a\x1E\xAAV[\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x86\x86`@Qa\x0F\xA5\x92\x91\x90a:\xAEV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80_a\x0F\xCA\x84\x87_\x01Q\x88` \x01Qa\x1F<V[\x91P\x91P_`\x10i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_\x1B\x87\x16\x90\x1C_\x1C\x90PF\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10)W`@Q\x7FzG\xC9\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86_\x1C\x90P_`Pj\xFF\0\0\0\0\0\0\0\0\0\0\x83\x16\x90\x1C\x90P\x84a\x15\xEEW_\x87Q\x90P_\x81\x03a\x10\x87W`@Q\x7F\xB2H\x1D\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88_\x81Q\x81\x10a\x10\x9BWa\x10\x9Aa:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P_\x89`\x01\x81Q\x81\x10a\x10\xC3Wa\x10\xC2a:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P\x83\x82\x11\x15\x80a\x10\xE5WP`\xFE\x84\x11[\x15a\x11\x1CW`@Q\x7Fc\xDF\x81q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`Aa\x11*\x91\x90a;\tV[\x83` a\x117\x91\x90a;\tV[`\x02a\x11C\x91\x90a;JV[a\x11M\x91\x90a;JV[\x90P\x80\x84\x10\x15a\x11\x89W`@Q\x7F\x18\x17\xEC\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xA4Wa\x11\xA3a.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xD2W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x81\x10\x15a\x12[W_` \x82\x02`\"\x01\x8E\x01Q\x90P_`\xFF\x16\x81_\x1C`\xFF\x16\x14a\x12-W`@Q\x7F_~\x1BT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x83\x81Q\x81\x10a\x12AWa\x12@a:TV[[` \x02` \x01\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x11\xD7V[P_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12wWa\x12va.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xAAW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\x95W\x90P[P\x90P_[\x84\x81\x10\x15a\x13\xFCW`Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xD2Wa\x12\xD1a.\xF3V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13\x04W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x82\x82\x81Q\x81\x10a\x13\x18Wa\x13\x17a:TV[[` \x02` \x01\x01\x81\x90RP_[`A\x81\x10\x15a\x13\xEEW\x8E\x81\x83`Aa\x13=\x91\x90a;\tV[\x89` a\x13J\x91\x90a;\tV[`\x02a\x13V\x91\x90a;JV[a\x13`\x91\x90a;JV[a\x13j\x91\x90a;JV[\x81Q\x81\x10a\x13{Wa\x13za:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x13\x99Wa\x13\x98a:TV[[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x13\xB3Wa\x13\xB2a:TV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x80\x80`\x01\x01\x91PPa\x13%V[P\x80\x80`\x01\x01\x91PPa\x12\xAFV[Pa\x14\x05a-~V[\x82\x81_\x01\x81\x90RP\x8F_\x01Q\x81` \x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x8F` \x01Q\x81`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPF\x81``\x01\x81\x81RPP_\x84\x8FQa\x14\x9C\x91\x90a;}V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xB8Wa\x14\xB7a.\xF3V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xEAW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x82`\x80\x01\x81\x90RP_[\x81\x81\x10\x15a\x15xW\x8F\x81\x87a\x15\n\x91\x90a;JV[\x81Q\x81\x10a\x15\x1BWa\x15\x1Aa:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x80\x01Q\x82\x81Q\x81\x10a\x15=Wa\x15<a:TV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x80\x80`\x01\x01\x91PPa\x14\xF5V[Pa\x15\x83\x82\x84a\x1F\x81V[a\x15\x8C\x8Ca\x1F\xD2V[\x83\x89\x81Q\x81\x10a\x15\x9FWa\x15\x9Ea:TV[[` \x02` \x01\x01Q_\x1C\x8A\x14a\x15\xE1W`@Q\x7F\x02X\xDF\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPa\x17\x1AV[_\x87_\x81Q\x81\x10a\x16\x02Wa\x16\x01a:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P\x81\x81`\xFF\x16\x11\x15\x80a\x16$WP`\xFE\x82\x11[\x15a\x16[W`@Q\x7Fc\xDF\x81q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80_\x90P[` \x81\x10\x15a\x16\xDDW\x80`\x1Fa\x16w\x91\x90a;}V[`\x08a\x16\x83\x91\x90a;\tV[\x8A\x82` \x87a\x16\x92\x91\x90a;\tV[`\x02a\x16\x9E\x91\x90a;JV[a\x16\xA8\x91\x90a;JV[\x81Q\x81\x10a\x16\xB9Wa\x16\xB8a:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90\x1B\x82\x17\x91P\x80\x80`\x01\x01\x91PPa\x16aV[P\x83\x81\x14a\x17\x17W`@Q\x7F\x02X\xDF\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP[\x81_\x1B\x95PPPPPP\x93\x92PPPV[_\x80a\x175a\x1D V[\x90P\x80`\x02\x01T\x91PP\x90V[`\x03_a\x17Ma\x1C\xDFV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x17\x95WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x17\xCCW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x18\x1B\x84\x84a\x0B\x01V[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x18e\x91\x90a8mV[`@Q\x80\x91\x03\x90\xA1PPPPV[``_`\x01a\x18\x81\x84a\x1F\xE7V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x9FWa\x18\x9Ea.\xF3V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\xD1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a\x192W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x19'Wa\x19&a;\xB0V[[\x04\x94P_\x85\x03a\x18\xDEW[\x81\x93PPPP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x19\xEAWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\xD1a!8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x1A!W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x80W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA4\x91\x90a9\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1B\x13W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\n\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1B~WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B{\x91\x90a;\xF1V[`\x01[a\x1B\xBFW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xB6\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a\x1C%W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x1C\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xFD[a\x1C/\x83\x83a!\x8BV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1C\xB9W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\x1C\xC4a\x1C\xDFV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a\x1D\x0Ea!\xFDV[a\x1D\x1A\x84\x84\x84\x84a\"=V[PPPPV[_\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\0\x90P\x90V[_\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\0\x90P\x90V[``_a\x1Dya\x1DGV[\x90P\x80`\x02\x01\x80Ta\x1D\x8A\x90a<IV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xB6\x90a<IV[\x80\x15a\x1E\x01W\x80`\x1F\x10a\x1D\xD8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x01V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a\x1E\x17a\x1DGV[\x90P\x80`\x03\x01\x80Ta\x1E(\x90a<IV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1ET\x90a<IV[\x80\x15a\x1E\x9FW\x80`\x1F\x10a\x1EvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x9FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x81\x03a\x1E\xE3W`@Q\x7F\xA8\xF8\x98\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1E\xECa\x1D V[\x90P\x80`\x01\x01\x80T\x90P\x82\x11\x15a\x1F/W`@Q\x7F5\x19Nc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81`\x02\x01\x81\x90UPPPV[_\x80_\x80\x84\x86\x88`@Q` \x01a\x1FU\x93\x92\x91\x90a=\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x80\\\x91P\x81\x81\x93P\x93PPP\x93P\x93\x91PPV[_a\x1F\x8B\x83a\"\xFDV[\x90Pa\x1F\x97\x81\x83a#\xBCV[a\x1F\xCDW`@Q\x7FKPl\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\x01\x81]_\\`\x01\x81\x01\x82\x81]\x80_]PPPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a CWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a 9Wa 8a;\xB0V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a \x80Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a vWa ua;\xB0V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a \xAFWf#\x86\xF2o\xC1\0\0\x83\x81a \xA5Wa \xA4a;\xB0V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a \xD8Wc\x05\xF5\xE1\0\x83\x81a \xCEWa \xCDa;\xB0V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a \xFDWa'\x10\x83\x81a \xF3Wa \xF2a;\xB0V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a! W`d\x83\x81a!\x16Wa!\x15a;\xB0V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a!/W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a!d\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba%\xD2V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a!\x94\x82a%\xDBV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a!\xF0Wa!\xEA\x82\x82a&\xA4V[Pa!\xF9V[a!\xF8a'$V[[PPV[a\"\x05a'`V[a\";W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\"Ea!\xFDV[_a\"Na\x1DGV[\x90P\x82\x81`\x04\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x81`\x04\x01`\x14a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84\x81`\x02\x01\x90\x81a\"\xCE\x91\x90a>\xD7V[P\x83\x81`\x03\x01\x90\x81a\"\xE0\x91\x90a>\xD7V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPPPV[_a#\xB5`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01aA\xEB`\x7F\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a#5\x91\x90a@WV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a#o\x91\x90a@mV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\x9A\x96\x95\x94\x93\x92\x91\x90a@\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a'~V[\x90P\x91\x90PV[_\x80\x82Q\x90P_\x81\x03a#\xFBW`@Q\x7F\xB3\n2B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a$\x04a\x17+V[\x90P\x80\x82\x10\x15a$KW\x81`@Q\x7F\xB9_\xFE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$B\x91\x90a7\x19V[`@Q\x80\x91\x03\x90\xFD[_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$fWa$ea.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$\x94W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x80_\x90P[\x84\x81\x10\x15a%\xB9W_a$\xCA\x89\x89\x84\x81Q\x81\x10a$\xBDWa$\xBCa:TV[[` \x02` \x01\x01Qa'\x97V[\x90Pa$\xD5\x81a\x07\x18V[a%\x16W\x80`@Q\x7F\xBF\x18\xAFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\r\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[a%\x1F\x81a'\xAFV[a%\x8BW\x80\x84\x84\x81Q\x81\x10a%7Wa%6a:TV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82\x80a%|\x90a@\xE2V[\x93PPa%\x8A\x81`\x01a'\xB9V[[\x84\x83\x10a%\xABWa%\x9C\x84\x84a'\xC0V[`\x01\x96PPPPPPPa%\xCCV[P\x80\x80`\x01\x01\x91PPa$\x9DV[Pa%\xC4\x82\x82a'\xC0V[_\x94PPPPP[\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a&6W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&-\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[\x80a&b\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba%\xD2V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa&\xCD\x91\x90a@mV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a'\x05W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'\nV[``\x91P[P\x91P\x91Pa'\x1A\x85\x83\x83a(\0V[\x92PPP\x92\x91PPV[_4\x11\x15a'^W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a'ia\x1C\xDFV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a'\x90a'\x8Aa(\x8DV[\x83a(\x9BV[\x90P\x91\x90PV[_\x80a'\xA3\x84\x84a(\xDBV[\x90P\x80\x91PP\x92\x91PPV[_\x81\\\x90P\x91\x90PV[\x80\x82]PPV[_[\x81\x81\x10\x15a'\xFBWa'\xEE\x83\x82\x81Q\x81\x10a'\xE0Wa'\xDFa:TV[[` \x02` \x01\x01Q_a'\xB9V[\x80\x80`\x01\x01\x91PPa'\xC2V[PPPV[``\x82a(\x15Wa(\x10\x82a)\x05V[a(\x85V[_\x82Q\x14\x80\x15a(;WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a(}W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(t\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa(\x86V[[\x93\x92PPPV[_a(\x96a)IV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_\x80a(\xE9\x86\x86a)\xF3V[\x92P\x92P\x92Pa(\xF9\x82\x82a*HV[\x82\x93PPPP\x92\x91PPV[_\x81Q\x11\x15a)\x17W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a)Sa\x1DGV[\x90P\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa)~a+\xAAV[a)\x86a, V[\x83`\x04\x01`\x14\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`\x04\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q` \x01a)\xD7\x95\x94\x93\x92\x91\x90aA)V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x90V[_\x80_`A\x84Q\x03a*3W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa*%\x88\x82\x85\x85a,\x97V[\x95P\x95P\x95PPPPa*AV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a*[Wa*ZaAzV[[\x82`\x03\x81\x11\x15a*nWa*maAzV[[\x03\x15a+\xA6W`\x01`\x03\x81\x11\x15a*\x88Wa*\x87aAzV[[\x82`\x03\x81\x11\x15a*\x9BWa*\x9AaAzV[[\x03a*\xD2W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a*\xE6Wa*\xE5aAzV[[\x82`\x03\x81\x11\x15a*\xF9Wa*\xF8aAzV[[\x03a+=W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+4\x91\x90a7\x19V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a+PWa+OaAzV[[\x82`\x03\x81\x11\x15a+cWa+baAzV[[\x03a+\xA5W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+\x9C\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x80a+\xB4a\x1DGV[\x90P_a+\xBFa\x1DnV[\x90P_\x81Q\x11\x15a+\xDBW\x80\x80Q\x90` \x01 \x92PPPa,\x1DV[_\x82_\x01T\x90P_\x80\x1B\x81\x14a+\xF6W\x80\x93PPPPa,\x1DV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a,*a\x1DGV[\x90P_a,5a\x1E\x0CV[\x90P_\x81Q\x11\x15a,QW\x80\x80Q\x90` \x01 \x92PPPa,\x94V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a,mW\x80\x93PPPPa,\x94V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a,\xD3W_`\x03\x85\x92P\x92P\x92Pa-tV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa,\xF6\x94\x93\x92\x91\x90aA\xA7V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a-\x16W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a-gW_`\x01_\x80\x1B\x93P\x93P\x93PPa-tV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a.\rW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa-\xF2V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a.2\x82a-\xD6V[a.<\x81\x85a-\xE0V[\x93Pa.L\x81\x85` \x86\x01a-\xF0V[a.U\x81a.\x18V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra.x\x81\x84a.(V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a.\xBA\x82a.\x91V[\x90P\x91\x90PV[a.\xCA\x81a.\xB0V[\x81\x14a.\xD4W_\x80\xFD[PV[_\x815\x90Pa.\xE5\x81a.\xC1V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a/)\x82a.\x18V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/HWa/Ga.\xF3V[[\x80`@RPPPV[_a/Za.\x80V[\x90Pa/f\x82\x82a/ V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/\x85Wa/\x84a.\xF3V[[a/\x8E\x82a.\x18V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a/\xBBa/\xB6\x84a/kV[a/QV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a/\xD7Wa/\xD6a.\xEFV[[a/\xE2\x84\x82\x85a/\x9BV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a/\xFEWa/\xFDa.\xEBV[[\x815a0\x0E\x84\x82` \x86\x01a/\xA9V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a0-Wa0,a.\x89V[[_a0:\x85\x82\x86\x01a.\xD7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0[Wa0Za.\x8DV[[a0g\x85\x82\x86\x01a/\xEAV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a0\x83\x81a0qV[\x82RPPV[_` \x82\x01\x90Pa0\x9C_\x83\x01\x84a0zV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a0\xBE\x81a0\xA2V[\x81\x14a0\xC8W_\x80\xFD[PV[_\x815\x90Pa0\xD9\x81a0\xB5V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a0\xFCWa0\xFBa.\xEBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x19Wa1\x18a0\xDFV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a15Wa14a0\xE3V[[\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a1N\x81a1<V[\x81\x14a1XW_\x80\xFD[PV[_\x815\x90Pa1i\x81a1EV[\x92\x91PPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a1\x88Wa1\x87a.\x89V[[_a1\x95\x88\x82\x89\x01a.\xD7V[\x95PP` a1\xA6\x88\x82\x89\x01a0\xCBV[\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xC7Wa1\xC6a.\x8DV[[a1\xD3\x88\x82\x89\x01a0\xE7V[\x93P\x93PP``a1\xE6\x88\x82\x89\x01a1[V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_`\xFF\x82\x16\x90P\x91\x90PV[a2\x08\x81a1\xF3V[\x82RPPV[_` \x82\x01\x90Pa2!_\x83\x01\x84a1\xFFV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2<Wa2;a.\x89V[[_a2I\x84\x82\x85\x01a.\xD7V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a2f\x81a2RV[\x82RPPV[_` \x82\x01\x90Pa2\x7F_\x83\x01\x84a2]V[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a2\xB9\x81a2\x85V[\x82RPPV[a2\xC8\x81a1<V[\x82RPPV[a2\xD7\x81a.\xB0V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a3\x0F\x81a1<V[\x82RPPV[_a3 \x83\x83a3\x06V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a3B\x82a2\xDDV[a3L\x81\x85a2\xE7V[\x93Pa3W\x83a2\xF7V[\x80_[\x83\x81\x10\x15a3\x87W\x81Qa3n\x88\x82a3\x15V[\x97Pa3y\x83a3,V[\x92PP`\x01\x81\x01\x90Pa3ZV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa3\xA7_\x83\x01\x8Aa2\xB0V[\x81\x81\x03` \x83\x01Ra3\xB9\x81\x89a.(V[\x90P\x81\x81\x03`@\x83\x01Ra3\xCD\x81\x88a.(V[\x90Pa3\xDC``\x83\x01\x87a2\xBFV[a3\xE9`\x80\x83\x01\x86a2\xCEV[a3\xF6`\xA0\x83\x01\x85a0zV[\x81\x81\x03`\xC0\x83\x01Ra4\x08\x81\x84a38V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a4H\x81a.\xB0V[\x82RPPV[_a4Y\x83\x83a4?V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a4{\x82a4\x16V[a4\x85\x81\x85a4 V[\x93Pa4\x90\x83a40V[\x80_[\x83\x81\x10\x15a4\xC0W\x81Qa4\xA7\x88\x82a4NV[\x97Pa4\xB2\x83a4eV[\x92PP`\x01\x81\x01\x90Pa4\x93V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4\xE5\x81\x84a4qV[\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a5\x02Wa5\x01a.\x89V[[_a5\x0F\x84\x82\x85\x01a1[V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a52Wa51a.\xF3V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a5Ua5P\x84a5\x18V[a/QV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a5xWa5wa0\xE3V[[\x83[\x81\x81\x10\x15a5\xA1W\x80a5\x8D\x88\x82a.\xD7V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa5zV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a5\xBFWa5\xBEa.\xEBV[[\x815a5\xCF\x84\x82` \x86\x01a5CV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a5\xEEWa5\xEDa.\x89V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x0BWa6\na.\x8DV[[a6\x17\x85\x82\x86\x01a5\xABV[\x92PP` a6(\x85\x82\x86\x01a1[V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_`@\x82\x84\x03\x12\x15a6KWa6Ja62V[[a6U`@a/QV[\x90P_a6d\x84\x82\x85\x01a.\xD7V[_\x83\x01RP` a6w\x84\x82\x85\x01a.\xD7V[` \x83\x01RP\x92\x91PPV[a6\x8C\x81a0qV[\x81\x14a6\x96W_\x80\xFD[PV[_\x815\x90Pa6\xA7\x81a6\x83V[\x92\x91PPV[_\x80_`\x80\x84\x86\x03\x12\x15a6\xC4Wa6\xC3a.\x89V[[_a6\xD1\x86\x82\x87\x01a66V[\x93PP`@a6\xE2\x86\x82\x87\x01a6\x99V[\x92PP``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x03Wa7\x02a.\x8DV[[a7\x0F\x86\x82\x87\x01a/\xEAV[\x91PP\x92P\x92P\x92V[_` \x82\x01\x90Pa7,_\x83\x01\x84a2\xBFV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a7F\x82a-\xD6V[a7P\x81\x85a72V[\x93Pa7`\x81\x85` \x86\x01a-\xF0V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a7\xA0`\x02\x83a72V[\x91Pa7\xAB\x82a7lV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a7\xEA`\x01\x83a72V[\x91Pa7\xF5\x82a7\xB6V[`\x01\x82\x01\x90P\x91\x90PV[_a8\x0B\x82\x87a7<V[\x91Pa8\x16\x82a7\x94V[\x91Pa8\"\x82\x86a7<V[\x91Pa8-\x82a7\xDEV[\x91Pa89\x82\x85a7<V[\x91Pa8D\x82a7\xDEV[\x91Pa8P\x82\x84a7<V[\x91P\x81\x90P\x95\x94PPPPPV[a8g\x81a0\xA2V[\x82RPPV[_` \x82\x01\x90Pa8\x80_\x83\x01\x84a8^V[\x92\x91PPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\xBA`\x15\x83a-\xE0V[\x91Pa8\xC5\x82a8\x86V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xE7\x81a8\xAEV[\x90P\x91\x90PV[_\x81Q\x90Pa8\xFC\x81a.\xC1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9\x17Wa9\x16a.\x89V[[_a9$\x84\x82\x85\x01a8\xEEV[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa9@_\x83\x01\x84a2\xCEV[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x81_\x1C\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a9\x9Ea9\x99\x83a9bV[a9mV[\x90P\x91\x90PV[_a9\xB0\x82Ta9\x8CV[\x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_a9\xCD\x82a9FV[a9\xD7\x81\x85a4 V[\x93Pa9\xE2\x83a9PV[\x80_[\x83\x81\x10\x15a:\x19Wa9\xF6\x82a9\xA5V[a:\0\x88\x82a4NV[\x97Pa:\x0B\x83a9\xB7V[\x92PP`\x01\x81\x01\x90Pa9\xE5V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra:>\x81\x85a9\xC3V[\x90Pa:M` \x83\x01\x84a2\xBFV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra:\xC6\x81\x85a4qV[\x90Pa:\xD5` \x83\x01\x84a2\xBFV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a;\x13\x82a1<V[\x91Pa;\x1E\x83a1<V[\x92P\x82\x82\x02a;,\x81a1<V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a;CWa;Ba:\xDCV[[P\x92\x91PPV[_a;T\x82a1<V[\x91Pa;_\x83a1<V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a;wWa;va:\xDCV[[\x92\x91PPV[_a;\x87\x82a1<V[\x91Pa;\x92\x83a1<V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a;\xAAWa;\xA9a:\xDCV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x81Q\x90Pa;\xEB\x81a6\x83V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a<\x06Wa<\x05a.\x89V[[_a<\x13\x84\x82\x85\x01a;\xDDV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a<`W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a<sWa<ra<\x1CV[[P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a<\x8F\x82a<yV[\x90P\x91\x90PV[_a<\xA0\x82a<\x85V[\x90P\x91\x90PV[a<\xB8a<\xB3\x82a.\xB0V[a<\x96V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a<\xDC\x82a<\xBEV[a<\xE6\x81\x85a<\xC8V[\x93Pa<\xF6\x81\x85` \x86\x01a-\xF0V[\x80\x84\x01\x91PP\x92\x91PPV[_a=\r\x82\x86a<\xA7V[`\x14\x82\x01\x91Pa=\x1D\x82\x85a<\xA7V[`\x14\x82\x01\x91Pa=-\x82\x84a<\xD2V[\x91P\x81\x90P\x94\x93PPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a=\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a=[V[a=\xA0\x86\x83a=[V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a=\xDBa=\xD6a=\xD1\x84a1<V[a=\xB8V[a1<V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a=\xF4\x83a=\xC1V[a>\x08a>\0\x82a=\xE2V[\x84\x84Ta=gV[\x82UPPPPV[_\x90V[a>\x1Ca>\x10V[a>'\x81\x84\x84a=\xEBV[PPPV[[\x81\x81\x10\x15a>JWa>?_\x82a>\x14V[`\x01\x81\x01\x90Pa>-V[PPV[`\x1F\x82\x11\x15a>\x8FWa>`\x81a=:V[a>i\x84a=LV[\x81\x01` \x85\x10\x15a>xW\x81\x90P[a>\x8Ca>\x84\x85a=LV[\x83\x01\x82a>,V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a>\xAF_\x19\x84`\x08\x02a>\x94V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a>\xC7\x83\x83a>\xA0V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a>\xE0\x82a-\xD6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xF9Wa>\xF8a.\xF3V[[a?\x03\x82Ta<IV[a?\x0E\x82\x82\x85a>NV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a??W_\x84\x15a?-W\x82\x87\x01Q\x90P[a?7\x85\x82a>\xBCV[\x86UPa?\x9EV[`\x1F\x19\x84\x16a?M\x86a=:V[_[\x82\x81\x10\x15a?tW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa?OV[\x86\x83\x10\x15a?\x91W\x84\x89\x01Qa?\x8D`\x1F\x89\x16\x82a>\xA0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a?\xD2\x81a0qV[\x82RPPV[_a?\xE3\x83\x83a?\xC9V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@\x05\x82a?\xA6V[a@\x0F\x81\x85a?\xB0V[\x93Pa@\x1A\x83a?\xBAV[\x80_[\x83\x81\x10\x15a@JW\x81Qa@1\x88\x82a?\xD8V[\x97Pa@<\x83a?\xEFV[\x92PP`\x01\x81\x01\x90Pa@\x1DV[P\x85\x93PPPP\x92\x91PPV[_a@b\x82\x84a?\xFBV[\x91P\x81\x90P\x92\x91PPV[_a@x\x82\x84a<\xD2V[\x91P\x81\x90P\x92\x91PPV[_`\xC0\x82\x01\x90Pa@\x96_\x83\x01\x89a0zV[a@\xA3` \x83\x01\x88a0zV[a@\xB0`@\x83\x01\x87a2\xCEV[a@\xBD``\x83\x01\x86a2\xCEV[a@\xCA`\x80\x83\x01\x85a2\xBFV[a@\xD7`\xA0\x83\x01\x84a0zV[\x97\x96PPPPPPPV[_a@\xEC\x82a1<V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aA\x1EWaA\x1Da:\xDCV[[`\x01\x82\x01\x90P\x91\x90PV[_`\xA0\x82\x01\x90PaA<_\x83\x01\x88a0zV[aAI` \x83\x01\x87a0zV[aAV`@\x83\x01\x86a0zV[aAc``\x83\x01\x85a8^V[aAp`\x80\x83\x01\x84a2\xCEV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\x80\x82\x01\x90PaA\xBA_\x83\x01\x87a0zV[aA\xC7` \x83\x01\x86a1\xFFV[aA\xD4`@\x83\x01\x85a0zV[aA\xE1``\x83\x01\x84a0zV[\x95\x94PPPPPV\xFECiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100fd575f3560e01c806384b0196e11610094578063ad3cb1cc11610063578063ad3cb1cc146102eb578063da53c47d14610315578063e6317df51461033d578063e75235b814610379578063e7d9e407146103a3576100fd565b806384b0196e1461023f5780638b2181231461026f5780639164d0ae14610299578063960bfe04146102c3576100fd565b806354130ccd116100d057806354130ccd146101875780635eed7675146101b15780637a297f4b146101d95780637df73e2714610203576100fd565b80630d8e6e2c1461010157806335334c231461012b5780634f1ef2861461014157806352d1902d1461015d575b5f80fd5b34801561010c575f80fd5b506101156103cb565b6040516101229190612e60565b60405180910390f35b348015610136575f80fd5b5061013f610446565b005b61015b60048036038101906101569190613017565b610473565b005b348015610168575f80fd5b50610171610492565b60405161017e9190613089565b60405180910390f35b348015610192575f80fd5b5061019b6104c3565b6040516101a89190612e60565b60405180910390f35b3480156101bc575f80fd5b506101d760048036038101906101d2919061316f565b6104df565b005b3480156101e4575f80fd5b506101ed610714565b6040516101fa919061320e565b60405180910390f35b34801561020e575f80fd5b5061022960048036038101906102249190613227565b610718565b604051610236919061326c565b60405180910390f35b34801561024a575f80fd5b50610253610777565b6040516102669796959493929190613394565b60405180910390f35b34801561027a575f80fd5b506102836108c8565b6040516102909190613089565b60405180910390f35b3480156102a4575f80fd5b506102ad6108eb565b6040516102ba91906134cd565b60405180910390f35b3480156102ce575f80fd5b506102e960048036038101906102e491906134ed565b610984565b005b3480156102f6575f80fd5b506102ff610ac8565b60405161030c9190612e60565b60405180910390f35b348015610320575f80fd5b5061033b600480360381019061033691906135d8565b610b01565b005b348015610348575f80fd5b50610363600480360381019061035e91906136ad565b610fb5565b6040516103709190613089565b60405180910390f35b348015610384575f80fd5b5061038d61172b565b60405161039a9190613719565b60405180910390f35b3480156103ae575f80fd5b506103c960048036038101906103c491906135d8565b611742565b005b60606040518060400160405280600d81526020017f496e70757456657269666965720000000000000000000000000000000000000081525061040c5f611873565b6104166002611873565b61041f5f611873565b6040516020016104329493929190613800565b604051602081830303815290604052905090565b5f5c5f805d6001810160015b8181101561046e57805c5f825d5f815d50600181019050610452565b505050565b61047b61193d565b61048482611a23565b61048e8282611b16565b5050565b5f61049b611c34565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b6040518060a00160405280607f81526020016141eb607f913981565b60016104e9611cbb565b67ffffffffffffffff161461052a576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610535611cdf565b9050805f0160089054906101000a900460ff168061057d57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156105b4576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061066f6040518060400160405280601181526020017f496e707574566572696669636174696f6e0000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152508989611d06565b6106b98585808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505084610b01565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610703919061386d565b60405180910390a150505050505050565b5f90565b5f80610722611d20565b9050805f015f8473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16915050919050565b5f6060805f805f60605f610789611d47565b90505f801b815f01541480156107a457505f801b8160010154145b6107e3576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016107da906138d0565b60405180910390fd5b6107eb611d6e565b6107f3611e0c565b8260040160149054906101000a900467ffffffffffffffff16836004015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f801b5f67ffffffffffffffff81111561084d5761084c612ef3565b5b60405190808252806020026020018201604052801561087b5781602001602082028036833780820191505090505b507f0f000000000000000000000000000000000000000000000000000000000000009594939291908367ffffffffffffffff16935097509750975097509750975097505090919293949596565b6040518060a00160405280607f81526020016141eb607f91398051906020012081565b60605f6108f6611d20565b90508060010180548060200260200160405190810160405280929190818152602001828054801561097957602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610930575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156109e1573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a059190613902565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610a7457336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610a6b919061392d565b60405180910390fd5b610a7d81611eaa565b5f610a86611d20565b90507f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8160010183604051610abc929190613a26565b60405180910390a15050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b5e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b829190613902565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610bf157336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610be8919061392d565b60405180910390fd5b5f825190505f8103610c2f576040517f1286e95100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610c38611d20565b90505f81600101805480602002602001604051908101604052809291908181526020018280548015610cbc57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311610c73575b505050505090505f815190505f5b81811015610d94575f845f015f858481518110610cea57610ce9613a54565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff02191690831515021790555083600101805480610d5557610d54613a81565b5b600190038181905f5260205f20015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905590558080600101915050610cca565b505f5b84811015610f6a575f878281518110610db357610db2613a54565b5b602002602001015190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603610e22576040517f101a729c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b845f015f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615610ea4576040517fae5bcf9200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6001855f015f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055508460010181908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550508080600101915050610d97565b50610f7485611eaa565b7f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8686604051610fa5929190613aae565b60405180910390a1505050505050565b5f805f610fca84875f01518860200151611f3c565b915091505f601069ffffffffffffffffffff5f1b8716901c5f1c9050468167ffffffffffffffff1614611029576040517f7a47c9a200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f865f1c90505f60506aff000000000000000000008316901c9050846115ee575f875190505f8103611087576040517fb2481d1600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f885f8151811061109b5761109a613a54565b5b602001015160f81c60f81b60f81c60ff1690505f896001815181106110c3576110c2613a54565b5b602001015160f81c60f81b60f81c60ff16905083821115806110e5575060fe84115b1561111c576040517f63df817100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f81604161112a9190613b09565b8360206111379190613b09565b60026111439190613b4a565b61114d9190613b4a565b905080841015611189576040517f1817ecd700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8367ffffffffffffffff8111156111a4576111a3612ef3565b5b6040519080825280602002602001820160405280156111d25781602001602082028036833780820191505090505b5090505f5b8481101561125b575f602082026022018e015190505f60ff16815f1c60ff161461122d576040517f5f7e1b5400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8083838151811061124157611240613a54565b5b6020026020010181815250505080806001019150506111d7565b505f8367ffffffffffffffff81111561127757611276612ef3565b5b6040519080825280602002602001820160405280156112aa57816020015b60608152602001906001900390816112955790505b5090505f5b848110156113fc57604167ffffffffffffffff8111156112d2576112d1612ef3565b5b6040519080825280601f01601f1916602001820160405280156113045781602001600182028036833780820191505090505b5082828151811061131857611317613a54565b5b60200260200101819052505f5b60418110156113ee578e8183604161133d9190613b09565b89602061134a9190613b09565b60026113569190613b4a565b6113609190613b4a565b61136a9190613b4a565b8151811061137b5761137a613a54565b5b602001015160f81c60f81b83838151811061139957611398613a54565b5b602002602001015182815181106113b3576113b2613a54565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a9053508080600101915050611325565b5080806001019150506112af565b50611405612d7e565b82815f01819052508f5f0151816020019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250508f60200151816040019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050468160600181815250505f848f5161149c9190613b7d565b90508067ffffffffffffffff8111156114b8576114b7612ef3565b5b6040519080825280601f01601f1916602001820160405280156114ea5781602001600182028036833780820191505090505b5082608001819052505f5b81811015611578578f818761150a9190613b4a565b8151811061151b5761151a613a54565b5b602001015160f81c60f81b8360800151828151811061153d5761153c613a54565b5b60200101907effffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff191690815f1a90535080806001019150506114f5565b506115838284611f81565b61158c8c611fd2565b83898151811061159f5761159e613a54565b5b60200260200101515f1c8a146115e1576040517f0258df8800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050505050505061171a565b5f875f8151811061160257611601613a54565b5b602001015160f81c60f81b60f81c9050818160ff16111580611624575060fe82115b1561165b576040517f63df817100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f805f90505b60208110156116dd5780601f6116779190613b7d565b60086116839190613b09565b8a826020876116929190613b09565b600261169e9190613b4a565b6116a89190613b4a565b815181106116b9576116b8613a54565b5b602001015160f81c60f81b60f81c60ff16901b821791508080600101915050611661565b50838114611717576040517f0258df8800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505b815f1b955050505050509392505050565b5f80611735611d20565b9050806002015491505090565b60035f61174d611cdf565b9050805f0160089054906101000a900460ff168061179557508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156117cc576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061181b8484610b01565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051611865919061386d565b60405180910390a150505050565b60605f600161188184611fe7565b0190505f8167ffffffffffffffff81111561189f5761189e612ef3565b5b6040519080825280601f01601f1916602001820160405280156118d15781602001600182028036833780820191505090505b5090505f82602001820190505b600115611932578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a858161192757611926613bb0565b5b0494505f85036118de575b819350505050919050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614806119ea57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166119d1612138565b73ffffffffffffffffffffffffffffffffffffffff1614155b15611a21576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a80573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611aa49190613902565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614611b1357336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401611b0a919061392d565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611b7e57506040513d601f19601f82011682018060405250810190611b7b9190613bf1565b60015b611bbf57816040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401611bb6919061392d565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b8114611c2557806040517faa1d49a4000000000000000000000000000000000000000000000000000000008152600401611c1c9190613089565b60405180910390fd5b611c2f838361218b565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff1614611cb9576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f611cc4611cdf565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b611d0e6121fd565b611d1a8484848461223d565b50505050565b5f7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace00905090565b5f7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e3373600905090565b60605f611d79611d47565b9050806002018054611d8a90613c49565b80601f0160208091040260200160405190810160405280929190818152602001828054611db690613c49565b8015611e015780601f10611dd857610100808354040283529160200191611e01565b820191905f5260205f20905b815481529060010190602001808311611de457829003601f168201915b505050505091505090565b60605f611e17611d47565b9050806003018054611e2890613c49565b80601f0160208091040260200160405190810160405280929190818152602001828054611e5490613c49565b8015611e9f5780601f10611e7657610100808354040283529160200191611e9f565b820191905f5260205f20905b815481529060010190602001808311611e8257829003601f168201915b505050505091505090565b5f8103611ee3576040517fa8f8988000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f611eec611d20565b90508060010180549050821115611f2f576040517f35194e6300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8181600201819055505050565b5f805f80848688604051602001611f5593929190613d02565b604051602081830303815290604052805190602001209050805c91508181935093505050935093915050565b5f611f8b836122fd565b9050611f9781836123bc565b611fcd576040517f4b506ccd00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505050565b6001815d5f5c6001810182815d805f5d505050565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310612043577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161203957612038613bb0565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310612080576d04ee2d6d415b85acef8100000000838161207657612075613bb0565b5b0492506020810190505b662386f26fc1000083106120af57662386f26fc1000083816120a5576120a4613bb0565b5b0492506010810190505b6305f5e10083106120d8576305f5e10083816120ce576120cd613bb0565b5b0492506008810190505b61271083106120fd5761271083816120f3576120f2613bb0565b5b0492506004810190505b60648310612120576064838161211657612115613bb0565b5b0492506002810190505b600a831061212f576001810190505b80915050919050565b5f6121647f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6125d2565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b612194826125db565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f815111156121f0576121ea82826126a4565b506121f9565b6121f8612724565b5b5050565b612205612760565b61223b576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6122456121fd565b5f61224e611d47565b905082816004015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550818160040160146101000a81548167ffffffffffffffff021916908367ffffffffffffffff160217905550848160020190816122ce9190613ed7565b50838160030190816122e09190613ed7565b505f801b815f01819055505f801b81600101819055505050505050565b5f6123b56040518060a00160405280607f81526020016141eb607f913980519060200120835f01516040516020016123359190614057565b60405160208183030381529060405280519060200120846020015185604001518660600151876080015160405160200161236f919061406d565b6040516020818303038152906040528051906020012060405160200161239a96959493929190614083565b6040516020818303038152906040528051906020012061277e565b9050919050565b5f80825190505f81036123fb576040517fb30a324200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61240461172b565b90508082101561244b57816040517fb95ffe0c0000000000000000000000000000000000000000000000000000000081526004016124429190613719565b60405180910390fd5b5f8267ffffffffffffffff81111561246657612465612ef3565b5b6040519080825280602002602001820160405280156124945781602001602082028036833780820191505090505b5090505f805f90505b848110156125b9575f6124ca898984815181106124bd576124bc613a54565b5b6020026020010151612797565b90506124d581610718565b61251657806040517fbf18af4300000000000000000000000000000000000000000000000000000000815260040161250d919061392d565b60405180910390fd5b61251f816127af565b61258b578084848151811061253757612536613a54565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff1681525050828061257c906140e2565b93505061258a8160016127b9565b5b8483106125ab5761259c84846127c0565b600196505050505050506125cc565b50808060010191505061249d565b506125c482826127c0565b5f9450505050505b92915050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b0361263657806040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161262d919061392d565b60405180910390fd5b806126627f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b6125d2565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff16846040516126cd919061406d565b5f60405180830381855af49150503d805f8114612705576040519150601f19603f3d011682016040523d82523d5f602084013e61270a565b606091505b509150915061271a858383612800565b9250505092915050565b5f34111561275e576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f612769611cdf565b5f0160089054906101000a900460ff16905090565b5f61279061278a61288d565b8361289b565b9050919050565b5f806127a384846128db565b90508091505092915050565b5f815c9050919050565b80825d5050565b5f5b818110156127fb576127ee8382815181106127e0576127df613a54565b5b60200260200101515f6127b9565b80806001019150506127c2565b505050565b6060826128155761281082612905565b612885565b5f825114801561283b57505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561287d57836040517f9996b315000000000000000000000000000000000000000000000000000000008152600401612874919061392d565b60405180910390fd5b819050612886565b5b9392505050565b5f612896612949565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f806128e986866129f3565b9250925092506128f98282612a48565b82935050505092915050565b5f815111156129175780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80612953611d47565b90507f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61297e612baa565b612986612c20565b8360040160149054906101000a900467ffffffffffffffff16846004015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516020016129d7959493929190614129565b6040516020818303038152906040528051906020012091505090565b5f805f6041845103612a33575f805f602087015192506040870151915060608701515f1a9050612a2588828585612c97565b955095509550505050612a41565b5f600285515f1b9250925092505b9250925092565b5f6003811115612a5b57612a5a61417a565b5b826003811115612a6e57612a6d61417a565b5b0315612ba65760016003811115612a8857612a8761417a565b5b826003811115612a9b57612a9a61417a565b5b03612ad2576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115612ae657612ae561417a565b5b826003811115612af957612af861417a565b5b03612b3d57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401612b349190613719565b60405180910390fd5b600380811115612b5057612b4f61417a565b5b826003811115612b6357612b6261417a565b5b03612ba557806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401612b9c9190613089565b60405180910390fd5b5b5050565b5f80612bb4611d47565b90505f612bbf611d6e565b90505f81511115612bdb57808051906020012092505050612c1d565b5f825f015490505f801b8114612bf657809350505050612c1d565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f80612c2a611d47565b90505f612c35611e0c565b90505f81511115612c5157808051906020012092505050612c94565b5f826001015490505f801b8114612c6d57809350505050612c94565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115612cd3575f600385925092509250612d74565b5f6001888888886040515f8152602001604052604051612cf694939291906141a7565b6020604051602081039080840390855afa158015612d16573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603612d67575f60015f801b93509350935050612d74565b805f805f1b935093509350505b9450945094915050565b6040518060a00160405280606081526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f73ffffffffffffffffffffffffffffffffffffffff1681526020015f8152602001606081525090565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015612e0d578082015181840152602081019050612df2565b5f8484015250505050565b5f601f19601f8301169050919050565b5f612e3282612dd6565b612e3c8185612de0565b9350612e4c818560208601612df0565b612e5581612e18565b840191505092915050565b5f6020820190508181035f830152612e788184612e28565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f612eba82612e91565b9050919050565b612eca81612eb0565b8114612ed4575f80fd5b50565b5f81359050612ee581612ec1565b92915050565b5f80fd5b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b612f2982612e18565b810181811067ffffffffffffffff82111715612f4857612f47612ef3565b5b80604052505050565b5f612f5a612e80565b9050612f668282612f20565b919050565b5f67ffffffffffffffff821115612f8557612f84612ef3565b5b612f8e82612e18565b9050602081019050919050565b828183375f83830152505050565b5f612fbb612fb684612f6b565b612f51565b905082815260208101848484011115612fd757612fd6612eef565b5b612fe2848285612f9b565b509392505050565b5f82601f830112612ffe57612ffd612eeb565b5b813561300e848260208601612fa9565b91505092915050565b5f806040838503121561302d5761302c612e89565b5b5f61303a85828601612ed7565b925050602083013567ffffffffffffffff81111561305b5761305a612e8d565b5b61306785828601612fea565b9150509250929050565b5f819050919050565b61308381613071565b82525050565b5f60208201905061309c5f83018461307a565b92915050565b5f67ffffffffffffffff82169050919050565b6130be816130a2565b81146130c8575f80fd5b50565b5f813590506130d9816130b5565b92915050565b5f80fd5b5f80fd5b5f8083601f8401126130fc576130fb612eeb565b5b8235905067ffffffffffffffff811115613119576131186130df565b5b602083019150836020820283011115613135576131346130e3565b5b9250929050565b5f819050919050565b61314e8161313c565b8114613158575f80fd5b50565b5f8135905061316981613145565b92915050565b5f805f805f6080868803121561318857613187612e89565b5b5f61319588828901612ed7565b95505060206131a6888289016130cb565b945050604086013567ffffffffffffffff8111156131c7576131c6612e8d565b5b6131d3888289016130e7565b935093505060606131e68882890161315b565b9150509295509295909350565b5f60ff82169050919050565b613208816131f3565b82525050565b5f6020820190506132215f8301846131ff565b92915050565b5f6020828403121561323c5761323b612e89565b5b5f61324984828501612ed7565b91505092915050565b5f8115159050919050565b61326681613252565b82525050565b5f60208201905061327f5f83018461325d565b92915050565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b6132b981613285565b82525050565b6132c88161313c565b82525050565b6132d781612eb0565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61330f8161313c565b82525050565b5f6133208383613306565b60208301905092915050565b5f602082019050919050565b5f613342826132dd565b61334c81856132e7565b9350613357836132f7565b805f5b8381101561338757815161336e8882613315565b97506133798361332c565b92505060018101905061335a565b5085935050505092915050565b5f60e0820190506133a75f83018a6132b0565b81810360208301526133b98189612e28565b905081810360408301526133cd8188612e28565b90506133dc60608301876132bf565b6133e960808301866132ce565b6133f660a083018561307a565b81810360c08301526134088184613338565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61344881612eb0565b82525050565b5f613459838361343f565b60208301905092915050565b5f602082019050919050565b5f61347b82613416565b6134858185613420565b935061349083613430565b805f5b838110156134c05781516134a7888261344e565b97506134b283613465565b925050600181019050613493565b5085935050505092915050565b5f6020820190508181035f8301526134e58184613471565b905092915050565b5f6020828403121561350257613501612e89565b5b5f61350f8482850161315b565b91505092915050565b5f67ffffffffffffffff82111561353257613531612ef3565b5b602082029050602081019050919050565b5f61355561355084613518565b612f51565b90508083825260208201905060208402830185811115613578576135776130e3565b5b835b818110156135a1578061358d8882612ed7565b84526020840193505060208101905061357a565b5050509392505050565b5f82601f8301126135bf576135be612eeb565b5b81356135cf848260208601613543565b91505092915050565b5f80604083850312156135ee576135ed612e89565b5b5f83013567ffffffffffffffff81111561360b5761360a612e8d565b5b613617858286016135ab565b92505060206136288582860161315b565b9150509250929050565b5f80fd5b5f6040828403121561364b5761364a613632565b5b6136556040612f51565b90505f61366484828501612ed7565b5f83015250602061367784828501612ed7565b60208301525092915050565b61368c81613071565b8114613696575f80fd5b50565b5f813590506136a781613683565b92915050565b5f805f608084860312156136c4576136c3612e89565b5b5f6136d186828701613636565b93505060406136e286828701613699565b925050606084013567ffffffffffffffff81111561370357613702612e8d565b5b61370f86828701612fea565b9150509250925092565b5f60208201905061372c5f8301846132bf565b92915050565b5f81905092915050565b5f61374682612dd6565b6137508185613732565b9350613760818560208601612df0565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6137a0600283613732565b91506137ab8261376c565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f6137ea600183613732565b91506137f5826137b6565b600182019050919050565b5f61380b828761373c565b915061381682613794565b9150613822828661373c565b915061382d826137de565b9150613839828561373c565b9150613844826137de565b9150613850828461373c565b915081905095945050505050565b613867816130a2565b82525050565b5f6020820190506138805f83018461385e565b92915050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6138ba601583612de0565b91506138c582613886565b602082019050919050565b5f6020820190508181035f8301526138e7816138ae565b9050919050565b5f815190506138fc81612ec1565b92915050565b5f6020828403121561391757613916612e89565b5b5f613924848285016138ee565b91505092915050565b5f6020820190506139405f8301846132ce565b92915050565b5f81549050919050565b5f819050815f5260205f209050919050565b5f815f1c9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61399e61399983613962565b61396d565b9050919050565b5f6139b0825461398c565b9050919050565b5f600182019050919050565b5f6139cd82613946565b6139d78185613420565b93506139e283613950565b805f5b83811015613a19576139f6826139a5565b613a00888261344e565b9750613a0b836139b7565b9250506001810190506139e5565b5085935050505092915050565b5f6040820190508181035f830152613a3e81856139c3565b9050613a4d60208301846132bf565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b5f6040820190508181035f830152613ac68185613471565b9050613ad560208301846132bf565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f613b138261313c565b9150613b1e8361313c565b9250828202613b2c8161313c565b91508282048414831517613b4357613b42613adc565b5b5092915050565b5f613b548261313c565b9150613b5f8361313c565b9250828201905080821115613b7757613b76613adc565b5b92915050565b5f613b878261313c565b9150613b928361313c565b9250828203905081811115613baa57613ba9613adc565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f81519050613beb81613683565b92915050565b5f60208284031215613c0657613c05612e89565b5b5f613c1384828501613bdd565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680613c6057607f821691505b602082108103613c7357613c72613c1c565b5b50919050565b5f8160601b9050919050565b5f613c8f82613c79565b9050919050565b5f613ca082613c85565b9050919050565b613cb8613cb382612eb0565b613c96565b82525050565b5f81519050919050565b5f81905092915050565b5f613cdc82613cbe565b613ce68185613cc8565b9350613cf6818560208601612df0565b80840191505092915050565b5f613d0d8286613ca7565b601482019150613d1d8285613ca7565b601482019150613d2d8284613cd2565b9150819050949350505050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302613d967fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82613d5b565b613da08683613d5b565b95508019841693508086168417925050509392505050565b5f819050919050565b5f613ddb613dd6613dd18461313c565b613db8565b61313c565b9050919050565b5f819050919050565b613df483613dc1565b613e08613e0082613de2565b848454613d67565b825550505050565b5f90565b613e1c613e10565b613e27818484613deb565b505050565b5b81811015613e4a57613e3f5f82613e14565b600181019050613e2d565b5050565b601f821115613e8f57613e6081613d3a565b613e6984613d4c565b81016020851015613e78578190505b613e8c613e8485613d4c565b830182613e2c565b50505b505050565b5f82821c905092915050565b5f613eaf5f1984600802613e94565b1980831691505092915050565b5f613ec78383613ea0565b9150826002028217905092915050565b613ee082612dd6565b67ffffffffffffffff811115613ef957613ef8612ef3565b5b613f038254613c49565b613f0e828285613e4e565b5f60209050601f831160018114613f3f575f8415613f2d578287015190505b613f378582613ebc565b865550613f9e565b601f198416613f4d86613d3a565b5f5b82811015613f7457848901518255600182019150602085019450602081019050613f4f565b86831015613f915784890151613f8d601f891682613ea0565b8355505b6001600288020188555050505b505050505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b613fd281613071565b82525050565b5f613fe38383613fc9565b60208301905092915050565b5f602082019050919050565b5f61400582613fa6565b61400f8185613fb0565b935061401a83613fba565b805f5b8381101561404a5781516140318882613fd8565b975061403c83613fef565b92505060018101905061401d565b5085935050505092915050565b5f6140628284613ffb565b915081905092915050565b5f6140788284613cd2565b915081905092915050565b5f60c0820190506140965f83018961307a565b6140a3602083018861307a565b6140b060408301876132ce565b6140bd60608301866132ce565b6140ca60808301856132bf565b6140d760a083018461307a565b979650505050505050565b5f6140ec8261313c565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361411e5761411d613adc565b5b600182019050919050565b5f60a08201905061413c5f83018861307a565b614149602083018761307a565b614156604083018661307a565b614163606083018561385e565b61417060808301846132ce565b9695505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f6080820190506141ba5f83018761307a565b6141c760208301866131ff565b6141d4604083018561307a565b6141e1606083018461307a565b9594505050505056fe43697068657274657874566572696669636174696f6e28627974657333325b5d20637448616e646c65732c616464726573732075736572416464726573732c6164647265737320636f6e7472616374416464726573732c75696e7432353620636f6e7472616374436861696e49642c62797465732065787472614461746129
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xFDW_5`\xE0\x1C\x80c\x84\xB0\x19n\x11a\0\x94W\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x02\xEBW\x80c\xDAS\xC4}\x14a\x03\x15W\x80c\xE61}\xF5\x14a\x03=W\x80c\xE7R5\xB8\x14a\x03yW\x80c\xE7\xD9\xE4\x07\x14a\x03\xA3Wa\0\xFDV[\x80c\x84\xB0\x19n\x14a\x02?W\x80c\x8B!\x81#\x14a\x02oW\x80c\x91d\xD0\xAE\x14a\x02\x99W\x80c\x96\x0B\xFE\x04\x14a\x02\xC3Wa\0\xFDV[\x80cT\x13\x0C\xCD\x11a\0\xD0W\x80cT\x13\x0C\xCD\x14a\x01\x87W\x80c^\xEDvu\x14a\x01\xB1W\x80cz)\x7FK\x14a\x01\xD9W\x80c}\xF7>'\x14a\x02\x03Wa\0\xFDV[\x80c\r\x8En,\x14a\x01\x01W\x80c53L#\x14a\x01+W\x80cO\x1E\xF2\x86\x14a\x01AW\x80cR\xD1\x90-\x14a\x01]W[_\x80\xFD[4\x80\x15a\x01\x0CW_\x80\xFD[Pa\x01\x15a\x03\xCBV[`@Qa\x01\"\x91\x90a.`V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x016W_\x80\xFD[Pa\x01?a\x04FV[\0[a\x01[`\x04\x806\x03\x81\x01\x90a\x01V\x91\x90a0\x17V[a\x04sV[\0[4\x80\x15a\x01hW_\x80\xFD[Pa\x01qa\x04\x92V[`@Qa\x01~\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01\x9Ba\x04\xC3V[`@Qa\x01\xA8\x91\x90a.`V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xBCW_\x80\xFD[Pa\x01\xD7`\x04\x806\x03\x81\x01\x90a\x01\xD2\x91\x90a1oV[a\x04\xDFV[\0[4\x80\x15a\x01\xE4W_\x80\xFD[Pa\x01\xEDa\x07\x14V[`@Qa\x01\xFA\x91\x90a2\x0EV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x0EW_\x80\xFD[Pa\x02)`\x04\x806\x03\x81\x01\x90a\x02$\x91\x90a2'V[a\x07\x18V[`@Qa\x026\x91\x90a2lV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02JW_\x80\xFD[Pa\x02Sa\x07wV[`@Qa\x02f\x97\x96\x95\x94\x93\x92\x91\x90a3\x94V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02zW_\x80\xFD[Pa\x02\x83a\x08\xC8V[`@Qa\x02\x90\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xA4W_\x80\xFD[Pa\x02\xADa\x08\xEBV[`@Qa\x02\xBA\x91\x90a4\xCDV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xCEW_\x80\xFD[Pa\x02\xE9`\x04\x806\x03\x81\x01\x90a\x02\xE4\x91\x90a4\xEDV[a\t\x84V[\0[4\x80\x15a\x02\xF6W_\x80\xFD[Pa\x02\xFFa\n\xC8V[`@Qa\x03\x0C\x91\x90a.`V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03 W_\x80\xFD[Pa\x03;`\x04\x806\x03\x81\x01\x90a\x036\x91\x90a5\xD8V[a\x0B\x01V[\0[4\x80\x15a\x03HW_\x80\xFD[Pa\x03c`\x04\x806\x03\x81\x01\x90a\x03^\x91\x90a6\xADV[a\x0F\xB5V[`@Qa\x03p\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x84W_\x80\xFD[Pa\x03\x8Da\x17+V[`@Qa\x03\x9A\x91\x90a7\x19V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xAEW_\x80\xFD[Pa\x03\xC9`\x04\x806\x03\x81\x01\x90a\x03\xC4\x91\x90a5\xD8V[a\x17BV[\0[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FInputVerifier\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04\x0C_a\x18sV[a\x04\x16`\x02a\x18sV[a\x04\x1F_a\x18sV[`@Q` \x01a\x042\x94\x93\x92\x91\x90a8\0V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\\_\x80]`\x01\x81\x01`\x01[\x81\x81\x10\x15a\x04nW\x80\\_\x82]_\x81]P`\x01\x81\x01\x90Pa\x04RV[PPPV[a\x04{a\x19=V[a\x04\x84\x82a\x1A#V[a\x04\x8E\x82\x82a\x1B\x16V[PPV[_a\x04\x9Ba\x1C4V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01aA\xEB`\x7F\x919\x81V[`\x01a\x04\xE9a\x1C\xBBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x05*W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x055a\x1C\xDFV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x05}WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x05\xB4W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x06o`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7FInputVerification\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x89\x89a\x1D\x06V[a\x06\xB9\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x84a\x0B\x01V[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x07\x03\x91\x90a8mV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x90V[_\x80a\x07\"a\x1D V[\x90P\x80_\x01_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_``\x80_\x80_``_a\x07\x89a\x1DGV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a\x07\xA4WP_\x80\x1B\x81`\x01\x01T\x14[a\x07\xE3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x07\xDA\x90a8\xD0V[`@Q\x80\x91\x03\x90\xFD[a\x07\xEBa\x1DnV[a\x07\xF3a\x1E\x0CV[\x82`\x04\x01`\x14\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83`\x04\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08MWa\x08La.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08{W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93P\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01aA\xEB`\x7F\x919\x80Q\x90` \x01 \x81V[``_a\x08\xF6a\x1D V[\x90P\x80`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tyW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\t0W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x05\x91\x90a9\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\ntW3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\nk\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[a\n}\x81a\x1E\xAAV[_a\n\x86a\x1D V[\x90P\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x81`\x01\x01\x83`@Qa\n\xBC\x92\x91\x90a:&V[`@Q\x80\x91\x03\x90\xA1PPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x82\x91\x90a9\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0B\xF1W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\xE8\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[_\x82Q\x90P_\x81\x03a\x0C/W`@Q\x7F\x12\x86\xE9Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C8a\x1D V[\x90P_\x81`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0C\xBCW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0CsW[PPPPP\x90P_\x81Q\x90P_[\x81\x81\x10\x15a\r\x94W_\x84_\x01_\x85\x84\x81Q\x81\x10a\x0C\xEAWa\x0C\xE9a:TV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x01\x80T\x80a\rUWa\rTa:\x81V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x90U\x80\x80`\x01\x01\x91PPa\x0C\xCAV[P_[\x84\x81\x10\x15a\x0FjW_\x87\x82\x81Q\x81\x10a\r\xB3Wa\r\xB2a:TV[[` \x02` \x01\x01Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x0E\"W`@Q\x7F\x10\x1Ar\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84_\x01_\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x0E\xA4W`@Q\x7F\xAE[\xCF\x92\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x84`\x01\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPP\x80\x80`\x01\x01\x91PPa\r\x97V[Pa\x0Ft\x85a\x1E\xAAV[\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x86\x86`@Qa\x0F\xA5\x92\x91\x90a:\xAEV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80_a\x0F\xCA\x84\x87_\x01Q\x88` \x01Qa\x1F<V[\x91P\x91P_`\x10i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF_\x1B\x87\x16\x90\x1C_\x1C\x90PF\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10)W`@Q\x7FzG\xC9\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x86_\x1C\x90P_`Pj\xFF\0\0\0\0\0\0\0\0\0\0\x83\x16\x90\x1C\x90P\x84a\x15\xEEW_\x87Q\x90P_\x81\x03a\x10\x87W`@Q\x7F\xB2H\x1D\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88_\x81Q\x81\x10a\x10\x9BWa\x10\x9Aa:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P_\x89`\x01\x81Q\x81\x10a\x10\xC3Wa\x10\xC2a:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P\x83\x82\x11\x15\x80a\x10\xE5WP`\xFE\x84\x11[\x15a\x11\x1CW`@Q\x7Fc\xDF\x81q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`Aa\x11*\x91\x90a;\tV[\x83` a\x117\x91\x90a;\tV[`\x02a\x11C\x91\x90a;JV[a\x11M\x91\x90a;JV[\x90P\x80\x84\x10\x15a\x11\x89W`@Q\x7F\x18\x17\xEC\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xA4Wa\x11\xA3a.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xD2W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x84\x81\x10\x15a\x12[W_` \x82\x02`\"\x01\x8E\x01Q\x90P_`\xFF\x16\x81_\x1C`\xFF\x16\x14a\x12-W`@Q\x7F_~\x1BT\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x83\x81Q\x81\x10a\x12AWa\x12@a:TV[[` \x02` \x01\x01\x81\x81RPPP\x80\x80`\x01\x01\x91PPa\x11\xD7V[P_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12wWa\x12va.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xAAW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\x95W\x90P[P\x90P_[\x84\x81\x10\x15a\x13\xFCW`Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xD2Wa\x12\xD1a.\xF3V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x13\x04W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x82\x82\x81Q\x81\x10a\x13\x18Wa\x13\x17a:TV[[` \x02` \x01\x01\x81\x90RP_[`A\x81\x10\x15a\x13\xEEW\x8E\x81\x83`Aa\x13=\x91\x90a;\tV[\x89` a\x13J\x91\x90a;\tV[`\x02a\x13V\x91\x90a;JV[a\x13`\x91\x90a;JV[a\x13j\x91\x90a;JV[\x81Q\x81\x10a\x13{Wa\x13za:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83\x83\x81Q\x81\x10a\x13\x99Wa\x13\x98a:TV[[` \x02` \x01\x01Q\x82\x81Q\x81\x10a\x13\xB3Wa\x13\xB2a:TV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x80\x80`\x01\x01\x91PPa\x13%V[P\x80\x80`\x01\x01\x91PPa\x12\xAFV[Pa\x14\x05a-~V[\x82\x81_\x01\x81\x90RP\x8F_\x01Q\x81` \x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x8F` \x01Q\x81`@\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPF\x81``\x01\x81\x81RPP_\x84\x8FQa\x14\x9C\x91\x90a;}V[\x90P\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xB8Wa\x14\xB7a.\xF3V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x14\xEAW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x82`\x80\x01\x81\x90RP_[\x81\x81\x10\x15a\x15xW\x8F\x81\x87a\x15\n\x91\x90a;JV[\x81Q\x81\x10a\x15\x1BWa\x15\x1Aa:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x83`\x80\x01Q\x82\x81Q\x81\x10a\x15=Wa\x15<a:TV[[` \x01\x01\x90~\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90\x81_\x1A\x90SP\x80\x80`\x01\x01\x91PPa\x14\xF5V[Pa\x15\x83\x82\x84a\x1F\x81V[a\x15\x8C\x8Ca\x1F\xD2V[\x83\x89\x81Q\x81\x10a\x15\x9FWa\x15\x9Ea:TV[[` \x02` \x01\x01Q_\x1C\x8A\x14a\x15\xE1W`@Q\x7F\x02X\xDF\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPa\x17\x1AV[_\x87_\x81Q\x81\x10a\x16\x02Wa\x16\x01a:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P\x81\x81`\xFF\x16\x11\x15\x80a\x16$WP`\xFE\x82\x11[\x15a\x16[W`@Q\x7Fc\xDF\x81q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80_\x90P[` \x81\x10\x15a\x16\xDDW\x80`\x1Fa\x16w\x91\x90a;}V[`\x08a\x16\x83\x91\x90a;\tV[\x8A\x82` \x87a\x16\x92\x91\x90a;\tV[`\x02a\x16\x9E\x91\x90a;JV[a\x16\xA8\x91\x90a;JV[\x81Q\x81\x10a\x16\xB9Wa\x16\xB8a:TV[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90\x1B\x82\x17\x91P\x80\x80`\x01\x01\x91PPa\x16aV[P\x83\x81\x14a\x17\x17W`@Q\x7F\x02X\xDF\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP[\x81_\x1B\x95PPPPPP\x93\x92PPPV[_\x80a\x175a\x1D V[\x90P\x80`\x02\x01T\x91PP\x90V[`\x03_a\x17Ma\x1C\xDFV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x17\x95WP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x17\xCCW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x18\x1B\x84\x84a\x0B\x01V[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x18e\x91\x90a8mV[`@Q\x80\x91\x03\x90\xA1PPPPV[``_`\x01a\x18\x81\x84a\x1F\xE7V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x18\x9FWa\x18\x9Ea.\xF3V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x18\xD1W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a\x192W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a\x19'Wa\x19&a;\xB0V[[\x04\x94P_\x85\x03a\x18\xDEW[\x81\x93PPPP\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a\x19\xEAWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\xD1a!8V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x1A!W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x80W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xA4\x91\x90a9\x02V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1B\x13W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\n\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1B~WP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B{\x91\x90a;\xF1V[`\x01[a\x1B\xBFW\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\xB6\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a\x1C%W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\x1C\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xFD[a\x1C/\x83\x83a!\x8BV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1C\xB9W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a\x1C\xC4a\x1C\xDFV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a\x1D\x0Ea!\xFDV[a\x1D\x1A\x84\x84\x84\x84a\"=V[PPPPV[_\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\0\x90P\x90V[_\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\0\x90P\x90V[``_a\x1Dya\x1DGV[\x90P\x80`\x02\x01\x80Ta\x1D\x8A\x90a<IV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xB6\x90a<IV[\x80\x15a\x1E\x01W\x80`\x1F\x10a\x1D\xD8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x01V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\xE4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a\x1E\x17a\x1DGV[\x90P\x80`\x03\x01\x80Ta\x1E(\x90a<IV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1ET\x90a<IV[\x80\x15a\x1E\x9FW\x80`\x1F\x10a\x1EvWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x9FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1E\x82W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x81\x03a\x1E\xE3W`@Q\x7F\xA8\xF8\x98\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1E\xECa\x1D V[\x90P\x80`\x01\x01\x80T\x90P\x82\x11\x15a\x1F/W`@Q\x7F5\x19Nc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81`\x02\x01\x81\x90UPPPV[_\x80_\x80\x84\x86\x88`@Q` \x01a\x1FU\x93\x92\x91\x90a=\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x80\\\x91P\x81\x81\x93P\x93PPP\x93P\x93\x91PPV[_a\x1F\x8B\x83a\"\xFDV[\x90Pa\x1F\x97\x81\x83a#\xBCV[a\x1F\xCDW`@Q\x7FKPl\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\x01\x81]_\\`\x01\x81\x01\x82\x81]\x80_]PPPV[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a CWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a 9Wa 8a;\xB0V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a \x80Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a vWa ua;\xB0V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a \xAFWf#\x86\xF2o\xC1\0\0\x83\x81a \xA5Wa \xA4a;\xB0V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a \xD8Wc\x05\xF5\xE1\0\x83\x81a \xCEWa \xCDa;\xB0V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a \xFDWa'\x10\x83\x81a \xF3Wa \xF2a;\xB0V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a! W`d\x83\x81a!\x16Wa!\x15a;\xB0V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a!/W`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[_a!d\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba%\xD2V[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a!\x94\x82a%\xDBV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a!\xF0Wa!\xEA\x82\x82a&\xA4V[Pa!\xF9V[a!\xF8a'$V[[PPV[a\"\x05a'`V[a\";W`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a\"Ea!\xFDV[_a\"Na\x1DGV[\x90P\x82\x81`\x04\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x81`\x04\x01`\x14a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84\x81`\x02\x01\x90\x81a\"\xCE\x91\x90a>\xD7V[P\x83\x81`\x03\x01\x90\x81a\"\xE0\x91\x90a>\xD7V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPPPV[_a#\xB5`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01aA\xEB`\x7F\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a#5\x91\x90a@WV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a#o\x91\x90a@mV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `@Q` \x01a#\x9A\x96\x95\x94\x93\x92\x91\x90a@\x83V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a'~V[\x90P\x91\x90PV[_\x80\x82Q\x90P_\x81\x03a#\xFBW`@Q\x7F\xB3\n2B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a$\x04a\x17+V[\x90P\x80\x82\x10\x15a$KW\x81`@Q\x7F\xB9_\xFE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$B\x91\x90a7\x19V[`@Q\x80\x91\x03\x90\xFD[_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$fWa$ea.\xF3V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$\x94W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x80_\x90P[\x84\x81\x10\x15a%\xB9W_a$\xCA\x89\x89\x84\x81Q\x81\x10a$\xBDWa$\xBCa:TV[[` \x02` \x01\x01Qa'\x97V[\x90Pa$\xD5\x81a\x07\x18V[a%\x16W\x80`@Q\x7F\xBF\x18\xAFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\r\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[a%\x1F\x81a'\xAFV[a%\x8BW\x80\x84\x84\x81Q\x81\x10a%7Wa%6a:TV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82\x80a%|\x90a@\xE2V[\x93PPa%\x8A\x81`\x01a'\xB9V[[\x84\x83\x10a%\xABWa%\x9C\x84\x84a'\xC0V[`\x01\x96PPPPPPPa%\xCCV[P\x80\x80`\x01\x01\x91PPa$\x9DV[Pa%\xC4\x82\x82a'\xC0V[_\x94PPPPP[\x92\x91PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a&6W\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&-\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[\x80a&b\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba%\xD2V[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa&\xCD\x91\x90a@mV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a'\x05W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'\nV[``\x91P[P\x91P\x91Pa'\x1A\x85\x83\x83a(\0V[\x92PPP\x92\x91PPV[_4\x11\x15a'^W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a'ia\x1C\xDFV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_a'\x90a'\x8Aa(\x8DV[\x83a(\x9BV[\x90P\x91\x90PV[_\x80a'\xA3\x84\x84a(\xDBV[\x90P\x80\x91PP\x92\x91PPV[_\x81\\\x90P\x91\x90PV[\x80\x82]PPV[_[\x81\x81\x10\x15a'\xFBWa'\xEE\x83\x82\x81Q\x81\x10a'\xE0Wa'\xDFa:TV[[` \x02` \x01\x01Q_a'\xB9V[\x80\x80`\x01\x01\x91PPa'\xC2V[PPPV[``\x82a(\x15Wa(\x10\x82a)\x05V[a(\x85V[_\x82Q\x14\x80\x15a(;WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a(}W\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(t\x91\x90a9-V[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa(\x86V[[\x93\x92PPPV[_a(\x96a)IV[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_\x80a(\xE9\x86\x86a)\xF3V[\x92P\x92P\x92Pa(\xF9\x82\x82a*HV[\x82\x93PPPP\x92\x91PPV[_\x81Q\x11\x15a)\x17W\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a)Sa\x1DGV[\x90P\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa)~a+\xAAV[a)\x86a, V[\x83`\x04\x01`\x14\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`\x04\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Q` \x01a)\xD7\x95\x94\x93\x92\x91\x90aA)V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x90V[_\x80_`A\x84Q\x03a*3W_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa*%\x88\x82\x85\x85a,\x97V[\x95P\x95P\x95PPPPa*AV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a*[Wa*ZaAzV[[\x82`\x03\x81\x11\x15a*nWa*maAzV[[\x03\x15a+\xA6W`\x01`\x03\x81\x11\x15a*\x88Wa*\x87aAzV[[\x82`\x03\x81\x11\x15a*\x9BWa*\x9AaAzV[[\x03a*\xD2W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a*\xE6Wa*\xE5aAzV[[\x82`\x03\x81\x11\x15a*\xF9Wa*\xF8aAzV[[\x03a+=W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+4\x91\x90a7\x19V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a+PWa+OaAzV[[\x82`\x03\x81\x11\x15a+cWa+baAzV[[\x03a+\xA5W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+\x9C\x91\x90a0\x89V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x80a+\xB4a\x1DGV[\x90P_a+\xBFa\x1DnV[\x90P_\x81Q\x11\x15a+\xDBW\x80\x80Q\x90` \x01 \x92PPPa,\x1DV[_\x82_\x01T\x90P_\x80\x1B\x81\x14a+\xF6W\x80\x93PPPPa,\x1DV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80a,*a\x1DGV[\x90P_a,5a\x1E\x0CV[\x90P_\x81Q\x11\x15a,QW\x80\x80Q\x90` \x01 \x92PPPa,\x94V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14a,mW\x80\x93PPPPa,\x94V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a,\xD3W_`\x03\x85\x92P\x92P\x92Pa-tV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa,\xF6\x94\x93\x92\x91\x90aA\xA7V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a-\x16W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a-gW_`\x01_\x80\x1B\x93P\x93P\x93PPa-tV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x81R` \x01``\x81RP\x90V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15a.\rW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa-\xF2V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_a.2\x82a-\xD6V[a.<\x81\x85a-\xE0V[\x93Pa.L\x81\x85` \x86\x01a-\xF0V[a.U\x81a.\x18V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra.x\x81\x84a.(V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a.\xBA\x82a.\x91V[\x90P\x91\x90PV[a.\xCA\x81a.\xB0V[\x81\x14a.\xD4W_\x80\xFD[PV[_\x815\x90Pa.\xE5\x81a.\xC1V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a/)\x82a.\x18V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a/HWa/Ga.\xF3V[[\x80`@RPPPV[_a/Za.\x80V[\x90Pa/f\x82\x82a/ V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a/\x85Wa/\x84a.\xF3V[[a/\x8E\x82a.\x18V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_a/\xBBa/\xB6\x84a/kV[a/QV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a/\xD7Wa/\xD6a.\xEFV[[a/\xE2\x84\x82\x85a/\x9BV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a/\xFEWa/\xFDa.\xEBV[[\x815a0\x0E\x84\x82` \x86\x01a/\xA9V[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a0-Wa0,a.\x89V[[_a0:\x85\x82\x86\x01a.\xD7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0[Wa0Za.\x8DV[[a0g\x85\x82\x86\x01a/\xEAV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a0\x83\x81a0qV[\x82RPPV[_` \x82\x01\x90Pa0\x9C_\x83\x01\x84a0zV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a0\xBE\x81a0\xA2V[\x81\x14a0\xC8W_\x80\xFD[PV[_\x815\x90Pa0\xD9\x81a0\xB5V[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a0\xFCWa0\xFBa.\xEBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\x19Wa1\x18a0\xDFV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a15Wa14a0\xE3V[[\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[a1N\x81a1<V[\x81\x14a1XW_\x80\xFD[PV[_\x815\x90Pa1i\x81a1EV[\x92\x91PPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a1\x88Wa1\x87a.\x89V[[_a1\x95\x88\x82\x89\x01a.\xD7V[\x95PP` a1\xA6\x88\x82\x89\x01a0\xCBV[\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a1\xC7Wa1\xC6a.\x8DV[[a1\xD3\x88\x82\x89\x01a0\xE7V[\x93P\x93PP``a1\xE6\x88\x82\x89\x01a1[V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_`\xFF\x82\x16\x90P\x91\x90PV[a2\x08\x81a1\xF3V[\x82RPPV[_` \x82\x01\x90Pa2!_\x83\x01\x84a1\xFFV[\x92\x91PPV[_` \x82\x84\x03\x12\x15a2<Wa2;a.\x89V[[_a2I\x84\x82\x85\x01a.\xD7V[\x91PP\x92\x91PPV[_\x81\x15\x15\x90P\x91\x90PV[a2f\x81a2RV[\x82RPPV[_` \x82\x01\x90Pa2\x7F_\x83\x01\x84a2]V[\x92\x91PPV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a2\xB9\x81a2\x85V[\x82RPPV[a2\xC8\x81a1<V[\x82RPPV[a2\xD7\x81a.\xB0V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a3\x0F\x81a1<V[\x82RPPV[_a3 \x83\x83a3\x06V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a3B\x82a2\xDDV[a3L\x81\x85a2\xE7V[\x93Pa3W\x83a2\xF7V[\x80_[\x83\x81\x10\x15a3\x87W\x81Qa3n\x88\x82a3\x15V[\x97Pa3y\x83a3,V[\x92PP`\x01\x81\x01\x90Pa3ZV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa3\xA7_\x83\x01\x8Aa2\xB0V[\x81\x81\x03` \x83\x01Ra3\xB9\x81\x89a.(V[\x90P\x81\x81\x03`@\x83\x01Ra3\xCD\x81\x88a.(V[\x90Pa3\xDC``\x83\x01\x87a2\xBFV[a3\xE9`\x80\x83\x01\x86a2\xCEV[a3\xF6`\xA0\x83\x01\x85a0zV[\x81\x81\x03`\xC0\x83\x01Ra4\x08\x81\x84a38V[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a4H\x81a.\xB0V[\x82RPPV[_a4Y\x83\x83a4?V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a4{\x82a4\x16V[a4\x85\x81\x85a4 V[\x93Pa4\x90\x83a40V[\x80_[\x83\x81\x10\x15a4\xC0W\x81Qa4\xA7\x88\x82a4NV[\x97Pa4\xB2\x83a4eV[\x92PP`\x01\x81\x01\x90Pa4\x93V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra4\xE5\x81\x84a4qV[\x90P\x92\x91PPV[_` \x82\x84\x03\x12\x15a5\x02Wa5\x01a.\x89V[[_a5\x0F\x84\x82\x85\x01a1[V[\x91PP\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a52Wa51a.\xF3V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a5Ua5P\x84a5\x18V[a/QV[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a5xWa5wa0\xE3V[[\x83[\x81\x81\x10\x15a5\xA1W\x80a5\x8D\x88\x82a.\xD7V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa5zV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a5\xBFWa5\xBEa.\xEBV[[\x815a5\xCF\x84\x82` \x86\x01a5CV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a5\xEEWa5\xEDa.\x89V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x0BWa6\na.\x8DV[[a6\x17\x85\x82\x86\x01a5\xABV[\x92PP` a6(\x85\x82\x86\x01a1[V[\x91PP\x92P\x92\x90PV[_\x80\xFD[_`@\x82\x84\x03\x12\x15a6KWa6Ja62V[[a6U`@a/QV[\x90P_a6d\x84\x82\x85\x01a.\xD7V[_\x83\x01RP` a6w\x84\x82\x85\x01a.\xD7V[` \x83\x01RP\x92\x91PPV[a6\x8C\x81a0qV[\x81\x14a6\x96W_\x80\xFD[PV[_\x815\x90Pa6\xA7\x81a6\x83V[\x92\x91PPV[_\x80_`\x80\x84\x86\x03\x12\x15a6\xC4Wa6\xC3a.\x89V[[_a6\xD1\x86\x82\x87\x01a66V[\x93PP`@a6\xE2\x86\x82\x87\x01a6\x99V[\x92PP``\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x03Wa7\x02a.\x8DV[[a7\x0F\x86\x82\x87\x01a/\xEAV[\x91PP\x92P\x92P\x92V[_` \x82\x01\x90Pa7,_\x83\x01\x84a2\xBFV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[_a7F\x82a-\xD6V[a7P\x81\x85a72V[\x93Pa7`\x81\x85` \x86\x01a-\xF0V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a7\xA0`\x02\x83a72V[\x91Pa7\xAB\x82a7lV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a7\xEA`\x01\x83a72V[\x91Pa7\xF5\x82a7\xB6V[`\x01\x82\x01\x90P\x91\x90PV[_a8\x0B\x82\x87a7<V[\x91Pa8\x16\x82a7\x94V[\x91Pa8\"\x82\x86a7<V[\x91Pa8-\x82a7\xDEV[\x91Pa89\x82\x85a7<V[\x91Pa8D\x82a7\xDEV[\x91Pa8P\x82\x84a7<V[\x91P\x81\x90P\x95\x94PPPPPV[a8g\x81a0\xA2V[\x82RPPV[_` \x82\x01\x90Pa8\x80_\x83\x01\x84a8^V[\x92\x91PPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a8\xBA`\x15\x83a-\xE0V[\x91Pa8\xC5\x82a8\x86V[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra8\xE7\x81a8\xAEV[\x90P\x91\x90PV[_\x81Q\x90Pa8\xFC\x81a.\xC1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9\x17Wa9\x16a.\x89V[[_a9$\x84\x82\x85\x01a8\xEEV[\x91PP\x92\x91PPV[_` \x82\x01\x90Pa9@_\x83\x01\x84a2\xCEV[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x81_\x1C\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a9\x9Ea9\x99\x83a9bV[a9mV[\x90P\x91\x90PV[_a9\xB0\x82Ta9\x8CV[\x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_a9\xCD\x82a9FV[a9\xD7\x81\x85a4 V[\x93Pa9\xE2\x83a9PV[\x80_[\x83\x81\x10\x15a:\x19Wa9\xF6\x82a9\xA5V[a:\0\x88\x82a4NV[\x97Pa:\x0B\x83a9\xB7V[\x92PP`\x01\x81\x01\x90Pa9\xE5V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra:>\x81\x85a9\xC3V[\x90Pa:M` \x83\x01\x84a2\xBFV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra:\xC6\x81\x85a4qV[\x90Pa:\xD5` \x83\x01\x84a2\xBFV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_a;\x13\x82a1<V[\x91Pa;\x1E\x83a1<V[\x92P\x82\x82\x02a;,\x81a1<V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17a;CWa;Ba:\xDCV[[P\x92\x91PPV[_a;T\x82a1<V[\x91Pa;_\x83a1<V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a;wWa;va:\xDCV[[\x92\x91PPV[_a;\x87\x82a1<V[\x91Pa;\x92\x83a1<V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15a;\xAAWa;\xA9a:\xDCV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x81Q\x90Pa;\xEB\x81a6\x83V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a<\x06Wa<\x05a.\x89V[[_a<\x13\x84\x82\x85\x01a;\xDDV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80a<`W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a<sWa<ra<\x1CV[[P\x91\x90PV[_\x81``\x1B\x90P\x91\x90PV[_a<\x8F\x82a<yV[\x90P\x91\x90PV[_a<\xA0\x82a<\x85V[\x90P\x91\x90PV[a<\xB8a<\xB3\x82a.\xB0V[a<\x96V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_a<\xDC\x82a<\xBEV[a<\xE6\x81\x85a<\xC8V[\x93Pa<\xF6\x81\x85` \x86\x01a-\xF0V[\x80\x84\x01\x91PP\x92\x91PPV[_a=\r\x82\x86a<\xA7V[`\x14\x82\x01\x91Pa=\x1D\x82\x85a<\xA7V[`\x14\x82\x01\x91Pa=-\x82\x84a<\xD2V[\x91P\x81\x90P\x94\x93PPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02a=\x96\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a=[V[a=\xA0\x86\x83a=[V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_a=\xDBa=\xD6a=\xD1\x84a1<V[a=\xB8V[a1<V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[a=\xF4\x83a=\xC1V[a>\x08a>\0\x82a=\xE2V[\x84\x84Ta=gV[\x82UPPPPV[_\x90V[a>\x1Ca>\x10V[a>'\x81\x84\x84a=\xEBV[PPPV[[\x81\x81\x10\x15a>JWa>?_\x82a>\x14V[`\x01\x81\x01\x90Pa>-V[PPV[`\x1F\x82\x11\x15a>\x8FWa>`\x81a=:V[a>i\x84a=LV[\x81\x01` \x85\x10\x15a>xW\x81\x90P[a>\x8Ca>\x84\x85a=LV[\x83\x01\x82a>,V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_a>\xAF_\x19\x84`\x08\x02a>\x94V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_a>\xC7\x83\x83a>\xA0V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a>\xE0\x82a-\xD6V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xF9Wa>\xF8a.\xF3V[[a?\x03\x82Ta<IV[a?\x0E\x82\x82\x85a>NV[_` \x90P`\x1F\x83\x11`\x01\x81\x14a??W_\x84\x15a?-W\x82\x87\x01Q\x90P[a?7\x85\x82a>\xBCV[\x86UPa?\x9EV[`\x1F\x19\x84\x16a?M\x86a=:V[_[\x82\x81\x10\x15a?tW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa?OV[\x86\x83\x10\x15a?\x91W\x84\x89\x01Qa?\x8D`\x1F\x89\x16\x82a>\xA0V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a?\xD2\x81a0qV[\x82RPPV[_a?\xE3\x83\x83a?\xC9V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@\x05\x82a?\xA6V[a@\x0F\x81\x85a?\xB0V[\x93Pa@\x1A\x83a?\xBAV[\x80_[\x83\x81\x10\x15a@JW\x81Qa@1\x88\x82a?\xD8V[\x97Pa@<\x83a?\xEFV[\x92PP`\x01\x81\x01\x90Pa@\x1DV[P\x85\x93PPPP\x92\x91PPV[_a@b\x82\x84a?\xFBV[\x91P\x81\x90P\x92\x91PPV[_a@x\x82\x84a<\xD2V[\x91P\x81\x90P\x92\x91PPV[_`\xC0\x82\x01\x90Pa@\x96_\x83\x01\x89a0zV[a@\xA3` \x83\x01\x88a0zV[a@\xB0`@\x83\x01\x87a2\xCEV[a@\xBD``\x83\x01\x86a2\xCEV[a@\xCA`\x80\x83\x01\x85a2\xBFV[a@\xD7`\xA0\x83\x01\x84a0zV[\x97\x96PPPPPPPV[_a@\xEC\x82a1<V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aA\x1EWaA\x1Da:\xDCV[[`\x01\x82\x01\x90P\x91\x90PV[_`\xA0\x82\x01\x90PaA<_\x83\x01\x88a0zV[aAI` \x83\x01\x87a0zV[aAV`@\x83\x01\x86a0zV[aAc``\x83\x01\x85a8^V[aAp`\x80\x83\x01\x84a2\xCEV[\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_`\x80\x82\x01\x90PaA\xBA_\x83\x01\x87a0zV[aA\xC7` \x83\x01\x86a1\xFFV[aA\xD4`@\x83\x01\x85a0zV[aA\xE1``\x83\x01\x84a0zV[\x95\x94PPPPPV\xFECiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)",
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
