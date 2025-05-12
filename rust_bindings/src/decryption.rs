///Module containing a contract's types and functions.
/**

```solidity
library IDecryption {
    struct RequestValidity { uint256 startTimestamp; uint256 durationDays; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IDecryption {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct RequestValidity { uint256 startTimestamp; uint256 durationDays; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestValidity {
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub durationDays: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<RequestValidity> for UnderlyingRustTuple<'_> {
            fn from(value: RequestValidity) -> Self {
                (value.startTimestamp, value.durationDays)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestValidity {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    startTimestamp: tuple.0,
                    durationDays: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RequestValidity {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RequestValidity {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.durationDays),
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
        impl alloy_sol_types::SolType for RequestValidity {
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
        impl alloy_sol_types::SolStruct for RequestValidity {
            const NAME: &'static str = "RequestValidity";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RequestValidity(uint256 startTimestamp,uint256 durationDays)",
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
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.durationDays)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RequestValidity {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.durationDays,
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
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.durationDays,
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
    /**Creates a new wrapper around an on-chain [`IDecryption`](self) contract instance.

See the [wrapper's documentation](`IDecryptionInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDecryptionInstance<T, P, N> {
        IDecryptionInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IDecryption`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IDecryption`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDecryptionInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IDecryptionInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDecryptionInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IDecryption`](self) contract instance.

See the [wrapper's documentation](`IDecryptionInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
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
    impl<T, P: ::core::clone::Clone, N> IDecryptionInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDecryptionInstance<T, P, N> {
            IDecryptionInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IDecryption {
    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }
}

interface Decryption {
    type FheType is uint8;
    struct CtHandleContractPair {
        bytes32 ctHandle;
        address contractAddress;
    }
    struct DelegationAccounts {
        address delegatorAddress;
        address delegatedAddress;
    }
    struct SnsCiphertextMaterial {
        bytes32 ctHandle;
        uint256 keyId;
        bytes32 snsCiphertextDigest;
        address[] coprocessorTxSenderAddresses;
    }

    error AddressEmptyCode(address target);
    error ContractAddressesMaxLengthExceeded(uint8 maxLength, uint256 actualLength);
    error ContractNotInContractAddresses(address contractAddress, address[] contractAddresses);
    error DecryptionNotDone(uint256 decryptionId);
    error DelegatorAddressInContractAddresses(address delegatorAddress, address[] contractAddresses);
    error DifferentKeyIdsNotAllowed(SnsCiphertextMaterial firstSnsCtMaterial, SnsCiphertextMaterial invalidSnsCtMaterial);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyCtHandleContractPairs();
    error EmptyCtHandles();
    error FailedCall();
    error InvalidFHEType(uint8 fheTypeUint8);
    error InvalidInitialization();
    error InvalidNullDurationDays();
    error InvalidUserSignature(bytes signature);
    error KmsNodeAlreadySigned(uint256 decryptionId, address signer);
    error MaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);
    error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);
    error NotInitializing();
    error OwnableInvalidOwner(address owner);
    error OwnableUnauthorizedAccount(address account);
    error StartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnsupportedFHEType(FheType fheType);
    error UserAddressInContractAddresses(address userAddress, address[] contractAddresses);
    error UserDecryptionRequestExpired(uint256 currentTimestamp, IDecryption.RequestValidity requestValidity);

    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event OwnershipTransferStarted(address indexed previousOwner, address indexed newOwner);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event PublicDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials);
    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures);
    event Upgraded(address indexed implementation);
    event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey);
    event UserDecryptionResponse(uint256 indexed decryptionId, bytes[] userDecryptedShares, bytes[] signatures);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function acceptOwnership() external;
    function checkDecryptionDone(uint256 decryptionId) external view;
    function checkDelegatedUserDecryptionReady(uint256 contractsChainId, DelegationAccounts memory delegationAccounts, CtHandleContractPair[] memory ctHandleContractPairs, address[] memory contractAddresses) external view;
    function checkPublicDecryptionReady(bytes32[] memory ctHandles) external view;
    function checkUserDecryptionReady(address userAddress, CtHandleContractPair[] memory ctHandleContractPairs) external view;
    function delegatedUserDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, DelegationAccounts memory delegationAccounts, uint256 contractsChainId, address[] memory contractAddresses, bytes memory publicKey, bytes memory signature) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getVersion() external pure returns (string memory);
    function initialize() external;
    function owner() external view returns (address);
    function pendingOwner() external view returns (address);
    function proxiableUUID() external view returns (bytes32);
    function publicDecryptionRequest(bytes32[] memory ctHandles) external;
    function publicDecryptionResponse(uint256 decryptionId, bytes memory decryptedResult, bytes memory signature) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function userDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, uint256 contractsChainId, address[] memory contractAddresses, address userAddress, bytes memory publicKey, bytes memory signature) external;
    function userDecryptionResponse(uint256 decryptionId, bytes memory userDecryptedShare, bytes memory signature) external;
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
    "name": "checkDecryptionDone",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkDelegatedUserDecryptionReady",
    "inputs": [
      {
        "name": "contractsChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "delegationAccounts",
        "type": "tuple",
        "internalType": "struct DelegationAccounts",
        "components": [
          {
            "name": "delegatorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkPublicDecryptionReady",
    "inputs": [
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkUserDecryptionReady",
    "inputs": [
      {
        "name": "userAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegatedUserDecryptionRequest",
    "inputs": [
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValidity",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationDays",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "delegationAccounts",
        "type": "tuple",
        "internalType": "struct DelegationAccounts",
        "components": [
          {
            "name": "delegatorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegatedAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "contractsChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "publicKey",
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
    "name": "initialize",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "publicDecryptionRequest",
    "inputs": [
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "publicDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "decryptedResult",
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
    "name": "userDecryptionRequest",
    "inputs": [
      {
        "name": "ctHandleContractPairs",
        "type": "tuple[]",
        "internalType": "struct CtHandleContractPair[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValidity",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationDays",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "contractsChainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "userAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "publicKey",
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
    "name": "userDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "userDecryptedShare",
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
    "name": "PublicDecryptionRequest",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "snsCtMaterials",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct SnsCiphertextMaterial[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PublicDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "decryptedResult",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "signatures",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
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
    "type": "event",
    "name": "UserDecryptionRequest",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "snsCtMaterials",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct SnsCiphertextMaterial[]",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      },
      {
        "name": "userAddress",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "publicKey",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserDecryptionResponse",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "userDecryptedShares",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
      },
      {
        "name": "signatures",
        "type": "bytes[]",
        "indexed": false,
        "internalType": "bytes[]"
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
    "name": "ContractAddressesMaxLengthExceeded",
    "inputs": [
      {
        "name": "maxLength",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "actualLength",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ContractNotInContractAddresses",
    "inputs": [
      {
        "name": "contractAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ]
  },
  {
    "type": "error",
    "name": "DecryptionNotDone",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "DelegatorAddressInContractAddresses",
    "inputs": [
      {
        "name": "delegatorAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ]
  },
  {
    "type": "error",
    "name": "DifferentKeyIdsNotAllowed",
    "inputs": [
      {
        "name": "firstSnsCtMaterial",
        "type": "tuple",
        "internalType": "struct SnsCiphertextMaterial",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
      },
      {
        "name": "invalidSnsCtMaterial",
        "type": "tuple",
        "internalType": "struct SnsCiphertextMaterial",
        "components": [
          {
            "name": "ctHandle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "snsCiphertextDigest",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "coprocessorTxSenderAddresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
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
    "name": "EmptyCtHandleContractPairs",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyCtHandles",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidFHEType",
    "inputs": [
      {
        "name": "fheTypeUint8",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "InvalidNullDurationDays",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidUserSignature",
    "inputs": [
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsNodeAlreadySigned",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "signer",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "MaxDecryptionRequestBitSizeExceeded",
    "inputs": [
      {
        "name": "maxBitSize",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "totalBitSize",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MaxDurationDaysExceeded",
    "inputs": [
      {
        "name": "maxValue",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "actualValue",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "StartTimestampInFuture",
    "inputs": [
      {
        "name": "currentTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "startTimestamp",
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
    "name": "UnsupportedFHEType",
    "inputs": [
      {
        "name": "fheType",
        "type": "uint8",
        "internalType": "enum FheType"
      }
    ]
  },
  {
    "type": "error",
    "name": "UserAddressInContractAddresses",
    "inputs": [
      {
        "name": "userAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "contractAddresses",
        "type": "address[]",
        "internalType": "address[]"
      }
    ]
  },
  {
    "type": "error",
    "name": "UserDecryptionRequestExpired",
    "inputs": [
      {
        "name": "currentTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValidity",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationDays",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
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
pub mod Decryption {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff16815250348015610042575f5ffd5b5061005161005660201b60201c565b6101b6565b5f61006561015460201b60201c565b9050805f0160089054906101000a900460ff16156100af576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146101515767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051610148919061019d565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b6101978161017b565b82525050565b5f6020820190506101b05f83018461018e565b92915050565b6080516169116101dc5f395f81816125ea0152818161263f01526127f901526169115ff3fe60806040526004361061011d575f3560e01c80638316001f1161009f578063ad3cb1cc11610063578063ad3cb1cc14610345578063b9bfe0a81461036f578063e30c397814610397578063f11d0638146103c1578063f2fde38b146103e95761011d565b80638316001f1461027357806384b0196e1461029b5780638da5cb5b146102cb578063a6090439146102f5578063aa39a3561461031d5761011d565b806352d1902d116100e657806352d1902d146101df578063715018a614610209578063760a04191461021f57806379ba5097146102475780638129fc1c1461025d5761011d565b80628bc3e11461012157806302fd1a64146101495780630d8e6e2c14610171578063187fe5291461019b5780634f1ef286146101c3575b5f5ffd5b34801561012c575f5ffd5b5061014760048036038101906101429190614399565b610411565b005b348015610154575f5ffd5b5061016f600480360381019061016a919061447e565b61061e565b005b34801561017c575f5ffd5b50610185610881565b604051610192919061457f565b60405180910390f35b3480156101a6575f5ffd5b506101c160048036038101906101bc91906145f4565b6108fc565b005b6101dd60048036038101906101d89190614767565b610aaa565b005b3480156101ea575f5ffd5b506101f3610ac9565b60405161020091906147d9565b60405180910390f35b348015610214575f5ffd5b5061021d610afa565b005b34801561022a575f5ffd5b5061024560048036038101906102409190614887565b610b0d565b005b348015610252575f5ffd5b5061025b610fce565b005b348015610268575f5ffd5b5061027161105c565b005b34801561027e575f5ffd5b50610299600480360381019061029491906149a6565b611205565b005b3480156102a6575f5ffd5b506102af6115c3565b6040516102c29796959493929190614bd3565b60405180910390f35b3480156102d6575f5ffd5b506102df6116cc565b6040516102ec9190614c55565b60405180910390f35b348015610300575f5ffd5b5061031b60048036038101906103169190614c6e565b611701565b005b348015610328575f5ffd5b50610343600480360381019061033e91906145f4565b611771565b005b348015610350575f5ffd5b506103596118b7565b604051610366919061457f565b60405180910390f35b34801561037a575f5ffd5b506103956004803603810190610390919061447e565b6118f0565b005b3480156103a2575f5ffd5b506103ab611c55565b6040516103b89190614c55565b60405180910390f35b3480156103cc575f5ffd5b506103e760048036038101906103e29190614c99565b611c8a565b005b3480156103f4575f5ffd5b5061040f600480360381019061040a9190614d3c565b611f2a565b005b5f5f90505b828290508110156106185773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84848481811061046457610463614d67565b5b9050604002015f0135866040518363ffffffff1660e01b815260040161048b929190614d94565b5f6040518083038186803b1580156104a1575f5ffd5b505afa1580156104b3573d5f5f3e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8484848181106104fa576104f9614d67565b5b9050604002015f013585858581811061051657610515614d67565b5b905060400201602001602081019061052e9190614d3c565b6040518363ffffffff1660e01b815260040161054b929190614d94565b5f6040518083038186803b158015610561575f5ffd5b505afa158015610573573d5f5f3e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f638484848181106105ba576105b9614d67565b5b9050604002015f01356040518263ffffffff1660e01b81526004016105df91906147d9565b5f6040518083038186803b1580156105f5575f5ffd5b505afa158015610607573d5f5f3e3d5ffd5b505050508080600101915050610416565b50505050565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b815260040161066b9190614c55565b5f6040518083038186803b158015610681575f5ffd5b505afa158015610693573d5f5f3e3d5ffd5b505050505f6106a0611fe3565b90505f6040518060400160405280836004015f8a81526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561070957602002820191905f5260205f20905b8154815260200190600101908083116106f5575b5050505050815260200187878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6107668261200a565b905061077488828787612098565b5f836003015f8a81526020019081526020015f205f8381526020019081526020015f20905080868690918060018154018082558091505060019003905f5260205f20015f9091929091929091929091925091826107d2929190614fc2565b50836001015f8a81526020019081526020015f205f9054906101000a900460ff1615801561080957506108088180549050612279565b5b15610876576001846001015f8b81526020019081526020015f205f6101000a81548160ff021916908315150217905550887f61568d6eb48e62870afffd55499206a54a8f78b04a627e00ed097161fc05d6be89898460405161086d93929190615219565b60405180910390a25b505050505050505050565b60606040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506108c25f61230a565b6108cc600161230a565b6108d55f61230a565b6040516020016108e8949392919061531e565b604051602081830303815290604052905090565b5f8282905003610938576040517f2de7543800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6109818282808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050506123d4565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f897184846040518363ffffffff1660e01b81526004016109d19291906153f4565b5f60405180830381865afa1580156109eb573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190610a13919061569f565b9050610a1e81612502565b5f610a27611fe3565b9050805f015f815480929190610a3c90615713565b91905055505f815f015490508484836004015f8481526020019081526020015f209190610a6a92919061421c565b50807f17c632196fbf6b96d9675971058d3701733094c3f2f1dcb9ba7d2a08bee0aafb84604051610a9b919061593b565b60405180910390a25050505050565b610ab26125e8565b610abb826126ce565b610ac582826126d9565b5050565b5f610ad26127f7565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b610b0261287e565b610b0b5f612905565b565b600a60ff16868690501115610b5f57600a868690506040517fc5ab467e000000000000000000000000000000000000000000000000000000008152600401610b56929190615976565b60405180910390fd5b610b7889803603810190610b7391906159ea565b612942565b610bd38686808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050895f016020810190610bce9190614d3c565b612a8d565b15610c2a57875f016020810190610bea9190614d3c565b86866040517fc3446ac7000000000000000000000000000000000000000000000000000000008152600401610c2193929190615aab565b60405180910390fd5b5f610c888c8c8989808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508c5f016020810190610c839190614d3c565b612b0b565b905073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e898b8a8a6040518563ffffffff1660e01b8152600401610cdd9493929190615b18565b5f6040518083038186803b158015610cf3575f5ffd5b505afa158015610d05573d5f5f3e3d5ffd5b505050505f6040518060c0016040528087878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018989808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b5f016020810190610db69190614d3c565b73ffffffffffffffffffffffffffffffffffffffff1681526020018a81526020018c5f013581526020018c602001358152509050610e08818b6020016020810190610e019190614d3c565b8686612de6565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b8152600401610e569190615bee565b5f60405180830381865afa158015610e70573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190610e98919061569f565b9050610ea381612502565b5f610eac611fe3565b9050805f015f815480929190610ec190615713565b91905055505f815f0154905060405180604001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f019081610f4b9190615c18565b506020820151816001019080519060200190610f68929190614267565b50905050807f1c3dcad6311be6d58dc4d4b9f1bc1625eb18d72de969db75e11a88ef3527d2f3848f6020016020810190610fa29190614d3c565b8c8c604051610fb49493929190615ce7565b60405180910390a250505050505050505050505050505050565b5f610fd7612ebc565b90508073ffffffffffffffffffffffffffffffffffffffff16610ff8611c55565b73ffffffffffffffffffffffffffffffffffffffff161461105057806040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016110479190614c55565b60405180910390fd5b61105981612905565b50565b60025f611067612ec3565b9050805f0160089054906101000a900460ff16806110af57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156110e6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061119f6040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250612eea565b6111af6111aa6116cc565b612f00565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516111f99190615d4e565b60405180910390a15050565b600a60ff1687879050111561125757600a878790506040517fc5ab467e00000000000000000000000000000000000000000000000000000000815260040161124e929190615976565b60405180910390fd5b6112708980360381019061126b91906159ea565b612942565b6112ba8787808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505086612a8d565b15611300578487876040517fdc4d78b10000000000000000000000000000000000000000000000000000000081526004016112f793929190615aab565b60405180910390fd5b5f61134d8c8c8a8a808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505089612b0b565b90505f6040518060a0016040528087878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018a8a808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b81526020018c5f013581526020018c60200135815250905061140f81888686612f14565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b815260040161145d9190615bee565b5f60405180830381865afa158015611477573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061149f919061569f565b90506114aa81612502565b5f6114b3611fe3565b9050805f015f8154809291906114c890615713565b91905055505f815f0154905060405180604001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f0190816115529190615c18565b50602082015181600101908051906020019061156f929190614267565b50905050807f1c3dcad6311be6d58dc4d4b9f1bc1625eb18d72de969db75e11a88ef3527d2f3848c8c8c6040516115a99493929190615ce7565b60405180910390a250505050505050505050505050505050565b5f6060805f5f5f60605f6115d5612fea565b90505f5f1b815f01541480156115f057505f5f1b8160010154145b61162f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161162690615db1565b60405180910390fd5b611637613011565b61163f6130af565b46305f5f1b5f67ffffffffffffffff81111561165e5761165d614643565b5b60405190808252806020026020018201604052801561168c5781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b5f5f6116d661314d565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b5f61170a611fe3565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff1661176d57816040517f0bf014060000000000000000000000000000000000000000000000000000000081526004016117649190615dcf565b60405180910390fd5b5050565b5f5f90505b828290508110156118b25773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c8484848181106117c4576117c3614d67565b5b905060200201356040518263ffffffff1660e01b81526004016117e791906147d9565b5f6040518083038186803b1580156117fd575f5ffd5b505afa15801561180f573d5f5f3e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f6384848481811061185657611855614d67565b5b905060200201356040518263ffffffff1660e01b815260040161187991906147d9565b5f6040518083038186803b15801561188f575f5ffd5b505afa1580156118a1573d5f5f3e3d5ffd5b505050508080600101915050611776565b505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b815260040161193d9190614c55565b5f6040518083038186803b158015611953575f5ffd5b505afa158015611965573d5f5f3e3d5ffd5b505050505f611972611fe3565b90505f816006015f8881526020019081526020015f206040518060400160405290815f820180546119a290614df2565b80601f01602080910402602001604051908101604052809291908181526020018280546119ce90614df2565b8015611a195780601f106119f057610100808354040283529160200191611a19565b820191905f5260205f20905b8154815290600101906020018083116119fc57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015611a6f57602002820191905f5260205f20905b815481526020019060010190808311611a5b575b50505050508152505090505f6040518060600160405280835f015181526020018360200151815260200188888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f611aec82613174565b9050611afa89828888612098565b5f846005015f8b81526020019081526020015f20905080878790918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182611b49929190614fc2565b50846008015f8b81526020019081526020015f20898990918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182611b95929190614fc2565b50846001015f8b81526020019081526020015f205f9054906101000a900460ff16158015611bcc5750611bcb818054905061320f565b5b15611c49576001856001015f8c81526020019081526020015f205f6101000a81548160ff021916908315150217905550897f7312dec4cead0d5d3da836cdbaed1eb6a81e218c519c8740da4ac75afcb6c5c7866008015f8d81526020019081526020015f2083604051611c40929190615e82565b60405180910390a25b50505050505050505050565b5f5f611c5f6132a0565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e878785856040518563ffffffff1660e01b8152600401611cdd9493929190615b18565b5f6040518083038186803b158015611cf3575f5ffd5b505afa158015611d05573d5f5f3e3d5ffd5b505050505f5f90505b84849050811015611f215773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d868684818110611d5c57611d5b614d67565b5b9050604002015f0135885f016020810190611d779190614d3c565b6040518363ffffffff1660e01b8152600401611d94929190614d94565b5f6040518083038186803b158015611daa575f5ffd5b505afa158015611dbc573d5f5f3e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d868684818110611e0357611e02614d67565b5b9050604002015f0135878785818110611e1f57611e1e614d67565b5b9050604002016020016020810190611e379190614d3c565b6040518363ffffffff1660e01b8152600401611e54929190614d94565b5f6040518083038186803b158015611e6a575f5ffd5b505afa158015611e7c573d5f5f3e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f63868684818110611ec357611ec2614d67565b5b9050604002015f01356040518263ffffffff1660e01b8152600401611ee891906147d9565b5f6040518083038186803b158015611efe575f5ffd5b505afa158015611f10573d5f5f3e3d5ffd5b505050508080600101915050611d0e565b50505050505050565b611f3261287e565b5f611f3b6132a0565b905081815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff16611f9d6116cc565b73ffffffffffffffffffffffffffffffffffffffff167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a35050565b5f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700905090565b5f6120916040518060800160405280604481526020016167e06044913980519060200120835f01516040516020016120429190615f43565b6040516020818303038152906040528051906020012084602001518051906020012060405160200161207693929190615f59565b604051602081830303815290604052805190602001206132c7565b9050919050565b5f6120a1611fe3565b90505f6120f18585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506132e0565b905073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16636c88eb43826040518263ffffffff1660e01b81526004016121409190614c55565b5f6040518083038186803b158015612156575f5ffd5b505afa158015612168573d5f5f3e3d5ffd5b50505050816002015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561220b5785816040517f99ec48d9000000000000000000000000000000000000000000000000000000008152600401612202929190615f8e565b60405180910390fd5b6001826002015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050505050565b5f5f73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff166347cd4b3e6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122d8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122fc9190615fb5565b905080831015915050919050565b60605f60016123188461330a565b0190505f8167ffffffffffffffff81111561233657612335614643565b5b6040519080825280601f01601f1916602001820160405280156123685781602001600182028036833780820191505090505b5090505f82602001820190505b6001156123c9578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816123be576123bd615fe0565b5b0494505f8503612375575b819350505050919050565b5f5f90505f5f90505b82518110156124b2575f8382815181106123fa576123f9614d67565b5b602002602001015190505f61240e8261345b565b9050612419816134e5565b61ffff1684612428919061600d565b935073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c836040518263ffffffff1660e01b815260040161247791906147d9565b5f6040518083038186803b15801561248d575f5ffd5b505afa15801561249f573d5f5f3e3d5ffd5b50505050505080806001019150506123dd565b506108008111156124fe57610800816040517fe7f4895d0000000000000000000000000000000000000000000000000000000081526004016124f5929190616040565b60405180910390fd5b5050565b6001815111156125e5575f815f815181106125205761251f614d67565b5b60200260200101516020015190505f600190505b82518110156125e2578183828151811061255157612550614d67565b5b602002602001015160200151146125d557825f8151811061257557612574614d67565b5b60200260200101518382815181106125905761258f614d67565b5b60200260200101516040517fcfae921f0000000000000000000000000000000000000000000000000000000081526004016125cc9291906160c7565b60405180910390fd5b8080600101915050612534565b50505b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061269557507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1661267c613772565b73ffffffffffffffffffffffffffffffffffffffff1614155b156126cc576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6126d661287e565b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561274157506040513d601f19601f8201168201806040525081019061273e91906160fc565b60015b61278257816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016127799190614c55565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146127e857806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016127df91906147d9565b60405180910390fd5b6127f283836137c5565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461287c576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b612886612ebc565b73ffffffffffffffffffffffffffffffffffffffff166128a46116cc565b73ffffffffffffffffffffffffffffffffffffffff1614612903576128c7612ebc565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016128fa9190614c55565b60405180910390fd5b565b5f61290e6132a0565b9050805f015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905561293e82613837565b5050565b5f81602001510361297f576040517fde2859c100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61016d61ffff16816020015111156129d65761016d81602001516040517f329518630000000000000000000000000000000000000000000000000000000081526004016129cd929190616164565b60405180910390fd5b42815f01511115612a235742815f01516040517ff24c0887000000000000000000000000000000000000000000000000000000008152600401612a1a929190616040565b60405180910390fd5b42620151808260200151612a37919061618b565b825f0151612a45919061600d565b1015612a8a5742816040517f30348040000000000000000000000000000000000000000000000000000000008152600401612a819291906161f9565b60405180910390fd5b50565b5f5f5f90505b8351811015612b00578273ffffffffffffffffffffffffffffffffffffffff16848281518110612ac657612ac5614d67565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1603612af3576001915050612b05565b8080600101915050612a93565b505f90505b92915050565b60605f8585905003612b49576040517fa6a6cb2100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8484905067ffffffffffffffff811115612b6657612b65614643565b5b604051908082528060200260200182016040528015612b945781602001602082028036833780820191505090505b5090505f5f90505f5f90505b86869050811015612d91575f878783818110612bbf57612bbe614d67565b5b9050604002015f013590505f888884818110612bde57612bdd614d67565b5b9050604002016020016020810190612bf69190614d3c565b90505f612c028361345b565b9050612c0d816134e5565b61ffff1685612c1c919061600d565b945073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84896040518363ffffffff1660e01b8152600401612c6d929190614d94565b5f6040518083038186803b158015612c83575f5ffd5b505afa158015612c95573d5f5f3e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84846040518363ffffffff1660e01b8152600401612ce8929190614d94565b5f6040518083038186803b158015612cfe575f5ffd5b505afa158015612d10573d5f5f3e3d5ffd5b50505050612d1e8883612a8d565b612d615781886040517fa4c30391000000000000000000000000000000000000000000000000000000008152600401612d5892919061627c565b60405180910390fd5b82868581518110612d7557612d74614d67565b5b6020026020010181815250505050508080600101915050612ba0565b50610800811115612ddd57610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401612dd4929190616040565b60405180910390fd5b50949350505050565b5f612df085613908565b90505f612e408285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506132e0565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614612eb45783836040517f2a873d27000000000000000000000000000000000000000000000000000000008152600401612eab9291906162aa565b60405180910390fd5b505050505050565b5f33905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b612ef26139ae565b612efc82826139ee565b5050565b612f086139ae565b612f1181613a3f565b50565b5f612f1e85613ac3565b90505f612f6e8285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506132e0565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614612fe25783836040517f2a873d27000000000000000000000000000000000000000000000000000000008152600401612fd99291906162aa565b60405180910390fd5b505050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f61301c612fea565b905080600201805461302d90614df2565b80601f016020809104026020016040519081016040528092919081815260200182805461305990614df2565b80156130a45780601f1061307b576101008083540402835291602001916130a4565b820191905f5260205f20905b81548152906001019060200180831161308757829003601f168201915b505050505091505090565b60605f6130ba612fea565b90508060030180546130cb90614df2565b80601f01602080910402602001604051908101604052809291908181526020018280546130f790614df2565b80156131425780601f1061311957610100808354040283529160200191613142565b820191905f5260205f20905b81548152906001019060200180831161312557829003601f168201915b505050505091505090565b5f7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300905090565b5f6132086040518060800160405280605d81526020016168b4605d913980519060200120835f01518051906020012084602001516040516020016131b89190615f43565b604051602081830303815290604052805190602001208560400151805190602001206040516020016131ed94939291906162cc565b604051602081830303815290604052805190602001206132c7565b9050919050565b5f5f73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663490413aa6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561326e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132929190615fb5565b905080831015915050919050565b5f7f237e158222e3e6968b72b9db0d8043aacf074ad9f650f0d1606b4d82ee432c00905090565b5f6132d96132d3613b63565b83613b71565b9050919050565b5f5f5f5f6132ee8686613bb1565b9250925092506132fe8282613c06565b82935050505092915050565b5f5f5f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310613366577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161335c5761335b615fe0565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106133a3576d04ee2d6d415b85acef8100000000838161339957613398615fe0565b5b0492506020810190505b662386f26fc1000083106133d257662386f26fc1000083816133c8576133c7615fe0565b5b0492506010810190505b6305f5e10083106133fb576305f5e10083816133f1576133f0615fe0565b5b0492506008810190505b612710831061342057612710838161341657613415615fe0565b5b0492506004810190505b60648310613443576064838161343957613438615fe0565b5b0492506002810190505b600a8310613452576001810190505b80915050919050565b5f5f60f860f084901b901c5f1c905060538081111561347d5761347c61630f565b5b60ff168160ff1611156134c757806040517f641950d70000000000000000000000000000000000000000000000000000000081526004016134be919061633c565b60405180910390fd5b8060ff1660538111156134dd576134dc61630f565b5b915050919050565b5f5f60538111156134f9576134f861630f565b5b82605381111561350c5761350b61630f565b5b0361351a576002905061376d565b6002605381111561352e5761352d61630f565b5b8260538111156135415761354061630f565b5b0361354f576008905061376d565b600360538111156135635761356261630f565b5b8260538111156135765761357561630f565b5b03613584576010905061376d565b600460538111156135985761359761630f565b5b8260538111156135ab576135aa61630f565b5b036135b9576020905061376d565b600560538111156135cd576135cc61630f565b5b8260538111156135e0576135df61630f565b5b036135ee576040905061376d565b600660538111156136025761360161630f565b5b8260538111156136155761361461630f565b5b03613623576080905061376d565b600760538111156136375761363661630f565b5b82605381111561364a5761364961630f565b5b036136585760a0905061376d565b6008605381111561366c5761366b61630f565b5b82605381111561367f5761367e61630f565b5b0361368e57610100905061376d565b600960538111156136a2576136a161630f565b5b8260538111156136b5576136b461630f565b5b036136c457610200905061376d565b600a60538111156136d8576136d761630f565b5b8260538111156136eb576136ea61630f565b5b036136fa57610400905061376d565b600b605381111561370e5761370d61630f565b5b8260538111156137215761372061630f565b5b0361373057610800905061376d565b816040517fbe7830b1000000000000000000000000000000000000000000000000000000008152600401613764919061639b565b60405180910390fd5b919050565b5f61379e7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613d68565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b6137ce82613d71565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f8151111561382a576138248282613e3a565b50613833565b613832613eba565b5b5050565b5f61384061314d565b90505f815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905082825f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a3505050565b5f6139a76040518060e0016040528060b2815260200161672e60b2913980519060200120835f015180519060200120846020015160405160200161394c9190616440565b604051602081830303815290604052805190602001208560400151866060015187608001518860a0015160405160200161398c9796959493929190616456565b604051602081830303815290604052805190602001206132c7565b9050919050565b6139b6613ef6565b6139ec576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6139f66139ae565b5f6139ff612fea565b905082816002019081613a12919061651b565b5081816003019081613a24919061651b565b505f5f1b815f01819055505f5f1b8160010181905550505050565b613a476139ae565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613ab7575f6040517f1e4fbdf7000000000000000000000000000000000000000000000000000000008152600401613aae9190614c55565b60405180910390fd5b613ac081612905565b50565b5f613b5c6040518060c00160405280609081526020016168246090913980519060200120835f0151805190602001208460200151604051602001613b079190616440565b60405160208183030381529060405280519060200120856040015186606001518760800151604051602001613b41969594939291906165ea565b604051602081830303815290604052805190602001206132c7565b9050919050565b5f613b6c613f14565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f5f5f6041845103613bf1575f5f5f602087015192506040870151915060608701515f1a9050613be388828585613f77565b955095509550505050613bff565b5f600285515f1b9250925092505b9250925092565b5f6003811115613c1957613c1861630f565b5b826003811115613c2c57613c2b61630f565b5b0315613d645760016003811115613c4657613c4561630f565b5b826003811115613c5957613c5861630f565b5b03613c90576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115613ca457613ca361630f565b5b826003811115613cb757613cb661630f565b5b03613cfb57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401613cf29190615dcf565b60405180910390fd5b600380811115613d0e57613d0d61630f565b5b826003811115613d2157613d2061630f565b5b03613d6357806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401613d5a91906147d9565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03613dcc57806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613dc39190614c55565b60405180910390fd5b80613df87f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613d68565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f5f8473ffffffffffffffffffffffffffffffffffffffff1684604051613e639190616683565b5f60405180830381855af49150503d805f8114613e9b576040519150601f19603f3d011682016040523d82523d5f602084013e613ea0565b606091505b5091509150613eb085838361405e565b9250505092915050565b5f341115613ef4576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f613eff612ec3565b5f0160089054906101000a900460ff16905090565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f613f3e6140eb565b613f46614161565b4630604051602001613f5c959493929190616699565b60405160208183030381529060405280519060200120905090565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613fb3575f600385925092509250614054565b5f6001888888886040515f8152602001604052604051613fd694939291906166ea565b6020604051602081039080840390855afa158015613ff6573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614047575f60015f5f1b93509350935050614054565b805f5f5f1b935093509350505b9450945094915050565b6060826140735761406e826141d8565b6140e3565b5f825114801561409957505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156140db57836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016140d29190614c55565b60405180910390fd5b8190506140e4565b5b9392505050565b5f5f6140f5612fea565b90505f614100613011565b90505f8151111561411c5780805190602001209250505061415e565b5f825f015490505f5f1b81146141375780935050505061415e565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f5f61416b612fea565b90505f6141766130af565b90505f81511115614192578080519060200120925050506141d5565b5f826001015490505f5f1b81146141ae578093505050506141d5565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156141ea5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f20908101928215614256579160200282015b8281111561425557823582559160200191906001019061423a565b5b50905061426391906142b2565b5090565b828054828255905f5260205f209081019282156142a1579160200282015b828111156142a0578251825591602001919060010190614285565b5b5090506142ae91906142b2565b5090565b5b808211156142c9575f815f9055506001016142b3565b5090565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f614307826142de565b9050919050565b614317816142fd565b8114614321575f5ffd5b50565b5f813590506143328161430e565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261435957614358614338565b5b8235905067ffffffffffffffff8111156143765761437561433c565b5b60208301915083604082028301111561439257614391614340565b5b9250929050565b5f5f5f604084860312156143b0576143af6142d6565b5b5f6143bd86828701614324565b935050602084013567ffffffffffffffff8111156143de576143dd6142da565b5b6143ea86828701614344565b92509250509250925092565b5f819050919050565b614408816143f6565b8114614412575f5ffd5b50565b5f81359050614423816143ff565b92915050565b5f5f83601f84011261443e5761443d614338565b5b8235905067ffffffffffffffff81111561445b5761445a61433c565b5b60208301915083600182028301111561447757614476614340565b5b9250929050565b5f5f5f5f5f60608688031215614497576144966142d6565b5b5f6144a488828901614415565b955050602086013567ffffffffffffffff8111156144c5576144c46142da565b5b6144d188828901614429565b9450945050604086013567ffffffffffffffff8111156144f4576144f36142da565b5b61450088828901614429565b92509250509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f6145518261450f565b61455b8185614519565b935061456b818560208601614529565b61457481614537565b840191505092915050565b5f6020820190508181035f8301526145978184614547565b905092915050565b5f5f83601f8401126145b4576145b3614338565b5b8235905067ffffffffffffffff8111156145d1576145d061433c565b5b6020830191508360208202830111156145ed576145ec614340565b5b9250929050565b5f5f6020838503121561460a576146096142d6565b5b5f83013567ffffffffffffffff811115614627576146266142da565b5b6146338582860161459f565b92509250509250929050565b5f5ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61467982614537565b810181811067ffffffffffffffff8211171561469857614697614643565b5b80604052505050565b5f6146aa6142cd565b90506146b68282614670565b919050565b5f67ffffffffffffffff8211156146d5576146d4614643565b5b6146de82614537565b9050602081019050919050565b828183375f83830152505050565b5f61470b614706846146bb565b6146a1565b9050828152602081018484840111156147275761472661463f565b5b6147328482856146eb565b509392505050565b5f82601f83011261474e5761474d614338565b5b813561475e8482602086016146f9565b91505092915050565b5f5f6040838503121561477d5761477c6142d6565b5b5f61478a85828601614324565b925050602083013567ffffffffffffffff8111156147ab576147aa6142da565b5b6147b78582860161473a565b9150509250929050565b5f819050919050565b6147d3816147c1565b82525050565b5f6020820190506147ec5f8301846147ca565b92915050565b5f5ffd5b5f6040828403121561480b5761480a6147f2565b5b81905092915050565b5f60408284031215614829576148286147f2565b5b81905092915050565b5f5f83601f84011261484757614846614338565b5b8235905067ffffffffffffffff8111156148645761486361433c565b5b6020830191508360208202830111156148805761487f614340565b5b9250929050565b5f5f5f5f5f5f5f5f5f5f5f6101208c8e0312156148a7576148a66142d6565b5b5f8c013567ffffffffffffffff8111156148c4576148c36142da565b5b6148d08e828f01614344565b9b509b505060206148e38e828f016147f6565b99505060606148f48e828f01614814565b98505060a06149058e828f01614415565b97505060c08c013567ffffffffffffffff811115614926576149256142da565b5b6149328e828f01614832565b965096505060e08c013567ffffffffffffffff811115614955576149546142da565b5b6149618e828f01614429565b94509450506101008c013567ffffffffffffffff811115614985576149846142da565b5b6149918e828f01614429565b92509250509295989b509295989b9093969950565b5f5f5f5f5f5f5f5f5f5f5f6101008c8e0312156149c6576149c56142d6565b5b5f8c013567ffffffffffffffff8111156149e3576149e26142da565b5b6149ef8e828f01614344565b9b509b50506020614a028e828f016147f6565b9950506060614a138e828f01614415565b98505060808c013567ffffffffffffffff811115614a3457614a336142da565b5b614a408e828f01614832565b975097505060a0614a538e828f01614324565b95505060c08c013567ffffffffffffffff811115614a7457614a736142da565b5b614a808e828f01614429565b945094505060e08c013567ffffffffffffffff811115614aa357614aa26142da565b5b614aaf8e828f01614429565b92509250509295989b509295989b9093969950565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b614af881614ac4565b82525050565b614b07816143f6565b82525050565b614b16816142fd565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b614b4e816143f6565b82525050565b5f614b5f8383614b45565b60208301905092915050565b5f602082019050919050565b5f614b8182614b1c565b614b8b8185614b26565b9350614b9683614b36565b805f5b83811015614bc6578151614bad8882614b54565b9750614bb883614b6b565b925050600181019050614b99565b5085935050505092915050565b5f60e082019050614be65f83018a614aef565b8181036020830152614bf88189614547565b90508181036040830152614c0c8188614547565b9050614c1b6060830187614afe565b614c286080830186614b0d565b614c3560a08301856147ca565b81810360c0830152614c478184614b77565b905098975050505050505050565b5f602082019050614c685f830184614b0d565b92915050565b5f60208284031215614c8357614c826142d6565b5b5f614c9084828501614415565b91505092915050565b5f5f5f5f5f5f60a08789031215614cb357614cb26142d6565b5b5f614cc089828a01614415565b9650506020614cd189828a01614814565b955050606087013567ffffffffffffffff811115614cf257614cf16142da565b5b614cfe89828a01614344565b9450945050608087013567ffffffffffffffff811115614d2157614d206142da565b5b614d2d89828a01614832565b92509250509295509295509295565b5f60208284031215614d5157614d506142d6565b5b5f614d5e84828501614324565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f604082019050614da75f8301856147ca565b614db46020830184614b0d565b9392505050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680614e0957607f821691505b602082108103614e1c57614e1b614dc5565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302614e7e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82614e43565b614e888683614e43565b95508019841693508086168417925050509392505050565b5f819050919050565b5f614ec3614ebe614eb9846143f6565b614ea0565b6143f6565b9050919050565b5f819050919050565b614edc83614ea9565b614ef0614ee882614eca565b848454614e4f565b825550505050565b5f5f905090565b614f07614ef8565b614f12818484614ed3565b505050565b5b81811015614f3557614f2a5f82614eff565b600181019050614f18565b5050565b601f821115614f7a57614f4b81614e22565b614f5484614e34565b81016020851015614f63578190505b614f77614f6f85614e34565b830182614f17565b50505b505050565b5f82821c905092915050565b5f614f9a5f1984600802614f7f565b1980831691505092915050565b5f614fb28383614f8b565b9150826002028217905092915050565b614fcc8383614dbb565b67ffffffffffffffff811115614fe557614fe4614643565b5b614fef8254614df2565b614ffa828285614f39565b5f601f831160018114615027575f8415615015578287013590505b61501f8582614fa7565b865550615086565b601f19841661503586614e22565b5f5b8281101561505c57848901358255600182019150602085019450602081019050615037565b868310156150795784890135615075601f891682614f8b565b8355505b6001600288020188555050505b50505050505050565b5f82825260208201905092915050565b5f6150aa838561508f565b93506150b78385846146eb565b6150c083614537565b840190509392505050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f82825260208201905092915050565b5f815461511381614df2565b61511d81866150f7565b9450600182165f8114615137576001811461514d5761517f565b60ff19831686528115156020028601935061517f565b61515685614e22565b5f5b8381101561517757815481890152600182019150602081019050615158565b808801955050505b50505092915050565b5f6151938383615107565b905092915050565b5f600182019050919050565b5f6151b1826150cb565b6151bb81856150d5565b9350836020820285016151cd856150e5565b805f5b85811015615207578484038952816151e88582615188565b94506151f38361519b565b925060208a019950506001810190506151d0565b50829750879550505050505092915050565b5f6040820190508181035f83015261523281858761509f565b9050818103602083015261524681846151a7565b9050949350505050565b5f81905092915050565b5f6152648261450f565b61526e8185615250565b935061527e818560208601614529565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6152be600283615250565b91506152c98261528a565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f615308600183615250565b9150615313826152d4565b600182019050919050565b5f615329828761525a565b9150615334826152b2565b9150615340828661525a565b915061534b826152fc565b9150615357828561525a565b9150615362826152fc565b915061536e828461525a565b915081905095945050505050565b5f82825260208201905092915050565b5f5ffd5b82818337505050565b5f6153a4838561537c565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8311156153d7576153d661538c565b5b6020830292506153e8838584615390565b82840190509392505050565b5f6020820190508181035f83015261540d818486615399565b90509392505050565b5f67ffffffffffffffff8211156154305761542f614643565b5b602082029050602081019050919050565b5f5ffd5b5f5ffd5b615452816147c1565b811461545c575f5ffd5b50565b5f8151905061546d81615449565b92915050565b5f81519050615481816143ff565b92915050565b5f67ffffffffffffffff8211156154a1576154a0614643565b5b602082029050602081019050919050565b5f815190506154c08161430e565b92915050565b5f6154d86154d384615487565b6146a1565b905080838252602082019050602084028301858111156154fb576154fa614340565b5b835b81811015615524578061551088826154b2565b8452602084019350506020810190506154fd565b5050509392505050565b5f82601f83011261554257615541614338565b5b81516155528482602086016154c6565b91505092915050565b5f608082840312156155705761556f615441565b5b61557a60806146a1565b90505f6155898482850161545f565b5f83015250602061559c84828501615473565b60208301525060406155b08482850161545f565b604083015250606082015167ffffffffffffffff8111156155d4576155d3615445565b5b6155e08482850161552e565b60608301525092915050565b5f6155fe6155f984615416565b6146a1565b9050808382526020820190506020840283018581111561562157615620614340565b5b835b8181101561566857805167ffffffffffffffff81111561564657615645614338565b5b808601615653898261555b565b85526020850194505050602081019050615623565b5050509392505050565b5f82601f83011261568657615685614338565b5b81516156968482602086016155ec565b91505092915050565b5f602082840312156156b4576156b36142d6565b5b5f82015167ffffffffffffffff8111156156d1576156d06142da565b5b6156dd84828501615672565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61571d826143f6565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361574f5761574e6156e6565b5b600182019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61578c816147c1565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6157c4816142fd565b82525050565b5f6157d583836157bb565b60208301905092915050565b5f602082019050919050565b5f6157f782615792565b615801818561579c565b935061580c836157ac565b805f5b8381101561583c57815161582388826157ca565b975061582e836157e1565b92505060018101905061580f565b5085935050505092915050565b5f608083015f83015161585e5f860182615783565b5060208301516158716020860182614b45565b5060408301516158846040860182615783565b506060830151848203606086015261589c82826157ed565b9150508091505092915050565b5f6158b48383615849565b905092915050565b5f602082019050919050565b5f6158d28261575a565b6158dc8185615764565b9350836020820285016158ee85615774565b805f5b85811015615929578484038952815161590a85826158a9565b9450615915836158bc565b925060208a019950506001810190506158f1565b50829750879550505050505092915050565b5f6020820190508181035f83015261595381846158c8565b905092915050565b5f60ff82169050919050565b6159708161595b565b82525050565b5f6040820190506159895f830185615967565b6159966020830184614afe565b9392505050565b5f604082840312156159b2576159b1615441565b5b6159bc60406146a1565b90505f6159cb84828501614415565b5f8301525060206159de84828501614415565b60208301525092915050565b5f604082840312156159ff576159fe6142d6565b5b5f615a0c8482850161599d565b91505092915050565b5f82825260208201905092915050565b5f819050919050565b5f615a3c6020840184614324565b905092915050565b5f602082019050919050565b5f615a5b8385615a15565b9350615a6682615a25565b805f5b85811015615a9e57615a7b8284615a2e565b615a8588826157ca565b9750615a9083615a44565b925050600181019050615a69565b5085925050509392505050565b5f604082019050615abe5f830186614b0d565b8181036020830152615ad1818486615a50565b9050949350505050565b60408201615aeb5f830183615a2e565b615af75f8501826157bb565b50615b056020830183615a2e565b615b1260208501826157bb565b50505050565b5f608082019050615b2b5f830187614afe565b615b386020830186615adb565b8181036060830152615b4b818486615a50565b905095945050505050565b5f81519050919050565b5f819050602082019050919050565b5f615b7a8383615783565b60208301905092915050565b5f602082019050919050565b5f615b9c82615b56565b615ba6818561537c565b9350615bb183615b60565b805f5b83811015615be1578151615bc88882615b6f565b9750615bd383615b86565b925050600181019050615bb4565b5085935050505092915050565b5f6020820190508181035f830152615c068184615b92565b905092915050565b5f81519050919050565b615c2182615c0e565b67ffffffffffffffff811115615c3a57615c39614643565b5b615c448254614df2565b615c4f828285614f39565b5f60209050601f831160018114615c80575f8415615c6e578287015190505b615c788582614fa7565b865550615cdf565b601f198416615c8e86614e22565b5f5b82811015615cb557848901518255600182019150602085019450602081019050615c90565b86831015615cd25784890151615cce601f891682614f8b565b8355505b6001600288020188555050505b505050505050565b5f6060820190508181035f830152615cff81876158c8565b9050615d0e6020830186614b0d565b8181036040830152615d2181848661509f565b905095945050505050565b5f67ffffffffffffffff82169050919050565b615d4881615d2c565b82525050565b5f602082019050615d615f830184615d3f565b92915050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f615d9b601583614519565b9150615da682615d67565b602082019050919050565b5f6020820190508181035f830152615dc881615d8f565b9050919050565b5f602082019050615de25f830184614afe565b92915050565b5f81549050919050565b5f819050815f5260205f209050919050565b5f600182019050919050565b5f615e1a82615de8565b615e2481856150d5565b935083602082028501615e3685615df2565b805f5b85811015615e7057848403895281615e518582615188565b9450615e5c83615e04565b925060208a01995050600181019050615e39565b50829750879550505050505092915050565b5f6040820190508181035f830152615e9a8185615e10565b90508181036020830152615eae81846151a7565b90509392505050565b5f81905092915050565b615eca816147c1565b82525050565b5f615edb8383615ec1565b60208301905092915050565b5f615ef182615b56565b615efb8185615eb7565b9350615f0683615b60565b805f5b83811015615f36578151615f1d8882615ed0565b9750615f2883615b86565b925050600181019050615f09565b5085935050505092915050565b5f615f4e8284615ee7565b915081905092915050565b5f606082019050615f6c5f8301866147ca565b615f7960208301856147ca565b615f8660408301846147ca565b949350505050565b5f604082019050615fa15f830185614afe565b615fae6020830184614b0d565b9392505050565b5f60208284031215615fca57615fc96142d6565b5b5f615fd784828501615473565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f616017826143f6565b9150616022836143f6565b925082820190508082111561603a576160396156e6565b5b92915050565b5f6040820190506160535f830185614afe565b6160606020830184614afe565b9392505050565b5f608083015f83015161607c5f860182615783565b50602083015161608f6020860182614b45565b5060408301516160a26040860182615783565b50606083015184820360608601526160ba82826157ed565b9150508091505092915050565b5f6040820190508181035f8301526160df8185616067565b905081810360208301526160f38184616067565b90509392505050565b5f60208284031215616111576161106142d6565b5b5f61611e8482850161545f565b91505092915050565b5f61ffff82169050919050565b5f61614e61614961614484616127565b614ea0565b6143f6565b9050919050565b61615e81616134565b82525050565b5f6040820190506161775f830185616155565b6161846020830184614afe565b9392505050565b5f616195826143f6565b91506161a0836143f6565b92508282026161ae816143f6565b915082820484148315176161c5576161c46156e6565b5b5092915050565b604082015f8201516161e05f850182614b45565b5060208201516161f36020850182614b45565b50505050565b5f60608201905061620c5f830185614afe565b61621960208301846161cc565b9392505050565b5f61622a82615792565b6162348185615a15565b935061623f836157ac565b805f5b8381101561626f57815161625688826157ca565b9750616261836157e1565b925050600181019050616242565b5085935050505092915050565b5f60408201905061628f5f830185614b0d565b81810360208301526162a18184616220565b90509392505050565b5f6020820190508181035f8301526162c381848661509f565b90509392505050565b5f6080820190506162df5f8301876147ca565b6162ec60208301866147ca565b6162f960408301856147ca565b61630660608301846147ca565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f60208201905061634f5f830184615967565b92915050565b605481106163665761636561630f565b5b50565b5f81905061637682616355565b919050565b5f61638582616369565b9050919050565b6163958161637b565b82525050565b5f6020820190506163ae5f83018461638c565b92915050565b5f81905092915050565b6163c7816142fd565b82525050565b5f6163d883836163be565b60208301905092915050565b5f6163ee82615792565b6163f881856163b4565b9350616403836157ac565b805f5b8381101561643357815161641a88826163cd565b9750616425836157e1565b925050600181019050616406565b5085935050505092915050565b5f61644b82846163e4565b915081905092915050565b5f60e0820190506164695f83018a6147ca565b61647660208301896147ca565b61648360408301886147ca565b6164906060830187614b0d565b61649d6080830186614afe565b6164aa60a0830185614afe565b6164b760c0830184614afe565b98975050505050505050565b5f819050815f5260205f209050919050565b601f821115616516576164e7816164c3565b6164f084614e34565b810160208510156164ff578190505b61651361650b85614e34565b830182614f17565b50505b505050565b6165248261450f565b67ffffffffffffffff81111561653d5761653c614643565b5b6165478254614df2565b6165528282856164d5565b5f60209050601f831160018114616583575f8415616571578287015190505b61657b8582614fa7565b8655506165e2565b601f198416616591866164c3565b5f5b828110156165b857848901518255600182019150602085019450602081019050616593565b868310156165d557848901516165d1601f891682614f8b565b8355505b6001600288020188555050505b505050505050565b5f60c0820190506165fd5f8301896147ca565b61660a60208301886147ca565b61661760408301876147ca565b6166246060830186614afe565b6166316080830185614afe565b61663e60a0830184614afe565b979650505050505050565b5f81905092915050565b5f61665d82615c0e565b6166678185616649565b9350616677818560208601614529565b80840191505092915050565b5f61668e8284616653565b915081905092915050565b5f60a0820190506166ac5f8301886147ca565b6166b960208301876147ca565b6166c660408301866147ca565b6166d36060830185614afe565b6166e06080830184614b0d565b9695505050505050565b5f6080820190506166fd5f8301876147ca565b61670a6020830186615967565b61671760408301856147ca565b61672460608301846147ca565b9594505050505056fe44656c656761746564557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c616464726573732064656c656761746f72416464726573732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e44617973295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c7429557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e44617973295573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c62797465732075736572446563727970746564536861726529
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15a\0BW__\xFD[Pa\0Qa\0V` \x1B` \x1CV[a\x01\xB6V[_a\0ea\x01T` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\0\xAFW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x01QWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qa\x01H\x91\x90a\x01\x9DV[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a\x01\x97\x81a\x01{V[\x82RPPV[_` \x82\x01\x90Pa\x01\xB0_\x83\x01\x84a\x01\x8EV[\x92\x91PPV[`\x80Qai\x11a\x01\xDC_9_\x81\x81a%\xEA\x01R\x81\x81a&?\x01Ra'\xF9\x01Rai\x11_\xF3\xFE`\x80`@R`\x046\x10a\x01\x1DW_5`\xE0\x1C\x80c\x83\x16\0\x1F\x11a\0\x9FW\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x03EW\x80c\xB9\xBF\xE0\xA8\x14a\x03oW\x80c\xE3\x0C9x\x14a\x03\x97W\x80c\xF1\x1D\x068\x14a\x03\xC1W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xE9Wa\x01\x1DV[\x80c\x83\x16\0\x1F\x14a\x02sW\x80c\x84\xB0\x19n\x14a\x02\x9BW\x80c\x8D\xA5\xCB[\x14a\x02\xCBW\x80c\xA6\t\x049\x14a\x02\xF5W\x80c\xAA9\xA3V\x14a\x03\x1DWa\x01\x1DV[\x80cR\xD1\x90-\x11a\0\xE6W\x80cR\xD1\x90-\x14a\x01\xDFW\x80cqP\x18\xA6\x14a\x02\tW\x80cv\n\x04\x19\x14a\x02\x1FW\x80cy\xBAP\x97\x14a\x02GW\x80c\x81)\xFC\x1C\x14a\x02]Wa\x01\x1DV[\x80b\x8B\xC3\xE1\x14a\x01!W\x80c\x02\xFD\x1Ad\x14a\x01IW\x80c\r\x8En,\x14a\x01qW\x80c\x18\x7F\xE5)\x14a\x01\x9BW\x80cO\x1E\xF2\x86\x14a\x01\xC3W[__\xFD[4\x80\x15a\x01,W__\xFD[Pa\x01G`\x04\x806\x03\x81\x01\x90a\x01B\x91\x90aC\x99V[a\x04\x11V[\0[4\x80\x15a\x01TW__\xFD[Pa\x01o`\x04\x806\x03\x81\x01\x90a\x01j\x91\x90aD~V[a\x06\x1EV[\0[4\x80\x15a\x01|W__\xFD[Pa\x01\x85a\x08\x81V[`@Qa\x01\x92\x91\x90aE\x7FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA6W__\xFD[Pa\x01\xC1`\x04\x806\x03\x81\x01\x90a\x01\xBC\x91\x90aE\xF4V[a\x08\xFCV[\0[a\x01\xDD`\x04\x806\x03\x81\x01\x90a\x01\xD8\x91\x90aGgV[a\n\xAAV[\0[4\x80\x15a\x01\xEAW__\xFD[Pa\x01\xF3a\n\xC9V[`@Qa\x02\0\x91\x90aG\xD9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x14W__\xFD[Pa\x02\x1Da\n\xFAV[\0[4\x80\x15a\x02*W__\xFD[Pa\x02E`\x04\x806\x03\x81\x01\x90a\x02@\x91\x90aH\x87V[a\x0B\rV[\0[4\x80\x15a\x02RW__\xFD[Pa\x02[a\x0F\xCEV[\0[4\x80\x15a\x02hW__\xFD[Pa\x02qa\x10\\V[\0[4\x80\x15a\x02~W__\xFD[Pa\x02\x99`\x04\x806\x03\x81\x01\x90a\x02\x94\x91\x90aI\xA6V[a\x12\x05V[\0[4\x80\x15a\x02\xA6W__\xFD[Pa\x02\xAFa\x15\xC3V[`@Qa\x02\xC2\x97\x96\x95\x94\x93\x92\x91\x90aK\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xD6W__\xFD[Pa\x02\xDFa\x16\xCCV[`@Qa\x02\xEC\x91\x90aLUV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\0W__\xFD[Pa\x03\x1B`\x04\x806\x03\x81\x01\x90a\x03\x16\x91\x90aLnV[a\x17\x01V[\0[4\x80\x15a\x03(W__\xFD[Pa\x03C`\x04\x806\x03\x81\x01\x90a\x03>\x91\x90aE\xF4V[a\x17qV[\0[4\x80\x15a\x03PW__\xFD[Pa\x03Ya\x18\xB7V[`@Qa\x03f\x91\x90aE\x7FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03zW__\xFD[Pa\x03\x95`\x04\x806\x03\x81\x01\x90a\x03\x90\x91\x90aD~V[a\x18\xF0V[\0[4\x80\x15a\x03\xA2W__\xFD[Pa\x03\xABa\x1CUV[`@Qa\x03\xB8\x91\x90aLUV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xCCW__\xFD[Pa\x03\xE7`\x04\x806\x03\x81\x01\x90a\x03\xE2\x91\x90aL\x99V[a\x1C\x8AV[\0[4\x80\x15a\x03\xF4W__\xFD[Pa\x04\x0F`\x04\x806\x03\x81\x01\x90a\x04\n\x91\x90aM<V[a\x1F*V[\0[__\x90P[\x82\x82\x90P\x81\x10\x15a\x06\x18Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84\x84\x81\x81\x10a\x04dWa\x04caMgV[[\x90P`@\x02\x01_\x015\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x8B\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xA1W__\xFD[PZ\xFA\x15\x80\x15a\x04\xB3W=__>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84\x84\x81\x81\x10a\x04\xFAWa\x04\xF9aMgV[[\x90P`@\x02\x01_\x015\x85\x85\x85\x81\x81\x10a\x05\x16Wa\x05\x15aMgV[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x05.\x91\x90aM<V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05K\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05aW__\xFD[PZ\xFA\x15\x80\x15a\x05sW=__>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x84\x84\x84\x81\x81\x10a\x05\xBAWa\x05\xB9aMgV[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xDF\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xF5W__\xFD[PZ\xFA\x15\x80\x15a\x06\x07W=__>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x04\x16V[PPPPV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06k\x91\x90aLUV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x81W__\xFD[PZ\xFA\x15\x80\x15a\x06\x93W=__>=_\xFD[PPPP_a\x06\xA0a\x1F\xE3V[\x90P_`@Q\x80`@\x01`@R\x80\x83`\x04\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\tW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\xF5W[PPPPP\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x07f\x82a \nV[\x90Pa\x07t\x88\x82\x87\x87a \x98V[_\x83`\x03\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x86\x86\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x07\xD2\x92\x91\x90aO\xC2V[P\x83`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x08\tWPa\x08\x08\x81\x80T\x90Pa\"yV[[\x15a\x08vW`\x01\x84`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x88\x7FaV\x8Dn\xB4\x8Eb\x87\n\xFF\xFDUI\x92\x06\xA5J\x8Fx\xB0Jb~\0\xED\tqa\xFC\x05\xD6\xBE\x89\x89\x84`@Qa\x08m\x93\x92\x91\x90aR\x19V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x08\xC2_a#\nV[a\x08\xCC`\x01a#\nV[a\x08\xD5_a#\nV[`@Q` \x01a\x08\xE8\x94\x93\x92\x91\x90aS\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x82\x82\x90P\x03a\t8W`@Q\x7F-\xE7T8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x81\x82\x82\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa#\xD4V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xD1\x92\x91\x90aS\xF4V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xEBW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x13\x91\x90aV\x9FV[\x90Pa\n\x1E\x81a%\x02V[_a\n'a\x1F\xE3V[\x90P\x80_\x01_\x81T\x80\x92\x91\x90a\n<\x90aW\x13V[\x91\x90PUP_\x81_\x01T\x90P\x84\x84\x83`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x91\x90a\nj\x92\x91\x90aB\x1CV[P\x80\x7F\x17\xC62\x19o\xBFk\x96\xD9gYq\x05\x8D7\x01s0\x94\xC3\xF2\xF1\xDC\xB9\xBA}*\x08\xBE\xE0\xAA\xFB\x84`@Qa\n\x9B\x91\x90aY;V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\n\xB2a%\xE8V[a\n\xBB\x82a&\xCEV[a\n\xC5\x82\x82a&\xD9V[PPV[_a\n\xD2a'\xF7V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[a\x0B\x02a(~V[a\x0B\x0B_a)\x05V[V[`\n`\xFF\x16\x86\x86\x90P\x11\x15a\x0B_W`\n\x86\x86\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BV\x92\x91\x90aYvV[`@Q\x80\x91\x03\x90\xFD[a\x0Bx\x89\x806\x03\x81\x01\x90a\x0Bs\x91\x90aY\xEAV[a)BV[a\x0B\xD3\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x89_\x01` \x81\x01\x90a\x0B\xCE\x91\x90aM<V[a*\x8DV[\x15a\x0C*W\x87_\x01` \x81\x01\x90a\x0B\xEA\x91\x90aM<V[\x86\x86`@Q\x7F\xC3Dj\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C!\x93\x92\x91\x90aZ\xABV[`@Q\x80\x91\x03\x90\xFD[_a\x0C\x88\x8C\x8C\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8C_\x01` \x81\x01\x90a\x0C\x83\x91\x90aM<V[a+\x0BV[\x90Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x89\x8B\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xDD\x94\x93\x92\x91\x90a[\x18V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xF3W__\xFD[PZ\xFA\x15\x80\x15a\r\x05W=__>=_\xFD[PPPP_`@Q\x80`\xC0\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B_\x01` \x81\x01\x90a\r\xB6\x91\x90aM<V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81RP\x90Pa\x0E\x08\x81\x8B` \x01` \x81\x01\x90a\x0E\x01\x91\x90aM<V[\x86\x86a-\xE6V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0EV\x91\x90a[\xEEV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EpW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x98\x91\x90aV\x9FV[\x90Pa\x0E\xA3\x81a%\x02V[_a\x0E\xACa\x1F\xE3V[\x90P\x80_\x01_\x81T\x80\x92\x91\x90a\x0E\xC1\x90aW\x13V[\x91\x90PUP_\x81_\x01T\x90P`@Q\x80`@\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x0FK\x91\x90a\\\x18V[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x0Fh\x92\x91\x90aBgV[P\x90PP\x80\x7F\x1C=\xCA\xD61\x1B\xE6\xD5\x8D\xC4\xD4\xB9\xF1\xBC\x16%\xEB\x18\xD7-\xE9i\xDBu\xE1\x1A\x88\xEF5'\xD2\xF3\x84\x8F` \x01` \x81\x01\x90a\x0F\xA2\x91\x90aM<V[\x8C\x8C`@Qa\x0F\xB4\x94\x93\x92\x91\x90a\\\xE7V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_a\x0F\xD7a.\xBCV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0F\xF8a\x1CUV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10PW\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10G\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[a\x10Y\x81a)\x05V[PV[`\x02_a\x10ga.\xC3V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x10\xAFWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x10\xE6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x11\x9F`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa.\xEAV[a\x11\xAFa\x11\xAAa\x16\xCCV[a/\0V[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x11\xF9\x91\x90a]NV[`@Q\x80\x91\x03\x90\xA1PPV[`\n`\xFF\x16\x87\x87\x90P\x11\x15a\x12WW`\n\x87\x87\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12N\x92\x91\x90aYvV[`@Q\x80\x91\x03\x90\xFD[a\x12p\x89\x806\x03\x81\x01\x90a\x12k\x91\x90aY\xEAV[a)BV[a\x12\xBA\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x86a*\x8DV[\x15a\x13\0W\x84\x87\x87`@Q\x7F\xDCMx\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xF7\x93\x92\x91\x90aZ\xABV[`@Q\x80\x91\x03\x90\xFD[_a\x13M\x8C\x8C\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x89a+\x0BV[\x90P_`@Q\x80`\xA0\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81RP\x90Pa\x14\x0F\x81\x88\x86\x86a/\x14V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14]\x91\x90a[\xEEV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14wW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9F\x91\x90aV\x9FV[\x90Pa\x14\xAA\x81a%\x02V[_a\x14\xB3a\x1F\xE3V[\x90P\x80_\x01_\x81T\x80\x92\x91\x90a\x14\xC8\x90aW\x13V[\x91\x90PUP_\x81_\x01T\x90P`@Q\x80`@\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x15R\x91\x90a\\\x18V[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x15o\x92\x91\x90aBgV[P\x90PP\x80\x7F\x1C=\xCA\xD61\x1B\xE6\xD5\x8D\xC4\xD4\xB9\xF1\xBC\x16%\xEB\x18\xD7-\xE9i\xDBu\xE1\x1A\x88\xEF5'\xD2\xF3\x84\x8C\x8C\x8C`@Qa\x15\xA9\x94\x93\x92\x91\x90a\\\xE7V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_``\x80___``_a\x15\xD5a/\xEAV[\x90P__\x1B\x81_\x01T\x14\x80\x15a\x15\xF0WP__\x1B\x81`\x01\x01T\x14[a\x16/W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16&\x90a]\xB1V[`@Q\x80\x91\x03\x90\xFD[a\x167a0\x11V[a\x16?a0\xAFV[F0__\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16^Wa\x16]aFCV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x8CW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[__a\x16\xD6a1MV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[_a\x17\na\x1F\xE3V[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x17mW\x81`@Q\x7F\x0B\xF0\x14\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17d\x91\x90a]\xCFV[`@Q\x80\x91\x03\x90\xFD[PPV[__\x90P[\x82\x82\x90P\x81\x10\x15a\x18\xB2Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x84\x84\x84\x81\x81\x10a\x17\xC4Wa\x17\xC3aMgV[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xE7\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xFDW__\xFD[PZ\xFA\x15\x80\x15a\x18\x0FW=__>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x84\x84\x84\x81\x81\x10a\x18VWa\x18UaMgV[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18y\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x8FW__\xFD[PZ\xFA\x15\x80\x15a\x18\xA1W=__>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x17vV[PPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19=\x91\x90aLUV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19SW__\xFD[PZ\xFA\x15\x80\x15a\x19eW=__>=_\xFD[PPPP_a\x19ra\x1F\xE3V[\x90P_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x19\xA2\x90aM\xF2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xCE\x90aM\xF2V[\x80\x15a\x1A\x19W\x80`\x1F\x10a\x19\xF0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x19V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1AoW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1A[W[PPPPP\x81RPP\x90P_`@Q\x80``\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x1A\xEC\x82a1tV[\x90Pa\x1A\xFA\x89\x82\x88\x88a \x98V[_\x84`\x05\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x87\x87\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x1BI\x92\x91\x90aO\xC2V[P\x84`\x08\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x89\x89\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x1B\x95\x92\x91\x90aO\xC2V[P\x84`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1B\xCCWPa\x1B\xCB\x81\x80T\x90Pa2\x0FV[[\x15a\x1CIW`\x01\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x7Fs\x12\xDE\xC4\xCE\xAD\r]=\xA86\xCD\xBA\xED\x1E\xB6\xA8\x1E!\x8CQ\x9C\x87@\xDAJ\xC7Z\xFC\xB6\xC5\xC7\x86`\x08\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x83`@Qa\x1C@\x92\x91\x90a^\x82V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPV[__a\x1C_a2\xA0V[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x87\x87\x85\x85`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xDD\x94\x93\x92\x91\x90a[\x18V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xF3W__\xFD[PZ\xFA\x15\x80\x15a\x1D\x05W=__>=_\xFD[PPPP__\x90P[\x84\x84\x90P\x81\x10\x15a\x1F!Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a\x1D\\Wa\x1D[aMgV[[\x90P`@\x02\x01_\x015\x88_\x01` \x81\x01\x90a\x1Dw\x91\x90aM<V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x94\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\xAAW__\xFD[PZ\xFA\x15\x80\x15a\x1D\xBCW=__>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a\x1E\x03Wa\x1E\x02aMgV[[\x90P`@\x02\x01_\x015\x87\x87\x85\x81\x81\x10a\x1E\x1FWa\x1E\x1EaMgV[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1E7\x91\x90aM<V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1ET\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1EjW__\xFD[PZ\xFA\x15\x80\x15a\x1E|W=__>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x86\x86\x84\x81\x81\x10a\x1E\xC3Wa\x1E\xC2aMgV[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xE8\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xFEW__\xFD[PZ\xFA\x15\x80\x15a\x1F\x10W=__>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x1D\x0EV[PPPPPPPV[a\x1F2a(~V[_a\x1F;a2\xA0V[\x90P\x81\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\x9Da\x16\xCCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x90P\x90V[_a \x91`@Q\x80`\x80\x01`@R\x80`D\x81R` \x01ag\xE0`D\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a B\x91\x90a_CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 `@Q` \x01a v\x93\x92\x91\x90a_YV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[_a \xA1a\x1F\xE3V[\x90P_a \xF1\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa2\xE0V[\x90Ps\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cl\x88\xEBC\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!@\x91\x90aLUV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a!VW__\xFD[PZ\xFA\x15\x80\x15a!hW=__>=_\xFD[PPPP\x81`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\"\x0BW\x85\x81`@Q\x7F\x99\xECH\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\x02\x92\x91\x90a_\x8EV[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPV[__s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cG\xCDK>`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xFC\x91\x90a_\xB5V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[``_`\x01a#\x18\x84a3\nV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#6Wa#5aFCV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#hW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a#\xC9W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#\xBEWa#\xBDa_\xE0V[[\x04\x94P_\x85\x03a#uW[\x81\x93PPPP\x91\x90PV[__\x90P__\x90P[\x82Q\x81\x10\x15a$\xB2W_\x83\x82\x81Q\x81\x10a#\xFAWa#\xF9aMgV[[` \x02` \x01\x01Q\x90P_a$\x0E\x82a4[V[\x90Pa$\x19\x81a4\xE5V[a\xFF\xFF\x16\x84a$(\x91\x90a`\rV[\x93Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$w\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a$\x8DW__\xFD[PZ\xFA\x15\x80\x15a$\x9FW=__>=_\xFD[PPPPPP\x80\x80`\x01\x01\x91PPa#\xDDV[Pa\x08\0\x81\x11\x15a$\xFEWa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xF5\x92\x91\x90a`@V[`@Q\x80\x91\x03\x90\xFD[PPV[`\x01\x81Q\x11\x15a%\xE5W_\x81_\x81Q\x81\x10a% Wa%\x1FaMgV[[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a%\xE2W\x81\x83\x82\x81Q\x81\x10a%QWa%PaMgV[[` \x02` \x01\x01Q` \x01Q\x14a%\xD5W\x82_\x81Q\x81\x10a%uWa%taMgV[[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a%\x90Wa%\x8FaMgV[[` \x02` \x01\x01Q`@Q\x7F\xCF\xAE\x92\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xCC\x92\x91\x90a`\xC7V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa%4V[PP[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a&\x95WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a&|a7rV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a&\xCCW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a&\xD6a(~V[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a'AWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'>\x91\x90a`\xFCV[`\x01[a'\x82W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'y\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a'\xE8W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\xDF\x91\x90aG\xD9V[`@Q\x80\x91\x03\x90\xFD[a'\xF2\x83\x83a7\xC5V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(|W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a(\x86a.\xBCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\xA4a\x16\xCCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)\x03Wa(\xC7a.\xBCV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\xFA\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[V[_a)\x0Ea2\xA0V[\x90P\x80_\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua)>\x82a87V[PPV[_\x81` \x01Q\x03a)\x7FW`@Q\x7F\xDE(Y\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01ma\xFF\xFF\x16\x81` \x01Q\x11\x15a)\xD6Wa\x01m\x81` \x01Q`@Q\x7F2\x95\x18c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xCD\x92\x91\x90aadV[`@Q\x80\x91\x03\x90\xFD[B\x81_\x01Q\x11\x15a*#WB\x81_\x01Q`@Q\x7F\xF2L\x08\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\x1A\x92\x91\x90a`@V[`@Q\x80\x91\x03\x90\xFD[Bb\x01Q\x80\x82` \x01Qa*7\x91\x90aa\x8BV[\x82_\x01Qa*E\x91\x90a`\rV[\x10\x15a*\x8AWB\x81`@Q\x7F04\x80@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\x81\x92\x91\x90aa\xF9V[`@Q\x80\x91\x03\x90\xFD[PV[___\x90P[\x83Q\x81\x10\x15a+\0W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x82\x81Q\x81\x10a*\xC6Wa*\xC5aMgV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a*\xF3W`\x01\x91PPa+\x05V[\x80\x80`\x01\x01\x91PPa*\x93V[P_\x90P[\x92\x91PPV[``_\x85\x85\x90P\x03a+IW`@Q\x7F\xA6\xA6\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+fWa+eaFCV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+\x94W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a-\x91W_\x87\x87\x83\x81\x81\x10a+\xBFWa+\xBEaMgV[[\x90P`@\x02\x01_\x015\x90P_\x88\x88\x84\x81\x81\x10a+\xDEWa+\xDDaMgV[[\x90P`@\x02\x01` \x01` \x81\x01\x90a+\xF6\x91\x90aM<V[\x90P_a,\x02\x83a4[V[\x90Pa,\r\x81a4\xE5V[a\xFF\xFF\x16\x85a,\x1C\x91\x90a`\rV[\x94Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,m\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a,\x83W__\xFD[PZ\xFA\x15\x80\x15a,\x95W=__>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xE8\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a,\xFEW__\xFD[PZ\xFA\x15\x80\x15a-\x10W=__>=_\xFD[PPPPa-\x1E\x88\x83a*\x8DV[a-aW\x81\x88`@Q\x7F\xA4\xC3\x03\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-X\x92\x91\x90ab|V[`@Q\x80\x91\x03\x90\xFD[\x82\x86\x85\x81Q\x81\x10a-uWa-taMgV[[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80`\x01\x01\x91PPa+\xA0V[Pa\x08\0\x81\x11\x15a-\xDDWa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xD4\x92\x91\x90a`@V[`@Q\x80\x91\x03\x90\xFD[P\x94\x93PPPPV[_a-\xF0\x85a9\x08V[\x90P_a.@\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa2\xE0V[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a.\xB4W\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xAB\x92\x91\x90ab\xAAV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[_3\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a.\xF2a9\xAEV[a.\xFC\x82\x82a9\xEEV[PPV[a/\x08a9\xAEV[a/\x11\x81a:?V[PV[_a/\x1E\x85a:\xC3V[\x90P_a/n\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa2\xE0V[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/\xE2W\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xD9\x92\x91\x90ab\xAAV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a0\x1Ca/\xEAV[\x90P\x80`\x02\x01\x80Ta0-\x90aM\xF2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0Y\x90aM\xF2V[\x80\x15a0\xA4W\x80`\x1F\x10a0{Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xA4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a0\xBAa/\xEAV[\x90P\x80`\x03\x01\x80Ta0\xCB\x90aM\xF2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\xF7\x90aM\xF2V[\x80\x15a1BW\x80`\x1F\x10a1\x19Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90P\x90V[_a2\x08`@Q\x80`\x80\x01`@R\x80`]\x81R` \x01ah\xB4`]\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a1\xB8\x91\x90a_CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 `@Q` \x01a1\xED\x94\x93\x92\x91\x90ab\xCCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[__s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cI\x04\x13\xAA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x92\x91\x90a_\xB5V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[_\x7F#~\x15\x82\"\xE3\xE6\x96\x8Br\xB9\xDB\r\x80C\xAA\xCF\x07J\xD9\xF6P\xF0\xD1`kM\x82\xEEC,\0\x90P\x90V[_a2\xD9a2\xD3a;cV[\x83a;qV[\x90P\x91\x90PV[____a2\xEE\x86\x86a;\xB1V[\x92P\x92P\x92Pa2\xFE\x82\x82a<\x06V[\x82\x93PPPP\x92\x91PPV[___\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a3fWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a3\\Wa3[a_\xE0V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a3\xA3Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a3\x99Wa3\x98a_\xE0V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a3\xD2Wf#\x86\xF2o\xC1\0\0\x83\x81a3\xC8Wa3\xC7a_\xE0V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a3\xFBWc\x05\xF5\xE1\0\x83\x81a3\xF1Wa3\xF0a_\xE0V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a4 Wa'\x10\x83\x81a4\x16Wa4\x15a_\xE0V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a4CW`d\x83\x81a49Wa48a_\xE0V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a4RW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[__`\xF8`\xF0\x84\x90\x1B\x90\x1C_\x1C\x90P`S\x80\x81\x11\x15a4}Wa4|ac\x0FV[[`\xFF\x16\x81`\xFF\x16\x11\x15a4\xC7W\x80`@Q\x7Fd\x19P\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xBE\x91\x90ac<V[`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`S\x81\x11\x15a4\xDDWa4\xDCac\x0FV[[\x91PP\x91\x90PV[__`S\x81\x11\x15a4\xF9Wa4\xF8ac\x0FV[[\x82`S\x81\x11\x15a5\x0CWa5\x0Bac\x0FV[[\x03a5\x1AW`\x02\x90Pa7mV[`\x02`S\x81\x11\x15a5.Wa5-ac\x0FV[[\x82`S\x81\x11\x15a5AWa5@ac\x0FV[[\x03a5OW`\x08\x90Pa7mV[`\x03`S\x81\x11\x15a5cWa5bac\x0FV[[\x82`S\x81\x11\x15a5vWa5uac\x0FV[[\x03a5\x84W`\x10\x90Pa7mV[`\x04`S\x81\x11\x15a5\x98Wa5\x97ac\x0FV[[\x82`S\x81\x11\x15a5\xABWa5\xAAac\x0FV[[\x03a5\xB9W` \x90Pa7mV[`\x05`S\x81\x11\x15a5\xCDWa5\xCCac\x0FV[[\x82`S\x81\x11\x15a5\xE0Wa5\xDFac\x0FV[[\x03a5\xEEW`@\x90Pa7mV[`\x06`S\x81\x11\x15a6\x02Wa6\x01ac\x0FV[[\x82`S\x81\x11\x15a6\x15Wa6\x14ac\x0FV[[\x03a6#W`\x80\x90Pa7mV[`\x07`S\x81\x11\x15a67Wa66ac\x0FV[[\x82`S\x81\x11\x15a6JWa6Iac\x0FV[[\x03a6XW`\xA0\x90Pa7mV[`\x08`S\x81\x11\x15a6lWa6kac\x0FV[[\x82`S\x81\x11\x15a6\x7FWa6~ac\x0FV[[\x03a6\x8EWa\x01\0\x90Pa7mV[`\t`S\x81\x11\x15a6\xA2Wa6\xA1ac\x0FV[[\x82`S\x81\x11\x15a6\xB5Wa6\xB4ac\x0FV[[\x03a6\xC4Wa\x02\0\x90Pa7mV[`\n`S\x81\x11\x15a6\xD8Wa6\xD7ac\x0FV[[\x82`S\x81\x11\x15a6\xEBWa6\xEAac\x0FV[[\x03a6\xFAWa\x04\0\x90Pa7mV[`\x0B`S\x81\x11\x15a7\x0EWa7\rac\x0FV[[\x82`S\x81\x11\x15a7!Wa7 ac\x0FV[[\x03a70Wa\x08\0\x90Pa7mV[\x81`@Q\x7F\xBEx0\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7d\x91\x90ac\x9BV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[_a7\x9E\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba=hV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a7\xCE\x82a=qV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a8*Wa8$\x82\x82a>:V[Pa83V[a82a>\xBAV[[PPV[_a8@a1MV[\x90P_\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82\x82_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPPV[_a9\xA7`@Q\x80`\xE0\x01`@R\x80`\xB2\x81R` \x01ag.`\xB2\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a9L\x91\x90ad@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q`@Q` \x01a9\x8C\x97\x96\x95\x94\x93\x92\x91\x90adVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[a9\xB6a>\xF6V[a9\xECW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a9\xF6a9\xAEV[_a9\xFFa/\xEAV[\x90P\x82\x81`\x02\x01\x90\x81a:\x12\x91\x90ae\x1BV[P\x81\x81`\x03\x01\x90\x81a:$\x91\x90ae\x1BV[P__\x1B\x81_\x01\x81\x90UP__\x1B\x81`\x01\x01\x81\x90UPPPPV[a:Ga9\xAEV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a:\xB7W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\xAE\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[a:\xC0\x81a)\x05V[PV[_a;\\`@Q\x80`\xC0\x01`@R\x80`\x90\x81R` \x01ah$`\x90\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a;\x07\x91\x90ad@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a;A\x96\x95\x94\x93\x92\x91\x90ae\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[_a;la?\x14V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[___`A\x84Q\x03a;\xF1W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa;\xE3\x88\x82\x85\x85a?wV[\x95P\x95P\x95PPPPa;\xFFV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a<\x19Wa<\x18ac\x0FV[[\x82`\x03\x81\x11\x15a<,Wa<+ac\x0FV[[\x03\x15a=dW`\x01`\x03\x81\x11\x15a<FWa<Eac\x0FV[[\x82`\x03\x81\x11\x15a<YWa<Xac\x0FV[[\x03a<\x90W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a<\xA4Wa<\xA3ac\x0FV[[\x82`\x03\x81\x11\x15a<\xB7Wa<\xB6ac\x0FV[[\x03a<\xFBW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<\xF2\x91\x90a]\xCFV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a=\x0EWa=\rac\x0FV[[\x82`\x03\x81\x11\x15a=!Wa= ac\x0FV[[\x03a=cW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=Z\x91\x90aG\xD9V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a=\xCCW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\xC3\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[\x80a=\xF8\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba=hV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa>c\x91\x90af\x83V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a>\x9BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\xA0V[``\x91P[P\x91P\x91Pa>\xB0\x85\x83\x83a@^V[\x92PPP\x92\x91PPV[_4\x11\x15a>\xF4W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a>\xFFa.\xC3V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa?>a@\xEBV[a?FaAaV[F0`@Q` \x01a?\\\x95\x94\x93\x92\x91\x90af\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a?\xB3W_`\x03\x85\x92P\x92P\x92Pa@TV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa?\xD6\x94\x93\x92\x91\x90af\xEAV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a?\xF6W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a@GW_`\x01__\x1B\x93P\x93P\x93PPa@TV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a@sWa@n\x82aA\xD8V[a@\xE3V[_\x82Q\x14\x80\x15a@\x99WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a@\xDBW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@\xD2\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa@\xE4V[[\x93\x92PPPV[__a@\xF5a/\xEAV[\x90P_aA\0a0\x11V[\x90P_\x81Q\x11\x15aA\x1CW\x80\x80Q\x90` \x01 \x92PPPaA^V[_\x82_\x01T\x90P__\x1B\x81\x14aA7W\x80\x93PPPPaA^V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[__aAka/\xEAV[\x90P_aAva0\xAFV[\x90P_\x81Q\x11\x15aA\x92W\x80\x80Q\x90` \x01 \x92PPPaA\xD5V[_\x82`\x01\x01T\x90P__\x1B\x81\x14aA\xAEW\x80\x93PPPPaA\xD5V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aA\xEAW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aBVW\x91` \x02\x82\x01[\x82\x81\x11\x15aBUW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aB:V[[P\x90PaBc\x91\x90aB\xB2V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aB\xA1W\x91` \x02\x82\x01[\x82\x81\x11\x15aB\xA0W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aB\x85V[[P\x90PaB\xAE\x91\x90aB\xB2V[P\x90V[[\x80\x82\x11\x15aB\xC9W_\x81_\x90UP`\x01\x01aB\xB3V[P\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aC\x07\x82aB\xDEV[\x90P\x91\x90PV[aC\x17\x81aB\xFDV[\x81\x14aC!W__\xFD[PV[_\x815\x90PaC2\x81aC\x0EV[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12aCYWaCXaC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aCvWaCuaC<V[[` \x83\x01\x91P\x83`@\x82\x02\x83\x01\x11\x15aC\x92WaC\x91aC@V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aC\xB0WaC\xAFaB\xD6V[[_aC\xBD\x86\x82\x87\x01aC$V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\xDEWaC\xDDaB\xDAV[[aC\xEA\x86\x82\x87\x01aCDV[\x92P\x92PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[aD\x08\x81aC\xF6V[\x81\x14aD\x12W__\xFD[PV[_\x815\x90PaD#\x81aC\xFFV[\x92\x91PPV[__\x83`\x1F\x84\x01\x12aD>WaD=aC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD[WaDZaC<V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aDwWaDvaC@V[[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15aD\x97WaD\x96aB\xD6V[[_aD\xA4\x88\x82\x89\x01aD\x15V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xC5WaD\xC4aB\xDAV[[aD\xD1\x88\x82\x89\x01aD)V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xF4WaD\xF3aB\xDAV[[aE\0\x88\x82\x89\x01aD)V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aEQ\x82aE\x0FV[aE[\x81\x85aE\x19V[\x93PaEk\x81\x85` \x86\x01aE)V[aEt\x81aE7V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaE\x97\x81\x84aEGV[\x90P\x92\x91PPV[__\x83`\x1F\x84\x01\x12aE\xB4WaE\xB3aC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xD1WaE\xD0aC<V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aE\xEDWaE\xECaC@V[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15aF\nWaF\taB\xD6V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF'WaF&aB\xDAV[[aF3\x85\x82\x86\x01aE\x9FV[\x92P\x92PP\x92P\x92\x90PV[__\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aFy\x82aE7V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aF\x98WaF\x97aFCV[[\x80`@RPPPV[_aF\xAAaB\xCDV[\x90PaF\xB6\x82\x82aFpV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aF\xD5WaF\xD4aFCV[[aF\xDE\x82aE7V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aG\x0BaG\x06\x84aF\xBBV[aF\xA1V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aG'WaG&aF?V[[aG2\x84\x82\x85aF\xEBV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aGNWaGMaC8V[[\x815aG^\x84\x82` \x86\x01aF\xF9V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aG}WaG|aB\xD6V[[_aG\x8A\x85\x82\x86\x01aC$V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xABWaG\xAAaB\xDAV[[aG\xB7\x85\x82\x86\x01aG:V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aG\xD3\x81aG\xC1V[\x82RPPV[_` \x82\x01\x90PaG\xEC_\x83\x01\x84aG\xCAV[\x92\x91PPV[__\xFD[_`@\x82\x84\x03\x12\x15aH\x0BWaH\naG\xF2V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15aH)WaH(aG\xF2V[[\x81\x90P\x92\x91PPV[__\x83`\x1F\x84\x01\x12aHGWaHFaC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aHdWaHcaC<V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aH\x80WaH\x7FaC@V[[\x92P\x92\x90PV[___________a\x01 \x8C\x8E\x03\x12\x15aH\xA7WaH\xA6aB\xD6V[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xC4WaH\xC3aB\xDAV[[aH\xD0\x8E\x82\x8F\x01aCDV[\x9BP\x9BPP` aH\xE3\x8E\x82\x8F\x01aG\xF6V[\x99PP``aH\xF4\x8E\x82\x8F\x01aH\x14V[\x98PP`\xA0aI\x05\x8E\x82\x8F\x01aD\x15V[\x97PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI&WaI%aB\xDAV[[aI2\x8E\x82\x8F\x01aH2V[\x96P\x96PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIUWaITaB\xDAV[[aIa\x8E\x82\x8F\x01aD)V[\x94P\x94PPa\x01\0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\x85WaI\x84aB\xDAV[[aI\x91\x8E\x82\x8F\x01aD)V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[___________a\x01\0\x8C\x8E\x03\x12\x15aI\xC6WaI\xC5aB\xD6V[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\xE3WaI\xE2aB\xDAV[[aI\xEF\x8E\x82\x8F\x01aCDV[\x9BP\x9BPP` aJ\x02\x8E\x82\x8F\x01aG\xF6V[\x99PP``aJ\x13\x8E\x82\x8F\x01aD\x15V[\x98PP`\x80\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ4WaJ3aB\xDAV[[aJ@\x8E\x82\x8F\x01aH2V[\x97P\x97PP`\xA0aJS\x8E\x82\x8F\x01aC$V[\x95PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJtWaJsaB\xDAV[[aJ\x80\x8E\x82\x8F\x01aD)V[\x94P\x94PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xA3WaJ\xA2aB\xDAV[[aJ\xAF\x8E\x82\x8F\x01aD)V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aJ\xF8\x81aJ\xC4V[\x82RPPV[aK\x07\x81aC\xF6V[\x82RPPV[aK\x16\x81aB\xFDV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aKN\x81aC\xF6V[\x82RPPV[_aK_\x83\x83aKEV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aK\x81\x82aK\x1CV[aK\x8B\x81\x85aK&V[\x93PaK\x96\x83aK6V[\x80_[\x83\x81\x10\x15aK\xC6W\x81QaK\xAD\x88\x82aKTV[\x97PaK\xB8\x83aKkV[\x92PP`\x01\x81\x01\x90PaK\x99V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaK\xE6_\x83\x01\x8AaJ\xEFV[\x81\x81\x03` \x83\x01RaK\xF8\x81\x89aEGV[\x90P\x81\x81\x03`@\x83\x01RaL\x0C\x81\x88aEGV[\x90PaL\x1B``\x83\x01\x87aJ\xFEV[aL(`\x80\x83\x01\x86aK\rV[aL5`\xA0\x83\x01\x85aG\xCAV[\x81\x81\x03`\xC0\x83\x01RaLG\x81\x84aKwV[\x90P\x98\x97PPPPPPPPV[_` \x82\x01\x90PaLh_\x83\x01\x84aK\rV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aL\x83WaL\x82aB\xD6V[[_aL\x90\x84\x82\x85\x01aD\x15V[\x91PP\x92\x91PPV[______`\xA0\x87\x89\x03\x12\x15aL\xB3WaL\xB2aB\xD6V[[_aL\xC0\x89\x82\x8A\x01aD\x15V[\x96PP` aL\xD1\x89\x82\x8A\x01aH\x14V[\x95PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aL\xF2WaL\xF1aB\xDAV[[aL\xFE\x89\x82\x8A\x01aCDV[\x94P\x94PP`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM!WaM aB\xDAV[[aM-\x89\x82\x8A\x01aH2V[\x92P\x92PP\x92\x95P\x92\x95P\x92\x95V[_` \x82\x84\x03\x12\x15aMQWaMPaB\xD6V[[_aM^\x84\x82\x85\x01aC$V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`@\x82\x01\x90PaM\xA7_\x83\x01\x85aG\xCAV[aM\xB4` \x83\x01\x84aK\rV[\x93\x92PPPV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aN\tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aN\x1CWaN\x1BaM\xC5V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aN~\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aNCV[aN\x88\x86\x83aNCV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aN\xC3aN\xBEaN\xB9\x84aC\xF6V[aN\xA0V[aC\xF6V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aN\xDC\x83aN\xA9V[aN\xF0aN\xE8\x82aN\xCAV[\x84\x84TaNOV[\x82UPPPPV[__\x90P\x90V[aO\x07aN\xF8V[aO\x12\x81\x84\x84aN\xD3V[PPPV[[\x81\x81\x10\x15aO5WaO*_\x82aN\xFFV[`\x01\x81\x01\x90PaO\x18V[PPV[`\x1F\x82\x11\x15aOzWaOK\x81aN\"V[aOT\x84aN4V[\x81\x01` \x85\x10\x15aOcW\x81\x90P[aOwaOo\x85aN4V[\x83\x01\x82aO\x17V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aO\x9A_\x19\x84`\x08\x02aO\x7FV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aO\xB2\x83\x83aO\x8BV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aO\xCC\x83\x83aM\xBBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\xE5WaO\xE4aFCV[[aO\xEF\x82TaM\xF2V[aO\xFA\x82\x82\x85aO9V[_`\x1F\x83\x11`\x01\x81\x14aP'W_\x84\x15aP\x15W\x82\x87\x015\x90P[aP\x1F\x85\x82aO\xA7V[\x86UPaP\x86V[`\x1F\x19\x84\x16aP5\x86aN\"V[_[\x82\x81\x10\x15aP\\W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaP7V[\x86\x83\x10\x15aPyW\x84\x89\x015aPu`\x1F\x89\x16\x82aO\x8BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aP\xAA\x83\x85aP\x8FV[\x93PaP\xB7\x83\x85\x84aF\xEBV[aP\xC0\x83aE7V[\x84\x01\x90P\x93\x92PPPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81TaQ\x13\x81aM\xF2V[aQ\x1D\x81\x86aP\xF7V[\x94P`\x01\x82\x16_\x81\x14aQ7W`\x01\x81\x14aQMWaQ\x7FV[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93PaQ\x7FV[aQV\x85aN\"V[_[\x83\x81\x10\x15aQwW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90PaQXV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_aQ\x93\x83\x83aQ\x07V[\x90P\x92\x91PPV[_`\x01\x82\x01\x90P\x91\x90PV[_aQ\xB1\x82aP\xCBV[aQ\xBB\x81\x85aP\xD5V[\x93P\x83` \x82\x02\x85\x01aQ\xCD\x85aP\xE5V[\x80_[\x85\x81\x10\x15aR\x07W\x84\x84\x03\x89R\x81aQ\xE8\x85\x82aQ\x88V[\x94PaQ\xF3\x83aQ\x9BV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaQ\xD0V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaR2\x81\x85\x87aP\x9FV[\x90P\x81\x81\x03` \x83\x01RaRF\x81\x84aQ\xA7V[\x90P\x94\x93PPPPV[_\x81\x90P\x92\x91PPV[_aRd\x82aE\x0FV[aRn\x81\x85aRPV[\x93PaR~\x81\x85` \x86\x01aE)V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aR\xBE`\x02\x83aRPV[\x91PaR\xC9\x82aR\x8AV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aS\x08`\x01\x83aRPV[\x91PaS\x13\x82aR\xD4V[`\x01\x82\x01\x90P\x91\x90PV[_aS)\x82\x87aRZV[\x91PaS4\x82aR\xB2V[\x91PaS@\x82\x86aRZV[\x91PaSK\x82aR\xFCV[\x91PaSW\x82\x85aRZV[\x91PaSb\x82aR\xFCV[\x91PaSn\x82\x84aRZV[\x91P\x81\x90P\x95\x94PPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[__\xFD[\x82\x81\x837PPPV[_aS\xA4\x83\x85aS|V[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aS\xD7WaS\xD6aS\x8CV[[` \x83\x02\x92PaS\xE8\x83\x85\x84aS\x90V[\x82\x84\x01\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT\r\x81\x84\x86aS\x99V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aT0WaT/aFCV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[__\xFD[aTR\x81aG\xC1V[\x81\x14aT\\W__\xFD[PV[_\x81Q\x90PaTm\x81aTIV[\x92\x91PPV[_\x81Q\x90PaT\x81\x81aC\xFFV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aT\xA1WaT\xA0aFCV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81Q\x90PaT\xC0\x81aC\x0EV[\x92\x91PPV[_aT\xD8aT\xD3\x84aT\x87V[aF\xA1V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aT\xFBWaT\xFAaC@V[[\x83[\x81\x81\x10\x15aU$W\x80aU\x10\x88\x82aT\xB2V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaT\xFDV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aUBWaUAaC8V[[\x81QaUR\x84\x82` \x86\x01aT\xC6V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aUpWaUoaTAV[[aUz`\x80aF\xA1V[\x90P_aU\x89\x84\x82\x85\x01aT_V[_\x83\x01RP` aU\x9C\x84\x82\x85\x01aTsV[` \x83\x01RP`@aU\xB0\x84\x82\x85\x01aT_V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xD4WaU\xD3aTEV[[aU\xE0\x84\x82\x85\x01aU.V[``\x83\x01RP\x92\x91PPV[_aU\xFEaU\xF9\x84aT\x16V[aF\xA1V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aV!WaV aC@V[[\x83[\x81\x81\x10\x15aVhW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aVFWaVEaC8V[[\x80\x86\x01aVS\x89\x82aU[V[\x85R` \x85\x01\x94PPP` \x81\x01\x90PaV#V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\x86WaV\x85aC8V[[\x81QaV\x96\x84\x82` \x86\x01aU\xECV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aV\xB4WaV\xB3aB\xD6V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xD1WaV\xD0aB\xDAV[[aV\xDD\x84\x82\x85\x01aVrV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aW\x1D\x82aC\xF6V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aWOWaWNaV\xE6V[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aW\x8C\x81aG\xC1V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aW\xC4\x81aB\xFDV[\x82RPPV[_aW\xD5\x83\x83aW\xBBV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aW\xF7\x82aW\x92V[aX\x01\x81\x85aW\x9CV[\x93PaX\x0C\x83aW\xACV[\x80_[\x83\x81\x10\x15aX<W\x81QaX#\x88\x82aW\xCAV[\x97PaX.\x83aW\xE1V[\x92PP`\x01\x81\x01\x90PaX\x0FV[P\x85\x93PPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01QaX^_\x86\x01\x82aW\x83V[P` \x83\x01QaXq` \x86\x01\x82aKEV[P`@\x83\x01QaX\x84`@\x86\x01\x82aW\x83V[P``\x83\x01Q\x84\x82\x03``\x86\x01RaX\x9C\x82\x82aW\xEDV[\x91PP\x80\x91PP\x92\x91PPV[_aX\xB4\x83\x83aXIV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aX\xD2\x82aWZV[aX\xDC\x81\x85aWdV[\x93P\x83` \x82\x02\x85\x01aX\xEE\x85aWtV[\x80_[\x85\x81\x10\x15aY)W\x84\x84\x03\x89R\x81QaY\n\x85\x82aX\xA9V[\x94PaY\x15\x83aX\xBCV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaX\xF1V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYS\x81\x84aX\xC8V[\x90P\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[aYp\x81aY[V[\x82RPPV[_`@\x82\x01\x90PaY\x89_\x83\x01\x85aYgV[aY\x96` \x83\x01\x84aJ\xFEV[\x93\x92PPPV[_`@\x82\x84\x03\x12\x15aY\xB2WaY\xB1aTAV[[aY\xBC`@aF\xA1V[\x90P_aY\xCB\x84\x82\x85\x01aD\x15V[_\x83\x01RP` aY\xDE\x84\x82\x85\x01aD\x15V[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15aY\xFFWaY\xFEaB\xD6V[[_aZ\x0C\x84\x82\x85\x01aY\x9DV[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aZ<` \x84\x01\x84aC$V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aZ[\x83\x85aZ\x15V[\x93PaZf\x82aZ%V[\x80_[\x85\x81\x10\x15aZ\x9EWaZ{\x82\x84aZ.V[aZ\x85\x88\x82aW\xCAV[\x97PaZ\x90\x83aZDV[\x92PP`\x01\x81\x01\x90PaZiV[P\x85\x92PPP\x93\x92PPPV[_`@\x82\x01\x90PaZ\xBE_\x83\x01\x86aK\rV[\x81\x81\x03` \x83\x01RaZ\xD1\x81\x84\x86aZPV[\x90P\x94\x93PPPPV[`@\x82\x01aZ\xEB_\x83\x01\x83aZ.V[aZ\xF7_\x85\x01\x82aW\xBBV[Pa[\x05` \x83\x01\x83aZ.V[a[\x12` \x85\x01\x82aW\xBBV[PPPPV[_`\x80\x82\x01\x90Pa[+_\x83\x01\x87aJ\xFEV[a[8` \x83\x01\x86aZ\xDBV[\x81\x81\x03``\x83\x01Ra[K\x81\x84\x86aZPV[\x90P\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a[z\x83\x83aW\x83V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[\x9C\x82a[VV[a[\xA6\x81\x85aS|V[\x93Pa[\xB1\x83a[`V[\x80_[\x83\x81\x10\x15a[\xE1W\x81Qa[\xC8\x88\x82a[oV[\x97Pa[\xD3\x83a[\x86V[\x92PP`\x01\x81\x01\x90Pa[\xB4V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\\\x06\x81\x84a[\x92V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[a\\!\x82a\\\x0EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\:Wa\\9aFCV[[a\\D\x82TaM\xF2V[a\\O\x82\x82\x85aO9V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a\\\x80W_\x84\x15a\\nW\x82\x87\x01Q\x90P[a\\x\x85\x82aO\xA7V[\x86UPa\\\xDFV[`\x1F\x19\x84\x16a\\\x8E\x86aN\"V[_[\x82\x81\x10\x15a\\\xB5W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\\\x90V[\x86\x83\x10\x15a\\\xD2W\x84\x89\x01Qa\\\xCE`\x1F\x89\x16\x82aO\x8BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\\\xFF\x81\x87aX\xC8V[\x90Pa]\x0E` \x83\x01\x86aK\rV[\x81\x81\x03`@\x83\x01Ra]!\x81\x84\x86aP\x9FV[\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a]H\x81a],V[\x82RPPV[_` \x82\x01\x90Pa]a_\x83\x01\x84a]?V[\x92\x91PPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a]\x9B`\x15\x83aE\x19V[\x91Pa]\xA6\x82a]gV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xC8\x81a]\x8FV[\x90P\x91\x90PV[_` \x82\x01\x90Pa]\xE2_\x83\x01\x84aJ\xFEV[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_a^\x1A\x82a]\xE8V[a^$\x81\x85aP\xD5V[\x93P\x83` \x82\x02\x85\x01a^6\x85a]\xF2V[\x80_[\x85\x81\x10\x15a^pW\x84\x84\x03\x89R\x81a^Q\x85\x82aQ\x88V[\x94Pa^\\\x83a^\x04V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa^9V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\x9A\x81\x85a^\x10V[\x90P\x81\x81\x03` \x83\x01Ra^\xAE\x81\x84aQ\xA7V[\x90P\x93\x92PPPV[_\x81\x90P\x92\x91PPV[a^\xCA\x81aG\xC1V[\x82RPPV[_a^\xDB\x83\x83a^\xC1V[` \x83\x01\x90P\x92\x91PPV[_a^\xF1\x82a[VV[a^\xFB\x81\x85a^\xB7V[\x93Pa_\x06\x83a[`V[\x80_[\x83\x81\x10\x15a_6W\x81Qa_\x1D\x88\x82a^\xD0V[\x97Pa_(\x83a[\x86V[\x92PP`\x01\x81\x01\x90Pa_\tV[P\x85\x93PPPP\x92\x91PPV[_a_N\x82\x84a^\xE7V[\x91P\x81\x90P\x92\x91PPV[_``\x82\x01\x90Pa_l_\x83\x01\x86aG\xCAV[a_y` \x83\x01\x85aG\xCAV[a_\x86`@\x83\x01\x84aG\xCAV[\x94\x93PPPPV[_`@\x82\x01\x90Pa_\xA1_\x83\x01\x85aJ\xFEV[a_\xAE` \x83\x01\x84aK\rV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a_\xCAWa_\xC9aB\xD6V[[_a_\xD7\x84\x82\x85\x01aTsV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a`\x17\x82aC\xF6V[\x91Pa`\"\x83aC\xF6V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a`:Wa`9aV\xE6V[[\x92\x91PPV[_`@\x82\x01\x90Pa`S_\x83\x01\x85aJ\xFEV[a``` \x83\x01\x84aJ\xFEV[\x93\x92PPPV[_`\x80\x83\x01_\x83\x01Qa`|_\x86\x01\x82aW\x83V[P` \x83\x01Qa`\x8F` \x86\x01\x82aKEV[P`@\x83\x01Qa`\xA2`@\x86\x01\x82aW\x83V[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra`\xBA\x82\x82aW\xEDV[\x91PP\x80\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra`\xDF\x81\x85a`gV[\x90P\x81\x81\x03` \x83\x01Ra`\xF3\x81\x84a`gV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aa\x11Waa\x10aB\xD6V[[_aa\x1E\x84\x82\x85\x01aT_V[\x91PP\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_aaNaaIaaD\x84aa'V[aN\xA0V[aC\xF6V[\x90P\x91\x90PV[aa^\x81aa4V[\x82RPPV[_`@\x82\x01\x90Paaw_\x83\x01\x85aaUV[aa\x84` \x83\x01\x84aJ\xFEV[\x93\x92PPPV[_aa\x95\x82aC\xF6V[\x91Paa\xA0\x83aC\xF6V[\x92P\x82\x82\x02aa\xAE\x81aC\xF6V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aa\xC5Waa\xC4aV\xE6V[[P\x92\x91PPV[`@\x82\x01_\x82\x01Qaa\xE0_\x85\x01\x82aKEV[P` \x82\x01Qaa\xF3` \x85\x01\x82aKEV[PPPPV[_``\x82\x01\x90Pab\x0C_\x83\x01\x85aJ\xFEV[ab\x19` \x83\x01\x84aa\xCCV[\x93\x92PPPV[_ab*\x82aW\x92V[ab4\x81\x85aZ\x15V[\x93Pab?\x83aW\xACV[\x80_[\x83\x81\x10\x15aboW\x81QabV\x88\x82aW\xCAV[\x97Paba\x83aW\xE1V[\x92PP`\x01\x81\x01\x90PabBV[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90Pab\x8F_\x83\x01\x85aK\rV[\x81\x81\x03` \x83\x01Rab\xA1\x81\x84ab V[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rab\xC3\x81\x84\x86aP\x9FV[\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pab\xDF_\x83\x01\x87aG\xCAV[ab\xEC` \x83\x01\x86aG\xCAV[ab\xF9`@\x83\x01\x85aG\xCAV[ac\x06``\x83\x01\x84aG\xCAV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x01\x90PacO_\x83\x01\x84aYgV[\x92\x91PPV[`T\x81\x10acfWaceac\x0FV[[PV[_\x81\x90Pacv\x82acUV[\x91\x90PV[_ac\x85\x82aciV[\x90P\x91\x90PV[ac\x95\x81ac{V[\x82RPPV[_` \x82\x01\x90Pac\xAE_\x83\x01\x84ac\x8CV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[ac\xC7\x81aB\xFDV[\x82RPPV[_ac\xD8\x83\x83ac\xBEV[` \x83\x01\x90P\x92\x91PPV[_ac\xEE\x82aW\x92V[ac\xF8\x81\x85ac\xB4V[\x93Pad\x03\x83aW\xACV[\x80_[\x83\x81\x10\x15ad3W\x81Qad\x1A\x88\x82ac\xCDV[\x97Pad%\x83aW\xE1V[\x92PP`\x01\x81\x01\x90Pad\x06V[P\x85\x93PPPP\x92\x91PPV[_adK\x82\x84ac\xE4V[\x91P\x81\x90P\x92\x91PPV[_`\xE0\x82\x01\x90Padi_\x83\x01\x8AaG\xCAV[adv` \x83\x01\x89aG\xCAV[ad\x83`@\x83\x01\x88aG\xCAV[ad\x90``\x83\x01\x87aK\rV[ad\x9D`\x80\x83\x01\x86aJ\xFEV[ad\xAA`\xA0\x83\x01\x85aJ\xFEV[ad\xB7`\xC0\x83\x01\x84aJ\xFEV[\x98\x97PPPPPPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ae\x16Wad\xE7\x81ad\xC3V[ad\xF0\x84aN4V[\x81\x01` \x85\x10\x15ad\xFFW\x81\x90P[ae\x13ae\x0B\x85aN4V[\x83\x01\x82aO\x17V[PP[PPPV[ae$\x82aE\x0FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ae=Wae<aFCV[[aeG\x82TaM\xF2V[aeR\x82\x82\x85ad\xD5V[_` \x90P`\x1F\x83\x11`\x01\x81\x14ae\x83W_\x84\x15aeqW\x82\x87\x01Q\x90P[ae{\x85\x82aO\xA7V[\x86UPae\xE2V[`\x1F\x19\x84\x16ae\x91\x86ad\xC3V[_[\x82\x81\x10\x15ae\xB8W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pae\x93V[\x86\x83\x10\x15ae\xD5W\x84\x89\x01Qae\xD1`\x1F\x89\x16\x82aO\x8BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\xC0\x82\x01\x90Pae\xFD_\x83\x01\x89aG\xCAV[af\n` \x83\x01\x88aG\xCAV[af\x17`@\x83\x01\x87aG\xCAV[af$``\x83\x01\x86aJ\xFEV[af1`\x80\x83\x01\x85aJ\xFEV[af>`\xA0\x83\x01\x84aJ\xFEV[\x97\x96PPPPPPPV[_\x81\x90P\x92\x91PPV[_af]\x82a\\\x0EV[afg\x81\x85afIV[\x93Pafw\x81\x85` \x86\x01aE)V[\x80\x84\x01\x91PP\x92\x91PPV[_af\x8E\x82\x84afSV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Paf\xAC_\x83\x01\x88aG\xCAV[af\xB9` \x83\x01\x87aG\xCAV[af\xC6`@\x83\x01\x86aG\xCAV[af\xD3``\x83\x01\x85aJ\xFEV[af\xE0`\x80\x83\x01\x84aK\rV[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Paf\xFD_\x83\x01\x87aG\xCAV[ag\n` \x83\x01\x86aYgV[ag\x17`@\x83\x01\x85aG\xCAV[ag$``\x83\x01\x84aG\xCAV[\x95\x94PPPPPV\xFEDelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays)UserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061011d575f3560e01c80638316001f1161009f578063ad3cb1cc11610063578063ad3cb1cc14610345578063b9bfe0a81461036f578063e30c397814610397578063f11d0638146103c1578063f2fde38b146103e95761011d565b80638316001f1461027357806384b0196e1461029b5780638da5cb5b146102cb578063a6090439146102f5578063aa39a3561461031d5761011d565b806352d1902d116100e657806352d1902d146101df578063715018a614610209578063760a04191461021f57806379ba5097146102475780638129fc1c1461025d5761011d565b80628bc3e11461012157806302fd1a64146101495780630d8e6e2c14610171578063187fe5291461019b5780634f1ef286146101c3575b5f5ffd5b34801561012c575f5ffd5b5061014760048036038101906101429190614399565b610411565b005b348015610154575f5ffd5b5061016f600480360381019061016a919061447e565b61061e565b005b34801561017c575f5ffd5b50610185610881565b604051610192919061457f565b60405180910390f35b3480156101a6575f5ffd5b506101c160048036038101906101bc91906145f4565b6108fc565b005b6101dd60048036038101906101d89190614767565b610aaa565b005b3480156101ea575f5ffd5b506101f3610ac9565b60405161020091906147d9565b60405180910390f35b348015610214575f5ffd5b5061021d610afa565b005b34801561022a575f5ffd5b5061024560048036038101906102409190614887565b610b0d565b005b348015610252575f5ffd5b5061025b610fce565b005b348015610268575f5ffd5b5061027161105c565b005b34801561027e575f5ffd5b50610299600480360381019061029491906149a6565b611205565b005b3480156102a6575f5ffd5b506102af6115c3565b6040516102c29796959493929190614bd3565b60405180910390f35b3480156102d6575f5ffd5b506102df6116cc565b6040516102ec9190614c55565b60405180910390f35b348015610300575f5ffd5b5061031b60048036038101906103169190614c6e565b611701565b005b348015610328575f5ffd5b50610343600480360381019061033e91906145f4565b611771565b005b348015610350575f5ffd5b506103596118b7565b604051610366919061457f565b60405180910390f35b34801561037a575f5ffd5b506103956004803603810190610390919061447e565b6118f0565b005b3480156103a2575f5ffd5b506103ab611c55565b6040516103b89190614c55565b60405180910390f35b3480156103cc575f5ffd5b506103e760048036038101906103e29190614c99565b611c8a565b005b3480156103f4575f5ffd5b5061040f600480360381019061040a9190614d3c565b611f2a565b005b5f5f90505b828290508110156106185773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84848481811061046457610463614d67565b5b9050604002015f0135866040518363ffffffff1660e01b815260040161048b929190614d94565b5f6040518083038186803b1580156104a1575f5ffd5b505afa1580156104b3573d5f5f3e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d8484848181106104fa576104f9614d67565b5b9050604002015f013585858581811061051657610515614d67565b5b905060400201602001602081019061052e9190614d3c565b6040518363ffffffff1660e01b815260040161054b929190614d94565b5f6040518083038186803b158015610561575f5ffd5b505afa158015610573573d5f5f3e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f638484848181106105ba576105b9614d67565b5b9050604002015f01356040518263ffffffff1660e01b81526004016105df91906147d9565b5f6040518083038186803b1580156105f5575f5ffd5b505afa158015610607573d5f5f3e3d5ffd5b505050508080600101915050610416565b50505050565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b815260040161066b9190614c55565b5f6040518083038186803b158015610681575f5ffd5b505afa158015610693573d5f5f3e3d5ffd5b505050505f6106a0611fe3565b90505f6040518060400160405280836004015f8a81526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561070957602002820191905f5260205f20905b8154815260200190600101908083116106f5575b5050505050815260200187878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f6107668261200a565b905061077488828787612098565b5f836003015f8a81526020019081526020015f205f8381526020019081526020015f20905080868690918060018154018082558091505060019003905f5260205f20015f9091929091929091929091925091826107d2929190614fc2565b50836001015f8a81526020019081526020015f205f9054906101000a900460ff1615801561080957506108088180549050612279565b5b15610876576001846001015f8b81526020019081526020015f205f6101000a81548160ff021916908315150217905550887f61568d6eb48e62870afffd55499206a54a8f78b04a627e00ed097161fc05d6be89898460405161086d93929190615219565b60405180910390a25b505050505050505050565b60606040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506108c25f61230a565b6108cc600161230a565b6108d55f61230a565b6040516020016108e8949392919061531e565b604051602081830303815290604052905090565b5f8282905003610938576040517f2de7543800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6109818282808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050506123d4565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f897184846040518363ffffffff1660e01b81526004016109d19291906153f4565b5f60405180830381865afa1580156109eb573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190610a13919061569f565b9050610a1e81612502565b5f610a27611fe3565b9050805f015f815480929190610a3c90615713565b91905055505f815f015490508484836004015f8481526020019081526020015f209190610a6a92919061421c565b50807f17c632196fbf6b96d9675971058d3701733094c3f2f1dcb9ba7d2a08bee0aafb84604051610a9b919061593b565b60405180910390a25050505050565b610ab26125e8565b610abb826126ce565b610ac582826126d9565b5050565b5f610ad26127f7565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b610b0261287e565b610b0b5f612905565b565b600a60ff16868690501115610b5f57600a868690506040517fc5ab467e000000000000000000000000000000000000000000000000000000008152600401610b56929190615976565b60405180910390fd5b610b7889803603810190610b7391906159ea565b612942565b610bd38686808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f82011690508083019250505050505050895f016020810190610bce9190614d3c565b612a8d565b15610c2a57875f016020810190610bea9190614d3c565b86866040517fc3446ac7000000000000000000000000000000000000000000000000000000008152600401610c2193929190615aab565b60405180910390fd5b5f610c888c8c8989808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f820116905080830192505050505050508c5f016020810190610c839190614d3c565b612b0b565b905073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e898b8a8a6040518563ffffffff1660e01b8152600401610cdd9493929190615b18565b5f6040518083038186803b158015610cf3575f5ffd5b505afa158015610d05573d5f5f3e3d5ffd5b505050505f6040518060c0016040528087878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018989808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b5f016020810190610db69190614d3c565b73ffffffffffffffffffffffffffffffffffffffff1681526020018a81526020018c5f013581526020018c602001358152509050610e08818b6020016020810190610e019190614d3c565b8686612de6565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b8152600401610e569190615bee565b5f60405180830381865afa158015610e70573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190610e98919061569f565b9050610ea381612502565b5f610eac611fe3565b9050805f015f815480929190610ec190615713565b91905055505f815f0154905060405180604001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f019081610f4b9190615c18565b506020820151816001019080519060200190610f68929190614267565b50905050807f1c3dcad6311be6d58dc4d4b9f1bc1625eb18d72de969db75e11a88ef3527d2f3848f6020016020810190610fa29190614d3c565b8c8c604051610fb49493929190615ce7565b60405180910390a250505050505050505050505050505050565b5f610fd7612ebc565b90508073ffffffffffffffffffffffffffffffffffffffff16610ff8611c55565b73ffffffffffffffffffffffffffffffffffffffff161461105057806040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016110479190614c55565b60405180910390fd5b61105981612905565b50565b60025f611067612ec3565b9050805f0160089054906101000a900460ff16806110af57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b156110e6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff02191690831515021790555061119f6040518060400160405280600a81526020017f44656372797074696f6e000000000000000000000000000000000000000000008152506040518060400160405280600181526020017f3100000000000000000000000000000000000000000000000000000000000000815250612eea565b6111af6111aa6116cc565b612f00565b5f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516111f99190615d4e565b60405180910390a15050565b600a60ff1687879050111561125757600a878790506040517fc5ab467e00000000000000000000000000000000000000000000000000000000815260040161124e929190615976565b60405180910390fd5b6112708980360381019061126b91906159ea565b612942565b6112ba8787808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505086612a8d565b15611300578487876040517fdc4d78b10000000000000000000000000000000000000000000000000000000081526004016112f793929190615aab565b60405180910390fd5b5f61134d8c8c8a8a808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505089612b0b565b90505f6040518060a0016040528087878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081526020018a8a808060200260200160405190810160405280939291908181526020018383602002808284375f81840152601f19601f8201169050808301925050505050505081526020018b81526020018c5f013581526020018c60200135815250905061140f81888686612f14565b5f73d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663a14f8971846040518263ffffffff1660e01b815260040161145d9190615bee565b5f60405180830381865afa158015611477573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061149f919061569f565b90506114aa81612502565b5f6114b3611fe3565b9050805f015f8154809291906114c890615713565b91905055505f815f0154905060405180604001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f82011690508083019250505050505050815260200186815250826006015f8381526020019081526020015f205f820151815f0190816115529190615c18565b50602082015181600101908051906020019061156f929190614267565b50905050807f1c3dcad6311be6d58dc4d4b9f1bc1625eb18d72de969db75e11a88ef3527d2f3848c8c8c6040516115a99493929190615ce7565b60405180910390a250505050505050505050505050505050565b5f6060805f5f5f60605f6115d5612fea565b90505f5f1b815f01541480156115f057505f5f1b8160010154145b61162f576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161162690615db1565b60405180910390fd5b611637613011565b61163f6130af565b46305f5f1b5f67ffffffffffffffff81111561165e5761165d614643565b5b60405190808252806020026020018201604052801561168c5781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b5f5f6116d661314d565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b5f61170a611fe3565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff1661176d57816040517f0bf014060000000000000000000000000000000000000000000000000000000081526004016117649190615dcf565b60405180910390fd5b5050565b5f5f90505b828290508110156118b25773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c8484848181106117c4576117c3614d67565b5b905060200201356040518263ffffffff1660e01b81526004016117e791906147d9565b5f6040518083038186803b1580156117fd575f5ffd5b505afa15801561180f573d5f5f3e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f6384848481811061185657611855614d67565b5b905060200201356040518263ffffffff1660e01b815260040161187991906147d9565b5f6040518083038186803b15801561188f575f5ffd5b505afa1580156118a1573d5f5f3e3d5ffd5b505050508080600101915050611776565b505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663c6275258336040518263ffffffff1660e01b815260040161193d9190614c55565b5f6040518083038186803b158015611953575f5ffd5b505afa158015611965573d5f5f3e3d5ffd5b505050505f611972611fe3565b90505f816006015f8881526020019081526020015f206040518060400160405290815f820180546119a290614df2565b80601f01602080910402602001604051908101604052809291908181526020018280546119ce90614df2565b8015611a195780601f106119f057610100808354040283529160200191611a19565b820191905f5260205f20905b8154815290600101906020018083116119fc57829003601f168201915b5050505050815260200160018201805480602002602001604051908101604052809291908181526020018280548015611a6f57602002820191905f5260205f20905b815481526020019060010190808311611a5b575b50505050508152505090505f6040518060600160405280835f015181526020018360200151815260200188888080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f8201169050808301925050505050505081525090505f611aec82613174565b9050611afa89828888612098565b5f846005015f8b81526020019081526020015f20905080878790918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182611b49929190614fc2565b50846008015f8b81526020019081526020015f20898990918060018154018082558091505060019003905f5260205f20015f909192909192909192909192509182611b95929190614fc2565b50846001015f8b81526020019081526020015f205f9054906101000a900460ff16158015611bcc5750611bcb818054905061320f565b5b15611c49576001856001015f8c81526020019081526020015f205f6101000a81548160ff021916908315150217905550897f7312dec4cead0d5d3da836cdbaed1eb6a81e218c519c8740da4ac75afcb6c5c7866008015f8d81526020019081526020015f2083604051611c40929190615e82565b60405180910390a25b50505050505050505050565b5f5f611c5f6132a0565b9050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1691505090565b73a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff166351c41d0e878785856040518563ffffffff1660e01b8152600401611cdd9493929190615b18565b5f6040518083038186803b158015611cf3575f5ffd5b505afa158015611d05573d5f5f3e3d5ffd5b505050505f5f90505b84849050811015611f215773a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d868684818110611d5c57611d5b614d67565b5b9050604002015f0135885f016020810190611d779190614d3c565b6040518363ffffffff1660e01b8152600401611d94929190614d94565b5f6040518083038186803b158015611daa575f5ffd5b505afa158015611dbc573d5f5f3e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d868684818110611e0357611e02614d67565b5b9050604002015f0135878785818110611e1f57611e1e614d67565b5b9050604002016020016020810190611e379190614d3c565b6040518363ffffffff1660e01b8152600401611e54929190614d94565b5f6040518083038186803b158015611e6a575f5ffd5b505afa158015611e7c573d5f5f3e3d5ffd5b5050505073d582ec82a1758322907df80da8a754e12a5acb9573ffffffffffffffffffffffffffffffffffffffff1663d4476f63868684818110611ec357611ec2614d67565b5b9050604002015f01356040518263ffffffff1660e01b8152600401611ee891906147d9565b5f6040518083038186803b158015611efe575f5ffd5b505afa158015611f10573d5f5f3e3d5ffd5b505050508080600101915050611d0e565b50505050505050565b611f3261287e565b5f611f3b6132a0565b905081815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508173ffffffffffffffffffffffffffffffffffffffff16611f9d6116cc565b73ffffffffffffffffffffffffffffffffffffffff167f38d16b8cac22d99fc7c124b9cd0de2d3fa1faef420bfe791d8c362d765e2270060405160405180910390a35050565b5f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700905090565b5f6120916040518060800160405280604481526020016167e06044913980519060200120835f01516040516020016120429190615f43565b6040516020818303038152906040528051906020012084602001518051906020012060405160200161207693929190615f59565b604051602081830303815290604052805190602001206132c7565b9050919050565b5f6120a1611fe3565b90505f6120f18585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506132e0565b905073c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff16636c88eb43826040518263ffffffff1660e01b81526004016121409190614c55565b5f6040518083038186803b158015612156575f5ffd5b505afa158015612168573d5f5f3e3d5ffd5b50505050816002015f8781526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561220b5785816040517f99ec48d9000000000000000000000000000000000000000000000000000000008152600401612202929190615f8e565b60405180910390fd5b6001826002015f8881526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff021916908315150217905550505050505050565b5f5f73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff166347cd4b3e6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122d8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122fc9190615fb5565b905080831015915050919050565b60605f60016123188461330a565b0190505f8167ffffffffffffffff81111561233657612335614643565b5b6040519080825280601f01601f1916602001820160405280156123685781602001600182028036833780820191505090505b5090505f82602001820190505b6001156123c9578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816123be576123bd615fe0565b5b0494505f8503612375575b819350505050919050565b5f5f90505f5f90505b82518110156124b2575f8382815181106123fa576123f9614d67565b5b602002602001015190505f61240e8261345b565b9050612419816134e5565b61ffff1684612428919061600d565b935073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff1663193f3f2c836040518263ffffffff1660e01b815260040161247791906147d9565b5f6040518083038186803b15801561248d575f5ffd5b505afa15801561249f573d5f5f3e3d5ffd5b50505050505080806001019150506123dd565b506108008111156124fe57610800816040517fe7f4895d0000000000000000000000000000000000000000000000000000000081526004016124f5929190616040565b60405180910390fd5b5050565b6001815111156125e5575f815f815181106125205761251f614d67565b5b60200260200101516020015190505f600190505b82518110156125e2578183828151811061255157612550614d67565b5b602002602001015160200151146125d557825f8151811061257557612574614d67565b5b60200260200101518382815181106125905761258f614d67565b5b60200260200101516040517fcfae921f0000000000000000000000000000000000000000000000000000000081526004016125cc9291906160c7565b60405180910390fd5b8080600101915050612534565b50505b50565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff16148061269557507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1661267c613772565b73ffffffffffffffffffffffffffffffffffffffff1614155b156126cc576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6126d661287e565b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561274157506040513d601f19601f8201168201806040525081019061273e91906160fc565b60015b61278257816040517f4c9c8ce30000000000000000000000000000000000000000000000000000000081526004016127799190614c55565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146127e857806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016127df91906147d9565b60405180910390fd5b6127f283836137c5565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461287c576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b612886612ebc565b73ffffffffffffffffffffffffffffffffffffffff166128a46116cc565b73ffffffffffffffffffffffffffffffffffffffff1614612903576128c7612ebc565b6040517f118cdaa70000000000000000000000000000000000000000000000000000000081526004016128fa9190614c55565b60405180910390fd5b565b5f61290e6132a0565b9050805f015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905561293e82613837565b5050565b5f81602001510361297f576040517fde2859c100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61016d61ffff16816020015111156129d65761016d81602001516040517f329518630000000000000000000000000000000000000000000000000000000081526004016129cd929190616164565b60405180910390fd5b42815f01511115612a235742815f01516040517ff24c0887000000000000000000000000000000000000000000000000000000008152600401612a1a929190616040565b60405180910390fd5b42620151808260200151612a37919061618b565b825f0151612a45919061600d565b1015612a8a5742816040517f30348040000000000000000000000000000000000000000000000000000000008152600401612a819291906161f9565b60405180910390fd5b50565b5f5f5f90505b8351811015612b00578273ffffffffffffffffffffffffffffffffffffffff16848281518110612ac657612ac5614d67565b5b602002602001015173ffffffffffffffffffffffffffffffffffffffff1603612af3576001915050612b05565b8080600101915050612a93565b505f90505b92915050565b60605f8585905003612b49576040517fa6a6cb2100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8484905067ffffffffffffffff811115612b6657612b65614643565b5b604051908082528060200260200182016040528015612b945781602001602082028036833780820191505090505b5090505f5f90505f5f90505b86869050811015612d91575f878783818110612bbf57612bbe614d67565b5b9050604002015f013590505f888884818110612bde57612bdd614d67565b5b9050604002016020016020810190612bf69190614d3c565b90505f612c028361345b565b9050612c0d816134e5565b61ffff1685612c1c919061600d565b945073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84896040518363ffffffff1660e01b8152600401612c6d929190614d94565b5f6040518083038186803b158015612c83575f5ffd5b505afa158015612c95573d5f5f3e3d5ffd5b5050505073a50f5243c70c80a8309e3d39d8c9d958cda8397973ffffffffffffffffffffffffffffffffffffffff16633bce498d84846040518363ffffffff1660e01b8152600401612ce8929190614d94565b5f6040518083038186803b158015612cfe575f5ffd5b505afa158015612d10573d5f5f3e3d5ffd5b50505050612d1e8883612a8d565b612d615781886040517fa4c30391000000000000000000000000000000000000000000000000000000008152600401612d5892919061627c565b60405180910390fd5b82868581518110612d7557612d74614d67565b5b6020026020010181815250505050508080600101915050612ba0565b50610800811115612ddd57610800816040517fe7f4895d000000000000000000000000000000000000000000000000000000008152600401612dd4929190616040565b60405180910390fd5b50949350505050565b5f612df085613908565b90505f612e408285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506132e0565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614612eb45783836040517f2a873d27000000000000000000000000000000000000000000000000000000008152600401612eab9291906162aa565b60405180910390fd5b505050505050565b5f33905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b612ef26139ae565b612efc82826139ee565b5050565b612f086139ae565b612f1181613a3f565b50565b5f612f1e85613ac3565b90505f612f6e8285858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506132e0565b90508473ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614612fe25783836040517f2a873d27000000000000000000000000000000000000000000000000000000008152600401612fd99291906162aa565b60405180910390fd5b505050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f61301c612fea565b905080600201805461302d90614df2565b80601f016020809104026020016040519081016040528092919081815260200182805461305990614df2565b80156130a45780601f1061307b576101008083540402835291602001916130a4565b820191905f5260205f20905b81548152906001019060200180831161308757829003601f168201915b505050505091505090565b60605f6130ba612fea565b90508060030180546130cb90614df2565b80601f01602080910402602001604051908101604052809291908181526020018280546130f790614df2565b80156131425780601f1061311957610100808354040283529160200191613142565b820191905f5260205f20905b81548152906001019060200180831161312557829003601f168201915b505050505091505090565b5f7f9016d09d72d40fdae2fd8ceac6b6234c7706214fd39c1cd1e609a0528c199300905090565b5f6132086040518060800160405280605d81526020016168b4605d913980519060200120835f01518051906020012084602001516040516020016131b89190615f43565b604051602081830303815290604052805190602001208560400151805190602001206040516020016131ed94939291906162cc565b604051602081830303815290604052805190602001206132c7565b9050919050565b5f5f73c7d45661a345ec5ca0e8521cfef7e32fda0daa6873ffffffffffffffffffffffffffffffffffffffff1663490413aa6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561326e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906132929190615fb5565b905080831015915050919050565b5f7f237e158222e3e6968b72b9db0d8043aacf074ad9f650f0d1606b4d82ee432c00905090565b5f6132d96132d3613b63565b83613b71565b9050919050565b5f5f5f5f6132ee8686613bb1565b9250925092506132fe8282613c06565b82935050505092915050565b5f5f5f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008310613366577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000838161335c5761335b615fe0565b5b0492506040810190505b6d04ee2d6d415b85acef810000000083106133a3576d04ee2d6d415b85acef8100000000838161339957613398615fe0565b5b0492506020810190505b662386f26fc1000083106133d257662386f26fc1000083816133c8576133c7615fe0565b5b0492506010810190505b6305f5e10083106133fb576305f5e10083816133f1576133f0615fe0565b5b0492506008810190505b612710831061342057612710838161341657613415615fe0565b5b0492506004810190505b60648310613443576064838161343957613438615fe0565b5b0492506002810190505b600a8310613452576001810190505b80915050919050565b5f5f60f860f084901b901c5f1c905060538081111561347d5761347c61630f565b5b60ff168160ff1611156134c757806040517f641950d70000000000000000000000000000000000000000000000000000000081526004016134be919061633c565b60405180910390fd5b8060ff1660538111156134dd576134dc61630f565b5b915050919050565b5f5f60538111156134f9576134f861630f565b5b82605381111561350c5761350b61630f565b5b0361351a576002905061376d565b6002605381111561352e5761352d61630f565b5b8260538111156135415761354061630f565b5b0361354f576008905061376d565b600360538111156135635761356261630f565b5b8260538111156135765761357561630f565b5b03613584576010905061376d565b600460538111156135985761359761630f565b5b8260538111156135ab576135aa61630f565b5b036135b9576020905061376d565b600560538111156135cd576135cc61630f565b5b8260538111156135e0576135df61630f565b5b036135ee576040905061376d565b600660538111156136025761360161630f565b5b8260538111156136155761361461630f565b5b03613623576080905061376d565b600760538111156136375761363661630f565b5b82605381111561364a5761364961630f565b5b036136585760a0905061376d565b6008605381111561366c5761366b61630f565b5b82605381111561367f5761367e61630f565b5b0361368e57610100905061376d565b600960538111156136a2576136a161630f565b5b8260538111156136b5576136b461630f565b5b036136c457610200905061376d565b600a60538111156136d8576136d761630f565b5b8260538111156136eb576136ea61630f565b5b036136fa57610400905061376d565b600b605381111561370e5761370d61630f565b5b8260538111156137215761372061630f565b5b0361373057610800905061376d565b816040517fbe7830b1000000000000000000000000000000000000000000000000000000008152600401613764919061639b565b60405180910390fd5b919050565b5f61379e7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613d68565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b6137ce82613d71565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f8151111561382a576138248282613e3a565b50613833565b613832613eba565b5b5050565b5f61384061314d565b90505f815f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905082825f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e060405160405180910390a3505050565b5f6139a76040518060e0016040528060b2815260200161672e60b2913980519060200120835f015180519060200120846020015160405160200161394c9190616440565b604051602081830303815290604052805190602001208560400151866060015187608001518860a0015160405160200161398c9796959493929190616456565b604051602081830303815290604052805190602001206132c7565b9050919050565b6139b6613ef6565b6139ec576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b6139f66139ae565b5f6139ff612fea565b905082816002019081613a12919061651b565b5081816003019081613a24919061651b565b505f5f1b815f01819055505f5f1b8160010181905550505050565b613a476139ae565b5f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603613ab7575f6040517f1e4fbdf7000000000000000000000000000000000000000000000000000000008152600401613aae9190614c55565b60405180910390fd5b613ac081612905565b50565b5f613b5c6040518060c00160405280609081526020016168246090913980519060200120835f0151805190602001208460200151604051602001613b079190616440565b60405160208183030381529060405280519060200120856040015186606001518760800151604051602001613b41969594939291906165ea565b604051602081830303815290604052805190602001206132c7565b9050919050565b5f613b6c613f14565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f5f5f6041845103613bf1575f5f5f602087015192506040870151915060608701515f1a9050613be388828585613f77565b955095509550505050613bff565b5f600285515f1b9250925092505b9250925092565b5f6003811115613c1957613c1861630f565b5b826003811115613c2c57613c2b61630f565b5b0315613d645760016003811115613c4657613c4561630f565b5b826003811115613c5957613c5861630f565b5b03613c90576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115613ca457613ca361630f565b5b826003811115613cb757613cb661630f565b5b03613cfb57805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401613cf29190615dcf565b60405180910390fd5b600380811115613d0e57613d0d61630f565b5b826003811115613d2157613d2061630f565b5b03613d6357806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401613d5a91906147d9565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03613dcc57806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401613dc39190614c55565b60405180910390fd5b80613df87f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b613d68565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f5f8473ffffffffffffffffffffffffffffffffffffffff1684604051613e639190616683565b5f60405180830381855af49150503d805f8114613e9b576040519150601f19603f3d011682016040523d82523d5f602084013e613ea0565b606091505b5091509150613eb085838361405e565b9250505092915050565b5f341115613ef4576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f613eff612ec3565b5f0160089054906101000a900460ff16905090565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f613f3e6140eb565b613f46614161565b4630604051602001613f5c959493929190616699565b60405160208183030381529060405280519060200120905090565b5f5f5f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c1115613fb3575f600385925092509250614054565b5f6001888888886040515f8152602001604052604051613fd694939291906166ea565b6020604051602081039080840390855afa158015613ff6573d5f5f3e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1603614047575f60015f5f1b93509350935050614054565b805f5f5f1b935093509350505b9450945094915050565b6060826140735761406e826141d8565b6140e3565b5f825114801561409957505f8473ffffffffffffffffffffffffffffffffffffffff163b145b156140db57836040517f9996b3150000000000000000000000000000000000000000000000000000000081526004016140d29190614c55565b60405180910390fd5b8190506140e4565b5b9392505050565b5f5f6140f5612fea565b90505f614100613011565b90505f8151111561411c5780805190602001209250505061415e565b5f825f015490505f5f1b81146141375780935050505061415e565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f5f61416b612fea565b90505f6141766130af565b90505f81511115614192578080519060200120925050506141d5565b5f826001015490505f5f1b81146141ae578093505050506141d5565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156141ea5780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b828054828255905f5260205f20908101928215614256579160200282015b8281111561425557823582559160200191906001019061423a565b5b50905061426391906142b2565b5090565b828054828255905f5260205f209081019282156142a1579160200282015b828111156142a0578251825591602001919060010190614285565b5b5090506142ae91906142b2565b5090565b5b808211156142c9575f815f9055506001016142b3565b5090565b5f604051905090565b5f5ffd5b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f614307826142de565b9050919050565b614317816142fd565b8114614321575f5ffd5b50565b5f813590506143328161430e565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f84011261435957614358614338565b5b8235905067ffffffffffffffff8111156143765761437561433c565b5b60208301915083604082028301111561439257614391614340565b5b9250929050565b5f5f5f604084860312156143b0576143af6142d6565b5b5f6143bd86828701614324565b935050602084013567ffffffffffffffff8111156143de576143dd6142da565b5b6143ea86828701614344565b92509250509250925092565b5f819050919050565b614408816143f6565b8114614412575f5ffd5b50565b5f81359050614423816143ff565b92915050565b5f5f83601f84011261443e5761443d614338565b5b8235905067ffffffffffffffff81111561445b5761445a61433c565b5b60208301915083600182028301111561447757614476614340565b5b9250929050565b5f5f5f5f5f60608688031215614497576144966142d6565b5b5f6144a488828901614415565b955050602086013567ffffffffffffffff8111156144c5576144c46142da565b5b6144d188828901614429565b9450945050604086013567ffffffffffffffff8111156144f4576144f36142da565b5b61450088828901614429565b92509250509295509295909350565b5f81519050919050565b5f82825260208201905092915050565b8281835e5f83830152505050565b5f601f19601f8301169050919050565b5f6145518261450f565b61455b8185614519565b935061456b818560208601614529565b61457481614537565b840191505092915050565b5f6020820190508181035f8301526145978184614547565b905092915050565b5f5f83601f8401126145b4576145b3614338565b5b8235905067ffffffffffffffff8111156145d1576145d061433c565b5b6020830191508360208202830111156145ed576145ec614340565b5b9250929050565b5f5f6020838503121561460a576146096142d6565b5b5f83013567ffffffffffffffff811115614627576146266142da565b5b6146338582860161459f565b92509250509250929050565b5f5ffd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61467982614537565b810181811067ffffffffffffffff8211171561469857614697614643565b5b80604052505050565b5f6146aa6142cd565b90506146b68282614670565b919050565b5f67ffffffffffffffff8211156146d5576146d4614643565b5b6146de82614537565b9050602081019050919050565b828183375f83830152505050565b5f61470b614706846146bb565b6146a1565b9050828152602081018484840111156147275761472661463f565b5b6147328482856146eb565b509392505050565b5f82601f83011261474e5761474d614338565b5b813561475e8482602086016146f9565b91505092915050565b5f5f6040838503121561477d5761477c6142d6565b5b5f61478a85828601614324565b925050602083013567ffffffffffffffff8111156147ab576147aa6142da565b5b6147b78582860161473a565b9150509250929050565b5f819050919050565b6147d3816147c1565b82525050565b5f6020820190506147ec5f8301846147ca565b92915050565b5f5ffd5b5f6040828403121561480b5761480a6147f2565b5b81905092915050565b5f60408284031215614829576148286147f2565b5b81905092915050565b5f5f83601f84011261484757614846614338565b5b8235905067ffffffffffffffff8111156148645761486361433c565b5b6020830191508360208202830111156148805761487f614340565b5b9250929050565b5f5f5f5f5f5f5f5f5f5f5f6101208c8e0312156148a7576148a66142d6565b5b5f8c013567ffffffffffffffff8111156148c4576148c36142da565b5b6148d08e828f01614344565b9b509b505060206148e38e828f016147f6565b99505060606148f48e828f01614814565b98505060a06149058e828f01614415565b97505060c08c013567ffffffffffffffff811115614926576149256142da565b5b6149328e828f01614832565b965096505060e08c013567ffffffffffffffff811115614955576149546142da565b5b6149618e828f01614429565b94509450506101008c013567ffffffffffffffff811115614985576149846142da565b5b6149918e828f01614429565b92509250509295989b509295989b9093969950565b5f5f5f5f5f5f5f5f5f5f5f6101008c8e0312156149c6576149c56142d6565b5b5f8c013567ffffffffffffffff8111156149e3576149e26142da565b5b6149ef8e828f01614344565b9b509b50506020614a028e828f016147f6565b9950506060614a138e828f01614415565b98505060808c013567ffffffffffffffff811115614a3457614a336142da565b5b614a408e828f01614832565b975097505060a0614a538e828f01614324565b95505060c08c013567ffffffffffffffff811115614a7457614a736142da565b5b614a808e828f01614429565b945094505060e08c013567ffffffffffffffff811115614aa357614aa26142da565b5b614aaf8e828f01614429565b92509250509295989b509295989b9093969950565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b614af881614ac4565b82525050565b614b07816143f6565b82525050565b614b16816142fd565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b614b4e816143f6565b82525050565b5f614b5f8383614b45565b60208301905092915050565b5f602082019050919050565b5f614b8182614b1c565b614b8b8185614b26565b9350614b9683614b36565b805f5b83811015614bc6578151614bad8882614b54565b9750614bb883614b6b565b925050600181019050614b99565b5085935050505092915050565b5f60e082019050614be65f83018a614aef565b8181036020830152614bf88189614547565b90508181036040830152614c0c8188614547565b9050614c1b6060830187614afe565b614c286080830186614b0d565b614c3560a08301856147ca565b81810360c0830152614c478184614b77565b905098975050505050505050565b5f602082019050614c685f830184614b0d565b92915050565b5f60208284031215614c8357614c826142d6565b5b5f614c9084828501614415565b91505092915050565b5f5f5f5f5f5f60a08789031215614cb357614cb26142d6565b5b5f614cc089828a01614415565b9650506020614cd189828a01614814565b955050606087013567ffffffffffffffff811115614cf257614cf16142da565b5b614cfe89828a01614344565b9450945050608087013567ffffffffffffffff811115614d2157614d206142da565b5b614d2d89828a01614832565b92509250509295509295509295565b5f60208284031215614d5157614d506142d6565b5b5f614d5e84828501614324565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f604082019050614da75f8301856147ca565b614db46020830184614b0d565b9392505050565b5f82905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f6002820490506001821680614e0957607f821691505b602082108103614e1c57614e1b614dc5565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f60088302614e7e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82614e43565b614e888683614e43565b95508019841693508086168417925050509392505050565b5f819050919050565b5f614ec3614ebe614eb9846143f6565b614ea0565b6143f6565b9050919050565b5f819050919050565b614edc83614ea9565b614ef0614ee882614eca565b848454614e4f565b825550505050565b5f5f905090565b614f07614ef8565b614f12818484614ed3565b505050565b5b81811015614f3557614f2a5f82614eff565b600181019050614f18565b5050565b601f821115614f7a57614f4b81614e22565b614f5484614e34565b81016020851015614f63578190505b614f77614f6f85614e34565b830182614f17565b50505b505050565b5f82821c905092915050565b5f614f9a5f1984600802614f7f565b1980831691505092915050565b5f614fb28383614f8b565b9150826002028217905092915050565b614fcc8383614dbb565b67ffffffffffffffff811115614fe557614fe4614643565b5b614fef8254614df2565b614ffa828285614f39565b5f601f831160018114615027575f8415615015578287013590505b61501f8582614fa7565b865550615086565b601f19841661503586614e22565b5f5b8281101561505c57848901358255600182019150602085019450602081019050615037565b868310156150795784890135615075601f891682614f8b565b8355505b6001600288020188555050505b50505050505050565b5f82825260208201905092915050565b5f6150aa838561508f565b93506150b78385846146eb565b6150c083614537565b840190509392505050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f82825260208201905092915050565b5f815461511381614df2565b61511d81866150f7565b9450600182165f8114615137576001811461514d5761517f565b60ff19831686528115156020028601935061517f565b61515685614e22565b5f5b8381101561517757815481890152600182019150602081019050615158565b808801955050505b50505092915050565b5f6151938383615107565b905092915050565b5f600182019050919050565b5f6151b1826150cb565b6151bb81856150d5565b9350836020820285016151cd856150e5565b805f5b85811015615207578484038952816151e88582615188565b94506151f38361519b565b925060208a019950506001810190506151d0565b50829750879550505050505092915050565b5f6040820190508181035f83015261523281858761509f565b9050818103602083015261524681846151a7565b9050949350505050565b5f81905092915050565b5f6152648261450f565b61526e8185615250565b935061527e818560208601614529565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6152be600283615250565b91506152c98261528a565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f615308600183615250565b9150615313826152d4565b600182019050919050565b5f615329828761525a565b9150615334826152b2565b9150615340828661525a565b915061534b826152fc565b9150615357828561525a565b9150615362826152fc565b915061536e828461525a565b915081905095945050505050565b5f82825260208201905092915050565b5f5ffd5b82818337505050565b5f6153a4838561537c565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8311156153d7576153d661538c565b5b6020830292506153e8838584615390565b82840190509392505050565b5f6020820190508181035f83015261540d818486615399565b90509392505050565b5f67ffffffffffffffff8211156154305761542f614643565b5b602082029050602081019050919050565b5f5ffd5b5f5ffd5b615452816147c1565b811461545c575f5ffd5b50565b5f8151905061546d81615449565b92915050565b5f81519050615481816143ff565b92915050565b5f67ffffffffffffffff8211156154a1576154a0614643565b5b602082029050602081019050919050565b5f815190506154c08161430e565b92915050565b5f6154d86154d384615487565b6146a1565b905080838252602082019050602084028301858111156154fb576154fa614340565b5b835b81811015615524578061551088826154b2565b8452602084019350506020810190506154fd565b5050509392505050565b5f82601f83011261554257615541614338565b5b81516155528482602086016154c6565b91505092915050565b5f608082840312156155705761556f615441565b5b61557a60806146a1565b90505f6155898482850161545f565b5f83015250602061559c84828501615473565b60208301525060406155b08482850161545f565b604083015250606082015167ffffffffffffffff8111156155d4576155d3615445565b5b6155e08482850161552e565b60608301525092915050565b5f6155fe6155f984615416565b6146a1565b9050808382526020820190506020840283018581111561562157615620614340565b5b835b8181101561566857805167ffffffffffffffff81111561564657615645614338565b5b808601615653898261555b565b85526020850194505050602081019050615623565b5050509392505050565b5f82601f83011261568657615685614338565b5b81516156968482602086016155ec565b91505092915050565b5f602082840312156156b4576156b36142d6565b5b5f82015167ffffffffffffffff8111156156d1576156d06142da565b5b6156dd84828501615672565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61571d826143f6565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff820361574f5761574e6156e6565b5b600182019050919050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b61578c816147c1565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b6157c4816142fd565b82525050565b5f6157d583836157bb565b60208301905092915050565b5f602082019050919050565b5f6157f782615792565b615801818561579c565b935061580c836157ac565b805f5b8381101561583c57815161582388826157ca565b975061582e836157e1565b92505060018101905061580f565b5085935050505092915050565b5f608083015f83015161585e5f860182615783565b5060208301516158716020860182614b45565b5060408301516158846040860182615783565b506060830151848203606086015261589c82826157ed565b9150508091505092915050565b5f6158b48383615849565b905092915050565b5f602082019050919050565b5f6158d28261575a565b6158dc8185615764565b9350836020820285016158ee85615774565b805f5b85811015615929578484038952815161590a85826158a9565b9450615915836158bc565b925060208a019950506001810190506158f1565b50829750879550505050505092915050565b5f6020820190508181035f83015261595381846158c8565b905092915050565b5f60ff82169050919050565b6159708161595b565b82525050565b5f6040820190506159895f830185615967565b6159966020830184614afe565b9392505050565b5f604082840312156159b2576159b1615441565b5b6159bc60406146a1565b90505f6159cb84828501614415565b5f8301525060206159de84828501614415565b60208301525092915050565b5f604082840312156159ff576159fe6142d6565b5b5f615a0c8482850161599d565b91505092915050565b5f82825260208201905092915050565b5f819050919050565b5f615a3c6020840184614324565b905092915050565b5f602082019050919050565b5f615a5b8385615a15565b9350615a6682615a25565b805f5b85811015615a9e57615a7b8284615a2e565b615a8588826157ca565b9750615a9083615a44565b925050600181019050615a69565b5085925050509392505050565b5f604082019050615abe5f830186614b0d565b8181036020830152615ad1818486615a50565b9050949350505050565b60408201615aeb5f830183615a2e565b615af75f8501826157bb565b50615b056020830183615a2e565b615b1260208501826157bb565b50505050565b5f608082019050615b2b5f830187614afe565b615b386020830186615adb565b8181036060830152615b4b818486615a50565b905095945050505050565b5f81519050919050565b5f819050602082019050919050565b5f615b7a8383615783565b60208301905092915050565b5f602082019050919050565b5f615b9c82615b56565b615ba6818561537c565b9350615bb183615b60565b805f5b83811015615be1578151615bc88882615b6f565b9750615bd383615b86565b925050600181019050615bb4565b5085935050505092915050565b5f6020820190508181035f830152615c068184615b92565b905092915050565b5f81519050919050565b615c2182615c0e565b67ffffffffffffffff811115615c3a57615c39614643565b5b615c448254614df2565b615c4f828285614f39565b5f60209050601f831160018114615c80575f8415615c6e578287015190505b615c788582614fa7565b865550615cdf565b601f198416615c8e86614e22565b5f5b82811015615cb557848901518255600182019150602085019450602081019050615c90565b86831015615cd25784890151615cce601f891682614f8b565b8355505b6001600288020188555050505b505050505050565b5f6060820190508181035f830152615cff81876158c8565b9050615d0e6020830186614b0d565b8181036040830152615d2181848661509f565b905095945050505050565b5f67ffffffffffffffff82169050919050565b615d4881615d2c565b82525050565b5f602082019050615d615f830184615d3f565b92915050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f615d9b601583614519565b9150615da682615d67565b602082019050919050565b5f6020820190508181035f830152615dc881615d8f565b9050919050565b5f602082019050615de25f830184614afe565b92915050565b5f81549050919050565b5f819050815f5260205f209050919050565b5f600182019050919050565b5f615e1a82615de8565b615e2481856150d5565b935083602082028501615e3685615df2565b805f5b85811015615e7057848403895281615e518582615188565b9450615e5c83615e04565b925060208a01995050600181019050615e39565b50829750879550505050505092915050565b5f6040820190508181035f830152615e9a8185615e10565b90508181036020830152615eae81846151a7565b90509392505050565b5f81905092915050565b615eca816147c1565b82525050565b5f615edb8383615ec1565b60208301905092915050565b5f615ef182615b56565b615efb8185615eb7565b9350615f0683615b60565b805f5b83811015615f36578151615f1d8882615ed0565b9750615f2883615b86565b925050600181019050615f09565b5085935050505092915050565b5f615f4e8284615ee7565b915081905092915050565b5f606082019050615f6c5f8301866147ca565b615f7960208301856147ca565b615f8660408301846147ca565b949350505050565b5f604082019050615fa15f830185614afe565b615fae6020830184614b0d565b9392505050565b5f60208284031215615fca57615fc96142d6565b5b5f615fd784828501615473565b91505092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f616017826143f6565b9150616022836143f6565b925082820190508082111561603a576160396156e6565b5b92915050565b5f6040820190506160535f830185614afe565b6160606020830184614afe565b9392505050565b5f608083015f83015161607c5f860182615783565b50602083015161608f6020860182614b45565b5060408301516160a26040860182615783565b50606083015184820360608601526160ba82826157ed565b9150508091505092915050565b5f6040820190508181035f8301526160df8185616067565b905081810360208301526160f38184616067565b90509392505050565b5f60208284031215616111576161106142d6565b5b5f61611e8482850161545f565b91505092915050565b5f61ffff82169050919050565b5f61614e61614961614484616127565b614ea0565b6143f6565b9050919050565b61615e81616134565b82525050565b5f6040820190506161775f830185616155565b6161846020830184614afe565b9392505050565b5f616195826143f6565b91506161a0836143f6565b92508282026161ae816143f6565b915082820484148315176161c5576161c46156e6565b5b5092915050565b604082015f8201516161e05f850182614b45565b5060208201516161f36020850182614b45565b50505050565b5f60608201905061620c5f830185614afe565b61621960208301846161cc565b9392505050565b5f61622a82615792565b6162348185615a15565b935061623f836157ac565b805f5b8381101561626f57815161625688826157ca565b9750616261836157e1565b925050600181019050616242565b5085935050505092915050565b5f60408201905061628f5f830185614b0d565b81810360208301526162a18184616220565b90509392505050565b5f6020820190508181035f8301526162c381848661509f565b90509392505050565b5f6080820190506162df5f8301876147ca565b6162ec60208301866147ca565b6162f960408301856147ca565b61630660608301846147ca565b95945050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b5f60208201905061634f5f830184615967565b92915050565b605481106163665761636561630f565b5b50565b5f81905061637682616355565b919050565b5f61638582616369565b9050919050565b6163958161637b565b82525050565b5f6020820190506163ae5f83018461638c565b92915050565b5f81905092915050565b6163c7816142fd565b82525050565b5f6163d883836163be565b60208301905092915050565b5f6163ee82615792565b6163f881856163b4565b9350616403836157ac565b805f5b8381101561643357815161641a88826163cd565b9750616425836157e1565b925050600181019050616406565b5085935050505092915050565b5f61644b82846163e4565b915081905092915050565b5f60e0820190506164695f83018a6147ca565b61647660208301896147ca565b61648360408301886147ca565b6164906060830187614b0d565b61649d6080830186614afe565b6164aa60a0830185614afe565b6164b760c0830184614afe565b98975050505050505050565b5f819050815f5260205f209050919050565b601f821115616516576164e7816164c3565b6164f084614e34565b810160208510156164ff578190505b61651361650b85614e34565b830182614f17565b50505b505050565b6165248261450f565b67ffffffffffffffff81111561653d5761653c614643565b5b6165478254614df2565b6165528282856164d5565b5f60209050601f831160018114616583575f8415616571578287015190505b61657b8582614fa7565b8655506165e2565b601f198416616591866164c3565b5f5b828110156165b857848901518255600182019150602085019450602081019050616593565b868310156165d557848901516165d1601f891682614f8b565b8355505b6001600288020188555050505b505050505050565b5f60c0820190506165fd5f8301896147ca565b61660a60208301886147ca565b61661760408301876147ca565b6166246060830186614afe565b6166316080830185614afe565b61663e60a0830184614afe565b979650505050505050565b5f81905092915050565b5f61665d82615c0e565b6166678185616649565b9350616677818560208601614529565b80840191505092915050565b5f61668e8284616653565b915081905092915050565b5f60a0820190506166ac5f8301886147ca565b6166b960208301876147ca565b6166c660408301866147ca565b6166d36060830185614afe565b6166e06080830184614b0d565b9695505050505050565b5f6080820190506166fd5f8301876147ca565b61670a6020830186615967565b61671760408301856147ca565b61672460608301846147ca565b9594505050505056fe44656c656761746564557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c616464726573732064656c656761746f72416464726573732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e44617973295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c7429557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620636f6e747261637473436861696e49642c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e44617973295573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c62797465732075736572446563727970746564536861726529
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\x1DW_5`\xE0\x1C\x80c\x83\x16\0\x1F\x11a\0\x9FW\x80c\xAD<\xB1\xCC\x11a\0cW\x80c\xAD<\xB1\xCC\x14a\x03EW\x80c\xB9\xBF\xE0\xA8\x14a\x03oW\x80c\xE3\x0C9x\x14a\x03\x97W\x80c\xF1\x1D\x068\x14a\x03\xC1W\x80c\xF2\xFD\xE3\x8B\x14a\x03\xE9Wa\x01\x1DV[\x80c\x83\x16\0\x1F\x14a\x02sW\x80c\x84\xB0\x19n\x14a\x02\x9BW\x80c\x8D\xA5\xCB[\x14a\x02\xCBW\x80c\xA6\t\x049\x14a\x02\xF5W\x80c\xAA9\xA3V\x14a\x03\x1DWa\x01\x1DV[\x80cR\xD1\x90-\x11a\0\xE6W\x80cR\xD1\x90-\x14a\x01\xDFW\x80cqP\x18\xA6\x14a\x02\tW\x80cv\n\x04\x19\x14a\x02\x1FW\x80cy\xBAP\x97\x14a\x02GW\x80c\x81)\xFC\x1C\x14a\x02]Wa\x01\x1DV[\x80b\x8B\xC3\xE1\x14a\x01!W\x80c\x02\xFD\x1Ad\x14a\x01IW\x80c\r\x8En,\x14a\x01qW\x80c\x18\x7F\xE5)\x14a\x01\x9BW\x80cO\x1E\xF2\x86\x14a\x01\xC3W[__\xFD[4\x80\x15a\x01,W__\xFD[Pa\x01G`\x04\x806\x03\x81\x01\x90a\x01B\x91\x90aC\x99V[a\x04\x11V[\0[4\x80\x15a\x01TW__\xFD[Pa\x01o`\x04\x806\x03\x81\x01\x90a\x01j\x91\x90aD~V[a\x06\x1EV[\0[4\x80\x15a\x01|W__\xFD[Pa\x01\x85a\x08\x81V[`@Qa\x01\x92\x91\x90aE\x7FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xA6W__\xFD[Pa\x01\xC1`\x04\x806\x03\x81\x01\x90a\x01\xBC\x91\x90aE\xF4V[a\x08\xFCV[\0[a\x01\xDD`\x04\x806\x03\x81\x01\x90a\x01\xD8\x91\x90aGgV[a\n\xAAV[\0[4\x80\x15a\x01\xEAW__\xFD[Pa\x01\xF3a\n\xC9V[`@Qa\x02\0\x91\x90aG\xD9V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x14W__\xFD[Pa\x02\x1Da\n\xFAV[\0[4\x80\x15a\x02*W__\xFD[Pa\x02E`\x04\x806\x03\x81\x01\x90a\x02@\x91\x90aH\x87V[a\x0B\rV[\0[4\x80\x15a\x02RW__\xFD[Pa\x02[a\x0F\xCEV[\0[4\x80\x15a\x02hW__\xFD[Pa\x02qa\x10\\V[\0[4\x80\x15a\x02~W__\xFD[Pa\x02\x99`\x04\x806\x03\x81\x01\x90a\x02\x94\x91\x90aI\xA6V[a\x12\x05V[\0[4\x80\x15a\x02\xA6W__\xFD[Pa\x02\xAFa\x15\xC3V[`@Qa\x02\xC2\x97\x96\x95\x94\x93\x92\x91\x90aK\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xD6W__\xFD[Pa\x02\xDFa\x16\xCCV[`@Qa\x02\xEC\x91\x90aLUV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\0W__\xFD[Pa\x03\x1B`\x04\x806\x03\x81\x01\x90a\x03\x16\x91\x90aLnV[a\x17\x01V[\0[4\x80\x15a\x03(W__\xFD[Pa\x03C`\x04\x806\x03\x81\x01\x90a\x03>\x91\x90aE\xF4V[a\x17qV[\0[4\x80\x15a\x03PW__\xFD[Pa\x03Ya\x18\xB7V[`@Qa\x03f\x91\x90aE\x7FV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03zW__\xFD[Pa\x03\x95`\x04\x806\x03\x81\x01\x90a\x03\x90\x91\x90aD~V[a\x18\xF0V[\0[4\x80\x15a\x03\xA2W__\xFD[Pa\x03\xABa\x1CUV[`@Qa\x03\xB8\x91\x90aLUV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xCCW__\xFD[Pa\x03\xE7`\x04\x806\x03\x81\x01\x90a\x03\xE2\x91\x90aL\x99V[a\x1C\x8AV[\0[4\x80\x15a\x03\xF4W__\xFD[Pa\x04\x0F`\x04\x806\x03\x81\x01\x90a\x04\n\x91\x90aM<V[a\x1F*V[\0[__\x90P[\x82\x82\x90P\x81\x10\x15a\x06\x18Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84\x84\x81\x81\x10a\x04dWa\x04caMgV[[\x90P`@\x02\x01_\x015\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\x8B\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xA1W__\xFD[PZ\xFA\x15\x80\x15a\x04\xB3W=__>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84\x84\x81\x81\x10a\x04\xFAWa\x04\xF9aMgV[[\x90P`@\x02\x01_\x015\x85\x85\x85\x81\x81\x10a\x05\x16Wa\x05\x15aMgV[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x05.\x91\x90aM<V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05K\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05aW__\xFD[PZ\xFA\x15\x80\x15a\x05sW=__>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x84\x84\x84\x81\x81\x10a\x05\xBAWa\x05\xB9aMgV[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xDF\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x05\xF5W__\xFD[PZ\xFA\x15\x80\x15a\x06\x07W=__>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x04\x16V[PPPPV[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06k\x91\x90aLUV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06\x81W__\xFD[PZ\xFA\x15\x80\x15a\x06\x93W=__>=_\xFD[PPPP_a\x06\xA0a\x1F\xE3V[\x90P_`@Q\x80`@\x01`@R\x80\x83`\x04\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x07\tW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x06\xF5W[PPPPP\x81R` \x01\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x07f\x82a \nV[\x90Pa\x07t\x88\x82\x87\x87a \x98V[_\x83`\x03\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x86\x86\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x07\xD2\x92\x91\x90aO\xC2V[P\x83`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x08\tWPa\x08\x08\x81\x80T\x90Pa\"yV[[\x15a\x08vW`\x01\x84`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x88\x7FaV\x8Dn\xB4\x8Eb\x87\n\xFF\xFDUI\x92\x06\xA5J\x8Fx\xB0Jb~\0\xED\tqa\xFC\x05\xD6\xBE\x89\x89\x84`@Qa\x08m\x93\x92\x91\x90aR\x19V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x08\xC2_a#\nV[a\x08\xCC`\x01a#\nV[a\x08\xD5_a#\nV[`@Q` \x01a\x08\xE8\x94\x93\x92\x91\x90aS\x1EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_\x82\x82\x90P\x03a\t8W`@Q\x7F-\xE7T8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x81\x82\x82\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa#\xD4V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xD1\x92\x91\x90aS\xF4V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xEBW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x13\x91\x90aV\x9FV[\x90Pa\n\x1E\x81a%\x02V[_a\n'a\x1F\xE3V[\x90P\x80_\x01_\x81T\x80\x92\x91\x90a\n<\x90aW\x13V[\x91\x90PUP_\x81_\x01T\x90P\x84\x84\x83`\x04\x01_\x84\x81R` \x01\x90\x81R` \x01_ \x91\x90a\nj\x92\x91\x90aB\x1CV[P\x80\x7F\x17\xC62\x19o\xBFk\x96\xD9gYq\x05\x8D7\x01s0\x94\xC3\xF2\xF1\xDC\xB9\xBA}*\x08\xBE\xE0\xAA\xFB\x84`@Qa\n\x9B\x91\x90aY;V[`@Q\x80\x91\x03\x90\xA2PPPPPV[a\n\xB2a%\xE8V[a\n\xBB\x82a&\xCEV[a\n\xC5\x82\x82a&\xD9V[PPV[_a\n\xD2a'\xF7V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[a\x0B\x02a(~V[a\x0B\x0B_a)\x05V[V[`\n`\xFF\x16\x86\x86\x90P\x11\x15a\x0B_W`\n\x86\x86\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0BV\x92\x91\x90aYvV[`@Q\x80\x91\x03\x90\xFD[a\x0Bx\x89\x806\x03\x81\x01\x90a\x0Bs\x91\x90aY\xEAV[a)BV[a\x0B\xD3\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x89_\x01` \x81\x01\x90a\x0B\xCE\x91\x90aM<V[a*\x8DV[\x15a\x0C*W\x87_\x01` \x81\x01\x90a\x0B\xEA\x91\x90aM<V[\x86\x86`@Q\x7F\xC3Dj\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C!\x93\x92\x91\x90aZ\xABV[`@Q\x80\x91\x03\x90\xFD[_a\x0C\x88\x8C\x8C\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8C_\x01` \x81\x01\x90a\x0C\x83\x91\x90aM<V[a+\x0BV[\x90Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x89\x8B\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xDD\x94\x93\x92\x91\x90a[\x18V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xF3W__\xFD[PZ\xFA\x15\x80\x15a\r\x05W=__>=_\xFD[PPPP_`@Q\x80`\xC0\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x89\x89\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B_\x01` \x81\x01\x90a\r\xB6\x91\x90aM<V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81RP\x90Pa\x0E\x08\x81\x8B` \x01` \x81\x01\x90a\x0E\x01\x91\x90aM<V[\x86\x86a-\xE6V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0EV\x91\x90a[\xEEV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0EpW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x98\x91\x90aV\x9FV[\x90Pa\x0E\xA3\x81a%\x02V[_a\x0E\xACa\x1F\xE3V[\x90P\x80_\x01_\x81T\x80\x92\x91\x90a\x0E\xC1\x90aW\x13V[\x91\x90PUP_\x81_\x01T\x90P`@Q\x80`@\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x0FK\x91\x90a\\\x18V[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x0Fh\x92\x91\x90aBgV[P\x90PP\x80\x7F\x1C=\xCA\xD61\x1B\xE6\xD5\x8D\xC4\xD4\xB9\xF1\xBC\x16%\xEB\x18\xD7-\xE9i\xDBu\xE1\x1A\x88\xEF5'\xD2\xF3\x84\x8F` \x01` \x81\x01\x90a\x0F\xA2\x91\x90aM<V[\x8C\x8C`@Qa\x0F\xB4\x94\x93\x92\x91\x90a\\\xE7V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_a\x0F\xD7a.\xBCV[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x0F\xF8a\x1CUV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x10PW\x80`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10G\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[a\x10Y\x81a)\x05V[PV[`\x02_a\x10ga.\xC3V[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x10\xAFWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x10\xE6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\x11\x9F`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa.\xEAV[a\x11\xAFa\x11\xAAa\x16\xCCV[a/\0V[_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x11\xF9\x91\x90a]NV[`@Q\x80\x91\x03\x90\xA1PPV[`\n`\xFF\x16\x87\x87\x90P\x11\x15a\x12WW`\n\x87\x87\x90P`@Q\x7F\xC5\xABF~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12N\x92\x91\x90aYvV[`@Q\x80\x91\x03\x90\xFD[a\x12p\x89\x806\x03\x81\x01\x90a\x12k\x91\x90aY\xEAV[a)BV[a\x12\xBA\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x86a*\x8DV[\x15a\x13\0W\x84\x87\x87`@Q\x7F\xDCMx\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xF7\x93\x92\x91\x90aZ\xABV[`@Q\x80\x91\x03\x90\xFD[_a\x13M\x8C\x8C\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x89a+\x0BV[\x90P_`@Q\x80`\xA0\x01`@R\x80\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x8B\x81R` \x01\x8C_\x015\x81R` \x01\x8C` \x015\x81RP\x90Pa\x14\x0F\x81\x88\x86\x86a/\x14V[_s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA1O\x89q\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14]\x91\x90a[\xEEV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14wW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9F\x91\x90aV\x9FV[\x90Pa\x14\xAA\x81a%\x02V[_a\x14\xB3a\x1F\xE3V[\x90P\x80_\x01_\x81T\x80\x92\x91\x90a\x14\xC8\x90aW\x13V[\x91\x90PUP_\x81_\x01T\x90P`@Q\x80`@\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81R` \x01\x86\x81RP\x82`\x06\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01\x90\x81a\x15R\x91\x90a\\\x18V[P` \x82\x01Q\x81`\x01\x01\x90\x80Q\x90` \x01\x90a\x15o\x92\x91\x90aBgV[P\x90PP\x80\x7F\x1C=\xCA\xD61\x1B\xE6\xD5\x8D\xC4\xD4\xB9\xF1\xBC\x16%\xEB\x18\xD7-\xE9i\xDBu\xE1\x1A\x88\xEF5'\xD2\xF3\x84\x8C\x8C\x8C`@Qa\x15\xA9\x94\x93\x92\x91\x90a\\\xE7V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[_``\x80___``_a\x15\xD5a/\xEAV[\x90P__\x1B\x81_\x01T\x14\x80\x15a\x15\xF0WP__\x1B\x81`\x01\x01T\x14[a\x16/W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16&\x90a]\xB1V[`@Q\x80\x91\x03\x90\xFD[a\x167a0\x11V[a\x16?a0\xAFV[F0__\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16^Wa\x16]aFCV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\x8CW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[__a\x16\xD6a1MV[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[_a\x17\na\x1F\xE3V[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x17mW\x81`@Q\x7F\x0B\xF0\x14\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17d\x91\x90a]\xCFV[`@Q\x80\x91\x03\x90\xFD[PPV[__\x90P[\x82\x82\x90P\x81\x10\x15a\x18\xB2Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x84\x84\x84\x81\x81\x10a\x17\xC4Wa\x17\xC3aMgV[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17\xE7\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x17\xFDW__\xFD[PZ\xFA\x15\x80\x15a\x18\x0FW=__>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x84\x84\x84\x81\x81\x10a\x18VWa\x18UaMgV[[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x18y\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x8FW__\xFD[PZ\xFA\x15\x80\x15a\x18\xA1W=__>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x17vV[PPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xC6'RX3`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19=\x91\x90aLUV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x19SW__\xFD[PZ\xFA\x15\x80\x15a\x19eW=__>=_\xFD[PPPP_a\x19ra\x1F\xE3V[\x90P_\x81`\x06\x01_\x88\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x19\xA2\x90aM\xF2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x19\xCE\x90aM\xF2V[\x80\x15a\x1A\x19W\x80`\x1F\x10a\x19\xF0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1A\x19V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x19\xFCW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x1AoW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x1A[W[PPPPP\x81RPP\x90P_`@Q\x80``\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x81RP\x90P_a\x1A\xEC\x82a1tV[\x90Pa\x1A\xFA\x89\x82\x88\x88a \x98V[_\x84`\x05\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x90P\x80\x87\x87\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x1BI\x92\x91\x90aO\xC2V[P\x84`\x08\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x89\x89\x90\x91\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x92\x90\x91\x92\x90\x91\x92\x90\x91\x92P\x91\x82a\x1B\x95\x92\x91\x90aO\xC2V[P\x84`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1B\xCCWPa\x1B\xCB\x81\x80T\x90Pa2\x0FV[[\x15a\x1CIW`\x01\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x7Fs\x12\xDE\xC4\xCE\xAD\r]=\xA86\xCD\xBA\xED\x1E\xB6\xA8\x1E!\x8CQ\x9C\x87@\xDAJ\xC7Z\xFC\xB6\xC5\xC7\x86`\x08\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x83`@Qa\x1C@\x92\x91\x90a^\x82V[`@Q\x80\x91\x03\x90\xA2[PPPPPPPPPPV[__a\x1C_a2\xA0V[\x90P\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91PP\x90V[s\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cQ\xC4\x1D\x0E\x87\x87\x85\x85`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C\xDD\x94\x93\x92\x91\x90a[\x18V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1C\xF3W__\xFD[PZ\xFA\x15\x80\x15a\x1D\x05W=__>=_\xFD[PPPP__\x90P[\x84\x84\x90P\x81\x10\x15a\x1F!Ws\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a\x1D\\Wa\x1D[aMgV[[\x90P`@\x02\x01_\x015\x88_\x01` \x81\x01\x90a\x1Dw\x91\x90aM<V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1D\x94\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1D\xAAW__\xFD[PZ\xFA\x15\x80\x15a\x1D\xBCW=__>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x86\x86\x84\x81\x81\x10a\x1E\x03Wa\x1E\x02aMgV[[\x90P`@\x02\x01_\x015\x87\x87\x85\x81\x81\x10a\x1E\x1FWa\x1E\x1EaMgV[[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1E7\x91\x90aM<V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1ET\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1EjW__\xFD[PZ\xFA\x15\x80\x15a\x1E|W=__>=_\xFD[PPPPs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xD4Goc\x86\x86\x84\x81\x81\x10a\x1E\xC3Wa\x1E\xC2aMgV[[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xE8\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xFEW__\xFD[PZ\xFA\x15\x80\x15a\x1F\x10W=__>=_\xFD[PPPP\x80\x80`\x01\x01\x91PPa\x1D\x0EV[PPPPPPPV[a\x1F2a(~V[_a\x1F;a2\xA0V[\x90P\x81\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\x9Da\x16\xCCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F8\xD1k\x8C\xAC\"\xD9\x9F\xC7\xC1$\xB9\xCD\r\xE2\xD3\xFA\x1F\xAE\xF4 \xBF\xE7\x91\xD8\xC3b\xD7e\xE2'\0`@Q`@Q\x80\x91\x03\x90\xA3PPV[_\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x90P\x90V[_a \x91`@Q\x80`\x80\x01`@R\x80`D\x81R` \x01ag\xE0`D\x919\x80Q\x90` \x01 \x83_\x01Q`@Q` \x01a B\x91\x90a_CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 `@Q` \x01a v\x93\x92\x91\x90a_YV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[_a \xA1a\x1F\xE3V[\x90P_a \xF1\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa2\xE0V[\x90Ps\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cl\x88\xEBC\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!@\x91\x90aLUV[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a!VW__\xFD[PZ\xFA\x15\x80\x15a!hW=__>=_\xFD[PPPP\x81`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\"\x0BW\x85\x81`@Q\x7F\x99\xECH\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\"\x02\x92\x91\x90a_\x8EV[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x02\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPPPPPPPV[__s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cG\xCDK>`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xFC\x91\x90a_\xB5V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[``_`\x01a#\x18\x84a3\nV[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#6Wa#5aFCV[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a#hW\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a#\xC9W\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a#\xBEWa#\xBDa_\xE0V[[\x04\x94P_\x85\x03a#uW[\x81\x93PPPP\x91\x90PV[__\x90P__\x90P[\x82Q\x81\x10\x15a$\xB2W_\x83\x82\x81Q\x81\x10a#\xFAWa#\xF9aMgV[[` \x02` \x01\x01Q\x90P_a$\x0E\x82a4[V[\x90Pa$\x19\x81a4\xE5V[a\xFF\xFF\x16\x84a$(\x91\x90a`\rV[\x93Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x19??,\x83`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a$w\x91\x90aG\xD9V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a$\x8DW__\xFD[PZ\xFA\x15\x80\x15a$\x9FW=__>=_\xFD[PPPPPP\x80\x80`\x01\x01\x91PPa#\xDDV[Pa\x08\0\x81\x11\x15a$\xFEWa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xF5\x92\x91\x90a`@V[`@Q\x80\x91\x03\x90\xFD[PPV[`\x01\x81Q\x11\x15a%\xE5W_\x81_\x81Q\x81\x10a% Wa%\x1FaMgV[[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a%\xE2W\x81\x83\x82\x81Q\x81\x10a%QWa%PaMgV[[` \x02` \x01\x01Q` \x01Q\x14a%\xD5W\x82_\x81Q\x81\x10a%uWa%taMgV[[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a%\x90Wa%\x8FaMgV[[` \x02` \x01\x01Q`@Q\x7F\xCF\xAE\x92\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xCC\x92\x91\x90a`\xC7V[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa%4V[PP[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a&\x95WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a&|a7rV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a&\xCCW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a&\xD6a(~V[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a'AWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'>\x91\x90a`\xFCV[`\x01[a'\x82W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'y\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a'\xE8W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'\xDF\x91\x90aG\xD9V[`@Q\x80\x91\x03\x90\xFD[a'\xF2\x83\x83a7\xC5V[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(|W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a(\x86a.\xBCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a(\xA4a\x16\xCCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)\x03Wa(\xC7a.\xBCV[`@Q\x7F\x11\x8C\xDA\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a(\xFA\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[V[_a)\x0Ea2\xA0V[\x90P\x80_\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90Ua)>\x82a87V[PPV[_\x81` \x01Q\x03a)\x7FW`@Q\x7F\xDE(Y\xC1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x01ma\xFF\xFF\x16\x81` \x01Q\x11\x15a)\xD6Wa\x01m\x81` \x01Q`@Q\x7F2\x95\x18c\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xCD\x92\x91\x90aadV[`@Q\x80\x91\x03\x90\xFD[B\x81_\x01Q\x11\x15a*#WB\x81_\x01Q`@Q\x7F\xF2L\x08\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\x1A\x92\x91\x90a`@V[`@Q\x80\x91\x03\x90\xFD[Bb\x01Q\x80\x82` \x01Qa*7\x91\x90aa\x8BV[\x82_\x01Qa*E\x91\x90a`\rV[\x10\x15a*\x8AWB\x81`@Q\x7F04\x80@\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\x81\x92\x91\x90aa\xF9V[`@Q\x80\x91\x03\x90\xFD[PV[___\x90P[\x83Q\x81\x10\x15a+\0W\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x82\x81Q\x81\x10a*\xC6Wa*\xC5aMgV[[` \x02` \x01\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a*\xF3W`\x01\x91PPa+\x05V[\x80\x80`\x01\x01\x91PPa*\x93V[P_\x90P[\x92\x91PPV[``_\x85\x85\x90P\x03a+IW`@Q\x7F\xA6\xA6\xCB!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+fWa+eaFCV[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a+\x94W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a-\x91W_\x87\x87\x83\x81\x81\x10a+\xBFWa+\xBEaMgV[[\x90P`@\x02\x01_\x015\x90P_\x88\x88\x84\x81\x81\x10a+\xDEWa+\xDDaMgV[[\x90P`@\x02\x01` \x01` \x81\x01\x90a+\xF6\x91\x90aM<V[\x90P_a,\x02\x83a4[V[\x90Pa,\r\x81a4\xE5V[a\xFF\xFF\x16\x85a,\x1C\x91\x90a`\rV[\x94Ps\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x89`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,m\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a,\x83W__\xFD[PZ\xFA\x15\x80\x15a,\x95W=__>=_\xFD[PPPPs\xA5\x0FRC\xC7\x0C\x80\xA80\x9E=9\xD8\xC9\xD9X\xCD\xA89ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c;\xCEI\x8D\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a,\xE8\x92\x91\x90aM\x94V[_`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a,\xFEW__\xFD[PZ\xFA\x15\x80\x15a-\x10W=__>=_\xFD[PPPPa-\x1E\x88\x83a*\x8DV[a-aW\x81\x88`@Q\x7F\xA4\xC3\x03\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-X\x92\x91\x90ab|V[`@Q\x80\x91\x03\x90\xFD[\x82\x86\x85\x81Q\x81\x10a-uWa-taMgV[[` \x02` \x01\x01\x81\x81RPPPPP\x80\x80`\x01\x01\x91PPa+\xA0V[Pa\x08\0\x81\x11\x15a-\xDDWa\x08\0\x81`@Q\x7F\xE7\xF4\x89]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xD4\x92\x91\x90a`@V[`@Q\x80\x91\x03\x90\xFD[P\x94\x93PPPPV[_a-\xF0\x85a9\x08V[\x90P_a.@\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa2\xE0V[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a.\xB4W\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xAB\x92\x91\x90ab\xAAV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[_3\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a.\xF2a9\xAEV[a.\xFC\x82\x82a9\xEEV[PPV[a/\x08a9\xAEV[a/\x11\x81a:?V[PV[_a/\x1E\x85a:\xC3V[\x90P_a/n\x82\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPa2\xE0V[\x90P\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/\xE2W\x83\x83`@Q\x7F*\x87='\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xD9\x92\x91\x90ab\xAAV[`@Q\x80\x91\x03\x90\xFD[PPPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_a0\x1Ca/\xEAV[\x90P\x80`\x02\x01\x80Ta0-\x90aM\xF2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0Y\x90aM\xF2V[\x80\x15a0\xA4W\x80`\x1F\x10a0{Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\xA4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a0\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_a0\xBAa/\xEAV[\x90P\x80`\x03\x01\x80Ta0\xCB\x90aM\xF2V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta0\xF7\x90aM\xF2V[\x80\x15a1BW\x80`\x1F\x10a1\x19Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1%W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x90P\x90V[_a2\x08`@Q\x80`\x80\x01`@R\x80`]\x81R` \x01ah\xB4`]\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a1\xB8\x91\x90a_CV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 `@Q` \x01a1\xED\x94\x93\x92\x91\x90ab\xCCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[__s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cI\x04\x13\xAA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x92\x91\x90a_\xB5V[\x90P\x80\x83\x10\x15\x91PP\x91\x90PV[_\x7F#~\x15\x82\"\xE3\xE6\x96\x8Br\xB9\xDB\r\x80C\xAA\xCF\x07J\xD9\xF6P\xF0\xD1`kM\x82\xEEC,\0\x90P\x90V[_a2\xD9a2\xD3a;cV[\x83a;qV[\x90P\x91\x90PV[____a2\xEE\x86\x86a;\xB1V[\x92P\x92P\x92Pa2\xFE\x82\x82a<\x06V[\x82\x93PPPP\x92\x91PPV[___\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a3fWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81a3\\Wa3[a_\xE0V[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a3\xA3Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81a3\x99Wa3\x98a_\xE0V[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10a3\xD2Wf#\x86\xF2o\xC1\0\0\x83\x81a3\xC8Wa3\xC7a_\xE0V[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10a3\xFBWc\x05\xF5\xE1\0\x83\x81a3\xF1Wa3\xF0a_\xE0V[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10a4 Wa'\x10\x83\x81a4\x16Wa4\x15a_\xE0V[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10a4CW`d\x83\x81a49Wa48a_\xE0V[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10a4RW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[__`\xF8`\xF0\x84\x90\x1B\x90\x1C_\x1C\x90P`S\x80\x81\x11\x15a4}Wa4|ac\x0FV[[`\xFF\x16\x81`\xFF\x16\x11\x15a4\xC7W\x80`@Q\x7Fd\x19P\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xBE\x91\x90ac<V[`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`S\x81\x11\x15a4\xDDWa4\xDCac\x0FV[[\x91PP\x91\x90PV[__`S\x81\x11\x15a4\xF9Wa4\xF8ac\x0FV[[\x82`S\x81\x11\x15a5\x0CWa5\x0Bac\x0FV[[\x03a5\x1AW`\x02\x90Pa7mV[`\x02`S\x81\x11\x15a5.Wa5-ac\x0FV[[\x82`S\x81\x11\x15a5AWa5@ac\x0FV[[\x03a5OW`\x08\x90Pa7mV[`\x03`S\x81\x11\x15a5cWa5bac\x0FV[[\x82`S\x81\x11\x15a5vWa5uac\x0FV[[\x03a5\x84W`\x10\x90Pa7mV[`\x04`S\x81\x11\x15a5\x98Wa5\x97ac\x0FV[[\x82`S\x81\x11\x15a5\xABWa5\xAAac\x0FV[[\x03a5\xB9W` \x90Pa7mV[`\x05`S\x81\x11\x15a5\xCDWa5\xCCac\x0FV[[\x82`S\x81\x11\x15a5\xE0Wa5\xDFac\x0FV[[\x03a5\xEEW`@\x90Pa7mV[`\x06`S\x81\x11\x15a6\x02Wa6\x01ac\x0FV[[\x82`S\x81\x11\x15a6\x15Wa6\x14ac\x0FV[[\x03a6#W`\x80\x90Pa7mV[`\x07`S\x81\x11\x15a67Wa66ac\x0FV[[\x82`S\x81\x11\x15a6JWa6Iac\x0FV[[\x03a6XW`\xA0\x90Pa7mV[`\x08`S\x81\x11\x15a6lWa6kac\x0FV[[\x82`S\x81\x11\x15a6\x7FWa6~ac\x0FV[[\x03a6\x8EWa\x01\0\x90Pa7mV[`\t`S\x81\x11\x15a6\xA2Wa6\xA1ac\x0FV[[\x82`S\x81\x11\x15a6\xB5Wa6\xB4ac\x0FV[[\x03a6\xC4Wa\x02\0\x90Pa7mV[`\n`S\x81\x11\x15a6\xD8Wa6\xD7ac\x0FV[[\x82`S\x81\x11\x15a6\xEBWa6\xEAac\x0FV[[\x03a6\xFAWa\x04\0\x90Pa7mV[`\x0B`S\x81\x11\x15a7\x0EWa7\rac\x0FV[[\x82`S\x81\x11\x15a7!Wa7 ac\x0FV[[\x03a70Wa\x08\0\x90Pa7mV[\x81`@Q\x7F\xBEx0\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a7d\x91\x90ac\x9BV[`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[_a7\x9E\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba=hV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[a7\xCE\x82a=qV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15a8*Wa8$\x82\x82a>:V[Pa83V[a82a>\xBAV[[PPV[_a8@a1MV[\x90P_\x81_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82\x82_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0`@Q`@Q\x80\x91\x03\x90\xA3PPPV[_a9\xA7`@Q\x80`\xE0\x01`@R\x80`\xB2\x81R` \x01ag.`\xB2\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a9L\x91\x90ad@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q\x88`\xA0\x01Q`@Q` \x01a9\x8C\x97\x96\x95\x94\x93\x92\x91\x90adVV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[a9\xB6a>\xF6V[a9\xECW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[a9\xF6a9\xAEV[_a9\xFFa/\xEAV[\x90P\x82\x81`\x02\x01\x90\x81a:\x12\x91\x90ae\x1BV[P\x81\x81`\x03\x01\x90\x81a:$\x91\x90ae\x1BV[P__\x1B\x81_\x01\x81\x90UP__\x1B\x81`\x01\x01\x81\x90UPPPPV[a:Ga9\xAEV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a:\xB7W_`@Q\x7F\x1EO\xBD\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a:\xAE\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[a:\xC0\x81a)\x05V[PV[_a;\\`@Q\x80`\xC0\x01`@R\x80`\x90\x81R` \x01ah$`\x90\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a;\x07\x91\x90ad@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x86``\x01Q\x87`\x80\x01Q`@Q` \x01a;A\x96\x95\x94\x93\x92\x91\x90ae\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a2\xC7V[\x90P\x91\x90PV[_a;la?\x14V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[___`A\x84Q\x03a;\xF1W___` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90Pa;\xE3\x88\x82\x85\x85a?wV[\x95P\x95P\x95PPPPa;\xFFV[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15a<\x19Wa<\x18ac\x0FV[[\x82`\x03\x81\x11\x15a<,Wa<+ac\x0FV[[\x03\x15a=dW`\x01`\x03\x81\x11\x15a<FWa<Eac\x0FV[[\x82`\x03\x81\x11\x15a<YWa<Xac\x0FV[[\x03a<\x90W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15a<\xA4Wa<\xA3ac\x0FV[[\x82`\x03\x81\x11\x15a<\xB7Wa<\xB6ac\x0FV[[\x03a<\xFBW\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a<\xF2\x91\x90a]\xCFV[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15a=\x0EWa=\rac\x0FV[[\x82`\x03\x81\x11\x15a=!Wa= ac\x0FV[[\x03a=cW\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=Z\x91\x90aG\xD9V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03a=\xCCW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a=\xC3\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[\x80a=\xF8\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1Ba=hV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``__\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@Qa>c\x91\x90af\x83V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a>\x9BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a>\xA0V[``\x91P[P\x91P\x91Pa>\xB0\x85\x83\x83a@^V[\x92PPP\x92\x91PPV[_4\x11\x15a>\xF4W`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_a>\xFFa.\xC3V[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0Fa?>a@\xEBV[a?FaAaV[F0`@Q` \x01a?\\\x95\x94\x93\x92\x91\x90af\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[___\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15a?\xB3W_`\x03\x85\x92P\x92P\x92Pa@TV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@Qa?\xD6\x94\x93\x92\x91\x90af\xEAV[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a?\xF6W=__>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a@GW_`\x01__\x1B\x93P\x93P\x93PPa@TV[\x80___\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82a@sWa@n\x82aA\xD8V[a@\xE3V[_\x82Q\x14\x80\x15a@\x99WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15a@\xDBW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@\xD2\x91\x90aLUV[`@Q\x80\x91\x03\x90\xFD[\x81\x90Pa@\xE4V[[\x93\x92PPPV[__a@\xF5a/\xEAV[\x90P_aA\0a0\x11V[\x90P_\x81Q\x11\x15aA\x1CW\x80\x80Q\x90` \x01 \x92PPPaA^V[_\x82_\x01T\x90P__\x1B\x81\x14aA7W\x80\x93PPPPaA^V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[__aAka/\xEAV[\x90P_aAva0\xAFV[\x90P_\x81Q\x11\x15aA\x92W\x80\x80Q\x90` \x01 \x92PPPaA\xD5V[_\x82`\x01\x01T\x90P__\x1B\x81\x14aA\xAEW\x80\x93PPPPaA\xD5V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aA\xEAW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aBVW\x91` \x02\x82\x01[\x82\x81\x11\x15aBUW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aB:V[[P\x90PaBc\x91\x90aB\xB2V[P\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aB\xA1W\x91` \x02\x82\x01[\x82\x81\x11\x15aB\xA0W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aB\x85V[[P\x90PaB\xAE\x91\x90aB\xB2V[P\x90V[[\x80\x82\x11\x15aB\xC9W_\x81_\x90UP`\x01\x01aB\xB3V[P\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aC\x07\x82aB\xDEV[\x90P\x91\x90PV[aC\x17\x81aB\xFDV[\x81\x14aC!W__\xFD[PV[_\x815\x90PaC2\x81aC\x0EV[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12aCYWaCXaC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aCvWaCuaC<V[[` \x83\x01\x91P\x83`@\x82\x02\x83\x01\x11\x15aC\x92WaC\x91aC@V[[\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aC\xB0WaC\xAFaB\xD6V[[_aC\xBD\x86\x82\x87\x01aC$V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\xDEWaC\xDDaB\xDAV[[aC\xEA\x86\x82\x87\x01aCDV[\x92P\x92PP\x92P\x92P\x92V[_\x81\x90P\x91\x90PV[aD\x08\x81aC\xF6V[\x81\x14aD\x12W__\xFD[PV[_\x815\x90PaD#\x81aC\xFFV[\x92\x91PPV[__\x83`\x1F\x84\x01\x12aD>WaD=aC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD[WaDZaC<V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aDwWaDvaC@V[[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15aD\x97WaD\x96aB\xD6V[[_aD\xA4\x88\x82\x89\x01aD\x15V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xC5WaD\xC4aB\xDAV[[aD\xD1\x88\x82\x89\x01aD)V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xF4WaD\xF3aB\xDAV[[aE\0\x88\x82\x89\x01aD)V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x82\x81\x83^_\x83\x83\x01RPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aEQ\x82aE\x0FV[aE[\x81\x85aE\x19V[\x93PaEk\x81\x85` \x86\x01aE)V[aEt\x81aE7V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaE\x97\x81\x84aEGV[\x90P\x92\x91PPV[__\x83`\x1F\x84\x01\x12aE\xB4WaE\xB3aC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xD1WaE\xD0aC<V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aE\xEDWaE\xECaC@V[[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15aF\nWaF\taB\xD6V[[_\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF'WaF&aB\xDAV[[aF3\x85\x82\x86\x01aE\x9FV[\x92P\x92PP\x92P\x92\x90PV[__\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aFy\x82aE7V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aF\x98WaF\x97aFCV[[\x80`@RPPPV[_aF\xAAaB\xCDV[\x90PaF\xB6\x82\x82aFpV[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aF\xD5WaF\xD4aFCV[[aF\xDE\x82aE7V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aG\x0BaG\x06\x84aF\xBBV[aF\xA1V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aG'WaG&aF?V[[aG2\x84\x82\x85aF\xEBV[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aGNWaGMaC8V[[\x815aG^\x84\x82` \x86\x01aF\xF9V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aG}WaG|aB\xD6V[[_aG\x8A\x85\x82\x86\x01aC$V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xABWaG\xAAaB\xDAV[[aG\xB7\x85\x82\x86\x01aG:V[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aG\xD3\x81aG\xC1V[\x82RPPV[_` \x82\x01\x90PaG\xEC_\x83\x01\x84aG\xCAV[\x92\x91PPV[__\xFD[_`@\x82\x84\x03\x12\x15aH\x0BWaH\naG\xF2V[[\x81\x90P\x92\x91PPV[_`@\x82\x84\x03\x12\x15aH)WaH(aG\xF2V[[\x81\x90P\x92\x91PPV[__\x83`\x1F\x84\x01\x12aHGWaHFaC8V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aHdWaHcaC<V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aH\x80WaH\x7FaC@V[[\x92P\x92\x90PV[___________a\x01 \x8C\x8E\x03\x12\x15aH\xA7WaH\xA6aB\xD6V[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xC4WaH\xC3aB\xDAV[[aH\xD0\x8E\x82\x8F\x01aCDV[\x9BP\x9BPP` aH\xE3\x8E\x82\x8F\x01aG\xF6V[\x99PP``aH\xF4\x8E\x82\x8F\x01aH\x14V[\x98PP`\xA0aI\x05\x8E\x82\x8F\x01aD\x15V[\x97PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI&WaI%aB\xDAV[[aI2\x8E\x82\x8F\x01aH2V[\x96P\x96PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIUWaITaB\xDAV[[aIa\x8E\x82\x8F\x01aD)V[\x94P\x94PPa\x01\0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\x85WaI\x84aB\xDAV[[aI\x91\x8E\x82\x8F\x01aD)V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[___________a\x01\0\x8C\x8E\x03\x12\x15aI\xC6WaI\xC5aB\xD6V[[_\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aI\xE3WaI\xE2aB\xDAV[[aI\xEF\x8E\x82\x8F\x01aCDV[\x9BP\x9BPP` aJ\x02\x8E\x82\x8F\x01aG\xF6V[\x99PP``aJ\x13\x8E\x82\x8F\x01aD\x15V[\x98PP`\x80\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ4WaJ3aB\xDAV[[aJ@\x8E\x82\x8F\x01aH2V[\x97P\x97PP`\xA0aJS\x8E\x82\x8F\x01aC$V[\x95PP`\xC0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJtWaJsaB\xDAV[[aJ\x80\x8E\x82\x8F\x01aD)V[\x94P\x94PP`\xE0\x8C\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xA3WaJ\xA2aB\xDAV[[aJ\xAF\x8E\x82\x8F\x01aD)V[\x92P\x92PP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[aJ\xF8\x81aJ\xC4V[\x82RPPV[aK\x07\x81aC\xF6V[\x82RPPV[aK\x16\x81aB\xFDV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aKN\x81aC\xF6V[\x82RPPV[_aK_\x83\x83aKEV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aK\x81\x82aK\x1CV[aK\x8B\x81\x85aK&V[\x93PaK\x96\x83aK6V[\x80_[\x83\x81\x10\x15aK\xC6W\x81QaK\xAD\x88\x82aKTV[\x97PaK\xB8\x83aKkV[\x92PP`\x01\x81\x01\x90PaK\x99V[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90PaK\xE6_\x83\x01\x8AaJ\xEFV[\x81\x81\x03` \x83\x01RaK\xF8\x81\x89aEGV[\x90P\x81\x81\x03`@\x83\x01RaL\x0C\x81\x88aEGV[\x90PaL\x1B``\x83\x01\x87aJ\xFEV[aL(`\x80\x83\x01\x86aK\rV[aL5`\xA0\x83\x01\x85aG\xCAV[\x81\x81\x03`\xC0\x83\x01RaLG\x81\x84aKwV[\x90P\x98\x97PPPPPPPPV[_` \x82\x01\x90PaLh_\x83\x01\x84aK\rV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aL\x83WaL\x82aB\xD6V[[_aL\x90\x84\x82\x85\x01aD\x15V[\x91PP\x92\x91PPV[______`\xA0\x87\x89\x03\x12\x15aL\xB3WaL\xB2aB\xD6V[[_aL\xC0\x89\x82\x8A\x01aD\x15V[\x96PP` aL\xD1\x89\x82\x8A\x01aH\x14V[\x95PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aL\xF2WaL\xF1aB\xDAV[[aL\xFE\x89\x82\x8A\x01aCDV[\x94P\x94PP`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aM!WaM aB\xDAV[[aM-\x89\x82\x8A\x01aH2V[\x92P\x92PP\x92\x95P\x92\x95P\x92\x95V[_` \x82\x84\x03\x12\x15aMQWaMPaB\xD6V[[_aM^\x84\x82\x85\x01aC$V[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_`@\x82\x01\x90PaM\xA7_\x83\x01\x85aG\xCAV[aM\xB4` \x83\x01\x84aK\rV[\x93\x92PPPV[_\x82\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80aN\tW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aN\x1CWaN\x1BaM\xC5V[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02aN~\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82aNCV[aN\x88\x86\x83aNCV[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aN\xC3aN\xBEaN\xB9\x84aC\xF6V[aN\xA0V[aC\xF6V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[aN\xDC\x83aN\xA9V[aN\xF0aN\xE8\x82aN\xCAV[\x84\x84TaNOV[\x82UPPPPV[__\x90P\x90V[aO\x07aN\xF8V[aO\x12\x81\x84\x84aN\xD3V[PPPV[[\x81\x81\x10\x15aO5WaO*_\x82aN\xFFV[`\x01\x81\x01\x90PaO\x18V[PPV[`\x1F\x82\x11\x15aOzWaOK\x81aN\"V[aOT\x84aN4V[\x81\x01` \x85\x10\x15aOcW\x81\x90P[aOwaOo\x85aN4V[\x83\x01\x82aO\x17V[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_aO\x9A_\x19\x84`\x08\x02aO\x7FV[\x19\x80\x83\x16\x91PP\x92\x91PPV[_aO\xB2\x83\x83aO\x8BV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[aO\xCC\x83\x83aM\xBBV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aO\xE5WaO\xE4aFCV[[aO\xEF\x82TaM\xF2V[aO\xFA\x82\x82\x85aO9V[_`\x1F\x83\x11`\x01\x81\x14aP'W_\x84\x15aP\x15W\x82\x87\x015\x90P[aP\x1F\x85\x82aO\xA7V[\x86UPaP\x86V[`\x1F\x19\x84\x16aP5\x86aN\"V[_[\x82\x81\x10\x15aP\\W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PaP7V[\x86\x83\x10\x15aPyW\x84\x89\x015aPu`\x1F\x89\x16\x82aO\x8BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_aP\xAA\x83\x85aP\x8FV[\x93PaP\xB7\x83\x85\x84aF\xEBV[aP\xC0\x83aE7V[\x84\x01\x90P\x93\x92PPPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81TaQ\x13\x81aM\xF2V[aQ\x1D\x81\x86aP\xF7V[\x94P`\x01\x82\x16_\x81\x14aQ7W`\x01\x81\x14aQMWaQ\x7FV[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93PaQ\x7FV[aQV\x85aN\"V[_[\x83\x81\x10\x15aQwW\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90PaQXV[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[_aQ\x93\x83\x83aQ\x07V[\x90P\x92\x91PPV[_`\x01\x82\x01\x90P\x91\x90PV[_aQ\xB1\x82aP\xCBV[aQ\xBB\x81\x85aP\xD5V[\x93P\x83` \x82\x02\x85\x01aQ\xCD\x85aP\xE5V[\x80_[\x85\x81\x10\x15aR\x07W\x84\x84\x03\x89R\x81aQ\xE8\x85\x82aQ\x88V[\x94PaQ\xF3\x83aQ\x9BV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaQ\xD0V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01RaR2\x81\x85\x87aP\x9FV[\x90P\x81\x81\x03` \x83\x01RaRF\x81\x84aQ\xA7V[\x90P\x94\x93PPPPV[_\x81\x90P\x92\x91PPV[_aRd\x82aE\x0FV[aRn\x81\x85aRPV[\x93PaR~\x81\x85` \x86\x01aE)V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aR\xBE`\x02\x83aRPV[\x91PaR\xC9\x82aR\x8AV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_aS\x08`\x01\x83aRPV[\x91PaS\x13\x82aR\xD4V[`\x01\x82\x01\x90P\x91\x90PV[_aS)\x82\x87aRZV[\x91PaS4\x82aR\xB2V[\x91PaS@\x82\x86aRZV[\x91PaSK\x82aR\xFCV[\x91PaSW\x82\x85aRZV[\x91PaSb\x82aR\xFCV[\x91PaSn\x82\x84aRZV[\x91P\x81\x90P\x95\x94PPPPPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[__\xFD[\x82\x81\x837PPPV[_aS\xA4\x83\x85aS|V[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aS\xD7WaS\xD6aS\x8CV[[` \x83\x02\x92PaS\xE8\x83\x85\x84aS\x90V[\x82\x84\x01\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT\r\x81\x84\x86aS\x99V[\x90P\x93\x92PPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aT0WaT/aFCV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[__\xFD[aTR\x81aG\xC1V[\x81\x14aT\\W__\xFD[PV[_\x81Q\x90PaTm\x81aTIV[\x92\x91PPV[_\x81Q\x90PaT\x81\x81aC\xFFV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aT\xA1WaT\xA0aFCV[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_\x81Q\x90PaT\xC0\x81aC\x0EV[\x92\x91PPV[_aT\xD8aT\xD3\x84aT\x87V[aF\xA1V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aT\xFBWaT\xFAaC@V[[\x83[\x81\x81\x10\x15aU$W\x80aU\x10\x88\x82aT\xB2V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaT\xFDV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aUBWaUAaC8V[[\x81QaUR\x84\x82` \x86\x01aT\xC6V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15aUpWaUoaTAV[[aUz`\x80aF\xA1V[\x90P_aU\x89\x84\x82\x85\x01aT_V[_\x83\x01RP` aU\x9C\x84\x82\x85\x01aTsV[` \x83\x01RP`@aU\xB0\x84\x82\x85\x01aT_V[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\xD4WaU\xD3aTEV[[aU\xE0\x84\x82\x85\x01aU.V[``\x83\x01RP\x92\x91PPV[_aU\xFEaU\xF9\x84aT\x16V[aF\xA1V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aV!WaV aC@V[[\x83[\x81\x81\x10\x15aVhW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aVFWaVEaC8V[[\x80\x86\x01aVS\x89\x82aU[V[\x85R` \x85\x01\x94PPP` \x81\x01\x90PaV#V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aV\x86WaV\x85aC8V[[\x81QaV\x96\x84\x82` \x86\x01aU\xECV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aV\xB4WaV\xB3aB\xD6V[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\xD1WaV\xD0aB\xDAV[[aV\xDD\x84\x82\x85\x01aVrV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aW\x1D\x82aC\xF6V[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03aWOWaWNaV\xE6V[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aW\x8C\x81aG\xC1V[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[aW\xC4\x81aB\xFDV[\x82RPPV[_aW\xD5\x83\x83aW\xBBV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aW\xF7\x82aW\x92V[aX\x01\x81\x85aW\x9CV[\x93PaX\x0C\x83aW\xACV[\x80_[\x83\x81\x10\x15aX<W\x81QaX#\x88\x82aW\xCAV[\x97PaX.\x83aW\xE1V[\x92PP`\x01\x81\x01\x90PaX\x0FV[P\x85\x93PPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01QaX^_\x86\x01\x82aW\x83V[P` \x83\x01QaXq` \x86\x01\x82aKEV[P`@\x83\x01QaX\x84`@\x86\x01\x82aW\x83V[P``\x83\x01Q\x84\x82\x03``\x86\x01RaX\x9C\x82\x82aW\xEDV[\x91PP\x80\x91PP\x92\x91PPV[_aX\xB4\x83\x83aXIV[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aX\xD2\x82aWZV[aX\xDC\x81\x85aWdV[\x93P\x83` \x82\x02\x85\x01aX\xEE\x85aWtV[\x80_[\x85\x81\x10\x15aY)W\x84\x84\x03\x89R\x81QaY\n\x85\x82aX\xA9V[\x94PaY\x15\x83aX\xBCV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90PaX\xF1V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYS\x81\x84aX\xC8V[\x90P\x92\x91PPV[_`\xFF\x82\x16\x90P\x91\x90PV[aYp\x81aY[V[\x82RPPV[_`@\x82\x01\x90PaY\x89_\x83\x01\x85aYgV[aY\x96` \x83\x01\x84aJ\xFEV[\x93\x92PPPV[_`@\x82\x84\x03\x12\x15aY\xB2WaY\xB1aTAV[[aY\xBC`@aF\xA1V[\x90P_aY\xCB\x84\x82\x85\x01aD\x15V[_\x83\x01RP` aY\xDE\x84\x82\x85\x01aD\x15V[` \x83\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15aY\xFFWaY\xFEaB\xD6V[[_aZ\x0C\x84\x82\x85\x01aY\x9DV[\x91PP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x91\x90PV[_aZ<` \x84\x01\x84aC$V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aZ[\x83\x85aZ\x15V[\x93PaZf\x82aZ%V[\x80_[\x85\x81\x10\x15aZ\x9EWaZ{\x82\x84aZ.V[aZ\x85\x88\x82aW\xCAV[\x97PaZ\x90\x83aZDV[\x92PP`\x01\x81\x01\x90PaZiV[P\x85\x92PPP\x93\x92PPPV[_`@\x82\x01\x90PaZ\xBE_\x83\x01\x86aK\rV[\x81\x81\x03` \x83\x01RaZ\xD1\x81\x84\x86aZPV[\x90P\x94\x93PPPPV[`@\x82\x01aZ\xEB_\x83\x01\x83aZ.V[aZ\xF7_\x85\x01\x82aW\xBBV[Pa[\x05` \x83\x01\x83aZ.V[a[\x12` \x85\x01\x82aW\xBBV[PPPPV[_`\x80\x82\x01\x90Pa[+_\x83\x01\x87aJ\xFEV[a[8` \x83\x01\x86aZ\xDBV[\x81\x81\x03``\x83\x01Ra[K\x81\x84\x86aZPV[\x90P\x95\x94PPPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a[z\x83\x83aW\x83V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[\x9C\x82a[VV[a[\xA6\x81\x85aS|V[\x93Pa[\xB1\x83a[`V[\x80_[\x83\x81\x10\x15a[\xE1W\x81Qa[\xC8\x88\x82a[oV[\x97Pa[\xD3\x83a[\x86V[\x92PP`\x01\x81\x01\x90Pa[\xB4V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\\\x06\x81\x84a[\x92V[\x90P\x92\x91PPV[_\x81Q\x90P\x91\x90PV[a\\!\x82a\\\x0EV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\:Wa\\9aFCV[[a\\D\x82TaM\xF2V[a\\O\x82\x82\x85aO9V[_` \x90P`\x1F\x83\x11`\x01\x81\x14a\\\x80W_\x84\x15a\\nW\x82\x87\x01Q\x90P[a\\x\x85\x82aO\xA7V[\x86UPa\\\xDFV[`\x1F\x19\x84\x16a\\\x8E\x86aN\"V[_[\x82\x81\x10\x15a\\\xB5W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\\\x90V[\x86\x83\x10\x15a\\\xD2W\x84\x89\x01Qa\\\xCE`\x1F\x89\x16\x82aO\x8BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_``\x82\x01\x90P\x81\x81\x03_\x83\x01Ra\\\xFF\x81\x87aX\xC8V[\x90Pa]\x0E` \x83\x01\x86aK\rV[\x81\x81\x03`@\x83\x01Ra]!\x81\x84\x86aP\x9FV[\x90P\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a]H\x81a],V[\x82RPPV[_` \x82\x01\x90Pa]a_\x83\x01\x84a]?V[\x92\x91PPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_a]\x9B`\x15\x83aE\x19V[\x91Pa]\xA6\x82a]gV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xC8\x81a]\x8FV[\x90P\x91\x90PV[_` \x82\x01\x90Pa]\xE2_\x83\x01\x84aJ\xFEV[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_a^\x1A\x82a]\xE8V[a^$\x81\x85aP\xD5V[\x93P\x83` \x82\x02\x85\x01a^6\x85a]\xF2V[\x80_[\x85\x81\x10\x15a^pW\x84\x84\x03\x89R\x81a^Q\x85\x82aQ\x88V[\x94Pa^\\\x83a^\x04V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa^9V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra^\x9A\x81\x85a^\x10V[\x90P\x81\x81\x03` \x83\x01Ra^\xAE\x81\x84aQ\xA7V[\x90P\x93\x92PPPV[_\x81\x90P\x92\x91PPV[a^\xCA\x81aG\xC1V[\x82RPPV[_a^\xDB\x83\x83a^\xC1V[` \x83\x01\x90P\x92\x91PPV[_a^\xF1\x82a[VV[a^\xFB\x81\x85a^\xB7V[\x93Pa_\x06\x83a[`V[\x80_[\x83\x81\x10\x15a_6W\x81Qa_\x1D\x88\x82a^\xD0V[\x97Pa_(\x83a[\x86V[\x92PP`\x01\x81\x01\x90Pa_\tV[P\x85\x93PPPP\x92\x91PPV[_a_N\x82\x84a^\xE7V[\x91P\x81\x90P\x92\x91PPV[_``\x82\x01\x90Pa_l_\x83\x01\x86aG\xCAV[a_y` \x83\x01\x85aG\xCAV[a_\x86`@\x83\x01\x84aG\xCAV[\x94\x93PPPPV[_`@\x82\x01\x90Pa_\xA1_\x83\x01\x85aJ\xFEV[a_\xAE` \x83\x01\x84aK\rV[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a_\xCAWa_\xC9aB\xD6V[[_a_\xD7\x84\x82\x85\x01aTsV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_a`\x17\x82aC\xF6V[\x91Pa`\"\x83aC\xF6V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15a`:Wa`9aV\xE6V[[\x92\x91PPV[_`@\x82\x01\x90Pa`S_\x83\x01\x85aJ\xFEV[a``` \x83\x01\x84aJ\xFEV[\x93\x92PPPV[_`\x80\x83\x01_\x83\x01Qa`|_\x86\x01\x82aW\x83V[P` \x83\x01Qa`\x8F` \x86\x01\x82aKEV[P`@\x83\x01Qa`\xA2`@\x86\x01\x82aW\x83V[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra`\xBA\x82\x82aW\xEDV[\x91PP\x80\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra`\xDF\x81\x85a`gV[\x90P\x81\x81\x03` \x83\x01Ra`\xF3\x81\x84a`gV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15aa\x11Waa\x10aB\xD6V[[_aa\x1E\x84\x82\x85\x01aT_V[\x91PP\x92\x91PPV[_a\xFF\xFF\x82\x16\x90P\x91\x90PV[_aaNaaIaaD\x84aa'V[aN\xA0V[aC\xF6V[\x90P\x91\x90PV[aa^\x81aa4V[\x82RPPV[_`@\x82\x01\x90Paaw_\x83\x01\x85aaUV[aa\x84` \x83\x01\x84aJ\xFEV[\x93\x92PPPV[_aa\x95\x82aC\xF6V[\x91Paa\xA0\x83aC\xF6V[\x92P\x82\x82\x02aa\xAE\x81aC\xF6V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aa\xC5Waa\xC4aV\xE6V[[P\x92\x91PPV[`@\x82\x01_\x82\x01Qaa\xE0_\x85\x01\x82aKEV[P` \x82\x01Qaa\xF3` \x85\x01\x82aKEV[PPPPV[_``\x82\x01\x90Pab\x0C_\x83\x01\x85aJ\xFEV[ab\x19` \x83\x01\x84aa\xCCV[\x93\x92PPPV[_ab*\x82aW\x92V[ab4\x81\x85aZ\x15V[\x93Pab?\x83aW\xACV[\x80_[\x83\x81\x10\x15aboW\x81QabV\x88\x82aW\xCAV[\x97Paba\x83aW\xE1V[\x92PP`\x01\x81\x01\x90PabBV[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90Pab\x8F_\x83\x01\x85aK\rV[\x81\x81\x03` \x83\x01Rab\xA1\x81\x84ab V[\x90P\x93\x92PPPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rab\xC3\x81\x84\x86aP\x9FV[\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pab\xDF_\x83\x01\x87aG\xCAV[ab\xEC` \x83\x01\x86aG\xCAV[ab\xF9`@\x83\x01\x85aG\xCAV[ac\x06``\x83\x01\x84aG\xCAV[\x95\x94PPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[_` \x82\x01\x90PacO_\x83\x01\x84aYgV[\x92\x91PPV[`T\x81\x10acfWaceac\x0FV[[PV[_\x81\x90Pacv\x82acUV[\x91\x90PV[_ac\x85\x82aciV[\x90P\x91\x90PV[ac\x95\x81ac{V[\x82RPPV[_` \x82\x01\x90Pac\xAE_\x83\x01\x84ac\x8CV[\x92\x91PPV[_\x81\x90P\x92\x91PPV[ac\xC7\x81aB\xFDV[\x82RPPV[_ac\xD8\x83\x83ac\xBEV[` \x83\x01\x90P\x92\x91PPV[_ac\xEE\x82aW\x92V[ac\xF8\x81\x85ac\xB4V[\x93Pad\x03\x83aW\xACV[\x80_[\x83\x81\x10\x15ad3W\x81Qad\x1A\x88\x82ac\xCDV[\x97Pad%\x83aW\xE1V[\x92PP`\x01\x81\x01\x90Pad\x06V[P\x85\x93PPPP\x92\x91PPV[_adK\x82\x84ac\xE4V[\x91P\x81\x90P\x92\x91PPV[_`\xE0\x82\x01\x90Padi_\x83\x01\x8AaG\xCAV[adv` \x83\x01\x89aG\xCAV[ad\x83`@\x83\x01\x88aG\xCAV[ad\x90``\x83\x01\x87aK\rV[ad\x9D`\x80\x83\x01\x86aJ\xFEV[ad\xAA`\xA0\x83\x01\x85aJ\xFEV[ad\xB7`\xC0\x83\x01\x84aJ\xFEV[\x98\x97PPPPPPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ae\x16Wad\xE7\x81ad\xC3V[ad\xF0\x84aN4V[\x81\x01` \x85\x10\x15ad\xFFW\x81\x90P[ae\x13ae\x0B\x85aN4V[\x83\x01\x82aO\x17V[PP[PPPV[ae$\x82aE\x0FV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ae=Wae<aFCV[[aeG\x82TaM\xF2V[aeR\x82\x82\x85ad\xD5V[_` \x90P`\x1F\x83\x11`\x01\x81\x14ae\x83W_\x84\x15aeqW\x82\x87\x01Q\x90P[ae{\x85\x82aO\xA7V[\x86UPae\xE2V[`\x1F\x19\x84\x16ae\x91\x86ad\xC3V[_[\x82\x81\x10\x15ae\xB8W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pae\x93V[\x86\x83\x10\x15ae\xD5W\x84\x89\x01Qae\xD1`\x1F\x89\x16\x82aO\x8BV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\xC0\x82\x01\x90Pae\xFD_\x83\x01\x89aG\xCAV[af\n` \x83\x01\x88aG\xCAV[af\x17`@\x83\x01\x87aG\xCAV[af$``\x83\x01\x86aJ\xFEV[af1`\x80\x83\x01\x85aJ\xFEV[af>`\xA0\x83\x01\x84aJ\xFEV[\x97\x96PPPPPPPV[_\x81\x90P\x92\x91PPV[_af]\x82a\\\x0EV[afg\x81\x85afIV[\x93Pafw\x81\x85` \x86\x01aE)V[\x80\x84\x01\x91PP\x92\x91PPV[_af\x8E\x82\x84afSV[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Paf\xAC_\x83\x01\x88aG\xCAV[af\xB9` \x83\x01\x87aG\xCAV[af\xC6`@\x83\x01\x86aG\xCAV[af\xD3``\x83\x01\x85aJ\xFEV[af\xE0`\x80\x83\x01\x84aK\rV[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Paf\xFD_\x83\x01\x87aG\xCAV[ag\n` \x83\x01\x86aYgV[ag\x17`@\x83\x01\x85aG\xCAV[ag$``\x83\x01\x84aG\xCAV[\x95\x94PPPPPV\xFEDelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 contractsChainId,uint256 startTimestamp,uint256 durationDays)UserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare)",
    );
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FheType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<FheType> for u8 {
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
        impl FheType {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
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
        impl alloy_sol_types::SolType for FheType {
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
        impl alloy_sol_types::EventTopic for FheType {
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct CtHandleContractPair { bytes32 ctHandle; address contractAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CtHandleContractPair {
        #[allow(missing_docs)]
        pub ctHandle: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<CtHandleContractPair> for UnderlyingRustTuple<'_> {
            fn from(value: CtHandleContractPair) -> Self {
                (value.ctHandle, value.contractAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CtHandleContractPair {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ctHandle: tuple.0,
                    contractAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CtHandleContractPair {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CtHandleContractPair {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandle),
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
        impl alloy_sol_types::SolType for CtHandleContractPair {
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
        impl alloy_sol_types::SolStruct for CtHandleContractPair {
            const NAME: &'static str = "CtHandleContractPair";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CtHandleContractPair(bytes32 ctHandle,address contractAddress)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ctHandle)
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
        impl alloy_sol_types::EventTopic for CtHandleContractPair {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ctHandle,
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ctHandle,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct DelegationAccounts { address delegatorAddress; address delegatedAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelegationAccounts {
        #[allow(missing_docs)]
        pub delegatorAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delegatedAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<DelegationAccounts> for UnderlyingRustTuple<'_> {
            fn from(value: DelegationAccounts) -> Self {
                (value.delegatorAddress, value.delegatedAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegationAccounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delegatorAddress: tuple.0,
                    delegatedAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DelegationAccounts {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DelegationAccounts {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatorAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.delegatedAddress,
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
        impl alloy_sol_types::SolType for DelegationAccounts {
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
        impl alloy_sol_types::SolStruct for DelegationAccounts {
            const NAME: &'static str = "DelegationAccounts";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DelegationAccounts(address delegatorAddress,address delegatedAddress)",
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
                            &self.delegatorAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delegatedAddress,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DelegationAccounts {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatorAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delegatedAddress,
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
                    &rust.delegatorAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delegatedAddress,
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct SnsCiphertextMaterial { bytes32 ctHandle; uint256 keyId; bytes32 snsCiphertextDigest; address[] coprocessorTxSenderAddresses; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SnsCiphertextMaterial {
        #[allow(missing_docs)]
        pub ctHandle: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub snsCiphertextDigest: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub coprocessorTxSenderAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<SnsCiphertextMaterial> for UnderlyingRustTuple<'_> {
            fn from(value: SnsCiphertextMaterial) -> Self {
                (
                    value.ctHandle,
                    value.keyId,
                    value.snsCiphertextDigest,
                    value.coprocessorTxSenderAddresses,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SnsCiphertextMaterial {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ctHandle: tuple.0,
                    keyId: tuple.1,
                    snsCiphertextDigest: tuple.2,
                    coprocessorTxSenderAddresses: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SnsCiphertextMaterial {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SnsCiphertextMaterial {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandle),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.snsCiphertextDigest),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.coprocessorTxSenderAddresses,
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
        impl alloy_sol_types::SolType for SnsCiphertextMaterial {
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
        impl alloy_sol_types::SolStruct for SnsCiphertextMaterial {
            const NAME: &'static str = "SnsCiphertextMaterial";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SnsCiphertextMaterial(bytes32 ctHandle,uint256 keyId,bytes32 snsCiphertextDigest,address[] coprocessorTxSenderAddresses)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.ctHandle)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.keyId)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.snsCiphertextDigest,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.coprocessorTxSenderAddresses,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SnsCiphertextMaterial {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ctHandle,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.keyId)
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.snsCiphertextDigest,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.coprocessorTxSenderAddresses,
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ctHandle,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyId,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.snsCiphertextDigest,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.coprocessorTxSenderAddresses,
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ContractAddressesMaxLengthExceeded(uint8,uint256)` and selector `0xc5ab467e`.
```solidity
error ContractAddressesMaxLengthExceeded(uint8 maxLength, uint256 actualLength);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractAddressesMaxLengthExceeded {
        #[allow(missing_docs)]
        pub maxLength: u8,
        #[allow(missing_docs)]
        pub actualLength: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<8>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u8,
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
        impl ::core::convert::From<ContractAddressesMaxLengthExceeded>
        for UnderlyingRustTuple<'_> {
            fn from(value: ContractAddressesMaxLengthExceeded) -> Self {
                (value.maxLength, value.actualLength)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ContractAddressesMaxLengthExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxLength: tuple.0,
                    actualLength: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContractAddressesMaxLengthExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContractAddressesMaxLengthExceeded(uint8,uint256)";
            const SELECTOR: [u8; 4] = [197u8, 171u8, 70u8, 126u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxLength),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualLength),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ContractNotInContractAddresses(address,address[])` and selector `0xa4c30391`.
```solidity
error ContractNotInContractAddresses(address contractAddress, address[] contractAddresses);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractNotInContractAddresses {
        #[allow(missing_docs)]
        pub contractAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<ContractNotInContractAddresses>
        for UnderlyingRustTuple<'_> {
            fn from(value: ContractNotInContractAddresses) -> Self {
                (value.contractAddress, value.contractAddresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ContractNotInContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    contractAddress: tuple.0,
                    contractAddresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContractNotInContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContractNotInContractAddresses(address,address[])";
            const SELECTOR: [u8; 4] = [164u8, 195u8, 3u8, 145u8];
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
                        &self.contractAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DecryptionNotDone(uint256)` and selector `0x0bf01406`.
```solidity
error DecryptionNotDone(uint256 decryptionId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DecryptionNotDone {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<DecryptionNotDone> for UnderlyingRustTuple<'_> {
            fn from(value: DecryptionNotDone) -> Self {
                (value.decryptionId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DecryptionNotDone {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { decryptionId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DecryptionNotDone {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DecryptionNotDone(uint256)";
            const SELECTOR: [u8; 4] = [11u8, 240u8, 20u8, 6u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DelegatorAddressInContractAddresses(address,address[])` and selector `0xc3446ac7`.
```solidity
error DelegatorAddressInContractAddresses(address delegatorAddress, address[] contractAddresses);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelegatorAddressInContractAddresses {
        #[allow(missing_docs)]
        pub delegatorAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<DelegatorAddressInContractAddresses>
        for UnderlyingRustTuple<'_> {
            fn from(value: DelegatorAddressInContractAddresses) -> Self {
                (value.delegatorAddress, value.contractAddresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DelegatorAddressInContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delegatorAddress: tuple.0,
                    contractAddresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelegatorAddressInContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelegatorAddressInContractAddresses(address,address[])";
            const SELECTOR: [u8; 4] = [195u8, 68u8, 106u8, 199u8];
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
                        &self.delegatorAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `DifferentKeyIdsNotAllowed((bytes32,uint256,bytes32,address[]),(bytes32,uint256,bytes32,address[]))` and selector `0xcfae921f`.
```solidity
error DifferentKeyIdsNotAllowed(SnsCiphertextMaterial firstSnsCtMaterial, SnsCiphertextMaterial invalidSnsCtMaterial);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DifferentKeyIdsNotAllowed {
        #[allow(missing_docs)]
        pub firstSnsCtMaterial: <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub invalidSnsCtMaterial: <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (SnsCiphertextMaterial, SnsCiphertextMaterial);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DifferentKeyIdsNotAllowed>
        for UnderlyingRustTuple<'_> {
            fn from(value: DifferentKeyIdsNotAllowed) -> Self {
                (value.firstSnsCtMaterial, value.invalidSnsCtMaterial)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DifferentKeyIdsNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    firstSnsCtMaterial: tuple.0,
                    invalidSnsCtMaterial: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DifferentKeyIdsNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DifferentKeyIdsNotAllowed((bytes32,uint256,bytes32,address[]),(bytes32,uint256,bytes32,address[]))";
            const SELECTOR: [u8; 4] = [207u8, 174u8, 146u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <SnsCiphertextMaterial as alloy_sol_types::SolType>::tokenize(
                        &self.firstSnsCtMaterial,
                    ),
                    <SnsCiphertextMaterial as alloy_sol_types::SolType>::tokenize(
                        &self.invalidSnsCtMaterial,
                    ),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ECDSAInvalidSignature()` and selector `0xf645eedf`.
```solidity
error ECDSAInvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAInvalidSignature {}
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
                Self {}
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
        }
    };
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
        }
    };
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
        }
    };
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ERC1967NonPayable()` and selector `0xb398979f`.
```solidity
error ERC1967NonPayable();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ERC1967NonPayable {}
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyCtHandleContractPairs()` and selector `0xa6a6cb21`.
```solidity
error EmptyCtHandleContractPairs();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCtHandleContractPairs {}
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
        impl ::core::convert::From<EmptyCtHandleContractPairs>
        for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCtHandleContractPairs) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EmptyCtHandleContractPairs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCtHandleContractPairs {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCtHandleContractPairs()";
            const SELECTOR: [u8; 4] = [166u8, 166u8, 203u8, 33u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyCtHandles()` and selector `0x2de75438`.
```solidity
error EmptyCtHandles();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCtHandles {}
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
        impl ::core::convert::From<EmptyCtHandles> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyCtHandles) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyCtHandles {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyCtHandles {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyCtHandles()";
            const SELECTOR: [u8; 4] = [45u8, 231u8, 84u8, 56u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FailedCall()` and selector `0xd6bda275`.
```solidity
error FailedCall();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FailedCall {}
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidFHEType(uint8)` and selector `0x641950d7`.
```solidity
error InvalidFHEType(uint8 fheTypeUint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidFHEType {
        #[allow(missing_docs)]
        pub fheTypeUint8: u8,
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
        impl ::core::convert::From<InvalidFHEType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidFHEType) -> Self {
                (value.fheTypeUint8,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidFHEType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { fheTypeUint8: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidFHEType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidFHEType(uint8)";
            const SELECTOR: [u8; 4] = [100u8, 25u8, 80u8, 215u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.fheTypeUint8),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidInitialization()` and selector `0xf92ee8a9`.
```solidity
error InvalidInitialization();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidInitialization {}
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
                Self {}
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidNullDurationDays()` and selector `0xde2859c1`.
```solidity
error InvalidNullDurationDays();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullDurationDays {}
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
        impl ::core::convert::From<InvalidNullDurationDays> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullDurationDays) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNullDurationDays {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullDurationDays {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullDurationDays()";
            const SELECTOR: [u8; 4] = [222u8, 40u8, 89u8, 193u8];
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidUserSignature(bytes)` and selector `0x2a873d27`.
```solidity
error InvalidUserSignature(bytes signature);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidUserSignature {
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
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
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidUserSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidUserSignature) -> Self {
                (value.signature,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidUserSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { signature: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidUserSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidUserSignature(bytes)";
            const SELECTOR: [u8; 4] = [42u8, 135u8, 61u8, 39u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `KmsNodeAlreadySigned(uint256,address)` and selector `0x99ec48d9`.
```solidity
error KmsNodeAlreadySigned(uint256 decryptionId, address signer);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsNodeAlreadySigned {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KmsNodeAlreadySigned> for UnderlyingRustTuple<'_> {
            fn from(value: KmsNodeAlreadySigned) -> Self {
                (value.decryptionId, value.signer)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KmsNodeAlreadySigned {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    decryptionId: tuple.0,
                    signer: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsNodeAlreadySigned {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsNodeAlreadySigned(uint256,address)";
            const SELECTOR: [u8; 4] = [153u8, 236u8, 72u8, 217u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.signer,
                    ),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MaxDecryptionRequestBitSizeExceeded(uint256,uint256)` and selector `0xe7f4895d`.
```solidity
error MaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxDecryptionRequestBitSizeExceeded {
        #[allow(missing_docs)]
        pub maxBitSize: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub totalBitSize: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<MaxDecryptionRequestBitSizeExceeded>
        for UnderlyingRustTuple<'_> {
            fn from(value: MaxDecryptionRequestBitSizeExceeded) -> Self {
                (value.maxBitSize, value.totalBitSize)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MaxDecryptionRequestBitSizeExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxBitSize: tuple.0,
                    totalBitSize: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxDecryptionRequestBitSizeExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxDecryptionRequestBitSizeExceeded(uint256,uint256)";
            const SELECTOR: [u8; 4] = [231u8, 244u8, 137u8, 93u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitSize),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalBitSize),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MaxDurationDaysExceeded(uint256,uint256)` and selector `0x32951863`.
```solidity
error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxDurationDaysExceeded {
        #[allow(missing_docs)]
        pub maxValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub actualValue: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<MaxDurationDaysExceeded> for UnderlyingRustTuple<'_> {
            fn from(value: MaxDurationDaysExceeded) -> Self {
                (value.maxValue, value.actualValue)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MaxDurationDaysExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxValue: tuple.0,
                    actualValue: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxDurationDaysExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxDurationDaysExceeded(uint256,uint256)";
            const SELECTOR: [u8; 4] = [50u8, 149u8, 24u8, 99u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.maxValue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualValue),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotInitializing()` and selector `0xd7e6bcf8`.
```solidity
error NotInitializing();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotInitializing {}
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
                Self {}
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
        }
    };
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
        }
    };
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `StartTimestampInFuture(uint256,uint256)` and selector `0xf24c0887`.
```solidity
error StartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StartTimestampInFuture {
        #[allow(missing_docs)]
        pub currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<StartTimestampInFuture> for UnderlyingRustTuple<'_> {
            fn from(value: StartTimestampInFuture) -> Self {
                (value.currentTimestamp, value.startTimestamp)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StartTimestampInFuture {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentTimestamp: tuple.0,
                    startTimestamp: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StartTimestampInFuture {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StartTimestampInFuture(uint256,uint256)";
            const SELECTOR: [u8; 4] = [242u8, 76u8, 8u8, 135u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.currentTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`.
```solidity
error UUPSUnauthorizedCallContext();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UUPSUnauthorizedCallContext {}
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
                Self {}
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
        }
    };
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
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UnsupportedFHEType(uint8)` and selector `0xbe7830b1`.
```solidity
error UnsupportedFHEType(FheType fheType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnsupportedFHEType {
        #[allow(missing_docs)]
        pub fheType: <FheType as alloy::sol_types::SolType>::RustType,
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
        type UnderlyingSolTuple<'a> = (FheType,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <FheType as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnsupportedFHEType> for UnderlyingRustTuple<'_> {
            fn from(value: UnsupportedFHEType) -> Self {
                (value.fheType,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UnsupportedFHEType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { fheType: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnsupportedFHEType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnsupportedFHEType(uint8)";
            const SELECTOR: [u8; 4] = [190u8, 120u8, 48u8, 177u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<FheType as alloy_sol_types::SolType>::tokenize(&self.fheType),)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UserAddressInContractAddresses(address,address[])` and selector `0xdc4d78b1`.
```solidity
error UserAddressInContractAddresses(address userAddress, address[] contractAddresses);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserAddressInContractAddresses {
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<UserAddressInContractAddresses>
        for UnderlyingRustTuple<'_> {
            fn from(value: UserAddressInContractAddresses) -> Self {
                (value.userAddress, value.contractAddresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UserAddressInContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    userAddress: tuple.0,
                    contractAddresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UserAddressInContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UserAddressInContractAddresses(address,address[])";
            const SELECTOR: [u8; 4] = [220u8, 77u8, 120u8, 177u8];
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
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                )
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `UserDecryptionRequestExpired(uint256,(uint256,uint256))` and selector `0x30348040`.
```solidity
error UserDecryptionRequestExpired(uint256 currentTimestamp, IDecryption.RequestValidity requestValidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserDecryptionRequestExpired {
        #[allow(missing_docs)]
        pub currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
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
            IDecryption::RequestValidity,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UserDecryptionRequestExpired>
        for UnderlyingRustTuple<'_> {
            fn from(value: UserDecryptionRequestExpired) -> Self {
                (value.currentTimestamp, value.requestValidity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UserDecryptionRequestExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentTimestamp: tuple.0,
                    requestValidity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UserDecryptionRequestExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UserDecryptionRequestExpired(uint256,(uint256,uint256))";
            const SELECTOR: [u8; 4] = [48u8, 52u8, 128u8, 64u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.currentTimestamp),
                    <IDecryption::RequestValidity as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                )
            }
        }
    };
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
    pub struct EIP712DomainChanged {}
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[])` and selector `0x17c632196fbf6b96d9675971058d3701733094c3f2f1dcb9ba7d2a08bee0aafb`.
```solidity
event PublicDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionRequest {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub snsCtMaterials: alloy::sol_types::private::Vec<
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for PublicDecryptionRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<SnsCiphertextMaterial>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                23u8, 198u8, 50u8, 25u8, 111u8, 191u8, 107u8, 150u8, 217u8, 103u8, 89u8,
                113u8, 5u8, 141u8, 55u8, 1u8, 115u8, 48u8, 148u8, 195u8, 242u8, 241u8,
                220u8, 185u8, 186u8, 125u8, 42u8, 8u8, 190u8, 224u8, 170u8, 251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    snsCtMaterials: data.0,
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
                        SnsCiphertextMaterial,
                    > as alloy_sol_types::SolType>::tokenize(&self.snsCtMaterials),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionRequest,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionResponse(uint256,bytes,bytes[])` and selector `0x61568d6eb48e62870afffd55499206a54a8f78b04a627e00ed097161fc05d6be`.
```solidity
event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionResponse {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptedResult: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signatures: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
        impl alloy_sol_types::SolEvent for PublicDecryptionResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionResponse(uint256,bytes,bytes[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                97u8, 86u8, 141u8, 110u8, 180u8, 142u8, 98u8, 135u8, 10u8, 255u8, 253u8,
                85u8, 73u8, 146u8, 6u8, 165u8, 74u8, 143u8, 120u8, 176u8, 74u8, 98u8,
                126u8, 0u8, 237u8, 9u8, 113u8, 97u8, 252u8, 5u8, 214u8, 190u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    decryptedResult: data.0,
                    signatures: data.1,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptedResult,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.signatures),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionResponse,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes)` and selector `0x1c3dcad6311be6d58dc4d4b9f1bc1625eb18d72de969db75e11a88ef3527d2f3`.
```solidity
event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionRequest {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub snsCtMaterials: alloy::sol_types::private::Vec<
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for UserDecryptionRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<SnsCiphertextMaterial>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8, 61u8, 202u8, 214u8, 49u8, 27u8, 230u8, 213u8, 141u8, 196u8, 212u8,
                185u8, 241u8, 188u8, 22u8, 37u8, 235u8, 24u8, 215u8, 45u8, 233u8, 105u8,
                219u8, 117u8, 225u8, 26u8, 136u8, 239u8, 53u8, 39u8, 210u8, 243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    snsCtMaterials: data.0,
                    userAddress: data.1,
                    publicKey: data.2,
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
                        SnsCiphertextMaterial,
                    > as alloy_sol_types::SolType>::tokenize(&self.snsCtMaterials),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserDecryptionRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserDecryptionRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionResponse(uint256,bytes[],bytes[])` and selector `0x7312dec4cead0d5d3da836cdbaed1eb6a81e218c519c8740da4ac75afcb6c5c7`.
```solidity
event UserDecryptionResponse(uint256 indexed decryptionId, bytes[] userDecryptedShares, bytes[] signatures);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionResponse {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryptedShares: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        #[allow(missing_docs)]
        pub signatures: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
        impl alloy_sol_types::SolEvent for UserDecryptionResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionResponse(uint256,bytes[],bytes[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                115u8, 18u8, 222u8, 196u8, 206u8, 173u8, 13u8, 93u8, 61u8, 168u8, 54u8,
                205u8, 186u8, 237u8, 30u8, 182u8, 168u8, 30u8, 33u8, 140u8, 81u8, 156u8,
                135u8, 64u8, 218u8, 74u8, 199u8, 90u8, 252u8, 182u8, 197u8, 199u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    decryptionId: topics.1,
                    userDecryptedShares: data.0,
                    signatures: data.1,
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
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.userDecryptedShares),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.signatures),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.decryptionId.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.decryptionId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for UserDecryptionResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UserDecryptionResponse) -> alloy_sol_types::private::LogData {
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
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`.
```solidity
function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UPGRADE_INTERFACE_VERSIONCall {}
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
                    Self {}
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
            type Return = UPGRADE_INTERFACE_VERSIONReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `acceptOwnership()` and selector `0x79ba5097`.
```solidity
function acceptOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct acceptOwnershipCall {}
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
                    Self {}
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkDecryptionDone(uint256)` and selector `0xa6090439`.
```solidity
function checkDecryptionDone(uint256 decryptionId) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDecryptionDoneCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`checkDecryptionDone(uint256)`](checkDecryptionDoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDecryptionDoneReturn {}
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
            impl ::core::convert::From<checkDecryptionDoneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDecryptionDoneCall) -> Self {
                    (value.decryptionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDecryptionDoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { decryptionId: tuple.0 }
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
            impl ::core::convert::From<checkDecryptionDoneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDecryptionDoneReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDecryptionDoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkDecryptionDoneCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkDecryptionDoneReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkDecryptionDone(uint256)";
            const SELECTOR: [u8; 4] = [166u8, 9u8, 4u8, 57u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkDelegatedUserDecryptionReady(uint256,(address,address),(bytes32,address)[],address[])` and selector `0xf11d0638`.
```solidity
function checkDelegatedUserDecryptionReady(uint256 contractsChainId, DelegationAccounts memory delegationAccounts, CtHandleContractPair[] memory ctHandleContractPairs, address[] memory contractAddresses) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDelegatedUserDecryptionReadyCall {
        #[allow(missing_docs)]
        pub contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`checkDelegatedUserDecryptionReady(uint256,(address,address),(bytes32,address)[],address[])`](checkDelegatedUserDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkDelegatedUserDecryptionReadyReturn {}
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
                DelegationAccounts,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <DelegationAccounts as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<checkDelegatedUserDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDelegatedUserDecryptionReadyCall) -> Self {
                    (
                        value.contractsChainId,
                        value.delegationAccounts,
                        value.ctHandleContractPairs,
                        value.contractAddresses,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDelegatedUserDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        contractsChainId: tuple.0,
                        delegationAccounts: tuple.1,
                        ctHandleContractPairs: tuple.2,
                        contractAddresses: tuple.3,
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
            impl ::core::convert::From<checkDelegatedUserDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkDelegatedUserDecryptionReadyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkDelegatedUserDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkDelegatedUserDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                DelegationAccounts,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkDelegatedUserDecryptionReadyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkDelegatedUserDecryptionReady(uint256,(address,address),(bytes32,address)[],address[])";
            const SELECTOR: [u8; 4] = [241u8, 29u8, 6u8, 56u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.contractsChainId),
                    <DelegationAccounts as alloy_sol_types::SolType>::tokenize(
                        &self.delegationAccounts,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkPublicDecryptionReady(bytes32[])` and selector `0xaa39a356`.
```solidity
function checkPublicDecryptionReady(bytes32[] memory ctHandles) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkPublicDecryptionReadyCall {
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`checkPublicDecryptionReady(bytes32[])`](checkPublicDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkPublicDecryptionReadyReturn {}
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
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<checkPublicDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkPublicDecryptionReadyCall) -> Self {
                    (value.ctHandles,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkPublicDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ctHandles: tuple.0 }
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
            impl ::core::convert::From<checkPublicDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkPublicDecryptionReadyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkPublicDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkPublicDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkPublicDecryptionReadyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkPublicDecryptionReady(bytes32[])";
            const SELECTOR: [u8; 4] = [170u8, 57u8, 163u8, 86u8];
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
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandles),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkUserDecryptionReady(address,(bytes32,address)[])` and selector `0x008bc3e1`.
```solidity
function checkUserDecryptionReady(address userAddress, CtHandleContractPair[] memory ctHandleContractPairs) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkUserDecryptionReadyCall {
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`checkUserDecryptionReady(address,(bytes32,address)[])`](checkUserDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkUserDecryptionReadyReturn {}
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
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkUserDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkUserDecryptionReadyCall) -> Self {
                    (value.userAddress, value.ctHandleContractPairs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkUserDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        userAddress: tuple.0,
                        ctHandleContractPairs: tuple.1,
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
            impl ::core::convert::From<checkUserDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkUserDecryptionReadyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkUserDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkUserDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkUserDecryptionReadyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkUserDecryptionReady(address,(bytes32,address)[])";
            const SELECTOR: [u8; 4] = [0u8, 139u8, 195u8, 225u8];
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
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandleContractPairs),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),uint256,address[],bytes,bytes)` and selector `0x760a0419`.
```solidity
function delegatedUserDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, DelegationAccounts memory delegationAccounts, uint256 contractsChainId, address[] memory contractAddresses, bytes memory publicKey, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedUserDecryptionRequestCall {
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),uint256,address[],bytes,bytes)`](delegatedUserDecryptionRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegatedUserDecryptionRequestReturn {}
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
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                DelegationAccounts,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
                <DelegationAccounts as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<delegatedUserDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegatedUserDecryptionRequestCall) -> Self {
                    (
                        value.ctHandleContractPairs,
                        value.requestValidity,
                        value.delegationAccounts,
                        value.contractsChainId,
                        value.contractAddresses,
                        value.publicKey,
                        value.signature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegatedUserDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandleContractPairs: tuple.0,
                        requestValidity: tuple.1,
                        delegationAccounts: tuple.2,
                        contractsChainId: tuple.3,
                        contractAddresses: tuple.4,
                        publicKey: tuple.5,
                        signature: tuple.6,
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
            impl ::core::convert::From<delegatedUserDecryptionRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegatedUserDecryptionRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegatedUserDecryptionRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatedUserDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                DelegationAccounts,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegatedUserDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),uint256,address[],bytes,bytes)";
            const SELECTOR: [u8; 4] = [118u8, 10u8, 4u8, 25u8];
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
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <IDecryption::RequestValidity as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                    <DelegationAccounts as alloy_sol_types::SolType>::tokenize(
                        &self.delegationAccounts,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractsChainId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
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
                    Self {}
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getVersion()` and selector `0x0d8e6e2c`.
```solidity
function getVersion() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getVersionCall {}
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
                    Self {}
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
            type Return = getVersionReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize()` and selector `0x8129fc1c`.
```solidity
function initialize() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {}
    ///Container type for the return parameters of the [`initialize()`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize()";
            const SELECTOR: [u8; 4] = [129u8, 41u8, 252u8, 28u8];
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
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
                    Self {}
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
            type Return = ownerReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pendingOwner()` and selector `0xe30c3978`.
```solidity
function pendingOwner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pendingOwnerCall {}
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
                    Self {}
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
            type Return = pendingOwnerReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proxiableUUID()` and selector `0x52d1902d`.
```solidity
function proxiableUUID() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proxiableUUIDCall {}
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
                    Self {}
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
            type Return = proxiableUUIDReturn;
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `publicDecryptionRequest(bytes32[])` and selector `0x187fe529`.
```solidity
function publicDecryptionRequest(bytes32[] memory ctHandles) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionRequestCall {
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`publicDecryptionRequest(bytes32[])`](publicDecryptionRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionRequestReturn {}
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
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<publicDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionRequestCall) -> Self {
                    (value.ctHandles,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { ctHandles: tuple.0 }
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
            impl ::core::convert::From<publicDecryptionRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for publicDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = publicDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "publicDecryptionRequest(bytes32[])";
            const SELECTOR: [u8; 4] = [24u8, 127u8, 229u8, 41u8];
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
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandles),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `publicDecryptionResponse(uint256,bytes,bytes)` and selector `0x02fd1a64`.
```solidity
function publicDecryptionResponse(uint256 decryptionId, bytes memory decryptedResult, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionResponseCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptedResult: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`publicDecryptionResponse(uint256,bytes,bytes)`](publicDecryptionResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionResponseReturn {}
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
            impl ::core::convert::From<publicDecryptionResponseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionResponseCall) -> Self {
                    (value.decryptionId, value.decryptedResult, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        decryptionId: tuple.0,
                        decryptedResult: tuple.1,
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
            impl ::core::convert::From<publicDecryptionResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for publicDecryptionResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = publicDecryptionResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "publicDecryptionResponse(uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [2u8, 253u8, 26u8, 100u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.decryptedResult,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
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
                    Self {}
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
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
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `userDecryptionRequest((bytes32,address)[],(uint256,uint256),uint256,address[],address,bytes,bytes)` and selector `0x8316001f`.
```solidity
function userDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, uint256 contractsChainId, address[] memory contractAddresses, address userAddress, bytes memory publicKey, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequestCall {
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub contractAddresses: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`userDecryptionRequest((bytes32,address)[],(uint256,uint256),uint256,address[],address,bytes,bytes)`](userDecryptionRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequestReturn {}
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
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<userDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequestCall) -> Self {
                    (
                        value.ctHandleContractPairs,
                        value.requestValidity,
                        value.contractsChainId,
                        value.contractAddresses,
                        value.userAddress,
                        value.publicKey,
                        value.signature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandleContractPairs: tuple.0,
                        requestValidity: tuple.1,
                        contractsChainId: tuple.2,
                        contractAddresses: tuple.3,
                        userAddress: tuple.4,
                        publicKey: tuple.5,
                        signature: tuple.6,
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
            impl ::core::convert::From<userDecryptionRequestReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userDecryptionRequest((bytes32,address)[],(uint256,uint256),uint256,address[],address,bytes,bytes)";
            const SELECTOR: [u8; 4] = [131u8, 22u8, 0u8, 31u8];
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
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <IDecryption::RequestValidity as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractsChainId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractAddresses),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `userDecryptionResponse(uint256,bytes,bytes)` and selector `0xb9bfe0a8`.
```solidity
function userDecryptionResponse(uint256 decryptionId, bytes memory userDecryptedShare, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionResponseCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryptedShare: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`userDecryptionResponse(uint256,bytes,bytes)`](userDecryptionResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionResponseReturn {}
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
            impl ::core::convert::From<userDecryptionResponseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionResponseCall) -> Self {
                    (value.decryptionId, value.userDecryptedShare, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        decryptionId: tuple.0,
                        userDecryptedShare: tuple.1,
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
            impl ::core::convert::From<userDecryptionResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDecryptionResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userDecryptionResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userDecryptionResponse(uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [185u8, 191u8, 224u8, 168u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.decryptionId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.userDecryptedShare,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`Decryption`](self) function calls.
    #[derive()]
    pub enum DecryptionCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        acceptOwnership(acceptOwnershipCall),
        #[allow(missing_docs)]
        checkDecryptionDone(checkDecryptionDoneCall),
        #[allow(missing_docs)]
        checkDelegatedUserDecryptionReady(checkDelegatedUserDecryptionReadyCall),
        #[allow(missing_docs)]
        checkPublicDecryptionReady(checkPublicDecryptionReadyCall),
        #[allow(missing_docs)]
        checkUserDecryptionReady(checkUserDecryptionReadyCall),
        #[allow(missing_docs)]
        delegatedUserDecryptionRequest(delegatedUserDecryptionRequestCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pendingOwner(pendingOwnerCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        publicDecryptionRequest(publicDecryptionRequestCall),
        #[allow(missing_docs)]
        publicDecryptionResponse(publicDecryptionResponseCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        userDecryptionRequest(userDecryptionRequestCall),
        #[allow(missing_docs)]
        userDecryptionResponse(userDecryptionResponseCall),
    }
    #[automatically_derived]
    impl DecryptionCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 139u8, 195u8, 225u8],
            [2u8, 253u8, 26u8, 100u8],
            [13u8, 142u8, 110u8, 44u8],
            [24u8, 127u8, 229u8, 41u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [113u8, 80u8, 24u8, 166u8],
            [118u8, 10u8, 4u8, 25u8],
            [121u8, 186u8, 80u8, 151u8],
            [129u8, 41u8, 252u8, 28u8],
            [131u8, 22u8, 0u8, 31u8],
            [132u8, 176u8, 25u8, 110u8],
            [141u8, 165u8, 203u8, 91u8],
            [166u8, 9u8, 4u8, 57u8],
            [170u8, 57u8, 163u8, 86u8],
            [173u8, 60u8, 177u8, 204u8],
            [185u8, 191u8, 224u8, 168u8],
            [227u8, 12u8, 57u8, 120u8],
            [241u8, 29u8, 6u8, 56u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionCalls {
        const NAME: &'static str = "DecryptionCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::acceptOwnership(_) => {
                    <acceptOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkDecryptionDone(_) => {
                    <checkDecryptionDoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkDelegatedUserDecryptionReady(_) => {
                    <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkPublicDecryptionReady(_) => {
                    <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkUserDecryptionReady(_) => {
                    <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegatedUserDecryptionRequest(_) => {
                    <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pendingOwner(_) => {
                    <pendingOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionRequest(_) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionResponse(_) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userDecryptionRequest(_) => {
                    <userDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userDecryptionResponse(_) => {
                    <userDecryptionResponseCall as alloy_sol_types::SolCall>::SELECTOR
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<DecryptionCalls>] = &[
                {
                    fn checkUserDecryptionReady(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::checkUserDecryptionReady)
                    }
                    checkUserDecryptionReady
                },
                {
                    fn publicDecryptionResponse(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::publicDecryptionResponse)
                    }
                    publicDecryptionResponse
                },
                {
                    fn getVersion(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn publicDecryptionRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::publicDecryptionRequest)
                    }
                    publicDecryptionRequest
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn delegatedUserDecryptionRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::delegatedUserDecryptionRequest)
                    }
                    delegatedUserDecryptionRequest
                },
                {
                    fn acceptOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <acceptOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::acceptOwnership)
                    }
                    acceptOwnership
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::initialize)
                    }
                    initialize
                },
                {
                    fn userDecryptionRequest(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::userDecryptionRequest)
                    }
                    userDecryptionRequest
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::owner)
                    }
                    owner
                },
                {
                    fn checkDecryptionDone(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkDecryptionDoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::checkDecryptionDone)
                    }
                    checkDecryptionDone
                },
                {
                    fn checkPublicDecryptionReady(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::checkPublicDecryptionReady)
                    }
                    checkPublicDecryptionReady
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn userDecryptionResponse(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::userDecryptionResponse)
                    }
                    userDecryptionResponse
                },
                {
                    fn pendingOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pendingOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::pendingOwner)
                    }
                    pendingOwner
                },
                {
                    fn checkDelegatedUserDecryptionReady(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::checkDelegatedUserDecryptionReady)
                    }
                    checkDelegatedUserDecryptionReady
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionCalls::transferOwnership)
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
            DECODE_SHIMS[idx](data, validate)
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
                Self::checkDecryptionDone(inner) => {
                    <checkDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkDelegatedUserDecryptionReady(inner) => {
                    <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkPublicDecryptionReady(inner) => {
                    <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkUserDecryptionReady(inner) => {
                    <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegatedUserDecryptionRequest(inner) => {
                    <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getVersion(inner) => {
                    <getVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::publicDecryptionRequest(inner) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::publicDecryptionResponse(inner) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userDecryptionRequest(inner) => {
                    <userDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userDecryptionResponse(inner) => {
                    <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::checkDecryptionDone(inner) => {
                    <checkDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkDelegatedUserDecryptionReady(inner) => {
                    <checkDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkPublicDecryptionReady(inner) => {
                    <checkPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkUserDecryptionReady(inner) => {
                    <checkUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegatedUserDecryptionRequest(inner) => {
                    <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getVersion(inner) => {
                    <getVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::publicDecryptionRequest(inner) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::publicDecryptionResponse(inner) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userDecryptionRequest(inner) => {
                    <userDecryptionRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userDecryptionResponse(inner) => {
                    <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Decryption`](self) custom errors.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DecryptionErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        ContractAddressesMaxLengthExceeded(ContractAddressesMaxLengthExceeded),
        #[allow(missing_docs)]
        ContractNotInContractAddresses(ContractNotInContractAddresses),
        #[allow(missing_docs)]
        DecryptionNotDone(DecryptionNotDone),
        #[allow(missing_docs)]
        DelegatorAddressInContractAddresses(DelegatorAddressInContractAddresses),
        #[allow(missing_docs)]
        DifferentKeyIdsNotAllowed(DifferentKeyIdsNotAllowed),
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
        EmptyCtHandleContractPairs(EmptyCtHandleContractPairs),
        #[allow(missing_docs)]
        EmptyCtHandles(EmptyCtHandles),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidFHEType(InvalidFHEType),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidNullDurationDays(InvalidNullDurationDays),
        #[allow(missing_docs)]
        InvalidUserSignature(InvalidUserSignature),
        #[allow(missing_docs)]
        KmsNodeAlreadySigned(KmsNodeAlreadySigned),
        #[allow(missing_docs)]
        MaxDecryptionRequestBitSizeExceeded(MaxDecryptionRequestBitSizeExceeded),
        #[allow(missing_docs)]
        MaxDurationDaysExceeded(MaxDurationDaysExceeded),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        OwnableInvalidOwner(OwnableInvalidOwner),
        #[allow(missing_docs)]
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        #[allow(missing_docs)]
        StartTimestampInFuture(StartTimestampInFuture),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnsupportedFHEType(UnsupportedFHEType),
        #[allow(missing_docs)]
        UserAddressInContractAddresses(UserAddressInContractAddresses),
        #[allow(missing_docs)]
        UserDecryptionRequestExpired(UserDecryptionRequestExpired),
    }
    #[automatically_derived]
    impl DecryptionErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 240u8, 20u8, 6u8],
            [17u8, 140u8, 218u8, 167u8],
            [30u8, 79u8, 189u8, 247u8],
            [42u8, 135u8, 61u8, 39u8],
            [45u8, 231u8, 84u8, 56u8],
            [48u8, 52u8, 128u8, 64u8],
            [50u8, 149u8, 24u8, 99u8],
            [76u8, 156u8, 140u8, 227u8],
            [100u8, 25u8, 80u8, 215u8],
            [153u8, 150u8, 179u8, 21u8],
            [153u8, 236u8, 72u8, 217u8],
            [164u8, 195u8, 3u8, 145u8],
            [166u8, 166u8, 203u8, 33u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [190u8, 120u8, 48u8, 177u8],
            [195u8, 68u8, 106u8, 199u8],
            [197u8, 171u8, 70u8, 126u8],
            [207u8, 174u8, 146u8, 31u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [220u8, 77u8, 120u8, 177u8],
            [222u8, 40u8, 89u8, 193u8],
            [224u8, 124u8, 141u8, 186u8],
            [231u8, 244u8, 137u8, 93u8],
            [242u8, 76u8, 8u8, 135u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionErrors {
        const NAME: &'static str = "DecryptionErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractAddressesMaxLengthExceeded(_) => {
                    <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractNotInContractAddresses(_) => {
                    <ContractNotInContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DecryptionNotDone(_) => {
                    <DecryptionNotDone as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelegatorAddressInContractAddresses(_) => {
                    <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DifferentKeyIdsNotAllowed(_) => {
                    <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyCtHandleContractPairs(_) => {
                    <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCtHandles(_) => {
                    <EmptyCtHandles as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidFHEType(_) => {
                    <InvalidFHEType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullDurationDays(_) => {
                    <InvalidNullDurationDays as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidUserSignature(_) => {
                    <InvalidUserSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsNodeAlreadySigned(_) => {
                    <KmsNodeAlreadySigned as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDecryptionRequestBitSizeExceeded(_) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDurationDaysExceeded(_) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotInitializing(_) => {
                    <NotInitializing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableInvalidOwner(_) => {
                    <OwnableInvalidOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OwnableUnauthorizedAccount(_) => {
                    <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StartTimestampInFuture(_) => {
                    <StartTimestampInFuture as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnauthorizedCallContext(_) => {
                    <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UUPSUnsupportedProxiableUUID(_) => {
                    <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnsupportedFHEType(_) => {
                    <UnsupportedFHEType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UserAddressInContractAddresses(_) => {
                    <UserAddressInContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UserDecryptionRequestExpired(_) => {
                    <UserDecryptionRequestExpired as alloy_sol_types::SolError>::SELECTOR
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
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<DecryptionErrors>] = &[
                {
                    fn DecryptionNotDone(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionNotDone as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::DecryptionNotDone)
                    }
                    DecryptionNotDone
                },
                {
                    fn OwnableUnauthorizedAccount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <OwnableUnauthorizedAccount as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::OwnableUnauthorizedAccount)
                    }
                    OwnableUnauthorizedAccount
                },
                {
                    fn OwnableInvalidOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <OwnableInvalidOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::OwnableInvalidOwner)
                    }
                    OwnableInvalidOwner
                },
                {
                    fn InvalidUserSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidUserSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::InvalidUserSignature)
                    }
                    InvalidUserSignature
                },
                {
                    fn EmptyCtHandles(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandles as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::EmptyCtHandles)
                    }
                    EmptyCtHandles
                },
                {
                    fn UserDecryptionRequestExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::UserDecryptionRequestExpired)
                    }
                    UserDecryptionRequestExpired
                },
                {
                    fn MaxDurationDaysExceeded(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::MaxDurationDaysExceeded)
                    }
                    MaxDurationDaysExceeded
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn InvalidFHEType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidFHEType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::InvalidFHEType)
                    }
                    InvalidFHEType
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn KmsNodeAlreadySigned(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::KmsNodeAlreadySigned)
                    }
                    KmsNodeAlreadySigned
                },
                {
                    fn ContractNotInContractAddresses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::ContractNotInContractAddresses)
                    }
                    ContractNotInContractAddresses
                },
                {
                    fn EmptyCtHandleContractPairs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::EmptyCtHandleContractPairs)
                    }
                    EmptyCtHandleContractPairs
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn UnsupportedFHEType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UnsupportedFHEType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::UnsupportedFHEType)
                    }
                    UnsupportedFHEType
                },
                {
                    fn DelegatorAddressInContractAddresses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::DelegatorAddressInContractAddresses)
                    }
                    DelegatorAddressInContractAddresses
                },
                {
                    fn ContractAddressesMaxLengthExceeded(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::ContractAddressesMaxLengthExceeded)
                    }
                    ContractAddressesMaxLengthExceeded
                },
                {
                    fn DifferentKeyIdsNotAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::DifferentKeyIdsNotAllowed)
                    }
                    DifferentKeyIdsNotAllowed
                },
                {
                    fn FailedCall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UserAddressInContractAddresses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::UserAddressInContractAddresses)
                    }
                    UserAddressInContractAddresses
                },
                {
                    fn InvalidNullDurationDays(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::InvalidNullDurationDays)
                    }
                    InvalidNullDurationDays
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn MaxDecryptionRequestBitSizeExceeded(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::MaxDecryptionRequestBitSizeExceeded)
                    }
                    MaxDecryptionRequestBitSizeExceeded
                },
                {
                    fn StartTimestampInFuture(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <StartTimestampInFuture as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::StartTimestampInFuture)
                    }
                    StartTimestampInFuture
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureLength)
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractAddressesMaxLengthExceeded(inner) => {
                    <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractNotInContractAddresses(inner) => {
                    <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DecryptionNotDone(inner) => {
                    <DecryptionNotDone as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DelegatorAddressInContractAddresses(inner) => {
                    <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DifferentKeyIdsNotAllowed(inner) => {
                    <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyCtHandleContractPairs(inner) => {
                    <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyCtHandles(inner) => {
                    <EmptyCtHandles as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidFHEType(inner) => {
                    <InvalidFHEType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullDurationDays(inner) => {
                    <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidUserSignature(inner) => {
                    <InvalidUserSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsNodeAlreadySigned(inner) => {
                    <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxDecryptionRequestBitSizeExceeded(inner) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MaxDurationDaysExceeded(inner) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotInitializing(inner) => {
                    <NotInitializing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::StartTimestampInFuture(inner) => {
                    <StartTimestampInFuture as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::UnsupportedFHEType(inner) => {
                    <UnsupportedFHEType as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UserAddressInContractAddresses(inner) => {
                    <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UserDecryptionRequestExpired(inner) => {
                    <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ContractAddressesMaxLengthExceeded(inner) => {
                    <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContractNotInContractAddresses(inner) => {
                    <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DecryptionNotDone(inner) => {
                    <DecryptionNotDone as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelegatorAddressInContractAddresses(inner) => {
                    <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DifferentKeyIdsNotAllowed(inner) => {
                    <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyCtHandleContractPairs(inner) => {
                    <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyCtHandles(inner) => {
                    <EmptyCtHandles as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidFHEType(inner) => {
                    <InvalidFHEType as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidNullDurationDays(inner) => {
                    <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidUserSignature(inner) => {
                    <InvalidUserSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsNodeAlreadySigned(inner) => {
                    <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxDecryptionRequestBitSizeExceeded(inner) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MaxDurationDaysExceeded(inner) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::StartTimestampInFuture(inner) => {
                    <StartTimestampInFuture as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UnsupportedFHEType(inner) => {
                    <UnsupportedFHEType as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UserAddressInContractAddresses(inner) => {
                    <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UserDecryptionRequestExpired(inner) => {
                    <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Decryption`](self) events.
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DecryptionEvents {
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OwnershipTransferStarted(OwnershipTransferStarted),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        PublicDecryptionRequest(PublicDecryptionRequest),
        #[allow(missing_docs)]
        PublicDecryptionResponse(PublicDecryptionResponse),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        UserDecryptionRequest(UserDecryptionRequest),
        #[allow(missing_docs)]
        UserDecryptionResponse(UserDecryptionResponse),
    }
    #[automatically_derived]
    impl DecryptionEvents {
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
                23u8, 198u8, 50u8, 25u8, 111u8, 191u8, 107u8, 150u8, 217u8, 103u8, 89u8,
                113u8, 5u8, 141u8, 55u8, 1u8, 115u8, 48u8, 148u8, 195u8, 242u8, 241u8,
                220u8, 185u8, 186u8, 125u8, 42u8, 8u8, 190u8, 224u8, 170u8, 251u8,
            ],
            [
                28u8, 61u8, 202u8, 214u8, 49u8, 27u8, 230u8, 213u8, 141u8, 196u8, 212u8,
                185u8, 241u8, 188u8, 22u8, 37u8, 235u8, 24u8, 215u8, 45u8, 233u8, 105u8,
                219u8, 117u8, 225u8, 26u8, 136u8, 239u8, 53u8, 39u8, 210u8, 243u8,
            ],
            [
                56u8, 209u8, 107u8, 140u8, 172u8, 34u8, 217u8, 159u8, 199u8, 193u8, 36u8,
                185u8, 205u8, 13u8, 226u8, 211u8, 250u8, 31u8, 174u8, 244u8, 32u8, 191u8,
                231u8, 145u8, 216u8, 195u8, 98u8, 215u8, 101u8, 226u8, 39u8, 0u8,
            ],
            [
                97u8, 86u8, 141u8, 110u8, 180u8, 142u8, 98u8, 135u8, 10u8, 255u8, 253u8,
                85u8, 73u8, 146u8, 6u8, 165u8, 74u8, 143u8, 120u8, 176u8, 74u8, 98u8,
                126u8, 0u8, 237u8, 9u8, 113u8, 97u8, 252u8, 5u8, 214u8, 190u8,
            ],
            [
                115u8, 18u8, 222u8, 196u8, 206u8, 173u8, 13u8, 93u8, 61u8, 168u8, 54u8,
                205u8, 186u8, 237u8, 30u8, 182u8, 168u8, 30u8, 33u8, 140u8, 81u8, 156u8,
                135u8, 64u8, 218u8, 74u8, 199u8, 90u8, 252u8, 182u8, 197u8, 199u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8,
                31u8, 208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8,
                218u8, 175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
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
    impl alloy_sol_types::SolEventInterface for DecryptionEvents {
        const NAME: &'static str = "DecryptionEvents";
        const COUNT: usize = 9usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EIP712DomainChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EIP712DomainChanged)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferStarted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferStarted)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <PublicDecryptionRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PublicDecryptionRequest)
                }
                Some(
                    <PublicDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PublicDecryptionResponse)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Upgraded)
                }
                Some(
                    <UserDecryptionRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserDecryptionRequest)
                }
                Some(
                    <UserDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UserDecryptionResponse)
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
    impl alloy_sol_types::private::IntoLogData for DecryptionEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EIP712DomainChanged(inner) => {
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
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionResponse(inner) => {
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
                Self::OwnershipTransferStarted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`Decryption`](self) contract instance.

See the [wrapper's documentation](`DecryptionInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DecryptionInstance<T, P, N> {
        DecryptionInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DecryptionInstance<T, P, N>>,
    > {
        DecryptionInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        DecryptionInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`Decryption`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Decryption`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DecryptionInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DecryptionInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DecryptionInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Decryption`](self) contract instance.

See the [wrapper's documentation](`DecryptionInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<DecryptionInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
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
    impl<T, P: ::core::clone::Clone, N> DecryptionInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DecryptionInstance<T, P, N> {
            DecryptionInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`UPGRADE_INTERFACE_VERSION`] function.
        pub fn UPGRADE_INTERFACE_VERSION(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, UPGRADE_INTERFACE_VERSIONCall, N> {
            self.call_builder(&UPGRADE_INTERFACE_VERSIONCall {})
        }
        ///Creates a new call builder for the [`acceptOwnership`] function.
        pub fn acceptOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, acceptOwnershipCall, N> {
            self.call_builder(&acceptOwnershipCall {})
        }
        ///Creates a new call builder for the [`checkDecryptionDone`] function.
        pub fn checkDecryptionDone(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkDecryptionDoneCall, N> {
            self.call_builder(
                &checkDecryptionDoneCall {
                    decryptionId,
                },
            )
        }
        ///Creates a new call builder for the [`checkDelegatedUserDecryptionReady`] function.
        pub fn checkDelegatedUserDecryptionReady(
            &self,
            contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
            delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            contractAddresses: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            checkDelegatedUserDecryptionReadyCall,
            N,
        > {
            self.call_builder(
                &checkDelegatedUserDecryptionReadyCall {
                    contractsChainId,
                    delegationAccounts,
                    ctHandleContractPairs,
                    contractAddresses,
                },
            )
        }
        ///Creates a new call builder for the [`checkPublicDecryptionReady`] function.
        pub fn checkPublicDecryptionReady(
            &self,
            ctHandles: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkPublicDecryptionReadyCall, N> {
            self.call_builder(
                &checkPublicDecryptionReadyCall {
                    ctHandles,
                },
            )
        }
        ///Creates a new call builder for the [`checkUserDecryptionReady`] function.
        pub fn checkUserDecryptionReady(
            &self,
            userAddress: alloy::sol_types::private::Address,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkUserDecryptionReadyCall, N> {
            self.call_builder(
                &checkUserDecryptionReadyCall {
                    userAddress,
                    ctHandleContractPairs,
                },
            )
        }
        ///Creates a new call builder for the [`delegatedUserDecryptionRequest`] function.
        pub fn delegatedUserDecryptionRequest(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
            delegationAccounts: <DelegationAccounts as alloy::sol_types::SolType>::RustType,
            contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
            contractAddresses: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            publicKey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            delegatedUserDecryptionRequestCall,
            N,
        > {
            self.call_builder(
                &delegatedUserDecryptionRequestCall {
                    ctHandleContractPairs,
                    requestValidity,
                    delegationAccounts,
                    contractsChainId,
                    contractAddresses,
                    publicKey,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`getVersion`] function.
        pub fn getVersion(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getVersionCall, N> {
            self.call_builder(&getVersionCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`pendingOwner`] function.
        pub fn pendingOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, pendingOwnerCall, N> {
            self.call_builder(&pendingOwnerCall {})
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall {})
        }
        ///Creates a new call builder for the [`publicDecryptionRequest`] function.
        pub fn publicDecryptionRequest(
            &self,
            ctHandles: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, publicDecryptionRequestCall, N> {
            self.call_builder(
                &publicDecryptionRequestCall {
                    ctHandles,
                },
            )
        }
        ///Creates a new call builder for the [`publicDecryptionResponse`] function.
        pub fn publicDecryptionResponse(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
            decryptedResult: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, publicDecryptionResponseCall, N> {
            self.call_builder(
                &publicDecryptionResponseCall {
                    decryptionId,
                    decryptedResult,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`upgradeToAndCall`] function.
        pub fn upgradeToAndCall(
            &self,
            newImplementation: alloy::sol_types::private::Address,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, upgradeToAndCallCall, N> {
            self.call_builder(
                &upgradeToAndCallCall {
                    newImplementation,
                    data,
                },
            )
        }
        ///Creates a new call builder for the [`userDecryptionRequest`] function.
        pub fn userDecryptionRequest(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
            contractsChainId: alloy::sol_types::private::primitives::aliases::U256,
            contractAddresses: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            userAddress: alloy::sol_types::private::Address,
            publicKey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, userDecryptionRequestCall, N> {
            self.call_builder(
                &userDecryptionRequestCall {
                    ctHandleContractPairs,
                    requestValidity,
                    contractsChainId,
                    contractAddresses,
                    userAddress,
                    publicKey,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`userDecryptionResponse`] function.
        pub fn userDecryptionResponse(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
            userDecryptedShare: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, userDecryptionResponseCall, N> {
            self.call_builder(
                &userDecryptionResponseCall {
                    decryptionId,
                    userDecryptedShare,
                    signature,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`EIP712DomainChanged`] event.
        pub fn EIP712DomainChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EIP712DomainChanged, N> {
            self.event_filter::<EIP712DomainChanged>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OwnershipTransferStarted`] event.
        pub fn OwnershipTransferStarted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferStarted, N> {
            self.event_filter::<OwnershipTransferStarted>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`PublicDecryptionRequest`] event.
        pub fn PublicDecryptionRequest_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PublicDecryptionRequest, N> {
            self.event_filter::<PublicDecryptionRequest>()
        }
        ///Creates a new event filter for the [`PublicDecryptionResponse`] event.
        pub fn PublicDecryptionResponse_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PublicDecryptionResponse, N> {
            self.event_filter::<PublicDecryptionResponse>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<T, &P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
        ///Creates a new event filter for the [`UserDecryptionRequest`] event.
        pub fn UserDecryptionRequest_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserDecryptionRequest, N> {
            self.event_filter::<UserDecryptionRequest>()
        }
        ///Creates a new event filter for the [`UserDecryptionResponse`] event.
        pub fn UserDecryptionResponse_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UserDecryptionResponse, N> {
            self.event_filter::<UserDecryptionResponse>()
        }
    }
}
