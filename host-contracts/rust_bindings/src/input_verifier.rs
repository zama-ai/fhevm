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
        __provider: P,
    ) -> FHEVMExecutorInstance<P, N> {
        FHEVMExecutorInstance::<P, N>::new(address, __provider)
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
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > FHEVMExecutorInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`FHEVMExecutor`](self) contract instance.

See the [wrapper's documentation](`FHEVMExecutorInstance`) for more details.*/
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
    function reinitializeV3() external;
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
    "name": "reinitializeV3",
    "inputs": [],
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
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b60805161285a620001005f395f81816110c2015281816110eb01526112ce015261285a5ff3fe6080604052600436106100fa575f3560e01c806384b0196e11610092578063ad3cb1cc11610062578063ad3cb1cc1461026a578063bac22bb81461029a578063da53c47d146102ae578063e6317df5146102cd578063e75235b8146102ec575f80fd5b806384b0196e146101ef5780638b218123146102165780639164d0ae1461022a578063960bfe041461024b575f80fd5b806354130ccd116100cd57806354130ccd146101735780635eed7675146101875780637a297f4b146101a65780637df73e27146101c0575f80fd5b80630d8e6e2c146100fe57806335334c23146101285780634f1ef2861461013e57806352d1902d14610151575b5f80fd5b348015610109575f80fd5b5061011261031f565b60405161011f9190611f86565b60405180910390f35b348015610133575f80fd5b5061013c61038a565b005b61013c61014c36600461205b565b6103b4565b34801561015c575f80fd5b506101656103d3565b60405190815260200161011f565b34801561017e575f80fd5b506101126103ee565b348015610192575f80fd5b5061013c6101a13660046120a7565b61040a565b3480156101b1575f80fd5b506040515f815260200161011f565b3480156101cb575f80fd5b506101df6101da366004612141565b610598565b604051901515815260200161011f565b3480156101fa575f80fd5b506102036105c1565b60405161011f979695949392919061215c565b348015610221575f80fd5b5061016561068f565b348015610235575f80fd5b5061023e6106b2565b60405161011f9190612236565b348015610256575f80fd5b5061013c610265366004612248565b610721565b348015610275575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156102a5575f80fd5b5061013c61083e565b3480156102b9575f80fd5b5061013c6102c836600461225f565b6108ec565b3480156102d8575f80fd5b506101656102e7366004612311565b610c09565b3480156102f7575f80fd5b507f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0254610165565b60606040518060400160405280600d81526020016c24b7383aba2b32b934b334b2b960991b8152506103505f611028565b61035a6003611028565b6103635f611028565b60405160200161037694939291906123b3565b604051602081830303815290604052905090565b5f5c5f805d600190810190805b828110156103af57805c5f825d5f815d508101610397565b505050565b6103bc6110b7565b6103c58261115d565b6103cf8282611207565b5050565b5f6103dc6112c3565b505f805160206127fa83398151915290565b6040518060a00160405280607f815260200161277b607f913981565b5f8051602061283a833981519152546001600160401b03166001600160401b031660011461044b57604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061283a833981519152805460049190600160401b900460ff1680610481575080546001600160401b03808416911610155b1561049f5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b178155604080518082018252601181527024b7383aba2b32b934b334b1b0ba34b7b760791b602080830191909152825180840190935260018352603160f81b9083015261050b91898961130c565b6105488585808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508792506108ec915050565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050505050565b6001600160a01b03165f9081525f8051602061275b833981519152602052604090205460ff1690565b5f60608082808083815f8051602061281a83398151915280549091501580156105ec57506001810154155b6106355760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064015b60405180910390fd5b61063d611326565b6106456113dd565b60049290920154604080515f80825260208201909252600f60f81b9c939b509399506001600160401b03600160a01b83041698506001600160a01b03909116965094509092509050565b6040518060a00160405280607f815260200161277b607f91398051906020012081565b60605f5f8051602061275b8339815191526001810180546040805160208084028201810190925282815293945083018282801561071657602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116106f8575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610771573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107959190612430565b6001600160a01b0316336001600160a01b0316146107c85760405163021bfda160e41b815233600482015260240161062c565b6107d18161141b565b6040515f8051602061275b833981519152907f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea90610832907f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0190859061244b565b60405180910390a15050565b5f8051602061283a833981519152805460049190600160401b900460ff1680610874575080546001600160401b03808416911610155b156108925760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1760ff60401b191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602001610832565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561093c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109609190612430565b6001600160a01b0316336001600160a01b0316146109935760405163021bfda160e41b815233600482015260240161062c565b81515f8190036109b657604051631286e95160e01b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace018054604080516020808402820181019092528281525f8051602061275b833981519152935f939192909190830182828015610a3957602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610a1b575b505083519394505f925050505b81811015610add575f845f015f858481518110610a6557610a656124a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f6101000a81548160ff02191690831515021790555083600101805480610ab557610ab56124ba565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610a46565b505f5b84811015610bbe575f878281518110610afb57610afb6124a6565b602002602001015190505f6001600160a01b0316816001600160a01b031603610b37576040516304069ca760e21b815260040160405180910390fd5b6001600160a01b0381165f9081526020869052604090205460ff1615610b705760405163572de7c960e11b815260040160405180910390fd5b6001600160a01b03165f818152602086815260408220805460ff1916600190811790915587810180548083018255908452919092200180546001600160a01b03191690921790915501610ae0565b50610bc88561141b565b7f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8686604051610bf99291906124ce565b60405180910390a1505050505050565b5f805f610c1e84875f01518860200151611492565b90925090506001600160401b03601086901c16468114610c5157604051633d23e4d160e11b815260040160405180910390fd5b8560ff605082901c1684610fa45786515f819003610c82576040516359240e8b60e11b815260040160405180910390fd5b5f885f81518110610c9557610c956124a6565b602001015160f81c60f81b60f81c60ff1690505f89600181518110610cbc57610cbc6124a6565b016020015160f81c90508382111580610cd5575060fe84115b15610cf3576040516363df817160e01b815260040160405180910390fd5b5f610cff826041612503565b610d0a846020612503565b610d1590600261251a565b610d1f919061251a565b905080841015610d4257604051631817ecd760e01b815260040160405180910390fd5b5f836001600160401b03811115610d5b57610d5b611fac565b604051908082528060200260200182016040528015610d84578160200160208202803683370190505b5090505f5b84811015610de457602081028d016022015160ff811615610dbd576040516317df86d560e21b815260040160405180910390fd5b80838381518110610dd057610dd06124a6565b602090810291909101015250600101610d89565b505f836001600160401b03811115610dfe57610dfe611fac565b604051908082528060200260200182016040528015610e3157816020015b6060815260200190600190039081610e1c5790505b5090505f5b84811015610e9957610e748e610e4d836041612503565b610e58896020612503565b610e6390600261251a565b610e6d919061251a565b60416114d2565b828281518110610e8657610e866124a6565b6020908102919091010152600101610e36565b50610edb6040518060a00160405280606081526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f8152602001606081525090565b82815f01819052508f5f015181602001906001600160a01b031690816001600160a01b0316815250508f6020015181604001906001600160a01b031690816001600160a01b03168152505046816060018181525050610f468e85868a610f41919061252d565b6114d2565b6080820152610f55818361152c565b610f5e8b61155f565b828881518110610f7057610f706124a6565b60200260200101515f1c8914610f9857604051624b1bf160e31b815260040160405180910390fd5b5050505050505061101a565b5f875f81518110610fb757610fb76124a6565b016020015160f81c90508181111580610fd0575060fe82115b15610fee576040516363df817160e01b815260040160405180910390fd5b6020820288016022015183811461101757604051624b1bf160e31b815260040160405180910390fd5b50505b5093505050505b9392505050565b60605f61103483611572565b60010190505f816001600160401b0381111561105257611052611fac565b6040519080825280601f01601f19166020018201604052801561107c576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461108657509392505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061113d57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166111315f805160206127fa833981519152546001600160a01b031690565b6001600160a01b031614155b1561115b5760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156111ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111d19190612430565b6001600160a01b0316336001600160a01b0316146112045760405163021bfda160e41b815233600482015260240161062c565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611261575060408051601f3d908101601f1916820190925261125e91810190612540565b60015b61128957604051634c9c8ce360e01b81526001600160a01b038316600482015260240161062c565b5f805160206127fa83398151915281146112b957604051632a87526960e21b81526004810182905260240161062c565b6103af838361164a565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461115b5760405163703e46dd60e11b815260040160405180910390fd5b61131461169f565b611320848484846116d5565b50505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360280546060915f8051602061281a8339815191529161136490612557565b80601f016020809104026020016040519081016040528092919081815260200182805461139090612557565b80156107165780601f106113b257610100808354040283529160200191610716565b820191905f5260205f20905b8154815290600101906020018083116113be5750939695505050505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360380546060915f8051602061281a8339815191529161136490612557565b805f0361143b57604051630151f13160e71b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace01545f8051602061275b8339815191529082111561148c576040516335194e6360e01b815260040160405180910390fd5b60020155565b5f805f808486886040516020016114ab9392919061258f565b60408051808303601f190181529190528051602090910120805c9890975095505050505050565b6060816001600160401b038111156114ec576114ec611fac565b6040519080825280601f01601f191660200182016040528015611516576020820181803683370190505b50905081836020860101602083015e9392505050565b5f61153683611783565b9050611542818361185a565b6103af57604051634b506ccd60e01b815260040160405180910390fd5b6001815d60015f5c0181815d805f5d5050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106115b05772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106115dc576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc1000083106115fa57662386f26fc10000830492506010015b6305f5e1008310611612576305f5e100830492506008015b612710831061162657612710830492506004015b60648310611638576064830492506002015b600a8310611644576001015b92915050565b611653826119fd565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611697576103af8282611a60565b6103cf611ad2565b5f8051602061283a83398151915254600160401b900460ff1661115b57604051631afcd79f60e31b815260040160405180910390fd5b6116dd61169f565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360480546001600160a01b0384166001600160e01b031990911617600160a01b6001600160401b038416021790555f8051602061281a8339815191527fe910845fd818f61127c84f3586776436a37dead33625056c65162537e3373602611763868261261f565b5060038101611772858261261f565b505f80825560019091015550505050565b5f6116446040518060a00160405280607f815260200161277b607f91398051602091820120845160405191926117b992016126de565b6040516020818303038152906040528051906020012084602001518560400151866060015187608001516040516020016117f39190612713565b60408051601f198184030181528282528051602091820120908301979097528101949094526001600160a01b0392831660608501529116608083015260a082015260c081019190915260e00160405160208183030381529060405280519060200120611af1565b80515f9080820361187e57604051635985192160e11b815260040160405180910390fd5b5f6118a77f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace025490565b9050808210156118cd57604051632e57ff8360e21b81526004810183905260240161062c565b5f826001600160401b038111156118e6576118e6611fac565b60405190808252806020026020018201604052801561190f578160200160208202803683370190505b5090505f805b848110156119e6575f61194189898481518110611934576119346124a6565b6020026020010151611b1d565b905061194c81610598565b6119745760405163bf18af4360e01b81526001600160a01b038216600482015260240161062c565b805c6119bd578084848151811061198d5761198d6124a6565b6001600160a01b0390921660209283029190910190910152826119af8161272e565b9350506119bd816001611b31565b8483106119dd576119ce8484611b38565b60019650505050505050611644565b50600101611915565b506119f18282611b38565b505f9695505050505050565b806001600160a01b03163b5f03611a3257604051634c9c8ce360e01b81526001600160a01b038216600482015260240161062c565b5f805160206127fa83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051611a7c9190612713565b5f60405180830381855af49150503d805f8114611ab4576040519150601f19603f3d011682016040523d82523d5f602084013e611ab9565b606091505b5091509150611ac9858383611b6d565b95945050505050565b341561115b5760405163b398979f60e01b815260040160405180910390fd5b5f611644611afd611bc9565b8360405161190160f01b8152600281019290925260228201526042902090565b5f80611b298484611bd7565b949350505050565b80825d5050565b5f5b818110156103af57611b65838281518110611b5757611b576124a6565b60200260200101515f611b31565b600101611b3a565b606082611b8257611b7d82611bff565b611021565b8151158015611b9957506001600160a01b0384163b155b15611bc257604051639996b31560e01b81526001600160a01b038516600482015260240161062c565b5080611021565b5f611bd2611c28565b905090565b5f805f80611be58686611cc6565b925092509250611bf58282611d0f565b5090949350505050565b805115611c0f5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5f8051602061281a8339815191527f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611c60611dc7565b611c68611e2f565b60048401546040805160208101959095528401929092526060830152600160a01b81046001600160401b031660808301526001600160a01b031660a082015260c0016040516020818303038152906040528051906020012091505090565b5f805f8351604103611cfd576020840151604085015160608601515f1a611cef88828585611e71565b955095509550505050611d08565b505081515f91506002905b9250925092565b5f826003811115611d2257611d22612746565b03611d2b575050565b6001826003811115611d3f57611d3f612746565b03611d5d5760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115611d7157611d71612746565b03611d925760405163fce698f760e01b81526004810182905260240161062c565b6003826003811115611da657611da6612746565b036103cf576040516335e2f38360e21b81526004810182905260240161062c565b5f5f8051602061281a83398151915281611ddf611326565b805190915015611df757805160209091012092915050565b81548015611e06579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f5f8051602061281a83398151915281611e476113dd565b805190915015611e5f57805160209091012092915050565b60018201548015611e06579392505050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115611eaa57505f91506003905082611f2f565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015611efb573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116611f2657505f925060019150829050611f2f565b92505f91508190505b9450945094915050565b5f5b83811015611f53578181015183820152602001611f3b565b50505f910152565b5f8151808452611f72816020860160208601611f39565b601f01601f19169290920160200192915050565b602081525f6110216020830184611f5b565b6001600160a01b0381168114611204575f80fd5b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b0381118282101715611fe857611fe8611fac565b604052919050565b5f82601f830112611fff575f80fd5b81356001600160401b0381111561201857612018611fac565b61202b601f8201601f1916602001611fc0565b81815284602083860101111561203f575f80fd5b816020850160208301375f918101602001919091529392505050565b5f806040838503121561206c575f80fd5b823561207781611f98565b915060208301356001600160401b03811115612091575f80fd5b61209d85828601611ff0565b9150509250929050565b5f805f805f608086880312156120bb575f80fd5b85356120c681611f98565b945060208601356001600160401b0380821682146120e2575f80fd5b909450604087013590808211156120f7575f80fd5b818801915088601f83011261210a575f80fd5b813581811115612118575f80fd5b8960208260051b850101111561212c575f80fd5b96999598505060200195606001359392505050565b5f60208284031215612151575f80fd5b813561102181611f98565b60ff60f81b881681525f602060e0602084015261217c60e084018a611f5b565b838103604085015261218e818a611f5b565b606085018990526001600160a01b038816608086015260a0850187905284810360c0860152855180825260208088019350909101905f5b818110156121e1578351835292840192918401916001016121c5565b50909c9b505050505050505050505050565b5f815180845260208085019450602084015f5b8381101561222b5781516001600160a01b031687529582019590820190600101612206565b509495945050505050565b602081525f61102160208301846121f3565b5f60208284031215612258575f80fd5b5035919050565b5f8060408385031215612270575f80fd5b82356001600160401b0380821115612286575f80fd5b818501915085601f830112612299575f80fd5b81356020828211156122ad576122ad611fac565b8160051b92506122be818401611fc0565b82815292840181019281810190898511156122d7575f80fd5b948201945b8486101561230157853593506122f184611f98565b83825294820194908201906122dc565b9997909101359750505050505050565b5f805f8385036080811215612324575f80fd5b6040811215612331575f80fd5b50604051604081016001600160401b03828210818311171561235557612355611fac565b816040528635915061236682611f98565b90825260208601359061237882611f98565b81602084015282955060408701359450606087013592508083111561239b575f80fd5b50506123a986828701611ff0565b9150509250925092565b5f85516123c4818460208a01611f39565b61103b60f11b90830190815285516123e3816002840160208a01611f39565b808201915050601760f91b8060028301528551612407816003850160208a01611f39565b60039201918201528351612422816004840160208801611f39565b016004019695505050505050565b5f60208284031215612440575f80fd5b815161102181611f98565b5f6040820160408352808554808352606085019150865f526020925060205f205f5b828110156124925781546001600160a01b03168452928401926001918201910161246d565b505050602093909301939093525092915050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52603160045260245ffd5b604081525f6124e060408301856121f3565b90508260208301529392505050565b634e487b7160e01b5f52601160045260245ffd5b8082028115828204841417611644576116446124ef565b80820180821115611644576116446124ef565b81810381811115611644576116446124ef565b5f60208284031215612550575f80fd5b5051919050565b600181811c9082168061256b57607f821691505b60208210810361258957634e487b7160e01b5f52602260045260245ffd5b50919050565b5f6bffffffffffffffffffffffff19808660601b168352808560601b1660148401525082516125c5816028850160208701611f39565b91909101602801949350505050565b601f8211156103af57805f5260205f20601f840160051c810160208510156125f95750805b601f840160051c820191505b81811015612618575f8155600101612605565b5050505050565b81516001600160401b0381111561263857612638611fac565b61264c816126468454612557565b846125d4565b602080601f83116001811461267f575f84156126685750858301515b5f19600386901b1c1916600185901b1785556126d6565b5f85815260208120601f198616915b828110156126ad5788860151825594840194600190910190840161268e565b50858210156126ca57878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b81515f9082906020808601845b83811015612707578151855293820193908201906001016126eb565b50929695505050505050565b5f8251612724818460208701611f39565b9190910192915050565b5f6001820161273f5761273f6124ef565b5060010190565b634e487b7160e01b5f52602160045260245ffdfe3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0043697068657274657874566572696669636174696f6e28627974657333325b5d20637448616e646c65732c616464726573732075736572416464726573732c6164647265737320636f6e7472616374416464726573732c75696e7432353620636f6e7472616374436861696e49642c62797465732065787472614461746129360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbce910845fd818f61127c84f3586776436a37dead33625056c65162537e3373600f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa(Zb\0\x01\0_9_\x81\x81a\x10\xC2\x01R\x81\x81a\x10\xEB\x01Ra\x12\xCE\x01Ra(Z_\xF3\xFE`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80c\x84\xB0\x19n\x11a\0\x92W\x80c\xAD<\xB1\xCC\x11a\0bW\x80c\xAD<\xB1\xCC\x14a\x02jW\x80c\xBA\xC2+\xB8\x14a\x02\x9AW\x80c\xDAS\xC4}\x14a\x02\xAEW\x80c\xE61}\xF5\x14a\x02\xCDW\x80c\xE7R5\xB8\x14a\x02\xECW_\x80\xFD[\x80c\x84\xB0\x19n\x14a\x01\xEFW\x80c\x8B!\x81#\x14a\x02\x16W\x80c\x91d\xD0\xAE\x14a\x02*W\x80c\x96\x0B\xFE\x04\x14a\x02KW_\x80\xFD[\x80cT\x13\x0C\xCD\x11a\0\xCDW\x80cT\x13\x0C\xCD\x14a\x01sW\x80c^\xEDvu\x14a\x01\x87W\x80cz)\x7FK\x14a\x01\xA6W\x80c}\xF7>'\x14a\x01\xC0W_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c53L#\x14a\x01(W\x80cO\x1E\xF2\x86\x14a\x01>W\x80cR\xD1\x90-\x14a\x01QW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03\x1FV[`@Qa\x01\x1F\x91\x90a\x1F\x86V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01<a\x03\x8AV[\0[a\x01<a\x01L6`\x04a [V[a\x03\xB4V[4\x80\x15a\x01\\W_\x80\xFD[Pa\x01ea\x03\xD3V[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01~W_\x80\xFD[Pa\x01\x12a\x03\xEEV[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01<a\x01\xA16`\x04a \xA7V[a\x04\nV[4\x80\x15a\x01\xB1W_\x80\xFD[P`@Q_\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xCBW_\x80\xFD[Pa\x01\xDFa\x01\xDA6`\x04a!AV[a\x05\x98V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xFAW_\x80\xFD[Pa\x02\x03a\x05\xC1V[`@Qa\x01\x1F\x97\x96\x95\x94\x93\x92\x91\x90a!\\V[4\x80\x15a\x02!W_\x80\xFD[Pa\x01ea\x06\x8FV[4\x80\x15a\x025W_\x80\xFD[Pa\x02>a\x06\xB2V[`@Qa\x01\x1F\x91\x90a\"6V[4\x80\x15a\x02VW_\x80\xFD[Pa\x01<a\x02e6`\x04a\"HV[a\x07!V[4\x80\x15a\x02uW_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x02\xA5W_\x80\xFD[Pa\x01<a\x08>V[4\x80\x15a\x02\xB9W_\x80\xFD[Pa\x01<a\x02\xC86`\x04a\"_V[a\x08\xECV[4\x80\x15a\x02\xD8W_\x80\xFD[Pa\x01ea\x02\xE76`\x04a#\x11V[a\x0C\tV[4\x80\x15a\x02\xF7W_\x80\xFD[P\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02Ta\x01eV[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01l$\xB78:\xBA+2\xB94\xB34\xB2\xB9`\x99\x1B\x81RPa\x03P_a\x10(V[a\x03Z`\x03a\x10(V[a\x03c_a\x10(V[`@Q` \x01a\x03v\x94\x93\x92\x91\x90a#\xB3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\\_\x80]`\x01\x90\x81\x01\x90\x80[\x82\x81\x10\x15a\x03\xAFW\x80\\_\x82]_\x81]P\x81\x01a\x03\x97V[PPPV[a\x03\xBCa\x10\xB7V[a\x03\xC5\x82a\x11]V[a\x03\xCF\x82\x82a\x12\x07V[PPV[_a\x03\xDCa\x12\xC3V[P_\x80Q` a'\xFA\x839\x81Q\x91R\x90V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a'{`\x7F\x919\x81V[_\x80Q` a(:\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x04KW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` a(:\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x04\x81WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x04\x9FW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\x11\x81Rp$\xB78:\xBA+2\xB94\xB34\xB1\xB0\xBA4\xB7\xB7`y\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\x05\x0B\x91\x89\x89a\x13\x0CV[a\x05H\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa\x08\xEC\x91PPV[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a'[\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x90V[_``\x80\x82\x80\x80\x83\x81_\x80Q` a(\x1A\x839\x81Q\x91R\x80T\x90\x91P\x15\x80\x15a\x05\xECWP`\x01\x81\x01T\x15[a\x065W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x06=a\x13&V[a\x06Ea\x13\xDDV[`\x04\x92\x90\x92\x01T`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x93\x99P`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x83\x04\x16\x98P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x96P\x94P\x90\x92P\x90PV[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a'{`\x7F\x919\x80Q\x90` \x01 \x81V[``__\x80Q` a'[\x839\x81Q\x91R`\x01\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a\x07\x16W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06\xF8W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07qW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x95\x91\x90a$0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xC8W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x06,V[a\x07\xD1\x81a\x14\x1BV[`@Q_\x80Q` a'[\x839\x81Q\x91R\x90\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x90a\x082\x90\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x90\x85\x90a$KV[`@Q\x80\x91\x03\x90\xA1PPV[_\x80Q` a(:\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x08tWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x08\x92W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17`\xFF`@\x1B\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x082V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t<W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t`\x91\x90a$0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\x93W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x06,V[\x81Q_\x81\x90\x03a\t\xB6W`@Qc\x12\x86\xE9Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R_\x80Q` a'[\x839\x81Q\x91R\x93_\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\n9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\x1BW[PP\x83Q\x93\x94P_\x92PPP[\x81\x81\x10\x15a\n\xDDW_\x84_\x01_\x85\x84\x81Q\x81\x10a\neWa\nea$\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x01\x80T\x80a\n\xB5Wa\n\xB5a$\xBAV[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\nFV[P_[\x84\x81\x10\x15a\x0B\xBEW_\x87\x82\x81Q\x81\x10a\n\xFBWa\n\xFBa$\xA6V[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0B7W`@Qc\x04\x06\x9C\xA7`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x86\x90R`@\x90 T`\xFF\x16\x15a\x0BpW`@QcW-\xE7\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R` \x86\x81R`@\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x87\x81\x01\x80T\x80\x83\x01\x82U\x90\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x92\x17\x90\x91U\x01a\n\xE0V[Pa\x0B\xC8\x85a\x14\x1BV[\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x86\x86`@Qa\x0B\xF9\x92\x91\x90a$\xCEV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80_a\x0C\x1E\x84\x87_\x01Q\x88` \x01Qa\x14\x92V[\x90\x92P\x90P`\x01`\x01`@\x1B\x03`\x10\x86\x90\x1C\x16F\x81\x14a\x0CQW`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\xFF`P\x82\x90\x1C\x16\x84a\x0F\xA4W\x86Q_\x81\x90\x03a\x0C\x82W`@QcY$\x0E\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88_\x81Q\x81\x10a\x0C\x95Wa\x0C\x95a$\xA6V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P_\x89`\x01\x81Q\x81\x10a\x0C\xBCWa\x0C\xBCa$\xA6V[\x01` \x01Q`\xF8\x1C\x90P\x83\x82\x11\x15\x80a\x0C\xD5WP`\xFE\x84\x11[\x15a\x0C\xF3W`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\xFF\x82`Aa%\x03V[a\r\n\x84` a%\x03V[a\r\x15\x90`\x02a%\x1AV[a\r\x1F\x91\x90a%\x1AV[\x90P\x80\x84\x10\x15a\rBW`@Qc\x18\x17\xEC\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\r[Wa\r[a\x1F\xACV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\r\xE4W` \x81\x02\x8D\x01`\"\x01Q`\xFF\x81\x16\x15a\r\xBDW`@Qc\x17\xDF\x86\xD5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x83\x81Q\x81\x10a\r\xD0Wa\r\xD0a$\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\r\x89V[P_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xFEWa\r\xFEa\x1F\xACV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E1W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\x1CW\x90P[P\x90P_[\x84\x81\x10\x15a\x0E\x99Wa\x0Et\x8Ea\x0EM\x83`Aa%\x03V[a\x0EX\x89` a%\x03V[a\x0Ec\x90`\x02a%\x1AV[a\x0Em\x91\x90a%\x1AV[`Aa\x14\xD2V[\x82\x82\x81Q\x81\x10a\x0E\x86Wa\x0E\x86a$\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0E6V[Pa\x0E\xDB`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x82\x81_\x01\x81\x90RP\x8F_\x01Q\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8F` \x01Q\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPF\x81``\x01\x81\x81RPPa\x0FF\x8E\x85\x86\x8Aa\x0FA\x91\x90a%-V[a\x14\xD2V[`\x80\x82\x01Ra\x0FU\x81\x83a\x15,V[a\x0F^\x8Ba\x15_V[\x82\x88\x81Q\x81\x10a\x0FpWa\x0Fpa$\xA6V[` \x02` \x01\x01Q_\x1C\x89\x14a\x0F\x98W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPa\x10\x1AV[_\x87_\x81Q\x81\x10a\x0F\xB7Wa\x0F\xB7a$\xA6V[\x01` \x01Q`\xF8\x1C\x90P\x81\x81\x11\x15\x80a\x0F\xD0WP`\xFE\x82\x11[\x15a\x0F\xEEW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x02\x88\x01`\"\x01Q\x83\x81\x14a\x10\x17W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP[P\x93PPPP[\x93\x92PPPV[``_a\x104\x83a\x15rV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10RWa\x10Ra\x1F\xACV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10|W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x10\x86WP\x93\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x11=WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x111_\x80Q` a'\xFA\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x11[W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD1\x91\x90a$0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\x04W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x06,V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x12aWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12^\x91\x81\x01\x90a%@V[`\x01[a\x12\x89W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x06,V[_\x80Q` a'\xFA\x839\x81Q\x91R\x81\x14a\x12\xB9W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06,V[a\x03\xAF\x83\x83a\x16JV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11[W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\x14a\x16\x9FV[a\x13 \x84\x84\x84\x84a\x16\xD5V[PPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02\x80T``\x91_\x80Q` a(\x1A\x839\x81Q\x91R\x91a\x13d\x90a%WV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x90\x90a%WV[\x80\x15a\x07\x16W\x80`\x1F\x10a\x13\xB2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x16V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xBEWP\x93\x96\x95PPPPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x03\x80T``\x91_\x80Q` a(\x1A\x839\x81Q\x91R\x91a\x13d\x90a%WV[\x80_\x03a\x14;W`@Qc\x01Q\xF11`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01T_\x80Q` a'[\x839\x81Q\x91R\x90\x82\x11\x15a\x14\x8CW`@Qc5\x19Nc`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x01UV[_\x80_\x80\x84\x86\x88`@Q` \x01a\x14\xAB\x93\x92\x91\x90a%\x8FV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x80\\\x98\x90\x97P\x95PPPPPPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xECWa\x14\xECa\x1F\xACV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15\x16W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x83` \x86\x01\x01` \x83\x01^\x93\x92PPPV[_a\x156\x83a\x17\x83V[\x90Pa\x15B\x81\x83a\x18ZV[a\x03\xAFW`@QcKPl\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81]`\x01_\\\x01\x81\x81]\x80_]PPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x15\xB0Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x15\xDCWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x15\xFAWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x16\x12Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x16&Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x168W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x16DW`\x01\x01[\x92\x91PPV[a\x16S\x82a\x19\xFDV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x16\x97Wa\x03\xAF\x82\x82a\x1A`V[a\x03\xCFa\x1A\xD2V[_\x80Q` a(:\x839\x81Q\x91RT`\x01`@\x1B\x90\x04`\xFF\x16a\x11[W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\xDDa\x16\x9FV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x04\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1B`\x01`\x01`@\x1B\x03\x84\x16\x02\x17\x90U_\x80Q` a(\x1A\x839\x81Q\x91R\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02a\x17c\x86\x82a&\x1FV[P`\x03\x81\x01a\x17r\x85\x82a&\x1FV[P_\x80\x82U`\x01\x90\x91\x01UPPPPV[_a\x16D`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a'{`\x7F\x919\x80Q` \x91\x82\x01 \x84Q`@Q\x91\x92a\x17\xB9\x92\x01a&\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a\x17\xF3\x91\x90a'\x13V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16``\x85\x01R\x91\x16`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1A\xF1V[\x80Q_\x90\x80\x82\x03a\x18~W`@QcY\x85\x19!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x18\xA7\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02T\x90V[\x90P\x80\x82\x10\x15a\x18\xCDW`@Qc.W\xFF\x83`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x06,V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xE6Wa\x18\xE6a\x1F\xACV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x0FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x84\x81\x10\x15a\x19\xE6W_a\x19A\x89\x89\x84\x81Q\x81\x10a\x194Wa\x194a$\xA6V[` \x02` \x01\x01Qa\x1B\x1DV[\x90Pa\x19L\x81a\x05\x98V[a\x19tW`@Qc\xBF\x18\xAFC`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06,V[\x80\\a\x19\xBDW\x80\x84\x84\x81Q\x81\x10a\x19\x8DWa\x19\x8Da$\xA6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x19\xAF\x81a'.V[\x93PPa\x19\xBD\x81`\x01a\x1B1V[\x84\x83\x10a\x19\xDDWa\x19\xCE\x84\x84a\x1B8V[`\x01\x96PPPPPPPa\x16DV[P`\x01\x01a\x19\x15V[Pa\x19\xF1\x82\x82a\x1B8V[P_\x96\x95PPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1A2W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06,V[_\x80Q` a'\xFA\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1A|\x91\x90a'\x13V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1A\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1A\xB9V[``\x91P[P\x91P\x91Pa\x1A\xC9\x85\x83\x83a\x1BmV[\x95\x94PPPPPV[4\x15a\x11[W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16Da\x1A\xFDa\x1B\xC9V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80a\x1B)\x84\x84a\x1B\xD7V[\x94\x93PPPPV[\x80\x82]PPV[_[\x81\x81\x10\x15a\x03\xAFWa\x1Be\x83\x82\x81Q\x81\x10a\x1BWWa\x1BWa$\xA6V[` \x02` \x01\x01Q_a\x1B1V[`\x01\x01a\x1B:V[``\x82a\x1B\x82Wa\x1B}\x82a\x1B\xFFV[a\x10!V[\x81Q\x15\x80\x15a\x1B\x99WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1B\xC2W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x06,V[P\x80a\x10!V[_a\x1B\xD2a\x1C(V[\x90P\x90V[_\x80_\x80a\x1B\xE5\x86\x86a\x1C\xC6V[\x92P\x92P\x92Pa\x1B\xF5\x82\x82a\x1D\x0FV[P\x90\x94\x93PPPPV[\x80Q\x15a\x1C\x0FW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x80Q` a(\x1A\x839\x81Q\x91R\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1C`a\x1D\xC7V[a\x1Cha\x1E/V[`\x04\x84\x01T`@\x80Q` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R`\x01`\xA0\x1B\x81\x04`\x01`\x01`@\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x90V[_\x80_\x83Q`A\x03a\x1C\xFDW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x1C\xEF\x88\x82\x85\x85a\x1EqV[\x95P\x95P\x95PPPPa\x1D\x08V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x1D\"Wa\x1D\"a'FV[\x03a\x1D+WPPV[`\x01\x82`\x03\x81\x11\x15a\x1D?Wa\x1D?a'FV[\x03a\x1D]W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x1DqWa\x1Dqa'FV[\x03a\x1D\x92W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06,V[`\x03\x82`\x03\x81\x11\x15a\x1D\xA6Wa\x1D\xA6a'FV[\x03a\x03\xCFW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06,V[__\x80Q` a(\x1A\x839\x81Q\x91R\x81a\x1D\xDFa\x13&V[\x80Q\x90\x91P\x15a\x1D\xF7W\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15a\x1E\x06W\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[__\x80Q` a(\x1A\x839\x81Q\x91R\x81a\x1EGa\x13\xDDV[\x80Q\x90\x91P\x15a\x1E_W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15a\x1E\x06W\x93\x92PPPV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x1E\xAAWP_\x91P`\x03\x90P\x82a\x1F/V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1E\xFBW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F&WP_\x92P`\x01\x91P\x82\x90Pa\x1F/V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_[\x83\x81\x10\x15a\x1FSW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F;V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x1Fr\x81` \x86\x01` \x86\x01a\x1F9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x10!` \x83\x01\x84a\x1F[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12\x04W_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F\xE8Wa\x1F\xE8a\x1F\xACV[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x1F\xFFW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a \x18Wa \x18a\x1F\xACV[a +`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1F\xC0V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a ?W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a lW_\x80\xFD[\x825a w\x81a\x1F\x98V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x91W_\x80\xFD[a \x9D\x85\x82\x86\x01a\x1F\xF0V[\x91PP\x92P\x92\x90PV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a \xBBW_\x80\xFD[\x855a \xC6\x81a\x1F\x98V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x16\x82\x14a \xE2W_\x80\xFD[\x90\x94P`@\x87\x015\x90\x80\x82\x11\x15a \xF7W_\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a!\nW_\x80\xFD[\x815\x81\x81\x11\x15a!\x18W_\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a!,W_\x80\xFD[\x96\x99\x95\x98PP` \x01\x95``\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a!QW_\x80\xFD[\x815a\x10!\x81a\x1F\x98V[`\xFF`\xF8\x1B\x88\x16\x81R_` `\xE0` \x84\x01Ra!|`\xE0\x84\x01\x8Aa\x1F[V[\x83\x81\x03`@\x85\x01Ra!\x8E\x81\x8Aa\x1F[V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R` \x80\x88\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a!\xE1W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a!\xC5V[P\x90\x9C\x9BPPPPPPPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\"+W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\"\x06V[P\x94\x95\x94PPPPPV[` \x81R_a\x10!` \x83\x01\x84a!\xF3V[_` \x82\x84\x03\x12\x15a\"XW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\"pW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"\x86W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\"\x99W_\x80\xFD[\x815` \x82\x82\x11\x15a\"\xADWa\"\xADa\x1F\xACV[\x81`\x05\x1B\x92Pa\"\xBE\x81\x84\x01a\x1F\xC0V[\x82\x81R\x92\x84\x01\x81\x01\x92\x81\x81\x01\x90\x89\x85\x11\x15a\"\xD7W_\x80\xFD[\x94\x82\x01\x94[\x84\x86\x10\x15a#\x01W\x855\x93Pa\"\xF1\x84a\x1F\x98V[\x83\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\"\xDCV[\x99\x97\x90\x91\x015\x97PPPPPPPV[_\x80_\x83\x85\x03`\x80\x81\x12\x15a#$W_\x80\xFD[`@\x81\x12\x15a#1W_\x80\xFD[P`@Q`@\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a#UWa#Ua\x1F\xACV[\x81`@R\x865\x91Pa#f\x82a\x1F\x98V[\x90\x82R` \x86\x015\x90a#x\x82a\x1F\x98V[\x81` \x84\x01R\x82\x95P`@\x87\x015\x94P``\x87\x015\x92P\x80\x83\x11\x15a#\x9BW_\x80\xFD[PPa#\xA9\x86\x82\x87\x01a\x1F\xF0V[\x91PP\x92P\x92P\x92V[_\x85Qa#\xC4\x81\x84` \x8A\x01a\x1F9V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa#\xE3\x81`\x02\x84\x01` \x8A\x01a\x1F9V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa$\x07\x81`\x03\x85\x01` \x8A\x01a\x1F9V[`\x03\x92\x01\x91\x82\x01R\x83Qa$\"\x81`\x04\x84\x01` \x88\x01a\x1F9V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a$@W_\x80\xFD[\x81Qa\x10!\x81a\x1F\x98V[_`@\x82\x01`@\x83R\x80\x85T\x80\x83R``\x85\x01\x91P\x86_R` \x92P` _ _[\x82\x81\x10\x15a$\x92W\x81T`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92`\x01\x91\x82\x01\x91\x01a$mV[PPP` \x93\x90\x93\x01\x93\x90\x93RP\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`@\x81R_a$\xE0`@\x83\x01\x85a!\xF3V[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x16DWa\x16Da$\xEFV[\x80\x82\x01\x80\x82\x11\x15a\x16DWa\x16Da$\xEFV[\x81\x81\x03\x81\x81\x11\x15a\x16DWa\x16Da$\xEFV[_` \x82\x84\x03\x12\x15a%PW_\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%kW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a%\x89WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x86``\x1B\x16\x83R\x80\x85``\x1B\x16`\x14\x84\x01RP\x82Qa%\xC5\x81`(\x85\x01` \x87\x01a\x1F9V[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[`\x1F\x82\x11\x15a\x03\xAFW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a%\xF9WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a&\x18W_\x81U`\x01\x01a&\x05V[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&8Wa&8a\x1F\xACV[a&L\x81a&F\x84Ta%WV[\x84a%\xD4V[` \x80`\x1F\x83\x11`\x01\x81\x14a&\x7FW_\x84\x15a&hWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua&\xD6V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a&\xADW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a&\x8EV[P\x85\x82\x10\x15a&\xCAW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a'\x07W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a&\xEBV[P\x92\x96\x95PPPPPPV[_\x82Qa'$\x81\x84` \x87\x01a\x1F9V[\x91\x90\x91\x01\x92\x91PPV[_`\x01\x82\x01a'?Wa'?a$\xEFV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD\xFE?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\0CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\0\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100fa575f3560e01c806384b0196e11610092578063ad3cb1cc11610062578063ad3cb1cc1461026a578063bac22bb81461029a578063da53c47d146102ae578063e6317df5146102cd578063e75235b8146102ec575f80fd5b806384b0196e146101ef5780638b218123146102165780639164d0ae1461022a578063960bfe041461024b575f80fd5b806354130ccd116100cd57806354130ccd146101735780635eed7675146101875780637a297f4b146101a65780637df73e27146101c0575f80fd5b80630d8e6e2c146100fe57806335334c23146101285780634f1ef2861461013e57806352d1902d14610151575b5f80fd5b348015610109575f80fd5b5061011261031f565b60405161011f9190611f86565b60405180910390f35b348015610133575f80fd5b5061013c61038a565b005b61013c61014c36600461205b565b6103b4565b34801561015c575f80fd5b506101656103d3565b60405190815260200161011f565b34801561017e575f80fd5b506101126103ee565b348015610192575f80fd5b5061013c6101a13660046120a7565b61040a565b3480156101b1575f80fd5b506040515f815260200161011f565b3480156101cb575f80fd5b506101df6101da366004612141565b610598565b604051901515815260200161011f565b3480156101fa575f80fd5b506102036105c1565b60405161011f979695949392919061215c565b348015610221575f80fd5b5061016561068f565b348015610235575f80fd5b5061023e6106b2565b60405161011f9190612236565b348015610256575f80fd5b5061013c610265366004612248565b610721565b348015610275575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156102a5575f80fd5b5061013c61083e565b3480156102b9575f80fd5b5061013c6102c836600461225f565b6108ec565b3480156102d8575f80fd5b506101656102e7366004612311565b610c09565b3480156102f7575f80fd5b507f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0254610165565b60606040518060400160405280600d81526020016c24b7383aba2b32b934b334b2b960991b8152506103505f611028565b61035a6003611028565b6103635f611028565b60405160200161037694939291906123b3565b604051602081830303815290604052905090565b5f5c5f805d600190810190805b828110156103af57805c5f825d5f815d508101610397565b505050565b6103bc6110b7565b6103c58261115d565b6103cf8282611207565b5050565b5f6103dc6112c3565b505f805160206127fa83398151915290565b6040518060a00160405280607f815260200161277b607f913981565b5f8051602061283a833981519152546001600160401b03166001600160401b031660011461044b57604051636f4f731f60e01b815260040160405180910390fd5b5f8051602061283a833981519152805460049190600160401b900460ff1680610481575080546001600160401b03808416911610155b1561049f5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b178155604080518082018252601181527024b7383aba2b32b934b334b1b0ba34b7b760791b602080830191909152825180840190935260018352603160f81b9083015261050b91898961130c565b6105488585808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508792506108ec915050565b805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a150505050505050565b6001600160a01b03165f9081525f8051602061275b833981519152602052604090205460ff1690565b5f60608082808083815f8051602061281a83398151915280549091501580156105ec57506001810154155b6106355760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b60448201526064015b60405180910390fd5b61063d611326565b6106456113dd565b60049290920154604080515f80825260208201909252600f60f81b9c939b509399506001600160401b03600160a01b83041698506001600160a01b03909116965094509092509050565b6040518060a00160405280607f815260200161277b607f91398051906020012081565b60605f5f8051602061275b8339815191526001810180546040805160208084028201810190925282815293945083018282801561071657602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116106f8575b505050505091505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610771573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107959190612430565b6001600160a01b0316336001600160a01b0316146107c85760405163021bfda160e41b815233600482015260240161062c565b6107d18161141b565b6040515f8051602061275b833981519152907f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea90610832907f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0190859061244b565b60405180910390a15050565b5f8051602061283a833981519152805460049190600160401b900460ff1680610874575080546001600160401b03808416911610155b156108925760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1760ff60401b191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602001610832565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561093c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109609190612430565b6001600160a01b0316336001600160a01b0316146109935760405163021bfda160e41b815233600482015260240161062c565b81515f8190036109b657604051631286e95160e01b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace018054604080516020808402820181019092528281525f8051602061275b833981519152935f939192909190830182828015610a3957602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610a1b575b505083519394505f925050505b81811015610add575f845f015f858481518110610a6557610a656124a6565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f205f6101000a81548160ff02191690831515021790555083600101805480610ab557610ab56124ba565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610a46565b505f5b84811015610bbe575f878281518110610afb57610afb6124a6565b602002602001015190505f6001600160a01b0316816001600160a01b031603610b37576040516304069ca760e21b815260040160405180910390fd5b6001600160a01b0381165f9081526020869052604090205460ff1615610b705760405163572de7c960e11b815260040160405180910390fd5b6001600160a01b03165f818152602086815260408220805460ff1916600190811790915587810180548083018255908452919092200180546001600160a01b03191690921790915501610ae0565b50610bc88561141b565b7f1dcd7e1de916ad3be0c1097968029899e2e7d0195cfa6967e16520c0e8d07cea8686604051610bf99291906124ce565b60405180910390a1505050505050565b5f805f610c1e84875f01518860200151611492565b90925090506001600160401b03601086901c16468114610c5157604051633d23e4d160e11b815260040160405180910390fd5b8560ff605082901c1684610fa45786515f819003610c82576040516359240e8b60e11b815260040160405180910390fd5b5f885f81518110610c9557610c956124a6565b602001015160f81c60f81b60f81c60ff1690505f89600181518110610cbc57610cbc6124a6565b016020015160f81c90508382111580610cd5575060fe84115b15610cf3576040516363df817160e01b815260040160405180910390fd5b5f610cff826041612503565b610d0a846020612503565b610d1590600261251a565b610d1f919061251a565b905080841015610d4257604051631817ecd760e01b815260040160405180910390fd5b5f836001600160401b03811115610d5b57610d5b611fac565b604051908082528060200260200182016040528015610d84578160200160208202803683370190505b5090505f5b84811015610de457602081028d016022015160ff811615610dbd576040516317df86d560e21b815260040160405180910390fd5b80838381518110610dd057610dd06124a6565b602090810291909101015250600101610d89565b505f836001600160401b03811115610dfe57610dfe611fac565b604051908082528060200260200182016040528015610e3157816020015b6060815260200190600190039081610e1c5790505b5090505f5b84811015610e9957610e748e610e4d836041612503565b610e58896020612503565b610e6390600261251a565b610e6d919061251a565b60416114d2565b828281518110610e8657610e866124a6565b6020908102919091010152600101610e36565b50610edb6040518060a00160405280606081526020015f6001600160a01b031681526020015f6001600160a01b031681526020015f8152602001606081525090565b82815f01819052508f5f015181602001906001600160a01b031690816001600160a01b0316815250508f6020015181604001906001600160a01b031690816001600160a01b03168152505046816060018181525050610f468e85868a610f41919061252d565b6114d2565b6080820152610f55818361152c565b610f5e8b61155f565b828881518110610f7057610f706124a6565b60200260200101515f1c8914610f9857604051624b1bf160e31b815260040160405180910390fd5b5050505050505061101a565b5f875f81518110610fb757610fb76124a6565b016020015160f81c90508181111580610fd0575060fe82115b15610fee576040516363df817160e01b815260040160405180910390fd5b6020820288016022015183811461101757604051624b1bf160e31b815260040160405180910390fd5b50505b5093505050505b9392505050565b60605f61103483611572565b60010190505f816001600160401b0381111561105257611052611fac565b6040519080825280601f01601f19166020018201604052801561107c576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461108657509392505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061113d57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166111315f805160206127fa833981519152546001600160a01b031690565b6001600160a01b031614155b1561115b5760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156111ad573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111d19190612430565b6001600160a01b0316336001600160a01b0316146112045760405163021bfda160e41b815233600482015260240161062c565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015611261575060408051601f3d908101601f1916820190925261125e91810190612540565b60015b61128957604051634c9c8ce360e01b81526001600160a01b038316600482015260240161062c565b5f805160206127fa83398151915281146112b957604051632a87526960e21b81526004810182905260240161062c565b6103af838361164a565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461115b5760405163703e46dd60e11b815260040160405180910390fd5b61131461169f565b611320848484846116d5565b50505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360280546060915f8051602061281a8339815191529161136490612557565b80601f016020809104026020016040519081016040528092919081815260200182805461139090612557565b80156107165780601f106113b257610100808354040283529160200191610716565b820191905f5260205f20905b8154815290600101906020018083116113be5750939695505050505050565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360380546060915f8051602061281a8339815191529161136490612557565b805f0361143b57604051630151f13160e71b815260040160405180910390fd5b7f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace01545f8051602061275b8339815191529082111561148c576040516335194e6360e01b815260040160405180910390fd5b60020155565b5f805f808486886040516020016114ab9392919061258f565b60408051808303601f190181529190528051602090910120805c9890975095505050505050565b6060816001600160401b038111156114ec576114ec611fac565b6040519080825280601f01601f191660200182016040528015611516576020820181803683370190505b50905081836020860101602083015e9392505050565b5f61153683611783565b9050611542818361185a565b6103af57604051634b506ccd60e01b815260040160405180910390fd5b6001815d60015f5c0181815d805f5d5050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106115b05772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106115dc576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc1000083106115fa57662386f26fc10000830492506010015b6305f5e1008310611612576305f5e100830492506008015b612710831061162657612710830492506004015b60648310611638576064830492506002015b600a8310611644576001015b92915050565b611653826119fd565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611697576103af8282611a60565b6103cf611ad2565b5f8051602061283a83398151915254600160401b900460ff1661115b57604051631afcd79f60e31b815260040160405180910390fd5b6116dd61169f565b7fe910845fd818f61127c84f3586776436a37dead33625056c65162537e337360480546001600160a01b0384166001600160e01b031990911617600160a01b6001600160401b038416021790555f8051602061281a8339815191527fe910845fd818f61127c84f3586776436a37dead33625056c65162537e3373602611763868261261f565b5060038101611772858261261f565b505f80825560019091015550505050565b5f6116446040518060a00160405280607f815260200161277b607f91398051602091820120845160405191926117b992016126de565b6040516020818303038152906040528051906020012084602001518560400151866060015187608001516040516020016117f39190612713565b60408051601f198184030181528282528051602091820120908301979097528101949094526001600160a01b0392831660608501529116608083015260a082015260c081019190915260e00160405160208183030381529060405280519060200120611af1565b80515f9080820361187e57604051635985192160e11b815260040160405180910390fd5b5f6118a77f3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace025490565b9050808210156118cd57604051632e57ff8360e21b81526004810183905260240161062c565b5f826001600160401b038111156118e6576118e6611fac565b60405190808252806020026020018201604052801561190f578160200160208202803683370190505b5090505f805b848110156119e6575f61194189898481518110611934576119346124a6565b6020026020010151611b1d565b905061194c81610598565b6119745760405163bf18af4360e01b81526001600160a01b038216600482015260240161062c565b805c6119bd578084848151811061198d5761198d6124a6565b6001600160a01b0390921660209283029190910190910152826119af8161272e565b9350506119bd816001611b31565b8483106119dd576119ce8484611b38565b60019650505050505050611644565b50600101611915565b506119f18282611b38565b505f9695505050505050565b806001600160a01b03163b5f03611a3257604051634c9c8ce360e01b81526001600160a01b038216600482015260240161062c565b5f805160206127fa83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051611a7c9190612713565b5f60405180830381855af49150503d805f8114611ab4576040519150601f19603f3d011682016040523d82523d5f602084013e611ab9565b606091505b5091509150611ac9858383611b6d565b95945050505050565b341561115b5760405163b398979f60e01b815260040160405180910390fd5b5f611644611afd611bc9565b8360405161190160f01b8152600281019290925260228201526042902090565b5f80611b298484611bd7565b949350505050565b80825d5050565b5f5b818110156103af57611b65838281518110611b5757611b576124a6565b60200260200101515f611b31565b600101611b3a565b606082611b8257611b7d82611bff565b611021565b8151158015611b9957506001600160a01b0384163b155b15611bc257604051639996b31560e01b81526001600160a01b038516600482015260240161062c565b5080611021565b5f611bd2611c28565b905090565b5f805f80611be58686611cc6565b925092509250611bf58282611d0f565b5090949350505050565b805115611c0f5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5f8051602061281a8339815191527f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f611c60611dc7565b611c68611e2f565b60048401546040805160208101959095528401929092526060830152600160a01b81046001600160401b031660808301526001600160a01b031660a082015260c0016040516020818303038152906040528051906020012091505090565b5f805f8351604103611cfd576020840151604085015160608601515f1a611cef88828585611e71565b955095509550505050611d08565b505081515f91506002905b9250925092565b5f826003811115611d2257611d22612746565b03611d2b575050565b6001826003811115611d3f57611d3f612746565b03611d5d5760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115611d7157611d71612746565b03611d925760405163fce698f760e01b81526004810182905260240161062c565b6003826003811115611da657611da6612746565b036103cf576040516335e2f38360e21b81526004810182905260240161062c565b5f5f8051602061281a83398151915281611ddf611326565b805190915015611df757805160209091012092915050565b81548015611e06579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f5f8051602061281a83398151915281611e476113dd565b805190915015611e5f57805160209091012092915050565b60018201548015611e06579392505050565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0841115611eaa57505f91506003905082611f2f565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015611efb573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116611f2657505f925060019150829050611f2f565b92505f91508190505b9450945094915050565b5f5b83811015611f53578181015183820152602001611f3b565b50505f910152565b5f8151808452611f72816020860160208601611f39565b601f01601f19169290920160200192915050565b602081525f6110216020830184611f5b565b6001600160a01b0381168114611204575f80fd5b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b0381118282101715611fe857611fe8611fac565b604052919050565b5f82601f830112611fff575f80fd5b81356001600160401b0381111561201857612018611fac565b61202b601f8201601f1916602001611fc0565b81815284602083860101111561203f575f80fd5b816020850160208301375f918101602001919091529392505050565b5f806040838503121561206c575f80fd5b823561207781611f98565b915060208301356001600160401b03811115612091575f80fd5b61209d85828601611ff0565b9150509250929050565b5f805f805f608086880312156120bb575f80fd5b85356120c681611f98565b945060208601356001600160401b0380821682146120e2575f80fd5b909450604087013590808211156120f7575f80fd5b818801915088601f83011261210a575f80fd5b813581811115612118575f80fd5b8960208260051b850101111561212c575f80fd5b96999598505060200195606001359392505050565b5f60208284031215612151575f80fd5b813561102181611f98565b60ff60f81b881681525f602060e0602084015261217c60e084018a611f5b565b838103604085015261218e818a611f5b565b606085018990526001600160a01b038816608086015260a0850187905284810360c0860152855180825260208088019350909101905f5b818110156121e1578351835292840192918401916001016121c5565b50909c9b505050505050505050505050565b5f815180845260208085019450602084015f5b8381101561222b5781516001600160a01b031687529582019590820190600101612206565b509495945050505050565b602081525f61102160208301846121f3565b5f60208284031215612258575f80fd5b5035919050565b5f8060408385031215612270575f80fd5b82356001600160401b0380821115612286575f80fd5b818501915085601f830112612299575f80fd5b81356020828211156122ad576122ad611fac565b8160051b92506122be818401611fc0565b82815292840181019281810190898511156122d7575f80fd5b948201945b8486101561230157853593506122f184611f98565b83825294820194908201906122dc565b9997909101359750505050505050565b5f805f8385036080811215612324575f80fd5b6040811215612331575f80fd5b50604051604081016001600160401b03828210818311171561235557612355611fac565b816040528635915061236682611f98565b90825260208601359061237882611f98565b81602084015282955060408701359450606087013592508083111561239b575f80fd5b50506123a986828701611ff0565b9150509250925092565b5f85516123c4818460208a01611f39565b61103b60f11b90830190815285516123e3816002840160208a01611f39565b808201915050601760f91b8060028301528551612407816003850160208a01611f39565b60039201918201528351612422816004840160208801611f39565b016004019695505050505050565b5f60208284031215612440575f80fd5b815161102181611f98565b5f6040820160408352808554808352606085019150865f526020925060205f205f5b828110156124925781546001600160a01b03168452928401926001918201910161246d565b505050602093909301939093525092915050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52603160045260245ffd5b604081525f6124e060408301856121f3565b90508260208301529392505050565b634e487b7160e01b5f52601160045260245ffd5b8082028115828204841417611644576116446124ef565b80820180821115611644576116446124ef565b81810381811115611644576116446124ef565b5f60208284031215612550575f80fd5b5051919050565b600181811c9082168061256b57607f821691505b60208210810361258957634e487b7160e01b5f52602260045260245ffd5b50919050565b5f6bffffffffffffffffffffffff19808660601b168352808560601b1660148401525082516125c5816028850160208701611f39565b91909101602801949350505050565b601f8211156103af57805f5260205f20601f840160051c810160208510156125f95750805b601f840160051c820191505b81811015612618575f8155600101612605565b5050505050565b81516001600160401b0381111561263857612638611fac565b61264c816126468454612557565b846125d4565b602080601f83116001811461267f575f84156126685750858301515b5f19600386901b1c1916600185901b1785556126d6565b5f85815260208120601f198616915b828110156126ad5788860151825594840194600190910190840161268e565b50858210156126ca57878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b81515f9082906020808601845b83811015612707578151855293820193908201906001016126eb565b50929695505050505050565b5f8251612724818460208701611f39565b9190910192915050565b5f6001820161273f5761273f6124ef565b5060010190565b634e487b7160e01b5f52602160045260245ffdfe3f7d7a96c8c7024e92d37afccfc9b87773a33b9bc22e23134b683e74a50ace0043697068657274657874566572696669636174696f6e28627974657333325b5d20637448616e646c65732c616464726573732075736572416464726573732c6164647265737320636f6e7472616374416464726573732c75696e7432353620636f6e7472616374436861696e49642c62797465732065787472614461746129360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbce910845fd818f61127c84f3586776436a37dead33625056c65162537e3373600f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80c\x84\xB0\x19n\x11a\0\x92W\x80c\xAD<\xB1\xCC\x11a\0bW\x80c\xAD<\xB1\xCC\x14a\x02jW\x80c\xBA\xC2+\xB8\x14a\x02\x9AW\x80c\xDAS\xC4}\x14a\x02\xAEW\x80c\xE61}\xF5\x14a\x02\xCDW\x80c\xE7R5\xB8\x14a\x02\xECW_\x80\xFD[\x80c\x84\xB0\x19n\x14a\x01\xEFW\x80c\x8B!\x81#\x14a\x02\x16W\x80c\x91d\xD0\xAE\x14a\x02*W\x80c\x96\x0B\xFE\x04\x14a\x02KW_\x80\xFD[\x80cT\x13\x0C\xCD\x11a\0\xCDW\x80cT\x13\x0C\xCD\x14a\x01sW\x80c^\xEDvu\x14a\x01\x87W\x80cz)\x7FK\x14a\x01\xA6W\x80c}\xF7>'\x14a\x01\xC0W_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c53L#\x14a\x01(W\x80cO\x1E\xF2\x86\x14a\x01>W\x80cR\xD1\x90-\x14a\x01QW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03\x1FV[`@Qa\x01\x1F\x91\x90a\x1F\x86V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01<a\x03\x8AV[\0[a\x01<a\x01L6`\x04a [V[a\x03\xB4V[4\x80\x15a\x01\\W_\x80\xFD[Pa\x01ea\x03\xD3V[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01~W_\x80\xFD[Pa\x01\x12a\x03\xEEV[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01<a\x01\xA16`\x04a \xA7V[a\x04\nV[4\x80\x15a\x01\xB1W_\x80\xFD[P`@Q_\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xCBW_\x80\xFD[Pa\x01\xDFa\x01\xDA6`\x04a!AV[a\x05\x98V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xFAW_\x80\xFD[Pa\x02\x03a\x05\xC1V[`@Qa\x01\x1F\x97\x96\x95\x94\x93\x92\x91\x90a!\\V[4\x80\x15a\x02!W_\x80\xFD[Pa\x01ea\x06\x8FV[4\x80\x15a\x025W_\x80\xFD[Pa\x02>a\x06\xB2V[`@Qa\x01\x1F\x91\x90a\"6V[4\x80\x15a\x02VW_\x80\xFD[Pa\x01<a\x02e6`\x04a\"HV[a\x07!V[4\x80\x15a\x02uW_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x02\xA5W_\x80\xFD[Pa\x01<a\x08>V[4\x80\x15a\x02\xB9W_\x80\xFD[Pa\x01<a\x02\xC86`\x04a\"_V[a\x08\xECV[4\x80\x15a\x02\xD8W_\x80\xFD[Pa\x01ea\x02\xE76`\x04a#\x11V[a\x0C\tV[4\x80\x15a\x02\xF7W_\x80\xFD[P\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02Ta\x01eV[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01l$\xB78:\xBA+2\xB94\xB34\xB2\xB9`\x99\x1B\x81RPa\x03P_a\x10(V[a\x03Z`\x03a\x10(V[a\x03c_a\x10(V[`@Q` \x01a\x03v\x94\x93\x92\x91\x90a#\xB3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\\_\x80]`\x01\x90\x81\x01\x90\x80[\x82\x81\x10\x15a\x03\xAFW\x80\\_\x82]_\x81]P\x81\x01a\x03\x97V[PPPV[a\x03\xBCa\x10\xB7V[a\x03\xC5\x82a\x11]V[a\x03\xCF\x82\x82a\x12\x07V[PPV[_a\x03\xDCa\x12\xC3V[P_\x80Q` a'\xFA\x839\x81Q\x91R\x90V[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a'{`\x7F\x919\x81V[_\x80Q` a(:\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x04KW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` a(:\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x04\x81WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x04\x9FW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\x11\x81Rp$\xB78:\xBA+2\xB94\xB34\xB1\xB0\xBA4\xB7\xB7`y\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\x05\x0B\x91\x89\x89a\x13\x0CV[a\x05H\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa\x08\xEC\x91PPV[\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R_\x80Q` a'[\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x90V[_``\x80\x82\x80\x80\x83\x81_\x80Q` a(\x1A\x839\x81Q\x91R\x80T\x90\x91P\x15\x80\x15a\x05\xECWP`\x01\x81\x01T\x15[a\x065W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x06=a\x13&V[a\x06Ea\x13\xDDV[`\x04\x92\x90\x92\x01T`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x93\x99P`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x83\x04\x16\x98P`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x96P\x94P\x90\x92P\x90PV[`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a'{`\x7F\x919\x80Q\x90` \x01 \x81V[``__\x80Q` a'[\x839\x81Q\x91R`\x01\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a\x07\x16W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06\xF8W[PPPPP\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07qW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x95\x91\x90a$0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xC8W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x06,V[a\x07\xD1\x81a\x14\x1BV[`@Q_\x80Q` a'[\x839\x81Q\x91R\x90\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x90a\x082\x90\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x90\x85\x90a$KV[`@Q\x80\x91\x03\x90\xA1PPV[_\x80Q` a(:\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x08tWP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x08\x92W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17`\xFF`@\x1B\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x082V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t<W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t`\x91\x90a$0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\x93W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x06,V[\x81Q_\x81\x90\x03a\t\xB6W`@Qc\x12\x86\xE9Q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R_\x80Q` a'[\x839\x81Q\x91R\x93_\x93\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\n9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\n\x1BW[PP\x83Q\x93\x94P_\x92PPP[\x81\x81\x10\x15a\n\xDDW_\x84_\x01_\x85\x84\x81Q\x81\x10a\neWa\nea$\xA6V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x83`\x01\x01\x80T\x80a\n\xB5Wa\n\xB5a$\xBAV[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\nFV[P_[\x84\x81\x10\x15a\x0B\xBEW_\x87\x82\x81Q\x81\x10a\n\xFBWa\n\xFBa$\xA6V[` \x02` \x01\x01Q\x90P_`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0B7W`@Qc\x04\x06\x9C\xA7`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x86\x90R`@\x90 T`\xFF\x16\x15a\x0BpW`@QcW-\xE7\xC9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R` \x86\x81R`@\x82 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x87\x81\x01\x80T\x80\x83\x01\x82U\x90\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x92\x17\x90\x91U\x01a\n\xE0V[Pa\x0B\xC8\x85a\x14\x1BV[\x7F\x1D\xCD~\x1D\xE9\x16\xAD;\xE0\xC1\tyh\x02\x98\x99\xE2\xE7\xD0\x19\\\xFAig\xE1e \xC0\xE8\xD0|\xEA\x86\x86`@Qa\x0B\xF9\x92\x91\x90a$\xCEV[`@Q\x80\x91\x03\x90\xA1PPPPPPV[_\x80_a\x0C\x1E\x84\x87_\x01Q\x88` \x01Qa\x14\x92V[\x90\x92P\x90P`\x01`\x01`@\x1B\x03`\x10\x86\x90\x1C\x16F\x81\x14a\x0CQW`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85`\xFF`P\x82\x90\x1C\x16\x84a\x0F\xA4W\x86Q_\x81\x90\x03a\x0C\x82W`@QcY$\x0E\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x88_\x81Q\x81\x10a\x0C\x95Wa\x0C\x95a$\xA6V[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x90P_\x89`\x01\x81Q\x81\x10a\x0C\xBCWa\x0C\xBCa$\xA6V[\x01` \x01Q`\xF8\x1C\x90P\x83\x82\x11\x15\x80a\x0C\xD5WP`\xFE\x84\x11[\x15a\x0C\xF3W`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\xFF\x82`Aa%\x03V[a\r\n\x84` a%\x03V[a\r\x15\x90`\x02a%\x1AV[a\r\x1F\x91\x90a%\x1AV[\x90P\x80\x84\x10\x15a\rBW`@Qc\x18\x17\xEC\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\r[Wa\r[a\x1F\xACV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\r\xE4W` \x81\x02\x8D\x01`\"\x01Q`\xFF\x81\x16\x15a\r\xBDW`@Qc\x17\xDF\x86\xD5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x83\x81Q\x81\x10a\r\xD0Wa\r\xD0a$\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\r\x89V[P_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\xFEWa\r\xFEa\x1F\xACV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E1W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0E\x1CW\x90P[P\x90P_[\x84\x81\x10\x15a\x0E\x99Wa\x0Et\x8Ea\x0EM\x83`Aa%\x03V[a\x0EX\x89` a%\x03V[a\x0Ec\x90`\x02a%\x1AV[a\x0Em\x91\x90a%\x1AV[`Aa\x14\xD2V[\x82\x82\x81Q\x81\x10a\x0E\x86Wa\x0E\x86a$\xA6V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0E6V[Pa\x0E\xDB`@Q\x80`\xA0\x01`@R\x80``\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x82\x81_\x01\x81\x90RP\x8F_\x01Q\x81` \x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP\x8F` \x01Q\x81`@\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPPF\x81``\x01\x81\x81RPPa\x0FF\x8E\x85\x86\x8Aa\x0FA\x91\x90a%-V[a\x14\xD2V[`\x80\x82\x01Ra\x0FU\x81\x83a\x15,V[a\x0F^\x8Ba\x15_V[\x82\x88\x81Q\x81\x10a\x0FpWa\x0Fpa$\xA6V[` \x02` \x01\x01Q_\x1C\x89\x14a\x0F\x98W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPa\x10\x1AV[_\x87_\x81Q\x81\x10a\x0F\xB7Wa\x0F\xB7a$\xA6V[\x01` \x01Q`\xF8\x1C\x90P\x81\x81\x11\x15\x80a\x0F\xD0WP`\xFE\x82\x11[\x15a\x0F\xEEW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x02\x88\x01`\"\x01Q\x83\x81\x14a\x10\x17W`@QbK\x1B\xF1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP[P\x93PPPP[\x93\x92PPPV[``_a\x104\x83a\x15rV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10RWa\x10Ra\x1F\xACV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x10|W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x10\x86WP\x93\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x11=WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x111_\x80Q` a'\xFA\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x11[W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xD1\x91\x90a$0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\x04W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x06,V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x12aWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x12^\x91\x81\x01\x90a%@V[`\x01[a\x12\x89W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x06,V[_\x80Q` a'\xFA\x839\x81Q\x91R\x81\x14a\x12\xB9W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06,V[a\x03\xAF\x83\x83a\x16JV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11[W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13\x14a\x16\x9FV[a\x13 \x84\x84\x84\x84a\x16\xD5V[PPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02\x80T``\x91_\x80Q` a(\x1A\x839\x81Q\x91R\x91a\x13d\x90a%WV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x90\x90a%WV[\x80\x15a\x07\x16W\x80`\x1F\x10a\x13\xB2Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x16V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xBEWP\x93\x96\x95PPPPPPV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x03\x80T``\x91_\x80Q` a(\x1A\x839\x81Q\x91R\x91a\x13d\x90a%WV[\x80_\x03a\x14;W`@Qc\x01Q\xF11`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x01T_\x80Q` a'[\x839\x81Q\x91R\x90\x82\x11\x15a\x14\x8CW`@Qc5\x19Nc`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x01UV[_\x80_\x80\x84\x86\x88`@Q` \x01a\x14\xAB\x93\x92\x91\x90a%\x8FV[`@\x80Q\x80\x83\x03`\x1F\x19\x01\x81R\x91\x90R\x80Q` \x90\x91\x01 \x80\\\x98\x90\x97P\x95PPPPPPV[``\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xECWa\x14\xECa\x1F\xACV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x15\x16W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x83` \x86\x01\x01` \x83\x01^\x93\x92PPPV[_a\x156\x83a\x17\x83V[\x90Pa\x15B\x81\x83a\x18ZV[a\x03\xAFW`@QcKPl\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81]`\x01_\\\x01\x81\x81]\x80_]PPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x15\xB0Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x15\xDCWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x15\xFAWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x16\x12Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x16&Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x168W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x16DW`\x01\x01[\x92\x91PPV[a\x16S\x82a\x19\xFDV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x16\x97Wa\x03\xAF\x82\x82a\x1A`V[a\x03\xCFa\x1A\xD2V[_\x80Q` a(:\x839\x81Q\x91RT`\x01`@\x1B\x90\x04`\xFF\x16a\x11[W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\xDDa\x16\x9FV[\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x04\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1B`\x01`\x01`@\x1B\x03\x84\x16\x02\x17\x90U_\x80Q` a(\x1A\x839\x81Q\x91R\x7F\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\x02a\x17c\x86\x82a&\x1FV[P`\x03\x81\x01a\x17r\x85\x82a&\x1FV[P_\x80\x82U`\x01\x90\x91\x01UPPPPV[_a\x16D`@Q\x80`\xA0\x01`@R\x80`\x7F\x81R` \x01a'{`\x7F\x919\x80Q` \x91\x82\x01 \x84Q`@Q\x91\x92a\x17\xB9\x92\x01a&\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a\x17\xF3\x91\x90a'\x13V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16``\x85\x01R\x91\x16`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a\x1A\xF1V[\x80Q_\x90\x80\x82\x03a\x18~W`@QcY\x85\x19!`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x18\xA7\x7F?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\x02T\x90V[\x90P\x80\x82\x10\x15a\x18\xCDW`@Qc.W\xFF\x83`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x06,V[_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xE6Wa\x18\xE6a\x1F\xACV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x0FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x84\x81\x10\x15a\x19\xE6W_a\x19A\x89\x89\x84\x81Q\x81\x10a\x194Wa\x194a$\xA6V[` \x02` \x01\x01Qa\x1B\x1DV[\x90Pa\x19L\x81a\x05\x98V[a\x19tW`@Qc\xBF\x18\xAFC`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06,V[\x80\\a\x19\xBDW\x80\x84\x84\x81Q\x81\x10a\x19\x8DWa\x19\x8Da$\xA6V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x82a\x19\xAF\x81a'.V[\x93PPa\x19\xBD\x81`\x01a\x1B1V[\x84\x83\x10a\x19\xDDWa\x19\xCE\x84\x84a\x1B8V[`\x01\x96PPPPPPPa\x16DV[P`\x01\x01a\x19\x15V[Pa\x19\xF1\x82\x82a\x1B8V[P_\x96\x95PPPPPPV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1A2W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06,V[_\x80Q` a'\xFA\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1A|\x91\x90a'\x13V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1A\xB4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1A\xB9V[``\x91P[P\x91P\x91Pa\x1A\xC9\x85\x83\x83a\x1BmV[\x95\x94PPPPPV[4\x15a\x11[W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16Da\x1A\xFDa\x1B\xC9V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80a\x1B)\x84\x84a\x1B\xD7V[\x94\x93PPPPV[\x80\x82]PPV[_[\x81\x81\x10\x15a\x03\xAFWa\x1Be\x83\x82\x81Q\x81\x10a\x1BWWa\x1BWa$\xA6V[` \x02` \x01\x01Q_a\x1B1V[`\x01\x01a\x1B:V[``\x82a\x1B\x82Wa\x1B}\x82a\x1B\xFFV[a\x10!V[\x81Q\x15\x80\x15a\x1B\x99WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1B\xC2W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x06,V[P\x80a\x10!V[_a\x1B\xD2a\x1C(V[\x90P\x90V[_\x80_\x80a\x1B\xE5\x86\x86a\x1C\xC6V[\x92P\x92P\x92Pa\x1B\xF5\x82\x82a\x1D\x0FV[P\x90\x94\x93PPPPV[\x80Q\x15a\x1C\x0FW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x80Q` a(\x1A\x839\x81Q\x91R\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa\x1C`a\x1D\xC7V[a\x1Cha\x1E/V[`\x04\x84\x01T`@\x80Q` \x81\x01\x95\x90\x95R\x84\x01\x92\x90\x92R``\x83\x01R`\x01`\xA0\x1B\x81\x04`\x01`\x01`@\x1B\x03\x16`\x80\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x91PP\x90V[_\x80_\x83Q`A\x03a\x1C\xFDW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa\x1C\xEF\x88\x82\x85\x85a\x1EqV[\x95P\x95P\x95PPPPa\x1D\x08V[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a\x1D\"Wa\x1D\"a'FV[\x03a\x1D+WPPV[`\x01\x82`\x03\x81\x11\x15a\x1D?Wa\x1D?a'FV[\x03a\x1D]W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a\x1DqWa\x1Dqa'FV[\x03a\x1D\x92W`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06,V[`\x03\x82`\x03\x81\x11\x15a\x1D\xA6Wa\x1D\xA6a'FV[\x03a\x03\xCFW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06,V[__\x80Q` a(\x1A\x839\x81Q\x91R\x81a\x1D\xDFa\x13&V[\x80Q\x90\x91P\x15a\x1D\xF7W\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15a\x1E\x06W\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[__\x80Q` a(\x1A\x839\x81Q\x91R\x81a\x1EGa\x13\xDDV[\x80Q\x90\x91P\x15a\x1E_W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15a\x1E\x06W\x93\x92PPPV[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15a\x1E\xAAWP_\x91P`\x03\x90P\x82a\x1F/V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1E\xFBW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1F&WP_\x92P`\x01\x91P\x82\x90Pa\x1F/V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[_[\x83\x81\x10\x15a\x1FSW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1F;V[PP_\x91\x01RV[_\x81Q\x80\x84Ra\x1Fr\x81` \x86\x01` \x86\x01a\x1F9V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x10!` \x83\x01\x84a\x1F[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12\x04W_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1F\xE8Wa\x1F\xE8a\x1F\xACV[`@R\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x1F\xFFW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a \x18Wa \x18a\x1F\xACV[a +`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1F\xC0V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a ?W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a lW_\x80\xFD[\x825a w\x81a\x1F\x98V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x91W_\x80\xFD[a \x9D\x85\x82\x86\x01a\x1F\xF0V[\x91PP\x92P\x92\x90PV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a \xBBW_\x80\xFD[\x855a \xC6\x81a\x1F\x98V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x16\x82\x14a \xE2W_\x80\xFD[\x90\x94P`@\x87\x015\x90\x80\x82\x11\x15a \xF7W_\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a!\nW_\x80\xFD[\x815\x81\x81\x11\x15a!\x18W_\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15a!,W_\x80\xFD[\x96\x99\x95\x98PP` \x01\x95``\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a!QW_\x80\xFD[\x815a\x10!\x81a\x1F\x98V[`\xFF`\xF8\x1B\x88\x16\x81R_` `\xE0` \x84\x01Ra!|`\xE0\x84\x01\x8Aa\x1F[V[\x83\x81\x03`@\x85\x01Ra!\x8E\x81\x8Aa\x1F[V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R` \x80\x88\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a!\xE1W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a!\xC5V[P\x90\x9C\x9BPPPPPPPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\"+W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\"\x06V[P\x94\x95\x94PPPPPV[` \x81R_a\x10!` \x83\x01\x84a!\xF3V[_` \x82\x84\x03\x12\x15a\"XW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\"pW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"\x86W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\"\x99W_\x80\xFD[\x815` \x82\x82\x11\x15a\"\xADWa\"\xADa\x1F\xACV[\x81`\x05\x1B\x92Pa\"\xBE\x81\x84\x01a\x1F\xC0V[\x82\x81R\x92\x84\x01\x81\x01\x92\x81\x81\x01\x90\x89\x85\x11\x15a\"\xD7W_\x80\xFD[\x94\x82\x01\x94[\x84\x86\x10\x15a#\x01W\x855\x93Pa\"\xF1\x84a\x1F\x98V[\x83\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\"\xDCV[\x99\x97\x90\x91\x015\x97PPPPPPPV[_\x80_\x83\x85\x03`\x80\x81\x12\x15a#$W_\x80\xFD[`@\x81\x12\x15a#1W_\x80\xFD[P`@Q`@\x81\x01`\x01`\x01`@\x1B\x03\x82\x82\x10\x81\x83\x11\x17\x15a#UWa#Ua\x1F\xACV[\x81`@R\x865\x91Pa#f\x82a\x1F\x98V[\x90\x82R` \x86\x015\x90a#x\x82a\x1F\x98V[\x81` \x84\x01R\x82\x95P`@\x87\x015\x94P``\x87\x015\x92P\x80\x83\x11\x15a#\x9BW_\x80\xFD[PPa#\xA9\x86\x82\x87\x01a\x1F\xF0V[\x91PP\x92P\x92P\x92V[_\x85Qa#\xC4\x81\x84` \x8A\x01a\x1F9V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa#\xE3\x81`\x02\x84\x01` \x8A\x01a\x1F9V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa$\x07\x81`\x03\x85\x01` \x8A\x01a\x1F9V[`\x03\x92\x01\x91\x82\x01R\x83Qa$\"\x81`\x04\x84\x01` \x88\x01a\x1F9V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a$@W_\x80\xFD[\x81Qa\x10!\x81a\x1F\x98V[_`@\x82\x01`@\x83R\x80\x85T\x80\x83R``\x85\x01\x91P\x86_R` \x92P` _ _[\x82\x81\x10\x15a$\x92W\x81T`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92`\x01\x91\x82\x01\x91\x01a$mV[PPP` \x93\x90\x93\x01\x93\x90\x93RP\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`@\x81R_a$\xE0`@\x83\x01\x85a!\xF3V[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x16DWa\x16Da$\xEFV[\x80\x82\x01\x80\x82\x11\x15a\x16DWa\x16Da$\xEFV[\x81\x81\x03\x81\x81\x11\x15a\x16DWa\x16Da$\xEFV[_` \x82\x84\x03\x12\x15a%PW_\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a%kW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a%\x89WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x80\x86``\x1B\x16\x83R\x80\x85``\x1B\x16`\x14\x84\x01RP\x82Qa%\xC5\x81`(\x85\x01` \x87\x01a\x1F9V[\x91\x90\x91\x01`(\x01\x94\x93PPPPV[`\x1F\x82\x11\x15a\x03\xAFW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a%\xF9WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a&\x18W_\x81U`\x01\x01a&\x05V[PPPPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&8Wa&8a\x1F\xACV[a&L\x81a&F\x84Ta%WV[\x84a%\xD4V[` \x80`\x1F\x83\x11`\x01\x81\x14a&\x7FW_\x84\x15a&hWP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua&\xD6V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a&\xADW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a&\x8EV[P\x85\x82\x10\x15a&\xCAW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a'\x07W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a&\xEBV[P\x92\x96\x95PPPPPPV[_\x82Qa'$\x81\x84` \x87\x01a\x1F9V[\x91\x90\x91\x01\x92\x91PPV[_`\x01\x82\x01a'?Wa'?a$\xEFV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD\xFE?}z\x96\xC8\xC7\x02N\x92\xD3z\xFC\xCF\xC9\xB8ws\xA3;\x9B\xC2.#\x13Kh>t\xA5\n\xCE\0CiphertextVerification(bytes32[] ctHandles,address userAddress,address contractAddress,uint256 contractChainId,bytes extraData)6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xE9\x10\x84_\xD8\x18\xF6\x11'\xC8O5\x86wd6\xA3}\xEA\xD36%\x05le\x16%7\xE376\0\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    #[derive(Clone)]
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
        reinitializeV3(reinitializeV3Call),
        #[allow(missing_docs)]
        setThreshold(setThresholdCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        verifyInput(verifyInputCall),
    }
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
            [186u8, 194u8, 43u8, 184u8],
            [218u8, 83u8, 196u8, 125u8],
            [230u8, 49u8, 125u8, 245u8],
            [231u8, 82u8, 53u8, 184u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(getVersion),
            ::core::stringify!(cleanTransientStorage),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(EIP712_INPUT_VERIFICATION_TYPE),
            ::core::stringify!(initializeFromEmptyProxy),
            ::core::stringify!(getHandleVersion),
            ::core::stringify!(isSigner),
            ::core::stringify!(eip712Domain),
            ::core::stringify!(EIP712_INPUT_VERIFICATION_TYPEHASH),
            ::core::stringify!(getCoprocessorSigners),
            ::core::stringify!(setThreshold),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(reinitializeV3),
            ::core::stringify!(defineNewContext),
            ::core::stringify!(verifyInput),
            ::core::stringify!(getThreshold),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <cleanTransientStorageCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <EIP712_INPUT_VERIFICATION_TYPECall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getHandleVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isSignerCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eip712DomainCall as alloy_sol_types::SolCall>::SIGNATURE,
            <EIP712_INPUT_VERIFICATION_TYPEHASHCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getCoprocessorSignersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <reinitializeV3Call as alloy_sol_types::SolCall>::SIGNATURE,
            <defineNewContextCall as alloy_sol_types::SolCall>::SIGNATURE,
            <verifyInputCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getThresholdCall as alloy_sol_types::SolCall>::SIGNATURE,
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
                Self::reinitializeV3(_) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::SELECTOR
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
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(InputVerifierCalls::reinitializeV3)
                    }
                    reinitializeV3
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
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<InputVerifierCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(InputVerifierCalls::reinitializeV3)
                    }
                    reinitializeV3
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
                Self::reinitializeV3(inner) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::reinitializeV3(inner) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive(Clone)]
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
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(InvalidInputHandle),
            ::core::stringify!(CoprocessorSignerNull),
            ::core::stringify!(SignersSetIsEmpty),
            ::core::stringify!(DeserializingInputProofFail),
            ::core::stringify!(NotHostOwner),
            ::core::stringify!(ThresholdIsAboveNumberOfSigners),
            ::core::stringify!(SignaturesVerificationFailed),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(InvalidHandleVersion),
            ::core::stringify!(InvalidIndex),
            ::core::stringify!(NotInitializingFromEmptyProxy),
            ::core::stringify!(InvalidChainId),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(ThresholdIsNull),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(CoprocessorAlreadySigner),
            ::core::stringify!(EmptyInputProof),
            ::core::stringify!(ZeroSignature),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(SignatureThresholdNotReached),
            ::core::stringify!(InvalidSigner),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(NotASigner),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <InvalidInputHandle as alloy_sol_types::SolError>::SIGNATURE,
            <CoprocessorSignerNull as alloy_sol_types::SolError>::SIGNATURE,
            <SignersSetIsEmpty as alloy_sol_types::SolError>::SIGNATURE,
            <DeserializingInputProofFail as alloy_sol_types::SolError>::SIGNATURE,
            <NotHostOwner as alloy_sol_types::SolError>::SIGNATURE,
            <ThresholdIsAboveNumberOfSigners as alloy_sol_types::SolError>::SIGNATURE,
            <SignaturesVerificationFailed as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidHandleVersion as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidIndex as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidChainId as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <ThresholdIsNull as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <CoprocessorAlreadySigner as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyInputProof as alloy_sol_types::SolError>::SIGNATURE,
            <ZeroSignature as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <SignatureThresholdNotReached as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidSigner as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <NotASigner as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
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
    #[derive(Clone)]
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
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(EIP712DomainChanged),
            ::core::stringify!(NewContextSet),
            ::core::stringify!(Upgraded),
            ::core::stringify!(Initialized),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <NewContextSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
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
        __provider: P,
    ) -> InputVerifierInstance<P, N> {
        InputVerifierInstance::<P, N>::new(address, __provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        __provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<InputVerifierInstance<P, N>>,
    > {
        InputVerifierInstance::<P, N>::deploy(__provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
        InputVerifierInstance::<P, N>::deploy_builder(__provider)
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
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > InputVerifierInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`InputVerifier`](self) contract instance.

See the [wrapper's documentation](`InputVerifierInstance`) for more details.*/
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
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            __provider: P,
        ) -> alloy_contract::Result<InputVerifierInstance<P, N>> {
            let call_builder = Self::deploy_builder(__provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(__provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                __provider,
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
        ///Creates a new call builder for the [`reinitializeV3`] function.
        pub fn reinitializeV3(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV3Call, N> {
            self.call_builder(&reinitializeV3Call)
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
