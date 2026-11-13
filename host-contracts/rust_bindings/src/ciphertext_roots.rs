///Module containing a contract's types and functions.
/**

```solidity
library ICiphertextRoots {
    struct EpochSchedule { uint256 startBlock; EpochScheduleEntry[] entries; }
    struct EpochScheduleEntry { uint256 fromEpoch; uint256 epochLength; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ICiphertextRoots {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct EpochSchedule { uint256 startBlock; EpochScheduleEntry[] entries; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochSchedule {
        #[allow(missing_docs)]
        pub startBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub entries: alloy::sol_types::private::Vec<
            <EpochScheduleEntry as alloy::sol_types::SolType>::RustType,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<EpochScheduleEntry>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Vec<
                <EpochScheduleEntry as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<EpochSchedule> for UnderlyingRustTuple<'_> {
            fn from(value: EpochSchedule) -> Self {
                (value.startBlock, value.entries)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochSchedule {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    startBlock: tuple.0,
                    entries: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for EpochSchedule {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for EpochSchedule {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Array<
                        EpochScheduleEntry,
                    > as alloy_sol_types::SolType>::tokenize(&self.entries),
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
        impl alloy_sol_types::SolType for EpochSchedule {
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
        impl alloy_sol_types::SolStruct for EpochSchedule {
            const NAME: &'static str = "EpochSchedule";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "EpochSchedule(uint256 startBlock,EpochScheduleEntry[] entries)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <EpochScheduleEntry as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <EpochScheduleEntry as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.startBlock)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        EpochScheduleEntry,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.entries)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for EpochSchedule {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startBlock,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        EpochScheduleEntry,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.entries,
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
                    &rust.startBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    EpochScheduleEntry,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.entries,
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
struct EpochScheduleEntry { uint256 fromEpoch; uint256 epochLength; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochScheduleEntry {
        #[allow(missing_docs)]
        pub fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochLength: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EpochScheduleEntry> for UnderlyingRustTuple<'_> {
            fn from(value: EpochScheduleEntry) -> Self {
                (value.fromEpoch, value.epochLength)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochScheduleEntry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    fromEpoch: tuple.0,
                    epochLength: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for EpochScheduleEntry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for EpochScheduleEntry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromEpoch),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochLength),
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
        impl alloy_sol_types::SolType for EpochScheduleEntry {
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
        impl alloy_sol_types::SolStruct for EpochScheduleEntry {
            const NAME: &'static str = "EpochScheduleEntry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "EpochScheduleEntry(uint256 fromEpoch,uint256 epochLength)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fromEpoch)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.epochLength)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for EpochScheduleEntry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.fromEpoch,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.epochLength,
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
                    &rust.fromEpoch,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.epochLength,
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
    /**Creates a new wrapper around an on-chain [`ICiphertextRoots`](self) contract instance.

See the [wrapper's documentation](`ICiphertextRootsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> ICiphertextRootsInstance<P, N> {
        ICiphertextRootsInstance::<P, N>::new(address, __provider)
    }
    /**A [`ICiphertextRoots`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ICiphertextRoots`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ICiphertextRootsInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ICiphertextRootsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ICiphertextRootsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > ICiphertextRootsInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`ICiphertextRoots`](self) contract instance.

See the [wrapper's documentation](`ICiphertextRootsInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> ICiphertextRootsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ICiphertextRootsInstance<P, N> {
            ICiphertextRootsInstance {
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
    > ICiphertextRootsInstance<P, N> {
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
    > ICiphertextRootsInstance<P, N> {
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
library ICiphertextRoots {
    struct EpochSchedule {
        uint256 startBlock;
        EpochScheduleEntry[] entries;
    }
    struct EpochScheduleEntry {
        uint256 fromEpoch;
        uint256 epochLength;
    }
}

interface CiphertextRoots {
    error AddressEmptyCode(address target);
    error BlockBeforeStartBlock(uint64 chainId, uint256 blockNumber);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EpochLengthNotFound(uint64 chainId, uint256 fromEpoch);
    error EpochRootAlreadyRecorded(uint64 chainId, uint256 epoch);
    error EpochScheduleAlreadySet(uint64 chainId);
    error FailedCall();
    error FromEpochTooLow(uint64 chainId, uint256 fromEpoch, uint256 minFromEpoch);
    error InvalidChainId();
    error InvalidEpochLength();
    error InvalidInitialization();
    error NoEpochRoot(uint64 chainId, uint256 epoch);
    error NoEpochRootRecorded(uint64 chainId);
    error NoEpochSchedule(uint64 chainId);
    error NoRootReachesThreshold(uint64 chainId, uint256 epoch);
    error NotCoprocessorTxSender(address sender);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error RootAlreadyPosted(uint64 chainId, uint256 epoch, address sender);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event EpochLengthRemoved(uint64 indexed chainId, uint256 fromEpoch);
    event EpochLengthSet(uint64 indexed chainId, uint256 fromEpoch, uint256 epochLength);
    event EpochRootRecorded(uint64 indexed chainId, uint256 indexed epoch, bytes32 root, uint256 coprocessorContextId);
    event EpochScheduleSet(uint64 indexed chainId, uint256 startBlock, uint256 epochLength);
    event Initialized(uint64 version);
    event RootPosted(uint64 indexed chainId, uint256 indexed epoch, address indexed sender, bytes32 root);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function getEpochBlocks(uint64 chainId, uint256 epoch) external view returns (uint256 startBlock, uint256 endBlock);
    function getEpochOfBlock(uint64 chainId, uint256 blockNumber) external view returns (uint256);
    function getEpochRoot(uint64 chainId, uint256 epoch) external view returns (bytes32);
    function getEpochSchedule(uint64 chainId) external view returns (ICiphertextRoots.EpochSchedule memory);
    function getHighestRecordedEpoch(uint64 chainId) external view returns (uint256);
    function getPostedRoots(uint64 chainId, uint256 epoch) external view returns (bytes32[] memory);
    function getRootSenders(uint64 chainId, uint256 epoch, bytes32 root) external view returns (address[] memory);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function postRoot(uint64 chainId, uint256 epoch, bytes32 root) external;
    function proxiableUUID() external view returns (bytes32);
    function recordEpochRoot(uint64 chainId, uint256 epoch) external;
    function removeEpochLength(uint64 chainId, uint256 fromEpoch) external;
    function setEpochLength(uint64 chainId, uint256 fromEpoch, uint256 epochLength) external;
    function setEpochSchedule(uint64 chainId, uint256 startBlock, uint256 epochLength) external;
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
    "name": "getEpochBlocks",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "startBlock",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "endBlock",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getEpochOfBlock",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blockNumber",
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
    "name": "getEpochRoot",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "getEpochSchedule",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct ICiphertextRoots.EpochSchedule",
        "components": [
          {
            "name": "startBlock",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "entries",
            "type": "tuple[]",
            "internalType": "struct ICiphertextRoots.EpochScheduleEntry[]",
            "components": [
              {
                "name": "fromEpoch",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "epochLength",
                "type": "uint256",
                "internalType": "uint256"
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
    "name": "getHighestRecordedEpoch",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "getPostedRoots",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRootSenders",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "postRoot",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "recordEpochRoot",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeEpochLength",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "fromEpoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEpochLength",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "fromEpoch",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "epochLength",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setEpochSchedule",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "startBlock",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "epochLength",
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
    "name": "EpochLengthRemoved",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "fromEpoch",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EpochLengthSet",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "fromEpoch",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "epochLength",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EpochRootRecorded",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "root",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "coprocessorContextId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EpochScheduleSet",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "startBlock",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "epochLength",
        "type": "uint256",
        "indexed": false,
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
    "name": "RootPosted",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "root",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
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
    "name": "BlockBeforeStartBlock",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "blockNumber",
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
    "name": "EpochLengthNotFound",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "fromEpoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "EpochRootAlreadyRecorded",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "EpochScheduleAlreadySet",
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
    "name": "FailedCall",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FromEpochTooLow",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "fromEpoch",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minFromEpoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "InvalidChainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidEpochLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoEpochRoot",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "NoEpochRootRecorded",
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
    "name": "NoEpochSchedule",
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
    "name": "NoRootReachesThreshold",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotCoprocessorTxSender",
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
    "name": "RootAlreadyPosted",
    "inputs": [
      {
        "name": "chainId",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "epoch",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "sender",
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
pub mod CiphertextRoots {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a060405230608052348015610013575f80fd5b5061001c610021565b6100d3565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff16156100715760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b03908116146100d05780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b6080516123ad620000fa5f395f818161172c01528181611755015261193d01526123ad5ff3fe6080604052600436106100fa575f3560e01c8063459500e2116100925780639194060811610062578063919406081461029a578063ad3cb1cc146102c6578063de93c514146102f6578063e8ad358714610315578063f5178ccf14610334575f80fd5b8063459500e21461021b5780634f1ef2861461024757806352d1902d1461025a5780635757a3231461026e575f80fd5b80632949ca47116100cd5780632949ca47146101875780633662bd62146101bb57806337a08d12146101e857806339f7381014610207575f80fd5b80630d8e6e2c146100fe578063114bf5711461012857806313b4d5701461014957806319ba5fcc14610168575b5f80fd5b348015610109575f80fd5b50610112610353565b60405161011f9190611f6f565b60405180910390f35b348015610133575f80fd5b50610147610142366004611fb7565b6103c0565b005b348015610154575f80fd5b50610147610163366004611fdf565b610462565b348015610173575f80fd5b50610147610182366004611fdf565b6105eb565b348015610192575f80fd5b506101a66101a1366004611fb7565b6107f5565b6040805192835260208301919091520161011f565b3480156101c6575f80fd5b506101da6101d536600461200f565b6108fb565b60405190815260200161011f565b3480156101f3575f80fd5b50610147610202366004611fb7565b610948565b348015610212575f80fd5b50610147610bc1565b348015610226575f80fd5b5061023a61023536600461200f565b610ce2565b60405161011f9190612028565b6101476102553660046120b3565b610d8a565b348015610265575f80fd5b506101da610da9565b348015610279575f80fd5b5061028d610288366004611fdf565b610dc5565b60405161011f9190612170565b3480156102a5575f80fd5b506102b96102b4366004611fb7565b610e49565b60405161011f91906121bc565b3480156102d1575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b348015610301575f80fd5b50610147610310366004611fdf565b610eb7565b348015610320575f80fd5b506101da61032f366004611fb7565b6110b2565b34801561033f575f80fd5b506101da61034e366004611fb7565b611111565b60606040518060400160405280600f81526020016e43697068657274657874526f6f747360881b8152506103865f61122d565b610390600161122d565b6103995f61122d565b6040516020016103ac94939291906121f3565b604051602081830303815290604052905090565b5f6103ca836112bc565b5f83815260048083016020526040909120908101549192509060ff161561041b57604051631fae076360e31b81526001600160401b0385166004820152602481018490526044015b60405180910390fd5b61042d8484610428611327565b611559565b61045c5760405163962f43bf60e01b81526001600160401b038516600482015260248101849052604401610412565b50505050565b5f61046c846112bc565b5f848152600482016020908152604080832033845260028101909252909120549192509060ff16156104c95760405163030cf27360e51b81526001600160401b038616600482015260248101859052336044820152606401610412565b5f6104d2611327565b90506104de81336115fb565b6104fd576040516352d725f560e01b8152336004820152602401610412565b60048201545f9060ff16806105185750610518878784611559565b335f9081526002850160209081526040808320805460ff191660019081179091558984528701909152812080549293509190036105645783546001810185555f85815260209020018690555b80546001810182555f828152602090200180546001600160a01b0319163390811790915560405188906001600160401b038b16907f34ed767fa5380a3ae0ab36060f70b8db750241206796264dc4c8d90a7b811a66906105c7908b815260200190565b60405180910390a4816105e1576105df888885611559565b505b5050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561063b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061065f9190612270565b6001600160a01b0316336001600160a01b0316146106925760405163021bfda160e41b8152336004820152602401610412565b805f036106b2576040516388ac089760e01b815260040160405180910390fd5b5f6106bc846112bc565b60018082018054929350915f9183916106d5919061229f565b815481106106e5576106e56122b2565b905f5260205f2090600202015f0154600161070091906122c6565b6002840154909150610100900460ff168015610720575080836003015410155b156107385760038301546107359060016122c6565b90505b808510156107725760405163c3216f1960e01b81526001600160401b03871660048201526024810186905260448101829052606401610412565b60408051808201825286815260208082018781528554600180820188555f88815284902094516002909202909401908155905192019190915581518781529081018690526001600160401b038816917fa52450ac0a285dfafbcb8f29a79432af527935b468a51ad7ec5ad00cdcc4b0c491015b60405180910390a2505050505050565b5f805f610801856112bc565b5f0190505f8160010190505f815f8154811061081f5761081f6122b2565b5f91825260209091208454600290920201915060015b83548110156108aa575f848281548110610851576108516122b2565b905f5260205f2090600202019050805f015489101561087057506108aa565b835481545f9161087f9161229f565b905084600101548161089191906122d9565b61089b90856122c6565b91945090925050600101610835565b50600182015482546108bc908961229f565b6108c691906122d9565b6108d090826122c6565b955060018260010154876108e491906122c6565b6108ee919061229f565b9450505050509250929050565b5f80610906836112bc565b6002810154909150610100900460ff1661093e576040516365d5ea3f60e11b81526001600160401b0384166004820152602401610412565b6003015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610998573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109bc9190612270565b6001600160a01b0316336001600160a01b0316146109ef5760405163021bfda160e41b8152336004820152602401610412565b5f6109f9836112bc565b6001808201805492935091905b8181108015610a35575084838281548110610a2357610a236122b2565b905f5260205f2090600202015f015414155b15610a4c5780610a44816122f0565b915050610a06565b818103610a7e5760405163bc59e79760e01b81526001600160401b038716600482015260248101869052604401610412565b6002840154610100900460ff168015610a9b575083600301548511155b15610ae557858585600301546001610ab391906122c6565b60405163c3216f1960e01b81526001600160401b03909316600484015260248301919091526044820152606401610412565b805b610af260018461229f565b811015610b575783610b058260016122c6565b81548110610b1557610b156122b2565b905f5260205f209060020201848281548110610b3357610b336122b2565b5f918252602090912082546002909202019081556001918201549082015501610ae7565b5082805480610b6857610b68612308565b5f8281526020812060025f19909301928302018181556001015590556040516001600160401b038716907f7b9a73a467acdb27eee35e880ec775db86925cd82b94cb4cb6ddf70eb5cca5db906107e59088815260200190565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b0316600114610c1557604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460029190600160401b900460ff1680610c5e575080546001600160401b03808416911610155b15610c7c5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1768ff0000000000000000191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b604080518082019091525f815260606020820152610cff826112bc565b5f016040518060400160405290815f820154815260200160018201805480602002602001604051908101604052809291908181526020015f905b82821015610d7c578382905f5260205f2090600202016040518060400160405290815f820154815260200160018201548152505081526020019060010190610d39565b505050915250909392505050565b610d92611721565b610d9b826117c7565b610da58282611871565b5050565b5f610db2611932565b505f8051602061238d8339815191525b90565b6060610dd0846112bc565b5f8481526004919091016020908152604080832085845260010182529182902080548351818402810184019094528084529091830182828015610e3a57602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610e1c575b505050505090505b9392505050565b6060610e54836112bc565b5f838152600491909101602090815260409182902080548351818402810184019094528084529091830182828015610ea957602002820191905f5260205f20905b815481526020019060010190808311610e95575b505050505090505b92915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f07573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f2b9190612270565b6001600160a01b0316336001600160a01b031614610f5e5760405163021bfda160e41b8152336004820152602401610412565b826001600160401b03165f03610f8757604051633d23e4d160e11b815260040160405180910390fd5b805f03610fa7576040516388ac089760e01b815260040160405180910390fd5b6001600160401b0383165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff161561100e57604051630f17637d60e21b81526001600160401b0385166004820152602401610412565b6002808201805460ff191660019081179091558483556040805180820182525f80825260208083018881528786018054808801825590845291909220925195029091019384555192909101919091555181906001600160401b038616907f3b3ac53f416d9ad04e258f647d21d53088f789d2492abf9108e409f1018f3bea906110a39087908790918252602082015260400190565b60405180910390a25050505050565b5f806110bd846112bc565b5f8481526004918201602052604090209081015490915060ff166111065760405163158062cb60e11b81526001600160401b038516600482015260248101849052604401610412565b600301549392505050565b5f8061111c846112bc565b805490915083101561115357604051631dd79bc560e21b81526001600160401b038516600482015260248101849052604401610412565b5f8160010190505f815f8154811061116d5761116d6122b2565b5f91825260209091208454600290920201915060015b83548110156111fc575f84828154811061119f5761119f6122b2565b5f91825260208220865460029092020180549093506111be919061229f565b90505f8560010154826111d191906122d9565b6111db90866122c6565b9050808a10156111ed575050506111fc565b91945090925050600101611183565b50600182015461120c828861229f565b611216919061231c565b825461122291906122c6565b979650505050505050565b60605f6112398361197b565b60010190505f816001600160401b038111156112575761125761209f565b6040519080825280601f01601f191660200182016040528015611281576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461128b57509392505050565b6001600160401b0381165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff16611322576040516329cbdd2360e01b81526001600160401b0383166004820152602401610412565b919050565b6113566040518060a001604052805f81526020015f81526020015f151581526020015f81526020015f81525090565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b031663170a29816040518163ffffffff1660e01b8152600401602060405180830381865afa1580156113a6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113ca919061233b565b808252604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa15801561141c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611440919061233b565b606082015280516114539060019061229f565b60208201819052604051630e1887c960e01b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac90630e1887c990602401602060405180830381865afa1580156114a9573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114cd9190612352565b1580156040830152610dc2576020810151604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa15801561152d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611551919061233b565b608082015290565b5f80611564856112bc565b5f85815260049190910160205260408120915081905b81548110156115ef575f828281548110611596576115966122b2565b905f5260205f20015490505f806115bf88876001015f8681526020019081526020015f20611a52565b9150915081156115e4576115d58a8a8584611c83565b60019650505050505050610e42565b50505060010161157a565b505f9695505050505050565b8151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201525f907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa15801561165d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116819190612352565b1561168e57506001610eb1565b82604001518015610e4257506020830151604051633ac3ddd960e21b815260048101919091526001600160a01b03831660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa1580156116fd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e429190612352565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806117a757507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661179b5f8051602061238d833981519152546001600160a01b031690565b6001600160a01b031614155b156117c55760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611817573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061183b9190612270565b6001600160a01b0316336001600160a01b03161461186e5760405163021bfda160e41b8152336004820152602401610412565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156118cb575060408051601f3d908101601f191682019092526118c89181019061233b565b60015b6118f357604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610412565b5f8051602061238d833981519152811461192357604051632a87526960e21b815260048101829052602401610412565b61192d8383611d7f565b505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146117c55760405163703e46dd60e11b815260040160405180910390fd5b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106119b95772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106119e5576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310611a0357662386f26fc10000830492506010015b6305f5e1008310611a1b576305f5e100830492506008015b6127108310611a2f57612710830492506004015b60648310611a41576064830492506002015b600a8310610eb15760010192915050565b8054606083015160408401515f9283929091908210159083908015611a7b575086608001518310155b905081158015611a89575080155b15611a9c575f8094509450505050611c7c565b5f805f5b85811015611c1b575f898281548110611abb57611abb6122b2565b5f918252602090912001546001600160a01b03169050858015611b5d57508a51604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611b39573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b5d9190612352565b15611b705783611b6c816122f0565b9450505b848015611bff575060208b0151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611bdb573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bff9190612352565b15611c125782611c0e816122f0565b9350505b50600101611aa0565b50838015611c2d575088606001518210155b15611c455750508651600195509350611c7c92505050565b828015611c56575088608001518110155b15611c705760018960200151965096505050505050611c7c565b5f809650965050505050505b9250929050565b5f611c8d856112bc565b5f85815260048083016020526040909120908101549192509060ff1615611cd957604051631fae076360e31b81526001600160401b038716600482015260248101869052604401610412565b6003810184905560048101805460ff191660011790556002820154610100900460ff16611d1d5760028201805461ff00191661010017905560038201859055611d31565b8160030154851115611d3157600382018590555b604080518581526020810185905286916001600160401b038916917f9689d936609eeed0401b4feccecfea81216de8ff02bc8e4e44d0cd9f672c0038910160405180910390a3505050505050565b611d8882611dd4565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611dcc5761192d8282611e37565b610da5611ea9565b806001600160a01b03163b5f03611e0957604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610412565b5f8051602061238d83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051611e539190612371565b5f60405180830381855af49150503d805f8114611e8b576040519150601f19603f3d011682016040523d82523d5f602084013e611e90565b606091505b5091509150611ea0858383611ec8565b95945050505050565b34156117c55760405163b398979f60e01b815260040160405180910390fd5b606082611edd57611ed882611f24565b610e42565b8151158015611ef457506001600160a01b0384163b155b15611f1d57604051639996b31560e01b81526001600160a01b0385166004820152602401610412565b5080610e42565b805115611f345780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b83811015611f67578181015183820152602001611f4f565b50505f910152565b602081525f8251806020840152611f8d816040850160208701611f4d565b601f01601f19169190910160400192915050565b80356001600160401b0381168114611322575f80fd5b5f8060408385031215611fc8575f80fd5b611fd183611fa1565b946020939093013593505050565b5f805f60608486031215611ff1575f80fd5b611ffa84611fa1565b95602085013595506040909401359392505050565b5f6020828403121561201f575f80fd5b610e4282611fa1565b60208082528251828201528281015160408084018190528151606085018190525f9392830191849160808701905b8084101561207f5784518051835286015186830152938501936001939093019290820190612056565b50979650505050505050565b6001600160a01b038116811461186e575f80fd5b634e487b7160e01b5f52604160045260245ffd5b5f80604083850312156120c4575f80fd5b82356120cf8161208b565b915060208301356001600160401b03808211156120ea575f80fd5b818501915085601f8301126120fd575f80fd5b81358181111561210f5761210f61209f565b604051601f8201601f19908116603f011681019083821181831017156121375761213761209f565b8160405282815288602084870101111561214f575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b818110156121b05783516001600160a01b03168352928401929184019160010161218b565b50909695505050505050565b602080825282518282018190525f9190848201906040850190845b818110156121b0578351835292840192918401916001016121d7565b5f8551612204818460208a01611f4d565b61103b60f11b9083019081528551612223816002840160208a01611f4d565b808201915050601760f91b8060028301528551612247816003850160208a01611f4d565b60039201918201528351612262816004840160208801611f4d565b016004019695505050505050565b5f60208284031215612280575f80fd5b8151610e428161208b565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610eb157610eb161228b565b634e487b7160e01b5f52603260045260245ffd5b80820180821115610eb157610eb161228b565b8082028115828204841417610eb157610eb161228b565b5f600182016123015761230161228b565b5060010190565b634e487b7160e01b5f52603160045260245ffd5b5f8261233657634e487b7160e01b5f52601260045260245ffd5b500490565b5f6020828403121561234b575f80fd5b5051919050565b5f60208284031215612362575f80fd5b81518015158114610e42575f80fd5b5f8251612382818460208701611f4d565b919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15a\0\x13W_\x80\xFD[Pa\0\x1Ca\0!V[a\0\xD3V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15a\0qW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a\0\xD0W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa#\xADb\0\0\xFA_9_\x81\x81a\x17,\x01R\x81\x81a\x17U\x01Ra\x19=\x01Ra#\xAD_\xF3\xFE`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80cE\x95\0\xE2\x11a\0\x92W\x80c\x91\x94\x06\x08\x11a\0bW\x80c\x91\x94\x06\x08\x14a\x02\x9AW\x80c\xAD<\xB1\xCC\x14a\x02\xC6W\x80c\xDE\x93\xC5\x14\x14a\x02\xF6W\x80c\xE8\xAD5\x87\x14a\x03\x15W\x80c\xF5\x17\x8C\xCF\x14a\x034W_\x80\xFD[\x80cE\x95\0\xE2\x14a\x02\x1BW\x80cO\x1E\xF2\x86\x14a\x02GW\x80cR\xD1\x90-\x14a\x02ZW\x80cWW\xA3#\x14a\x02nW_\x80\xFD[\x80c)I\xCAG\x11a\0\xCDW\x80c)I\xCAG\x14a\x01\x87W\x80c6b\xBDb\x14a\x01\xBBW\x80c7\xA0\x8D\x12\x14a\x01\xE8W\x80c9\xF78\x10\x14a\x02\x07W_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c\x11K\xF5q\x14a\x01(W\x80c\x13\xB4\xD5p\x14a\x01IW\x80c\x19\xBA_\xCC\x14a\x01hW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03SV[`@Qa\x01\x1F\x91\x90a\x1FoV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01Ga\x01B6`\x04a\x1F\xB7V[a\x03\xC0V[\0[4\x80\x15a\x01TW_\x80\xFD[Pa\x01Ga\x01c6`\x04a\x1F\xDFV[a\x04bV[4\x80\x15a\x01sW_\x80\xFD[Pa\x01Ga\x01\x826`\x04a\x1F\xDFV[a\x05\xEBV[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01\xA6a\x01\xA16`\x04a\x1F\xB7V[a\x07\xF5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1FV[4\x80\x15a\x01\xC6W_\x80\xFD[Pa\x01\xDAa\x01\xD56`\x04a \x0FV[a\x08\xFBV[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xF3W_\x80\xFD[Pa\x01Ga\x02\x026`\x04a\x1F\xB7V[a\tHV[4\x80\x15a\x02\x12W_\x80\xFD[Pa\x01Ga\x0B\xC1V[4\x80\x15a\x02&W_\x80\xFD[Pa\x02:a\x0256`\x04a \x0FV[a\x0C\xE2V[`@Qa\x01\x1F\x91\x90a (V[a\x01Ga\x02U6`\x04a \xB3V[a\r\x8AV[4\x80\x15a\x02eW_\x80\xFD[Pa\x01\xDAa\r\xA9V[4\x80\x15a\x02yW_\x80\xFD[Pa\x02\x8Da\x02\x886`\x04a\x1F\xDFV[a\r\xC5V[`@Qa\x01\x1F\x91\x90a!pV[4\x80\x15a\x02\xA5W_\x80\xFD[Pa\x02\xB9a\x02\xB46`\x04a\x1F\xB7V[a\x0EIV[`@Qa\x01\x1F\x91\x90a!\xBCV[4\x80\x15a\x02\xD1W_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x01Ga\x03\x106`\x04a\x1F\xDFV[a\x0E\xB7V[4\x80\x15a\x03 W_\x80\xFD[Pa\x01\xDAa\x03/6`\x04a\x1F\xB7V[a\x10\xB2V[4\x80\x15a\x03?W_\x80\xFD[Pa\x01\xDAa\x03N6`\x04a\x1F\xB7V[a\x11\x11V[```@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nCiphertextRoots`\x88\x1B\x81RPa\x03\x86_a\x12-V[a\x03\x90`\x01a\x12-V[a\x03\x99_a\x12-V[`@Q` \x01a\x03\xAC\x94\x93\x92\x91\x90a!\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_a\x03\xCA\x83a\x12\xBCV[_\x83\x81R`\x04\x80\x83\x01` R`@\x90\x91 \x90\x81\x01T\x91\x92P\x90`\xFF\x16\x15a\x04\x1BW`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x04-\x84\x84a\x04(a\x13'V[a\x15YV[a\x04\\W`@Qc\x96/C\xBF`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x12V[PPPPV[_a\x04l\x84a\x12\xBCV[_\x84\x81R`\x04\x82\x01` \x90\x81R`@\x80\x83 3\x84R`\x02\x81\x01\x90\x92R\x90\x91 T\x91\x92P\x90`\xFF\x16\x15a\x04\xC9W`@Qc\x03\x0C\xF2s`\xE5\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`$\x81\x01\x85\x90R3`D\x82\x01R`d\x01a\x04\x12V[_a\x04\xD2a\x13'V[\x90Pa\x04\xDE\x813a\x15\xFBV[a\x04\xFDW`@QcR\xD7%\xF5`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[`\x04\x82\x01T_\x90`\xFF\x16\x80a\x05\x18WPa\x05\x18\x87\x87\x84a\x15YV[3_\x90\x81R`\x02\x85\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x89\x84R\x87\x01\x90\x91R\x81 \x80T\x92\x93P\x91\x90\x03a\x05dW\x83T`\x01\x81\x01\x85U_\x85\x81R` \x90 \x01\x86\x90U[\x80T`\x01\x81\x01\x82U_\x82\x81R` \x90 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U`@Q\x88\x90`\x01`\x01`@\x1B\x03\x8B\x16\x90\x7F4\xEDv\x7F\xA58\n:\xE0\xAB6\x06\x0Fp\xB8\xDBu\x02A g\x96&M\xC4\xC8\xD9\n{\x81\x1Af\x90a\x05\xC7\x90\x8B\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x81a\x05\xE1Wa\x05\xDF\x88\x88\x85a\x15YV[P[PPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06;W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06_\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x92W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[\x80_\x03a\x06\xB2W`@Qc\x88\xAC\x08\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06\xBC\x84a\x12\xBCV[`\x01\x80\x82\x01\x80T\x92\x93P\x91_\x91\x83\x91a\x06\xD5\x91\x90a\"\x9FV[\x81T\x81\x10a\x06\xE5Wa\x06\xE5a\"\xB2V[\x90_R` _ \x90`\x02\x02\x01_\x01T`\x01a\x07\0\x91\x90a\"\xC6V[`\x02\x84\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16\x80\x15a\x07 WP\x80\x83`\x03\x01T\x10\x15[\x15a\x078W`\x03\x83\x01Ta\x075\x90`\x01a\"\xC6V[\x90P[\x80\x85\x10\x15a\x07rW`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x82\x90R`d\x01a\x04\x12V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R\x85T`\x01\x80\x82\x01\x88U_\x88\x81R\x84\x90 \x94Q`\x02\x90\x92\x02\x90\x94\x01\x90\x81U\x90Q\x92\x01\x91\x90\x91U\x81Q\x87\x81R\x90\x81\x01\x86\x90R`\x01`\x01`@\x1B\x03\x88\x16\x91\x7F\xA5$P\xAC\n(]\xFA\xFB\xCB\x8F)\xA7\x942\xAFRy5\xB4h\xA5\x1A\xD7\xECZ\xD0\x0C\xDC\xC4\xB0\xC4\x91\x01[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80_a\x08\x01\x85a\x12\xBCV[_\x01\x90P_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x08\x1FWa\x08\x1Fa\"\xB2V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x08\xAAW_\x84\x82\x81T\x81\x10a\x08QWa\x08Qa\"\xB2V[\x90_R` _ \x90`\x02\x02\x01\x90P\x80_\x01T\x89\x10\x15a\x08pWPa\x08\xAAV[\x83T\x81T_\x91a\x08\x7F\x91a\"\x9FV[\x90P\x84`\x01\x01T\x81a\x08\x91\x91\x90a\"\xD9V[a\x08\x9B\x90\x85a\"\xC6V[\x91\x94P\x90\x92PP`\x01\x01a\x085V[P`\x01\x82\x01T\x82Ta\x08\xBC\x90\x89a\"\x9FV[a\x08\xC6\x91\x90a\"\xD9V[a\x08\xD0\x90\x82a\"\xC6V[\x95P`\x01\x82`\x01\x01T\x87a\x08\xE4\x91\x90a\"\xC6V[a\x08\xEE\x91\x90a\"\x9FV[\x94PPPPP\x92P\x92\x90PV[_\x80a\t\x06\x83a\x12\xBCV[`\x02\x81\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16a\t>W`@Qce\xD5\xEA?`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x04\x12V[`\x03\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x98W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xBC\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xEFW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[_a\t\xF9\x83a\x12\xBCV[`\x01\x80\x82\x01\x80T\x92\x93P\x91\x90[\x81\x81\x10\x80\x15a\n5WP\x84\x83\x82\x81T\x81\x10a\n#Wa\n#a\"\xB2V[\x90_R` _ \x90`\x02\x02\x01_\x01T\x14\x15[\x15a\nLW\x80a\nD\x81a\"\xF0V[\x91PPa\n\x06V[\x81\x81\x03a\n~W`@Qc\xBCY\xE7\x97`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x12V[`\x02\x84\x01Ta\x01\0\x90\x04`\xFF\x16\x80\x15a\n\x9BWP\x83`\x03\x01T\x85\x11\x15[\x15a\n\xE5W\x85\x85\x85`\x03\x01T`\x01a\n\xB3\x91\x90a\"\xC6V[`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`D\x82\x01R`d\x01a\x04\x12V[\x80[a\n\xF2`\x01\x84a\"\x9FV[\x81\x10\x15a\x0BWW\x83a\x0B\x05\x82`\x01a\"\xC6V[\x81T\x81\x10a\x0B\x15Wa\x0B\x15a\"\xB2V[\x90_R` _ \x90`\x02\x02\x01\x84\x82\x81T\x81\x10a\x0B3Wa\x0B3a\"\xB2V[_\x91\x82R` \x90\x91 \x82T`\x02\x90\x92\x02\x01\x90\x81U`\x01\x91\x82\x01T\x90\x82\x01U\x01a\n\xE7V[P\x82\x80T\x80a\x0BhWa\x0Bha#\x08V[_\x82\x81R` \x81 `\x02_\x19\x90\x93\x01\x92\x83\x02\x01\x81\x81U`\x01\x01U\x90U`@Q`\x01`\x01`@\x1B\x03\x87\x16\x90\x7F{\x9As\xA4g\xAC\xDB'\xEE\xE3^\x88\x0E\xC7u\xDB\x86\x92\\\xD8+\x94\xCBL\xB6\xDD\xF7\x0E\xB5\xCC\xA5\xDB\x90a\x07\xE5\x90\x88\x81R` \x01\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x0C\x15W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x02\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x0C^WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x0C|W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17h\xFF\0\0\0\0\0\0\0\0\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01Ra\x0C\xFF\x82a\x12\xBCV[_\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\r|W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r9V[PPP\x91RP\x90\x93\x92PPPV[a\r\x92a\x17!V[a\r\x9B\x82a\x17\xC7V[a\r\xA5\x82\x82a\x18qV[PPV[_a\r\xB2a\x192V[P_\x80Q` a#\x8D\x839\x81Q\x91R[\x90V[``a\r\xD0\x84a\x12\xBCV[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 \x85\x84R`\x01\x01\x82R\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R\x90\x91\x83\x01\x82\x82\x80\x15a\x0E:W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0E\x1CW[PPPPP\x90P[\x93\x92PPPV[``a\x0ET\x83a\x12\xBCV[_\x83\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R\x90\x91\x83\x01\x82\x82\x80\x15a\x0E\xA9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\x95W[PPPPP\x90P[\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x07W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F+\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F^W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[\x82`\x01`\x01`@\x1B\x03\x16_\x03a\x0F\x87W`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x03a\x0F\xA7W`@Qc\x88\xAC\x08\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16\x15a\x10\x0EW`@Qc\x0F\x17c}`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x12V[`\x02\x80\x82\x01\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x84\x83U`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x88\x81R\x87\x86\x01\x80T\x80\x88\x01\x82U\x90\x84R\x91\x90\x92 \x92Q\x95\x02\x90\x91\x01\x93\x84UQ\x92\x90\x91\x01\x91\x90\x91UQ\x81\x90`\x01`\x01`@\x1B\x03\x86\x16\x90\x7F;:\xC5?Am\x9A\xD0N%\x8Fd}!\xD50\x88\xF7\x89\xD2I*\xBF\x91\x08\xE4\t\xF1\x01\x8F;\xEA\x90a\x10\xA3\x90\x87\x90\x87\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[_\x80a\x10\xBD\x84a\x12\xBCV[_\x84\x81R`\x04\x91\x82\x01` R`@\x90 \x90\x81\x01T\x90\x91P`\xFF\x16a\x11\x06W`@Qc\x15\x80b\xCB`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x12V[`\x03\x01T\x93\x92PPPV[_\x80a\x11\x1C\x84a\x12\xBCV[\x80T\x90\x91P\x83\x10\x15a\x11SW`@Qc\x1D\xD7\x9B\xC5`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x12V[_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x11mWa\x11ma\"\xB2V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x11\xFCW_\x84\x82\x81T\x81\x10a\x11\x9FWa\x11\x9Fa\"\xB2V[_\x91\x82R` \x82 \x86T`\x02\x90\x92\x02\x01\x80T\x90\x93Pa\x11\xBE\x91\x90a\"\x9FV[\x90P_\x85`\x01\x01T\x82a\x11\xD1\x91\x90a\"\xD9V[a\x11\xDB\x90\x86a\"\xC6V[\x90P\x80\x8A\x10\x15a\x11\xEDWPPPa\x11\xFCV[\x91\x94P\x90\x92PP`\x01\x01a\x11\x83V[P`\x01\x82\x01Ta\x12\x0C\x82\x88a\"\x9FV[a\x12\x16\x91\x90a#\x1CV[\x82Ta\x12\"\x91\x90a\"\xC6V[\x97\x96PPPPPPPV[``_a\x129\x83a\x19{V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12WWa\x12Wa \x9FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\x81W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12\x8BWP\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16a\x13\"W`@Qc)\xCB\xDD#`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x12V[\x91\x90PV[a\x13V`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81RP\x90V[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x17\n)\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xCA\x91\x90a#;V[\x80\x82R`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14@\x91\x90a#;V[``\x82\x01R\x80Qa\x14S\x90`\x01\x90a\"\x9FV[` \x82\x01\x81\x90R`@Qc\x0E\x18\x87\xC9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\x0E\x18\x87\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCD\x91\x90a#RV[\x15\x80\x15`@\x83\x01Ra\r\xC2W` \x81\x01Q`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15-W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15Q\x91\x90a#;V[`\x80\x82\x01R\x90V[_\x80a\x15d\x85a\x12\xBCV[_\x85\x81R`\x04\x91\x90\x91\x01` R`@\x81 \x91P\x81\x90[\x81T\x81\x10\x15a\x15\xEFW_\x82\x82\x81T\x81\x10a\x15\x96Wa\x15\x96a\"\xB2V[\x90_R` _ \x01T\x90P_\x80a\x15\xBF\x88\x87`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x1ARV[\x91P\x91P\x81\x15a\x15\xE4Wa\x15\xD5\x8A\x8A\x85\x84a\x1C\x83V[`\x01\x96PPPPPPPa\x0EBV[PPP`\x01\x01a\x15zV[P_\x96\x95PPPPPPV[\x81Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16]W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x81\x91\x90a#RV[\x15a\x16\x8EWP`\x01a\x0E\xB1V[\x82`@\x01Q\x80\x15a\x0EBWP` \x83\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xFDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EB\x91\x90a#RV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x17\xA7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x17\x9B_\x80Q` a#\x8D\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x17\xC5W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x17W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18;\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18nW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18\xCBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18\xC8\x91\x81\x01\x90a#;V[`\x01[a\x18\xF3W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x12V[_\x80Q` a#\x8D\x839\x81Q\x91R\x81\x14a\x19#W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\x12V[a\x19-\x83\x83a\x1D\x7FV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\xC5W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x19\xB9Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x19\xE5Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1A\x03Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1A\x1BWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1A/Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1AAW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0E\xB1W`\x01\x01\x92\x91PPV[\x80T``\x83\x01Q`@\x84\x01Q_\x92\x83\x92\x90\x91\x90\x82\x10\x15\x90\x83\x90\x80\x15a\x1A{WP\x86`\x80\x01Q\x83\x10\x15[\x90P\x81\x15\x80\x15a\x1A\x89WP\x80\x15[\x15a\x1A\x9CW_\x80\x94P\x94PPPPa\x1C|V[_\x80_[\x85\x81\x10\x15a\x1C\x1BW_\x89\x82\x81T\x81\x10a\x1A\xBBWa\x1A\xBBa\"\xB2V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x85\x80\x15a\x1B]WP\x8AQ`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B]\x91\x90a#RV[\x15a\x1BpW\x83a\x1Bl\x81a\"\xF0V[\x94PP[\x84\x80\x15a\x1B\xFFWP` \x8B\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xFF\x91\x90a#RV[\x15a\x1C\x12W\x82a\x1C\x0E\x81a\"\xF0V[\x93PP[P`\x01\x01a\x1A\xA0V[P\x83\x80\x15a\x1C-WP\x88``\x01Q\x82\x10\x15[\x15a\x1CEWPP\x86Q`\x01\x95P\x93Pa\x1C|\x92PPPV[\x82\x80\x15a\x1CVWP\x88`\x80\x01Q\x81\x10\x15[\x15a\x1CpW`\x01\x89` \x01Q\x96P\x96PPPPPPa\x1C|V[_\x80\x96P\x96PPPPPP[\x92P\x92\x90PV[_a\x1C\x8D\x85a\x12\xBCV[_\x85\x81R`\x04\x80\x83\x01` R`@\x90\x91 \x90\x81\x01T\x91\x92P\x90`\xFF\x16\x15a\x1C\xD9W`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x12V[`\x03\x81\x01\x84\x90U`\x04\x81\x01\x80T`\xFF\x19\x16`\x01\x17\x90U`\x02\x82\x01Ta\x01\0\x90\x04`\xFF\x16a\x1D\x1DW`\x02\x82\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U`\x03\x82\x01\x85\x90Ua\x1D1V[\x81`\x03\x01T\x85\x11\x15a\x1D1W`\x03\x82\x01\x85\x90U[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x86\x91`\x01`\x01`@\x1B\x03\x89\x16\x91\x7F\x96\x89\xD96`\x9E\xEE\xD0@\x1BO\xEC\xCE\xCF\xEA\x81!m\xE8\xFF\x02\xBC\x8END\xD0\xCD\x9Fg,\08\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[a\x1D\x88\x82a\x1D\xD4V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x1D\xCCWa\x19-\x82\x82a\x1E7V[a\r\xA5a\x1E\xA9V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1E\tW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04\x12V[_\x80Q` a#\x8D\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1ES\x91\x90a#qV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1E\x8BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1E\x90V[``\x91P[P\x91P\x91Pa\x1E\xA0\x85\x83\x83a\x1E\xC8V[\x95\x94PPPPPV[4\x15a\x17\xC5W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1E\xDDWa\x1E\xD8\x82a\x1F$V[a\x0EBV[\x81Q\x15\x80\x15a\x1E\xF4WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1F\x1DW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x12V[P\x80a\x0EBV[\x80Q\x15a\x1F4W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a\x1FgW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FOV[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra\x1F\x8D\x81`@\x85\x01` \x87\x01a\x1FMV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13\"W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\x1F\xC8W_\x80\xFD[a\x1F\xD1\x83a\x1F\xA1V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_``\x84\x86\x03\x12\x15a\x1F\xF1W_\x80\xFD[a\x1F\xFA\x84a\x1F\xA1V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a \x1FW_\x80\xFD[a\x0EB\x82a\x1F\xA1V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R_\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a \x7FW\x84Q\x80Q\x83R\x86\x01Q\x86\x83\x01R\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a VV[P\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18nW_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a \xC4W_\x80\xFD[\x825a \xCF\x81a \x8BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \xEAW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \xFDW_\x80\xFD[\x815\x81\x81\x11\x15a!\x0FWa!\x0Fa \x9FV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a!7Wa!7a \x9FV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a!OW_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a!\xB0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a!\x8BV[P\x90\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a!\xB0W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a!\xD7V[_\x85Qa\"\x04\x81\x84` \x8A\x01a\x1FMV[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa\"#\x81`\x02\x84\x01` \x8A\x01a\x1FMV[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa\"G\x81`\x03\x85\x01` \x8A\x01a\x1FMV[`\x03\x92\x01\x91\x82\x01R\x83Qa\"b\x81`\x04\x84\x01` \x88\x01a\x1FMV[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\"\x80W_\x80\xFD[\x81Qa\x0EB\x81a \x8BV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xB1Wa\x0E\xB1a\"\x8BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0E\xB1Wa\x0E\xB1a\"\x8BV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\xB1Wa\x0E\xB1a\"\x8BV[_`\x01\x82\x01a#\x01Wa#\x01a\"\x8BV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82a#6WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a#KW_\x80\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a#bW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0EBW_\x80\xFD[_\x82Qa#\x82\x81\x84` \x87\x01a\x1FMV[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100fa575f3560e01c8063459500e2116100925780639194060811610062578063919406081461029a578063ad3cb1cc146102c6578063de93c514146102f6578063e8ad358714610315578063f5178ccf14610334575f80fd5b8063459500e21461021b5780634f1ef2861461024757806352d1902d1461025a5780635757a3231461026e575f80fd5b80632949ca47116100cd5780632949ca47146101875780633662bd62146101bb57806337a08d12146101e857806339f7381014610207575f80fd5b80630d8e6e2c146100fe578063114bf5711461012857806313b4d5701461014957806319ba5fcc14610168575b5f80fd5b348015610109575f80fd5b50610112610353565b60405161011f9190611f6f565b60405180910390f35b348015610133575f80fd5b50610147610142366004611fb7565b6103c0565b005b348015610154575f80fd5b50610147610163366004611fdf565b610462565b348015610173575f80fd5b50610147610182366004611fdf565b6105eb565b348015610192575f80fd5b506101a66101a1366004611fb7565b6107f5565b6040805192835260208301919091520161011f565b3480156101c6575f80fd5b506101da6101d536600461200f565b6108fb565b60405190815260200161011f565b3480156101f3575f80fd5b50610147610202366004611fb7565b610948565b348015610212575f80fd5b50610147610bc1565b348015610226575f80fd5b5061023a61023536600461200f565b610ce2565b60405161011f9190612028565b6101476102553660046120b3565b610d8a565b348015610265575f80fd5b506101da610da9565b348015610279575f80fd5b5061028d610288366004611fdf565b610dc5565b60405161011f9190612170565b3480156102a5575f80fd5b506102b96102b4366004611fb7565b610e49565b60405161011f91906121bc565b3480156102d1575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b348015610301575f80fd5b50610147610310366004611fdf565b610eb7565b348015610320575f80fd5b506101da61032f366004611fb7565b6110b2565b34801561033f575f80fd5b506101da61034e366004611fb7565b611111565b60606040518060400160405280600f81526020016e43697068657274657874526f6f747360881b8152506103865f61122d565b610390600161122d565b6103995f61122d565b6040516020016103ac94939291906121f3565b604051602081830303815290604052905090565b5f6103ca836112bc565b5f83815260048083016020526040909120908101549192509060ff161561041b57604051631fae076360e31b81526001600160401b0385166004820152602481018490526044015b60405180910390fd5b61042d8484610428611327565b611559565b61045c5760405163962f43bf60e01b81526001600160401b038516600482015260248101849052604401610412565b50505050565b5f61046c846112bc565b5f848152600482016020908152604080832033845260028101909252909120549192509060ff16156104c95760405163030cf27360e51b81526001600160401b038616600482015260248101859052336044820152606401610412565b5f6104d2611327565b90506104de81336115fb565b6104fd576040516352d725f560e01b8152336004820152602401610412565b60048201545f9060ff16806105185750610518878784611559565b335f9081526002850160209081526040808320805460ff191660019081179091558984528701909152812080549293509190036105645783546001810185555f85815260209020018690555b80546001810182555f828152602090200180546001600160a01b0319163390811790915560405188906001600160401b038b16907f34ed767fa5380a3ae0ab36060f70b8db750241206796264dc4c8d90a7b811a66906105c7908b815260200190565b60405180910390a4816105e1576105df888885611559565b505b5050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561063b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061065f9190612270565b6001600160a01b0316336001600160a01b0316146106925760405163021bfda160e41b8152336004820152602401610412565b805f036106b2576040516388ac089760e01b815260040160405180910390fd5b5f6106bc846112bc565b60018082018054929350915f9183916106d5919061229f565b815481106106e5576106e56122b2565b905f5260205f2090600202015f0154600161070091906122c6565b6002840154909150610100900460ff168015610720575080836003015410155b156107385760038301546107359060016122c6565b90505b808510156107725760405163c3216f1960e01b81526001600160401b03871660048201526024810186905260448101829052606401610412565b60408051808201825286815260208082018781528554600180820188555f88815284902094516002909202909401908155905192019190915581518781529081018690526001600160401b038816917fa52450ac0a285dfafbcb8f29a79432af527935b468a51ad7ec5ad00cdcc4b0c491015b60405180910390a2505050505050565b5f805f610801856112bc565b5f0190505f8160010190505f815f8154811061081f5761081f6122b2565b5f91825260209091208454600290920201915060015b83548110156108aa575f848281548110610851576108516122b2565b905f5260205f2090600202019050805f015489101561087057506108aa565b835481545f9161087f9161229f565b905084600101548161089191906122d9565b61089b90856122c6565b91945090925050600101610835565b50600182015482546108bc908961229f565b6108c691906122d9565b6108d090826122c6565b955060018260010154876108e491906122c6565b6108ee919061229f565b9450505050509250929050565b5f80610906836112bc565b6002810154909150610100900460ff1661093e576040516365d5ea3f60e11b81526001600160401b0384166004820152602401610412565b6003015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610998573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109bc9190612270565b6001600160a01b0316336001600160a01b0316146109ef5760405163021bfda160e41b8152336004820152602401610412565b5f6109f9836112bc565b6001808201805492935091905b8181108015610a35575084838281548110610a2357610a236122b2565b905f5260205f2090600202015f015414155b15610a4c5780610a44816122f0565b915050610a06565b818103610a7e5760405163bc59e79760e01b81526001600160401b038716600482015260248101869052604401610412565b6002840154610100900460ff168015610a9b575083600301548511155b15610ae557858585600301546001610ab391906122c6565b60405163c3216f1960e01b81526001600160401b03909316600484015260248301919091526044820152606401610412565b805b610af260018461229f565b811015610b575783610b058260016122c6565b81548110610b1557610b156122b2565b905f5260205f209060020201848281548110610b3357610b336122b2565b5f918252602090912082546002909202019081556001918201549082015501610ae7565b5082805480610b6857610b68612308565b5f8281526020812060025f19909301928302018181556001015590556040516001600160401b038716907f7b9a73a467acdb27eee35e880ec775db86925cd82b94cb4cb6ddf70eb5cca5db906107e59088815260200190565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b0316600114610c1557604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460029190600160401b900460ff1680610c5e575080546001600160401b03808416911610155b15610c7c5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1768ff0000000000000000191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b604080518082019091525f815260606020820152610cff826112bc565b5f016040518060400160405290815f820154815260200160018201805480602002602001604051908101604052809291908181526020015f905b82821015610d7c578382905f5260205f2090600202016040518060400160405290815f820154815260200160018201548152505081526020019060010190610d39565b505050915250909392505050565b610d92611721565b610d9b826117c7565b610da58282611871565b5050565b5f610db2611932565b505f8051602061238d8339815191525b90565b6060610dd0846112bc565b5f8481526004919091016020908152604080832085845260010182529182902080548351818402810184019094528084529091830182828015610e3a57602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311610e1c575b505050505090505b9392505050565b6060610e54836112bc565b5f838152600491909101602090815260409182902080548351818402810184019094528084529091830182828015610ea957602002820191905f5260205f20905b815481526020019060010190808311610e95575b505050505090505b92915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610f07573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f2b9190612270565b6001600160a01b0316336001600160a01b031614610f5e5760405163021bfda160e41b8152336004820152602401610412565b826001600160401b03165f03610f8757604051633d23e4d160e11b815260040160405180910390fd5b805f03610fa7576040516388ac089760e01b815260040160405180910390fd5b6001600160401b0383165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff161561100e57604051630f17637d60e21b81526001600160401b0385166004820152602401610412565b6002808201805460ff191660019081179091558483556040805180820182525f80825260208083018881528786018054808801825590845291909220925195029091019384555192909101919091555181906001600160401b038616907f3b3ac53f416d9ad04e258f647d21d53088f789d2492abf9108e409f1018f3bea906110a39087908790918252602082015260400190565b60405180910390a25050505050565b5f806110bd846112bc565b5f8481526004918201602052604090209081015490915060ff166111065760405163158062cb60e11b81526001600160401b038516600482015260248101849052604401610412565b600301549392505050565b5f8061111c846112bc565b805490915083101561115357604051631dd79bc560e21b81526001600160401b038516600482015260248101849052604401610412565b5f8160010190505f815f8154811061116d5761116d6122b2565b5f91825260209091208454600290920201915060015b83548110156111fc575f84828154811061119f5761119f6122b2565b5f91825260208220865460029092020180549093506111be919061229f565b90505f8560010154826111d191906122d9565b6111db90866122c6565b9050808a10156111ed575050506111fc565b91945090925050600101611183565b50600182015461120c828861229f565b611216919061231c565b825461122291906122c6565b979650505050505050565b60605f6112398361197b565b60010190505f816001600160401b038111156112575761125761209f565b6040519080825280601f01601f191660200182016040528015611281576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a850494508461128b57509392505050565b6001600160401b0381165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff16611322576040516329cbdd2360e01b81526001600160401b0383166004820152602401610412565b919050565b6113566040518060a001604052805f81526020015f81526020015f151581526020015f81526020015f81525090565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b031663170a29816040518163ffffffff1660e01b8152600401602060405180830381865afa1580156113a6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113ca919061233b565b808252604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa15801561141c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611440919061233b565b606082015280516114539060019061229f565b60208201819052604051630e1887c960e01b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac90630e1887c990602401602060405180830381865afa1580156114a9573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114cd9190612352565b1580156040830152610dc2576020810151604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa15801561152d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611551919061233b565b608082015290565b5f80611564856112bc565b5f85815260049190910160205260408120915081905b81548110156115ef575f828281548110611596576115966122b2565b905f5260205f20015490505f806115bf88876001015f8681526020019081526020015f20611a52565b9150915081156115e4576115d58a8a8584611c83565b60019650505050505050610e42565b50505060010161157a565b505f9695505050505050565b8151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201525f907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa15801561165d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116819190612352565b1561168e57506001610eb1565b82604001518015610e4257506020830151604051633ac3ddd960e21b815260048101919091526001600160a01b03831660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa1580156116fd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e429190612352565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806117a757507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031661179b5f8051602061238d833981519152546001600160a01b031690565b6001600160a01b031614155b156117c55760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611817573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061183b9190612270565b6001600160a01b0316336001600160a01b03161461186e5760405163021bfda160e41b8152336004820152602401610412565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156118cb575060408051601f3d908101601f191682019092526118c89181019061233b565b60015b6118f357604051634c9c8ce360e01b81526001600160a01b0383166004820152602401610412565b5f8051602061238d833981519152811461192357604051632a87526960e21b815260048101829052602401610412565b61192d8383611d7f565b505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146117c55760405163703e46dd60e11b815260040160405180910390fd5b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b83106119b95772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef810000000083106119e5576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310611a0357662386f26fc10000830492506010015b6305f5e1008310611a1b576305f5e100830492506008015b6127108310611a2f57612710830492506004015b60648310611a41576064830492506002015b600a8310610eb15760010192915050565b8054606083015160408401515f9283929091908210159083908015611a7b575086608001518310155b905081158015611a89575080155b15611a9c575f8094509450505050611c7c565b5f805f5b85811015611c1b575f898281548110611abb57611abb6122b2565b5f918252602090912001546001600160a01b03169050858015611b5d57508a51604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611b39573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b5d9190612352565b15611b705783611b6c816122f0565b9450505b848015611bff575060208b0151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611bdb573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bff9190612352565b15611c125782611c0e816122f0565b9350505b50600101611aa0565b50838015611c2d575088606001518210155b15611c455750508651600195509350611c7c92505050565b828015611c56575088608001518110155b15611c705760018960200151965096505050505050611c7c565b5f809650965050505050505b9250929050565b5f611c8d856112bc565b5f85815260048083016020526040909120908101549192509060ff1615611cd957604051631fae076360e31b81526001600160401b038716600482015260248101869052604401610412565b6003810184905560048101805460ff191660011790556002820154610100900460ff16611d1d5760028201805461ff00191661010017905560038201859055611d31565b8160030154851115611d3157600382018590555b604080518581526020810185905286916001600160401b038916917f9689d936609eeed0401b4feccecfea81216de8ff02bc8e4e44d0cd9f672c0038910160405180910390a3505050505050565b611d8882611dd4565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a2805115611dcc5761192d8282611e37565b610da5611ea9565b806001600160a01b03163b5f03611e0957604051634c9c8ce360e01b81526001600160a01b0382166004820152602401610412565b5f8051602061238d83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b031684604051611e539190612371565b5f60405180830381855af49150503d805f8114611e8b576040519150601f19603f3d011682016040523d82523d5f602084013e611e90565b606091505b5091509150611ea0858383611ec8565b95945050505050565b34156117c55760405163b398979f60e01b815260040160405180910390fd5b606082611edd57611ed882611f24565b610e42565b8151158015611ef457506001600160a01b0384163b155b15611f1d57604051639996b31560e01b81526001600160a01b0385166004820152602401610412565b5080610e42565b805115611f345780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b83811015611f67578181015183820152602001611f4f565b50505f910152565b602081525f8251806020840152611f8d816040850160208701611f4d565b601f01601f19169190910160400192915050565b80356001600160401b0381168114611322575f80fd5b5f8060408385031215611fc8575f80fd5b611fd183611fa1565b946020939093013593505050565b5f805f60608486031215611ff1575f80fd5b611ffa84611fa1565b95602085013595506040909401359392505050565b5f6020828403121561201f575f80fd5b610e4282611fa1565b60208082528251828201528281015160408084018190528151606085018190525f9392830191849160808701905b8084101561207f5784518051835286015186830152938501936001939093019290820190612056565b50979650505050505050565b6001600160a01b038116811461186e575f80fd5b634e487b7160e01b5f52604160045260245ffd5b5f80604083850312156120c4575f80fd5b82356120cf8161208b565b915060208301356001600160401b03808211156120ea575f80fd5b818501915085601f8301126120fd575f80fd5b81358181111561210f5761210f61209f565b604051601f8201601f19908116603f011681019083821181831017156121375761213761209f565b8160405282815288602084870101111561214f575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b818110156121b05783516001600160a01b03168352928401929184019160010161218b565b50909695505050505050565b602080825282518282018190525f9190848201906040850190845b818110156121b0578351835292840192918401916001016121d7565b5f8551612204818460208a01611f4d565b61103b60f11b9083019081528551612223816002840160208a01611f4d565b808201915050601760f91b8060028301528551612247816003850160208a01611f4d565b60039201918201528351612262816004840160208801611f4d565b016004019695505050505050565b5f60208284031215612280575f80fd5b8151610e428161208b565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610eb157610eb161228b565b634e487b7160e01b5f52603260045260245ffd5b80820180821115610eb157610eb161228b565b8082028115828204841417610eb157610eb161228b565b5f600182016123015761230161228b565b5060010190565b634e487b7160e01b5f52603160045260245ffd5b5f8261233657634e487b7160e01b5f52601260045260245ffd5b500490565b5f6020828403121561234b575f80fd5b5051919050565b5f60208284031215612362575f80fd5b81518015158114610e42575f80fd5b5f8251612382818460208701611f4d565b919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80cE\x95\0\xE2\x11a\0\x92W\x80c\x91\x94\x06\x08\x11a\0bW\x80c\x91\x94\x06\x08\x14a\x02\x9AW\x80c\xAD<\xB1\xCC\x14a\x02\xC6W\x80c\xDE\x93\xC5\x14\x14a\x02\xF6W\x80c\xE8\xAD5\x87\x14a\x03\x15W\x80c\xF5\x17\x8C\xCF\x14a\x034W_\x80\xFD[\x80cE\x95\0\xE2\x14a\x02\x1BW\x80cO\x1E\xF2\x86\x14a\x02GW\x80cR\xD1\x90-\x14a\x02ZW\x80cWW\xA3#\x14a\x02nW_\x80\xFD[\x80c)I\xCAG\x11a\0\xCDW\x80c)I\xCAG\x14a\x01\x87W\x80c6b\xBDb\x14a\x01\xBBW\x80c7\xA0\x8D\x12\x14a\x01\xE8W\x80c9\xF78\x10\x14a\x02\x07W_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c\x11K\xF5q\x14a\x01(W\x80c\x13\xB4\xD5p\x14a\x01IW\x80c\x19\xBA_\xCC\x14a\x01hW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03SV[`@Qa\x01\x1F\x91\x90a\x1FoV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01Ga\x01B6`\x04a\x1F\xB7V[a\x03\xC0V[\0[4\x80\x15a\x01TW_\x80\xFD[Pa\x01Ga\x01c6`\x04a\x1F\xDFV[a\x04bV[4\x80\x15a\x01sW_\x80\xFD[Pa\x01Ga\x01\x826`\x04a\x1F\xDFV[a\x05\xEBV[4\x80\x15a\x01\x92W_\x80\xFD[Pa\x01\xA6a\x01\xA16`\x04a\x1F\xB7V[a\x07\xF5V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1FV[4\x80\x15a\x01\xC6W_\x80\xFD[Pa\x01\xDAa\x01\xD56`\x04a \x0FV[a\x08\xFBV[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xF3W_\x80\xFD[Pa\x01Ga\x02\x026`\x04a\x1F\xB7V[a\tHV[4\x80\x15a\x02\x12W_\x80\xFD[Pa\x01Ga\x0B\xC1V[4\x80\x15a\x02&W_\x80\xFD[Pa\x02:a\x0256`\x04a \x0FV[a\x0C\xE2V[`@Qa\x01\x1F\x91\x90a (V[a\x01Ga\x02U6`\x04a \xB3V[a\r\x8AV[4\x80\x15a\x02eW_\x80\xFD[Pa\x01\xDAa\r\xA9V[4\x80\x15a\x02yW_\x80\xFD[Pa\x02\x8Da\x02\x886`\x04a\x1F\xDFV[a\r\xC5V[`@Qa\x01\x1F\x91\x90a!pV[4\x80\x15a\x02\xA5W_\x80\xFD[Pa\x02\xB9a\x02\xB46`\x04a\x1F\xB7V[a\x0EIV[`@Qa\x01\x1F\x91\x90a!\xBCV[4\x80\x15a\x02\xD1W_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x01Ga\x03\x106`\x04a\x1F\xDFV[a\x0E\xB7V[4\x80\x15a\x03 W_\x80\xFD[Pa\x01\xDAa\x03/6`\x04a\x1F\xB7V[a\x10\xB2V[4\x80\x15a\x03?W_\x80\xFD[Pa\x01\xDAa\x03N6`\x04a\x1F\xB7V[a\x11\x11V[```@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nCiphertextRoots`\x88\x1B\x81RPa\x03\x86_a\x12-V[a\x03\x90`\x01a\x12-V[a\x03\x99_a\x12-V[`@Q` \x01a\x03\xAC\x94\x93\x92\x91\x90a!\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_a\x03\xCA\x83a\x12\xBCV[_\x83\x81R`\x04\x80\x83\x01` R`@\x90\x91 \x90\x81\x01T\x91\x92P\x90`\xFF\x16\x15a\x04\x1BW`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x04-\x84\x84a\x04(a\x13'V[a\x15YV[a\x04\\W`@Qc\x96/C\xBF`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x12V[PPPPV[_a\x04l\x84a\x12\xBCV[_\x84\x81R`\x04\x82\x01` \x90\x81R`@\x80\x83 3\x84R`\x02\x81\x01\x90\x92R\x90\x91 T\x91\x92P\x90`\xFF\x16\x15a\x04\xC9W`@Qc\x03\x0C\xF2s`\xE5\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`$\x81\x01\x85\x90R3`D\x82\x01R`d\x01a\x04\x12V[_a\x04\xD2a\x13'V[\x90Pa\x04\xDE\x813a\x15\xFBV[a\x04\xFDW`@QcR\xD7%\xF5`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[`\x04\x82\x01T_\x90`\xFF\x16\x80a\x05\x18WPa\x05\x18\x87\x87\x84a\x15YV[3_\x90\x81R`\x02\x85\x01` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x89\x84R\x87\x01\x90\x91R\x81 \x80T\x92\x93P\x91\x90\x03a\x05dW\x83T`\x01\x81\x01\x85U_\x85\x81R` \x90 \x01\x86\x90U[\x80T`\x01\x81\x01\x82U_\x82\x81R` \x90 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U`@Q\x88\x90`\x01`\x01`@\x1B\x03\x8B\x16\x90\x7F4\xEDv\x7F\xA58\n:\xE0\xAB6\x06\x0Fp\xB8\xDBu\x02A g\x96&M\xC4\xC8\xD9\n{\x81\x1Af\x90a\x05\xC7\x90\x8B\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x81a\x05\xE1Wa\x05\xDF\x88\x88\x85a\x15YV[P[PPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06;W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06_\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x92W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[\x80_\x03a\x06\xB2W`@Qc\x88\xAC\x08\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06\xBC\x84a\x12\xBCV[`\x01\x80\x82\x01\x80T\x92\x93P\x91_\x91\x83\x91a\x06\xD5\x91\x90a\"\x9FV[\x81T\x81\x10a\x06\xE5Wa\x06\xE5a\"\xB2V[\x90_R` _ \x90`\x02\x02\x01_\x01T`\x01a\x07\0\x91\x90a\"\xC6V[`\x02\x84\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16\x80\x15a\x07 WP\x80\x83`\x03\x01T\x10\x15[\x15a\x078W`\x03\x83\x01Ta\x075\x90`\x01a\"\xC6V[\x90P[\x80\x85\x10\x15a\x07rW`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x82\x90R`d\x01a\x04\x12V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R\x85T`\x01\x80\x82\x01\x88U_\x88\x81R\x84\x90 \x94Q`\x02\x90\x92\x02\x90\x94\x01\x90\x81U\x90Q\x92\x01\x91\x90\x91U\x81Q\x87\x81R\x90\x81\x01\x86\x90R`\x01`\x01`@\x1B\x03\x88\x16\x91\x7F\xA5$P\xAC\n(]\xFA\xFB\xCB\x8F)\xA7\x942\xAFRy5\xB4h\xA5\x1A\xD7\xECZ\xD0\x0C\xDC\xC4\xB0\xC4\x91\x01[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80_a\x08\x01\x85a\x12\xBCV[_\x01\x90P_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x08\x1FWa\x08\x1Fa\"\xB2V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x08\xAAW_\x84\x82\x81T\x81\x10a\x08QWa\x08Qa\"\xB2V[\x90_R` _ \x90`\x02\x02\x01\x90P\x80_\x01T\x89\x10\x15a\x08pWPa\x08\xAAV[\x83T\x81T_\x91a\x08\x7F\x91a\"\x9FV[\x90P\x84`\x01\x01T\x81a\x08\x91\x91\x90a\"\xD9V[a\x08\x9B\x90\x85a\"\xC6V[\x91\x94P\x90\x92PP`\x01\x01a\x085V[P`\x01\x82\x01T\x82Ta\x08\xBC\x90\x89a\"\x9FV[a\x08\xC6\x91\x90a\"\xD9V[a\x08\xD0\x90\x82a\"\xC6V[\x95P`\x01\x82`\x01\x01T\x87a\x08\xE4\x91\x90a\"\xC6V[a\x08\xEE\x91\x90a\"\x9FV[\x94PPPPP\x92P\x92\x90PV[_\x80a\t\x06\x83a\x12\xBCV[`\x02\x81\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16a\t>W`@Qce\xD5\xEA?`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x04\x12V[`\x03\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x98W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xBC\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\t\xEFW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[_a\t\xF9\x83a\x12\xBCV[`\x01\x80\x82\x01\x80T\x92\x93P\x91\x90[\x81\x81\x10\x80\x15a\n5WP\x84\x83\x82\x81T\x81\x10a\n#Wa\n#a\"\xB2V[\x90_R` _ \x90`\x02\x02\x01_\x01T\x14\x15[\x15a\nLW\x80a\nD\x81a\"\xF0V[\x91PPa\n\x06V[\x81\x81\x03a\n~W`@Qc\xBCY\xE7\x97`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x12V[`\x02\x84\x01Ta\x01\0\x90\x04`\xFF\x16\x80\x15a\n\x9BWP\x83`\x03\x01T\x85\x11\x15[\x15a\n\xE5W\x85\x85\x85`\x03\x01T`\x01a\n\xB3\x91\x90a\"\xC6V[`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`D\x82\x01R`d\x01a\x04\x12V[\x80[a\n\xF2`\x01\x84a\"\x9FV[\x81\x10\x15a\x0BWW\x83a\x0B\x05\x82`\x01a\"\xC6V[\x81T\x81\x10a\x0B\x15Wa\x0B\x15a\"\xB2V[\x90_R` _ \x90`\x02\x02\x01\x84\x82\x81T\x81\x10a\x0B3Wa\x0B3a\"\xB2V[_\x91\x82R` \x90\x91 \x82T`\x02\x90\x92\x02\x01\x90\x81U`\x01\x91\x82\x01T\x90\x82\x01U\x01a\n\xE7V[P\x82\x80T\x80a\x0BhWa\x0Bha#\x08V[_\x82\x81R` \x81 `\x02_\x19\x90\x93\x01\x92\x83\x02\x01\x81\x81U`\x01\x01U\x90U`@Q`\x01`\x01`@\x1B\x03\x87\x16\x90\x7F{\x9As\xA4g\xAC\xDB'\xEE\xE3^\x88\x0E\xC7u\xDB\x86\x92\\\xD8+\x94\xCBL\xB6\xDD\xF7\x0E\xB5\xCC\xA5\xDB\x90a\x07\xE5\x90\x88\x81R` \x01\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\x0C\x15W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x02\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x0C^WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\x0C|W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17h\xFF\0\0\0\0\0\0\0\0\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01Ra\x0C\xFF\x82a\x12\xBCV[_\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\r|W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r9V[PPP\x91RP\x90\x93\x92PPPV[a\r\x92a\x17!V[a\r\x9B\x82a\x17\xC7V[a\r\xA5\x82\x82a\x18qV[PPV[_a\r\xB2a\x192V[P_\x80Q` a#\x8D\x839\x81Q\x91R[\x90V[``a\r\xD0\x84a\x12\xBCV[_\x84\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x80\x83 \x85\x84R`\x01\x01\x82R\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R\x90\x91\x83\x01\x82\x82\x80\x15a\x0E:W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0E\x1CW[PPPPP\x90P[\x93\x92PPPV[``a\x0ET\x83a\x12\xBCV[_\x83\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R\x90\x91\x83\x01\x82\x82\x80\x15a\x0E\xA9W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\x95W[PPPPP\x90P[\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x07W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F+\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0F^W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[\x82`\x01`\x01`@\x1B\x03\x16_\x03a\x0F\x87W`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x03a\x0F\xA7W`@Qc\x88\xAC\x08\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16\x15a\x10\x0EW`@Qc\x0F\x17c}`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x12V[`\x02\x80\x82\x01\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x84\x83U`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x88\x81R\x87\x86\x01\x80T\x80\x88\x01\x82U\x90\x84R\x91\x90\x92 \x92Q\x95\x02\x90\x91\x01\x93\x84UQ\x92\x90\x91\x01\x91\x90\x91UQ\x81\x90`\x01`\x01`@\x1B\x03\x86\x16\x90\x7F;:\xC5?Am\x9A\xD0N%\x8Fd}!\xD50\x88\xF7\x89\xD2I*\xBF\x91\x08\xE4\t\xF1\x01\x8F;\xEA\x90a\x10\xA3\x90\x87\x90\x87\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[_\x80a\x10\xBD\x84a\x12\xBCV[_\x84\x81R`\x04\x91\x82\x01` R`@\x90 \x90\x81\x01T\x90\x91P`\xFF\x16a\x11\x06W`@Qc\x15\x80b\xCB`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x12V[`\x03\x01T\x93\x92PPPV[_\x80a\x11\x1C\x84a\x12\xBCV[\x80T\x90\x91P\x83\x10\x15a\x11SW`@Qc\x1D\xD7\x9B\xC5`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x12V[_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x11mWa\x11ma\"\xB2V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x11\xFCW_\x84\x82\x81T\x81\x10a\x11\x9FWa\x11\x9Fa\"\xB2V[_\x91\x82R` \x82 \x86T`\x02\x90\x92\x02\x01\x80T\x90\x93Pa\x11\xBE\x91\x90a\"\x9FV[\x90P_\x85`\x01\x01T\x82a\x11\xD1\x91\x90a\"\xD9V[a\x11\xDB\x90\x86a\"\xC6V[\x90P\x80\x8A\x10\x15a\x11\xEDWPPPa\x11\xFCV[\x91\x94P\x90\x92PP`\x01\x01a\x11\x83V[P`\x01\x82\x01Ta\x12\x0C\x82\x88a\"\x9FV[a\x12\x16\x91\x90a#\x1CV[\x82Ta\x12\"\x91\x90a\"\xC6V[\x97\x96PPPPPPPV[``_a\x129\x83a\x19{V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12WWa\x12Wa \x9FV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\x81W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12\x8BWP\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16a\x13\"W`@Qc)\xCB\xDD#`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x12V[\x91\x90PV[a\x13V`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81RP\x90V[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x17\n)\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xA6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xCA\x91\x90a#;V[\x80\x82R`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14@\x91\x90a#;V[``\x82\x01R\x80Qa\x14S\x90`\x01\x90a\"\x9FV[` \x82\x01\x81\x90R`@Qc\x0E\x18\x87\xC9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\x0E\x18\x87\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xCD\x91\x90a#RV[\x15\x80\x15`@\x83\x01Ra\r\xC2W` \x81\x01Q`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15-W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15Q\x91\x90a#;V[`\x80\x82\x01R\x90V[_\x80a\x15d\x85a\x12\xBCV[_\x85\x81R`\x04\x91\x90\x91\x01` R`@\x81 \x91P\x81\x90[\x81T\x81\x10\x15a\x15\xEFW_\x82\x82\x81T\x81\x10a\x15\x96Wa\x15\x96a\"\xB2V[\x90_R` _ \x01T\x90P_\x80a\x15\xBF\x88\x87`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ a\x1ARV[\x91P\x91P\x81\x15a\x15\xE4Wa\x15\xD5\x8A\x8A\x85\x84a\x1C\x83V[`\x01\x96PPPPPPPa\x0EBV[PPP`\x01\x01a\x15zV[P_\x96\x95PPPPPPV[\x81Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16]W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\x81\x91\x90a#RV[\x15a\x16\x8EWP`\x01a\x0E\xB1V[\x82`@\x01Q\x80\x15a\x0EBWP` \x83\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xFDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EB\x91\x90a#RV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x17\xA7WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x17\x9B_\x80Q` a#\x8D\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x17\xC5W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x17W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18;\x91\x90a\"pV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18nW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x12V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18\xCBWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18\xC8\x91\x81\x01\x90a#;V[`\x01[a\x18\xF3W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x12V[_\x80Q` a#\x8D\x839\x81Q\x91R\x81\x14a\x19#W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\x12V[a\x19-\x83\x83a\x1D\x7FV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\xC5W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x19\xB9Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x19\xE5Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1A\x03Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1A\x1BWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1A/Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1AAW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x0E\xB1W`\x01\x01\x92\x91PPV[\x80T``\x83\x01Q`@\x84\x01Q_\x92\x83\x92\x90\x91\x90\x82\x10\x15\x90\x83\x90\x80\x15a\x1A{WP\x86`\x80\x01Q\x83\x10\x15[\x90P\x81\x15\x80\x15a\x1A\x89WP\x80\x15[\x15a\x1A\x9CW_\x80\x94P\x94PPPPa\x1C|V[_\x80_[\x85\x81\x10\x15a\x1C\x1BW_\x89\x82\x81T\x81\x10a\x1A\xBBWa\x1A\xBBa\"\xB2V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x85\x80\x15a\x1B]WP\x8AQ`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B]\x91\x90a#RV[\x15a\x1BpW\x83a\x1Bl\x81a\"\xF0V[\x94PP[\x84\x80\x15a\x1B\xFFWP` \x8B\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xFF\x91\x90a#RV[\x15a\x1C\x12W\x82a\x1C\x0E\x81a\"\xF0V[\x93PP[P`\x01\x01a\x1A\xA0V[P\x83\x80\x15a\x1C-WP\x88``\x01Q\x82\x10\x15[\x15a\x1CEWPP\x86Q`\x01\x95P\x93Pa\x1C|\x92PPPV[\x82\x80\x15a\x1CVWP\x88`\x80\x01Q\x81\x10\x15[\x15a\x1CpW`\x01\x89` \x01Q\x96P\x96PPPPPPa\x1C|V[_\x80\x96P\x96PPPPPP[\x92P\x92\x90PV[_a\x1C\x8D\x85a\x12\xBCV[_\x85\x81R`\x04\x80\x83\x01` R`@\x90\x91 \x90\x81\x01T\x91\x92P\x90`\xFF\x16\x15a\x1C\xD9W`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x12V[`\x03\x81\x01\x84\x90U`\x04\x81\x01\x80T`\xFF\x19\x16`\x01\x17\x90U`\x02\x82\x01Ta\x01\0\x90\x04`\xFF\x16a\x1D\x1DW`\x02\x82\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U`\x03\x82\x01\x85\x90Ua\x1D1V[\x81`\x03\x01T\x85\x11\x15a\x1D1W`\x03\x82\x01\x85\x90U[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x86\x91`\x01`\x01`@\x1B\x03\x89\x16\x91\x7F\x96\x89\xD96`\x9E\xEE\xD0@\x1BO\xEC\xCE\xCF\xEA\x81!m\xE8\xFF\x02\xBC\x8END\xD0\xCD\x9Fg,\08\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[a\x1D\x88\x82a\x1D\xD4V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x1D\xCCWa\x19-\x82\x82a\x1E7V[a\r\xA5a\x1E\xA9V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x1E\tW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04\x12V[_\x80Q` a#\x8D\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x1ES\x91\x90a#qV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x1E\x8BW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1E\x90V[``\x91P[P\x91P\x91Pa\x1E\xA0\x85\x83\x83a\x1E\xC8V[\x95\x94PPPPPV[4\x15a\x17\xC5W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x1E\xDDWa\x1E\xD8\x82a\x1F$V[a\x0EBV[\x81Q\x15\x80\x15a\x1E\xF4WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x1F\x1DW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x12V[P\x80a\x0EBV[\x80Q\x15a\x1F4W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a\x1FgW\x81\x81\x01Q\x83\x82\x01R` \x01a\x1FOV[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra\x1F\x8D\x81`@\x85\x01` \x87\x01a\x1FMV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13\"W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\x1F\xC8W_\x80\xFD[a\x1F\xD1\x83a\x1F\xA1V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_``\x84\x86\x03\x12\x15a\x1F\xF1W_\x80\xFD[a\x1F\xFA\x84a\x1F\xA1V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a \x1FW_\x80\xFD[a\x0EB\x82a\x1F\xA1V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R_\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a \x7FW\x84Q\x80Q\x83R\x86\x01Q\x86\x83\x01R\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a VV[P\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18nW_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a \xC4W_\x80\xFD[\x825a \xCF\x81a \x8BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \xEAW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a \xFDW_\x80\xFD[\x815\x81\x81\x11\x15a!\x0FWa!\x0Fa \x9FV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a!7Wa!7a \x9FV[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a!OW_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a!\xB0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a!\x8BV[P\x90\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a!\xB0W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a!\xD7V[_\x85Qa\"\x04\x81\x84` \x8A\x01a\x1FMV[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa\"#\x81`\x02\x84\x01` \x8A\x01a\x1FMV[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa\"G\x81`\x03\x85\x01` \x8A\x01a\x1FMV[`\x03\x92\x01\x91\x82\x01R\x83Qa\"b\x81`\x04\x84\x01` \x88\x01a\x1FMV[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\"\x80W_\x80\xFD[\x81Qa\x0EB\x81a \x8BV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E\xB1Wa\x0E\xB1a\"\x8BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0E\xB1Wa\x0E\xB1a\"\x8BV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E\xB1Wa\x0E\xB1a\"\x8BV[_`\x01\x82\x01a#\x01Wa#\x01a\"\x8BV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82a#6WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a#KW_\x80\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a#bW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0EBW_\x80\xFD[_\x82Qa#\x82\x81\x84` \x87\x01a\x1FMV[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
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
    /**Custom error with signature `BlockBeforeStartBlock(uint64,uint256)` and selector `0x775e6f14`.
```solidity
error BlockBeforeStartBlock(uint64 chainId, uint256 blockNumber);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BlockBeforeStartBlock {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
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
        impl ::core::convert::From<BlockBeforeStartBlock> for UnderlyingRustTuple<'_> {
            fn from(value: BlockBeforeStartBlock) -> Self {
                (value.chainId, value.blockNumber)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BlockBeforeStartBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    blockNumber: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BlockBeforeStartBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BlockBeforeStartBlock(uint64,uint256)";
            const SELECTOR: [u8; 4] = [119u8, 94u8, 111u8, 20u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Custom error with signature `EpochLengthNotFound(uint64,uint256)` and selector `0xbc59e797`.
```solidity
error EpochLengthNotFound(uint64 chainId, uint256 fromEpoch);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochLengthNotFound {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
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
        impl ::core::convert::From<EpochLengthNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: EpochLengthNotFound) -> Self {
                (value.chainId, value.fromEpoch)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochLengthNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    fromEpoch: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochLengthNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochLengthNotFound(uint64,uint256)";
            const SELECTOR: [u8; 4] = [188u8, 89u8, 231u8, 151u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromEpoch),
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
    /**Custom error with signature `EpochRootAlreadyRecorded(uint64,uint256)` and selector `0xfd703b18`.
```solidity
error EpochRootAlreadyRecorded(uint64 chainId, uint256 epoch);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochRootAlreadyRecorded {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
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
        impl ::core::convert::From<EpochRootAlreadyRecorded>
        for UnderlyingRustTuple<'_> {
            fn from(value: EpochRootAlreadyRecorded) -> Self {
                (value.chainId, value.epoch)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EpochRootAlreadyRecorded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    epoch: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochRootAlreadyRecorded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochRootAlreadyRecorded(uint64,uint256)";
            const SELECTOR: [u8; 4] = [253u8, 112u8, 59u8, 24u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Custom error with signature `EpochScheduleAlreadySet(uint64)` and selector `0x3c5d8df4`.
```solidity
error EpochScheduleAlreadySet(uint64 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochScheduleAlreadySet {
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<EpochScheduleAlreadySet> for UnderlyingRustTuple<'_> {
            fn from(value: EpochScheduleAlreadySet) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochScheduleAlreadySet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochScheduleAlreadySet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochScheduleAlreadySet(uint64)";
            const SELECTOR: [u8; 4] = [60u8, 93u8, 141u8, 244u8];
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
    /**Custom error with signature `FromEpochTooLow(uint64,uint256,uint256)` and selector `0xc3216f19`.
```solidity
error FromEpochTooLow(uint64 chainId, uint256 fromEpoch, uint256 minFromEpoch);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FromEpochTooLow {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minFromEpoch: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
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
        impl ::core::convert::From<FromEpochTooLow> for UnderlyingRustTuple<'_> {
            fn from(value: FromEpochTooLow) -> Self {
                (value.chainId, value.fromEpoch, value.minFromEpoch)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FromEpochTooLow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    fromEpoch: tuple.1,
                    minFromEpoch: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FromEpochTooLow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FromEpochTooLow(uint64,uint256,uint256)";
            const SELECTOR: [u8; 4] = [195u8, 33u8, 111u8, 25u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromEpoch),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minFromEpoch),
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
    /**Custom error with signature `InvalidEpochLength()` and selector `0x88ac0897`.
```solidity
error InvalidEpochLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEpochLength;
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
        impl ::core::convert::From<InvalidEpochLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEpochLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidEpochLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEpochLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEpochLength()";
            const SELECTOR: [u8; 4] = [136u8, 172u8, 8u8, 151u8];
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
    /**Custom error with signature `NoEpochRoot(uint64,uint256)` and selector `0x2b00c596`.
```solidity
error NoEpochRoot(uint64 chainId, uint256 epoch);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoEpochRoot {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
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
        impl ::core::convert::From<NoEpochRoot> for UnderlyingRustTuple<'_> {
            fn from(value: NoEpochRoot) -> Self {
                (value.chainId, value.epoch)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoEpochRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    epoch: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoEpochRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoEpochRoot(uint64,uint256)";
            const SELECTOR: [u8; 4] = [43u8, 0u8, 197u8, 150u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Custom error with signature `NoEpochRootRecorded(uint64)` and selector `0xcbabd47e`.
```solidity
error NoEpochRootRecorded(uint64 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoEpochRootRecorded {
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<NoEpochRootRecorded> for UnderlyingRustTuple<'_> {
            fn from(value: NoEpochRootRecorded) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoEpochRootRecorded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoEpochRootRecorded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoEpochRootRecorded(uint64)";
            const SELECTOR: [u8; 4] = [203u8, 171u8, 212u8, 126u8];
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
    /**Custom error with signature `NoEpochSchedule(uint64)` and selector `0x29cbdd23`.
```solidity
error NoEpochSchedule(uint64 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoEpochSchedule {
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
        #[allow(dead_code)]
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
        impl ::core::convert::From<NoEpochSchedule> for UnderlyingRustTuple<'_> {
            fn from(value: NoEpochSchedule) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoEpochSchedule {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoEpochSchedule {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoEpochSchedule(uint64)";
            const SELECTOR: [u8; 4] = [41u8, 203u8, 221u8, 35u8];
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
    /**Custom error with signature `NoRootReachesThreshold(uint64,uint256)` and selector `0x962f43bf`.
```solidity
error NoRootReachesThreshold(uint64 chainId, uint256 epoch);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoRootReachesThreshold {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
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
        impl ::core::convert::From<NoRootReachesThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: NoRootReachesThreshold) -> Self {
                (value.chainId, value.epoch)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoRootReachesThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    epoch: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoRootReachesThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoRootReachesThreshold(uint64,uint256)";
            const SELECTOR: [u8; 4] = [150u8, 47u8, 67u8, 191u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
error NotCoprocessorTxSender(address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotCoprocessorTxSender {
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
        impl ::core::convert::From<NotCoprocessorTxSender> for UnderlyingRustTuple<'_> {
            fn from(value: NotCoprocessorTxSender) -> Self {
                (value.sender,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotCoprocessorTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { sender: tuple.0 }
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
    /**Custom error with signature `RootAlreadyPosted(uint64,uint256,address)` and selector `0x619e4e60`.
```solidity
error RootAlreadyPosted(uint64 chainId, uint256 epoch, address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RootAlreadyPosted {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
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
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
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
        impl ::core::convert::From<RootAlreadyPosted> for UnderlyingRustTuple<'_> {
            fn from(value: RootAlreadyPosted) -> Self {
                (value.chainId, value.epoch, value.sender)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RootAlreadyPosted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    epoch: tuple.1,
                    sender: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RootAlreadyPosted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RootAlreadyPosted(uint64,uint256,address)";
            const SELECTOR: [u8; 4] = [97u8, 158u8, 78u8, 96u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
    /**Event with signature `EpochLengthRemoved(uint64,uint256)` and selector `0x7b9a73a467acdb27eee35e880ec775db86925cd82b94cb4cb6ddf70eb5cca5db`.
```solidity
event EpochLengthRemoved(uint64 indexed chainId, uint256 fromEpoch);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochLengthRemoved {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EpochLengthRemoved {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "EpochLengthRemoved(uint64,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                123u8, 154u8, 115u8, 164u8, 103u8, 172u8, 219u8, 39u8, 238u8, 227u8,
                94u8, 136u8, 14u8, 199u8, 117u8, 219u8, 134u8, 146u8, 92u8, 216u8, 43u8,
                148u8, 203u8, 76u8, 182u8, 221u8, 247u8, 14u8, 181u8, 204u8, 165u8, 219u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    chainId: topics.1,
                    fromEpoch: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.fromEpoch),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.chainId.clone())
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.chainId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EpochLengthRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochLengthRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EpochLengthRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EpochLengthSet(uint64,uint256,uint256)` and selector `0xa52450ac0a285dfafbcb8f29a79432af527935b468a51ad7ec5ad00cdcc4b0c4`.
```solidity
event EpochLengthSet(uint64 indexed chainId, uint256 fromEpoch, uint256 epochLength);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochLengthSet {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochLength: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EpochLengthSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "EpochLengthSet(uint64,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                165u8, 36u8, 80u8, 172u8, 10u8, 40u8, 93u8, 250u8, 251u8, 203u8, 143u8,
                41u8, 167u8, 148u8, 50u8, 175u8, 82u8, 121u8, 53u8, 180u8, 104u8, 165u8,
                26u8, 215u8, 236u8, 90u8, 208u8, 12u8, 220u8, 196u8, 176u8, 196u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    chainId: topics.1,
                    fromEpoch: data.0,
                    epochLength: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.fromEpoch),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochLength),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.chainId.clone())
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.chainId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EpochLengthSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochLengthSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EpochLengthSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EpochRootRecorded(uint64,uint256,bytes32,uint256)` and selector `0x9689d936609eeed0401b4feccecfea81216de8ff02bc8e4e44d0cd9f672c0038`.
```solidity
event EpochRootRecorded(uint64 indexed chainId, uint256 indexed epoch, bytes32 root, uint256 coprocessorContextId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochRootRecorded {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for EpochRootRecorded {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "EpochRootRecorded(uint64,uint256,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                150u8, 137u8, 217u8, 54u8, 96u8, 158u8, 238u8, 208u8, 64u8, 27u8, 79u8,
                236u8, 206u8, 207u8, 234u8, 129u8, 33u8, 109u8, 232u8, 255u8, 2u8, 188u8,
                142u8, 78u8, 68u8, 208u8, 205u8, 159u8, 103u8, 44u8, 0u8, 56u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    chainId: topics.1,
                    epoch: topics.2,
                    root: data.0,
                    coprocessorContextId: data.1,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.coprocessorContextId),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.chainId.clone(), self.epoch.clone())
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.chainId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epoch);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EpochRootRecorded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochRootRecorded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EpochRootRecorded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EpochScheduleSet(uint64,uint256,uint256)` and selector `0x3b3ac53f416d9ad04e258f647d21d53088f789d2492abf9108e409f1018f3bea`.
```solidity
event EpochScheduleSet(uint64 indexed chainId, uint256 startBlock, uint256 epochLength);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochScheduleSet {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub startBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochLength: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for EpochScheduleSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "EpochScheduleSet(uint64,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8, 58u8, 197u8, 63u8, 65u8, 109u8, 154u8, 208u8, 78u8, 37u8, 143u8,
                100u8, 125u8, 33u8, 213u8, 48u8, 136u8, 247u8, 137u8, 210u8, 73u8, 42u8,
                191u8, 145u8, 8u8, 228u8, 9u8, 241u8, 1u8, 143u8, 59u8, 234u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    chainId: topics.1,
                    startBlock: data.0,
                    epochLength: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochLength),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.chainId.clone())
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.chainId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EpochScheduleSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochScheduleSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EpochScheduleSet) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `RootPosted(uint64,uint256,address,bytes32)` and selector `0x34ed767fa5380a3ae0ab36060f70b8db750241206796264dc4c8d90a7b811a66`.
```solidity
event RootPosted(uint64 indexed chainId, uint256 indexed epoch, address indexed sender, bytes32 root);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RootPosted {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for RootPosted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RootPosted(uint64,uint256,address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                52u8, 237u8, 118u8, 127u8, 165u8, 56u8, 10u8, 58u8, 224u8, 171u8, 54u8,
                6u8, 15u8, 112u8, 184u8, 219u8, 117u8, 2u8, 65u8, 32u8, 103u8, 150u8,
                38u8, 77u8, 196u8, 200u8, 217u8, 10u8, 123u8, 129u8, 26u8, 102u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    chainId: topics.1,
                    epoch: topics.2,
                    sender: topics.3,
                    root: data.0,
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.chainId.clone(),
                    self.epoch.clone(),
                    self.sender.clone(),
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.chainId);
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.epoch);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RootPosted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RootPosted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RootPosted) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `getEpochBlocks(uint64,uint256)` and selector `0x2949ca47`.
```solidity
function getEpochBlocks(uint64 chainId, uint256 epoch) external view returns (uint256 startBlock, uint256 endBlock);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochBlocksCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getEpochBlocks(uint64,uint256)`](getEpochBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochBlocksReturn {
        #[allow(missing_docs)]
        pub startBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endBlock: alloy::sol_types::private::primitives::aliases::U256,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<getEpochBlocksCall> for UnderlyingRustTuple<'_> {
                fn from(value: getEpochBlocksCall) -> Self {
                    (value.chainId, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getEpochBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        epoch: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getEpochBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEpochBlocksReturn) -> Self {
                    (value.startBlock, value.endBlock)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEpochBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startBlock: tuple.0,
                        endBlock: tuple.1,
                    }
                }
            }
        }
        impl getEpochBlocksReturn {
            fn _tokenize(
                &self,
            ) -> <getEpochBlocksCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.endBlock),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEpochBlocksCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getEpochBlocksReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEpochBlocks(uint64,uint256)";
            const SELECTOR: [u8; 4] = [41u8, 73u8, 202u8, 71u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getEpochBlocksReturn::_tokenize(ret)
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
    /**Function with signature `getEpochOfBlock(uint64,uint256)` and selector `0xf5178ccf`.
```solidity
function getEpochOfBlock(uint64 chainId, uint256 blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochOfBlockCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getEpochOfBlock(uint64,uint256)`](getEpochOfBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochOfBlockReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<getEpochOfBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getEpochOfBlockCall) -> Self {
                    (value.chainId, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getEpochOfBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        blockNumber: tuple.1,
                    }
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
            impl ::core::convert::From<getEpochOfBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEpochOfBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEpochOfBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEpochOfBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEpochOfBlock(uint64,uint256)";
            const SELECTOR: [u8; 4] = [245u8, 23u8, 140u8, 207u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
                        let r: getEpochOfBlockReturn = r.into();
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
                        let r: getEpochOfBlockReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getEpochRoot(uint64,uint256)` and selector `0xe8ad3587`.
```solidity
function getEpochRoot(uint64 chainId, uint256 epoch) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochRootCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getEpochRoot(uint64,uint256)`](getEpochRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochRootReturn {
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<getEpochRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getEpochRootCall) -> Self {
                    (value.chainId, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getEpochRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        epoch: tuple.1,
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
            impl ::core::convert::From<getEpochRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getEpochRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getEpochRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEpochRootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEpochRoot(uint64,uint256)";
            const SELECTOR: [u8; 4] = [232u8, 173u8, 53u8, 135u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
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
                        let r: getEpochRootReturn = r.into();
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
                        let r: getEpochRootReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getEpochSchedule(uint64)` and selector `0x459500e2`.
```solidity
function getEpochSchedule(uint64 chainId) external view returns (ICiphertextRoots.EpochSchedule memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochScheduleCall {
        #[allow(missing_docs)]
        pub chainId: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`getEpochSchedule(uint64)`](getEpochScheduleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getEpochScheduleReturn {
        #[allow(missing_docs)]
        pub _0: <ICiphertextRoots::EpochSchedule as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getEpochScheduleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEpochScheduleCall) -> Self {
                    (value.chainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEpochScheduleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
            type UnderlyingSolTuple<'a> = (ICiphertextRoots::EpochSchedule,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ICiphertextRoots::EpochSchedule as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getEpochScheduleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getEpochScheduleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getEpochScheduleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getEpochScheduleCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ICiphertextRoots::EpochSchedule as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ICiphertextRoots::EpochSchedule,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getEpochSchedule(uint64)";
            const SELECTOR: [u8; 4] = [69u8, 149u8, 0u8, 226u8];
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
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <ICiphertextRoots::EpochSchedule as alloy_sol_types::SolType>::tokenize(
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
                        let r: getEpochScheduleReturn = r.into();
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
                        let r: getEpochScheduleReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getHighestRecordedEpoch(uint64)` and selector `0x3662bd62`.
```solidity
function getHighestRecordedEpoch(uint64 chainId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHighestRecordedEpochCall {
        #[allow(missing_docs)]
        pub chainId: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getHighestRecordedEpoch(uint64)`](getHighestRecordedEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHighestRecordedEpochReturn {
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
            impl ::core::convert::From<getHighestRecordedEpochCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHighestRecordedEpochCall) -> Self {
                    (value.chainId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHighestRecordedEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainId: tuple.0 }
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
            impl ::core::convert::From<getHighestRecordedEpochReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHighestRecordedEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHighestRecordedEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHighestRecordedEpochCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHighestRecordedEpoch(uint64)";
            const SELECTOR: [u8; 4] = [54u8, 98u8, 189u8, 98u8];
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
                        let r: getHighestRecordedEpochReturn = r.into();
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
                        let r: getHighestRecordedEpochReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getPostedRoots(uint64,uint256)` and selector `0x91940608`.
```solidity
function getPostedRoots(uint64 chainId, uint256 epoch) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPostedRootsCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getPostedRoots(uint64,uint256)`](getPostedRootsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPostedRootsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<getPostedRootsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPostedRootsCall) -> Self {
                    (value.chainId, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPostedRootsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        epoch: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            #[allow(dead_code)]
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
            impl ::core::convert::From<getPostedRootsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPostedRootsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPostedRootsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPostedRootsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPostedRoots(uint64,uint256)";
            const SELECTOR: [u8; 4] = [145u8, 148u8, 6u8, 8u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getPostedRootsReturn = r.into();
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
                        let r: getPostedRootsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getRootSenders(uint64,uint256,bytes32)` and selector `0x5757a323`.
```solidity
function getRootSenders(uint64 chainId, uint256 epoch, bytes32 root) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRootSendersCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getRootSenders(uint64,uint256,bytes32)`](getRootSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRootSendersReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRootSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRootSendersCall) -> Self {
                    (value.chainId, value.epoch, value.root)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRootSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        epoch: tuple.1,
                        root: tuple.2,
                    }
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
            impl ::core::convert::From<getRootSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRootSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRootSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRootSendersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
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
            const SIGNATURE: &'static str = "getRootSenders(uint64,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [87u8, 87u8, 163u8, 35u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
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
                        let r: getRootSendersReturn = r.into();
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
                        let r: getRootSendersReturn = r.into();
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
    /**Function with signature `postRoot(uint64,uint256,bytes32)` and selector `0x13b4d570`.
```solidity
function postRoot(uint64 chainId, uint256 epoch, bytes32 root) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postRootCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`postRoot(uint64,uint256,bytes32)`](postRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postRootReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<postRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: postRootCall) -> Self {
                    (value.chainId, value.epoch, value.root)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        epoch: tuple.1,
                        root: tuple.2,
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
            impl ::core::convert::From<postRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: postRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl postRootReturn {
            fn _tokenize(
                &self,
            ) -> <postRootCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for postRootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = postRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "postRoot(uint64,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [19u8, 180u8, 213u8, 112u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                postRootReturn::_tokenize(ret)
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
    /**Function with signature `recordEpochRoot(uint64,uint256)` and selector `0x114bf571`.
```solidity
function recordEpochRoot(uint64 chainId, uint256 epoch) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recordEpochRootCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`recordEpochRoot(uint64,uint256)`](recordEpochRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recordEpochRootReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<recordEpochRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: recordEpochRootCall) -> Self {
                    (value.chainId, value.epoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for recordEpochRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        epoch: tuple.1,
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
            impl ::core::convert::From<recordEpochRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: recordEpochRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for recordEpochRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl recordEpochRootReturn {
            fn _tokenize(
                &self,
            ) -> <recordEpochRootCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for recordEpochRootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = recordEpochRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "recordEpochRoot(uint64,uint256)";
            const SELECTOR: [u8; 4] = [17u8, 75u8, 245u8, 113u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epoch),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                recordEpochRootReturn::_tokenize(ret)
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
    /**Function with signature `removeEpochLength(uint64,uint256)` and selector `0x37a08d12`.
```solidity
function removeEpochLength(uint64 chainId, uint256 fromEpoch) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeEpochLengthCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removeEpochLength(uint64,uint256)`](removeEpochLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeEpochLengthReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<removeEpochLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeEpochLengthCall) -> Self {
                    (value.chainId, value.fromEpoch)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeEpochLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        fromEpoch: tuple.1,
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
            impl ::core::convert::From<removeEpochLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeEpochLengthReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeEpochLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeEpochLengthReturn {
            fn _tokenize(
                &self,
            ) -> <removeEpochLengthCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeEpochLengthCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeEpochLengthReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeEpochLength(uint64,uint256)";
            const SELECTOR: [u8; 4] = [55u8, 160u8, 141u8, 18u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromEpoch),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeEpochLengthReturn::_tokenize(ret)
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
    /**Function with signature `setEpochLength(uint64,uint256,uint256)` and selector `0x19ba5fcc`.
```solidity
function setEpochLength(uint64 chainId, uint256 fromEpoch, uint256 epochLength) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEpochLengthCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochLength: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setEpochLength(uint64,uint256,uint256)`](setEpochLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEpochLengthReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<setEpochLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: setEpochLengthCall) -> Self {
                    (value.chainId, value.fromEpoch, value.epochLength)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setEpochLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        fromEpoch: tuple.1,
                        epochLength: tuple.2,
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
            impl ::core::convert::From<setEpochLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setEpochLengthReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setEpochLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setEpochLengthReturn {
            fn _tokenize(
                &self,
            ) -> <setEpochLengthCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEpochLengthCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEpochLengthReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEpochLength(uint64,uint256,uint256)";
            const SELECTOR: [u8; 4] = [25u8, 186u8, 95u8, 204u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.fromEpoch),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochLength),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setEpochLengthReturn::_tokenize(ret)
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
    /**Function with signature `setEpochSchedule(uint64,uint256,uint256)` and selector `0xde93c514`.
```solidity
function setEpochSchedule(uint64 chainId, uint256 startBlock, uint256 epochLength) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEpochScheduleCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub startBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub epochLength: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setEpochSchedule(uint64,uint256,uint256)`](setEpochScheduleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setEpochScheduleReturn {}
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
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
            impl ::core::convert::From<setEpochScheduleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setEpochScheduleCall) -> Self {
                    (value.chainId, value.startBlock, value.epochLength)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setEpochScheduleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainId: tuple.0,
                        startBlock: tuple.1,
                        epochLength: tuple.2,
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
            impl ::core::convert::From<setEpochScheduleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setEpochScheduleReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setEpochScheduleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setEpochScheduleReturn {
            fn _tokenize(
                &self,
            ) -> <setEpochScheduleCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setEpochScheduleCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setEpochScheduleReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setEpochSchedule(uint64,uint256,uint256)";
            const SELECTOR: [u8; 4] = [222u8, 147u8, 197u8, 20u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startBlock),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.epochLength),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setEpochScheduleReturn::_tokenize(ret)
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
    ///Container for all the [`CiphertextRoots`](self) function calls.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum CiphertextRootsCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        getEpochBlocks(getEpochBlocksCall),
        #[allow(missing_docs)]
        getEpochOfBlock(getEpochOfBlockCall),
        #[allow(missing_docs)]
        getEpochRoot(getEpochRootCall),
        #[allow(missing_docs)]
        getEpochSchedule(getEpochScheduleCall),
        #[allow(missing_docs)]
        getHighestRecordedEpoch(getHighestRecordedEpochCall),
        #[allow(missing_docs)]
        getPostedRoots(getPostedRootsCall),
        #[allow(missing_docs)]
        getRootSenders(getRootSendersCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        postRoot(postRootCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        recordEpochRoot(recordEpochRootCall),
        #[allow(missing_docs)]
        removeEpochLength(removeEpochLengthCall),
        #[allow(missing_docs)]
        setEpochLength(setEpochLengthCall),
        #[allow(missing_docs)]
        setEpochSchedule(setEpochScheduleCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    impl CiphertextRootsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 142u8, 110u8, 44u8],
            [17u8, 75u8, 245u8, 113u8],
            [19u8, 180u8, 213u8, 112u8],
            [25u8, 186u8, 95u8, 204u8],
            [41u8, 73u8, 202u8, 71u8],
            [54u8, 98u8, 189u8, 98u8],
            [55u8, 160u8, 141u8, 18u8],
            [57u8, 247u8, 56u8, 16u8],
            [69u8, 149u8, 0u8, 226u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [87u8, 87u8, 163u8, 35u8],
            [145u8, 148u8, 6u8, 8u8],
            [173u8, 60u8, 177u8, 204u8],
            [222u8, 147u8, 197u8, 20u8],
            [232u8, 173u8, 53u8, 135u8],
            [245u8, 23u8, 140u8, 207u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(getVersion),
            ::core::stringify!(recordEpochRoot),
            ::core::stringify!(postRoot),
            ::core::stringify!(setEpochLength),
            ::core::stringify!(getEpochBlocks),
            ::core::stringify!(getHighestRecordedEpoch),
            ::core::stringify!(removeEpochLength),
            ::core::stringify!(initializeFromEmptyProxy),
            ::core::stringify!(getEpochSchedule),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(getRootSenders),
            ::core::stringify!(getPostedRoots),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(setEpochSchedule),
            ::core::stringify!(getEpochRoot),
            ::core::stringify!(getEpochOfBlock),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <recordEpochRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <postRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setEpochLengthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getEpochBlocksCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getHighestRecordedEpochCall as alloy_sol_types::SolCall>::SIGNATURE,
            <removeEpochLengthCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getEpochScheduleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getRootSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getPostedRootsCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <setEpochScheduleCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getEpochRootCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getEpochOfBlockCall as alloy_sol_types::SolCall>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for CiphertextRootsCalls {
        const NAME: &'static str = "CiphertextRootsCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEpochBlocks(_) => {
                    <getEpochBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEpochOfBlock(_) => {
                    <getEpochOfBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEpochRoot(_) => {
                    <getEpochRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getEpochSchedule(_) => {
                    <getEpochScheduleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHighestRecordedEpoch(_) => {
                    <getHighestRecordedEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPostedRoots(_) => {
                    <getPostedRootsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRootSenders(_) => {
                    <getRootSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::postRoot(_) => <postRootCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::recordEpochRoot(_) => {
                    <recordEpochRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeEpochLength(_) => {
                    <removeEpochLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEpochLength(_) => {
                    <setEpochLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setEpochSchedule(_) => {
                    <setEpochScheduleCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<CiphertextRootsCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn recordEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <recordEpochRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::recordEpochRoot)
                    }
                    recordEpochRoot
                },
                {
                    fn postRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <postRootCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CiphertextRootsCalls::postRoot)
                    }
                    postRoot
                },
                {
                    fn setEpochLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <setEpochLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::setEpochLength)
                    }
                    setEpochLength
                },
                {
                    fn getEpochBlocks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochBlocks)
                    }
                    getEpochBlocks
                },
                {
                    fn getHighestRecordedEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getHighestRecordedEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getHighestRecordedEpoch)
                    }
                    getHighestRecordedEpoch
                },
                {
                    fn removeEpochLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <removeEpochLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::removeEpochLength)
                    }
                    removeEpochLength
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getEpochSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochScheduleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochSchedule)
                    }
                    getEpochSchedule
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn getRootSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getRootSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getRootSenders)
                    }
                    getRootSenders
                },
                {
                    fn getPostedRoots(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getPostedRootsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getPostedRoots)
                    }
                    getPostedRoots
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn setEpochSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <setEpochScheduleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::setEpochSchedule)
                    }
                    setEpochSchedule
                },
                {
                    fn getEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochRoot)
                    }
                    getEpochRoot
                },
                {
                    fn getEpochOfBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochOfBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochOfBlock)
                    }
                    getEpochOfBlock
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
            ) -> alloy_sol_types::Result<CiphertextRootsCalls>] = &[
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn recordEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <recordEpochRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::recordEpochRoot)
                    }
                    recordEpochRoot
                },
                {
                    fn postRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <postRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::postRoot)
                    }
                    postRoot
                },
                {
                    fn setEpochLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <setEpochLengthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::setEpochLength)
                    }
                    setEpochLength
                },
                {
                    fn getEpochBlocks(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochBlocks)
                    }
                    getEpochBlocks
                },
                {
                    fn getHighestRecordedEpoch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getHighestRecordedEpochCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getHighestRecordedEpoch)
                    }
                    getHighestRecordedEpoch
                },
                {
                    fn removeEpochLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <removeEpochLengthCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::removeEpochLength)
                    }
                    removeEpochLength
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getEpochSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochScheduleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochSchedule)
                    }
                    getEpochSchedule
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn getRootSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getRootSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getRootSenders)
                    }
                    getRootSenders
                },
                {
                    fn getPostedRoots(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getPostedRootsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getPostedRoots)
                    }
                    getPostedRoots
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn setEpochSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <setEpochScheduleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::setEpochSchedule)
                    }
                    setEpochSchedule
                },
                {
                    fn getEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochRoot)
                    }
                    getEpochRoot
                },
                {
                    fn getEpochOfBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <getEpochOfBlockCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::getEpochOfBlock)
                    }
                    getEpochOfBlock
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
                Self::getEpochBlocks(inner) => {
                    <getEpochBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getEpochOfBlock(inner) => {
                    <getEpochOfBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getEpochRoot(inner) => {
                    <getEpochRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getEpochSchedule(inner) => {
                    <getEpochScheduleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHighestRecordedEpoch(inner) => {
                    <getHighestRecordedEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPostedRoots(inner) => {
                    <getPostedRootsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRootSenders(inner) => {
                    <getRootSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::postRoot(inner) => {
                    <postRootCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proxiableUUID(inner) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::recordEpochRoot(inner) => {
                    <recordEpochRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeEpochLength(inner) => {
                    <removeEpochLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEpochLength(inner) => {
                    <setEpochLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setEpochSchedule(inner) => {
                    <setEpochScheduleCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getEpochBlocks(inner) => {
                    <getEpochBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getEpochOfBlock(inner) => {
                    <getEpochOfBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getEpochRoot(inner) => {
                    <getEpochRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getEpochSchedule(inner) => {
                    <getEpochScheduleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHighestRecordedEpoch(inner) => {
                    <getHighestRecordedEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPostedRoots(inner) => {
                    <getPostedRootsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRootSenders(inner) => {
                    <getRootSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::postRoot(inner) => {
                    <postRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::recordEpochRoot(inner) => {
                    <recordEpochRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeEpochLength(inner) => {
                    <removeEpochLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEpochLength(inner) => {
                    <setEpochLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setEpochSchedule(inner) => {
                    <setEpochScheduleCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`CiphertextRoots`](self) custom errors.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum CiphertextRootsErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        BlockBeforeStartBlock(BlockBeforeStartBlock),
        #[allow(missing_docs)]
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        #[allow(missing_docs)]
        ERC1967NonPayable(ERC1967NonPayable),
        #[allow(missing_docs)]
        EpochLengthNotFound(EpochLengthNotFound),
        #[allow(missing_docs)]
        EpochRootAlreadyRecorded(EpochRootAlreadyRecorded),
        #[allow(missing_docs)]
        EpochScheduleAlreadySet(EpochScheduleAlreadySet),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        FromEpochTooLow(FromEpochTooLow),
        #[allow(missing_docs)]
        InvalidChainId(InvalidChainId),
        #[allow(missing_docs)]
        InvalidEpochLength(InvalidEpochLength),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        NoEpochRoot(NoEpochRoot),
        #[allow(missing_docs)]
        NoEpochRootRecorded(NoEpochRootRecorded),
        #[allow(missing_docs)]
        NoEpochSchedule(NoEpochSchedule),
        #[allow(missing_docs)]
        NoRootReachesThreshold(NoRootReachesThreshold),
        #[allow(missing_docs)]
        NotCoprocessorTxSender(NotCoprocessorTxSender),
        #[allow(missing_docs)]
        NotHostOwner(NotHostOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        RootAlreadyPosted(RootAlreadyPosted),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
    }
    impl CiphertextRootsErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [33u8, 191u8, 218u8, 16u8],
            [41u8, 203u8, 221u8, 35u8],
            [43u8, 0u8, 197u8, 150u8],
            [60u8, 93u8, 141u8, 244u8],
            [76u8, 156u8, 140u8, 227u8],
            [82u8, 215u8, 37u8, 245u8],
            [97u8, 158u8, 78u8, 96u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 94u8, 111u8, 20u8],
            [122u8, 71u8, 201u8, 162u8],
            [136u8, 172u8, 8u8, 151u8],
            [150u8, 47u8, 67u8, 191u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [188u8, 89u8, 231u8, 151u8],
            [195u8, 33u8, 111u8, 25u8],
            [203u8, 171u8, 212u8, 126u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [249u8, 46u8, 232u8, 169u8],
            [253u8, 112u8, 59u8, 24u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(NotHostOwner),
            ::core::stringify!(NoEpochSchedule),
            ::core::stringify!(NoEpochRoot),
            ::core::stringify!(EpochScheduleAlreadySet),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(NotCoprocessorTxSender),
            ::core::stringify!(RootAlreadyPosted),
            ::core::stringify!(NotInitializingFromEmptyProxy),
            ::core::stringify!(BlockBeforeStartBlock),
            ::core::stringify!(InvalidChainId),
            ::core::stringify!(InvalidEpochLength),
            ::core::stringify!(NoRootReachesThreshold),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(EpochLengthNotFound),
            ::core::stringify!(FromEpochTooLow),
            ::core::stringify!(NoEpochRootRecorded),
            ::core::stringify!(FailedCall),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(EpochRootAlreadyRecorded),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <NotHostOwner as alloy_sol_types::SolError>::SIGNATURE,
            <NoEpochSchedule as alloy_sol_types::SolError>::SIGNATURE,
            <NoEpochRoot as alloy_sol_types::SolError>::SIGNATURE,
            <EpochScheduleAlreadySet as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <NotCoprocessorTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <RootAlreadyPosted as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SIGNATURE,
            <BlockBeforeStartBlock as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidChainId as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidEpochLength as alloy_sol_types::SolError>::SIGNATURE,
            <NoRootReachesThreshold as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <EpochLengthNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <FromEpochTooLow as alloy_sol_types::SolError>::SIGNATURE,
            <NoEpochRootRecorded as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidInitialization as alloy_sol_types::SolError>::SIGNATURE,
            <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for CiphertextRootsErrors {
        const NAME: &'static str = "CiphertextRootsErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BlockBeforeStartBlock(_) => {
                    <BlockBeforeStartBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967InvalidImplementation(_) => {
                    <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ERC1967NonPayable(_) => {
                    <ERC1967NonPayable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochLengthNotFound(_) => {
                    <EpochLengthNotFound as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochRootAlreadyRecorded(_) => {
                    <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochScheduleAlreadySet(_) => {
                    <EpochScheduleAlreadySet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FromEpochTooLow(_) => {
                    <FromEpochTooLow as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidChainId(_) => {
                    <InvalidChainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidEpochLength(_) => {
                    <InvalidEpochLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEpochRoot(_) => {
                    <NoEpochRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEpochRootRecorded(_) => {
                    <NoEpochRootRecorded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEpochSchedule(_) => {
                    <NoEpochSchedule as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoRootReachesThreshold(_) => {
                    <NoRootReachesThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotCoprocessorTxSender(_) => {
                    <NotCoprocessorTxSender as alloy_sol_types::SolError>::SELECTOR
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
                Self::RootAlreadyPosted(_) => {
                    <RootAlreadyPosted as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<CiphertextRootsErrors>] = &[
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CiphertextRootsErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn NoEpochSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochSchedule as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoEpochSchedule)
                    }
                    NoEpochSchedule
                },
                {
                    fn NoEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochRoot as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CiphertextRootsErrors::NoEpochRoot)
                    }
                    NoEpochRoot
                },
                {
                    fn EpochScheduleAlreadySet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochScheduleAlreadySet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochScheduleAlreadySet)
                    }
                    EpochScheduleAlreadySet
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
                },
                {
                    fn RootAlreadyPosted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <RootAlreadyPosted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::RootAlreadyPosted)
                    }
                    RootAlreadyPosted
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn BlockBeforeStartBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <BlockBeforeStartBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::BlockBeforeStartBlock)
                    }
                    BlockBeforeStartBlock
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn InvalidEpochLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <InvalidEpochLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::InvalidEpochLength)
                    }
                    InvalidEpochLength
                },
                {
                    fn NoRootReachesThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoRootReachesThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoRootReachesThreshold)
                    }
                    NoRootReachesThreshold
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn EpochLengthNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochLengthNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochLengthNotFound)
                    }
                    EpochLengthNotFound
                },
                {
                    fn FromEpochTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <FromEpochTooLow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::FromEpochTooLow)
                    }
                    FromEpochTooLow
                },
                {
                    fn NoEpochRootRecorded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochRootRecorded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoEpochRootRecorded)
                    }
                    NoEpochRootRecorded
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CiphertextRootsErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn EpochRootAlreadyRecorded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochRootAlreadyRecorded)
                    }
                    EpochRootAlreadyRecorded
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
            ) -> alloy_sol_types::Result<CiphertextRootsErrors>] = &[
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn NoEpochSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochSchedule as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoEpochSchedule)
                    }
                    NoEpochSchedule
                },
                {
                    fn NoEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoEpochRoot)
                    }
                    NoEpochRoot
                },
                {
                    fn EpochScheduleAlreadySet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochScheduleAlreadySet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochScheduleAlreadySet)
                    }
                    EpochScheduleAlreadySet
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
                },
                {
                    fn RootAlreadyPosted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <RootAlreadyPosted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::RootAlreadyPosted)
                    }
                    RootAlreadyPosted
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn BlockBeforeStartBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <BlockBeforeStartBlock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::BlockBeforeStartBlock)
                    }
                    BlockBeforeStartBlock
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn InvalidEpochLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <InvalidEpochLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::InvalidEpochLength)
                    }
                    InvalidEpochLength
                },
                {
                    fn NoRootReachesThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoRootReachesThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoRootReachesThreshold)
                    }
                    NoRootReachesThreshold
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn EpochLengthNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochLengthNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochLengthNotFound)
                    }
                    EpochLengthNotFound
                },
                {
                    fn FromEpochTooLow(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <FromEpochTooLow as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::FromEpochTooLow)
                    }
                    FromEpochTooLow
                },
                {
                    fn NoEpochRootRecorded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochRootRecorded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoEpochRootRecorded)
                    }
                    NoEpochRootRecorded
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn EpochRootAlreadyRecorded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochRootAlreadyRecorded)
                    }
                    EpochRootAlreadyRecorded
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
                Self::BlockBeforeStartBlock(inner) => {
                    <BlockBeforeStartBlock as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EpochLengthNotFound(inner) => {
                    <EpochLengthNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EpochRootAlreadyRecorded(inner) => {
                    <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EpochScheduleAlreadySet(inner) => {
                    <EpochScheduleAlreadySet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FromEpochTooLow(inner) => {
                    <FromEpochTooLow as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidEpochLength(inner) => {
                    <InvalidEpochLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoEpochRoot(inner) => {
                    <NoEpochRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NoEpochRootRecorded(inner) => {
                    <NoEpochRootRecorded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoEpochSchedule(inner) => {
                    <NoEpochSchedule as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoRootReachesThreshold(inner) => {
                    <NoRootReachesThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotCoprocessorTxSender(inner) => {
                    <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::RootAlreadyPosted(inner) => {
                    <RootAlreadyPosted as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::BlockBeforeStartBlock(inner) => {
                    <BlockBeforeStartBlock as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EpochLengthNotFound(inner) => {
                    <EpochLengthNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EpochRootAlreadyRecorded(inner) => {
                    <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EpochScheduleAlreadySet(inner) => {
                    <EpochScheduleAlreadySet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::FromEpochTooLow(inner) => {
                    <FromEpochTooLow as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidEpochLength(inner) => {
                    <InvalidEpochLength as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NoEpochRoot(inner) => {
                    <NoEpochRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoEpochRootRecorded(inner) => {
                    <NoEpochRootRecorded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoEpochSchedule(inner) => {
                    <NoEpochSchedule as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoRootReachesThreshold(inner) => {
                    <NoRootReachesThreshold as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::RootAlreadyPosted(inner) => {
                    <RootAlreadyPosted as alloy_sol_types::SolError>::abi_encode_raw(
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
    ///Container for all the [`CiphertextRoots`](self) events.
    #[derive(Clone)]
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum CiphertextRootsEvents {
        #[allow(missing_docs)]
        EpochLengthRemoved(EpochLengthRemoved),
        #[allow(missing_docs)]
        EpochLengthSet(EpochLengthSet),
        #[allow(missing_docs)]
        EpochRootRecorded(EpochRootRecorded),
        #[allow(missing_docs)]
        EpochScheduleSet(EpochScheduleSet),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        RootPosted(RootPosted),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    impl CiphertextRootsEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                52u8, 237u8, 118u8, 127u8, 165u8, 56u8, 10u8, 58u8, 224u8, 171u8, 54u8,
                6u8, 15u8, 112u8, 184u8, 219u8, 117u8, 2u8, 65u8, 32u8, 103u8, 150u8,
                38u8, 77u8, 196u8, 200u8, 217u8, 10u8, 123u8, 129u8, 26u8, 102u8,
            ],
            [
                59u8, 58u8, 197u8, 63u8, 65u8, 109u8, 154u8, 208u8, 78u8, 37u8, 143u8,
                100u8, 125u8, 33u8, 213u8, 48u8, 136u8, 247u8, 137u8, 210u8, 73u8, 42u8,
                191u8, 145u8, 8u8, 228u8, 9u8, 241u8, 1u8, 143u8, 59u8, 234u8,
            ],
            [
                123u8, 154u8, 115u8, 164u8, 103u8, 172u8, 219u8, 39u8, 238u8, 227u8,
                94u8, 136u8, 14u8, 199u8, 117u8, 219u8, 134u8, 146u8, 92u8, 216u8, 43u8,
                148u8, 203u8, 76u8, 182u8, 221u8, 247u8, 14u8, 181u8, 204u8, 165u8, 219u8,
            ],
            [
                150u8, 137u8, 217u8, 54u8, 96u8, 158u8, 238u8, 208u8, 64u8, 27u8, 79u8,
                236u8, 206u8, 207u8, 234u8, 129u8, 33u8, 109u8, 232u8, 255u8, 2u8, 188u8,
                142u8, 78u8, 68u8, 208u8, 205u8, 159u8, 103u8, 44u8, 0u8, 56u8,
            ],
            [
                165u8, 36u8, 80u8, 172u8, 10u8, 40u8, 93u8, 250u8, 251u8, 203u8, 143u8,
                41u8, 167u8, 148u8, 50u8, 175u8, 82u8, 121u8, 53u8, 180u8, 104u8, 165u8,
                26u8, 215u8, 236u8, 90u8, 208u8, 12u8, 220u8, 196u8, 176u8, 196u8,
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
            ::core::stringify!(RootPosted),
            ::core::stringify!(EpochScheduleSet),
            ::core::stringify!(EpochLengthRemoved),
            ::core::stringify!(EpochRootRecorded),
            ::core::stringify!(EpochLengthSet),
            ::core::stringify!(Upgraded),
            ::core::stringify!(Initialized),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <RootPosted as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochScheduleSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochLengthRemoved as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochRootRecorded as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochLengthSet as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for CiphertextRootsEvents {
        const NAME: &'static str = "CiphertextRootsEvents";
        const COUNT: usize = 7usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <EpochLengthRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EpochLengthRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochLengthRemoved)
                }
                Some(<EpochLengthSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EpochLengthSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochLengthSet)
                }
                Some(
                    <EpochRootRecorded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EpochRootRecorded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochRootRecorded)
                }
                Some(<EpochScheduleSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EpochScheduleSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochScheduleSet)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(<RootPosted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RootPosted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::RootPosted)
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
    impl alloy_sol_types::private::IntoLogData for CiphertextRootsEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EpochLengthRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EpochLengthSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EpochRootRecorded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EpochScheduleSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RootPosted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EpochLengthRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EpochLengthSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EpochRootRecorded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EpochScheduleSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RootPosted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`CiphertextRoots`](self) contract instance.

See the [wrapper's documentation](`CiphertextRootsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        __provider: P,
    ) -> CiphertextRootsInstance<P, N> {
        CiphertextRootsInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<CiphertextRootsInstance<P, N>>,
    > {
        CiphertextRootsInstance::<P, N>::deploy(__provider)
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
        CiphertextRootsInstance::<P, N>::deploy_builder(__provider)
    }
    /**A [`CiphertextRoots`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`CiphertextRoots`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CiphertextRootsInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for CiphertextRootsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CiphertextRootsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CiphertextRootsInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`CiphertextRoots`](self) contract instance.

See the [wrapper's documentation](`CiphertextRootsInstance`) for more details.*/
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
        ) -> alloy_contract::Result<CiphertextRootsInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> CiphertextRootsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CiphertextRootsInstance<P, N> {
            CiphertextRootsInstance {
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
    > CiphertextRootsInstance<P, N> {
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
        ///Creates a new call builder for the [`getEpochBlocks`] function.
        pub fn getEpochBlocks(
            &self,
            chainId: u64,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getEpochBlocksCall, N> {
            self.call_builder(
                &getEpochBlocksCall {
                    chainId,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`getEpochOfBlock`] function.
        pub fn getEpochOfBlock(
            &self,
            chainId: u64,
            blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getEpochOfBlockCall, N> {
            self.call_builder(
                &getEpochOfBlockCall {
                    chainId,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getEpochRoot`] function.
        pub fn getEpochRoot(
            &self,
            chainId: u64,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getEpochRootCall, N> {
            self.call_builder(&getEpochRootCall { chainId, epoch })
        }
        ///Creates a new call builder for the [`getEpochSchedule`] function.
        pub fn getEpochSchedule(
            &self,
            chainId: u64,
        ) -> alloy_contract::SolCallBuilder<&P, getEpochScheduleCall, N> {
            self.call_builder(&getEpochScheduleCall { chainId })
        }
        ///Creates a new call builder for the [`getHighestRecordedEpoch`] function.
        pub fn getHighestRecordedEpoch(
            &self,
            chainId: u64,
        ) -> alloy_contract::SolCallBuilder<&P, getHighestRecordedEpochCall, N> {
            self.call_builder(
                &getHighestRecordedEpochCall {
                    chainId,
                },
            )
        }
        ///Creates a new call builder for the [`getPostedRoots`] function.
        pub fn getPostedRoots(
            &self,
            chainId: u64,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getPostedRootsCall, N> {
            self.call_builder(
                &getPostedRootsCall {
                    chainId,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`getRootSenders`] function.
        pub fn getRootSenders(
            &self,
            chainId: u64,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, getRootSendersCall, N> {
            self.call_builder(
                &getRootSendersCall {
                    chainId,
                    epoch,
                    root,
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
        ) -> alloy_contract::SolCallBuilder<&P, initializeFromEmptyProxyCall, N> {
            self.call_builder(&initializeFromEmptyProxyCall)
        }
        ///Creates a new call builder for the [`postRoot`] function.
        pub fn postRoot(
            &self,
            chainId: u64,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, postRootCall, N> {
            self.call_builder(
                &postRootCall {
                    chainId,
                    epoch,
                    root,
                },
            )
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`recordEpochRoot`] function.
        pub fn recordEpochRoot(
            &self,
            chainId: u64,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, recordEpochRootCall, N> {
            self.call_builder(
                &recordEpochRootCall {
                    chainId,
                    epoch,
                },
            )
        }
        ///Creates a new call builder for the [`removeEpochLength`] function.
        pub fn removeEpochLength(
            &self,
            chainId: u64,
            fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, removeEpochLengthCall, N> {
            self.call_builder(
                &removeEpochLengthCall {
                    chainId,
                    fromEpoch,
                },
            )
        }
        ///Creates a new call builder for the [`setEpochLength`] function.
        pub fn setEpochLength(
            &self,
            chainId: u64,
            fromEpoch: alloy::sol_types::private::primitives::aliases::U256,
            epochLength: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setEpochLengthCall, N> {
            self.call_builder(
                &setEpochLengthCall {
                    chainId,
                    fromEpoch,
                    epochLength,
                },
            )
        }
        ///Creates a new call builder for the [`setEpochSchedule`] function.
        pub fn setEpochSchedule(
            &self,
            chainId: u64,
            startBlock: alloy::sol_types::private::primitives::aliases::U256,
            epochLength: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, setEpochScheduleCall, N> {
            self.call_builder(
                &setEpochScheduleCall {
                    chainId,
                    startBlock,
                    epochLength,
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
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > CiphertextRootsInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`EpochLengthRemoved`] event.
        pub fn EpochLengthRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochLengthRemoved, N> {
            self.event_filter::<EpochLengthRemoved>()
        }
        ///Creates a new event filter for the [`EpochLengthSet`] event.
        pub fn EpochLengthSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochLengthSet, N> {
            self.event_filter::<EpochLengthSet>()
        }
        ///Creates a new event filter for the [`EpochRootRecorded`] event.
        pub fn EpochRootRecorded_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochRootRecorded, N> {
            self.event_filter::<EpochRootRecorded>()
        }
        ///Creates a new event filter for the [`EpochScheduleSet`] event.
        pub fn EpochScheduleSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochScheduleSet, N> {
            self.event_filter::<EpochScheduleSet>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`RootPosted`] event.
        pub fn RootPosted_filter(&self) -> alloy_contract::Event<&P, RootPosted, N> {
            self.event_filter::<RootPosted>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
