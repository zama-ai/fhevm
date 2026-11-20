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
    error EpochLengthNull();
    error EpochRootAlreadyPosted(uint64 chainId, uint256 epoch, address sender);
    error EpochRootAlreadyRecorded(uint64 chainId, uint256 epoch);
    error EpochRootNull();
    error EpochScheduleAlreadySet(uint64 chainId);
    error FailedCall();
    error FromEpochTooLow(uint64 chainId, uint256 fromEpoch, uint256 minFromEpoch);
    error HostChainNullId();
    error InvalidInitialization();
    error NoEpochRoot(uint64 chainId, uint256 epoch);
    error NoEpochRootReachesThreshold(uint64 chainId, uint256 epoch);
    error NoEpochRootRecorded(uint64 chainId);
    error NoEpochSchedule(uint64 chainId);
    error NotCoprocessorTxSender(address sender);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);

    event EpochLengthRemoved(uint64 indexed chainId, uint256 fromEpoch);
    event EpochLengthSet(uint64 indexed chainId, uint256 fromEpoch, uint256 epochLength);
    event EpochRootPosted(uint64 indexed chainId, uint256 indexed epoch, address indexed sender, bytes32 root);
    event EpochRootRecorded(uint64 indexed chainId, uint256 indexed epoch, bytes32 root, uint256 coprocessorContextId);
    event EpochScheduleSet(uint64 indexed chainId, uint256 startBlock, uint256 epochLength);
    event Initialized(uint64 version);
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
    function postEpochRoot(uint64 chainId, uint256 epoch, bytes32 root) external;
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
    "name": "postEpochRoot",
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
    "name": "EpochRootPosted",
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
    "name": "EpochLengthNull",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EpochRootAlreadyPosted",
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
    "name": "EpochRootNull",
    "inputs": []
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
    "name": "HostChainNullId",
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
    "name": "NoEpochRootReachesThreshold",
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
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b608051612713620001005f395f818161175d01528181611786015261196e01526127135ff3fe6080604052600436106100fa575f3560e01c80634f1ef28611610092578063ad3cb1cc11610062578063ad3cb1cc146102a7578063de93c514146102d7578063e7069431146102f6578063e8ad358714610315578063f5178ccf14610334575f80fd5b80634f1ef2861461022857806352d1902d1461023b5780635757a3231461024f578063919406081461027b575f80fd5b80633662bd62116100cd5780633662bd621461019c57806337a08d12146101c957806339f73810146101e8578063459500e2146101fc575f80fd5b80630d8e6e2c146100fe578063114bf5711461012857806319ba5fcc146101495780632949ca4714610168575b5f80fd5b348015610109575f80fd5b50610112610353565b60405161011f91906122d5565b60405180910390f35b348015610133575f80fd5b5061014761014236600461231d565b6103c0565b005b348015610154575f80fd5b50610147610163366004612345565b61045e565b348015610173575f80fd5b5061018761018236600461231d565b610668565b6040805192835260208301919091520161011f565b3480156101a7575f80fd5b506101bb6101b6366004612375565b61076e565b60405190815260200161011f565b3480156101d4575f80fd5b506101476101e336600461231d565b6107bb565b3480156101f3575f80fd5b50610147610a34565b348015610207575f80fd5b5061021b610216366004612375565b610b55565b60405161011f919061238e565b610147610236366004612419565b610bfd565b348015610246575f80fd5b506101bb610c1c565b34801561025a575f80fd5b5061026e610269366004612345565b610c38565b60405161011f91906124d6565b348015610286575f80fd5b5061029a61029536600461231d565b610d0f565b60405161011f9190612522565b3480156102b2575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156102e2575f80fd5b506101476102f1366004612345565b610d7d565b348015610301575f80fd5b50610147610310366004612345565b610f77565b348015610320575f80fd5b506101bb61032f36600461231d565b611124565b34801561033f575f80fd5b506101bb61034e36600461231d565b611176565b60606040518060400160405280600f81526020016e43697068657274657874526f6f747360881b8152506103865f611292565b6103906001611292565b6103995f611292565b6040516020016103ac9493929190612559565b604051602081830303815290604052905090565b5f6103ca83611321565b5f83815260048201602052604090206002810154919250901561041757604051631fae076360e31b81526001600160401b0385166004820152602481018490526044015b60405180910390fd5b610429848461042461138c565b611628565b61045857604051637cbde47960e01b81526001600160401b03851660048201526024810184905260440161040e565b50505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156104ae573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104d291906125d6565b6001600160a01b0316336001600160a01b0316146105055760405163021bfda160e41b815233600482015260240161040e565b805f036105255760405163c784f54b60e01b815260040160405180910390fd5b5f61052f84611321565b60018082018054929350915f9183916105489190612605565b8154811061055857610558612618565b905f5260205f2090600202015f01546001610573919061262c565b6002840154909150610100900460ff168015610593575080836003015410155b156105ab5760038301546105a890600161262c565b90505b808510156105e55760405163c3216f1960e01b81526001600160401b0387166004820152602481018690526044810182905260640161040e565b60408051808201825286815260208082018781528554600180820188555f88815284902094516002909202909401908155905192019190915581518781529081018690526001600160401b038816917fa52450ac0a285dfafbcb8f29a79432af527935b468a51ad7ec5ad00cdcc4b0c491015b60405180910390a2505050505050565b5f805f61067485611321565b5f0190505f8160010190505f815f8154811061069257610692612618565b5f91825260209091208454600290920201915060015b835481101561071d575f8482815481106106c4576106c4612618565b905f5260205f2090600202019050805f01548910156106e3575061071d565b835481545f916106f291612605565b9050846001015481610704919061263f565b61070e908561262c565b919450909250506001016106a8565b506001820154825461072f9089612605565b610739919061263f565b610743908261262c565b95506001826001015487610757919061262c565b6107619190612605565b9450505050509250929050565b5f8061077983611321565b6002810154909150610100900460ff166107b1576040516365d5ea3f60e11b81526001600160401b038416600482015260240161040e565b6003015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561080b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082f91906125d6565b6001600160a01b0316336001600160a01b0316146108625760405163021bfda160e41b815233600482015260240161040e565b5f61086c83611321565b6002810154909150610100900460ff16801561088c575080600301548211155b156108d6578282826003015460016108a4919061262c565b60405163c3216f1960e01b81526001600160401b0390931660048401526024830191909152604482015260640161040e565b6001808201805490915b818110801561090f5750848382815481106108fd576108fd612618565b905f5260205f2090600202015f015414155b15610926578061091e81612656565b9150506108e0565b8181036109585760405163bc59e79760e01b81526001600160401b03871660048201526024810186905260440161040e565b805b610965600184612605565b8110156109ca578361097882600161262c565b8154811061098857610988612618565b905f5260205f2090600202018482815481106109a6576109a6612618565b5f91825260209091208254600290920201908155600191820154908201550161095a565b50828054806109db576109db61266e565b5f8281526020812060025f19909301928302018181556001015590556040516001600160401b038716907f7b9a73a467acdb27eee35e880ec775db86925cd82b94cb4cb6ddf70eb5cca5db906106589088815260200190565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b0316600114610a8857604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460029190600160401b900460ff1680610ad1575080546001600160401b03808416911610155b15610aef5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1768ff0000000000000000191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b604080518082019091525f815260606020820152610b7282611321565b5f016040518060400160405290815f820154815260200160018201805480602002602001604051908101604052809291908181526020015f905b82821015610bef578382905f5260205f2090600202016040518060400160405290815f820154815260200160018201548152505081526020019060010190610bac565b505050915250909392505050565b610c05611752565b610c0e826117f8565b610c1882826118a2565b5050565b5f610c25611963565b505f805160206126f38339815191525b90565b60605f610c4485611321565b5f85815260049190910160205260408120915080610c6283866119ac565b9150915081610c84575050604080515f8152602081019091529150610d089050565b610d0283600101805480602002602001604051908101604052809291908181526020015f905b82821015610cf8575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101610caa565b50505050826119fe565b93505050505b9392505050565b6060610d1a83611321565b5f838152600491909101602090815260409182902080548351818402810184019094528084529091830182828015610d6f57602002820191905f5260205f20905b815481526020019060010190808311610d5b575b505050505090505b92915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610dcd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610df191906125d6565b6001600160a01b0316336001600160a01b031614610e245760405163021bfda160e41b815233600482015260240161040e565b826001600160401b03165f03610e4c576040516273401f60e71b815260040160405180910390fd5b805f03610e6c5760405163c784f54b60e01b815260040160405180910390fd5b6001600160401b0383165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff1615610ed357604051630f17637d60e21b81526001600160401b038516600482015260240161040e565b6002808201805460ff191660019081179091558483556040805180820182525f80825260208083018881528786018054808801825590845291909220925195029091019384555192909101919091555181906001600160401b038616907f3b3ac53f416d9ad04e258f647d21d53088f789d2492abf9108e409f1018f3bea90610f689087908790918252602082015260400190565b60405180910390a25050505050565b80610f9557604051637284928560e11b815260040160405180910390fd5b5f610f9f84611321565b5f8481526004820160205260409020909150610fbb8133611b33565b15610ff1576040516304f2eb0160e11b81526001600160401b03861660048201526024810185905233604482015260640161040e565b5f610ffa61138c565b90506110068133611b95565b611025576040516352d725f560e01b815233600482015260240161040e565b60028201545f9015158061103f575061103f878784611628565b90505f8061104d85886119ac565b915091508161106d575083546001810185555f8581526020902081018790555b604080518082018252338082526001600160601b0380851660208085019182526001808c01805491820181555f9081529190912094519151909216600160a01b026001600160a01b039190911617920191909155905189906001600160401b038c16907fac3e570e86481985ba4e8e32ee33ccb4cf7365bfccfd2e7e8b5a0535f960ea44906110ff908c815260200190565b60405180910390a48261111957611117898986611628565b505b505050505050505050565b5f8061112f84611321565b5f8481526004919091016020526040902060020154905080610d085760405163158062cb60e11b81526001600160401b03851660048201526024810184905260440161040e565b5f8061118184611321565b80549091508310156111b857604051631dd79bc560e21b81526001600160401b03851660048201526024810184905260440161040e565b5f8160010190505f815f815481106111d2576111d2612618565b5f91825260209091208454600290920201915060015b8354811015611261575f84828154811061120457611204612618565b5f91825260208220865460029092020180549093506112239190612605565b90505f856001015482611236919061263f565b611240908661262c565b9050808a101561125257505050611261565b919450909250506001016111e8565b5060018201546112718288612605565b61127b9190612682565b8254611287919061262c565b979650505050505050565b60605f61129e83611cbb565b60010190505f816001600160401b038111156112bc576112bc612405565b6040519080825280601f01601f1916602001820160405280156112e6576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a85049450846112f057509392505050565b6001600160401b0381165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff16611387576040516329cbdd2360e01b81526001600160401b038316600482015260240161040e565b919050565b6113bb6040518060a001604052805f81526020015f81526020015f151581526020015f81526020015f81525090565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b031663170a29816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561140b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061142f91906126a1565b808252604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa158015611481573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114a591906126a1565b8160600181815250507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166389090a446040518163ffffffff1660e01b8152600401602060405180830381865afa1580156114fe573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061152291906126a1565b60208201819052604051630e1887c960e01b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac90630e1887c990602401602060405180830381865afa158015611578573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061159c91906126b8565b1580156040830152610c35576020810151604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa1580156115fc573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061162091906126a1565b608082015290565b5f8061163385611321565b5f858152600491909101602052604090206001810154909150611657908490611d92565b611664575f915050610d08565b5f81600101805480602002602001604051908101604052809291908181526020015f905b828210156116d6575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101611688565b5050505090505f5b8254811015611746575f806116fc876116f786866119fe565b611dbe565b91509150811561173c5761172e8989875f01868154811061171f5761171f612618565b905f5260205f20015484611fdf565b600195505050505050610d08565b50506001016116de565b505f9695505050505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806117d857507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166117cc5f805160206126f3833981519152546001600160a01b031690565b6001600160a01b031614155b156117f65760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611848573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061186c91906125d6565b6001600160a01b0316336001600160a01b03161461189f5760405163021bfda160e41b815233600482015260240161040e565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156118fc575060408051601f3d908101601f191682019092526118f9918101906126a1565b60015b61192457604051634c9c8ce360e01b81526001600160a01b038316600482015260240161040e565b5f805160206126f3833981519152811461195457604051632a87526960e21b81526004810182905260240161040e565b61195e83836120e5565b505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146117f65760405163703e46dd60e11b815260040160405180910390fd5b5f805f5b84548110156119ef5783855f0182815481106119ce576119ce612618565b905f5260205f200154036119e7576001925090506119f7565b6001016119b0565b505f80915091505b9250929050565b60605f805b8451811015611a4f5783858281518110611a1f57611a1f612618565b6020026020010151602001516001600160601b031603611a475781611a4381612656565b9250505b600101611a03565b50806001600160401b03811115611a6857611a68612405565b604051908082528060200260200182016040528015611a91578160200160208202803683370190505b5091505f805b8551811015611b2a5784868281518110611ab357611ab3612618565b6020026020010151602001516001600160601b031603611b2257858181518110611adf57611adf612618565b60200260200101515f0151848381518110611afc57611afc612618565b6001600160a01b039092166020928302919091019091015281611b1e81612656565b9250505b600101611a97565b50505092915050565b5f805b6001840154811015611b8c57826001600160a01b0316846001018281548110611b6157611b61612618565b5f918252602090912001546001600160a01b031603611b84576001915050610d77565b600101611b36565b505f9392505050565b8151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201525f907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611bf7573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c1b91906126b8565b15611c2857506001610d77565b82604001518015610d0857506020830151604051633ac3ddd960e21b815260048101919091526001600160a01b03831660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611c97573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d0891906126b8565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b8310611cf95772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef81000000008310611d25576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310611d4357662386f26fc10000830492506010015b6305f5e1008310611d5b576305f5e100830492506008015b6127108310611d6f57612710830492506004015b60648310611d81576064830492506002015b600a8310610d775760010192915050565b5f82606001518210611da657506001610d77565b82604001518015610d08575050608090910151111590565b5f805f835190505f856060015182101590505f86604001518015611de6575086608001518310155b905081158015611df4575080155b15611e07575f80945094505050506119f7565b5f805f5b85811015611f7a575f898281518110611e2657611e26612618565b60200260200101519050858015611ebc57508a51604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611e98573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ebc91906126b8565b15611ecf5783611ecb81612656565b9450505b848015611f5e575060208b0151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611f3a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5e91906126b8565b15611f715782611f6d81612656565b9350505b50600101611e0b565b50838015611f8c575088606001518210155b15611fa457505086516001955093506119f792505050565b828015611fb5575088608001518110155b15611fcf57600189602001519650965050505050506119f7565b505f988998509650505050505050565b81611ffd57604051637284928560e11b815260040160405180910390fd5b5f61200785611321565b5f85815260048201602052604090206002810154919250901561204f57604051631fae076360e31b81526001600160401b03871660048201526024810186905260440161040e565b6002818101859055820154610100900460ff166120835760028201805461ff00191661010017905560038201859055612097565b816003015485111561209757600382018590555b604080518581526020810185905286916001600160401b038916917f9689d936609eeed0401b4feccecfea81216de8ff02bc8e4e44d0cd9f672c0038910160405180910390a3505050505050565b6120ee8261213a565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156121325761195e828261219d565b610c1861220f565b806001600160a01b03163b5f0361216f57604051634c9c8ce360e01b81526001600160a01b038216600482015260240161040e565b5f805160206126f383398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516121b991906126d7565b5f60405180830381855af49150503d805f81146121f1576040519150601f19603f3d011682016040523d82523d5f602084013e6121f6565b606091505b509150915061220685838361222e565b95945050505050565b34156117f65760405163b398979f60e01b815260040160405180910390fd5b6060826122435761223e8261228a565b610d08565b815115801561225a57506001600160a01b0384163b155b1561228357604051639996b31560e01b81526001600160a01b038516600482015260240161040e565b5080610d08565b80511561229a5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b838110156122cd5781810151838201526020016122b5565b50505f910152565b602081525f82518060208401526122f38160408501602087016122b3565b601f01601f19169190910160400192915050565b80356001600160401b0381168114611387575f80fd5b5f806040838503121561232e575f80fd5b61233783612307565b946020939093013593505050565b5f805f60608486031215612357575f80fd5b61236084612307565b95602085013595506040909401359392505050565b5f60208284031215612385575f80fd5b610d0882612307565b60208082528251828201528281015160408084018190528151606085018190525f9392830191849160808701905b808410156123e557845180518352860151868301529385019360019390930192908201906123bc565b50979650505050505050565b6001600160a01b038116811461189f575f80fd5b634e487b7160e01b5f52604160045260245ffd5b5f806040838503121561242a575f80fd5b8235612435816123f1565b915060208301356001600160401b0380821115612450575f80fd5b818501915085601f830112612463575f80fd5b81358181111561247557612475612405565b604051601f8201601f19908116603f0116810190838211818310171561249d5761249d612405565b816040528281528860208487010111156124b5575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b818110156125165783516001600160a01b0316835292840192918401916001016124f1565b50909695505050505050565b602080825282518282018190525f9190848201906040850190845b818110156125165783518352928401929184019160010161253d565b5f855161256a818460208a016122b3565b61103b60f11b9083019081528551612589816002840160208a016122b3565b808201915050601760f91b80600283015285516125ad816003850160208a016122b3565b600392019182015283516125c88160048401602088016122b3565b016004019695505050505050565b5f602082840312156125e6575f80fd5b8151610d08816123f1565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610d7757610d776125f1565b634e487b7160e01b5f52603260045260245ffd5b80820180821115610d7757610d776125f1565b8082028115828204841417610d7757610d776125f1565b5f60018201612667576126676125f1565b5060010190565b634e487b7160e01b5f52603160045260245ffd5b5f8261269c57634e487b7160e01b5f52601260045260245ffd5b500490565b5f602082840312156126b1575f80fd5b5051919050565b5f602082840312156126c8575f80fd5b81518015158114610d08575f80fd5b5f82516126e88184602087016122b3565b919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa'\x13b\0\x01\0_9_\x81\x81a\x17]\x01R\x81\x81a\x17\x86\x01Ra\x19n\x01Ra'\x13_\xF3\xFE`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80cO\x1E\xF2\x86\x11a\0\x92W\x80c\xAD<\xB1\xCC\x11a\0bW\x80c\xAD<\xB1\xCC\x14a\x02\xA7W\x80c\xDE\x93\xC5\x14\x14a\x02\xD7W\x80c\xE7\x06\x941\x14a\x02\xF6W\x80c\xE8\xAD5\x87\x14a\x03\x15W\x80c\xF5\x17\x8C\xCF\x14a\x034W_\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x02(W\x80cR\xD1\x90-\x14a\x02;W\x80cWW\xA3#\x14a\x02OW\x80c\x91\x94\x06\x08\x14a\x02{W_\x80\xFD[\x80c6b\xBDb\x11a\0\xCDW\x80c6b\xBDb\x14a\x01\x9CW\x80c7\xA0\x8D\x12\x14a\x01\xC9W\x80c9\xF78\x10\x14a\x01\xE8W\x80cE\x95\0\xE2\x14a\x01\xFCW_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c\x11K\xF5q\x14a\x01(W\x80c\x19\xBA_\xCC\x14a\x01IW\x80c)I\xCAG\x14a\x01hW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03SV[`@Qa\x01\x1F\x91\x90a\"\xD5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01Ga\x01B6`\x04a#\x1DV[a\x03\xC0V[\0[4\x80\x15a\x01TW_\x80\xFD[Pa\x01Ga\x01c6`\x04a#EV[a\x04^V[4\x80\x15a\x01sW_\x80\xFD[Pa\x01\x87a\x01\x826`\x04a#\x1DV[a\x06hV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1FV[4\x80\x15a\x01\xA7W_\x80\xFD[Pa\x01\xBBa\x01\xB66`\x04a#uV[a\x07nV[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xD4W_\x80\xFD[Pa\x01Ga\x01\xE36`\x04a#\x1DV[a\x07\xBBV[4\x80\x15a\x01\xF3W_\x80\xFD[Pa\x01Ga\n4V[4\x80\x15a\x02\x07W_\x80\xFD[Pa\x02\x1Ba\x02\x166`\x04a#uV[a\x0BUV[`@Qa\x01\x1F\x91\x90a#\x8EV[a\x01Ga\x0266`\x04a$\x19V[a\x0B\xFDV[4\x80\x15a\x02FW_\x80\xFD[Pa\x01\xBBa\x0C\x1CV[4\x80\x15a\x02ZW_\x80\xFD[Pa\x02na\x02i6`\x04a#EV[a\x0C8V[`@Qa\x01\x1F\x91\x90a$\xD6V[4\x80\x15a\x02\x86W_\x80\xFD[Pa\x02\x9Aa\x02\x956`\x04a#\x1DV[a\r\x0FV[`@Qa\x01\x1F\x91\x90a%\"V[4\x80\x15a\x02\xB2W_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x01Ga\x02\xF16`\x04a#EV[a\r}V[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x01Ga\x03\x106`\x04a#EV[a\x0FwV[4\x80\x15a\x03 W_\x80\xFD[Pa\x01\xBBa\x03/6`\x04a#\x1DV[a\x11$V[4\x80\x15a\x03?W_\x80\xFD[Pa\x01\xBBa\x03N6`\x04a#\x1DV[a\x11vV[```@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nCiphertextRoots`\x88\x1B\x81RPa\x03\x86_a\x12\x92V[a\x03\x90`\x01a\x12\x92V[a\x03\x99_a\x12\x92V[`@Q` \x01a\x03\xAC\x94\x93\x92\x91\x90a%YV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_a\x03\xCA\x83a\x13!V[_\x83\x81R`\x04\x82\x01` R`@\x90 `\x02\x81\x01T\x91\x92P\x90\x15a\x04\x17W`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x04)\x84\x84a\x04$a\x13\x8CV[a\x16(V[a\x04XW`@Qc|\xBD\xE4y`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x0EV[PPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xAEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD2\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\x05W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[\x80_\x03a\x05%W`@Qc\xC7\x84\xF5K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x05/\x84a\x13!V[`\x01\x80\x82\x01\x80T\x92\x93P\x91_\x91\x83\x91a\x05H\x91\x90a&\x05V[\x81T\x81\x10a\x05XWa\x05Xa&\x18V[\x90_R` _ \x90`\x02\x02\x01_\x01T`\x01a\x05s\x91\x90a&,V[`\x02\x84\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16\x80\x15a\x05\x93WP\x80\x83`\x03\x01T\x10\x15[\x15a\x05\xABW`\x03\x83\x01Ta\x05\xA8\x90`\x01a&,V[\x90P[\x80\x85\x10\x15a\x05\xE5W`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x82\x90R`d\x01a\x04\x0EV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R\x85T`\x01\x80\x82\x01\x88U_\x88\x81R\x84\x90 \x94Q`\x02\x90\x92\x02\x90\x94\x01\x90\x81U\x90Q\x92\x01\x91\x90\x91U\x81Q\x87\x81R\x90\x81\x01\x86\x90R`\x01`\x01`@\x1B\x03\x88\x16\x91\x7F\xA5$P\xAC\n(]\xFA\xFB\xCB\x8F)\xA7\x942\xAFRy5\xB4h\xA5\x1A\xD7\xECZ\xD0\x0C\xDC\xC4\xB0\xC4\x91\x01[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80_a\x06t\x85a\x13!V[_\x01\x90P_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x06\x92Wa\x06\x92a&\x18V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x07\x1DW_\x84\x82\x81T\x81\x10a\x06\xC4Wa\x06\xC4a&\x18V[\x90_R` _ \x90`\x02\x02\x01\x90P\x80_\x01T\x89\x10\x15a\x06\xE3WPa\x07\x1DV[\x83T\x81T_\x91a\x06\xF2\x91a&\x05V[\x90P\x84`\x01\x01T\x81a\x07\x04\x91\x90a&?V[a\x07\x0E\x90\x85a&,V[\x91\x94P\x90\x92PP`\x01\x01a\x06\xA8V[P`\x01\x82\x01T\x82Ta\x07/\x90\x89a&\x05V[a\x079\x91\x90a&?V[a\x07C\x90\x82a&,V[\x95P`\x01\x82`\x01\x01T\x87a\x07W\x91\x90a&,V[a\x07a\x91\x90a&\x05V[\x94PPPPP\x92P\x92\x90PV[_\x80a\x07y\x83a\x13!V[`\x02\x81\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16a\x07\xB1W`@Qce\xD5\xEA?`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x04\x0EV[`\x03\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x0BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08/\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08bW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[_a\x08l\x83a\x13!V[`\x02\x81\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16\x80\x15a\x08\x8CWP\x80`\x03\x01T\x82\x11\x15[\x15a\x08\xD6W\x82\x82\x82`\x03\x01T`\x01a\x08\xA4\x91\x90a&,V[`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`D\x82\x01R`d\x01a\x04\x0EV[`\x01\x80\x82\x01\x80T\x90\x91[\x81\x81\x10\x80\x15a\t\x0FWP\x84\x83\x82\x81T\x81\x10a\x08\xFDWa\x08\xFDa&\x18V[\x90_R` _ \x90`\x02\x02\x01_\x01T\x14\x15[\x15a\t&W\x80a\t\x1E\x81a&VV[\x91PPa\x08\xE0V[\x81\x81\x03a\tXW`@Qc\xBCY\xE7\x97`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x0EV[\x80[a\te`\x01\x84a&\x05V[\x81\x10\x15a\t\xCAW\x83a\tx\x82`\x01a&,V[\x81T\x81\x10a\t\x88Wa\t\x88a&\x18V[\x90_R` _ \x90`\x02\x02\x01\x84\x82\x81T\x81\x10a\t\xA6Wa\t\xA6a&\x18V[_\x91\x82R` \x90\x91 \x82T`\x02\x90\x92\x02\x01\x90\x81U`\x01\x91\x82\x01T\x90\x82\x01U\x01a\tZV[P\x82\x80T\x80a\t\xDBWa\t\xDBa&nV[_\x82\x81R` \x81 `\x02_\x19\x90\x93\x01\x92\x83\x02\x01\x81\x81U`\x01\x01U\x90U`@Q`\x01`\x01`@\x1B\x03\x87\x16\x90\x7F{\x9As\xA4g\xAC\xDB'\xEE\xE3^\x88\x0E\xC7u\xDB\x86\x92\\\xD8+\x94\xCBL\xB6\xDD\xF7\x0E\xB5\xCC\xA5\xDB\x90a\x06X\x90\x88\x81R` \x01\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\n\x88W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x02\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\n\xD1WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\n\xEFW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17h\xFF\0\0\0\0\0\0\0\0\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01Ra\x0Br\x82a\x13!V[_\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0B\xEFW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B\xACV[PPP\x91RP\x90\x93\x92PPPV[a\x0C\x05a\x17RV[a\x0C\x0E\x82a\x17\xF8V[a\x0C\x18\x82\x82a\x18\xA2V[PPV[_a\x0C%a\x19cV[P_\x80Q` a&\xF3\x839\x81Q\x91R[\x90V[``_a\x0CD\x85a\x13!V[_\x85\x81R`\x04\x91\x90\x91\x01` R`@\x81 \x91P\x80a\x0Cb\x83\x86a\x19\xACV[\x91P\x91P\x81a\x0C\x84WPP`@\x80Q_\x81R` \x81\x01\x90\x91R\x91Pa\r\x08\x90PV[a\r\x02\x83`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xF8W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0C\xAAV[PPPP\x82a\x19\xFEV[\x93PPPP[\x93\x92PPPV[``a\r\x1A\x83a\x13!V[_\x83\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R\x90\x91\x83\x01\x82\x82\x80\x15a\roW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r[W[PPPPP\x90P[\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF1\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E$W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[\x82`\x01`\x01`@\x1B\x03\x16_\x03a\x0ELW`@Qbs@\x1F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x03a\x0ElW`@Qc\xC7\x84\xF5K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16\x15a\x0E\xD3W`@Qc\x0F\x17c}`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x0EV[`\x02\x80\x82\x01\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x84\x83U`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x88\x81R\x87\x86\x01\x80T\x80\x88\x01\x82U\x90\x84R\x91\x90\x92 \x92Q\x95\x02\x90\x91\x01\x93\x84UQ\x92\x90\x91\x01\x91\x90\x91UQ\x81\x90`\x01`\x01`@\x1B\x03\x86\x16\x90\x7F;:\xC5?Am\x9A\xD0N%\x8Fd}!\xD50\x88\xF7\x89\xD2I*\xBF\x91\x08\xE4\t\xF1\x01\x8F;\xEA\x90a\x0Fh\x90\x87\x90\x87\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[\x80a\x0F\x95W`@Qcr\x84\x92\x85`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\x9F\x84a\x13!V[_\x84\x81R`\x04\x82\x01` R`@\x90 \x90\x91Pa\x0F\xBB\x813a\x1B3V[\x15a\x0F\xF1W`@Qc\x04\xF2\xEB\x01`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`$\x81\x01\x85\x90R3`D\x82\x01R`d\x01a\x04\x0EV[_a\x0F\xFAa\x13\x8CV[\x90Pa\x10\x06\x813a\x1B\x95V[a\x10%W`@QcR\xD7%\xF5`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[`\x02\x82\x01T_\x90\x15\x15\x80a\x10?WPa\x10?\x87\x87\x84a\x16(V[\x90P_\x80a\x10M\x85\x88a\x19\xACV[\x91P\x91P\x81a\x10mWP\x83T`\x01\x81\x01\x85U_\x85\x81R` \x90 \x81\x01\x87\x90U[`@\x80Q\x80\x82\x01\x82R3\x80\x82R`\x01`\x01``\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R`\x01\x80\x8C\x01\x80T\x91\x82\x01\x81U_\x90\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x92\x01\x91\x90\x91U\x90Q\x89\x90`\x01`\x01`@\x1B\x03\x8C\x16\x90\x7F\xAC>W\x0E\x86H\x19\x85\xBAN\x8E2\xEE3\xCC\xB4\xCFse\xBF\xCC\xFD.~\x8BZ\x055\xF9`\xEAD\x90a\x10\xFF\x90\x8C\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x82a\x11\x19Wa\x11\x17\x89\x89\x86a\x16(V[P[PPPPPPPPPV[_\x80a\x11/\x84a\x13!V[_\x84\x81R`\x04\x91\x90\x91\x01` R`@\x90 `\x02\x01T\x90P\x80a\r\x08W`@Qc\x15\x80b\xCB`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x0EV[_\x80a\x11\x81\x84a\x13!V[\x80T\x90\x91P\x83\x10\x15a\x11\xB8W`@Qc\x1D\xD7\x9B\xC5`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x0EV[_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x11\xD2Wa\x11\xD2a&\x18V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x12aW_\x84\x82\x81T\x81\x10a\x12\x04Wa\x12\x04a&\x18V[_\x91\x82R` \x82 \x86T`\x02\x90\x92\x02\x01\x80T\x90\x93Pa\x12#\x91\x90a&\x05V[\x90P_\x85`\x01\x01T\x82a\x126\x91\x90a&?V[a\x12@\x90\x86a&,V[\x90P\x80\x8A\x10\x15a\x12RWPPPa\x12aV[\x91\x94P\x90\x92PP`\x01\x01a\x11\xE8V[P`\x01\x82\x01Ta\x12q\x82\x88a&\x05V[a\x12{\x91\x90a&\x82V[\x82Ta\x12\x87\x91\x90a&,V[\x97\x96PPPPPPPV[``_a\x12\x9E\x83a\x1C\xBBV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xBCWa\x12\xBCa$\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\xE6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12\xF0WP\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16a\x13\x87W`@Qc)\xCB\xDD#`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x0EV[\x91\x90PV[a\x13\xBB`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81RP\x90V[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x17\n)\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x0BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14/\x91\x90a&\xA1V[\x80\x82R`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xA5\x91\x90a&\xA1V[\x81``\x01\x81\x81RPPsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x89\t\nD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xFEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\"\x91\x90a&\xA1V[` \x82\x01\x81\x90R`@Qc\x0E\x18\x87\xC9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\x0E\x18\x87\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15xW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9C\x91\x90a&\xB8V[\x15\x80\x15`@\x83\x01Ra\x0C5W` \x81\x01Q`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16 \x91\x90a&\xA1V[`\x80\x82\x01R\x90V[_\x80a\x163\x85a\x13!V[_\x85\x81R`\x04\x91\x90\x91\x01` R`@\x90 `\x01\x81\x01T\x90\x91Pa\x16W\x90\x84\x90a\x1D\x92V[a\x16dW_\x91PPa\r\x08V[_\x81`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16\xD6W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x16\x88V[PPPP\x90P_[\x82T\x81\x10\x15a\x17FW_\x80a\x16\xFC\x87a\x16\xF7\x86\x86a\x19\xFEV[a\x1D\xBEV[\x91P\x91P\x81\x15a\x17<Wa\x17.\x89\x89\x87_\x01\x86\x81T\x81\x10a\x17\x1FWa\x17\x1Fa&\x18V[\x90_R` _ \x01T\x84a\x1F\xDFV[`\x01\x95PPPPPPa\r\x08V[PP`\x01\x01a\x16\xDEV[P_\x96\x95PPPPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x17\xD8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x17\xCC_\x80Q` a&\xF3\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x17\xF6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18HW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18l\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x9FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18\xFCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18\xF9\x91\x81\x01\x90a&\xA1V[`\x01[a\x19$W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x0EV[_\x80Q` a&\xF3\x839\x81Q\x91R\x81\x14a\x19TW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\x0EV[a\x19^\x83\x83a \xE5V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\xF6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80_[\x84T\x81\x10\x15a\x19\xEFW\x83\x85_\x01\x82\x81T\x81\x10a\x19\xCEWa\x19\xCEa&\x18V[\x90_R` _ \x01T\x03a\x19\xE7W`\x01\x92P\x90Pa\x19\xF7V[`\x01\x01a\x19\xB0V[P_\x80\x91P\x91P[\x92P\x92\x90PV[``_\x80[\x84Q\x81\x10\x15a\x1AOW\x83\x85\x82\x81Q\x81\x10a\x1A\x1FWa\x1A\x1Fa&\x18V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x03a\x1AGW\x81a\x1AC\x81a&VV[\x92PP[`\x01\x01a\x1A\x03V[P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AhWa\x1Aha$\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x91W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P_\x80[\x85Q\x81\x10\x15a\x1B*W\x84\x86\x82\x81Q\x81\x10a\x1A\xB3Wa\x1A\xB3a&\x18V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x03a\x1B\"W\x85\x81\x81Q\x81\x10a\x1A\xDFWa\x1A\xDFa&\x18V[` \x02` \x01\x01Q_\x01Q\x84\x83\x81Q\x81\x10a\x1A\xFCWa\x1A\xFCa&\x18V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81a\x1B\x1E\x81a&VV[\x92PP[`\x01\x01a\x1A\x97V[PPP\x92\x91PPV[_\x80[`\x01\x84\x01T\x81\x10\x15a\x1B\x8CW\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01\x01\x82\x81T\x81\x10a\x1BaWa\x1Baa&\x18V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B\x84W`\x01\x91PPa\rwV[`\x01\x01a\x1B6V[P_\x93\x92PPPV[\x81Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xF7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x1B\x91\x90a&\xB8V[\x15a\x1C(WP`\x01a\rwV[\x82`@\x01Q\x80\x15a\r\x08WP` \x83\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x97W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x08\x91\x90a&\xB8V[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x1C\xF9Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1D%Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1DCWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1D[Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1DoWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1D\x81W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\rwW`\x01\x01\x92\x91PPV[_\x82``\x01Q\x82\x10a\x1D\xA6WP`\x01a\rwV[\x82`@\x01Q\x80\x15a\r\x08WPP`\x80\x90\x91\x01Q\x11\x15\x90V[_\x80_\x83Q\x90P_\x85``\x01Q\x82\x10\x15\x90P_\x86`@\x01Q\x80\x15a\x1D\xE6WP\x86`\x80\x01Q\x83\x10\x15[\x90P\x81\x15\x80\x15a\x1D\xF4WP\x80\x15[\x15a\x1E\x07W_\x80\x94P\x94PPPPa\x19\xF7V[_\x80_[\x85\x81\x10\x15a\x1FzW_\x89\x82\x81Q\x81\x10a\x1E&Wa\x1E&a&\x18V[` \x02` \x01\x01Q\x90P\x85\x80\x15a\x1E\xBCWP\x8AQ`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x98W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xBC\x91\x90a&\xB8V[\x15a\x1E\xCFW\x83a\x1E\xCB\x81a&VV[\x94PP[\x84\x80\x15a\x1F^WP` \x8B\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F:W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F^\x91\x90a&\xB8V[\x15a\x1FqW\x82a\x1Fm\x81a&VV[\x93PP[P`\x01\x01a\x1E\x0BV[P\x83\x80\x15a\x1F\x8CWP\x88``\x01Q\x82\x10\x15[\x15a\x1F\xA4WPP\x86Q`\x01\x95P\x93Pa\x19\xF7\x92PPPV[\x82\x80\x15a\x1F\xB5WP\x88`\x80\x01Q\x81\x10\x15[\x15a\x1F\xCFW`\x01\x89` \x01Q\x96P\x96PPPPPPa\x19\xF7V[P_\x98\x89\x98P\x96PPPPPPPV[\x81a\x1F\xFDW`@Qcr\x84\x92\x85`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a \x07\x85a\x13!V[_\x85\x81R`\x04\x82\x01` R`@\x90 `\x02\x81\x01T\x91\x92P\x90\x15a OW`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x0EV[`\x02\x81\x81\x01\x85\x90U\x82\x01Ta\x01\0\x90\x04`\xFF\x16a \x83W`\x02\x82\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U`\x03\x82\x01\x85\x90Ua \x97V[\x81`\x03\x01T\x85\x11\x15a \x97W`\x03\x82\x01\x85\x90U[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x86\x91`\x01`\x01`@\x1B\x03\x89\x16\x91\x7F\x96\x89\xD96`\x9E\xEE\xD0@\x1BO\xEC\xCE\xCF\xEA\x81!m\xE8\xFF\x02\xBC\x8END\xD0\xCD\x9Fg,\08\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[a \xEE\x82a!:V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a!2Wa\x19^\x82\x82a!\x9DV[a\x0C\x18a\"\x0FV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a!oW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04\x0EV[_\x80Q` a&\xF3\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa!\xB9\x91\x90a&\xD7V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a!\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\xF6V[``\x91P[P\x91P\x91Pa\"\x06\x85\x83\x83a\".V[\x95\x94PPPPPV[4\x15a\x17\xF6W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\"CWa\">\x82a\"\x8AV[a\r\x08V[\x81Q\x15\x80\x15a\"ZWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\"\x83W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x0EV[P\x80a\r\x08V[\x80Q\x15a\"\x9AW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a\"\xCDW\x81\x81\x01Q\x83\x82\x01R` \x01a\"\xB5V[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra\"\xF3\x81`@\x85\x01` \x87\x01a\"\xB3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13\x87W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a#.W_\x80\xFD[a#7\x83a#\x07V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_``\x84\x86\x03\x12\x15a#WW_\x80\xFD[a#`\x84a#\x07V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a#\x85W_\x80\xFD[a\r\x08\x82a#\x07V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R_\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a#\xE5W\x84Q\x80Q\x83R\x86\x01Q\x86\x83\x01R\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a#\xBCV[P\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x9FW_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a$*W_\x80\xFD[\x825a$5\x81a#\xF1V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$PW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a$cW_\x80\xFD[\x815\x81\x81\x11\x15a$uWa$ua$\x05V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a$\x9DWa$\x9Da$\x05V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a$\xB5W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a%\x16W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a$\xF1V[P\x90\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a%\x16W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a%=V[_\x85Qa%j\x81\x84` \x8A\x01a\"\xB3V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa%\x89\x81`\x02\x84\x01` \x8A\x01a\"\xB3V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa%\xAD\x81`\x03\x85\x01` \x8A\x01a\"\xB3V[`\x03\x92\x01\x91\x82\x01R\x83Qa%\xC8\x81`\x04\x84\x01` \x88\x01a\"\xB3V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a%\xE6W_\x80\xFD[\x81Qa\r\x08\x81a#\xF1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\rwWa\rwa%\xF1V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\rwWa\rwa%\xF1V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\rwWa\rwa%\xF1V[_`\x01\x82\x01a&gWa&ga%\xF1V[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82a&\x9CWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a&\xB1W_\x80\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a&\xC8W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\r\x08W_\x80\xFD[_\x82Qa&\xE8\x81\x84` \x87\x01a\"\xB3V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106100fa575f3560e01c80634f1ef28611610092578063ad3cb1cc11610062578063ad3cb1cc146102a7578063de93c514146102d7578063e7069431146102f6578063e8ad358714610315578063f5178ccf14610334575f80fd5b80634f1ef2861461022857806352d1902d1461023b5780635757a3231461024f578063919406081461027b575f80fd5b80633662bd62116100cd5780633662bd621461019c57806337a08d12146101c957806339f73810146101e8578063459500e2146101fc575f80fd5b80630d8e6e2c146100fe578063114bf5711461012857806319ba5fcc146101495780632949ca4714610168575b5f80fd5b348015610109575f80fd5b50610112610353565b60405161011f91906122d5565b60405180910390f35b348015610133575f80fd5b5061014761014236600461231d565b6103c0565b005b348015610154575f80fd5b50610147610163366004612345565b61045e565b348015610173575f80fd5b5061018761018236600461231d565b610668565b6040805192835260208301919091520161011f565b3480156101a7575f80fd5b506101bb6101b6366004612375565b61076e565b60405190815260200161011f565b3480156101d4575f80fd5b506101476101e336600461231d565b6107bb565b3480156101f3575f80fd5b50610147610a34565b348015610207575f80fd5b5061021b610216366004612375565b610b55565b60405161011f919061238e565b610147610236366004612419565b610bfd565b348015610246575f80fd5b506101bb610c1c565b34801561025a575f80fd5b5061026e610269366004612345565b610c38565b60405161011f91906124d6565b348015610286575f80fd5b5061029a61029536600461231d565b610d0f565b60405161011f9190612522565b3480156102b2575f80fd5b50610112604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156102e2575f80fd5b506101476102f1366004612345565b610d7d565b348015610301575f80fd5b50610147610310366004612345565b610f77565b348015610320575f80fd5b506101bb61032f36600461231d565b611124565b34801561033f575f80fd5b506101bb61034e36600461231d565b611176565b60606040518060400160405280600f81526020016e43697068657274657874526f6f747360881b8152506103865f611292565b6103906001611292565b6103995f611292565b6040516020016103ac9493929190612559565b604051602081830303815290604052905090565b5f6103ca83611321565b5f83815260048201602052604090206002810154919250901561041757604051631fae076360e31b81526001600160401b0385166004820152602481018490526044015b60405180910390fd5b610429848461042461138c565b611628565b61045857604051637cbde47960e01b81526001600160401b03851660048201526024810184905260440161040e565b50505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156104ae573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906104d291906125d6565b6001600160a01b0316336001600160a01b0316146105055760405163021bfda160e41b815233600482015260240161040e565b805f036105255760405163c784f54b60e01b815260040160405180910390fd5b5f61052f84611321565b60018082018054929350915f9183916105489190612605565b8154811061055857610558612618565b905f5260205f2090600202015f01546001610573919061262c565b6002840154909150610100900460ff168015610593575080836003015410155b156105ab5760038301546105a890600161262c565b90505b808510156105e55760405163c3216f1960e01b81526001600160401b0387166004820152602481018690526044810182905260640161040e565b60408051808201825286815260208082018781528554600180820188555f88815284902094516002909202909401908155905192019190915581518781529081018690526001600160401b038816917fa52450ac0a285dfafbcb8f29a79432af527935b468a51ad7ec5ad00cdcc4b0c491015b60405180910390a2505050505050565b5f805f61067485611321565b5f0190505f8160010190505f815f8154811061069257610692612618565b5f91825260209091208454600290920201915060015b835481101561071d575f8482815481106106c4576106c4612618565b905f5260205f2090600202019050805f01548910156106e3575061071d565b835481545f916106f291612605565b9050846001015481610704919061263f565b61070e908561262c565b919450909250506001016106a8565b506001820154825461072f9089612605565b610739919061263f565b610743908261262c565b95506001826001015487610757919061262c565b6107619190612605565b9450505050509250929050565b5f8061077983611321565b6002810154909150610100900460ff166107b1576040516365d5ea3f60e11b81526001600160401b038416600482015260240161040e565b6003015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561080b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061082f91906125d6565b6001600160a01b0316336001600160a01b0316146108625760405163021bfda160e41b815233600482015260240161040e565b5f61086c83611321565b6002810154909150610100900460ff16801561088c575080600301548211155b156108d6578282826003015460016108a4919061262c565b60405163c3216f1960e01b81526001600160401b0390931660048401526024830191909152604482015260640161040e565b6001808201805490915b818110801561090f5750848382815481106108fd576108fd612618565b905f5260205f2090600202015f015414155b15610926578061091e81612656565b9150506108e0565b8181036109585760405163bc59e79760e01b81526001600160401b03871660048201526024810186905260440161040e565b805b610965600184612605565b8110156109ca578361097882600161262c565b8154811061098857610988612618565b905f5260205f2090600202018482815481106109a6576109a6612618565b5f91825260209091208254600290920201908155600191820154908201550161095a565b50828054806109db576109db61266e565b5f8281526020812060025f19909301928302018181556001015590556040516001600160401b038716907f7b9a73a467acdb27eee35e880ec775db86925cd82b94cb4cb6ddf70eb5cca5db906106589088815260200190565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00546001600160401b03166001600160401b0316600114610a8857604051636f4f731f60e01b815260040160405180910390fd5b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460029190600160401b900460ff1680610ad1575080546001600160401b03808416911610155b15610aef5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1768ff0000000000000000191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15050565b604080518082019091525f815260606020820152610b7282611321565b5f016040518060400160405290815f820154815260200160018201805480602002602001604051908101604052809291908181526020015f905b82821015610bef578382905f5260205f2090600202016040518060400160405290815f820154815260200160018201548152505081526020019060010190610bac565b505050915250909392505050565b610c05611752565b610c0e826117f8565b610c1882826118a2565b5050565b5f610c25611963565b505f805160206126f38339815191525b90565b60605f610c4485611321565b5f85815260049190910160205260408120915080610c6283866119ac565b9150915081610c84575050604080515f8152602081019091529150610d089050565b610d0283600101805480602002602001604051908101604052809291908181526020015f905b82821015610cf8575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101610caa565b50505050826119fe565b93505050505b9392505050565b6060610d1a83611321565b5f838152600491909101602090815260409182902080548351818402810184019094528084529091830182828015610d6f57602002820191905f5260205f20905b815481526020019060010190808311610d5b575b505050505090505b92915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610dcd573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610df191906125d6565b6001600160a01b0316336001600160a01b031614610e245760405163021bfda160e41b815233600482015260240161040e565b826001600160401b03165f03610e4c576040516273401f60e71b815260040160405180910390fd5b805f03610e6c5760405163c784f54b60e01b815260040160405180910390fd5b6001600160401b0383165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff1615610ed357604051630f17637d60e21b81526001600160401b038516600482015260240161040e565b6002808201805460ff191660019081179091558483556040805180820182525f80825260208083018881528786018054808801825590845291909220925195029091019384555192909101919091555181906001600160401b038616907f3b3ac53f416d9ad04e258f647d21d53088f789d2492abf9108e409f1018f3bea90610f689087908790918252602082015260400190565b60405180910390a25050505050565b80610f9557604051637284928560e11b815260040160405180910390fd5b5f610f9f84611321565b5f8481526004820160205260409020909150610fbb8133611b33565b15610ff1576040516304f2eb0160e11b81526001600160401b03861660048201526024810185905233604482015260640161040e565b5f610ffa61138c565b90506110068133611b95565b611025576040516352d725f560e01b815233600482015260240161040e565b60028201545f9015158061103f575061103f878784611628565b90505f8061104d85886119ac565b915091508161106d575083546001810185555f8581526020902081018790555b604080518082018252338082526001600160601b0380851660208085019182526001808c01805491820181555f9081529190912094519151909216600160a01b026001600160a01b039190911617920191909155905189906001600160401b038c16907fac3e570e86481985ba4e8e32ee33ccb4cf7365bfccfd2e7e8b5a0535f960ea44906110ff908c815260200190565b60405180910390a48261111957611117898986611628565b505b505050505050505050565b5f8061112f84611321565b5f8481526004919091016020526040902060020154905080610d085760405163158062cb60e11b81526001600160401b03851660048201526024810184905260440161040e565b5f8061118184611321565b80549091508310156111b857604051631dd79bc560e21b81526001600160401b03851660048201526024810184905260440161040e565b5f8160010190505f815f815481106111d2576111d2612618565b5f91825260209091208454600290920201915060015b8354811015611261575f84828154811061120457611204612618565b5f91825260208220865460029092020180549093506112239190612605565b90505f856001015482611236919061263f565b611240908661262c565b9050808a101561125257505050611261565b919450909250506001016111e8565b5060018201546112718288612605565b61127b9190612682565b8254611287919061262c565b979650505050505050565b60605f61129e83611cbb565b60010190505f816001600160401b038111156112bc576112bc612405565b6040519080825280601f01601f1916602001820160405280156112e6576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a85049450846112f057509392505050565b6001600160401b0381165f9081527f3cb20319aa5e29e6b1c7b40fc8b1cf52b6eafa219f4e9b747b4b0c9dc1f1200060205260409020600281015460ff16611387576040516329cbdd2360e01b81526001600160401b038316600482015260240161040e565b919050565b6113bb6040518060a001604052805f81526020015f81526020015f151581526020015f81526020015f81525090565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b031663170a29816040518163ffffffff1660e01b8152600401602060405180830381865afa15801561140b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061142f91906126a1565b808252604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa158015611481573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114a591906126a1565b8160600181815250507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166389090a446040518163ffffffff1660e01b8152600401602060405180830381865afa1580156114fe573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061152291906126a1565b60208201819052604051630e1887c960e01b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac90630e1887c990602401602060405180830381865afa158015611578573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061159c91906126b8565b1580156040830152610c35576020810151604051633d9f78df60e21b815260048101919091527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063f67de37c90602401602060405180830381865afa1580156115fc573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061162091906126a1565b608082015290565b5f8061163385611321565b5f858152600491909101602052604090206001810154909150611657908490611d92565b611664575f915050610d08565b5f81600101805480602002602001604051908101604052809291908181526020015f905b828210156116d6575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b031681830152825260019092019101611688565b5050505090505f5b8254811015611746575f806116fc876116f786866119fe565b611dbe565b91509150811561173c5761172e8989875f01868154811061171f5761171f612618565b905f5260205f20015484611fdf565b600195505050505050610d08565b50506001016116de565b505f9695505050505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806117d857507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166117cc5f805160206126f3833981519152546001600160a01b031690565b6001600160a01b031614155b156117f65760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611848573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061186c91906125d6565b6001600160a01b0316336001600160a01b03161461189f5760405163021bfda160e41b815233600482015260240161040e565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156118fc575060408051601f3d908101601f191682019092526118f9918101906126a1565b60015b61192457604051634c9c8ce360e01b81526001600160a01b038316600482015260240161040e565b5f805160206126f3833981519152811461195457604051632a87526960e21b81526004810182905260240161040e565b61195e83836120e5565b505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146117f65760405163703e46dd60e11b815260040160405180910390fd5b5f805f5b84548110156119ef5783855f0182815481106119ce576119ce612618565b905f5260205f200154036119e7576001925090506119f7565b6001016119b0565b505f80915091505b9250929050565b60605f805b8451811015611a4f5783858281518110611a1f57611a1f612618565b6020026020010151602001516001600160601b031603611a475781611a4381612656565b9250505b600101611a03565b50806001600160401b03811115611a6857611a68612405565b604051908082528060200260200182016040528015611a91578160200160208202803683370190505b5091505f805b8551811015611b2a5784868281518110611ab357611ab3612618565b6020026020010151602001516001600160601b031603611b2257858181518110611adf57611adf612618565b60200260200101515f0151848381518110611afc57611afc612618565b6001600160a01b039092166020928302919091019091015281611b1e81612656565b9250505b600101611a97565b50505092915050565b5f805b6001840154811015611b8c57826001600160a01b0316846001018281548110611b6157611b61612618565b5f918252602090912001546001600160a01b031603611b84576001915050610d77565b600101611b36565b505f9392505050565b8151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201525f907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611bf7573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c1b91906126b8565b15611c2857506001610d77565b82604001518015610d0857506020830151604051633ac3ddd960e21b815260048101919091526001600160a01b03831660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611c97573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d0891906126b8565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b8310611cf95772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef81000000008310611d25576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310611d4357662386f26fc10000830492506010015b6305f5e1008310611d5b576305f5e100830492506008015b6127108310611d6f57612710830492506004015b60648310611d81576064830492506002015b600a8310610d775760010192915050565b5f82606001518210611da657506001610d77565b82604001518015610d08575050608090910151111590565b5f805f835190505f856060015182101590505f86604001518015611de6575086608001518310155b905081158015611df4575080155b15611e07575f80945094505050506119f7565b5f805f5b85811015611f7a575f898281518110611e2657611e26612618565b60200260200101519050858015611ebc57508a51604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611e98573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ebc91906126b8565b15611ecf5783611ecb81612656565b9450505b848015611f5e575060208b0151604051633ac3ddd960e21b815260048101919091526001600160a01b03821660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac9063eb0f776490604401602060405180830381865afa158015611f3a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611f5e91906126b8565b15611f715782611f6d81612656565b9350505b50600101611e0b565b50838015611f8c575088606001518210155b15611fa457505086516001955093506119f792505050565b828015611fb5575088608001518110155b15611fcf57600189602001519650965050505050506119f7565b505f988998509650505050505050565b81611ffd57604051637284928560e11b815260040160405180910390fd5b5f61200785611321565b5f85815260048201602052604090206002810154919250901561204f57604051631fae076360e31b81526001600160401b03871660048201526024810186905260440161040e565b6002818101859055820154610100900460ff166120835760028201805461ff00191661010017905560038201859055612097565b816003015485111561209757600382018590555b604080518581526020810185905286916001600160401b038916917f9689d936609eeed0401b4feccecfea81216de8ff02bc8e4e44d0cd9f672c0038910160405180910390a3505050505050565b6120ee8261213a565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156121325761195e828261219d565b610c1861220f565b806001600160a01b03163b5f0361216f57604051634c9c8ce360e01b81526001600160a01b038216600482015260240161040e565b5f805160206126f383398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516121b991906126d7565b5f60405180830381855af49150503d805f81146121f1576040519150601f19603f3d011682016040523d82523d5f602084013e6121f6565b606091505b509150915061220685838361222e565b95945050505050565b34156117f65760405163b398979f60e01b815260040160405180910390fd5b6060826122435761223e8261228a565b610d08565b815115801561225a57506001600160a01b0384163b155b1561228357604051639996b31560e01b81526001600160a01b038516600482015260240161040e565b5080610d08565b80511561229a5780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b5f5b838110156122cd5781810151838201526020016122b5565b50505f910152565b602081525f82518060208401526122f38160408501602087016122b3565b601f01601f19169190910160400192915050565b80356001600160401b0381168114611387575f80fd5b5f806040838503121561232e575f80fd5b61233783612307565b946020939093013593505050565b5f805f60608486031215612357575f80fd5b61236084612307565b95602085013595506040909401359392505050565b5f60208284031215612385575f80fd5b610d0882612307565b60208082528251828201528281015160408084018190528151606085018190525f9392830191849160808701905b808410156123e557845180518352860151868301529385019360019390930192908201906123bc565b50979650505050505050565b6001600160a01b038116811461189f575f80fd5b634e487b7160e01b5f52604160045260245ffd5b5f806040838503121561242a575f80fd5b8235612435816123f1565b915060208301356001600160401b0380821115612450575f80fd5b818501915085601f830112612463575f80fd5b81358181111561247557612475612405565b604051601f8201601f19908116603f0116810190838211818310171561249d5761249d612405565b816040528281528860208487010111156124b5575f80fd5b826020860160208301375f6020848301015280955050505050509250929050565b602080825282518282018190525f9190848201906040850190845b818110156125165783516001600160a01b0316835292840192918401916001016124f1565b50909695505050505050565b602080825282518282018190525f9190848201906040850190845b818110156125165783518352928401929184019160010161253d565b5f855161256a818460208a016122b3565b61103b60f11b9083019081528551612589816002840160208a016122b3565b808201915050601760f91b80600283015285516125ad816003850160208a016122b3565b600392019182015283516125c88160048401602088016122b3565b016004019695505050505050565b5f602082840312156125e6575f80fd5b8151610d08816123f1565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610d7757610d776125f1565b634e487b7160e01b5f52603260045260245ffd5b80820180821115610d7757610d776125f1565b8082028115828204841417610d7757610d776125f1565b5f60018201612667576126676125f1565b5060010190565b634e487b7160e01b5f52603160045260245ffd5b5f8261269c57634e487b7160e01b5f52601260045260245ffd5b500490565b5f602082840312156126b1575f80fd5b5051919050565b5f602082840312156126c8575f80fd5b81518015158114610d08575f80fd5b5f82516126e88184602087016122b3565b919091019291505056fe360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80cO\x1E\xF2\x86\x11a\0\x92W\x80c\xAD<\xB1\xCC\x11a\0bW\x80c\xAD<\xB1\xCC\x14a\x02\xA7W\x80c\xDE\x93\xC5\x14\x14a\x02\xD7W\x80c\xE7\x06\x941\x14a\x02\xF6W\x80c\xE8\xAD5\x87\x14a\x03\x15W\x80c\xF5\x17\x8C\xCF\x14a\x034W_\x80\xFD[\x80cO\x1E\xF2\x86\x14a\x02(W\x80cR\xD1\x90-\x14a\x02;W\x80cWW\xA3#\x14a\x02OW\x80c\x91\x94\x06\x08\x14a\x02{W_\x80\xFD[\x80c6b\xBDb\x11a\0\xCDW\x80c6b\xBDb\x14a\x01\x9CW\x80c7\xA0\x8D\x12\x14a\x01\xC9W\x80c9\xF78\x10\x14a\x01\xE8W\x80cE\x95\0\xE2\x14a\x01\xFCW_\x80\xFD[\x80c\r\x8En,\x14a\0\xFEW\x80c\x11K\xF5q\x14a\x01(W\x80c\x19\xBA_\xCC\x14a\x01IW\x80c)I\xCAG\x14a\x01hW[_\x80\xFD[4\x80\x15a\x01\tW_\x80\xFD[Pa\x01\x12a\x03SV[`@Qa\x01\x1F\x91\x90a\"\xD5V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x013W_\x80\xFD[Pa\x01Ga\x01B6`\x04a#\x1DV[a\x03\xC0V[\0[4\x80\x15a\x01TW_\x80\xFD[Pa\x01Ga\x01c6`\x04a#EV[a\x04^V[4\x80\x15a\x01sW_\x80\xFD[Pa\x01\x87a\x01\x826`\x04a#\x1DV[a\x06hV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1FV[4\x80\x15a\x01\xA7W_\x80\xFD[Pa\x01\xBBa\x01\xB66`\x04a#uV[a\x07nV[`@Q\x90\x81R` \x01a\x01\x1FV[4\x80\x15a\x01\xD4W_\x80\xFD[Pa\x01Ga\x01\xE36`\x04a#\x1DV[a\x07\xBBV[4\x80\x15a\x01\xF3W_\x80\xFD[Pa\x01Ga\n4V[4\x80\x15a\x02\x07W_\x80\xFD[Pa\x02\x1Ba\x02\x166`\x04a#uV[a\x0BUV[`@Qa\x01\x1F\x91\x90a#\x8EV[a\x01Ga\x0266`\x04a$\x19V[a\x0B\xFDV[4\x80\x15a\x02FW_\x80\xFD[Pa\x01\xBBa\x0C\x1CV[4\x80\x15a\x02ZW_\x80\xFD[Pa\x02na\x02i6`\x04a#EV[a\x0C8V[`@Qa\x01\x1F\x91\x90a$\xD6V[4\x80\x15a\x02\x86W_\x80\xFD[Pa\x02\x9Aa\x02\x956`\x04a#\x1DV[a\r\x0FV[`@Qa\x01\x1F\x91\x90a%\"V[4\x80\x15a\x02\xB2W_\x80\xFD[Pa\x01\x12`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x01Ga\x02\xF16`\x04a#EV[a\r}V[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x01Ga\x03\x106`\x04a#EV[a\x0FwV[4\x80\x15a\x03 W_\x80\xFD[Pa\x01\xBBa\x03/6`\x04a#\x1DV[a\x11$V[4\x80\x15a\x03?W_\x80\xFD[Pa\x01\xBBa\x03N6`\x04a#\x1DV[a\x11vV[```@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nCiphertextRoots`\x88\x1B\x81RPa\x03\x86_a\x12\x92V[a\x03\x90`\x01a\x12\x92V[a\x03\x99_a\x12\x92V[`@Q` \x01a\x03\xAC\x94\x93\x92\x91\x90a%YV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[_a\x03\xCA\x83a\x13!V[_\x83\x81R`\x04\x82\x01` R`@\x90 `\x02\x81\x01T\x91\x92P\x90\x15a\x04\x17W`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x04)\x84\x84a\x04$a\x13\x8CV[a\x16(V[a\x04XW`@Qc|\xBD\xE4y`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x0EV[PPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xAEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xD2\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\x05W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[\x80_\x03a\x05%W`@Qc\xC7\x84\xF5K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x05/\x84a\x13!V[`\x01\x80\x82\x01\x80T\x92\x93P\x91_\x91\x83\x91a\x05H\x91\x90a&\x05V[\x81T\x81\x10a\x05XWa\x05Xa&\x18V[\x90_R` _ \x90`\x02\x02\x01_\x01T`\x01a\x05s\x91\x90a&,V[`\x02\x84\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16\x80\x15a\x05\x93WP\x80\x83`\x03\x01T\x10\x15[\x15a\x05\xABW`\x03\x83\x01Ta\x05\xA8\x90`\x01a&,V[\x90P[\x80\x85\x10\x15a\x05\xE5W`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x81\x01\x82\x90R`d\x01a\x04\x0EV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R\x85T`\x01\x80\x82\x01\x88U_\x88\x81R\x84\x90 \x94Q`\x02\x90\x92\x02\x90\x94\x01\x90\x81U\x90Q\x92\x01\x91\x90\x91U\x81Q\x87\x81R\x90\x81\x01\x86\x90R`\x01`\x01`@\x1B\x03\x88\x16\x91\x7F\xA5$P\xAC\n(]\xFA\xFB\xCB\x8F)\xA7\x942\xAFRy5\xB4h\xA5\x1A\xD7\xECZ\xD0\x0C\xDC\xC4\xB0\xC4\x91\x01[`@Q\x80\x91\x03\x90\xA2PPPPPPV[_\x80_a\x06t\x85a\x13!V[_\x01\x90P_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x06\x92Wa\x06\x92a&\x18V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x07\x1DW_\x84\x82\x81T\x81\x10a\x06\xC4Wa\x06\xC4a&\x18V[\x90_R` _ \x90`\x02\x02\x01\x90P\x80_\x01T\x89\x10\x15a\x06\xE3WPa\x07\x1DV[\x83T\x81T_\x91a\x06\xF2\x91a&\x05V[\x90P\x84`\x01\x01T\x81a\x07\x04\x91\x90a&?V[a\x07\x0E\x90\x85a&,V[\x91\x94P\x90\x92PP`\x01\x01a\x06\xA8V[P`\x01\x82\x01T\x82Ta\x07/\x90\x89a&\x05V[a\x079\x91\x90a&?V[a\x07C\x90\x82a&,V[\x95P`\x01\x82`\x01\x01T\x87a\x07W\x91\x90a&,V[a\x07a\x91\x90a&\x05V[\x94PPPPP\x92P\x92\x90PV[_\x80a\x07y\x83a\x13!V[`\x02\x81\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16a\x07\xB1W`@Qce\xD5\xEA?`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`$\x01a\x04\x0EV[`\x03\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x0BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08/\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x08bW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[_a\x08l\x83a\x13!V[`\x02\x81\x01T\x90\x91Pa\x01\0\x90\x04`\xFF\x16\x80\x15a\x08\x8CWP\x80`\x03\x01T\x82\x11\x15[\x15a\x08\xD6W\x82\x82\x82`\x03\x01T`\x01a\x08\xA4\x91\x90a&,V[`@Qc\xC3!o\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x93\x16`\x04\x84\x01R`$\x83\x01\x91\x90\x91R`D\x82\x01R`d\x01a\x04\x0EV[`\x01\x80\x82\x01\x80T\x90\x91[\x81\x81\x10\x80\x15a\t\x0FWP\x84\x83\x82\x81T\x81\x10a\x08\xFDWa\x08\xFDa&\x18V[\x90_R` _ \x90`\x02\x02\x01_\x01T\x14\x15[\x15a\t&W\x80a\t\x1E\x81a&VV[\x91PPa\x08\xE0V[\x81\x81\x03a\tXW`@Qc\xBCY\xE7\x97`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x0EV[\x80[a\te`\x01\x84a&\x05V[\x81\x10\x15a\t\xCAW\x83a\tx\x82`\x01a&,V[\x81T\x81\x10a\t\x88Wa\t\x88a&\x18V[\x90_R` _ \x90`\x02\x02\x01\x84\x82\x81T\x81\x10a\t\xA6Wa\t\xA6a&\x18V[_\x91\x82R` \x90\x91 \x82T`\x02\x90\x92\x02\x01\x90\x81U`\x01\x91\x82\x01T\x90\x82\x01U\x01a\tZV[P\x82\x80T\x80a\t\xDBWa\t\xDBa&nV[_\x82\x81R` \x81 `\x02_\x19\x90\x93\x01\x92\x83\x02\x01\x81\x81U`\x01\x01U\x90U`@Q`\x01`\x01`@\x1B\x03\x87\x16\x90\x7F{\x9As\xA4g\xAC\xDB'\xEE\xE3^\x88\x0E\xC7u\xDB\x86\x92\\\xD8+\x94\xCBL\xB6\xDD\xF7\x0E\xB5\xCC\xA5\xDB\x90a\x06X\x90\x88\x81R` \x01\x90V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\n\x88W`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x02\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\n\xD1WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\n\xEFW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17h\xFF\0\0\0\0\0\0\0\0\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01Ra\x0Br\x82a\x13!V[_\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0B\xEFW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01T\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B\xACV[PPP\x91RP\x90\x93\x92PPPV[a\x0C\x05a\x17RV[a\x0C\x0E\x82a\x17\xF8V[a\x0C\x18\x82\x82a\x18\xA2V[PPV[_a\x0C%a\x19cV[P_\x80Q` a&\xF3\x839\x81Q\x91R[\x90V[``_a\x0CD\x85a\x13!V[_\x85\x81R`\x04\x91\x90\x91\x01` R`@\x81 \x91P\x80a\x0Cb\x83\x86a\x19\xACV[\x91P\x91P\x81a\x0C\x84WPP`@\x80Q_\x81R` \x81\x01\x90\x91R\x91Pa\r\x08\x90PV[a\r\x02\x83`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xF8W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0C\xAAV[PPPP\x82a\x19\xFEV[\x93PPPP[\x93\x92PPPV[``a\r\x1A\x83a\x13!V[_\x83\x81R`\x04\x91\x90\x91\x01` \x90\x81R`@\x91\x82\x90 \x80T\x83Q\x81\x84\x02\x81\x01\x84\x01\x90\x94R\x80\x84R\x90\x91\x83\x01\x82\x82\x80\x15a\roW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\r[W[PPPPP\x90P[\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xCDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xF1\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E$W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[\x82`\x01`\x01`@\x1B\x03\x16_\x03a\x0ELW`@Qbs@\x1F`\xE7\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80_\x03a\x0ElW`@Qc\xC7\x84\xF5K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x83\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16\x15a\x0E\xD3W`@Qc\x0F\x17c}`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x0EV[`\x02\x80\x82\x01\x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x84\x83U`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x88\x81R\x87\x86\x01\x80T\x80\x88\x01\x82U\x90\x84R\x91\x90\x92 \x92Q\x95\x02\x90\x91\x01\x93\x84UQ\x92\x90\x91\x01\x91\x90\x91UQ\x81\x90`\x01`\x01`@\x1B\x03\x86\x16\x90\x7F;:\xC5?Am\x9A\xD0N%\x8Fd}!\xD50\x88\xF7\x89\xD2I*\xBF\x91\x08\xE4\t\xF1\x01\x8F;\xEA\x90a\x0Fh\x90\x87\x90\x87\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA2PPPPPV[\x80a\x0F\x95W`@Qcr\x84\x92\x85`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\x9F\x84a\x13!V[_\x84\x81R`\x04\x82\x01` R`@\x90 \x90\x91Pa\x0F\xBB\x813a\x1B3V[\x15a\x0F\xF1W`@Qc\x04\xF2\xEB\x01`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x82\x01R`$\x81\x01\x85\x90R3`D\x82\x01R`d\x01a\x04\x0EV[_a\x0F\xFAa\x13\x8CV[\x90Pa\x10\x06\x813a\x1B\x95V[a\x10%W`@QcR\xD7%\xF5`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[`\x02\x82\x01T_\x90\x15\x15\x80a\x10?WPa\x10?\x87\x87\x84a\x16(V[\x90P_\x80a\x10M\x85\x88a\x19\xACV[\x91P\x91P\x81a\x10mWP\x83T`\x01\x81\x01\x85U_\x85\x81R` \x90 \x81\x01\x87\x90U[`@\x80Q\x80\x82\x01\x82R3\x80\x82R`\x01`\x01``\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R`\x01\x80\x8C\x01\x80T\x91\x82\x01\x81U_\x90\x81R\x91\x90\x91 \x94Q\x91Q\x90\x92\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x92\x01\x91\x90\x91U\x90Q\x89\x90`\x01`\x01`@\x1B\x03\x8C\x16\x90\x7F\xAC>W\x0E\x86H\x19\x85\xBAN\x8E2\xEE3\xCC\xB4\xCFse\xBF\xCC\xFD.~\x8BZ\x055\xF9`\xEAD\x90a\x10\xFF\x90\x8C\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA4\x82a\x11\x19Wa\x11\x17\x89\x89\x86a\x16(V[P[PPPPPPPPPV[_\x80a\x11/\x84a\x13!V[_\x84\x81R`\x04\x91\x90\x91\x01` R`@\x90 `\x02\x01T\x90P\x80a\r\x08W`@Qc\x15\x80b\xCB`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x0EV[_\x80a\x11\x81\x84a\x13!V[\x80T\x90\x91P\x83\x10\x15a\x11\xB8W`@Qc\x1D\xD7\x9B\xC5`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x01a\x04\x0EV[_\x81`\x01\x01\x90P_\x81_\x81T\x81\x10a\x11\xD2Wa\x11\xD2a&\x18V[_\x91\x82R` \x90\x91 \x84T`\x02\x90\x92\x02\x01\x91P`\x01[\x83T\x81\x10\x15a\x12aW_\x84\x82\x81T\x81\x10a\x12\x04Wa\x12\x04a&\x18V[_\x91\x82R` \x82 \x86T`\x02\x90\x92\x02\x01\x80T\x90\x93Pa\x12#\x91\x90a&\x05V[\x90P_\x85`\x01\x01T\x82a\x126\x91\x90a&?V[a\x12@\x90\x86a&,V[\x90P\x80\x8A\x10\x15a\x12RWPPPa\x12aV[\x91\x94P\x90\x92PP`\x01\x01a\x11\xE8V[P`\x01\x82\x01Ta\x12q\x82\x88a&\x05V[a\x12{\x91\x90a&\x82V[\x82Ta\x12\x87\x91\x90a&,V[\x97\x96PPPPPPPV[``_a\x12\x9E\x83a\x1C\xBBV[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xBCWa\x12\xBCa$\x05V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x12\xE6W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x12\xF0WP\x93\x92PPPV[`\x01`\x01`@\x1B\x03\x81\x16_\x90\x81R\x7F<\xB2\x03\x19\xAA^)\xE6\xB1\xC7\xB4\x0F\xC8\xB1\xCFR\xB6\xEA\xFA!\x9FN\x9Bt{K\x0C\x9D\xC1\xF1 \0` R`@\x90 `\x02\x81\x01T`\xFF\x16a\x13\x87W`@Qc)\xCB\xDD#`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x0EV[\x91\x90PV[a\x13\xBB`@Q\x80`\xA0\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x15\x15\x81R` \x01_\x81R` \x01_\x81RP\x90V[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x17\n)\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x0BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14/\x91\x90a&\xA1V[\x80\x82R`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\x81W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xA5\x91\x90a&\xA1V[\x81``\x01\x81\x81RPPsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x89\t\nD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xFEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\"\x91\x90a&\xA1V[` \x82\x01\x81\x90R`@Qc\x0E\x18\x87\xC9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\x0E\x18\x87\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15xW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x9C\x91\x90a&\xB8V[\x15\x80\x15`@\x83\x01Ra\x0C5W` \x81\x01Q`@Qc=\x9Fx\xDF`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xF6}\xE3|\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xFCW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16 \x91\x90a&\xA1V[`\x80\x82\x01R\x90V[_\x80a\x163\x85a\x13!V[_\x85\x81R`\x04\x91\x90\x91\x01` R`@\x90 `\x01\x81\x01T\x90\x91Pa\x16W\x90\x84\x90a\x1D\x92V[a\x16dW_\x91PPa\r\x08V[_\x81`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x16\xD6W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x16\x88V[PPPP\x90P_[\x82T\x81\x10\x15a\x17FW_\x80a\x16\xFC\x87a\x16\xF7\x86\x86a\x19\xFEV[a\x1D\xBEV[\x91P\x91P\x81\x15a\x17<Wa\x17.\x89\x89\x87_\x01\x86\x81T\x81\x10a\x17\x1FWa\x17\x1Fa&\x18V[\x90_R` _ \x01T\x84a\x1F\xDFV[`\x01\x95PPPPPPa\r\x08V[PP`\x01\x01a\x16\xDEV[P_\x96\x95PPPPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x17\xD8WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x17\xCC_\x80Q` a&\xF3\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x17\xF6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18HW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18l\x91\x90a%\xD6V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\x9FW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\x0EV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x18\xFCWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x18\xF9\x91\x81\x01\x90a&\xA1V[`\x01[a\x19$W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\x0EV[_\x80Q` a&\xF3\x839\x81Q\x91R\x81\x14a\x19TW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\x0EV[a\x19^\x83\x83a \xE5V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x17\xF6W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80_[\x84T\x81\x10\x15a\x19\xEFW\x83\x85_\x01\x82\x81T\x81\x10a\x19\xCEWa\x19\xCEa&\x18V[\x90_R` _ \x01T\x03a\x19\xE7W`\x01\x92P\x90Pa\x19\xF7V[`\x01\x01a\x19\xB0V[P_\x80\x91P\x91P[\x92P\x92\x90PV[``_\x80[\x84Q\x81\x10\x15a\x1AOW\x83\x85\x82\x81Q\x81\x10a\x1A\x1FWa\x1A\x1Fa&\x18V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x03a\x1AGW\x81a\x1AC\x81a&VV[\x92PP[`\x01\x01a\x1A\x03V[P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AhWa\x1Aha$\x05V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x91W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P_\x80[\x85Q\x81\x10\x15a\x1B*W\x84\x86\x82\x81Q\x81\x10a\x1A\xB3Wa\x1A\xB3a&\x18V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x03a\x1B\"W\x85\x81\x81Q\x81\x10a\x1A\xDFWa\x1A\xDFa&\x18V[` \x02` \x01\x01Q_\x01Q\x84\x83\x81Q\x81\x10a\x1A\xFCWa\x1A\xFCa&\x18V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81a\x1B\x1E\x81a&VV[\x92PP[`\x01\x01a\x1A\x97V[PPP\x92\x91PPV[_\x80[`\x01\x84\x01T\x81\x10\x15a\x1B\x8CW\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01\x01\x82\x81T\x81\x10a\x1BaWa\x1Baa&\x18V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1B\x84W`\x01\x91PPa\rwV[`\x01\x01a\x1B6V[P_\x93\x92PPPV[\x81Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xF7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\x1B\x91\x90a&\xB8V[\x15a\x1C(WP`\x01a\rwV[\x82`@\x01Q\x80\x15a\r\x08WP` \x83\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x97W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x08\x91\x90a&\xB8V[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x1C\xF9Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x1D%Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x1DCWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x1D[Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x1DoWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x1D\x81W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\rwW`\x01\x01\x92\x91PPV[_\x82``\x01Q\x82\x10a\x1D\xA6WP`\x01a\rwV[\x82`@\x01Q\x80\x15a\r\x08WPP`\x80\x90\x91\x01Q\x11\x15\x90V[_\x80_\x83Q\x90P_\x85``\x01Q\x82\x10\x15\x90P_\x86`@\x01Q\x80\x15a\x1D\xE6WP\x86`\x80\x01Q\x83\x10\x15[\x90P\x81\x15\x80\x15a\x1D\xF4WP\x80\x15[\x15a\x1E\x07W_\x80\x94P\x94PPPPa\x19\xF7V[_\x80_[\x85\x81\x10\x15a\x1FzW_\x89\x82\x81Q\x81\x10a\x1E&Wa\x1E&a&\x18V[` \x02` \x01\x01Q\x90P\x85\x80\x15a\x1E\xBCWP\x8AQ`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x98W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xBC\x91\x90a&\xB8V[\x15a\x1E\xCFW\x83a\x1E\xCB\x81a&VV[\x94PP[\x84\x80\x15a\x1F^WP` \x8B\x01Q`@Qc:\xC3\xDD\xD9`\xE2\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\xEB\x0Fwd\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F:W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F^\x91\x90a&\xB8V[\x15a\x1FqW\x82a\x1Fm\x81a&VV[\x93PP[P`\x01\x01a\x1E\x0BV[P\x83\x80\x15a\x1F\x8CWP\x88``\x01Q\x82\x10\x15[\x15a\x1F\xA4WPP\x86Q`\x01\x95P\x93Pa\x19\xF7\x92PPPV[\x82\x80\x15a\x1F\xB5WP\x88`\x80\x01Q\x81\x10\x15[\x15a\x1F\xCFW`\x01\x89` \x01Q\x96P\x96PPPPPPa\x19\xF7V[P_\x98\x89\x98P\x96PPPPPPPV[\x81a\x1F\xFDW`@Qcr\x84\x92\x85`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a \x07\x85a\x13!V[_\x85\x81R`\x04\x82\x01` R`@\x90 `\x02\x81\x01T\x91\x92P\x90\x15a OW`@Qc\x1F\xAE\x07c`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16`\x04\x82\x01R`$\x81\x01\x86\x90R`D\x01a\x04\x0EV[`\x02\x81\x81\x01\x85\x90U\x82\x01Ta\x01\0\x90\x04`\xFF\x16a \x83W`\x02\x82\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U`\x03\x82\x01\x85\x90Ua \x97V[\x81`\x03\x01T\x85\x11\x15a \x97W`\x03\x82\x01\x85\x90U[`@\x80Q\x85\x81R` \x81\x01\x85\x90R\x86\x91`\x01`\x01`@\x1B\x03\x89\x16\x91\x7F\x96\x89\xD96`\x9E\xEE\xD0@\x1BO\xEC\xCE\xCF\xEA\x81!m\xE8\xFF\x02\xBC\x8END\xD0\xCD\x9Fg,\08\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPV[a \xEE\x82a!:V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a!2Wa\x19^\x82\x82a!\x9DV[a\x0C\x18a\"\x0FV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a!oW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04\x0EV[_\x80Q` a&\xF3\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa!\xB9\x91\x90a&\xD7V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a!\xF1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\xF6V[``\x91P[P\x91P\x91Pa\"\x06\x85\x83\x83a\".V[\x95\x94PPPPPV[4\x15a\x17\xF6W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\"CWa\">\x82a\"\x8AV[a\r\x08V[\x81Q\x15\x80\x15a\"ZWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\"\x83W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\x0EV[P\x80a\r\x08V[\x80Q\x15a\"\x9AW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a\"\xCDW\x81\x81\x01Q\x83\x82\x01R` \x01a\"\xB5V[PP_\x91\x01RV[` \x81R_\x82Q\x80` \x84\x01Ra\"\xF3\x81`@\x85\x01` \x87\x01a\"\xB3V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x13\x87W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a#.W_\x80\xFD[a#7\x83a#\x07V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_``\x84\x86\x03\x12\x15a#WW_\x80\xFD[a#`\x84a#\x07V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_` \x82\x84\x03\x12\x15a#\x85W_\x80\xFD[a\r\x08\x82a#\x07V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R_\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a#\xE5W\x84Q\x80Q\x83R\x86\x01Q\x86\x83\x01R\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a#\xBCV[P\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x9FW_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a$*W_\x80\xFD[\x825a$5\x81a#\xF1V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a$PW_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a$cW_\x80\xFD[\x815\x81\x81\x11\x15a$uWa$ua$\x05V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a$\x9DWa$\x9Da$\x05V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a$\xB5W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a%\x16W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a$\xF1V[P\x90\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a%\x16W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a%=V[_\x85Qa%j\x81\x84` \x8A\x01a\"\xB3V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85Qa%\x89\x81`\x02\x84\x01` \x8A\x01a\"\xB3V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85Qa%\xAD\x81`\x03\x85\x01` \x8A\x01a\"\xB3V[`\x03\x92\x01\x91\x82\x01R\x83Qa%\xC8\x81`\x04\x84\x01` \x88\x01a\"\xB3V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a%\xE6W_\x80\xFD[\x81Qa\r\x08\x81a#\xF1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\rwWa\rwa%\xF1V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\rwWa\rwa%\xF1V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\rwWa\rwa%\xF1V[_`\x01\x82\x01a&gWa&ga%\xF1V[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82a&\x9CWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a&\xB1W_\x80\xFD[PQ\x91\x90PV[_` \x82\x84\x03\x12\x15a&\xC8W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\r\x08W_\x80\xFD[_\x82Qa&\xE8\x81\x84` \x87\x01a\"\xB3V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC",
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
    /**Custom error with signature `EpochLengthNull()` and selector `0xc784f54b`.
```solidity
error EpochLengthNull();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochLengthNull;
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
        impl ::core::convert::From<EpochLengthNull> for UnderlyingRustTuple<'_> {
            fn from(value: EpochLengthNull) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochLengthNull {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochLengthNull {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochLengthNull()";
            const SELECTOR: [u8; 4] = [199u8, 132u8, 245u8, 75u8];
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
    /**Custom error with signature `EpochRootAlreadyPosted(uint64,uint256,address)` and selector `0x09e5d602`.
```solidity
error EpochRootAlreadyPosted(uint64 chainId, uint256 epoch, address sender);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochRootAlreadyPosted {
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
        impl ::core::convert::From<EpochRootAlreadyPosted> for UnderlyingRustTuple<'_> {
            fn from(value: EpochRootAlreadyPosted) -> Self {
                (value.chainId, value.epoch, value.sender)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochRootAlreadyPosted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    epoch: tuple.1,
                    sender: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochRootAlreadyPosted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochRootAlreadyPosted(uint64,uint256,address)";
            const SELECTOR: [u8; 4] = [9u8, 229u8, 214u8, 2u8];
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
    /**Custom error with signature `EpochRootNull()` and selector `0xe509250a`.
```solidity
error EpochRootNull();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EpochRootNull;
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
        impl ::core::convert::From<EpochRootNull> for UnderlyingRustTuple<'_> {
            fn from(value: EpochRootNull) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EpochRootNull {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EpochRootNull {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EpochRootNull()";
            const SELECTOR: [u8; 4] = [229u8, 9u8, 37u8, 10u8];
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
    /**Custom error with signature `HostChainNullId()` and selector `0x39a00f80`.
```solidity
error HostChainNullId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HostChainNullId;
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
        impl ::core::convert::From<HostChainNullId> for UnderlyingRustTuple<'_> {
            fn from(value: HostChainNullId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HostChainNullId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for HostChainNullId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "HostChainNullId()";
            const SELECTOR: [u8; 4] = [57u8, 160u8, 15u8, 128u8];
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
    /**Custom error with signature `NoEpochRootReachesThreshold(uint64,uint256)` and selector `0x7cbde479`.
```solidity
error NoEpochRootReachesThreshold(uint64 chainId, uint256 epoch);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoEpochRootReachesThreshold {
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
        impl ::core::convert::From<NoEpochRootReachesThreshold>
        for UnderlyingRustTuple<'_> {
            fn from(value: NoEpochRootReachesThreshold) -> Self {
                (value.chainId, value.epoch)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NoEpochRootReachesThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    epoch: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoEpochRootReachesThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoEpochRootReachesThreshold(uint64,uint256)";
            const SELECTOR: [u8; 4] = [124u8, 189u8, 228u8, 121u8];
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
    /**Event with signature `EpochRootPosted(uint64,uint256,address,bytes32)` and selector `0xac3e570e86481985ba4e8e32ee33ccb4cf7365bfccfd2e7e8b5a0535f960ea44`.
```solidity
event EpochRootPosted(uint64 indexed chainId, uint256 indexed epoch, address indexed sender, bytes32 root);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EpochRootPosted {
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
        impl alloy_sol_types::SolEvent for EpochRootPosted {
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
            const SIGNATURE: &'static str = "EpochRootPosted(uint64,uint256,address,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                172u8, 62u8, 87u8, 14u8, 134u8, 72u8, 25u8, 133u8, 186u8, 78u8, 142u8,
                50u8, 238u8, 51u8, 204u8, 180u8, 207u8, 115u8, 101u8, 191u8, 204u8,
                253u8, 46u8, 126u8, 139u8, 90u8, 5u8, 53u8, 249u8, 96u8, 234u8, 68u8,
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
        impl alloy_sol_types::private::IntoLogData for EpochRootPosted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EpochRootPosted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EpochRootPosted) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `postEpochRoot(uint64,uint256,bytes32)` and selector `0xe7069431`.
```solidity
function postEpochRoot(uint64 chainId, uint256 epoch, bytes32 root) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postEpochRootCall {
        #[allow(missing_docs)]
        pub chainId: u64,
        #[allow(missing_docs)]
        pub epoch: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`postEpochRoot(uint64,uint256,bytes32)`](postEpochRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct postEpochRootReturn {}
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
            impl ::core::convert::From<postEpochRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: postEpochRootCall) -> Self {
                    (value.chainId, value.epoch, value.root)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postEpochRootCall {
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
            impl ::core::convert::From<postEpochRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: postEpochRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for postEpochRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl postEpochRootReturn {
            fn _tokenize(
                &self,
            ) -> <postEpochRootCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for postEpochRootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = postEpochRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "postEpochRoot(uint64,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [231u8, 6u8, 148u8, 49u8];
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
                postEpochRootReturn::_tokenize(ret)
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
        postEpochRoot(postEpochRootCall),
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
            [231u8, 6u8, 148u8, 49u8],
            [232u8, 173u8, 53u8, 135u8],
            [245u8, 23u8, 140u8, 207u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(getVersion),
            ::core::stringify!(recordEpochRoot),
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
            ::core::stringify!(postEpochRoot),
            ::core::stringify!(getEpochRoot),
            ::core::stringify!(getEpochOfBlock),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <recordEpochRootCall as alloy_sol_types::SolCall>::SIGNATURE,
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
            <postEpochRootCall as alloy_sol_types::SolCall>::SIGNATURE,
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
                Self::postEpochRoot(_) => {
                    <postEpochRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                    fn postEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <postEpochRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsCalls::postEpochRoot)
                    }
                    postEpochRoot
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
                    fn postEpochRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsCalls> {
                        <postEpochRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsCalls::postEpochRoot)
                    }
                    postEpochRoot
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
                Self::postEpochRoot(inner) => {
                    <postEpochRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::postEpochRoot(inner) => {
                    <postEpochRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        EpochLengthNull(EpochLengthNull),
        #[allow(missing_docs)]
        EpochRootAlreadyPosted(EpochRootAlreadyPosted),
        #[allow(missing_docs)]
        EpochRootAlreadyRecorded(EpochRootAlreadyRecorded),
        #[allow(missing_docs)]
        EpochRootNull(EpochRootNull),
        #[allow(missing_docs)]
        EpochScheduleAlreadySet(EpochScheduleAlreadySet),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        FromEpochTooLow(FromEpochTooLow),
        #[allow(missing_docs)]
        HostChainNullId(HostChainNullId),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        NoEpochRoot(NoEpochRoot),
        #[allow(missing_docs)]
        NoEpochRootReachesThreshold(NoEpochRootReachesThreshold),
        #[allow(missing_docs)]
        NoEpochRootRecorded(NoEpochRootRecorded),
        #[allow(missing_docs)]
        NoEpochSchedule(NoEpochSchedule),
        #[allow(missing_docs)]
        NotCoprocessorTxSender(NotCoprocessorTxSender),
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
    impl CiphertextRootsErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 229u8, 214u8, 2u8],
            [33u8, 191u8, 218u8, 16u8],
            [41u8, 203u8, 221u8, 35u8],
            [43u8, 0u8, 197u8, 150u8],
            [57u8, 160u8, 15u8, 128u8],
            [60u8, 93u8, 141u8, 244u8],
            [76u8, 156u8, 140u8, 227u8],
            [82u8, 215u8, 37u8, 245u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 94u8, 111u8, 20u8],
            [124u8, 189u8, 228u8, 121u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [179u8, 152u8, 151u8, 159u8],
            [188u8, 89u8, 231u8, 151u8],
            [195u8, 33u8, 111u8, 25u8],
            [199u8, 132u8, 245u8, 75u8],
            [203u8, 171u8, 212u8, 126u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 230u8, 188u8, 248u8],
            [224u8, 124u8, 141u8, 186u8],
            [229u8, 9u8, 37u8, 10u8],
            [249u8, 46u8, 232u8, 169u8],
            [253u8, 112u8, 59u8, 24u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(EpochRootAlreadyPosted),
            ::core::stringify!(NotHostOwner),
            ::core::stringify!(NoEpochSchedule),
            ::core::stringify!(NoEpochRoot),
            ::core::stringify!(HostChainNullId),
            ::core::stringify!(EpochScheduleAlreadySet),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(NotCoprocessorTxSender),
            ::core::stringify!(NotInitializingFromEmptyProxy),
            ::core::stringify!(BlockBeforeStartBlock),
            ::core::stringify!(NoEpochRootReachesThreshold),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(EpochLengthNotFound),
            ::core::stringify!(FromEpochTooLow),
            ::core::stringify!(EpochLengthNull),
            ::core::stringify!(NoEpochRootRecorded),
            ::core::stringify!(FailedCall),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(EpochRootNull),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(EpochRootAlreadyRecorded),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <EpochRootAlreadyPosted as alloy_sol_types::SolError>::SIGNATURE,
            <NotHostOwner as alloy_sol_types::SolError>::SIGNATURE,
            <NoEpochSchedule as alloy_sol_types::SolError>::SIGNATURE,
            <NoEpochRoot as alloy_sol_types::SolError>::SIGNATURE,
            <HostChainNullId as alloy_sol_types::SolError>::SIGNATURE,
            <EpochScheduleAlreadySet as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <NotCoprocessorTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SIGNATURE,
            <BlockBeforeStartBlock as alloy_sol_types::SolError>::SIGNATURE,
            <NoEpochRootReachesThreshold as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <EpochLengthNotFound as alloy_sol_types::SolError>::SIGNATURE,
            <FromEpochTooLow as alloy_sol_types::SolError>::SIGNATURE,
            <EpochLengthNull as alloy_sol_types::SolError>::SIGNATURE,
            <NoEpochRootRecorded as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <EpochRootNull as alloy_sol_types::SolError>::SIGNATURE,
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
        const COUNT: usize = 24usize;
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
                Self::EpochLengthNull(_) => {
                    <EpochLengthNull as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochRootAlreadyPosted(_) => {
                    <EpochRootAlreadyPosted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochRootAlreadyRecorded(_) => {
                    <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EpochRootNull(_) => {
                    <EpochRootNull as alloy_sol_types::SolError>::SELECTOR
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
                Self::HostChainNullId(_) => {
                    <HostChainNullId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEpochRoot(_) => {
                    <NoEpochRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEpochRootReachesThreshold(_) => {
                    <NoEpochRootReachesThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEpochRootRecorded(_) => {
                    <NoEpochRootRecorded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEpochSchedule(_) => {
                    <NoEpochSchedule as alloy_sol_types::SolError>::SELECTOR
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
                    fn EpochRootAlreadyPosted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochRootAlreadyPosted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochRootAlreadyPosted)
                    }
                    EpochRootAlreadyPosted
                },
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
                    fn HostChainNullId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <HostChainNullId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::HostChainNullId)
                    }
                    HostChainNullId
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
                    fn NoEpochRootReachesThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochRootReachesThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoEpochRootReachesThreshold)
                    }
                    NoEpochRootReachesThreshold
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
                    fn EpochLengthNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochLengthNull as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochLengthNull)
                    }
                    EpochLengthNull
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
                    fn EpochRootNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochRootNull as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochRootNull)
                    }
                    EpochRootNull
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
                    fn EpochRootAlreadyPosted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochRootAlreadyPosted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochRootAlreadyPosted)
                    }
                    EpochRootAlreadyPosted
                },
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
                    fn HostChainNullId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <HostChainNullId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::HostChainNullId)
                    }
                    HostChainNullId
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
                    fn NoEpochRootReachesThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <NoEpochRootReachesThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::NoEpochRootReachesThreshold)
                    }
                    NoEpochRootReachesThreshold
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
                    fn EpochLengthNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochLengthNull as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochLengthNull)
                    }
                    EpochLengthNull
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
                    fn EpochRootNull(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CiphertextRootsErrors> {
                        <EpochRootNull as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CiphertextRootsErrors::EpochRootNull)
                    }
                    EpochRootNull
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
                Self::EpochLengthNull(inner) => {
                    <EpochLengthNull as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EpochRootAlreadyPosted(inner) => {
                    <EpochRootAlreadyPosted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EpochRootAlreadyRecorded(inner) => {
                    <EpochRootAlreadyRecorded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EpochRootNull(inner) => {
                    <EpochRootNull as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::HostChainNullId(inner) => {
                    <HostChainNullId as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NoEpochRootReachesThreshold(inner) => {
                    <NoEpochRootReachesThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::EpochLengthNull(inner) => {
                    <EpochLengthNull as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EpochRootAlreadyPosted(inner) => {
                    <EpochRootAlreadyPosted as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EpochRootNull(inner) => {
                    <EpochRootNull as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::HostChainNullId(inner) => {
                    <HostChainNullId as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NoEpochRootReachesThreshold(inner) => {
                    <NoEpochRootReachesThreshold as alloy_sol_types::SolError>::abi_encode_raw(
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
        EpochRootPosted(EpochRootPosted),
        #[allow(missing_docs)]
        EpochRootRecorded(EpochRootRecorded),
        #[allow(missing_docs)]
        EpochScheduleSet(EpochScheduleSet),
        #[allow(missing_docs)]
        Initialized(Initialized),
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
                172u8, 62u8, 87u8, 14u8, 134u8, 72u8, 25u8, 133u8, 186u8, 78u8, 142u8,
                50u8, 238u8, 51u8, 204u8, 180u8, 207u8, 115u8, 101u8, 191u8, 204u8,
                253u8, 46u8, 126u8, 139u8, 90u8, 5u8, 53u8, 249u8, 96u8, 234u8, 68u8,
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
            ::core::stringify!(EpochScheduleSet),
            ::core::stringify!(EpochLengthRemoved),
            ::core::stringify!(EpochRootRecorded),
            ::core::stringify!(EpochLengthSet),
            ::core::stringify!(EpochRootPosted),
            ::core::stringify!(Upgraded),
            ::core::stringify!(Initialized),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <EpochScheduleSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochLengthRemoved as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochRootRecorded as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochLengthSet as alloy_sol_types::SolEvent>::SIGNATURE,
            <EpochRootPosted as alloy_sol_types::SolEvent>::SIGNATURE,
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
                Some(<EpochRootPosted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EpochRootPosted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::EpochRootPosted)
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
                Self::EpochRootPosted(inner) => {
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
                Self::EpochRootPosted(inner) => {
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
        ///Creates a new call builder for the [`postEpochRoot`] function.
        pub fn postEpochRoot(
            &self,
            chainId: u64,
            epoch: alloy::sol_types::private::primitives::aliases::U256,
            root: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, postEpochRootCall, N> {
            self.call_builder(
                &postEpochRootCall {
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
        ///Creates a new event filter for the [`EpochRootPosted`] event.
        pub fn EpochRootPosted_filter(
            &self,
        ) -> alloy_contract::Event<&P, EpochRootPosted, N> {
            self.event_filter::<EpochRootPosted>()
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
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
