///Module containing a contract's types and functions.
/**

```solidity
library IDecryption {
    struct ContractsInfo { uint256 chainId; address[] addresses; }
    struct DelegationAccounts { address delegatorAddress; address delegateAddress; }
    struct RequestValidity { uint256 startTimestamp; uint256 durationDays; }
    struct RequestValiditySeconds { uint256 startTimestamp; uint256 durationSeconds; }
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct ContractsInfo { uint256 chainId; address[] addresses; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractsInfo {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub addresses: alloy::sol_types::private::Vec<
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<ContractsInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ContractsInfo) -> Self {
                (value.chainId, value.addresses)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContractsInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    addresses: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ContractsInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ContractsInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.addresses),
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
        impl alloy_sol_types::SolType for ContractsInfo {
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
        impl alloy_sol_types::SolStruct for ContractsInfo {
            const NAME: &'static str = "ContractsInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ContractsInfo(uint256 chainId,address[] addresses)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.addresses)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ContractsInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addresses,
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addresses,
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
struct DelegationAccounts { address delegatorAddress; address delegateAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelegationAccounts {
        #[allow(missing_docs)]
        pub delegatorAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delegateAddress: alloy::sol_types::private::Address,
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
                (value.delegatorAddress, value.delegateAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegationAccounts {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delegatorAddress: tuple.0,
                    delegateAddress: tuple.1,
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
                        &self.delegateAddress,
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
                    "DelegationAccounts(address delegatorAddress,address delegateAddress)",
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
                            &self.delegateAddress,
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
                        &rust.delegateAddress,
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
                    &rust.delegateAddress,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct RequestValiditySeconds { uint256 startTimestamp; uint256 durationSeconds; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequestValiditySeconds {
        #[allow(missing_docs)]
        pub startTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub durationSeconds: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<RequestValiditySeconds> for UnderlyingRustTuple<'_> {
            fn from(value: RequestValiditySeconds) -> Self {
                (value.startTimestamp, value.durationSeconds)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequestValiditySeconds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    startTimestamp: tuple.0,
                    durationSeconds: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RequestValiditySeconds {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RequestValiditySeconds {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.durationSeconds),
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
        impl alloy_sol_types::SolType for RequestValiditySeconds {
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
        impl alloy_sol_types::SolStruct for RequestValiditySeconds {
            const NAME: &'static str = "RequestValiditySeconds";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RequestValiditySeconds(uint256 startTimestamp,uint256 durationSeconds)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.durationSeconds,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RequestValiditySeconds {
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
                        &rust.durationSeconds,
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
                    &rust.durationSeconds,
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDecryptionInstance<P, N> {
        IDecryptionInstance::<P, N>::new(address, provider)
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
    pub struct IDecryptionInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IDecryptionInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDecryptionInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<P, N> {
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
    impl<P: ::core::clone::Clone, N> IDecryptionInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDecryptionInstance<P, N> {
            IDecryptionInstance {
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
    > IDecryptionInstance<P, N> {
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
    > IDecryptionInstance<P, N> {
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
library IDecryption {
    struct ContractsInfo {
        uint256 chainId;
        address[] addresses;
    }
    struct DelegationAccounts {
        address delegatorAddress;
        address delegateAddress;
    }
    struct RequestValidity {
        uint256 startTimestamp;
        uint256 durationDays;
    }
    struct RequestValiditySeconds {
        uint256 startTimestamp;
        uint256 durationSeconds;
    }
}

interface Decryption {
    type FheType is uint8;
    struct CtHandleContractPair {
        bytes32 ctHandle;
        address contractAddress;
    }
    struct HandleEntry {
        bytes32 handle;
        address contractAddress;
        address ownerAddress;
    }
    struct SnsCiphertextMaterial {
        bytes32 ctHandle;
        uint256 keyId;
        bytes32 snsCiphertextDigest;
        address[] coprocessorTxSenderAddresses;
    }

    error AddressEmptyCode(address target);
    error ContractAddressesMaxLengthExceeded(uint256 maxLength, uint256 actualLength);
    error ContractNotInContractAddresses(address contractAddress, address[] contractAddresses);
    error CoprocessorSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error CtHandleChainIdDiffersFromContractChainId(bytes32 ctHandle, uint256 chainId, uint256 contractChainId);
    error DecryptionNotRequested(uint256 decryptionId);
    error DelegatorAddressInContractAddresses(address delegatorAddress, address[] contractAddresses);
    error DifferentKeyIdsNotAllowed(SnsCiphertextMaterial firstSnsCtMaterial, SnsCiphertextMaterial invalidSnsCtMaterial);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyContractAddresses();
    error EmptyCtHandleContractPairs();
    error EmptyCtHandles();
    error EmptyHandles();
    error EnforcedPause();
    error ExpectedPause();
    error FailedCall();
    error HostChainNotRegistered(uint256 chainId);
    error InvalidExtraDataLength(uint256 length, uint256 minimumLength);
    error InvalidFHEType(uint8 fheTypeUint8);
    error InvalidInitialization();
    error InvalidNullDurationDays();
    error InvalidNullDurationSeconds();
    error InvalidUserSignature(bytes signature);
    error KmsNodeAlreadySigned(uint256 decryptionId, address signer);
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error MaxDecryptionRequestBitSizeExceeded(uint256 maxBitSize, uint256 totalBitSize);
    error MaxDurationDaysExceeded(uint256 maxValue, uint256 actualValue);
    error MaxDurationSecondsExceeded(uint256 maxValue, uint256 actualValue);
    error NotCoprocessorSigner(address signerAddress);
    error NotCoprocessorTxSender(address txSenderAddress);
    error NotCustodianSigner(address signerAddress);
    error NotCustodianTxSender(address txSenderAddress);
    error NotGatewayOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsSigner(address signerAddress);
    error NotKmsTxSender(address txSenderAddress);
    error NotOwnerOrGatewayConfig(address notOwnerOrGatewayConfig);
    error NotPauserOrGatewayConfig(address notPauserOrGatewayConfig);
    error StartTimestampInFuture(uint256 currentTimestamp, uint256 startTimestamp);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnsupportedExtraDataVersion(uint8 version);
    error UnsupportedFHEType(FheType fheType);
    error UserAddressInContractAddresses(address userAddress, address[] contractAddresses);
    error UserDecryptionRequestExpired(uint256 currentTimestamp, IDecryption.RequestValidity requestValidity);
    error UserDecryptionRequestExpiredSeconds(uint256 currentTimestamp, IDecryption.RequestValiditySeconds requestValidity);

    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event Paused(address account);
    event PublicDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, bytes extraData);
    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures, bytes extraData);
    event PublicDecryptionResponseCall(uint256 indexed decryptionId, bytes decryptedResult, bytes signature, address kmsTxSender, bytes extraData);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);
    event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey, bytes extraData);
    event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, HandleEntry[] handles, address userAddress, bytes publicKey, address[] allowedContracts, IDecryption.RequestValiditySeconds requestValidity, bytes signature, bytes extraData);
    event UserDecryptionResponse(uint256 indexed decryptionId, uint256 indexShare, bytes userDecryptedShare, bytes signature, bytes extraData);
    event UserDecryptionResponseThresholdReached(uint256 indexed decryptionId);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function delegatedUserDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, IDecryption.DelegationAccounts memory delegationAccounts, IDecryption.ContractsInfo memory contractsInfo, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getDecryptionConsensusTxSenders(uint256 decryptionId) external view returns (address[] memory);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function isDecryptionDone(uint256 decryptionId) external view returns (bool);
    function isDelegatedUserDecryptionReady(CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view returns (bool);
    function isPublicDecryptionReady(bytes32[] memory ctHandles, bytes memory) external view returns (bool);
    function isUserDecryptionReady(HandleEntry[] memory handles, bytes memory) external view returns (bool);
    function isUserDecryptionReady(CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view returns (bool);
    function isUserDecryptionReady(address, CtHandleContractPair[] memory ctHandleContractPairs, bytes memory extraData) external view returns (bool);
    function pause() external;
    function paused() external view returns (bool);
    function proxiableUUID() external view returns (bytes32);
    function publicDecryptionRequest(bytes32[] memory ctHandles, bytes memory extraData) external;
    function publicDecryptionResponse(uint256 decryptionId, bytes memory decryptedResult, bytes memory signature, bytes memory extraData) external;
    function reinitializeV4() external;
    function unpause() external;
    function upgradeToAndCall(address newImplementation, bytes memory data) external payable;
    function userDecryptionRequest(HandleEntry[] memory handles, address userAddress, bytes memory publicKey, address[] memory allowedContracts, IDecryption.RequestValiditySeconds memory requestValidity, bytes memory signature, bytes memory extraData) external;
    function userDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, IDecryption.ContractsInfo memory contractsInfo, address userAddress, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
    function userDecryptionResponse(uint256 decryptionId, bytes memory userDecryptedShare, bytes memory signature, bytes memory extraData) external;
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
        "internalType": "struct IDecryption.DelegationAccounts",
        "components": [
          {
            "name": "delegatorAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "delegateAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "contractsInfo",
        "type": "tuple",
        "internalType": "struct IDecryption.ContractsInfo",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "addresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
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
      },
      {
        "name": "extraData",
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
    "name": "getDecryptionConsensusTxSenders",
    "inputs": [
      {
        "name": "decryptionId",
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
    "name": "isDecryptionDone",
    "inputs": [
      {
        "name": "decryptionId",
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
    "name": "isDelegatedUserDecryptionReady",
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
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "isPublicDecryptionReady",
    "inputs": [
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "isUserDecryptionReady",
    "inputs": [
      {
        "name": "handles",
        "type": "tuple[]",
        "internalType": "struct HandleEntry[]",
        "components": [
          {
            "name": "handle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ownerAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "isUserDecryptionReady",
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
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "isUserDecryptionReady",
    "inputs": [
      {
        "name": "",
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
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "pause",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [],
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
    "name": "publicDecryptionRequest",
    "inputs": [
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
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
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "reinitializeV4",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpause",
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
    "type": "function",
    "name": "userDecryptionRequest",
    "inputs": [
      {
        "name": "handles",
        "type": "tuple[]",
        "internalType": "struct HandleEntry[]",
        "components": [
          {
            "name": "handle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ownerAddress",
            "type": "address",
            "internalType": "address"
          }
        ]
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
        "name": "allowedContracts",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValiditySeconds",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationSeconds",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
        "name": "contractsInfo",
        "type": "tuple",
        "internalType": "struct IDecryption.ContractsInfo",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "addresses",
            "type": "address[]",
            "internalType": "address[]"
          }
        ]
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
      },
      {
        "name": "extraData",
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
      },
      {
        "name": "extraData",
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
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
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
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PublicDecryptionResponseCall",
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
        "name": "signature",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "kmsTxSender",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
        "name": "handles",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct HandleEntry[]",
        "components": [
          {
            "name": "handle",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "contractAddress",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "ownerAddress",
            "type": "address",
            "internalType": "address"
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
      },
      {
        "name": "allowedContracts",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDecryption.RequestValiditySeconds",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationSeconds",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "signature",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "extraData",
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
        "name": "indexShare",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "userDecryptedShare",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      },
      {
        "name": "extraData",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UserDecryptionResponseThresholdReached",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
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
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "CoprocessorSignerDoesNotMatchTxSender",
    "inputs": [
      {
        "name": "signerAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "txSenderAddress",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "CtHandleChainIdDiffersFromContractChainId",
    "inputs": [
      {
        "name": "ctHandle",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "contractChainId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "DecryptionNotRequested",
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
    "name": "EmptyContractAddresses",
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
    "name": "EmptyHandles",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EnforcedPause",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpectedPause",
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
    "name": "InvalidExtraDataLength",
    "inputs": [
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "minimumLength",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
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
    "name": "InvalidNullDurationSeconds",
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
    "name": "KmsSignerDoesNotMatchTxSender",
    "inputs": [
      {
        "name": "signerAddress",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "txSenderAddress",
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
    "name": "MaxDurationSecondsExceeded",
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
    "name": "NotOwnerOrGatewayConfig",
    "inputs": [
      {
        "name": "notOwnerOrGatewayConfig",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "NotPauserOrGatewayConfig",
    "inputs": [
      {
        "name": "notPauserOrGatewayConfig",
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
    "name": "UnsupportedExtraDataVersion",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "internalType": "uint8"
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
  },
  {
    "type": "error",
    "name": "UserDecryptionRequestExpiredSeconds",
    "inputs": [
      {
        "name": "currentTimestamp",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requestValidity",
        "type": "tuple",
        "internalType": "struct IDecryption.RequestValiditySeconds",
        "components": [
          {
            "name": "startTimestamp",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "durationSeconds",
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
    ///0x60a08060405234620000d157306080527ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a009081549060ff8260401c16620000c257506001600160401b036002600160401b0319828216016200007c575b604051614c769081620000d682396080518181816122e001526123920152f35b6001600160401b031990911681179091556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602090a15f80806200005c565b63f92ee8a960e01b8152600490fd5b5f80fdfe60a0806040526004361015610012575f80fd5b5f905f3560e01c908163046f9eb314612ece575080630900cc6914612dcc5780630d8e6e2c14612ca5578063123abb2814612c0d57806339f73810146127b15780633f4ba83a1461267b5780634014c4cd14612660578063410bf0ba146126075780634f1ef2861461234257806352d1902d146122c657806358f5b8ab146122785780635c975abb146122375780636f8913bc14611c7e57806376227eed146109245780638456cb5914611b7e57806384b0196e14611a075780639fad5a2f1461132a578063ad3cb1cc146112c9578063b4de2c3714610c2d578063d8998f4514610929578063e22d1b2614610924578063f1b57adb1461018f5763fbb832591461011b575f80fd5b3461018857606036600319011261018857610134613446565b506001600160401b039060243582811161018b5761015690369060040161357c565b92909160443591821161018857602061017e858561017736600488016132c5565b5050613b35565b6040519015158152f35b80fd5b5080fd5b5034610188576003196101003682011261018b576004356001600160401b038111610920576101c290369060040161357c565b90916040366023190112610918576001600160401b03606435116109185760409060643536030112610920576001600160a01b03608435166084350361091c5760a4356001600160401b038111610918576102219036906004016132c5565b909160c4356001600160401b038111610914576102429036906004016132c5565b949060e4356001600160401b038111610910576102639036906004016132c5565b93909261026e6143f8565b604051635ff9d55d60e11b815260046064358101359082015260208160248173d582ec82a1758322907df80da8a754e12a5acb955afa908115610905578a916108d6575b50156108b9576102cc602460643501606435600401613bde565b9050156108a757600a6102e9602460643501606435600401613bde565b905011610873576103016102fc36613c13565b614442565b61032d61032861031b602460643501606435600401613bde565b9190608435923691613c67565b61450c565b61083f576103419160643560040191614577565b95610356602460643501606435600401613bde565b604051918260a08101106001600160401b0360a08501111761082b578261039d61054a9261055b9460a061056497016040526103948d8d3691613546565b84523691613c67565b60208201908152602435604083015260443560608301526103bf368a8a613546565b608083015260876040516103d2816134d4565b8181527f726144617461290000000000000000000000000000000000000000000000000060a060208301927f557365724465637279707452657175657374566572696669636174696f6e286284527f79746573207075626c69634b65792c616464726573735b5d20636f6e7472616360408201527f744164647265737365732c75696e7432353620737461727454696d657374616d60608201527f702c75696e74323536206475726174696f6e446179732c6279746573206578746080820152015220916104b76104c582516020815191012093516040519283916020830190614989565b03601f19810183528261350a565b6020815191012090604081015160806060830151920151604051610506602082816104f9818301968781519384920161335a565b810103808452018261350a565b51902092604051946020860196875260408601526060850152608084015260a083015260c082015260c0815261053b816134ef565b51902060643560040135614a67565b610555368587613546565b906149be565b909291926149f8565b6001600160a01b03806084351691160361080357505060405163a14f897160e01b8152928684806105988960048301613e6b565b038173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa9384156107f85787946107d4575b506105c98461469a565b6105f37f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70854613ea6565b95867f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee708556040519061062482613470565b61062f368489613546565b825260208201528688527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee707602052604088209080518051906001600160401b038211610752576106898261068386546135fe565b86613ae2565b602090601f8311600114610771576106b892918c9183610766575b50508160011b915f199060031b1c19161790565b82555b60206001809301910151908151916001600160401b038311610752576020906106e48484613eb4565b01908a5260208a208a5b83811061073f578b8b7ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b8c8c6107398d8d8d6107293361474b565b6040519586956084359087613fb7565b0390a280f35b84906020845194019381840155016106ee565b634e487b7160e01b8b52604160045260248bfd5b015190505f806106a4565b848c5260208c209190601f1984168d5b8181106107bc57509084600195949392106107a4575b505050811b0182556106bb565b01515f1960f88460031b161c191690555f8080610797565b92936020600181928786015181550195019301610781565b6107f19194503d8089833e6107e9818361350a565b810190613d25565b925f6105bf565b6040513d89823e3d90fd5b610827604051928392632a873d2760e01b8452602060048501526024840191613904565b0390fd5b634e487b7160e01b5f52604160045260245ffd5b610853602460643501606435600401613bde565b60405163dc4d78b160e01b81529182916108279160843560048501613d02565b6044610889602460643501606435600401613bde565b90506040519063af1f049560e01b8252600a60048301526024820152fd5b6040516357cfa21760e01b8152600490fd5b602460405163b6679c3b60e01b8152606435600401356004820152fd5b6108f8915060203d6020116108fe575b6108f0818361350a565b810190613971565b5f6102b2565b503d6108e6565b6040513d8c823e3d90fd5b8780fd5b8580fd5b8380fd5b5f80fd5b8280fd5b6135ac565b503461091c57610938366133d0565b919390926109446143f8565b8415610c1b5761095385613bc7565b92610961604051948561350a565b8584526020808501918760051b928385019036821161091c578386915b838310610c0b57505050505f965f975b87518910156109ca576109c260019161ffff6109bb6109b66109b08e8e6144f8565b516148b4565b6148fa565b1690614435565b98019761098e565b899061080090818111610bed5750506040519463a14f897160e01b86528460048701528160248701527f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821161091c5785604481835f948b84840137810103018173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa948515610bb4575f95610bd1575b50610a5a8561469a565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70695610a868754613ea6565b809755865f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee705855260405f20906001600160401b03831161082b57610acc8383613eb4565b905f52845f205f5b838110610bbf57505050507333e0c7a03d2b040b518580c365f4b3bde7cc4e6e803b1561091c575f809160246040518094819363247bac9f60e21b83523360048401525af18015610bb457610b71575b50610b64927f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff949261073992604051958695604087526040870190613f5b565b9285840390860152613904565b610739919650927f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff9492610ba7610b64956134a6565b5f97925092945092610b24565b6040513d5f823e3d90fd5b82358282015591860191600101610ad4565b610be69195503d805f833e6107e9818361350a565b9387610a50565b604492506040519163e7f4895d60e01b835260048301526024820152fd5b823581529181019185910161097e565b6040516305bcea8760e31b8152600490fd5b3461091c5761010036600319011261091c576004356001600160401b03811161091c57610c5e903690600401613416565b906024356001600160a01b038116810361091c576044356001600160401b03811161091c57610c919036906004016132c5565b6064356001600160401b03811161091c57610cb09036906004016133a0565b90604036608319011261091c5760c4356001600160401b03811161091c57610cdc9036906004016132c5565b9290966001600160401b0360e4351161091c57610cfe3660e4356004016132c5565b959096610d096143f8565b8a156112b757600a841161129757604051610d2381613470565b608435815260a435602082015260a435156112855760208101516301e1338090818111611267575050805142811161124957508051610d684291602084015190614435565b1061121f5750610d773361474b565b610d808b614545565b9860109b604051635ff9d55d60e11b81526001600160401b03863560101c16600482015260208160248173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f91611200575b50156111db575f9c5f6080525b816080511061113a5750610800808e1161111c575060405163a14f897160e01b81529b9c50999a995f8b80610e148f60048301613e6b565b038173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa9a8b15610bb4575f9b611100575b50610e458b61469a565b610e6f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70854613ea6565b9b8c7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70855604051610e9f81613470565b610eaa368787613546565b8152602081019182528d5f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70760205260405f2090518051906001600160401b03821161082b57610f0582610eff85546135fe565b85613ae2565b602090601f8311600114611099579180610f3792600195945f926107665750508160011b915f199060031b1c19161790565b81555b019051908151916001600160401b03831161082b57602090610f5c8484613eb4565b01905f5260205f205f5b83811061108557505050506020610f896040519c8d610120908181520190613f5b565b8c8103828e01528281520194905f5b818110611038575050509461102494610ff87f044711c64cf18abf4960220e3e1ecf01059c72b8152c8415b58ea1fdd84a2b4b9c9d958c9b9995611006958d60406001600160a01b036110339f9d169101528d6060818503910152613904565b918a830360808c0152613cbd565b9160843560a089015260a43560c089015287830360e0890152613904565b91848303610100860152613904565b0390a2005b909195600190873581526001600160a01b0361105660208a0161345c565b1660208201526001600160a01b0361107060408a0161345c565b16604082015260609081019701929101610f98565b600190602084519401938184015501610f66565b90601f19831691845f5260205f20925f5b8181106110e857509160019594929183879593106110d0575b505050811b018155610f3a565b01515f1960f88460031b161c191690555f80806110c3565b929360206001819287860151815501950193016110aa565b611115919b503d805f833e6107e9818361350a565b998d610e3b565b6044908e6040519163e7f4895d60e01b835260048301526024820152fd5b6001600160401b039d6111506080518489613a2a565b359e8f831c166001600160401b03883560101c16810361119c575061117f908f6109bb6109b661ffff926148b4565b9d61118d8d608051906144f8565b52600160805101608052610ddc565b8f61082789916001600160401b0393604051948594634ac8748b60e11b86523560101c1691600485016040919493926060820195825260208201520152565b60405163b6679c3b60e01b8152853560101c6001600160401b03166004820152602490fd5b611219915060203d6020116108fe576108f0818361350a565b8e610dcf565b6040516333c7e7e760e11b8152426004820152815160248201526020909101516044820152606490fd5b6044906040519063f24c088760e01b82524260048301526024820152fd5b6044925060405191635729758960e11b835260048301526024820152fd5b604051631229e23760e21b8152600490fd5b60405163af1f049560e01b8152600a600482015260248101859052604490fd5b60405163240e930960e01b8152600490fd5b3461091c575f36600319011261091c576113266040516112e881613470565b600581527f352e302e30000000000000000000000000000000000000000000000000000000602082015260405191829160208352602083019061337b565b0390f35b3461091c576003196101203682011261091c576004356001600160401b03811161091c5761135c90369060040161357c565b9091604036602319011261091c57604036606319011261091c576001600160401b0360a4351161091c5760409060a4353603011261091c5760c4356001600160401b03811161091c576113b39036906004016132c5565b60e4356001600160401b03811161091c576113d29036906004016132c5565b9490610104356001600160401b03811161091c576113f49036906004016132c5565b9690926113ff6143f8565b604051635ff9d55d60e11b8152600460a4358101359082015260208160248173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f916119e8575b50156119cb57602460a435019661145f8860a435600401613bde565b9050156108a757600a6114778960a435600401613bde565b9050116119ba5761148a6102fc36613c13565b6114b161032861149f8a60a435600401613bde565b91906114a9613c3b565b923691613c67565b611983576114d5916114c99160a43560040191614577565b9660a435600401613bde565b906114de613c3b565b604051928360c08101106001600160401b0360c08601111761082b576001600160a01b03926115229160c08601604052611519368b8d613546565b86523691613c67565b602084015216604082015260243560608201526044356080820152611548368986613546565b60a0820152611555613c51565b60a9604051611563816134ef565b8181527f787472614461746129000000000000000000000000000000000000000000000060c060208301927f44656c656761746564557365724465637279707452657175657374566572696684527f69636174696f6e286279746573207075626c69634b65792c616464726573735b60408201527f5d20636f6e74726163744164647265737365732c616464726573732064656c6560608201527f6761746f72416464726573732c75696e7432353620737461727454696d65737460808201527f616d702c75696e74323536206475726174696f6e446179732c6279746573206560a082015201522091805160208151910120906104b761167260208301516040519283916020830190614989565b602081519101206001600160a01b0360408301511660608301519160a060808501519401516040516116b4602082816104f9818301968781519384920161335a565b5190209460405197602089015260408801526060870152608086015260a085015260c084015260e083015260e08252816101008101106001600160401b036101008401111761082b576001600160a01b03809161173761172c8561010061174097016040526020815191012060a43560040135614a67565b61055536888a613546565b909591956149f8565b1691160361080357505060405163a14f897160e01b8152915f83806117688860048301613e6b565b038173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315610bb4575f93611967575b506117998361469a565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee708946117c58654613ea6565b809655604051906117d582613470565b6117e0368488613546565b825260208201908152865f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70760205260405f2091518051906001600160401b03821161082b576118358261068386546135fe565b602090601f83116001146119035761186392915f91836118f85750508160011b915f199060031b1c19161790565b82555b60018092019051908151916001600160401b03831161082b5760209061188c8484613eb4565b01905f5260205f205f5b8381106118e557897ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b8a8a6110338f8c8c6118d03361474b565b6118d8613c51565b9560405196879687613fb7565b8490602084519401938184015501611896565b015190508b806106a4565b90601f19831691855f5260205f20925f5b81811061194f5750908460019594939210611937575b505050811b018255611866565b01515f1960f88460031b161c191690558a808061192a565b92936020600181928786015181550195019301611914565b61197c9193503d805f833e6107e9818361350a565b918661178f565b6108278861199e611992613c3b565b9160a435600401613bde565b60405163c3446ac760e01b815293849390929060048501613d02565b60446108898960a435600401613bde565b602460405163b6679c3b60e01b815260a435600401356004820152fd5b611a01915060203d6020116108fe576108f0818361350a565b89611443565b3461091c575f36600319011261091c577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100541580611b55575b15611b1057604051611a5c81611a5581613636565b038261350a565b604051611a6c81611a558161370c565b6040516020808201928284106001600160401b0385111761082b57916020611ac58594611ab797966040525f8452604051978897600f60f81b895260e0858a015260e089019061337b565b90878203604089015261337b565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110611af957505050500390f35b835185528695509381019392810192600101611aea565b60405162461bcd60e51b815260206004820152601560248201527f4549503731323a20556e696e697469616c697a656400000000000000000000006044820152606490fd5b507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1015415611a40565b3461091c575f36600319011261091c5760405163237dfb4760e11b815233600482015273d582ec82a1758322907df80da8a754e12a5acb9590602081602481855afa908115610bb4575f91611c5f575b50159081611c54575b50611c3c57611be46143f8565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300600160ff198254161790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b60405163388916bb60e01b8152336004820152602490fd5b905033141581611bd7565b611c78915060203d6020116108fe576108f0818361350a565b82611bce565b3461091c57611c8c366132f2565b929394600160f89392931b871180159061220d575b6121f457865f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70560205260405f20604051606081018181106001600160401b0382111761082b57611e2292611cf99160405261384a565b8152611d06368985613546565b6020820152611d16368787613546565b60408201526054604051611d298161348b565b8181527f756c742c62797465732065787472614461746129000000000000000000000000606060208301927f5075626c696344656372797074566572696669636174696f6e2862797465733384527f325b5d20637448616e646c65732c6279746573206465637279707465645265736040820152015220906104b7611db982516040519283916020830190613fff565b60208151910120906040602082015160208151910120910151604051611def602082816104f9818301968781519384920161335a565b51902090604051926020840194855260408401526060830152608082015260808152611e1a816134b9565b5190206147a5565b611e2c858561402b565b92611e3a8188848c88614141565b885f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70460205260405f20825f5260205260405f209687546801000000000000000081101561082b57806001611e9392018a558961389a565b9190916121e1576001600160401b03831161082b578282611ec08d95611eba8e96546135fe565b83613ae2565b5f601f831160011461214957611f719383611f9493611f1a827f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e59997611f7f965f9161213e575b508160011b915f199060031b1c19161790565b90555b875f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70260205260405f20895f52602052611f5c60405f2033906138c3565b6040519586956080875260808701908c613904565b918583036020870152613904565b33604084015282810360608401528a8a613904565b0390a2875f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700928360205260ff60405f2054161590816120bf575b50611fd757005b61202b92885f5260205260405f20600160ff198254161790557f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70360205260405f205560405195606087526060870191613904565b84810360208601528354808252602082019160208260051b820101955f5260205f20925f915b83831061209357505050508484036040860152507fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f9392839261103392613904565b9091929396602060016120b08193601f198682030187528b6137ba565b99019301930191939290612051565b90508654604051916361d5552d60e11b8352600483015260208260248173d582ec82a1758322907df80da8a754e12a5acb955afa918215610bb4575f9261210a575b50101589611fd0565b9091506020813d602011612136575b816121266020938361350a565b8101031261091c5751908a612101565b3d9150612119565b90508501355f611f07565b815f5260205f20905f5b601f19851681106121c357509383611f9493611f7f93611f71977f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e59997601f198116106121aa575b5050600182811b019055611f1d565b8401355f19600385901b60f8161c191690555f8061219b565b8582013583558f97508e965060019092019160209182019101612153565b634e487b7160e01b5f525f60045260245ffd5b604051636a457ca160e11b815260048101889052602490fd5b507f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee706548711611ca1565b3461091c575f36600319011261091c57602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b3461091c57602036600319011261091c576004355f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700602052602060ff60405f2054166040519015158152f35b3461091c575f36600319011261091c576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630036123305760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b60405163703e46dd60e11b8152600490fd5b604036600319011261091c57612356613446565b60249081356001600160401b03811161091c573660238201121561091c576123879036908481600401359101613546565b6001600160a01b03807f0000000000000000000000000000000000000000000000000000000000000000168030149081156125d9575b5061233057604051638da5cb5b60e01b815260209190828160048173d582ec82a1758322907df80da8a754e12a5acb955afa8015610bb45782915f916125a1575b5016330361258a578316926040516352d1902d60e01b81528281600481885afa5f918161255b575b5061244357604051634c9c8ce360e01b8152600481018690528690fd5b8490867f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc918281036125465750833b156125305750817fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055604051907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a283511561251657505f80848461250b96519101845af4903d1561250d573d6124ef8161352b565b906124fd604051928361350a565b81525f81943d92013e614b54565b005b60609250614b54565b925050503461252157005b63b398979f60e01b8152600490fd5b604051634c9c8ce360e01b815260048101849052fd5b60405190632a87526960e21b82526004820152fd5b9091508381813d8311612583575b612573818361350a565b8101031261091c57519087612426565b503d612569565b604051630e56cf3d60e01b81523360048201528590fd5b809250848092503d83116125d2575b6125ba818361350a565b8101031261091c576125cc829161395d565b876123fe565b503d6125b0565b9050817f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc54161415856123bd565b3461091c57604036600319011261091c576001600160401b0360043581811161091c57612638903690600401613416565b60249291923591821161091c5760209261265961017e9336906004016132c5565b5050613a3a565b3461091c57602061017e612673366133d0565b505090613989565b3461091c575f36600319011261091c57604051638da5cb5b60e01b815273d582ec82a1758322907df80da8a754e12a5acb9590602081600481855afa8015610bb4575f90612771575b6001600160a01b039150163314159081612766575b5061274e577fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300805460ff81161561273c5760ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b604051638dfc202b60e01b8152600490fd5b6040516370c8b37760e11b8152336004820152602490fd5b9050331415816126d9565b506020813d6020116127a9575b8161278b6020938361350a565b8101031261091c576127a46001600160a01b039161395d565b6126c4565b3d915061277e565b3461091c575f36600319011261091c577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0080546001916001600160401b03918281165f198101612bfb5760ff8260401c16908115612bef575b50612bdd5768ffffffffffffffffff191668010000000000000005178155612830613924565b926040519261283e84613470565b818452602093603160f81b85820152612855614873565b61285d614873565b855182811161082b577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1029061289282546135fe565b97601f98898111612b93575b508790898311600114612b17576128cb92915f9183612b0c5750508160011b915f199060031b1c19161790565b90555b805191821161082b577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1039261290384546135fe565b878111612ab8575b5085968311600114612a1e575090807fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29661295a935f92612a135750508160011b915f199060031b1c19161790565b90555b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100555f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d101556129ab614873565b600160f81b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70655600160f91b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70855805468ff00000000000000001916905560405160058152a1005b0151905087806106a4565b9190601f19821696845f527f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b75915f5b898110612aa35750837fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29910612a8b575b505050811b01905561295d565b01515f1960f88460031b161c19169055868080612a7e565b81830151845592850192918801918801612a4d565b612afd90855f527f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b758980870160051c8201928a8810612b03575b0160051c0190613acc565b8761290b565b92508192612af2565b015190508a806106a4565b869291601f19831691855f527f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d925f5b8c828210612b7d5750508411612b65575b505050811b0190556128ce565b01515f1960f88460031b161c19169055898080612b58565b8385015186558b97909501949384019301612b47565b612bd790845f527f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d8b80860160051c8201928c8710612b03570160051c0190613acc565b8961289e565b60405163f92ee8a960e01b8152600490fd5b6005915010158561280a565b604051636f4f731f60e01b8152600490fd5b3461091c575f36600319011261091c577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460ff8160401c168015612c91575b612bdd5760059068ffffffffffffffffff19161790557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160058152a1005b5060056001600160401b0382161015612c4e565b3461091c575f36600319011261091c57612cbd613924565b612cc5614396565b90600460405191612cd583613470565b60019260018152602093848201938536863760218301825b612d96575b50505090612d8292602492612d05614396565b916040519784612d1e8a965180928b808a01910161335a565b850161103b60f11b89820152612d3d825180938b60228501910161335a565b01612d5a601760f91b93846022840152518093602384019061335a565b01906023820152612d738251809388878501910161335a565b0103600481018552018361350a565b61132660405192828493845283019061337b565b5f190190600a906f181899199a1a9b1b9c1cb0b131b232b360811b8282061a835304918215612dc757919082612ced565b612cf2565b3461091c5760208060031936011261091c576004355f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee703815260405f20547f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee702825260405f20905f52815260405f20604051908183825491828152019081925f52845f20905f5b86828210612eb1578686612e698288038361350a565b60405192839281840190828552518091526040840192915f5b828110612e9157505050500390f35b83516001600160a01b031685528695509381019392810192600101612e82565b83546001600160a01b031685529093019260019283019201612e53565b3461091c57612edc366132f2565b94929190939596600160f91b881180159061329b575b6132855750865f526020947f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70786526130b660405f20612f52600160405192612f3984613470565b604051612f4a81611a5581856137ba565b84520161384a565b908189820152519060405191612f678361348b565b8252888201908152612f7a368b89613546565b60408301908152612f8c36868b613546565b60608401908152606d8b7f6573206578747261446174612900000000000000000000000000000000000000608060405192612fc6846134b9565b8484528301927f5573657244656372797074526573706f6e7365566572696669636174696f6e2884527f6279746573207075626c69634b65792c627974657333325b5d20637448616e6460408201527f6c65732c6279746573207573657244656372797074656453686172652c627974606082015201522093518b815191012092516104b761305e8d60405192839182018095613fff565b51902091518b815191012090516040516130878d82816104f9818301968781519384920161335a565b51902091604051938c850195865260408501526060840152608083015260a082015260a08152611e1a816134d4565b966130ce83856130c6858a61402b565b9a8c8c614141565b885f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee702875260405f205f8052875260405f2061310b33826138c3565b54955f198701928784116132715761316e7f7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b2968a966131608e9a61317c9760806040519b8c9b8c528b015260808a0191613904565b918783036040890152613904565b918483036060860152613904565b0390a2835f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee7009283835260ff60405f2054161591826131f8575b50506131bf57005b825f525260405f20600160ff198254161790557fe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed255f80a2005b9091506040519163140f45ff60e11b83526004830152828260248173d582ec82a1758322907df80da8a754e12a5acb955afa918215610bb4575f92613242575b50101584806131b7565b9091508281813d831161326a575b61325a818361350a565b8101031261091c57519085613238565b503d613250565b634e487b7160e01b5f52601160045260245ffd5b636a457ca160e11b815260048101889052602490fd5b507f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee708548811612ef2565b9181601f8401121561091c578235916001600160401b03831161091c576020838186019501011161091c57565b608060031982011261091c57600435916001600160401b0360243581811161091c5783613321916004016132c5565b9390939260443583811161091c578261333c916004016132c5565b9390939260643591821161091c57613356916004016132c5565b9091565b5f5b83811061336b5750505f910152565b818101518382015260200161335c565b906020916133948151809281855285808601910161335a565b601f01601f1916010190565b9181601f8401121561091c578235916001600160401b03831161091c576020808501948460051b01011161091c57565b604060031982011261091c576001600160401b039160043583811161091c57826133fc916004016133a0565b9390939260243591821161091c57613356916004016132c5565b9181601f8401121561091c578235916001600160401b03831161091c576020808501946060850201011161091c57565b600435906001600160a01b038216820361091c57565b35906001600160a01b038216820361091c57565b604081019081106001600160401b0382111761082b57604052565b608081019081106001600160401b0382111761082b57604052565b6001600160401b03811161082b57604052565b60a081019081106001600160401b0382111761082b57604052565b60c081019081106001600160401b0382111761082b57604052565b60e081019081106001600160401b0382111761082b57604052565b90601f801991011681019081106001600160401b0382111761082b57604052565b6001600160401b03811161082b57601f01601f191660200190565b9291926135528261352b565b91613560604051938461350a565b82948184528183011161091c578281602093845f960137010152565b9181601f8401121561091c578235916001600160401b03831161091c576020808501948460061b01011161091c57565b3461091c57604036600319011261091c576001600160401b0360043581811161091c576135dd90369060040161357c565b60249291923591821161091c5760209261017761017e9336906004016132c5565b90600182811c9216801561362c575b602083101461361857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161360d565b905f917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10290815491613667836135fe565b808352926020916001918281169081156136e5575060011461368b575b5050505050565b90939495505f527f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d925f935b8585106136d257505050602092500101905f80808080613684565b80548585018401529382019381016136b7565b9350505050602093945060ff929192191683830152151560051b0101905f80808080613684565b905f917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1039081549161373d836135fe565b808352926020916001918281169081156136e55750600114613760575050505050565b90939495505f527f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b75925f935b8585106137a757505050602092500101905f80808080613684565b805485850184015293820193810161378c565b80545f93926137c8826135fe565b918282526020936001916001811690815f1461382b57506001146137ed575050505050565b90939495505f92919252835f2092845f945b83861061381757505050500101905f80808080613684565b8054858701830152940193859082016137ff565b60ff19168685015250505090151560051b010191505f80808080613684565b90604051918281549182825260209260208301915f5260205f20935f905b8282106138805750505061387e9250038361350a565b565b855484526001958601958895509381019390910190613868565b80548210156138af575f5260205f2001905f90565b634e487b7160e01b5f52603260045260245ffd5b80546801000000000000000081101561082b576138e59160018201815561389a565b6001600160a01b039291928084549260031b9316831b921b1916179055565b908060209392818452848401375f828201840152601f01601f1916010190565b6040519061393182613470565b600a82527f44656372797074696f6e000000000000000000000000000000000000000000006020830152565b51906001600160a01b038216820361091c57565b9081602091031261091c5751801515810361091c5790565b8115613a24575f5b8281106139a057505050600190565b60408051632ddc9a6f60e01b8152600583901b84013560048201526020808260248173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315613a1b57505f926139fe575b5050156139f757600101613991565b5050505f90565b613a149250803d106108fe576108f0818361350a565b5f806139e8565b513d5f823e3d90fd5b50505f90565b91908110156138af576060020190565b908015613a24575f5b818110613a5257505050600190565b613a5d818385613a2a565b60408051632ddc9a6f60e01b815291356004830152906020808260248173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315613a1b57505f92613aaf575b5050156139f757600101613a43565b613ac59250803d106108fe576108f0818361350a565b5f80613aa0565b818110613ad7575050565b5f8155600101613acc565b9190601f8111613af157505050565b61387e925f5260205f20906020601f840160051c83019310613b1b575b601f0160051c0190613acc565b9091508190613b0e565b91908110156138af5760061b0190565b908015613a24575f5b818110613b4d57505050600190565b613b58818385613b25565b60408051632ddc9a6f60e01b815291356004830152906020808260248173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315613a1b57505f92613baa575b5050156139f757600101613b3e565b613bc09250803d106108fe576108f0818361350a565b5f80613b9b565b6001600160401b03811161082b5760051b60200190565b903590601e198136030182121561091c57018035906001600160401b03821161091c57602001918160051b3603831361091c57565b604090602319011261091c5760405190613c2c82613470565b60243582526044356020830152565b6064356001600160a01b038116810361091c5790565b6084356001600160a01b038116810361091c5790565b9291613c7282613bc7565b91613c80604051938461350a565b829481845260208094019160051b810192831161091c57905b828210613ca65750505050565b838091613cb28461345c565b815201910190613c99565b9190808252602080920192915f5b828110613cd9575050505090565b9091929382806001926001600160a01b03613cf38961345c565b16815201950193929101613ccb565b6040906001600160a01b03613d2295931681528160208201520191613cbd565b90565b602090818184031261091c5780516001600160401b039182821161091c57019083601f8301121561091c57815190613d5c82613bc7565b946040613d6b8151978861350a565b838752858701928660059560051b8701019583871161091c57878101945b878610613d9c5750505050505050505090565b855183811161091c578201608080601f19838903011261091c57855191613dc28361348b565b8b8101518352868101518c84015260609182820151888501528101519086821161091c57019087603f8301121561091c57818c8093015188613e0382613bc7565b94613e108251968761350a565b8286528501918d1b830101918a831161091c5791898f969492979593015b818110613e48575050849550820152815201950194613d89565b9193958091939597613e598461395d565b8152019101918e959391969492613e2e565b60209060206040818301928281528551809452019301915f5b828110613e92575050505090565b835185529381019392810192600101613e84565b5f1981146132715760010190565b9068010000000000000000811161082b57815491818155828210613ed757505050565b5f5260205f2091820191015b818110613eee575050565b5f8155600101613ee3565b6080820181518352602060a060608294838101518488015260408101516040880152015194608060608201528551809452019301915f5b828110613f3e575050505090565b83516001600160a01b031685529381019392810192600101613f30565b90808251908181526020809101926020808460051b8301019501935f915b848310613f895750505050505090565b9091929394958480613fa7600193601f198682030187528a51613ef9565b9801930193019194939290613f79565b949293613d2296946001600160a01b03613fdd613ff1959460808a5260808a0190613f5b565b931660208801528683036040880152613904565b926060818503910152613904565b80516020809201915f5b828110614017575050505090565b835185529381019392810192600101614009565b811561410457803560f81c918215614096576001831461405e5760405163084e730b60e21b815260048101849052602490fd5b909150602181106140775760211161091c576001013590565b604490604051906349aa453360e11b8252600482015260216024820152fd5b505060405163976f3eb960e01b8152905060208160048173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f916140d5575090565b90506020813d6020116140fc575b816140f06020938361350a565b8101031261091c575190565b3d91506140e3565b505060405163976f3eb960e01b815260208160048173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f916140d5575090565b9493916105556141609461415793943691613546565b909391936149f8565b60408051632511f3f560e21b8152600481018690526001600160a01b03841660248201529092602092909173d582ec82a1758322907df80da8a754e12a5acb95908481604481855afa90811561438c575f9161436f575b501561434f57845163063fe83960e31b815260048101979097523360248801525f90879060449082905afa801561434557839495965f916142a7575b506001600160a01b0394859101511693821680940361428957805f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70191828452855f20855f52845260ff865f20541661426257505f528152825f20915f52525f20600160ff19825416179055565b85516399ec48d960e01b815260048101929092526001600160a01b03166024820152604490fd5b8451630d86f52160e01b815260048101859052336024820152604490fd5b9350503d805f853e6142b9818561350a565b8301848482031261091c5783516001600160401b039485821161091c570160808183031261091c578651916142ed8361348b565b6142f68261395d565b835261430387830161395d565b878401528782015186811161091c578161431e918401614831565b88840152606082015195861161091c57869561433a9201614831565b60608201525f6141f3565b84513d5f823e3d90fd5b845163153e377b60e11b81526001600160a01b0384166004820152602490fd5b6143869150853d87116108fe576108f0818361350a565b5f6141b7565b86513d5f823e3d90fd5b6040515f6143a382613470565b600190600183526020368185013760218301825b6143c2575b50505090565b5f190190600a906f181899199a1a9b1b9c1cb0b131b232b360811b8282061a8353049182156143f3579190826143b7565b6143bc565b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541661442357565b60405163d93c066560e01b8152600490fd5b9190820180921161327157565b602081018051156144d957805161016d908181116144bb575050815142811161124957508151905162015180908181029181830414901517156132715761448a904292614435565b106144925750565b60405162c0d20160e61b8152426004820152815160248201526020909101516044820152606490fd5b6044925060405191633295186360e01b835260048301526024820152fd5b60405163de2859c160e01b8152600490fd5b8051156138af5760200190565b80518210156138af5760209160051b010190565b905f5b82518110156139f7576001600160a01b038061452b83866144f8565b51169083161461453d5760010161450f565b505050600190565b9061454f82613bc7565b61455c604051918261350a565b828152809261456d601f1991613bc7565b0190602036910137565b92919080156146885761458981614545565b935f925f5b8381106145a8575050505061080090818111610bed575050565b6145b3818585613b25565b35602095866145c3848888613b25565b0135906001600160a01b038216820361091c576001600160401b038360101c1685358082036146625750506146019061ffff6109bb6109b6866148b4565b96840161461c826103286146158489613bde565b3691613c67565b1561463857505090600191614631828a6144f8565b520161458e565b906146466108279286613bde565b60405163a4c3039160e01b815293849390929060048501613d02565b604051634ac8748b60e11b81526004810186905260248101929092526044820152606490fd5b60405163a6a6cb2160e01b8152600490fd5b60019060018151111561474757908092916020806146b7836144eb565b510151906001955b6146cc575b505050509050565b82518610156147425781816146e188866144f8565b510151036146f35794830194836146bf565b8261470887614701836144eb565b51926144f8565b51610827604091614730835194859463cfae921f60e01b865260048601526044850190613ef9565b83810360031901602485015290613ef9565b6146c4565b5050565b7333e0c7a03d2b040b518580c365f4b3bde7cc4e6e90813b1561091c576001600160a01b0360245f9283604051958694859363988a2d2d60e01b85521660048401525af18015610bb45761479c5750565b61387e906134a6565b613d22906147b1614bb7565b6104b76148116147bf614c29565b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f602082019081529181019590955260608501919091524660808501523060a08501529291829060c0820190565b5190206042916040519161190160f01b8352600283015260228201522090565b81601f8201121561091c5780516148478161352b565b92614855604051948561350a565b8184526020828401011161091c57613d22916020808501910161335a565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c16156148a257565b604051631afcd79f60e31b8152600490fd5b60081c60ff16605381116148e25760548110156148ce5790565b634e487b7160e01b5f52602160045260245ffd5b6024906040519063641950d760e01b82526004820152fd5b60548110156148ce578061490e5750600290565b6002810361491c5750600890565b6003810361492a5750601090565b600481036149385750602090565b600581036149465750604090565b600681036149545750608090565b60078103614962575060a090565b60088103614971575061010090565b6024906040519063be7830b160e01b82526004820152fd5b80516020809201915f5b8281106149a1575050505090565b83516001600160a01b031685529381019392810192600101614993565b81519190604183036149ee576149e79250602082015190606060408401519301515f1a90614ad2565b9192909190565b50505f9160029190565b60048110156148ce5780614a0a575050565b60018103614a245760405163f645eedf60e01b8152600490fd5b60028103614a455760405163fce698f760e01b815260048101839052602490fd5b600314614a4f5750565b602490604051906335e2f38360e21b82526004820152fd5b90613d2291614a74614bb7565b614811614a7f614c29565b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6020820190815291810194909452606084019190915260808301939093523060a08301528160c081016104b7565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411614b49579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15610bb4575f516001600160a01b03811615614b3f57905f905f90565b505f906001905f90565b5050505f9160039190565b90614b7b5750805115614b6957602081519101fd5b60405163d6bda27560e01b8152600490fd5b81511580614bae575b614b8c575090565b604051639996b31560e01b81526001600160a01b039091166004820152602490fd5b50803b15614b84565b604051614bc781611a5581613636565b8051908115614bd7576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100548015614c045790565b507fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47090565b604051614c3981611a558161370c565b8051908115614c49576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d101548015614c04579056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0\x80`@R4b\0\0\xD1W0`\x80R\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90\x81T\x90`\xFF\x82`@\x1C\x16b\0\0\xC2WP`\x01`\x01`@\x1B\x03`\x02`\x01`@\x1B\x03\x19\x82\x82\x16\x01b\0\0|W[`@QaLv\x90\x81b\0\0\xD6\x829`\x80Q\x81\x81\x81a\"\xE0\x01Ra#\x92\x01R\xF3[`\x01`\x01`@\x1B\x03\x19\x90\x91\x16\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x90\xA1_\x80\x80b\0\0\\V[c\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x90\xFD[_\x80\xFD\xFE`\xA0\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04o\x9E\xB3\x14a.\xCEWP\x80c\t\0\xCCi\x14a-\xCCW\x80c\r\x8En,\x14a,\xA5W\x80c\x12:\xBB(\x14a,\rW\x80c9\xF78\x10\x14a'\xB1W\x80c?K\xA8:\x14a&{W\x80c@\x14\xC4\xCD\x14a&`W\x80cA\x0B\xF0\xBA\x14a&\x07W\x80cO\x1E\xF2\x86\x14a#BW\x80cR\xD1\x90-\x14a\"\xC6W\x80cX\xF5\xB8\xAB\x14a\"xW\x80c\\\x97Z\xBB\x14a\"7W\x80co\x89\x13\xBC\x14a\x1C~W\x80cv\"~\xED\x14a\t$W\x80c\x84V\xCBY\x14a\x1B~W\x80c\x84\xB0\x19n\x14a\x1A\x07W\x80c\x9F\xADZ/\x14a\x13*W\x80c\xAD<\xB1\xCC\x14a\x12\xC9W\x80c\xB4\xDE,7\x14a\x0C-W\x80c\xD8\x99\x8FE\x14a\t)W\x80c\xE2-\x1B&\x14a\t$W\x80c\xF1\xB5z\xDB\x14a\x01\x8FWc\xFB\xB82Y\x14a\x01\x1BW_\x80\xFD[4a\x01\x88W``6`\x03\x19\x01\x12a\x01\x88Wa\x014a4FV[P`\x01`\x01`@\x1B\x03\x90`$5\x82\x81\x11a\x01\x8BWa\x01V\x906\x90`\x04\x01a5|V[\x92\x90\x91`D5\x91\x82\x11a\x01\x88W` a\x01~\x85\x85a\x01w6`\x04\x88\x01a2\xC5V[PPa;5V[`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P\x80\xFD[P4a\x01\x88W`\x03\x19a\x01\x006\x82\x01\x12a\x01\x8BW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\t Wa\x01\xC2\x906\x90`\x04\x01a5|V[\x90\x91`@6`#\x19\x01\x12a\t\x18W`\x01`\x01`@\x1B\x03`d5\x11a\t\x18W`@\x90`d56\x03\x01\x12a\t W`\x01`\x01`\xA0\x1B\x03`\x845\x16`\x845\x03a\t\x1CW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\t\x18Wa\x02!\x906\x90`\x04\x01a2\xC5V[\x90\x91`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\t\x14Wa\x02B\x906\x90`\x04\x01a2\xC5V[\x94\x90`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\t\x10Wa\x02c\x906\x90`\x04\x01a2\xC5V[\x93\x90\x92a\x02naC\xF8V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x04`d5\x81\x015\x90\x82\x01R` \x81`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\t\x05W\x8A\x91a\x08\xD6W[P\x15a\x08\xB9Wa\x02\xCC`$`d5\x01`d5`\x04\x01a;\xDEV[\x90P\x15a\x08\xA7W`\na\x02\xE9`$`d5\x01`d5`\x04\x01a;\xDEV[\x90P\x11a\x08sWa\x03\x01a\x02\xFC6a<\x13V[aDBV[a\x03-a\x03(a\x03\x1B`$`d5\x01`d5`\x04\x01a;\xDEV[\x91\x90`\x845\x926\x91a<gV[aE\x0CV[a\x08?Wa\x03A\x91`d5`\x04\x01\x91aEwV[\x95a\x03V`$`d5\x01`d5`\x04\x01a;\xDEV[`@Q\x91\x82`\xA0\x81\x01\x10`\x01`\x01`@\x1B\x03`\xA0\x85\x01\x11\x17a\x08+W\x82a\x03\x9Da\x05J\x92a\x05[\x94`\xA0a\x05d\x97\x01`@Ra\x03\x94\x8D\x8D6\x91a5FV[\x84R6\x91a<gV[` \x82\x01\x90\x81R`$5`@\x83\x01R`D5``\x83\x01Ra\x03\xBF6\x8A\x8Aa5FV[`\x80\x83\x01R`\x87`@Qa\x03\xD2\x81a4\xD4V[\x81\x81R\x7FraData)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xA0` \x83\x01\x92\x7FUserDecryptRequestVerification(b\x84R\x7Fytes publicKey,address[] contrac`@\x82\x01R\x7FtAddresses,uint256 startTimestam``\x82\x01R\x7Fp,uint256 durationDays,bytes ext`\x80\x82\x01R\x01R \x91a\x04\xB7a\x04\xC5\x82Q` \x81Q\x91\x01 \x93Q`@Q\x92\x83\x91` \x83\x01\x90aI\x89V[\x03`\x1F\x19\x81\x01\x83R\x82a5\nV[` \x81Q\x91\x01 \x90`@\x81\x01Q`\x80``\x83\x01Q\x92\x01Q`@Qa\x05\x06` \x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[\x81\x01\x03\x80\x84R\x01\x82a5\nV[Q\x90 \x92`@Q\x94` \x86\x01\x96\x87R`@\x86\x01R``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x05;\x81a4\xEFV[Q\x90 `d5`\x04\x015aJgV[a\x05U6\x85\x87a5FV[\x90aI\xBEV[\x90\x92\x91\x92aI\xF8V[`\x01`\x01`\xA0\x1B\x03\x80`\x845\x16\x91\x16\x03a\x08\x03WPP`@Qc\xA1O\x89q`\xE0\x1B\x81R\x92\x86\x84\x80a\x05\x98\x89`\x04\x83\x01a>kV[\x03\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x93\x84\x15a\x07\xF8W\x87\x94a\x07\xD4W[Pa\x05\xC9\x84aF\x9AV[a\x05\xF3\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08Ta>\xA6V[\x95\x86\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U`@Q\x90a\x06$\x82a4pV[a\x06/6\x84\x89a5FV[\x82R` \x82\x01R\x86\x88R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07` R`@\x88 \x90\x80Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07RWa\x06\x89\x82a\x06\x83\x86Ta5\xFEV[\x86a:\xE2V[` \x90`\x1F\x83\x11`\x01\x14a\x07qWa\x06\xB8\x92\x91\x8C\x91\x83a\x07fW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x82U[` `\x01\x80\x93\x01\x91\x01Q\x90\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x07RW` \x90a\x06\xE4\x84\x84a>\xB4V[\x01\x90\x8AR` \x8A \x8A[\x83\x81\x10a\x07?W\x8B\x8B\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x8C\x8Ca\x079\x8D\x8D\x8Da\x07)3aGKV[`@Q\x95\x86\x95`\x845\x90\x87a?\xB7V[\x03\x90\xA2\x80\xF3[\x84\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x06\xEEV[cNH{q`\xE0\x1B\x8BR`A`\x04R`$\x8B\xFD[\x01Q\x90P_\x80a\x06\xA4V[\x84\x8CR` \x8C \x91\x90`\x1F\x19\x84\x16\x8D[\x81\x81\x10a\x07\xBCWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x07\xA4W[PPP\x81\x1B\x01\x82Ua\x06\xBBV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x07\x97V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\x81V[a\x07\xF1\x91\x94P=\x80\x89\x83>a\x07\xE9\x81\x83a5\nV[\x81\x01\x90a=%V[\x92_a\x05\xBFV[`@Q=\x89\x82>=\x90\xFD[a\x08'`@Q\x92\x83\x92c*\x87='`\xE0\x1B\x84R` `\x04\x85\x01R`$\x84\x01\x91a9\x04V[\x03\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[a\x08S`$`d5\x01`d5`\x04\x01a;\xDEV[`@Qc\xDCMx\xB1`\xE0\x1B\x81R\x91\x82\x91a\x08'\x91`\x845`\x04\x85\x01a=\x02V[`Da\x08\x89`$`d5\x01`d5`\x04\x01a;\xDEV[\x90P`@Q\x90c\xAF\x1F\x04\x95`\xE0\x1B\x82R`\n`\x04\x83\x01R`$\x82\x01R\xFD[`@QcW\xCF\xA2\x17`\xE0\x1B\x81R`\x04\x90\xFD[`$`@Qc\xB6g\x9C;`\xE0\x1B\x81R`d5`\x04\x015`\x04\x82\x01R\xFD[a\x08\xF8\x91P` =` \x11a\x08\xFEW[a\x08\xF0\x81\x83a5\nV[\x81\x01\x90a9qV[_a\x02\xB2V[P=a\x08\xE6V[`@Q=\x8C\x82>=\x90\xFD[\x87\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[_\x80\xFD[\x82\x80\xFD[a5\xACV[P4a\t\x1CWa\t86a3\xD0V[\x91\x93\x90\x92a\tDaC\xF8V[\x84\x15a\x0C\x1BWa\tS\x85a;\xC7V[\x92a\ta`@Q\x94\x85a5\nV[\x85\x84R` \x80\x85\x01\x91\x87`\x05\x1B\x92\x83\x85\x01\x906\x82\x11a\t\x1CW\x83\x86\x91[\x83\x83\x10a\x0C\x0BWPPPP_\x96_\x97[\x87Q\x89\x10\x15a\t\xCAWa\t\xC2`\x01\x91a\xFF\xFFa\t\xBBa\t\xB6a\t\xB0\x8E\x8EaD\xF8V[QaH\xB4V[aH\xFAV[\x16\x90aD5V[\x98\x01\x97a\t\x8EV[\x89\x90a\x08\0\x90\x81\x81\x11a\x0B\xEDWPP`@Q\x94c\xA1O\x89q`\xE0\x1B\x86R\x84`\x04\x87\x01R\x81`$\x87\x01R\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\t\x1CW\x85`D\x81\x83_\x94\x8B\x84\x84\x017\x81\x01\x03\x01\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x94\x85\x15a\x0B\xB4W_\x95a\x0B\xD1W[Pa\nZ\x85aF\x9AV[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06\x95a\n\x86\x87Ta>\xA6V[\x80\x97U\x86_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x05\x85R`@_ \x90`\x01`\x01`@\x1B\x03\x83\x11a\x08+Wa\n\xCC\x83\x83a>\xB4V[\x90_R\x84_ _[\x83\x81\x10a\x0B\xBFWPPPPs3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNn\x80;\x15a\t\x1CW_\x80\x91`$`@Q\x80\x94\x81\x93c${\xAC\x9F`\xE2\x1B\x83R3`\x04\x84\x01RZ\xF1\x80\x15a\x0B\xB4Wa\x0BqW[Pa\x0Bd\x92\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x94\x92a\x079\x92`@Q\x95\x86\x95`@\x87R`@\x87\x01\x90a?[V[\x92\x85\x84\x03\x90\x86\x01Ra9\x04V[a\x079\x91\x96P\x92\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x94\x92a\x0B\xA7a\x0Bd\x95a4\xA6V[_\x97\x92P\x92\x94P\x92a\x0B$V[`@Q=_\x82>=\x90\xFD[\x825\x82\x82\x01U\x91\x86\x01\x91`\x01\x01a\n\xD4V[a\x0B\xE6\x91\x95P=\x80_\x83>a\x07\xE9\x81\x83a5\nV[\x93\x87a\nPV[`D\x92P`@Q\x91c\xE7\xF4\x89]`\xE0\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[\x825\x81R\x91\x81\x01\x91\x85\x91\x01a\t~V[`@Qc\x05\xBC\xEA\x87`\xE3\x1B\x81R`\x04\x90\xFD[4a\t\x1CWa\x01\x006`\x03\x19\x01\x12a\t\x1CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C^\x906\x90`\x04\x01a4\x16V[\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\t\x1CW`D5`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C\x91\x906\x90`\x04\x01a2\xC5V[`d5`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C\xB0\x906\x90`\x04\x01a3\xA0V[\x90`@6`\x83\x19\x01\x12a\t\x1CW`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C\xDC\x906\x90`\x04\x01a2\xC5V[\x92\x90\x96`\x01`\x01`@\x1B\x03`\xE45\x11a\t\x1CWa\x0C\xFE6`\xE45`\x04\x01a2\xC5V[\x95\x90\x96a\r\taC\xF8V[\x8A\x15a\x12\xB7W`\n\x84\x11a\x12\x97W`@Qa\r#\x81a4pV[`\x845\x81R`\xA45` \x82\x01R`\xA45\x15a\x12\x85W` \x81\x01Qc\x01\xE13\x80\x90\x81\x81\x11a\x12gWPP\x80QB\x81\x11a\x12IWP\x80Qa\rhB\x91` \x84\x01Q\x90aD5V[\x10a\x12\x1FWPa\rw3aGKV[a\r\x80\x8BaEEV[\x98`\x10\x9B`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x865`\x10\x1C\x16`\x04\x82\x01R` \x81`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a\x12\0W[P\x15a\x11\xDBW_\x9C_`\x80R[\x81`\x80Q\x10a\x11:WPa\x08\0\x80\x8E\x11a\x11\x1CWP`@Qc\xA1O\x89q`\xE0\x1B\x81R\x9B\x9CP\x99\x9A\x99_\x8B\x80a\x0E\x14\x8F`\x04\x83\x01a>kV[\x03\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x9A\x8B\x15a\x0B\xB4W_\x9Ba\x11\0W[Pa\x0EE\x8BaF\x9AV[a\x0Eo\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08Ta>\xA6V[\x9B\x8C\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U`@Qa\x0E\x9F\x81a4pV[a\x0E\xAA6\x87\x87a5FV[\x81R` \x81\x01\x91\x82R\x8D_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07` R`@_ \x90Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x08+Wa\x0F\x05\x82a\x0E\xFF\x85Ta5\xFEV[\x85a:\xE2V[` \x90`\x1F\x83\x11`\x01\x14a\x10\x99W\x91\x80a\x0F7\x92`\x01\x95\x94_\x92a\x07fWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x81U[\x01\x90Q\x90\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x08+W` \x90a\x0F\\\x84\x84a>\xB4V[\x01\x90_R` _ _[\x83\x81\x10a\x10\x85WPPPP` a\x0F\x89`@Q\x9C\x8Da\x01 \x90\x81\x81R\x01\x90a?[V[\x8C\x81\x03\x82\x8E\x01R\x82\x81R\x01\x94\x90_[\x81\x81\x10a\x108WPPP\x94a\x10$\x94a\x0F\xF8\x7F\x04G\x11\xC6L\xF1\x8A\xBFI`\"\x0E>\x1E\xCF\x01\x05\x9Cr\xB8\x15,\x84\x15\xB5\x8E\xA1\xFD\xD8J+K\x9C\x9D\x95\x8C\x9B\x99\x95a\x10\x06\x95\x8D`@`\x01`\x01`\xA0\x1B\x03a\x103\x9F\x9D\x16\x91\x01R\x8D``\x81\x85\x03\x91\x01Ra9\x04V[\x91\x8A\x83\x03`\x80\x8C\x01Ra<\xBDV[\x91`\x845`\xA0\x89\x01R`\xA45`\xC0\x89\x01R\x87\x83\x03`\xE0\x89\x01Ra9\x04V[\x91\x84\x83\x03a\x01\0\x86\x01Ra9\x04V[\x03\x90\xA2\0[\x90\x91\x95`\x01\x90\x875\x81R`\x01`\x01`\xA0\x1B\x03a\x10V` \x8A\x01a4\\V[\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03a\x10p`@\x8A\x01a4\\V[\x16`@\x82\x01R``\x90\x81\x01\x97\x01\x92\x91\x01a\x0F\x98V[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x0FfV[\x90`\x1F\x19\x83\x16\x91\x84_R` _ \x92_[\x81\x81\x10a\x10\xE8WP\x91`\x01\x95\x94\x92\x91\x83\x87\x95\x93\x10a\x10\xD0W[PPP\x81\x1B\x01\x81Ua\x0F:V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x10\xC3V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x10\xAAV[a\x11\x15\x91\x9BP=\x80_\x83>a\x07\xE9\x81\x83a5\nV[\x99\x8Da\x0E;V[`D\x90\x8E`@Q\x91c\xE7\xF4\x89]`\xE0\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[`\x01`\x01`@\x1B\x03\x9Da\x11P`\x80Q\x84\x89a:*V[5\x9E\x8F\x83\x1C\x16`\x01`\x01`@\x1B\x03\x885`\x10\x1C\x16\x81\x03a\x11\x9CWPa\x11\x7F\x90\x8Fa\t\xBBa\t\xB6a\xFF\xFF\x92aH\xB4V[\x9Da\x11\x8D\x8D`\x80Q\x90aD\xF8V[R`\x01`\x80Q\x01`\x80Ra\r\xDCV[\x8Fa\x08'\x89\x91`\x01`\x01`@\x1B\x03\x93`@Q\x94\x85\x94cJ\xC8t\x8B`\xE1\x1B\x86R5`\x10\x1C\x16\x91`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`@Qc\xB6g\x9C;`\xE0\x1B\x81R\x855`\x10\x1C`\x01`\x01`@\x1B\x03\x16`\x04\x82\x01R`$\x90\xFD[a\x12\x19\x91P` =` \x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[\x8Ea\r\xCFV[`@Qc3\xC7\xE7\xE7`\xE1\x1B\x81RB`\x04\x82\x01R\x81Q`$\x82\x01R` \x90\x91\x01Q`D\x82\x01R`d\x90\xFD[`D\x90`@Q\x90c\xF2L\x08\x87`\xE0\x1B\x82RB`\x04\x83\x01R`$\x82\x01R\xFD[`D\x92P`@Q\x91cW)u\x89`\xE1\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[`@Qc\x12)\xE27`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\xAF\x1F\x04\x95`\xE0\x1B\x81R`\n`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[`@Qc$\x0E\x93\t`\xE0\x1B\x81R`\x04\x90\xFD[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CWa\x13&`@Qa\x12\xE8\x81a4pV[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a3{V[\x03\x90\xF3[4a\t\x1CW`\x03\x19a\x01 6\x82\x01\x12a\t\x1CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\\\x906\x90`\x04\x01a5|V[\x90\x91`@6`#\x19\x01\x12a\t\x1CW`@6`c\x19\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03`\xA45\x11a\t\x1CW`@\x90`\xA456\x03\x01\x12a\t\x1CW`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\xB3\x906\x90`\x04\x01a2\xC5V[`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\xD2\x906\x90`\x04\x01a2\xC5V[\x94\x90a\x01\x045`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\xF4\x906\x90`\x04\x01a2\xC5V[\x96\x90\x92a\x13\xFFaC\xF8V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x04`\xA45\x81\x015\x90\x82\x01R` \x81`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a\x19\xE8W[P\x15a\x19\xCBW`$`\xA45\x01\x96a\x14_\x88`\xA45`\x04\x01a;\xDEV[\x90P\x15a\x08\xA7W`\na\x14w\x89`\xA45`\x04\x01a;\xDEV[\x90P\x11a\x19\xBAWa\x14\x8Aa\x02\xFC6a<\x13V[a\x14\xB1a\x03(a\x14\x9F\x8A`\xA45`\x04\x01a;\xDEV[\x91\x90a\x14\xA9a<;V[\x926\x91a<gV[a\x19\x83Wa\x14\xD5\x91a\x14\xC9\x91`\xA45`\x04\x01\x91aEwV[\x96`\xA45`\x04\x01a;\xDEV[\x90a\x14\xDEa<;V[`@Q\x92\x83`\xC0\x81\x01\x10`\x01`\x01`@\x1B\x03`\xC0\x86\x01\x11\x17a\x08+W`\x01`\x01`\xA0\x1B\x03\x92a\x15\"\x91`\xC0\x86\x01`@Ra\x15\x196\x8B\x8Da5FV[\x86R6\x91a<gV[` \x84\x01R\x16`@\x82\x01R`$5``\x82\x01R`D5`\x80\x82\x01Ra\x15H6\x89\x86a5FV[`\xA0\x82\x01Ra\x15Ua<QV[`\xA9`@Qa\x15c\x81a4\xEFV[\x81\x81R\x7FxtraData)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0` \x83\x01\x92\x7FDelegatedUserDecryptRequestVerif\x84R\x7Fication(bytes publicKey,address[`@\x82\x01R\x7F] contractAddresses,address dele``\x82\x01R\x7FgatorAddress,uint256 startTimest`\x80\x82\x01R\x7Famp,uint256 durationDays,bytes e`\xA0\x82\x01R\x01R \x91\x80Q` \x81Q\x91\x01 \x90a\x04\xB7a\x16r` \x83\x01Q`@Q\x92\x83\x91` \x83\x01\x90aI\x89V[` \x81Q\x91\x01 `\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16``\x83\x01Q\x91`\xA0`\x80\x85\x01Q\x94\x01Q`@Qa\x16\xB4` \x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[Q\x90 \x94`@Q\x97` \x89\x01R`@\x88\x01R``\x87\x01R`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01R`\xE0\x82R\x81a\x01\0\x81\x01\x10`\x01`\x01`@\x1B\x03a\x01\0\x84\x01\x11\x17a\x08+W`\x01`\x01`\xA0\x1B\x03\x80\x91a\x177a\x17,\x85a\x01\0a\x17@\x97\x01`@R` \x81Q\x91\x01 `\xA45`\x04\x015aJgV[a\x05U6\x88\x8Aa5FV[\x90\x95\x91\x95aI\xF8V[\x16\x91\x16\x03a\x08\x03WPP`@Qc\xA1O\x89q`\xE0\x1B\x81R\x91_\x83\x80a\x17h\x88`\x04\x83\x01a>kV[\x03\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a\x0B\xB4W_\x93a\x19gW[Pa\x17\x99\x83aF\x9AV[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x94a\x17\xC5\x86Ta>\xA6V[\x80\x96U`@Q\x90a\x17\xD5\x82a4pV[a\x17\xE06\x84\x88a5FV[\x82R` \x82\x01\x90\x81R\x86_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07` R`@_ \x91Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x08+Wa\x185\x82a\x06\x83\x86Ta5\xFEV[` \x90`\x1F\x83\x11`\x01\x14a\x19\x03Wa\x18c\x92\x91_\x91\x83a\x18\xF8WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x82U[`\x01\x80\x92\x01\x90Q\x90\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x08+W` \x90a\x18\x8C\x84\x84a>\xB4V[\x01\x90_R` _ _[\x83\x81\x10a\x18\xE5W\x89\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x8A\x8Aa\x103\x8F\x8C\x8Ca\x18\xD03aGKV[a\x18\xD8a<QV[\x95`@Q\x96\x87\x96\x87a?\xB7V[\x84\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x18\x96V[\x01Q\x90P\x8B\x80a\x06\xA4V[\x90`\x1F\x19\x83\x16\x91\x85_R` _ \x92_[\x81\x81\x10a\x19OWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x197W[PPP\x81\x1B\x01\x82Ua\x18fV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8A\x80\x80a\x19*V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x19\x14V[a\x19|\x91\x93P=\x80_\x83>a\x07\xE9\x81\x83a5\nV[\x91\x86a\x17\x8FV[a\x08'\x88a\x19\x9Ea\x19\x92a<;V[\x91`\xA45`\x04\x01a;\xDEV[`@Qc\xC3Dj\xC7`\xE0\x1B\x81R\x93\x84\x93\x90\x92\x90`\x04\x85\x01a=\x02V[`Da\x08\x89\x89`\xA45`\x04\x01a;\xDEV[`$`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\xA45`\x04\x015`\x04\x82\x01R\xFD[a\x1A\x01\x91P` =` \x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[\x89a\x14CV[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x15\x80a\x1BUW[\x15a\x1B\x10W`@Qa\x1A\\\x81a\x1AU\x81a66V[\x03\x82a5\nV[`@Qa\x1Al\x81a\x1AU\x81a7\x0CV[`@Q` \x80\x82\x01\x92\x82\x84\x10`\x01`\x01`@\x1B\x03\x85\x11\x17a\x08+W\x91` a\x1A\xC5\x85\x94a\x1A\xB7\x97\x96`@R_\x84R`@Q\x97\x88\x97`\x0F`\xF8\x1B\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a3{V[\x90\x87\x82\x03`@\x89\x01Ra3{V[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x1A\xF9WPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1A\xEAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[P\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x15a\x1A@V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a\x1C_W[P\x15\x90\x81a\x1CTW[Pa\x1C<Wa\x1B\xE4aC\xF8V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0`\x01`\xFF\x19\x82T\x16\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[`@Qc8\x89\x16\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[\x90P3\x14\x15\x81a\x1B\xD7V[a\x1Cx\x91P` =` \x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[\x82a\x1B\xCEV[4a\t\x1CWa\x1C\x8C6a2\xF2V[\x92\x93\x94`\x01`\xF8\x93\x92\x93\x1B\x87\x11\x80\x15\x90a\"\rW[a!\xF4W\x86_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x05` R`@_ `@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+Wa\x1E\"\x92a\x1C\xF9\x91`@Ra8JV[\x81Ra\x1D\x066\x89\x85a5FV[` \x82\x01Ra\x1D\x166\x87\x87a5FV[`@\x82\x01R`T`@Qa\x1D)\x81a4\x8BV[\x81\x81R\x7Fult,bytes extraData)\0\0\0\0\0\0\0\0\0\0\0\0``` \x83\x01\x92\x7FPublicDecryptVerification(bytes3\x84R\x7F2[] ctHandles,bytes decryptedRes`@\x82\x01R\x01R \x90a\x04\xB7a\x1D\xB9\x82Q`@Q\x92\x83\x91` \x83\x01\x90a?\xFFV[` \x81Q\x91\x01 \x90`@` \x82\x01Q` \x81Q\x91\x01 \x91\x01Q`@Qa\x1D\xEF` \x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[Q\x90 \x90`@Q\x92` \x84\x01\x94\x85R`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x1E\x1A\x81a4\xB9V[Q\x90 aG\xA5V[a\x1E,\x85\x85a@+V[\x92a\x1E:\x81\x88\x84\x8C\x88aAAV[\x88_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x04` R`@_ \x82_R` R`@_ \x96\x87Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x08+W\x80`\x01a\x1E\x93\x92\x01\x8AU\x89a8\x9AV[\x91\x90\x91a!\xE1W`\x01`\x01`@\x1B\x03\x83\x11a\x08+W\x82\x82a\x1E\xC0\x8D\x95a\x1E\xBA\x8E\x96Ta5\xFEV[\x83a:\xE2V[_`\x1F\x83\x11`\x01\x14a!IWa\x1Fq\x93\x83a\x1F\x94\x93a\x1F\x1A\x82\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x99\x97a\x1F\x7F\x96_\x91a!>W[P\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[\x87_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02` R`@_ \x89_R` Ra\x1F\\`@_ 3\x90a8\xC3V[`@Q\x95\x86\x95`\x80\x87R`\x80\x87\x01\x90\x8Ca9\x04V[\x91\x85\x83\x03` \x87\x01Ra9\x04V[3`@\x84\x01R\x82\x81\x03``\x84\x01R\x8A\x8Aa9\x04V[\x03\x90\xA2\x87_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x92\x83` R`\xFF`@_ T\x16\x15\x90\x81a \xBFW[Pa\x1F\xD7W\0[a +\x92\x88_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x03` R`@_ U`@Q\x95``\x87R``\x87\x01\x91a9\x04V[\x84\x81\x03` \x86\x01R\x83T\x80\x82R` \x82\x01\x91` \x82`\x05\x1B\x82\x01\x01\x95_R` _ \x92_\x91[\x83\x83\x10a \x93WPPPP\x84\x84\x03`@\x86\x01RP\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x93\x92\x83\x92a\x103\x92a9\x04V[\x90\x91\x92\x93\x96` `\x01a \xB0\x81\x93`\x1F\x19\x86\x82\x03\x01\x87R\x8Ba7\xBAV[\x99\x01\x93\x01\x93\x01\x91\x93\x92\x90a QV[\x90P\x86T`@Q\x91ca\xD5U-`\xE1\x1B\x83R`\x04\x83\x01R` \x82`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x91\x82\x15a\x0B\xB4W_\x92a!\nW[P\x10\x15\x89a\x1F\xD0V[\x90\x91P` \x81=` \x11a!6W[\x81a!&` \x93\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90\x8Aa!\x01V[=\x91Pa!\x19V[\x90P\x85\x015_a\x1F\x07V[\x81_R` _ \x90_[`\x1F\x19\x85\x16\x81\x10a!\xC3WP\x93\x83a\x1F\x94\x93a\x1F\x7F\x93a\x1Fq\x97\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x99\x97`\x1F\x19\x81\x16\x10a!\xAAW[PP`\x01\x82\x81\x1B\x01\x90Ua\x1F\x1DV[\x84\x015_\x19`\x03\x85\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80a!\x9BV[\x85\x82\x015\x83U\x8F\x97P\x8E\x96P`\x01\x90\x92\x01\x91` \x91\x82\x01\x91\x01a!SV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`@QcjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x90\xFD[P\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06T\x87\x11a\x1C\xA1V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\t\x1CW` 6`\x03\x19\x01\x12a\t\x1CW`\x045_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a#0W` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x90\xFD[`@6`\x03\x19\x01\x12a\t\x1CWa#Va4FV[`$\x90\x815`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CW6`#\x82\x01\x12\x15a\t\x1CWa#\x87\x906\x90\x84\x81`\x04\x015\x91\x01a5FV[`\x01`\x01`\xA0\x1B\x03\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a%\xD9W[Pa#0W`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x91\x90\x82\x81`\x04\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x80\x15a\x0B\xB4W\x82\x91_\x91a%\xA1W[P\x163\x03a%\x8AW\x83\x16\x92`@QcR\xD1\x90-`\xE0\x1B\x81R\x82\x81`\x04\x81\x88Z\xFA_\x91\x81a%[W[Pa$CW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R\x86\x90\xFD[\x84\x90\x86\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x91\x82\x81\x03a%FWP\x83;\x15a%0WP\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90U`@Q\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x83Q\x15a%\x16WP_\x80\x84\x84a%\x0B\x96Q\x91\x01\x84Z\xF4\x90=\x15a%\rW=a$\xEF\x81a5+V[\x90a$\xFD`@Q\x92\x83a5\nV[\x81R_\x81\x94=\x92\x01>aKTV[\0[``\x92PaKTV[\x92PPP4a%!W\0[c\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x90\xFD[`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\xFD[`@Q\x90c*\x87Ri`\xE2\x1B\x82R`\x04\x82\x01R\xFD[\x90\x91P\x83\x81\x81=\x83\x11a%\x83W[a%s\x81\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90\x87a$&V[P=a%iV[`@Qc\x0EV\xCF=`\xE0\x1B\x81R3`\x04\x82\x01R\x85\x90\xFD[\x80\x92P\x84\x80\x92P=\x83\x11a%\xD2W[a%\xBA\x81\x83a5\nV[\x81\x01\x03\x12a\t\x1CWa%\xCC\x82\x91a9]V[\x87a#\xFEV[P=a%\xB0V[\x90P\x81\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x85a#\xBDV[4a\t\x1CW`@6`\x03\x19\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03`\x045\x81\x81\x11a\t\x1CWa&8\x906\x90`\x04\x01a4\x16V[`$\x92\x91\x925\x91\x82\x11a\t\x1CW` \x92a&Ya\x01~\x936\x90`\x04\x01a2\xC5V[PPa::V[4a\t\x1CW` a\x01~a&s6a3\xD0V[PP\x90a9\x89V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x0B\xB4W_\x90a'qW[`\x01`\x01`\xA0\x1B\x03\x91P\x163\x14\x15\x90\x81a'fW[Pa'NW\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x80T`\xFF\x81\x16\x15a'<W`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcp\xC8\xB3w`\xE1\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[\x90P3\x14\x15\x81a&\xD9V[P` \x81=` \x11a'\xA9W[\x81a'\x8B` \x93\x83a5\nV[\x81\x01\x03\x12a\t\x1CWa'\xA4`\x01`\x01`\xA0\x1B\x03\x91a9]V[a&\xC4V[=\x91Pa'~V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01\x91`\x01`\x01`@\x1B\x03\x91\x82\x81\x16_\x19\x81\x01a+\xFBW`\xFF\x82`@\x1C\x16\x90\x81\x15a+\xEFW[Pa+\xDDWh\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x05\x17\x81Ua(0a9$V[\x92`@Q\x92a(>\x84a4pV[\x81\x84R` \x93`1`\xF8\x1B\x85\x82\x01Ra(UaHsV[a(]aHsV[\x85Q\x82\x81\x11a\x08+W\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x90a(\x92\x82Ta5\xFEV[\x97`\x1F\x98\x89\x81\x11a+\x93W[P\x87\x90\x89\x83\x11`\x01\x14a+\x17Wa(\xCB\x92\x91_\x91\x83a+\x0CWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[\x80Q\x91\x82\x11a\x08+W\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x92a)\x03\x84Ta5\xFEV[\x87\x81\x11a*\xB8W[P\x85\x96\x83\x11`\x01\x14a*\x1EWP\x90\x80\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x96a)Z\x93_\x92a*\x13WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0U_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01Ua)\xABaHsV[`\x01`\xF8\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06U`\x01`\xF9\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U`@Q`\x05\x81R\xA1\0[\x01Q\x90P\x87\x80a\x06\xA4V[\x91\x90`\x1F\x19\x82\x16\x96\x84_R\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x91_[\x89\x81\x10a*\xA3WP\x83\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x99\x10a*\x8BW[PPP\x81\x1B\x01\x90Ua)]V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x86\x80\x80a*~V[\x81\x83\x01Q\x84U\x92\x85\x01\x92\x91\x88\x01\x91\x88\x01a*MV[a*\xFD\x90\x85_R\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x89\x80\x87\x01`\x05\x1C\x82\x01\x92\x8A\x88\x10a+\x03W[\x01`\x05\x1C\x01\x90a:\xCCV[\x87a)\x0BV[\x92P\x81\x92a*\xF2V[\x01Q\x90P\x8A\x80a\x06\xA4V[\x86\x92\x91`\x1F\x19\x83\x16\x91\x85_R\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x92_[\x8C\x82\x82\x10a+}WPP\x84\x11a+eW[PPP\x81\x1B\x01\x90Ua(\xCEV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x89\x80\x80a+XV[\x83\x85\x01Q\x86U\x8B\x97\x90\x95\x01\x94\x93\x84\x01\x93\x01a+GV[a+\xD7\x90\x84_R\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x8B\x80\x86\x01`\x05\x1C\x82\x01\x92\x8C\x87\x10a+\x03W\x01`\x05\x1C\x01\x90a:\xCCV[\x89a(\x9EV[`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x90\xFD[`\x05\x91P\x10\x15\x85a(\nV[`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x90\xFD[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\xFF\x81`@\x1C\x16\x80\x15a,\x91W[a+\xDDW`\x05\x90h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x05\x81R\xA1\0[P`\x05`\x01`\x01`@\x1B\x03\x82\x16\x10\x15a,NV[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CWa,\xBDa9$V[a,\xC5aC\x96V[\x90`\x04`@Q\x91a,\xD5\x83a4pV[`\x01\x92`\x01\x81R` \x93\x84\x82\x01\x93\x856\x867`!\x83\x01\x82[a-\x96W[PPP\x90a-\x82\x92`$\x92a-\x05aC\x96V[\x91`@Q\x97\x84a-\x1E\x8A\x96Q\x80\x92\x8B\x80\x8A\x01\x91\x01a3ZV[\x85\x01a\x10;`\xF1\x1B\x89\x82\x01Ra-=\x82Q\x80\x93\x8B`\"\x85\x01\x91\x01a3ZV[\x01a-Z`\x17`\xF9\x1B\x93\x84`\"\x84\x01RQ\x80\x93`#\x84\x01\x90a3ZV[\x01\x90`#\x82\x01Ra-s\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a3ZV[\x01\x03`\x04\x81\x01\x85R\x01\x83a5\nV[a\x13&`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a3{V[_\x19\x01\x90`\n\x90o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x82\x82\x06\x1A\x83S\x04\x91\x82\x15a-\xC7W\x91\x90\x82a,\xEDV[a,\xF2V[4a\t\x1CW` \x80`\x03\x196\x01\x12a\t\x1CW`\x045_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x03\x81R`@_ T\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02\x82R`@_ \x90_R\x81R`@_ `@Q\x90\x81\x83\x82T\x91\x82\x81R\x01\x90\x81\x92_R\x84_ \x90_[\x86\x82\x82\x10a.\xB1W\x86\x86a.i\x82\x88\x03\x83a5\nV[`@Q\x92\x83\x92\x81\x84\x01\x90\x82\x85RQ\x80\x91R`@\x84\x01\x92\x91_[\x82\x81\x10a.\x91WPPPP\x03\x90\xF3[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a.\x82V[\x83T`\x01`\x01`\xA0\x1B\x03\x16\x85R\x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a.SV[4a\t\x1CWa.\xDC6a2\xF2V[\x94\x92\x91\x90\x93\x95\x96`\x01`\xF9\x1B\x88\x11\x80\x15\x90a2\x9BW[a2\x85WP\x86_R` \x94\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07\x86Ra0\xB6`@_ a/R`\x01`@Q\x92a/9\x84a4pV[`@Qa/J\x81a\x1AU\x81\x85a7\xBAV[\x84R\x01a8JV[\x90\x81\x89\x82\x01RQ\x90`@Q\x91a/g\x83a4\x8BV[\x82R\x88\x82\x01\x90\x81Ra/z6\x8B\x89a5FV[`@\x83\x01\x90\x81Ra/\x8C6\x86\x8Ba5FV[``\x84\x01\x90\x81R`m\x8B\x7Fes extraData)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80`@Q\x92a/\xC6\x84a4\xB9V[\x84\x84R\x83\x01\x92\x7FUserDecryptResponseVerification(\x84R\x7Fbytes publicKey,bytes32[] ctHand`@\x82\x01R\x7Fles,bytes userDecryptedShare,byt``\x82\x01R\x01R \x93Q\x8B\x81Q\x91\x01 \x92Qa\x04\xB7a0^\x8D`@Q\x92\x83\x91\x82\x01\x80\x95a?\xFFV[Q\x90 \x91Q\x8B\x81Q\x91\x01 \x90Q`@Qa0\x87\x8D\x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[Q\x90 \x91`@Q\x93\x8C\x85\x01\x95\x86R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x1E\x1A\x81a4\xD4V[\x96a0\xCE\x83\x85a0\xC6\x85\x8Aa@+V[\x9A\x8C\x8CaAAV[\x88_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02\x87R`@_ _\x80R\x87R`@_ a1\x0B3\x82a8\xC3V[T\x95_\x19\x87\x01\x92\x87\x84\x11a2qWa1n\x7F\x7F\xCD\xFBS\x81\x91\x7FUJq}\nTp\xA3?ZI\xBAdE\xF0^\xC4<t\xC0\xBC,\xC6\x08\xB2\x96\x8A\x96a1`\x8E\x9Aa1|\x97`\x80`@Q\x9B\x8C\x9B\x8CR\x8B\x01R`\x80\x8A\x01\x91a9\x04V[\x91\x87\x83\x03`@\x89\x01Ra9\x04V[\x91\x84\x83\x03``\x86\x01Ra9\x04V[\x03\x90\xA2\x83_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x92\x83\x83R`\xFF`@_ T\x16\x15\x91\x82a1\xF8W[PPa1\xBFW\0[\x82_RR`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xE8\x97R\xBE\x0E\xCD\xB6\x8B*n\xB5\xEF\x1A\x89\x109\xE0\xE9*\xE3\xC8\xA6\"t\xC5\x88\x1EH\xEE\xA1\xED%_\x80\xA2\0[\x90\x91P`@Q\x91c\x14\x0FE\xFF`\xE1\x1B\x83R`\x04\x83\x01R\x82\x82`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x91\x82\x15a\x0B\xB4W_\x92a2BW[P\x10\x15\x84\x80a1\xB7V[\x90\x91P\x82\x81\x81=\x83\x11a2jW[a2Z\x81\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90\x85a28V[P=a2PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x90\xFD[P\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08T\x88\x11a.\xF2V[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x83\x81\x86\x01\x95\x01\x01\x11a\t\x1CWV[`\x80`\x03\x19\x82\x01\x12a\t\x1CW`\x045\x91`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\t\x1CW\x83a3!\x91`\x04\x01a2\xC5V[\x93\x90\x93\x92`D5\x83\x81\x11a\t\x1CW\x82a3<\x91`\x04\x01a2\xC5V[\x93\x90\x93\x92`d5\x91\x82\x11a\t\x1CWa3V\x91`\x04\x01a2\xC5V[\x90\x91V[_[\x83\x81\x10a3kWPP_\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a3\\V[\x90` \x91a3\x94\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a3ZV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\t\x1CWV[`@`\x03\x19\x82\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03\x91`\x045\x83\x81\x11a\t\x1CW\x82a3\xFC\x91`\x04\x01a3\xA0V[\x93\x90\x93\x92`$5\x91\x82\x11a\t\x1CWa3V\x91`\x04\x01a2\xC5V[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x80\x85\x01\x94``\x85\x02\x01\x01\x11a\t\x1CWV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CWV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CWV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x08+W`@RV[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x08+W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a5R\x82a5+V[\x91a5``@Q\x93\x84a5\nV[\x82\x94\x81\x84R\x81\x83\x01\x11a\t\x1CW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x80\x85\x01\x94\x84`\x06\x1B\x01\x01\x11a\t\x1CWV[4a\t\x1CW`@6`\x03\x19\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03`\x045\x81\x81\x11a\t\x1CWa5\xDD\x906\x90`\x04\x01a5|V[`$\x92\x91\x925\x91\x82\x11a\t\x1CW` \x92a\x01wa\x01~\x936\x90`\x04\x01a2\xC5V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a6,W[` \x83\x10\x14a6\x18WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a6\rV[\x90_\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x90\x81T\x91a6g\x83a5\xFEV[\x80\x83R\x92` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a6\xE5WP`\x01\x14a6\x8BW[PPPPPV[\x90\x93\x94\x95P_R\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x92_\x93[\x85\x85\x10a6\xD2WPPP` \x92P\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x80T\x85\x85\x01\x84\x01R\x93\x82\x01\x93\x81\x01a6\xB7V[\x93PPPP` \x93\x94P`\xFF\x92\x91\x92\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x90_\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x90\x81T\x91a7=\x83a5\xFEV[\x80\x83R\x92` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a6\xE5WP`\x01\x14a7`WPPPPPV[\x90\x93\x94\x95P_R\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x92_\x93[\x85\x85\x10a7\xA7WPPP` \x92P\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x80T\x85\x85\x01\x84\x01R\x93\x82\x01\x93\x81\x01a7\x8CV[\x80T_\x93\x92a7\xC8\x82a5\xFEV[\x91\x82\x82R` \x93`\x01\x91`\x01\x81\x16\x90\x81_\x14a8+WP`\x01\x14a7\xEDWPPPPPV[\x90\x93\x94\x95P_\x92\x91\x92R\x83_ \x92\x84_\x94[\x83\x86\x10a8\x17WPPPP\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x80T\x85\x87\x01\x83\x01R\x94\x01\x93\x85\x90\x82\x01a7\xFFV[`\xFF\x19\x16\x86\x85\x01RPPP\x90\x15\x15`\x05\x1B\x01\x01\x91P_\x80\x80\x80\x80a6\x84V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x92` \x83\x01\x91_R` _ \x93_\x90[\x82\x82\x10a8\x80WPPPa8~\x92P\x03\x83a5\nV[V[\x85T\x84R`\x01\x95\x86\x01\x95\x88\x95P\x93\x81\x01\x93\x90\x91\x01\x90a8hV[\x80T\x82\x10\x15a8\xAFW_R` _ \x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x08+Wa8\xE5\x91`\x01\x82\x01\x81Ua8\x9AV[`\x01`\x01`\xA0\x1B\x03\x92\x91\x92\x80\x84T\x92`\x03\x1B\x93\x16\x83\x1B\x92\x1B\x19\x16\x17\x90UV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q\x90a91\x82a4pV[`\n\x82R\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CWV[\x90\x81` \x91\x03\x12a\t\x1CWQ\x80\x15\x15\x81\x03a\t\x1CW\x90V[\x81\x15a:$W_[\x82\x81\x10a9\xA0WPPP`\x01\x90V[`@\x80Qc-\xDC\x9Ao`\xE0\x1B\x81R`\x05\x83\x90\x1B\x84\x015`\x04\x82\x01R` \x80\x82`$\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a:\x1BWP_\x92a9\xFEW[PP\x15a9\xF7W`\x01\x01a9\x91V[PPP_\x90V[a:\x14\x92P\x80=\x10a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_\x80a9\xE8V[Q=_\x82>=\x90\xFD[PP_\x90V[\x91\x90\x81\x10\x15a8\xAFW``\x02\x01\x90V[\x90\x80\x15a:$W_[\x81\x81\x10a:RWPPP`\x01\x90V[a:]\x81\x83\x85a:*V[`@\x80Qc-\xDC\x9Ao`\xE0\x1B\x81R\x915`\x04\x83\x01R\x90` \x80\x82`$\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a:\x1BWP_\x92a:\xAFW[PP\x15a9\xF7W`\x01\x01a:CV[a:\xC5\x92P\x80=\x10a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_\x80a:\xA0V[\x81\x81\x10a:\xD7WPPV[_\x81U`\x01\x01a:\xCCV[\x91\x90`\x1F\x81\x11a:\xF1WPPPV[a8~\x92_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a;\x1BW[`\x1F\x01`\x05\x1C\x01\x90a:\xCCV[\x90\x91P\x81\x90a;\x0EV[\x91\x90\x81\x10\x15a8\xAFW`\x06\x1B\x01\x90V[\x90\x80\x15a:$W_[\x81\x81\x10a;MWPPP`\x01\x90V[a;X\x81\x83\x85a;%V[`@\x80Qc-\xDC\x9Ao`\xE0\x1B\x81R\x915`\x04\x83\x01R\x90` \x80\x82`$\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a:\x1BWP_\x92a;\xAAW[PP\x15a9\xF7W`\x01\x01a;>V[a;\xC0\x92P\x80=\x10a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_\x80a;\x9BV[`\x01`\x01`@\x1B\x03\x81\x11a\x08+W`\x05\x1B` \x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\t\x1CW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\t\x1CW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\t\x1CWV[`@\x90`#\x19\x01\x12a\t\x1CW`@Q\x90a<,\x82a4pV[`$5\x82R`D5` \x83\x01RV[`d5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\t\x1CW\x90V[`\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\t\x1CW\x90V[\x92\x91a<r\x82a;\xC7V[\x91a<\x80`@Q\x93\x84a5\nV[\x82\x94\x81\x84R` \x80\x94\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\t\x1CW\x90[\x82\x82\x10a<\xA6WPPPPV[\x83\x80\x91a<\xB2\x84a4\\V[\x81R\x01\x91\x01\x90a<\x99V[\x91\x90\x80\x82R` \x80\x92\x01\x92\x91_[\x82\x81\x10a<\xD9WPPPP\x90V[\x90\x91\x92\x93\x82\x80`\x01\x92`\x01`\x01`\xA0\x1B\x03a<\xF3\x89a4\\V[\x16\x81R\x01\x95\x01\x93\x92\x91\x01a<\xCBV[`@\x90`\x01`\x01`\xA0\x1B\x03a=\"\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a<\xBDV[\x90V[` \x90\x81\x81\x84\x03\x12a\t\x1CW\x80Q`\x01`\x01`@\x1B\x03\x91\x82\x82\x11a\t\x1CW\x01\x90\x83`\x1F\x83\x01\x12\x15a\t\x1CW\x81Q\x90a=\\\x82a;\xC7V[\x94`@a=k\x81Q\x97\x88a5\nV[\x83\x87R\x85\x87\x01\x92\x86`\x05\x95`\x05\x1B\x87\x01\x01\x95\x83\x87\x11a\t\x1CW\x87\x81\x01\x94[\x87\x86\x10a=\x9CWPPPPPPPPP\x90V[\x85Q\x83\x81\x11a\t\x1CW\x82\x01`\x80\x80`\x1F\x19\x83\x89\x03\x01\x12a\t\x1CW\x85Q\x91a=\xC2\x83a4\x8BV[\x8B\x81\x01Q\x83R\x86\x81\x01Q\x8C\x84\x01R``\x91\x82\x82\x01Q\x88\x85\x01R\x81\x01Q\x90\x86\x82\x11a\t\x1CW\x01\x90\x87`?\x83\x01\x12\x15a\t\x1CW\x81\x8C\x80\x93\x01Q\x88a>\x03\x82a;\xC7V[\x94a>\x10\x82Q\x96\x87a5\nV[\x82\x86R\x85\x01\x91\x8D\x1B\x83\x01\x01\x91\x8A\x83\x11a\t\x1CW\x91\x89\x8F\x96\x94\x92\x97\x95\x93\x01[\x81\x81\x10a>HWPP\x84\x95P\x82\x01R\x81R\x01\x95\x01\x94a=\x89V[\x91\x93\x95\x80\x91\x93\x95\x97a>Y\x84a9]V[\x81R\x01\x91\x01\x91\x8E\x95\x93\x91\x96\x94\x92a>.V[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a>\x92WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a>\x84V[_\x19\x81\x14a2qW`\x01\x01\x90V[\x90h\x01\0\0\0\0\0\0\0\0\x81\x11a\x08+W\x81T\x91\x81\x81U\x82\x82\x10a>\xD7WPPPV[_R` _ \x91\x82\x01\x91\x01[\x81\x81\x10a>\xEEWPPV[_\x81U`\x01\x01a>\xE3V[`\x80\x82\x01\x81Q\x83R` `\xA0``\x82\x94\x83\x81\x01Q\x84\x88\x01R`@\x81\x01Q`@\x88\x01R\x01Q\x94`\x80``\x82\x01R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a?>WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a?0V[\x90\x80\x82Q\x90\x81\x81R` \x80\x91\x01\x92` \x80\x84`\x05\x1B\x83\x01\x01\x95\x01\x93_\x91[\x84\x83\x10a?\x89WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a?\xA7`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x8AQa>\xF9V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a?yV[\x94\x92\x93a=\"\x96\x94`\x01`\x01`\xA0\x1B\x03a?\xDDa?\xF1\x95\x94`\x80\x8AR`\x80\x8A\x01\x90a?[V[\x93\x16` \x88\x01R\x86\x83\x03`@\x88\x01Ra9\x04V[\x92``\x81\x85\x03\x91\x01Ra9\x04V[\x80Q` \x80\x92\x01\x91_[\x82\x81\x10a@\x17WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a@\tV[\x81\x15aA\x04W\x805`\xF8\x1C\x91\x82\x15a@\x96W`\x01\x83\x14a@^W`@Qc\x08Ns\x0B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x90\xFD[\x90\x91P`!\x81\x10a@wW`!\x11a\t\x1CW`\x01\x015\x90V[`D\x90`@Q\x90cI\xAAE3`\xE1\x1B\x82R`\x04\x82\x01R`!`$\x82\x01R\xFD[PP`@Qc\x97o>\xB9`\xE0\x1B\x81R\x90P` \x81`\x04\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a@\xD5WP\x90V[\x90P` \x81=` \x11a@\xFCW[\x81a@\xF0` \x93\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90V[=\x91Pa@\xE3V[PP`@Qc\x97o>\xB9`\xE0\x1B\x81R` \x81`\x04\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a@\xD5WP\x90V[\x94\x93\x91a\x05UaA`\x94aAW\x93\x946\x91a5FV[\x90\x93\x91\x93aI\xF8V[`@\x80Qc%\x11\xF3\xF5`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x90\x92` \x92\x90\x91s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90\x84\x81`D\x81\x85Z\xFA\x90\x81\x15aC\x8CW_\x91aCoW[P\x15aCOW\x84Qc\x06?\xE89`\xE3\x1B\x81R`\x04\x81\x01\x97\x90\x97R3`$\x88\x01R_\x90\x87\x90`D\x90\x82\x90Z\xFA\x80\x15aCEW\x83\x94\x95\x96_\x91aB\xA7W[P`\x01`\x01`\xA0\x1B\x03\x94\x85\x91\x01Q\x16\x93\x82\x16\x80\x94\x03aB\x89W\x80_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x01\x91\x82\x84R\x85_ \x85_R\x84R`\xFF\x86_ T\x16aBbWP_R\x81R\x82_ \x91_RR_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[\x85Qc\x99\xECH\xD9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x90\xFD[\x84Qc\r\x86\xF5!`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R3`$\x82\x01R`D\x90\xFD[\x93PP=\x80_\x85>aB\xB9\x81\x85a5\nV[\x83\x01\x84\x84\x82\x03\x12a\t\x1CW\x83Q`\x01`\x01`@\x1B\x03\x94\x85\x82\x11a\t\x1CW\x01`\x80\x81\x83\x03\x12a\t\x1CW\x86Q\x91aB\xED\x83a4\x8BV[aB\xF6\x82a9]V[\x83RaC\x03\x87\x83\x01a9]V[\x87\x84\x01R\x87\x82\x01Q\x86\x81\x11a\t\x1CW\x81aC\x1E\x91\x84\x01aH1V[\x88\x84\x01R``\x82\x01Q\x95\x86\x11a\t\x1CW\x86\x95aC:\x92\x01aH1V[``\x82\x01R_aA\xF3V[\x84Q=_\x82>=\x90\xFD[\x84Qc\x15>7{`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x90\xFD[aC\x86\x91P\x85=\x87\x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_aA\xB7V[\x86Q=_\x82>=\x90\xFD[`@Q_aC\xA3\x82a4pV[`\x01\x90`\x01\x83R` 6\x81\x85\x017`!\x83\x01\x82[aC\xC2W[PPP\x90V[_\x19\x01\x90`\n\x90o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x82\x82\x06\x1A\x83S\x04\x91\x82\x15aC\xF3W\x91\x90\x82aC\xB7V[aC\xBCV[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16aD#WV[`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x90\xFD[\x91\x90\x82\x01\x80\x92\x11a2qWV[` \x81\x01\x80Q\x15aD\xD9W\x80Qa\x01m\x90\x81\x81\x11aD\xBBWPP\x81QB\x81\x11a\x12IWP\x81Q\x90Qb\x01Q\x80\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a2qWaD\x8A\x90B\x92aD5V[\x10aD\x92WPV[`@Qb\xC0\xD2\x01`\xE6\x1B\x81RB`\x04\x82\x01R\x81Q`$\x82\x01R` \x90\x91\x01Q`D\x82\x01R`d\x90\xFD[`D\x92P`@Q\x91c2\x95\x18c`\xE0\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[`@Qc\xDE(Y\xC1`\xE0\x1B\x81R`\x04\x90\xFD[\x80Q\x15a8\xAFW` \x01\x90V[\x80Q\x82\x10\x15a8\xAFW` \x91`\x05\x1B\x01\x01\x90V[\x90_[\x82Q\x81\x10\x15a9\xF7W`\x01`\x01`\xA0\x1B\x03\x80aE+\x83\x86aD\xF8V[Q\x16\x90\x83\x16\x14aE=W`\x01\x01aE\x0FV[PPP`\x01\x90V[\x90aEO\x82a;\xC7V[aE\\`@Q\x91\x82a5\nV[\x82\x81R\x80\x92aEm`\x1F\x19\x91a;\xC7V[\x01\x90` 6\x91\x017V[\x92\x91\x90\x80\x15aF\x88WaE\x89\x81aEEV[\x93_\x92_[\x83\x81\x10aE\xA8WPPPPa\x08\0\x90\x81\x81\x11a\x0B\xEDWPPV[aE\xB3\x81\x85\x85a;%V[5` \x95\x86aE\xC3\x84\x88\x88a;%V[\x015\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CW`\x01`\x01`@\x1B\x03\x83`\x10\x1C\x16\x855\x80\x82\x03aFbWPPaF\x01\x90a\xFF\xFFa\t\xBBa\t\xB6\x86aH\xB4V[\x96\x84\x01aF\x1C\x82a\x03(aF\x15\x84\x89a;\xDEV[6\x91a<gV[\x15aF8WPP\x90`\x01\x91aF1\x82\x8AaD\xF8V[R\x01aE\x8EV[\x90aFFa\x08'\x92\x86a;\xDEV[`@Qc\xA4\xC3\x03\x91`\xE0\x1B\x81R\x93\x84\x93\x90\x92\x90`\x04\x85\x01a=\x02V[`@QcJ\xC8t\x8B`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x92\x90\x92R`D\x82\x01R`d\x90\xFD[`@Qc\xA6\xA6\xCB!`\xE0\x1B\x81R`\x04\x90\xFD[`\x01\x90`\x01\x81Q\x11\x15aGGW\x90\x80\x92\x91` \x80aF\xB7\x83aD\xEBV[Q\x01Q\x90`\x01\x95[aF\xCCW[PPPP\x90PV[\x82Q\x86\x10\x15aGBW\x81\x81aF\xE1\x88\x86aD\xF8V[Q\x01Q\x03aF\xF3W\x94\x83\x01\x94\x83aF\xBFV[\x82aG\x08\x87aG\x01\x83aD\xEBV[Q\x92aD\xF8V[Qa\x08'`@\x91aG0\x83Q\x94\x85\x94c\xCF\xAE\x92\x1F`\xE0\x1B\x86R`\x04\x86\x01R`D\x85\x01\x90a>\xF9V[\x83\x81\x03`\x03\x19\x01`$\x85\x01R\x90a>\xF9V[aF\xC4V[PPV[s3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNn\x90\x81;\x15a\t\x1CW`\x01`\x01`\xA0\x1B\x03`$_\x92\x83`@Q\x95\x86\x94\x85\x93c\x98\x8A--`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x0B\xB4WaG\x9CWPV[a8~\x90a4\xA6V[a=\"\x90aG\xB1aK\xB7V[a\x04\xB7aH\x11aG\xBFaL)V[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01\x90\x81R\x91\x81\x01\x95\x90\x95R``\x85\x01\x91\x90\x91RF`\x80\x85\x01R0`\xA0\x85\x01R\x92\x91\x82\x90`\xC0\x82\x01\x90V[Q\x90 `B\x91`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x81`\x1F\x82\x01\x12\x15a\t\x1CW\x80QaHG\x81a5+V[\x92aHU`@Q\x94\x85a5\nV[\x81\x84R` \x82\x84\x01\x01\x11a\t\x1CWa=\"\x91` \x80\x85\x01\x91\x01a3ZV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15aH\xA2WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD[`\x08\x1C`\xFF\x16`S\x81\x11aH\xE2W`T\x81\x10\x15aH\xCEW\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`$\x90`@Q\x90cd\x19P\xD7`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`T\x81\x10\x15aH\xCEW\x80aI\x0EWP`\x02\x90V[`\x02\x81\x03aI\x1CWP`\x08\x90V[`\x03\x81\x03aI*WP`\x10\x90V[`\x04\x81\x03aI8WP` \x90V[`\x05\x81\x03aIFWP`@\x90V[`\x06\x81\x03aITWP`\x80\x90V[`\x07\x81\x03aIbWP`\xA0\x90V[`\x08\x81\x03aIqWPa\x01\0\x90V[`$\x90`@Q\x90c\xBEx0\xB1`\xE0\x1B\x82R`\x04\x82\x01R\xFD[\x80Q` \x80\x92\x01\x91_[\x82\x81\x10aI\xA1WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01aI\x93V[\x81Q\x91\x90`A\x83\x03aI\xEEWaI\xE7\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aJ\xD2V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aH\xCEW\x80aJ\nWPPV[`\x01\x81\x03aJ$W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x90\xFD[`\x02\x81\x03aJEW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`\x03\x14aJOWPV[`$\x90`@Q\x90c5\xE2\xF3\x83`\xE2\x1B\x82R`\x04\x82\x01R\xFD[\x90a=\"\x91aJtaK\xB7V[aH\x11aJ\x7FaL)V[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01\x90\x81R\x91\x81\x01\x94\x90\x94R``\x84\x01\x91\x90\x91R`\x80\x83\x01\x93\x90\x93R0`\xA0\x83\x01R\x81`\xC0\x81\x01a\x04\xB7V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aKIW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x0B\xB4W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aK?W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x90aK{WP\x80Q\x15aKiW` \x81Q\x91\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x90\xFD[\x81Q\x15\x80aK\xAEW[aK\x8CWP\x90V[`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[P\x80;\x15aK\x84V[`@QaK\xC7\x81a\x1AU\x81a66V[\x80Q\x90\x81\x15aK\xD7W` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x80\x15aL\x04W\x90V[P\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x90V[`@QaL9\x81a\x1AU\x81a7\x0CV[\x80Q\x90\x81\x15aLIW` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x80\x15aL\x04W\x90V",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60a0806040526004361015610012575f80fd5b5f905f3560e01c908163046f9eb314612ece575080630900cc6914612dcc5780630d8e6e2c14612ca5578063123abb2814612c0d57806339f73810146127b15780633f4ba83a1461267b5780634014c4cd14612660578063410bf0ba146126075780634f1ef2861461234257806352d1902d146122c657806358f5b8ab146122785780635c975abb146122375780636f8913bc14611c7e57806376227eed146109245780638456cb5914611b7e57806384b0196e14611a075780639fad5a2f1461132a578063ad3cb1cc146112c9578063b4de2c3714610c2d578063d8998f4514610929578063e22d1b2614610924578063f1b57adb1461018f5763fbb832591461011b575f80fd5b3461018857606036600319011261018857610134613446565b506001600160401b039060243582811161018b5761015690369060040161357c565b92909160443591821161018857602061017e858561017736600488016132c5565b5050613b35565b6040519015158152f35b80fd5b5080fd5b5034610188576003196101003682011261018b576004356001600160401b038111610920576101c290369060040161357c565b90916040366023190112610918576001600160401b03606435116109185760409060643536030112610920576001600160a01b03608435166084350361091c5760a4356001600160401b038111610918576102219036906004016132c5565b909160c4356001600160401b038111610914576102429036906004016132c5565b949060e4356001600160401b038111610910576102639036906004016132c5565b93909261026e6143f8565b604051635ff9d55d60e11b815260046064358101359082015260208160248173d582ec82a1758322907df80da8a754e12a5acb955afa908115610905578a916108d6575b50156108b9576102cc602460643501606435600401613bde565b9050156108a757600a6102e9602460643501606435600401613bde565b905011610873576103016102fc36613c13565b614442565b61032d61032861031b602460643501606435600401613bde565b9190608435923691613c67565b61450c565b61083f576103419160643560040191614577565b95610356602460643501606435600401613bde565b604051918260a08101106001600160401b0360a08501111761082b578261039d61054a9261055b9460a061056497016040526103948d8d3691613546565b84523691613c67565b60208201908152602435604083015260443560608301526103bf368a8a613546565b608083015260876040516103d2816134d4565b8181527f726144617461290000000000000000000000000000000000000000000000000060a060208301927f557365724465637279707452657175657374566572696669636174696f6e286284527f79746573207075626c69634b65792c616464726573735b5d20636f6e7472616360408201527f744164647265737365732c75696e7432353620737461727454696d657374616d60608201527f702c75696e74323536206475726174696f6e446179732c6279746573206578746080820152015220916104b76104c582516020815191012093516040519283916020830190614989565b03601f19810183528261350a565b6020815191012090604081015160806060830151920151604051610506602082816104f9818301968781519384920161335a565b810103808452018261350a565b51902092604051946020860196875260408601526060850152608084015260a083015260c082015260c0815261053b816134ef565b51902060643560040135614a67565b610555368587613546565b906149be565b909291926149f8565b6001600160a01b03806084351691160361080357505060405163a14f897160e01b8152928684806105988960048301613e6b565b038173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa9384156107f85787946107d4575b506105c98461469a565b6105f37f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70854613ea6565b95867f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee708556040519061062482613470565b61062f368489613546565b825260208201528688527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee707602052604088209080518051906001600160401b038211610752576106898261068386546135fe565b86613ae2565b602090601f8311600114610771576106b892918c9183610766575b50508160011b915f199060031b1c19161790565b82555b60206001809301910151908151916001600160401b038311610752576020906106e48484613eb4565b01908a5260208a208a5b83811061073f578b8b7ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b8c8c6107398d8d8d6107293361474b565b6040519586956084359087613fb7565b0390a280f35b84906020845194019381840155016106ee565b634e487b7160e01b8b52604160045260248bfd5b015190505f806106a4565b848c5260208c209190601f1984168d5b8181106107bc57509084600195949392106107a4575b505050811b0182556106bb565b01515f1960f88460031b161c191690555f8080610797565b92936020600181928786015181550195019301610781565b6107f19194503d8089833e6107e9818361350a565b810190613d25565b925f6105bf565b6040513d89823e3d90fd5b610827604051928392632a873d2760e01b8452602060048501526024840191613904565b0390fd5b634e487b7160e01b5f52604160045260245ffd5b610853602460643501606435600401613bde565b60405163dc4d78b160e01b81529182916108279160843560048501613d02565b6044610889602460643501606435600401613bde565b90506040519063af1f049560e01b8252600a60048301526024820152fd5b6040516357cfa21760e01b8152600490fd5b602460405163b6679c3b60e01b8152606435600401356004820152fd5b6108f8915060203d6020116108fe575b6108f0818361350a565b810190613971565b5f6102b2565b503d6108e6565b6040513d8c823e3d90fd5b8780fd5b8580fd5b8380fd5b5f80fd5b8280fd5b6135ac565b503461091c57610938366133d0565b919390926109446143f8565b8415610c1b5761095385613bc7565b92610961604051948561350a565b8584526020808501918760051b928385019036821161091c578386915b838310610c0b57505050505f965f975b87518910156109ca576109c260019161ffff6109bb6109b66109b08e8e6144f8565b516148b4565b6148fa565b1690614435565b98019761098e565b899061080090818111610bed5750506040519463a14f897160e01b86528460048701528160248701527f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821161091c5785604481835f948b84840137810103018173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa948515610bb4575f95610bd1575b50610a5a8561469a565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70695610a868754613ea6565b809755865f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee705855260405f20906001600160401b03831161082b57610acc8383613eb4565b905f52845f205f5b838110610bbf57505050507333e0c7a03d2b040b518580c365f4b3bde7cc4e6e803b1561091c575f809160246040518094819363247bac9f60e21b83523360048401525af18015610bb457610b71575b50610b64927f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff949261073992604051958695604087526040870190613f5b565b9285840390860152613904565b610739919650927f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff9492610ba7610b64956134a6565b5f97925092945092610b24565b6040513d5f823e3d90fd5b82358282015591860191600101610ad4565b610be69195503d805f833e6107e9818361350a565b9387610a50565b604492506040519163e7f4895d60e01b835260048301526024820152fd5b823581529181019185910161097e565b6040516305bcea8760e31b8152600490fd5b3461091c5761010036600319011261091c576004356001600160401b03811161091c57610c5e903690600401613416565b906024356001600160a01b038116810361091c576044356001600160401b03811161091c57610c919036906004016132c5565b6064356001600160401b03811161091c57610cb09036906004016133a0565b90604036608319011261091c5760c4356001600160401b03811161091c57610cdc9036906004016132c5565b9290966001600160401b0360e4351161091c57610cfe3660e4356004016132c5565b959096610d096143f8565b8a156112b757600a841161129757604051610d2381613470565b608435815260a435602082015260a435156112855760208101516301e1338090818111611267575050805142811161124957508051610d684291602084015190614435565b1061121f5750610d773361474b565b610d808b614545565b9860109b604051635ff9d55d60e11b81526001600160401b03863560101c16600482015260208160248173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f91611200575b50156111db575f9c5f6080525b816080511061113a5750610800808e1161111c575060405163a14f897160e01b81529b9c50999a995f8b80610e148f60048301613e6b565b038173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa9a8b15610bb4575f9b611100575b50610e458b61469a565b610e6f7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70854613ea6565b9b8c7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70855604051610e9f81613470565b610eaa368787613546565b8152602081019182528d5f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70760205260405f2090518051906001600160401b03821161082b57610f0582610eff85546135fe565b85613ae2565b602090601f8311600114611099579180610f3792600195945f926107665750508160011b915f199060031b1c19161790565b81555b019051908151916001600160401b03831161082b57602090610f5c8484613eb4565b01905f5260205f205f5b83811061108557505050506020610f896040519c8d610120908181520190613f5b565b8c8103828e01528281520194905f5b818110611038575050509461102494610ff87f044711c64cf18abf4960220e3e1ecf01059c72b8152c8415b58ea1fdd84a2b4b9c9d958c9b9995611006958d60406001600160a01b036110339f9d169101528d6060818503910152613904565b918a830360808c0152613cbd565b9160843560a089015260a43560c089015287830360e0890152613904565b91848303610100860152613904565b0390a2005b909195600190873581526001600160a01b0361105660208a0161345c565b1660208201526001600160a01b0361107060408a0161345c565b16604082015260609081019701929101610f98565b600190602084519401938184015501610f66565b90601f19831691845f5260205f20925f5b8181106110e857509160019594929183879593106110d0575b505050811b018155610f3a565b01515f1960f88460031b161c191690555f80806110c3565b929360206001819287860151815501950193016110aa565b611115919b503d805f833e6107e9818361350a565b998d610e3b565b6044908e6040519163e7f4895d60e01b835260048301526024820152fd5b6001600160401b039d6111506080518489613a2a565b359e8f831c166001600160401b03883560101c16810361119c575061117f908f6109bb6109b661ffff926148b4565b9d61118d8d608051906144f8565b52600160805101608052610ddc565b8f61082789916001600160401b0393604051948594634ac8748b60e11b86523560101c1691600485016040919493926060820195825260208201520152565b60405163b6679c3b60e01b8152853560101c6001600160401b03166004820152602490fd5b611219915060203d6020116108fe576108f0818361350a565b8e610dcf565b6040516333c7e7e760e11b8152426004820152815160248201526020909101516044820152606490fd5b6044906040519063f24c088760e01b82524260048301526024820152fd5b6044925060405191635729758960e11b835260048301526024820152fd5b604051631229e23760e21b8152600490fd5b60405163af1f049560e01b8152600a600482015260248101859052604490fd5b60405163240e930960e01b8152600490fd5b3461091c575f36600319011261091c576113266040516112e881613470565b600581527f352e302e30000000000000000000000000000000000000000000000000000000602082015260405191829160208352602083019061337b565b0390f35b3461091c576003196101203682011261091c576004356001600160401b03811161091c5761135c90369060040161357c565b9091604036602319011261091c57604036606319011261091c576001600160401b0360a4351161091c5760409060a4353603011261091c5760c4356001600160401b03811161091c576113b39036906004016132c5565b60e4356001600160401b03811161091c576113d29036906004016132c5565b9490610104356001600160401b03811161091c576113f49036906004016132c5565b9690926113ff6143f8565b604051635ff9d55d60e11b8152600460a4358101359082015260208160248173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f916119e8575b50156119cb57602460a435019661145f8860a435600401613bde565b9050156108a757600a6114778960a435600401613bde565b9050116119ba5761148a6102fc36613c13565b6114b161032861149f8a60a435600401613bde565b91906114a9613c3b565b923691613c67565b611983576114d5916114c99160a43560040191614577565b9660a435600401613bde565b906114de613c3b565b604051928360c08101106001600160401b0360c08601111761082b576001600160a01b03926115229160c08601604052611519368b8d613546565b86523691613c67565b602084015216604082015260243560608201526044356080820152611548368986613546565b60a0820152611555613c51565b60a9604051611563816134ef565b8181527f787472614461746129000000000000000000000000000000000000000000000060c060208301927f44656c656761746564557365724465637279707452657175657374566572696684527f69636174696f6e286279746573207075626c69634b65792c616464726573735b60408201527f5d20636f6e74726163744164647265737365732c616464726573732064656c6560608201527f6761746f72416464726573732c75696e7432353620737461727454696d65737460808201527f616d702c75696e74323536206475726174696f6e446179732c6279746573206560a082015201522091805160208151910120906104b761167260208301516040519283916020830190614989565b602081519101206001600160a01b0360408301511660608301519160a060808501519401516040516116b4602082816104f9818301968781519384920161335a565b5190209460405197602089015260408801526060870152608086015260a085015260c084015260e083015260e08252816101008101106001600160401b036101008401111761082b576001600160a01b03809161173761172c8561010061174097016040526020815191012060a43560040135614a67565b61055536888a613546565b909591956149f8565b1691160361080357505060405163a14f897160e01b8152915f83806117688860048301613e6b565b038173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315610bb4575f93611967575b506117998361469a565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee708946117c58654613ea6565b809655604051906117d582613470565b6117e0368488613546565b825260208201908152865f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70760205260405f2091518051906001600160401b03821161082b576118358261068386546135fe565b602090601f83116001146119035761186392915f91836118f85750508160011b915f199060031b1c19161790565b82555b60018092019051908151916001600160401b03831161082b5760209061188c8484613eb4565b01905f5260205f205f5b8381106118e557897ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b8a8a6110338f8c8c6118d03361474b565b6118d8613c51565b9560405196879687613fb7565b8490602084519401938184015501611896565b015190508b806106a4565b90601f19831691855f5260205f20925f5b81811061194f5750908460019594939210611937575b505050811b018255611866565b01515f1960f88460031b161c191690558a808061192a565b92936020600181928786015181550195019301611914565b61197c9193503d805f833e6107e9818361350a565b918661178f565b6108278861199e611992613c3b565b9160a435600401613bde565b60405163c3446ac760e01b815293849390929060048501613d02565b60446108898960a435600401613bde565b602460405163b6679c3b60e01b815260a435600401356004820152fd5b611a01915060203d6020116108fe576108f0818361350a565b89611443565b3461091c575f36600319011261091c577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100541580611b55575b15611b1057604051611a5c81611a5581613636565b038261350a565b604051611a6c81611a558161370c565b6040516020808201928284106001600160401b0385111761082b57916020611ac58594611ab797966040525f8452604051978897600f60f81b895260e0858a015260e089019061337b565b90878203604089015261337b565b914660608701523060808701525f60a087015285830360c087015251918281520192915f5b828110611af957505050500390f35b835185528695509381019392810192600101611aea565b60405162461bcd60e51b815260206004820152601560248201527f4549503731323a20556e696e697469616c697a656400000000000000000000006044820152606490fd5b507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1015415611a40565b3461091c575f36600319011261091c5760405163237dfb4760e11b815233600482015273d582ec82a1758322907df80da8a754e12a5acb9590602081602481855afa908115610bb4575f91611c5f575b50159081611c54575b50611c3c57611be46143f8565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300600160ff198254161790557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a2586020604051338152a1005b60405163388916bb60e01b8152336004820152602490fd5b905033141581611bd7565b611c78915060203d6020116108fe576108f0818361350a565b82611bce565b3461091c57611c8c366132f2565b929394600160f89392931b871180159061220d575b6121f457865f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70560205260405f20604051606081018181106001600160401b0382111761082b57611e2292611cf99160405261384a565b8152611d06368985613546565b6020820152611d16368787613546565b60408201526054604051611d298161348b565b8181527f756c742c62797465732065787472614461746129000000000000000000000000606060208301927f5075626c696344656372797074566572696669636174696f6e2862797465733384527f325b5d20637448616e646c65732c6279746573206465637279707465645265736040820152015220906104b7611db982516040519283916020830190613fff565b60208151910120906040602082015160208151910120910151604051611def602082816104f9818301968781519384920161335a565b51902090604051926020840194855260408401526060830152608082015260808152611e1a816134b9565b5190206147a5565b611e2c858561402b565b92611e3a8188848c88614141565b885f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70460205260405f20825f5260205260405f209687546801000000000000000081101561082b57806001611e9392018a558961389a565b9190916121e1576001600160401b03831161082b578282611ec08d95611eba8e96546135fe565b83613ae2565b5f601f831160011461214957611f719383611f9493611f1a827f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e59997611f7f965f9161213e575b508160011b915f199060031b1c19161790565b90555b875f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70260205260405f20895f52602052611f5c60405f2033906138c3565b6040519586956080875260808701908c613904565b918583036020870152613904565b33604084015282810360608401528a8a613904565b0390a2875f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700928360205260ff60405f2054161590816120bf575b50611fd757005b61202b92885f5260205260405f20600160ff198254161790557f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70360205260405f205560405195606087526060870191613904565b84810360208601528354808252602082019160208260051b820101955f5260205f20925f915b83831061209357505050508484036040860152507fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f9392839261103392613904565b9091929396602060016120b08193601f198682030187528b6137ba565b99019301930191939290612051565b90508654604051916361d5552d60e11b8352600483015260208260248173d582ec82a1758322907df80da8a754e12a5acb955afa918215610bb4575f9261210a575b50101589611fd0565b9091506020813d602011612136575b816121266020938361350a565b8101031261091c5751908a612101565b3d9150612119565b90508501355f611f07565b815f5260205f20905f5b601f19851681106121c357509383611f9493611f7f93611f71977f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e59997601f198116106121aa575b5050600182811b019055611f1d565b8401355f19600385901b60f8161c191690555f8061219b565b8582013583558f97508e965060019092019160209182019101612153565b634e487b7160e01b5f525f60045260245ffd5b604051636a457ca160e11b815260048101889052602490fd5b507f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee706548711611ca1565b3461091c575f36600319011261091c57602060ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f0330054166040519015158152f35b3461091c57602036600319011261091c576004355f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700602052602060ff60405f2054166040519015158152f35b3461091c575f36600319011261091c576001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001630036123305760206040517f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8152f35b60405163703e46dd60e11b8152600490fd5b604036600319011261091c57612356613446565b60249081356001600160401b03811161091c573660238201121561091c576123879036908481600401359101613546565b6001600160a01b03807f0000000000000000000000000000000000000000000000000000000000000000168030149081156125d9575b5061233057604051638da5cb5b60e01b815260209190828160048173d582ec82a1758322907df80da8a754e12a5acb955afa8015610bb45782915f916125a1575b5016330361258a578316926040516352d1902d60e01b81528281600481885afa5f918161255b575b5061244357604051634c9c8ce360e01b8152600481018690528690fd5b8490867f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc918281036125465750833b156125305750817fffffffffffffffffffffffff0000000000000000000000000000000000000000825416179055604051907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b5f80a283511561251657505f80848461250b96519101845af4903d1561250d573d6124ef8161352b565b906124fd604051928361350a565b81525f81943d92013e614b54565b005b60609250614b54565b925050503461252157005b63b398979f60e01b8152600490fd5b604051634c9c8ce360e01b815260048101849052fd5b60405190632a87526960e21b82526004820152fd5b9091508381813d8311612583575b612573818361350a565b8101031261091c57519087612426565b503d612569565b604051630e56cf3d60e01b81523360048201528590fd5b809250848092503d83116125d2575b6125ba818361350a565b8101031261091c576125cc829161395d565b876123fe565b503d6125b0565b9050817f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc54161415856123bd565b3461091c57604036600319011261091c576001600160401b0360043581811161091c57612638903690600401613416565b60249291923591821161091c5760209261265961017e9336906004016132c5565b5050613a3a565b3461091c57602061017e612673366133d0565b505090613989565b3461091c575f36600319011261091c57604051638da5cb5b60e01b815273d582ec82a1758322907df80da8a754e12a5acb9590602081600481855afa8015610bb4575f90612771575b6001600160a01b039150163314159081612766575b5061274e577fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300805460ff81161561273c5760ff191690557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa6020604051338152a1005b604051638dfc202b60e01b8152600490fd5b6040516370c8b37760e11b8152336004820152602490fd5b9050331415816126d9565b506020813d6020116127a9575b8161278b6020938361350a565b8101031261091c576127a46001600160a01b039161395d565b6126c4565b3d915061277e565b3461091c575f36600319011261091c577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0080546001916001600160401b03918281165f198101612bfb5760ff8260401c16908115612bef575b50612bdd5768ffffffffffffffffff191668010000000000000005178155612830613924565b926040519261283e84613470565b818452602093603160f81b85820152612855614873565b61285d614873565b855182811161082b577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1029061289282546135fe565b97601f98898111612b93575b508790898311600114612b17576128cb92915f9183612b0c5750508160011b915f199060031b1c19161790565b90555b805191821161082b577fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1039261290384546135fe565b878111612ab8575b5085968311600114612a1e575090807fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29661295a935f92612a135750508160011b915f199060031b1c19161790565b90555b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100555f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d101556129ab614873565b600160f81b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70655600160f91b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70855805468ff00000000000000001916905560405160058152a1005b0151905087806106a4565b9190601f19821696845f527f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b75915f5b898110612aa35750837fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29910612a8b575b505050811b01905561295d565b01515f1960f88460031b161c19169055868080612a7e565b81830151845592850192918801918801612a4d565b612afd90855f527f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b758980870160051c8201928a8810612b03575b0160051c0190613acc565b8761290b565b92508192612af2565b015190508a806106a4565b869291601f19831691855f527f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d925f5b8c828210612b7d5750508411612b65575b505050811b0190556128ce565b01515f1960f88460031b161c19169055898080612b58565b8385015186558b97909501949384019301612b47565b612bd790845f527f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d8b80860160051c8201928c8710612b03570160051c0190613acc565b8961289e565b60405163f92ee8a960e01b8152600490fd5b6005915010158561280a565b604051636f4f731f60e01b8152600490fd5b3461091c575f36600319011261091c577ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805460ff8160401c168015612c91575b612bdd5760059068ffffffffffffffffff19161790557fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2602060405160058152a1005b5060056001600160401b0382161015612c4e565b3461091c575f36600319011261091c57612cbd613924565b612cc5614396565b90600460405191612cd583613470565b60019260018152602093848201938536863760218301825b612d96575b50505090612d8292602492612d05614396565b916040519784612d1e8a965180928b808a01910161335a565b850161103b60f11b89820152612d3d825180938b60228501910161335a565b01612d5a601760f91b93846022840152518093602384019061335a565b01906023820152612d738251809388878501910161335a565b0103600481018552018361350a565b61132660405192828493845283019061337b565b5f190190600a906f181899199a1a9b1b9c1cb0b131b232b360811b8282061a835304918215612dc757919082612ced565b612cf2565b3461091c5760208060031936011261091c576004355f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee703815260405f20547f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee702825260405f20905f52815260405f20604051908183825491828152019081925f52845f20905f5b86828210612eb1578686612e698288038361350a565b60405192839281840190828552518091526040840192915f5b828110612e9157505050500390f35b83516001600160a01b031685528695509381019392810192600101612e82565b83546001600160a01b031685529093019260019283019201612e53565b3461091c57612edc366132f2565b94929190939596600160f91b881180159061329b575b6132855750865f526020947f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70786526130b660405f20612f52600160405192612f3984613470565b604051612f4a81611a5581856137ba565b84520161384a565b908189820152519060405191612f678361348b565b8252888201908152612f7a368b89613546565b60408301908152612f8c36868b613546565b60608401908152606d8b7f6573206578747261446174612900000000000000000000000000000000000000608060405192612fc6846134b9565b8484528301927f5573657244656372797074526573706f6e7365566572696669636174696f6e2884527f6279746573207075626c69634b65792c627974657333325b5d20637448616e6460408201527f6c65732c6279746573207573657244656372797074656453686172652c627974606082015201522093518b815191012092516104b761305e8d60405192839182018095613fff565b51902091518b815191012090516040516130878d82816104f9818301968781519384920161335a565b51902091604051938c850195865260408501526060840152608083015260a082015260a08152611e1a816134d4565b966130ce83856130c6858a61402b565b9a8c8c614141565b885f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee702875260405f205f8052875260405f2061310b33826138c3565b54955f198701928784116132715761316e7f7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b2968a966131608e9a61317c9760806040519b8c9b8c528b015260808a0191613904565b918783036040890152613904565b918483036060860152613904565b0390a2835f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee7009283835260ff60405f2054161591826131f8575b50506131bf57005b825f525260405f20600160ff198254161790557fe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed255f80a2005b9091506040519163140f45ff60e11b83526004830152828260248173d582ec82a1758322907df80da8a754e12a5acb955afa918215610bb4575f92613242575b50101584806131b7565b9091508281813d831161326a575b61325a818361350a565b8101031261091c57519085613238565b503d613250565b634e487b7160e01b5f52601160045260245ffd5b636a457ca160e11b815260048101889052602490fd5b507f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee708548811612ef2565b9181601f8401121561091c578235916001600160401b03831161091c576020838186019501011161091c57565b608060031982011261091c57600435916001600160401b0360243581811161091c5783613321916004016132c5565b9390939260443583811161091c578261333c916004016132c5565b9390939260643591821161091c57613356916004016132c5565b9091565b5f5b83811061336b5750505f910152565b818101518382015260200161335c565b906020916133948151809281855285808601910161335a565b601f01601f1916010190565b9181601f8401121561091c578235916001600160401b03831161091c576020808501948460051b01011161091c57565b604060031982011261091c576001600160401b039160043583811161091c57826133fc916004016133a0565b9390939260243591821161091c57613356916004016132c5565b9181601f8401121561091c578235916001600160401b03831161091c576020808501946060850201011161091c57565b600435906001600160a01b038216820361091c57565b35906001600160a01b038216820361091c57565b604081019081106001600160401b0382111761082b57604052565b608081019081106001600160401b0382111761082b57604052565b6001600160401b03811161082b57604052565b60a081019081106001600160401b0382111761082b57604052565b60c081019081106001600160401b0382111761082b57604052565b60e081019081106001600160401b0382111761082b57604052565b90601f801991011681019081106001600160401b0382111761082b57604052565b6001600160401b03811161082b57601f01601f191660200190565b9291926135528261352b565b91613560604051938461350a565b82948184528183011161091c578281602093845f960137010152565b9181601f8401121561091c578235916001600160401b03831161091c576020808501948460061b01011161091c57565b3461091c57604036600319011261091c576001600160401b0360043581811161091c576135dd90369060040161357c565b60249291923591821161091c5760209261017761017e9336906004016132c5565b90600182811c9216801561362c575b602083101461361857565b634e487b7160e01b5f52602260045260245ffd5b91607f169161360d565b905f917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10290815491613667836135fe565b808352926020916001918281169081156136e5575060011461368b575b5050505050565b90939495505f527f42ad5d3e1f2e6e70edcf6d991b8a3023d3fca8047a131592f9edb9fd9b89d57d925f935b8585106136d257505050602092500101905f80808080613684565b80548585018401529382019381016136b7565b9350505050602093945060ff929192191683830152151560051b0101905f80808080613684565b905f917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1039081549161373d836135fe565b808352926020916001918281169081156136e55750600114613760575050505050565b90939495505f527f5f9ce34815f8e11431c7bb75a8e6886a91478f7ffc1dbb0a98dc240fddd76b75925f935b8585106137a757505050602092500101905f80808080613684565b805485850184015293820193810161378c565b80545f93926137c8826135fe565b918282526020936001916001811690815f1461382b57506001146137ed575050505050565b90939495505f92919252835f2092845f945b83861061381757505050500101905f80808080613684565b8054858701830152940193859082016137ff565b60ff19168685015250505090151560051b010191505f80808080613684565b90604051918281549182825260209260208301915f5260205f20935f905b8282106138805750505061387e9250038361350a565b565b855484526001958601958895509381019390910190613868565b80548210156138af575f5260205f2001905f90565b634e487b7160e01b5f52603260045260245ffd5b80546801000000000000000081101561082b576138e59160018201815561389a565b6001600160a01b039291928084549260031b9316831b921b1916179055565b908060209392818452848401375f828201840152601f01601f1916010190565b6040519061393182613470565b600a82527f44656372797074696f6e000000000000000000000000000000000000000000006020830152565b51906001600160a01b038216820361091c57565b9081602091031261091c5751801515810361091c5790565b8115613a24575f5b8281106139a057505050600190565b60408051632ddc9a6f60e01b8152600583901b84013560048201526020808260248173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315613a1b57505f926139fe575b5050156139f757600101613991565b5050505f90565b613a149250803d106108fe576108f0818361350a565b5f806139e8565b513d5f823e3d90fd5b50505f90565b91908110156138af576060020190565b908015613a24575f5b818110613a5257505050600190565b613a5d818385613a2a565b60408051632ddc9a6f60e01b815291356004830152906020808260248173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315613a1b57505f92613aaf575b5050156139f757600101613a43565b613ac59250803d106108fe576108f0818361350a565b5f80613aa0565b818110613ad7575050565b5f8155600101613acc565b9190601f8111613af157505050565b61387e925f5260205f20906020601f840160051c83019310613b1b575b601f0160051c0190613acc565b9091508190613b0e565b91908110156138af5760061b0190565b908015613a24575f5b818110613b4d57505050600190565b613b58818385613b25565b60408051632ddc9a6f60e01b815291356004830152906020808260248173c7d45661a345ec5ca0e8521cfef7e32fda0daa685afa928315613a1b57505f92613baa575b5050156139f757600101613b3e565b613bc09250803d106108fe576108f0818361350a565b5f80613b9b565b6001600160401b03811161082b5760051b60200190565b903590601e198136030182121561091c57018035906001600160401b03821161091c57602001918160051b3603831361091c57565b604090602319011261091c5760405190613c2c82613470565b60243582526044356020830152565b6064356001600160a01b038116810361091c5790565b6084356001600160a01b038116810361091c5790565b9291613c7282613bc7565b91613c80604051938461350a565b829481845260208094019160051b810192831161091c57905b828210613ca65750505050565b838091613cb28461345c565b815201910190613c99565b9190808252602080920192915f5b828110613cd9575050505090565b9091929382806001926001600160a01b03613cf38961345c565b16815201950193929101613ccb565b6040906001600160a01b03613d2295931681528160208201520191613cbd565b90565b602090818184031261091c5780516001600160401b039182821161091c57019083601f8301121561091c57815190613d5c82613bc7565b946040613d6b8151978861350a565b838752858701928660059560051b8701019583871161091c57878101945b878610613d9c5750505050505050505090565b855183811161091c578201608080601f19838903011261091c57855191613dc28361348b565b8b8101518352868101518c84015260609182820151888501528101519086821161091c57019087603f8301121561091c57818c8093015188613e0382613bc7565b94613e108251968761350a565b8286528501918d1b830101918a831161091c5791898f969492979593015b818110613e48575050849550820152815201950194613d89565b9193958091939597613e598461395d565b8152019101918e959391969492613e2e565b60209060206040818301928281528551809452019301915f5b828110613e92575050505090565b835185529381019392810192600101613e84565b5f1981146132715760010190565b9068010000000000000000811161082b57815491818155828210613ed757505050565b5f5260205f2091820191015b818110613eee575050565b5f8155600101613ee3565b6080820181518352602060a060608294838101518488015260408101516040880152015194608060608201528551809452019301915f5b828110613f3e575050505090565b83516001600160a01b031685529381019392810192600101613f30565b90808251908181526020809101926020808460051b8301019501935f915b848310613f895750505050505090565b9091929394958480613fa7600193601f198682030187528a51613ef9565b9801930193019194939290613f79565b949293613d2296946001600160a01b03613fdd613ff1959460808a5260808a0190613f5b565b931660208801528683036040880152613904565b926060818503910152613904565b80516020809201915f5b828110614017575050505090565b835185529381019392810192600101614009565b811561410457803560f81c918215614096576001831461405e5760405163084e730b60e21b815260048101849052602490fd5b909150602181106140775760211161091c576001013590565b604490604051906349aa453360e11b8252600482015260216024820152fd5b505060405163976f3eb960e01b8152905060208160048173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f916140d5575090565b90506020813d6020116140fc575b816140f06020938361350a565b8101031261091c575190565b3d91506140e3565b505060405163976f3eb960e01b815260208160048173d582ec82a1758322907df80da8a754e12a5acb955afa908115610bb4575f916140d5575090565b9493916105556141609461415793943691613546565b909391936149f8565b60408051632511f3f560e21b8152600481018690526001600160a01b03841660248201529092602092909173d582ec82a1758322907df80da8a754e12a5acb95908481604481855afa90811561438c575f9161436f575b501561434f57845163063fe83960e31b815260048101979097523360248801525f90879060449082905afa801561434557839495965f916142a7575b506001600160a01b0394859101511693821680940361428957805f527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70191828452855f20855f52845260ff865f20541661426257505f528152825f20915f52525f20600160ff19825416179055565b85516399ec48d960e01b815260048101929092526001600160a01b03166024820152604490fd5b8451630d86f52160e01b815260048101859052336024820152604490fd5b9350503d805f853e6142b9818561350a565b8301848482031261091c5783516001600160401b039485821161091c570160808183031261091c578651916142ed8361348b565b6142f68261395d565b835261430387830161395d565b878401528782015186811161091c578161431e918401614831565b88840152606082015195861161091c57869561433a9201614831565b60608201525f6141f3565b84513d5f823e3d90fd5b845163153e377b60e11b81526001600160a01b0384166004820152602490fd5b6143869150853d87116108fe576108f0818361350a565b5f6141b7565b86513d5f823e3d90fd5b6040515f6143a382613470565b600190600183526020368185013760218301825b6143c2575b50505090565b5f190190600a906f181899199a1a9b1b9c1cb0b131b232b360811b8282061a8353049182156143f3579190826143b7565b6143bc565b60ff7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300541661442357565b60405163d93c066560e01b8152600490fd5b9190820180921161327157565b602081018051156144d957805161016d908181116144bb575050815142811161124957508151905162015180908181029181830414901517156132715761448a904292614435565b106144925750565b60405162c0d20160e61b8152426004820152815160248201526020909101516044820152606490fd5b6044925060405191633295186360e01b835260048301526024820152fd5b60405163de2859c160e01b8152600490fd5b8051156138af5760200190565b80518210156138af5760209160051b010190565b905f5b82518110156139f7576001600160a01b038061452b83866144f8565b51169083161461453d5760010161450f565b505050600190565b9061454f82613bc7565b61455c604051918261350a565b828152809261456d601f1991613bc7565b0190602036910137565b92919080156146885761458981614545565b935f925f5b8381106145a8575050505061080090818111610bed575050565b6145b3818585613b25565b35602095866145c3848888613b25565b0135906001600160a01b038216820361091c576001600160401b038360101c1685358082036146625750506146019061ffff6109bb6109b6866148b4565b96840161461c826103286146158489613bde565b3691613c67565b1561463857505090600191614631828a6144f8565b520161458e565b906146466108279286613bde565b60405163a4c3039160e01b815293849390929060048501613d02565b604051634ac8748b60e11b81526004810186905260248101929092526044820152606490fd5b60405163a6a6cb2160e01b8152600490fd5b60019060018151111561474757908092916020806146b7836144eb565b510151906001955b6146cc575b505050509050565b82518610156147425781816146e188866144f8565b510151036146f35794830194836146bf565b8261470887614701836144eb565b51926144f8565b51610827604091614730835194859463cfae921f60e01b865260048601526044850190613ef9565b83810360031901602485015290613ef9565b6146c4565b5050565b7333e0c7a03d2b040b518580c365f4b3bde7cc4e6e90813b1561091c576001600160a01b0360245f9283604051958694859363988a2d2d60e01b85521660048401525af18015610bb45761479c5750565b61387e906134a6565b613d22906147b1614bb7565b6104b76148116147bf614c29565b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f602082019081529181019590955260608501919091524660808501523060a08501529291829060c0820190565b5190206042916040519161190160f01b8352600283015260228201522090565b81601f8201121561091c5780516148478161352b565b92614855604051948561350a565b8184526020828401011161091c57613d22916020808501910161335a565b60ff7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a005460401c16156148a257565b604051631afcd79f60e31b8152600490fd5b60081c60ff16605381116148e25760548110156148ce5790565b634e487b7160e01b5f52602160045260245ffd5b6024906040519063641950d760e01b82526004820152fd5b60548110156148ce578061490e5750600290565b6002810361491c5750600890565b6003810361492a5750601090565b600481036149385750602090565b600581036149465750604090565b600681036149545750608090565b60078103614962575060a090565b60088103614971575061010090565b6024906040519063be7830b160e01b82526004820152fd5b80516020809201915f5b8281106149a1575050505090565b83516001600160a01b031685529381019392810192600101614993565b81519190604183036149ee576149e79250602082015190606060408401519301515f1a90614ad2565b9192909190565b50505f9160029190565b60048110156148ce5780614a0a575050565b60018103614a245760405163f645eedf60e01b8152600490fd5b60028103614a455760405163fce698f760e01b815260048101839052602490fd5b600314614a4f5750565b602490604051906335e2f38360e21b82526004820152fd5b90613d2291614a74614bb7565b614811614a7f614c29565b604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6020820190815291810194909452606084019190915260808301939093523060a08301528160c081016104b7565b91907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411614b49579160209360809260ff5f9560405194855216868401526040830152606082015282805260015afa15610bb4575f516001600160a01b03811615614b3f57905f905f90565b505f906001905f90565b5050505f9160039190565b90614b7b5750805115614b6957602081519101fd5b60405163d6bda27560e01b8152600490fd5b81511580614bae575b614b8c575090565b604051639996b31560e01b81526001600160a01b039091166004820152602490fd5b50803b15614b84565b604051614bc781611a5581613636565b8051908115614bd7576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100548015614c045790565b507fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47090565b604051614c3981611a558161370c565b8051908115614c49576020012090565b50507fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d101548015614c04579056
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_\x90_5`\xE0\x1C\x90\x81c\x04o\x9E\xB3\x14a.\xCEWP\x80c\t\0\xCCi\x14a-\xCCW\x80c\r\x8En,\x14a,\xA5W\x80c\x12:\xBB(\x14a,\rW\x80c9\xF78\x10\x14a'\xB1W\x80c?K\xA8:\x14a&{W\x80c@\x14\xC4\xCD\x14a&`W\x80cA\x0B\xF0\xBA\x14a&\x07W\x80cO\x1E\xF2\x86\x14a#BW\x80cR\xD1\x90-\x14a\"\xC6W\x80cX\xF5\xB8\xAB\x14a\"xW\x80c\\\x97Z\xBB\x14a\"7W\x80co\x89\x13\xBC\x14a\x1C~W\x80cv\"~\xED\x14a\t$W\x80c\x84V\xCBY\x14a\x1B~W\x80c\x84\xB0\x19n\x14a\x1A\x07W\x80c\x9F\xADZ/\x14a\x13*W\x80c\xAD<\xB1\xCC\x14a\x12\xC9W\x80c\xB4\xDE,7\x14a\x0C-W\x80c\xD8\x99\x8FE\x14a\t)W\x80c\xE2-\x1B&\x14a\t$W\x80c\xF1\xB5z\xDB\x14a\x01\x8FWc\xFB\xB82Y\x14a\x01\x1BW_\x80\xFD[4a\x01\x88W``6`\x03\x19\x01\x12a\x01\x88Wa\x014a4FV[P`\x01`\x01`@\x1B\x03\x90`$5\x82\x81\x11a\x01\x8BWa\x01V\x906\x90`\x04\x01a5|V[\x92\x90\x91`D5\x91\x82\x11a\x01\x88W` a\x01~\x85\x85a\x01w6`\x04\x88\x01a2\xC5V[PPa;5V[`@Q\x90\x15\x15\x81R\xF3[\x80\xFD[P\x80\xFD[P4a\x01\x88W`\x03\x19a\x01\x006\x82\x01\x12a\x01\x8BW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\t Wa\x01\xC2\x906\x90`\x04\x01a5|V[\x90\x91`@6`#\x19\x01\x12a\t\x18W`\x01`\x01`@\x1B\x03`d5\x11a\t\x18W`@\x90`d56\x03\x01\x12a\t W`\x01`\x01`\xA0\x1B\x03`\x845\x16`\x845\x03a\t\x1CW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\t\x18Wa\x02!\x906\x90`\x04\x01a2\xC5V[\x90\x91`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\t\x14Wa\x02B\x906\x90`\x04\x01a2\xC5V[\x94\x90`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\t\x10Wa\x02c\x906\x90`\x04\x01a2\xC5V[\x93\x90\x92a\x02naC\xF8V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x04`d5\x81\x015\x90\x82\x01R` \x81`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\t\x05W\x8A\x91a\x08\xD6W[P\x15a\x08\xB9Wa\x02\xCC`$`d5\x01`d5`\x04\x01a;\xDEV[\x90P\x15a\x08\xA7W`\na\x02\xE9`$`d5\x01`d5`\x04\x01a;\xDEV[\x90P\x11a\x08sWa\x03\x01a\x02\xFC6a<\x13V[aDBV[a\x03-a\x03(a\x03\x1B`$`d5\x01`d5`\x04\x01a;\xDEV[\x91\x90`\x845\x926\x91a<gV[aE\x0CV[a\x08?Wa\x03A\x91`d5`\x04\x01\x91aEwV[\x95a\x03V`$`d5\x01`d5`\x04\x01a;\xDEV[`@Q\x91\x82`\xA0\x81\x01\x10`\x01`\x01`@\x1B\x03`\xA0\x85\x01\x11\x17a\x08+W\x82a\x03\x9Da\x05J\x92a\x05[\x94`\xA0a\x05d\x97\x01`@Ra\x03\x94\x8D\x8D6\x91a5FV[\x84R6\x91a<gV[` \x82\x01\x90\x81R`$5`@\x83\x01R`D5``\x83\x01Ra\x03\xBF6\x8A\x8Aa5FV[`\x80\x83\x01R`\x87`@Qa\x03\xD2\x81a4\xD4V[\x81\x81R\x7FraData)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xA0` \x83\x01\x92\x7FUserDecryptRequestVerification(b\x84R\x7Fytes publicKey,address[] contrac`@\x82\x01R\x7FtAddresses,uint256 startTimestam``\x82\x01R\x7Fp,uint256 durationDays,bytes ext`\x80\x82\x01R\x01R \x91a\x04\xB7a\x04\xC5\x82Q` \x81Q\x91\x01 \x93Q`@Q\x92\x83\x91` \x83\x01\x90aI\x89V[\x03`\x1F\x19\x81\x01\x83R\x82a5\nV[` \x81Q\x91\x01 \x90`@\x81\x01Q`\x80``\x83\x01Q\x92\x01Q`@Qa\x05\x06` \x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[\x81\x01\x03\x80\x84R\x01\x82a5\nV[Q\x90 \x92`@Q\x94` \x86\x01\x96\x87R`@\x86\x01R``\x85\x01R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xC0\x81Ra\x05;\x81a4\xEFV[Q\x90 `d5`\x04\x015aJgV[a\x05U6\x85\x87a5FV[\x90aI\xBEV[\x90\x92\x91\x92aI\xF8V[`\x01`\x01`\xA0\x1B\x03\x80`\x845\x16\x91\x16\x03a\x08\x03WPP`@Qc\xA1O\x89q`\xE0\x1B\x81R\x92\x86\x84\x80a\x05\x98\x89`\x04\x83\x01a>kV[\x03\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x93\x84\x15a\x07\xF8W\x87\x94a\x07\xD4W[Pa\x05\xC9\x84aF\x9AV[a\x05\xF3\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08Ta>\xA6V[\x95\x86\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U`@Q\x90a\x06$\x82a4pV[a\x06/6\x84\x89a5FV[\x82R` \x82\x01R\x86\x88R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07` R`@\x88 \x90\x80Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x07RWa\x06\x89\x82a\x06\x83\x86Ta5\xFEV[\x86a:\xE2V[` \x90`\x1F\x83\x11`\x01\x14a\x07qWa\x06\xB8\x92\x91\x8C\x91\x83a\x07fW[PP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x82U[` `\x01\x80\x93\x01\x91\x01Q\x90\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x07RW` \x90a\x06\xE4\x84\x84a>\xB4V[\x01\x90\x8AR` \x8A \x8A[\x83\x81\x10a\x07?W\x8B\x8B\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x8C\x8Ca\x079\x8D\x8D\x8Da\x07)3aGKV[`@Q\x95\x86\x95`\x845\x90\x87a?\xB7V[\x03\x90\xA2\x80\xF3[\x84\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x06\xEEV[cNH{q`\xE0\x1B\x8BR`A`\x04R`$\x8B\xFD[\x01Q\x90P_\x80a\x06\xA4V[\x84\x8CR` \x8C \x91\x90`\x1F\x19\x84\x16\x8D[\x81\x81\x10a\x07\xBCWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x07\xA4W[PPP\x81\x1B\x01\x82Ua\x06\xBBV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x07\x97V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07\x81V[a\x07\xF1\x91\x94P=\x80\x89\x83>a\x07\xE9\x81\x83a5\nV[\x81\x01\x90a=%V[\x92_a\x05\xBFV[`@Q=\x89\x82>=\x90\xFD[a\x08'`@Q\x92\x83\x92c*\x87='`\xE0\x1B\x84R` `\x04\x85\x01R`$\x84\x01\x91a9\x04V[\x03\x90\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[a\x08S`$`d5\x01`d5`\x04\x01a;\xDEV[`@Qc\xDCMx\xB1`\xE0\x1B\x81R\x91\x82\x91a\x08'\x91`\x845`\x04\x85\x01a=\x02V[`Da\x08\x89`$`d5\x01`d5`\x04\x01a;\xDEV[\x90P`@Q\x90c\xAF\x1F\x04\x95`\xE0\x1B\x82R`\n`\x04\x83\x01R`$\x82\x01R\xFD[`@QcW\xCF\xA2\x17`\xE0\x1B\x81R`\x04\x90\xFD[`$`@Qc\xB6g\x9C;`\xE0\x1B\x81R`d5`\x04\x015`\x04\x82\x01R\xFD[a\x08\xF8\x91P` =` \x11a\x08\xFEW[a\x08\xF0\x81\x83a5\nV[\x81\x01\x90a9qV[_a\x02\xB2V[P=a\x08\xE6V[`@Q=\x8C\x82>=\x90\xFD[\x87\x80\xFD[\x85\x80\xFD[\x83\x80\xFD[_\x80\xFD[\x82\x80\xFD[a5\xACV[P4a\t\x1CWa\t86a3\xD0V[\x91\x93\x90\x92a\tDaC\xF8V[\x84\x15a\x0C\x1BWa\tS\x85a;\xC7V[\x92a\ta`@Q\x94\x85a5\nV[\x85\x84R` \x80\x85\x01\x91\x87`\x05\x1B\x92\x83\x85\x01\x906\x82\x11a\t\x1CW\x83\x86\x91[\x83\x83\x10a\x0C\x0BWPPPP_\x96_\x97[\x87Q\x89\x10\x15a\t\xCAWa\t\xC2`\x01\x91a\xFF\xFFa\t\xBBa\t\xB6a\t\xB0\x8E\x8EaD\xF8V[QaH\xB4V[aH\xFAV[\x16\x90aD5V[\x98\x01\x97a\t\x8EV[\x89\x90a\x08\0\x90\x81\x81\x11a\x0B\xEDWPP`@Q\x94c\xA1O\x89q`\xE0\x1B\x86R\x84`\x04\x87\x01R\x81`$\x87\x01R\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\t\x1CW\x85`D\x81\x83_\x94\x8B\x84\x84\x017\x81\x01\x03\x01\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x94\x85\x15a\x0B\xB4W_\x95a\x0B\xD1W[Pa\nZ\x85aF\x9AV[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06\x95a\n\x86\x87Ta>\xA6V[\x80\x97U\x86_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x05\x85R`@_ \x90`\x01`\x01`@\x1B\x03\x83\x11a\x08+Wa\n\xCC\x83\x83a>\xB4V[\x90_R\x84_ _[\x83\x81\x10a\x0B\xBFWPPPPs3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNn\x80;\x15a\t\x1CW_\x80\x91`$`@Q\x80\x94\x81\x93c${\xAC\x9F`\xE2\x1B\x83R3`\x04\x84\x01RZ\xF1\x80\x15a\x0B\xB4Wa\x0BqW[Pa\x0Bd\x92\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x94\x92a\x079\x92`@Q\x95\x86\x95`@\x87R`@\x87\x01\x90a?[V[\x92\x85\x84\x03\x90\x86\x01Ra9\x04V[a\x079\x91\x96P\x92\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x94\x92a\x0B\xA7a\x0Bd\x95a4\xA6V[_\x97\x92P\x92\x94P\x92a\x0B$V[`@Q=_\x82>=\x90\xFD[\x825\x82\x82\x01U\x91\x86\x01\x91`\x01\x01a\n\xD4V[a\x0B\xE6\x91\x95P=\x80_\x83>a\x07\xE9\x81\x83a5\nV[\x93\x87a\nPV[`D\x92P`@Q\x91c\xE7\xF4\x89]`\xE0\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[\x825\x81R\x91\x81\x01\x91\x85\x91\x01a\t~V[`@Qc\x05\xBC\xEA\x87`\xE3\x1B\x81R`\x04\x90\xFD[4a\t\x1CWa\x01\x006`\x03\x19\x01\x12a\t\x1CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C^\x906\x90`\x04\x01a4\x16V[\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\t\x1CW`D5`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C\x91\x906\x90`\x04\x01a2\xC5V[`d5`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C\xB0\x906\x90`\x04\x01a3\xA0V[\x90`@6`\x83\x19\x01\x12a\t\x1CW`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x0C\xDC\x906\x90`\x04\x01a2\xC5V[\x92\x90\x96`\x01`\x01`@\x1B\x03`\xE45\x11a\t\x1CWa\x0C\xFE6`\xE45`\x04\x01a2\xC5V[\x95\x90\x96a\r\taC\xF8V[\x8A\x15a\x12\xB7W`\n\x84\x11a\x12\x97W`@Qa\r#\x81a4pV[`\x845\x81R`\xA45` \x82\x01R`\xA45\x15a\x12\x85W` \x81\x01Qc\x01\xE13\x80\x90\x81\x81\x11a\x12gWPP\x80QB\x81\x11a\x12IWP\x80Qa\rhB\x91` \x84\x01Q\x90aD5V[\x10a\x12\x1FWPa\rw3aGKV[a\r\x80\x8BaEEV[\x98`\x10\x9B`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x865`\x10\x1C\x16`\x04\x82\x01R` \x81`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a\x12\0W[P\x15a\x11\xDBW_\x9C_`\x80R[\x81`\x80Q\x10a\x11:WPa\x08\0\x80\x8E\x11a\x11\x1CWP`@Qc\xA1O\x89q`\xE0\x1B\x81R\x9B\x9CP\x99\x9A\x99_\x8B\x80a\x0E\x14\x8F`\x04\x83\x01a>kV[\x03\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x9A\x8B\x15a\x0B\xB4W_\x9Ba\x11\0W[Pa\x0EE\x8BaF\x9AV[a\x0Eo\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08Ta>\xA6V[\x9B\x8C\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U`@Qa\x0E\x9F\x81a4pV[a\x0E\xAA6\x87\x87a5FV[\x81R` \x81\x01\x91\x82R\x8D_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07` R`@_ \x90Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x08+Wa\x0F\x05\x82a\x0E\xFF\x85Ta5\xFEV[\x85a:\xE2V[` \x90`\x1F\x83\x11`\x01\x14a\x10\x99W\x91\x80a\x0F7\x92`\x01\x95\x94_\x92a\x07fWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x81U[\x01\x90Q\x90\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x08+W` \x90a\x0F\\\x84\x84a>\xB4V[\x01\x90_R` _ _[\x83\x81\x10a\x10\x85WPPPP` a\x0F\x89`@Q\x9C\x8Da\x01 \x90\x81\x81R\x01\x90a?[V[\x8C\x81\x03\x82\x8E\x01R\x82\x81R\x01\x94\x90_[\x81\x81\x10a\x108WPPP\x94a\x10$\x94a\x0F\xF8\x7F\x04G\x11\xC6L\xF1\x8A\xBFI`\"\x0E>\x1E\xCF\x01\x05\x9Cr\xB8\x15,\x84\x15\xB5\x8E\xA1\xFD\xD8J+K\x9C\x9D\x95\x8C\x9B\x99\x95a\x10\x06\x95\x8D`@`\x01`\x01`\xA0\x1B\x03a\x103\x9F\x9D\x16\x91\x01R\x8D``\x81\x85\x03\x91\x01Ra9\x04V[\x91\x8A\x83\x03`\x80\x8C\x01Ra<\xBDV[\x91`\x845`\xA0\x89\x01R`\xA45`\xC0\x89\x01R\x87\x83\x03`\xE0\x89\x01Ra9\x04V[\x91\x84\x83\x03a\x01\0\x86\x01Ra9\x04V[\x03\x90\xA2\0[\x90\x91\x95`\x01\x90\x875\x81R`\x01`\x01`\xA0\x1B\x03a\x10V` \x8A\x01a4\\V[\x16` \x82\x01R`\x01`\x01`\xA0\x1B\x03a\x10p`@\x8A\x01a4\\V[\x16`@\x82\x01R``\x90\x81\x01\x97\x01\x92\x91\x01a\x0F\x98V[`\x01\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x0FfV[\x90`\x1F\x19\x83\x16\x91\x84_R` _ \x92_[\x81\x81\x10a\x10\xE8WP\x91`\x01\x95\x94\x92\x91\x83\x87\x95\x93\x10a\x10\xD0W[PPP\x81\x1B\x01\x81Ua\x0F:V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U_\x80\x80a\x10\xC3V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x10\xAAV[a\x11\x15\x91\x9BP=\x80_\x83>a\x07\xE9\x81\x83a5\nV[\x99\x8Da\x0E;V[`D\x90\x8E`@Q\x91c\xE7\xF4\x89]`\xE0\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[`\x01`\x01`@\x1B\x03\x9Da\x11P`\x80Q\x84\x89a:*V[5\x9E\x8F\x83\x1C\x16`\x01`\x01`@\x1B\x03\x885`\x10\x1C\x16\x81\x03a\x11\x9CWPa\x11\x7F\x90\x8Fa\t\xBBa\t\xB6a\xFF\xFF\x92aH\xB4V[\x9Da\x11\x8D\x8D`\x80Q\x90aD\xF8V[R`\x01`\x80Q\x01`\x80Ra\r\xDCV[\x8Fa\x08'\x89\x91`\x01`\x01`@\x1B\x03\x93`@Q\x94\x85\x94cJ\xC8t\x8B`\xE1\x1B\x86R5`\x10\x1C\x16\x91`\x04\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`@Qc\xB6g\x9C;`\xE0\x1B\x81R\x855`\x10\x1C`\x01`\x01`@\x1B\x03\x16`\x04\x82\x01R`$\x90\xFD[a\x12\x19\x91P` =` \x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[\x8Ea\r\xCFV[`@Qc3\xC7\xE7\xE7`\xE1\x1B\x81RB`\x04\x82\x01R\x81Q`$\x82\x01R` \x90\x91\x01Q`D\x82\x01R`d\x90\xFD[`D\x90`@Q\x90c\xF2L\x08\x87`\xE0\x1B\x82RB`\x04\x83\x01R`$\x82\x01R\xFD[`D\x92P`@Q\x91cW)u\x89`\xE1\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[`@Qc\x12)\xE27`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\xAF\x1F\x04\x95`\xE0\x1B\x81R`\n`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[`@Qc$\x0E\x93\t`\xE0\x1B\x81R`\x04\x90\xFD[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CWa\x13&`@Qa\x12\xE8\x81a4pV[`\x05\x81R\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a3{V[\x03\x90\xF3[4a\t\x1CW`\x03\x19a\x01 6\x82\x01\x12a\t\x1CW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\\\x906\x90`\x04\x01a5|V[\x90\x91`@6`#\x19\x01\x12a\t\x1CW`@6`c\x19\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03`\xA45\x11a\t\x1CW`@\x90`\xA456\x03\x01\x12a\t\x1CW`\xC45`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\xB3\x906\x90`\x04\x01a2\xC5V[`\xE45`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\xD2\x906\x90`\x04\x01a2\xC5V[\x94\x90a\x01\x045`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CWa\x13\xF4\x906\x90`\x04\x01a2\xC5V[\x96\x90\x92a\x13\xFFaC\xF8V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x04`\xA45\x81\x015\x90\x82\x01R` \x81`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a\x19\xE8W[P\x15a\x19\xCBW`$`\xA45\x01\x96a\x14_\x88`\xA45`\x04\x01a;\xDEV[\x90P\x15a\x08\xA7W`\na\x14w\x89`\xA45`\x04\x01a;\xDEV[\x90P\x11a\x19\xBAWa\x14\x8Aa\x02\xFC6a<\x13V[a\x14\xB1a\x03(a\x14\x9F\x8A`\xA45`\x04\x01a;\xDEV[\x91\x90a\x14\xA9a<;V[\x926\x91a<gV[a\x19\x83Wa\x14\xD5\x91a\x14\xC9\x91`\xA45`\x04\x01\x91aEwV[\x96`\xA45`\x04\x01a;\xDEV[\x90a\x14\xDEa<;V[`@Q\x92\x83`\xC0\x81\x01\x10`\x01`\x01`@\x1B\x03`\xC0\x86\x01\x11\x17a\x08+W`\x01`\x01`\xA0\x1B\x03\x92a\x15\"\x91`\xC0\x86\x01`@Ra\x15\x196\x8B\x8Da5FV[\x86R6\x91a<gV[` \x84\x01R\x16`@\x82\x01R`$5``\x82\x01R`D5`\x80\x82\x01Ra\x15H6\x89\x86a5FV[`\xA0\x82\x01Ra\x15Ua<QV[`\xA9`@Qa\x15c\x81a4\xEFV[\x81\x81R\x7FxtraData)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xC0` \x83\x01\x92\x7FDelegatedUserDecryptRequestVerif\x84R\x7Fication(bytes publicKey,address[`@\x82\x01R\x7F] contractAddresses,address dele``\x82\x01R\x7FgatorAddress,uint256 startTimest`\x80\x82\x01R\x7Famp,uint256 durationDays,bytes e`\xA0\x82\x01R\x01R \x91\x80Q` \x81Q\x91\x01 \x90a\x04\xB7a\x16r` \x83\x01Q`@Q\x92\x83\x91` \x83\x01\x90aI\x89V[` \x81Q\x91\x01 `\x01`\x01`\xA0\x1B\x03`@\x83\x01Q\x16``\x83\x01Q\x91`\xA0`\x80\x85\x01Q\x94\x01Q`@Qa\x16\xB4` \x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[Q\x90 \x94`@Q\x97` \x89\x01R`@\x88\x01R``\x87\x01R`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01R`\xE0\x82R\x81a\x01\0\x81\x01\x10`\x01`\x01`@\x1B\x03a\x01\0\x84\x01\x11\x17a\x08+W`\x01`\x01`\xA0\x1B\x03\x80\x91a\x177a\x17,\x85a\x01\0a\x17@\x97\x01`@R` \x81Q\x91\x01 `\xA45`\x04\x015aJgV[a\x05U6\x88\x8Aa5FV[\x90\x95\x91\x95aI\xF8V[\x16\x91\x16\x03a\x08\x03WPP`@Qc\xA1O\x89q`\xE0\x1B\x81R\x91_\x83\x80a\x17h\x88`\x04\x83\x01a>kV[\x03\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a\x0B\xB4W_\x93a\x19gW[Pa\x17\x99\x83aF\x9AV[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x94a\x17\xC5\x86Ta>\xA6V[\x80\x96U`@Q\x90a\x17\xD5\x82a4pV[a\x17\xE06\x84\x88a5FV[\x82R` \x82\x01\x90\x81R\x86_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07` R`@_ \x91Q\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x08+Wa\x185\x82a\x06\x83\x86Ta5\xFEV[` \x90`\x1F\x83\x11`\x01\x14a\x19\x03Wa\x18c\x92\x91_\x91\x83a\x18\xF8WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x82U[`\x01\x80\x92\x01\x90Q\x90\x81Q\x91`\x01`\x01`@\x1B\x03\x83\x11a\x08+W` \x90a\x18\x8C\x84\x84a>\xB4V[\x01\x90_R` _ _[\x83\x81\x10a\x18\xE5W\x89\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x8A\x8Aa\x103\x8F\x8C\x8Ca\x18\xD03aGKV[a\x18\xD8a<QV[\x95`@Q\x96\x87\x96\x87a?\xB7V[\x84\x90` \x84Q\x94\x01\x93\x81\x84\x01U\x01a\x18\x96V[\x01Q\x90P\x8B\x80a\x06\xA4V[\x90`\x1F\x19\x83\x16\x91\x85_R` _ \x92_[\x81\x81\x10a\x19OWP\x90\x84`\x01\x95\x94\x93\x92\x10a\x197W[PPP\x81\x1B\x01\x82Ua\x18fV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x8A\x80\x80a\x19*V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x19\x14V[a\x19|\x91\x93P=\x80_\x83>a\x07\xE9\x81\x83a5\nV[\x91\x86a\x17\x8FV[a\x08'\x88a\x19\x9Ea\x19\x92a<;V[\x91`\xA45`\x04\x01a;\xDEV[`@Qc\xC3Dj\xC7`\xE0\x1B\x81R\x93\x84\x93\x90\x92\x90`\x04\x85\x01a=\x02V[`Da\x08\x89\x89`\xA45`\x04\x01a;\xDEV[`$`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\xA45`\x04\x015`\x04\x82\x01R\xFD[a\x1A\x01\x91P` =` \x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[\x89a\x14CV[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x15\x80a\x1BUW[\x15a\x1B\x10W`@Qa\x1A\\\x81a\x1AU\x81a66V[\x03\x82a5\nV[`@Qa\x1Al\x81a\x1AU\x81a7\x0CV[`@Q` \x80\x82\x01\x92\x82\x84\x10`\x01`\x01`@\x1B\x03\x85\x11\x17a\x08+W\x91` a\x1A\xC5\x85\x94a\x1A\xB7\x97\x96`@R_\x84R`@Q\x97\x88\x97`\x0F`\xF8\x1B\x89R`\xE0\x85\x8A\x01R`\xE0\x89\x01\x90a3{V[\x90\x87\x82\x03`@\x89\x01Ra3{V[\x91F``\x87\x01R0`\x80\x87\x01R_`\xA0\x87\x01R\x85\x83\x03`\xC0\x87\x01RQ\x91\x82\x81R\x01\x92\x91_[\x82\x81\x10a\x1A\xF9WPPPP\x03\x90\xF3[\x83Q\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x1A\xEAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[P\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x15a\x1A@V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90` \x81`$\x81\x85Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a\x1C_W[P\x15\x90\x81a\x1CTW[Pa\x1C<Wa\x1B\xE4aC\xF8V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0`\x01`\xFF\x19\x82T\x16\x17\x90U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X` `@Q3\x81R\xA1\0[`@Qc8\x89\x16\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[\x90P3\x14\x15\x81a\x1B\xD7V[a\x1Cx\x91P` =` \x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[\x82a\x1B\xCEV[4a\t\x1CWa\x1C\x8C6a2\xF2V[\x92\x93\x94`\x01`\xF8\x93\x92\x93\x1B\x87\x11\x80\x15\x90a\"\rW[a!\xF4W\x86_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x05` R`@_ `@Q``\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+Wa\x1E\"\x92a\x1C\xF9\x91`@Ra8JV[\x81Ra\x1D\x066\x89\x85a5FV[` \x82\x01Ra\x1D\x166\x87\x87a5FV[`@\x82\x01R`T`@Qa\x1D)\x81a4\x8BV[\x81\x81R\x7Fult,bytes extraData)\0\0\0\0\0\0\0\0\0\0\0\0``` \x83\x01\x92\x7FPublicDecryptVerification(bytes3\x84R\x7F2[] ctHandles,bytes decryptedRes`@\x82\x01R\x01R \x90a\x04\xB7a\x1D\xB9\x82Q`@Q\x92\x83\x91` \x83\x01\x90a?\xFFV[` \x81Q\x91\x01 \x90`@` \x82\x01Q` \x81Q\x91\x01 \x91\x01Q`@Qa\x1D\xEF` \x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[Q\x90 \x90`@Q\x92` \x84\x01\x94\x85R`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\x80\x81Ra\x1E\x1A\x81a4\xB9V[Q\x90 aG\xA5V[a\x1E,\x85\x85a@+V[\x92a\x1E:\x81\x88\x84\x8C\x88aAAV[\x88_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x04` R`@_ \x82_R` R`@_ \x96\x87Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x08+W\x80`\x01a\x1E\x93\x92\x01\x8AU\x89a8\x9AV[\x91\x90\x91a!\xE1W`\x01`\x01`@\x1B\x03\x83\x11a\x08+W\x82\x82a\x1E\xC0\x8D\x95a\x1E\xBA\x8E\x96Ta5\xFEV[\x83a:\xE2V[_`\x1F\x83\x11`\x01\x14a!IWa\x1Fq\x93\x83a\x1F\x94\x93a\x1F\x1A\x82\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x99\x97a\x1F\x7F\x96_\x91a!>W[P\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[\x87_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02` R`@_ \x89_R` Ra\x1F\\`@_ 3\x90a8\xC3V[`@Q\x95\x86\x95`\x80\x87R`\x80\x87\x01\x90\x8Ca9\x04V[\x91\x85\x83\x03` \x87\x01Ra9\x04V[3`@\x84\x01R\x82\x81\x03``\x84\x01R\x8A\x8Aa9\x04V[\x03\x90\xA2\x87_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x92\x83` R`\xFF`@_ T\x16\x15\x90\x81a \xBFW[Pa\x1F\xD7W\0[a +\x92\x88_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x03` R`@_ U`@Q\x95``\x87R``\x87\x01\x91a9\x04V[\x84\x81\x03` \x86\x01R\x83T\x80\x82R` \x82\x01\x91` \x82`\x05\x1B\x82\x01\x01\x95_R` _ \x92_\x91[\x83\x83\x10a \x93WPPPP\x84\x84\x03`@\x86\x01RP\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x93\x92\x83\x92a\x103\x92a9\x04V[\x90\x91\x92\x93\x96` `\x01a \xB0\x81\x93`\x1F\x19\x86\x82\x03\x01\x87R\x8Ba7\xBAV[\x99\x01\x93\x01\x93\x01\x91\x93\x92\x90a QV[\x90P\x86T`@Q\x91ca\xD5U-`\xE1\x1B\x83R`\x04\x83\x01R` \x82`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x91\x82\x15a\x0B\xB4W_\x92a!\nW[P\x10\x15\x89a\x1F\xD0V[\x90\x91P` \x81=` \x11a!6W[\x81a!&` \x93\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90\x8Aa!\x01V[=\x91Pa!\x19V[\x90P\x85\x015_a\x1F\x07V[\x81_R` _ \x90_[`\x1F\x19\x85\x16\x81\x10a!\xC3WP\x93\x83a\x1F\x94\x93a\x1F\x7F\x93a\x1Fq\x97\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x99\x97`\x1F\x19\x81\x16\x10a!\xAAW[PP`\x01\x82\x81\x1B\x01\x90Ua\x1F\x1DV[\x84\x015_\x19`\x03\x85\x90\x1B`\xF8\x16\x1C\x19\x16\x90U_\x80a!\x9BV[\x85\x82\x015\x83U\x8F\x97P\x8E\x96P`\x01\x90\x92\x01\x91` \x91\x82\x01\x91\x01a!SV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[`@QcjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x90\xFD[P\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06T\x87\x11a\x1C\xA1V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW` `\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16`@Q\x90\x15\x15\x81R\xF3[4a\t\x1CW` 6`\x03\x19\x01\x12a\t\x1CW`\x045_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x160\x03a#0W` `@Q\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81R\xF3[`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x90\xFD[`@6`\x03\x19\x01\x12a\t\x1CWa#Va4FV[`$\x90\x815`\x01`\x01`@\x1B\x03\x81\x11a\t\x1CW6`#\x82\x01\x12\x15a\t\x1CWa#\x87\x906\x90\x84\x81`\x04\x015\x91\x01a5FV[`\x01`\x01`\xA0\x1B\x03\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x800\x14\x90\x81\x15a%\xD9W[Pa#0W`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x91\x90\x82\x81`\x04\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x80\x15a\x0B\xB4W\x82\x91_\x91a%\xA1W[P\x163\x03a%\x8AW\x83\x16\x92`@QcR\xD1\x90-`\xE0\x1B\x81R\x82\x81`\x04\x81\x88Z\xFA_\x91\x81a%[W[Pa$CW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R\x86\x90\xFD[\x84\x90\x86\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x91\x82\x81\x03a%FWP\x83;\x15a%0WP\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82T\x16\x17\x90U`@Q\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;_\x80\xA2\x83Q\x15a%\x16WP_\x80\x84\x84a%\x0B\x96Q\x91\x01\x84Z\xF4\x90=\x15a%\rW=a$\xEF\x81a5+V[\x90a$\xFD`@Q\x92\x83a5\nV[\x81R_\x81\x94=\x92\x01>aKTV[\0[``\x92PaKTV[\x92PPP4a%!W\0[c\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x90\xFD[`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R\xFD[`@Q\x90c*\x87Ri`\xE2\x1B\x82R`\x04\x82\x01R\xFD[\x90\x91P\x83\x81\x81=\x83\x11a%\x83W[a%s\x81\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90\x87a$&V[P=a%iV[`@Qc\x0EV\xCF=`\xE0\x1B\x81R3`\x04\x82\x01R\x85\x90\xFD[\x80\x92P\x84\x80\x92P=\x83\x11a%\xD2W[a%\xBA\x81\x83a5\nV[\x81\x01\x03\x12a\t\x1CWa%\xCC\x82\x91a9]V[\x87a#\xFEV[P=a%\xB0V[\x90P\x81\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT\x16\x14\x15\x85a#\xBDV[4a\t\x1CW`@6`\x03\x19\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03`\x045\x81\x81\x11a\t\x1CWa&8\x906\x90`\x04\x01a4\x16V[`$\x92\x91\x925\x91\x82\x11a\t\x1CW` \x92a&Ya\x01~\x936\x90`\x04\x01a2\xC5V[PPa::V[4a\t\x1CW` a\x01~a&s6a3\xD0V[PP\x90a9\x89V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x0B\xB4W_\x90a'qW[`\x01`\x01`\xA0\x1B\x03\x91P\x163\x14\x15\x90\x81a'fW[Pa'NW\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x80T`\xFF\x81\x16\x15a'<W`\xFF\x19\x16\x90U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA` `@Q3\x81R\xA1\0[`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcp\xC8\xB3w`\xE1\x1B\x81R3`\x04\x82\x01R`$\x90\xFD[\x90P3\x14\x15\x81a&\xD9V[P` \x81=` \x11a'\xA9W[\x81a'\x8B` \x93\x83a5\nV[\x81\x01\x03\x12a\t\x1CWa'\xA4`\x01`\x01`\xA0\x1B\x03\x91a9]V[a&\xC4V[=\x91Pa'~V[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01\x91`\x01`\x01`@\x1B\x03\x91\x82\x81\x16_\x19\x81\x01a+\xFBW`\xFF\x82`@\x1C\x16\x90\x81\x15a+\xEFW[Pa+\xDDWh\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16h\x01\0\0\0\0\0\0\0\x05\x17\x81Ua(0a9$V[\x92`@Q\x92a(>\x84a4pV[\x81\x84R` \x93`1`\xF8\x1B\x85\x82\x01Ra(UaHsV[a(]aHsV[\x85Q\x82\x81\x11a\x08+W\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x90a(\x92\x82Ta5\xFEV[\x97`\x1F\x98\x89\x81\x11a+\x93W[P\x87\x90\x89\x83\x11`\x01\x14a+\x17Wa(\xCB\x92\x91_\x91\x83a+\x0CWPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[\x80Q\x91\x82\x11a\x08+W\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x92a)\x03\x84Ta5\xFEV[\x87\x81\x11a*\xB8W[P\x85\x96\x83\x11`\x01\x14a*\x1EWP\x90\x80\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x96a)Z\x93_\x92a*\x13WPP\x81`\x01\x1B\x91_\x19\x90`\x03\x1B\x1C\x19\x16\x17\x90V[\x90U[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0U_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01Ua)\xABaHsV[`\x01`\xF8\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06U`\x01`\xF9\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x90U`@Q`\x05\x81R\xA1\0[\x01Q\x90P\x87\x80a\x06\xA4V[\x91\x90`\x1F\x19\x82\x16\x96\x84_R\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x91_[\x89\x81\x10a*\xA3WP\x83\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x99\x10a*\x8BW[PPP\x81\x1B\x01\x90Ua)]V[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x86\x80\x80a*~V[\x81\x83\x01Q\x84U\x92\x85\x01\x92\x91\x88\x01\x91\x88\x01a*MV[a*\xFD\x90\x85_R\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x89\x80\x87\x01`\x05\x1C\x82\x01\x92\x8A\x88\x10a+\x03W[\x01`\x05\x1C\x01\x90a:\xCCV[\x87a)\x0BV[\x92P\x81\x92a*\xF2V[\x01Q\x90P\x8A\x80a\x06\xA4V[\x86\x92\x91`\x1F\x19\x83\x16\x91\x85_R\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x92_[\x8C\x82\x82\x10a+}WPP\x84\x11a+eW[PPP\x81\x1B\x01\x90Ua(\xCEV[\x01Q_\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x89\x80\x80a+XV[\x83\x85\x01Q\x86U\x8B\x97\x90\x95\x01\x94\x93\x84\x01\x93\x01a+GV[a+\xD7\x90\x84_R\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x8B\x80\x86\x01`\x05\x1C\x82\x01\x92\x8C\x87\x10a+\x03W\x01`\x05\x1C\x01\x90a:\xCCV[\x89a(\x9EV[`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x90\xFD[`\x05\x91P\x10\x15\x85a(\nV[`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x90\xFD[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CW\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\xFF\x81`@\x1C\x16\x80\x15a,\x91W[a+\xDDW`\x05\x90h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90U\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2` `@Q`\x05\x81R\xA1\0[P`\x05`\x01`\x01`@\x1B\x03\x82\x16\x10\x15a,NV[4a\t\x1CW_6`\x03\x19\x01\x12a\t\x1CWa,\xBDa9$V[a,\xC5aC\x96V[\x90`\x04`@Q\x91a,\xD5\x83a4pV[`\x01\x92`\x01\x81R` \x93\x84\x82\x01\x93\x856\x867`!\x83\x01\x82[a-\x96W[PPP\x90a-\x82\x92`$\x92a-\x05aC\x96V[\x91`@Q\x97\x84a-\x1E\x8A\x96Q\x80\x92\x8B\x80\x8A\x01\x91\x01a3ZV[\x85\x01a\x10;`\xF1\x1B\x89\x82\x01Ra-=\x82Q\x80\x93\x8B`\"\x85\x01\x91\x01a3ZV[\x01a-Z`\x17`\xF9\x1B\x93\x84`\"\x84\x01RQ\x80\x93`#\x84\x01\x90a3ZV[\x01\x90`#\x82\x01Ra-s\x82Q\x80\x93\x88\x87\x85\x01\x91\x01a3ZV[\x01\x03`\x04\x81\x01\x85R\x01\x83a5\nV[a\x13&`@Q\x92\x82\x84\x93\x84R\x83\x01\x90a3{V[_\x19\x01\x90`\n\x90o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x82\x82\x06\x1A\x83S\x04\x91\x82\x15a-\xC7W\x91\x90\x82a,\xEDV[a,\xF2V[4a\t\x1CW` \x80`\x03\x196\x01\x12a\t\x1CW`\x045_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x03\x81R`@_ T\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02\x82R`@_ \x90_R\x81R`@_ `@Q\x90\x81\x83\x82T\x91\x82\x81R\x01\x90\x81\x92_R\x84_ \x90_[\x86\x82\x82\x10a.\xB1W\x86\x86a.i\x82\x88\x03\x83a5\nV[`@Q\x92\x83\x92\x81\x84\x01\x90\x82\x85RQ\x80\x91R`@\x84\x01\x92\x91_[\x82\x81\x10a.\x91WPPPP\x03\x90\xF3[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x86\x95P\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a.\x82V[\x83T`\x01`\x01`\xA0\x1B\x03\x16\x85R\x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a.SV[4a\t\x1CWa.\xDC6a2\xF2V[\x94\x92\x91\x90\x93\x95\x96`\x01`\xF9\x1B\x88\x11\x80\x15\x90a2\x9BW[a2\x85WP\x86_R` \x94\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x07\x86Ra0\xB6`@_ a/R`\x01`@Q\x92a/9\x84a4pV[`@Qa/J\x81a\x1AU\x81\x85a7\xBAV[\x84R\x01a8JV[\x90\x81\x89\x82\x01RQ\x90`@Q\x91a/g\x83a4\x8BV[\x82R\x88\x82\x01\x90\x81Ra/z6\x8B\x89a5FV[`@\x83\x01\x90\x81Ra/\x8C6\x86\x8Ba5FV[``\x84\x01\x90\x81R`m\x8B\x7Fes extraData)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80`@Q\x92a/\xC6\x84a4\xB9V[\x84\x84R\x83\x01\x92\x7FUserDecryptResponseVerification(\x84R\x7Fbytes publicKey,bytes32[] ctHand`@\x82\x01R\x7Fles,bytes userDecryptedShare,byt``\x82\x01R\x01R \x93Q\x8B\x81Q\x91\x01 \x92Qa\x04\xB7a0^\x8D`@Q\x92\x83\x91\x82\x01\x80\x95a?\xFFV[Q\x90 \x91Q\x8B\x81Q\x91\x01 \x90Q`@Qa0\x87\x8D\x82\x81a\x04\xF9\x81\x83\x01\x96\x87\x81Q\x93\x84\x92\x01a3ZV[Q\x90 \x91`@Q\x93\x8C\x85\x01\x95\x86R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\x1E\x1A\x81a4\xD4V[\x96a0\xCE\x83\x85a0\xC6\x85\x8Aa@+V[\x9A\x8C\x8CaAAV[\x88_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02\x87R`@_ _\x80R\x87R`@_ a1\x0B3\x82a8\xC3V[T\x95_\x19\x87\x01\x92\x87\x84\x11a2qWa1n\x7F\x7F\xCD\xFBS\x81\x91\x7FUJq}\nTp\xA3?ZI\xBAdE\xF0^\xC4<t\xC0\xBC,\xC6\x08\xB2\x96\x8A\x96a1`\x8E\x9Aa1|\x97`\x80`@Q\x9B\x8C\x9B\x8CR\x8B\x01R`\x80\x8A\x01\x91a9\x04V[\x91\x87\x83\x03`@\x89\x01Ra9\x04V[\x91\x84\x83\x03``\x86\x01Ra9\x04V[\x03\x90\xA2\x83_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0\x92\x83\x83R`\xFF`@_ T\x16\x15\x91\x82a1\xF8W[PPa1\xBFW\0[\x82_RR`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U\x7F\xE8\x97R\xBE\x0E\xCD\xB6\x8B*n\xB5\xEF\x1A\x89\x109\xE0\xE9*\xE3\xC8\xA6\"t\xC5\x88\x1EH\xEE\xA1\xED%_\x80\xA2\0[\x90\x91P`@Q\x91c\x14\x0FE\xFF`\xE1\x1B\x83R`\x04\x83\x01R\x82\x82`$\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x91\x82\x15a\x0B\xB4W_\x92a2BW[P\x10\x15\x84\x80a1\xB7V[\x90\x91P\x82\x81\x81=\x83\x11a2jW[a2Z\x81\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90\x85a28V[P=a2PV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[cjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x90\xFD[P\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08T\x88\x11a.\xF2V[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x83\x81\x86\x01\x95\x01\x01\x11a\t\x1CWV[`\x80`\x03\x19\x82\x01\x12a\t\x1CW`\x045\x91`\x01`\x01`@\x1B\x03`$5\x81\x81\x11a\t\x1CW\x83a3!\x91`\x04\x01a2\xC5V[\x93\x90\x93\x92`D5\x83\x81\x11a\t\x1CW\x82a3<\x91`\x04\x01a2\xC5V[\x93\x90\x93\x92`d5\x91\x82\x11a\t\x1CWa3V\x91`\x04\x01a2\xC5V[\x90\x91V[_[\x83\x81\x10a3kWPP_\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a3\\V[\x90` \x91a3\x94\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a3ZV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\t\x1CWV[`@`\x03\x19\x82\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03\x91`\x045\x83\x81\x11a\t\x1CW\x82a3\xFC\x91`\x04\x01a3\xA0V[\x93\x90\x93\x92`$5\x91\x82\x11a\t\x1CWa3V\x91`\x04\x01a2\xC5V[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x80\x85\x01\x94``\x85\x02\x01\x01\x11a\t\x1CWV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CWV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CWV[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x08+W`@RV[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\xE0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x08+W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x08+W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a5R\x82a5+V[\x91a5``@Q\x93\x84a5\nV[\x82\x94\x81\x84R\x81\x83\x01\x11a\t\x1CW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x91\x81`\x1F\x84\x01\x12\x15a\t\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\t\x1CW` \x80\x85\x01\x94\x84`\x06\x1B\x01\x01\x11a\t\x1CWV[4a\t\x1CW`@6`\x03\x19\x01\x12a\t\x1CW`\x01`\x01`@\x1B\x03`\x045\x81\x81\x11a\t\x1CWa5\xDD\x906\x90`\x04\x01a5|V[`$\x92\x91\x925\x91\x82\x11a\t\x1CW` \x92a\x01wa\x01~\x936\x90`\x04\x01a2\xC5V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a6,W[` \x83\x10\x14a6\x18WV[cNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x91`\x7F\x16\x91a6\rV[\x90_\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x90\x81T\x91a6g\x83a5\xFEV[\x80\x83R\x92` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a6\xE5WP`\x01\x14a6\x8BW[PPPPPV[\x90\x93\x94\x95P_R\x7FB\xAD]>\x1F.np\xED\xCFm\x99\x1B\x8A0#\xD3\xFC\xA8\x04z\x13\x15\x92\xF9\xED\xB9\xFD\x9B\x89\xD5}\x92_\x93[\x85\x85\x10a6\xD2WPPP` \x92P\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x80T\x85\x85\x01\x84\x01R\x93\x82\x01\x93\x81\x01a6\xB7V[\x93PPPP` \x93\x94P`\xFF\x92\x91\x92\x19\x16\x83\x83\x01R\x15\x15`\x05\x1B\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x90_\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x90\x81T\x91a7=\x83a5\xFEV[\x80\x83R\x92` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a6\xE5WP`\x01\x14a7`WPPPPPV[\x90\x93\x94\x95P_R\x7F_\x9C\xE3H\x15\xF8\xE1\x141\xC7\xBBu\xA8\xE6\x88j\x91G\x8F\x7F\xFC\x1D\xBB\n\x98\xDC$\x0F\xDD\xD7ku\x92_\x93[\x85\x85\x10a7\xA7WPPP` \x92P\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x80T\x85\x85\x01\x84\x01R\x93\x82\x01\x93\x81\x01a7\x8CV[\x80T_\x93\x92a7\xC8\x82a5\xFEV[\x91\x82\x82R` \x93`\x01\x91`\x01\x81\x16\x90\x81_\x14a8+WP`\x01\x14a7\xEDWPPPPPV[\x90\x93\x94\x95P_\x92\x91\x92R\x83_ \x92\x84_\x94[\x83\x86\x10a8\x17WPPPP\x01\x01\x90_\x80\x80\x80\x80a6\x84V[\x80T\x85\x87\x01\x83\x01R\x94\x01\x93\x85\x90\x82\x01a7\xFFV[`\xFF\x19\x16\x86\x85\x01RPPP\x90\x15\x15`\x05\x1B\x01\x01\x91P_\x80\x80\x80\x80a6\x84V[\x90`@Q\x91\x82\x81T\x91\x82\x82R` \x92` \x83\x01\x91_R` _ \x93_\x90[\x82\x82\x10a8\x80WPPPa8~\x92P\x03\x83a5\nV[V[\x85T\x84R`\x01\x95\x86\x01\x95\x88\x95P\x93\x81\x01\x93\x90\x91\x01\x90a8hV[\x80T\x82\x10\x15a8\xAFW_R` _ \x01\x90_\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x08+Wa8\xE5\x91`\x01\x82\x01\x81Ua8\x9AV[`\x01`\x01`\xA0\x1B\x03\x92\x91\x92\x80\x84T\x92`\x03\x1B\x93\x16\x83\x1B\x92\x1B\x19\x16\x17\x90UV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q\x90a91\x82a4pV[`\n\x82R\x7FDecryption\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x83\x01RV[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CWV[\x90\x81` \x91\x03\x12a\t\x1CWQ\x80\x15\x15\x81\x03a\t\x1CW\x90V[\x81\x15a:$W_[\x82\x81\x10a9\xA0WPPP`\x01\x90V[`@\x80Qc-\xDC\x9Ao`\xE0\x1B\x81R`\x05\x83\x90\x1B\x84\x015`\x04\x82\x01R` \x80\x82`$\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a:\x1BWP_\x92a9\xFEW[PP\x15a9\xF7W`\x01\x01a9\x91V[PPP_\x90V[a:\x14\x92P\x80=\x10a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_\x80a9\xE8V[Q=_\x82>=\x90\xFD[PP_\x90V[\x91\x90\x81\x10\x15a8\xAFW``\x02\x01\x90V[\x90\x80\x15a:$W_[\x81\x81\x10a:RWPPP`\x01\x90V[a:]\x81\x83\x85a:*V[`@\x80Qc-\xDC\x9Ao`\xE0\x1B\x81R\x915`\x04\x83\x01R\x90` \x80\x82`$\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a:\x1BWP_\x92a:\xAFW[PP\x15a9\xF7W`\x01\x01a:CV[a:\xC5\x92P\x80=\x10a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_\x80a:\xA0V[\x81\x81\x10a:\xD7WPPV[_\x81U`\x01\x01a:\xCCV[\x91\x90`\x1F\x81\x11a:\xF1WPPPV[a8~\x92_R` _ \x90` `\x1F\x84\x01`\x05\x1C\x83\x01\x93\x10a;\x1BW[`\x1F\x01`\x05\x1C\x01\x90a:\xCCV[\x90\x91P\x81\x90a;\x0EV[\x91\x90\x81\x10\x15a8\xAFW`\x06\x1B\x01\x90V[\x90\x80\x15a:$W_[\x81\x81\x10a;MWPPP`\x01\x90V[a;X\x81\x83\x85a;%V[`@\x80Qc-\xDC\x9Ao`\xE0\x1B\x81R\x915`\x04\x83\x01R\x90` \x80\x82`$\x81s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhZ\xFA\x92\x83\x15a:\x1BWP_\x92a;\xAAW[PP\x15a9\xF7W`\x01\x01a;>V[a;\xC0\x92P\x80=\x10a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_\x80a;\x9BV[`\x01`\x01`@\x1B\x03\x81\x11a\x08+W`\x05\x1B` \x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\t\x1CW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\t\x1CW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\t\x1CWV[`@\x90`#\x19\x01\x12a\t\x1CW`@Q\x90a<,\x82a4pV[`$5\x82R`D5` \x83\x01RV[`d5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\t\x1CW\x90V[`\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\t\x1CW\x90V[\x92\x91a<r\x82a;\xC7V[\x91a<\x80`@Q\x93\x84a5\nV[\x82\x94\x81\x84R` \x80\x94\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\t\x1CW\x90[\x82\x82\x10a<\xA6WPPPPV[\x83\x80\x91a<\xB2\x84a4\\V[\x81R\x01\x91\x01\x90a<\x99V[\x91\x90\x80\x82R` \x80\x92\x01\x92\x91_[\x82\x81\x10a<\xD9WPPPP\x90V[\x90\x91\x92\x93\x82\x80`\x01\x92`\x01`\x01`\xA0\x1B\x03a<\xF3\x89a4\\V[\x16\x81R\x01\x95\x01\x93\x92\x91\x01a<\xCBV[`@\x90`\x01`\x01`\xA0\x1B\x03a=\"\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a<\xBDV[\x90V[` \x90\x81\x81\x84\x03\x12a\t\x1CW\x80Q`\x01`\x01`@\x1B\x03\x91\x82\x82\x11a\t\x1CW\x01\x90\x83`\x1F\x83\x01\x12\x15a\t\x1CW\x81Q\x90a=\\\x82a;\xC7V[\x94`@a=k\x81Q\x97\x88a5\nV[\x83\x87R\x85\x87\x01\x92\x86`\x05\x95`\x05\x1B\x87\x01\x01\x95\x83\x87\x11a\t\x1CW\x87\x81\x01\x94[\x87\x86\x10a=\x9CWPPPPPPPPP\x90V[\x85Q\x83\x81\x11a\t\x1CW\x82\x01`\x80\x80`\x1F\x19\x83\x89\x03\x01\x12a\t\x1CW\x85Q\x91a=\xC2\x83a4\x8BV[\x8B\x81\x01Q\x83R\x86\x81\x01Q\x8C\x84\x01R``\x91\x82\x82\x01Q\x88\x85\x01R\x81\x01Q\x90\x86\x82\x11a\t\x1CW\x01\x90\x87`?\x83\x01\x12\x15a\t\x1CW\x81\x8C\x80\x93\x01Q\x88a>\x03\x82a;\xC7V[\x94a>\x10\x82Q\x96\x87a5\nV[\x82\x86R\x85\x01\x91\x8D\x1B\x83\x01\x01\x91\x8A\x83\x11a\t\x1CW\x91\x89\x8F\x96\x94\x92\x97\x95\x93\x01[\x81\x81\x10a>HWPP\x84\x95P\x82\x01R\x81R\x01\x95\x01\x94a=\x89V[\x91\x93\x95\x80\x91\x93\x95\x97a>Y\x84a9]V[\x81R\x01\x91\x01\x91\x8E\x95\x93\x91\x96\x94\x92a>.V[` \x90` `@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a>\x92WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a>\x84V[_\x19\x81\x14a2qW`\x01\x01\x90V[\x90h\x01\0\0\0\0\0\0\0\0\x81\x11a\x08+W\x81T\x91\x81\x81U\x82\x82\x10a>\xD7WPPPV[_R` _ \x91\x82\x01\x91\x01[\x81\x81\x10a>\xEEWPPV[_\x81U`\x01\x01a>\xE3V[`\x80\x82\x01\x81Q\x83R` `\xA0``\x82\x94\x83\x81\x01Q\x84\x88\x01R`@\x81\x01Q`@\x88\x01R\x01Q\x94`\x80``\x82\x01R\x85Q\x80\x94R\x01\x93\x01\x91_[\x82\x81\x10a?>WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a?0V[\x90\x80\x82Q\x90\x81\x81R` \x80\x91\x01\x92` \x80\x84`\x05\x1B\x83\x01\x01\x95\x01\x93_\x91[\x84\x83\x10a?\x89WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a?\xA7`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x8AQa>\xF9V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a?yV[\x94\x92\x93a=\"\x96\x94`\x01`\x01`\xA0\x1B\x03a?\xDDa?\xF1\x95\x94`\x80\x8AR`\x80\x8A\x01\x90a?[V[\x93\x16` \x88\x01R\x86\x83\x03`@\x88\x01Ra9\x04V[\x92``\x81\x85\x03\x91\x01Ra9\x04V[\x80Q` \x80\x92\x01\x91_[\x82\x81\x10a@\x17WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a@\tV[\x81\x15aA\x04W\x805`\xF8\x1C\x91\x82\x15a@\x96W`\x01\x83\x14a@^W`@Qc\x08Ns\x0B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x90\xFD[\x90\x91P`!\x81\x10a@wW`!\x11a\t\x1CW`\x01\x015\x90V[`D\x90`@Q\x90cI\xAAE3`\xE1\x1B\x82R`\x04\x82\x01R`!`$\x82\x01R\xFD[PP`@Qc\x97o>\xB9`\xE0\x1B\x81R\x90P` \x81`\x04\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a@\xD5WP\x90V[\x90P` \x81=` \x11a@\xFCW[\x81a@\xF0` \x93\x83a5\nV[\x81\x01\x03\x12a\t\x1CWQ\x90V[=\x91Pa@\xE3V[PP`@Qc\x97o>\xB9`\xE0\x1B\x81R` \x81`\x04\x81s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95Z\xFA\x90\x81\x15a\x0B\xB4W_\x91a@\xD5WP\x90V[\x94\x93\x91a\x05UaA`\x94aAW\x93\x946\x91a5FV[\x90\x93\x91\x93aI\xF8V[`@\x80Qc%\x11\xF3\xF5`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R\x90\x92` \x92\x90\x91s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90\x84\x81`D\x81\x85Z\xFA\x90\x81\x15aC\x8CW_\x91aCoW[P\x15aCOW\x84Qc\x06?\xE89`\xE3\x1B\x81R`\x04\x81\x01\x97\x90\x97R3`$\x88\x01R_\x90\x87\x90`D\x90\x82\x90Z\xFA\x80\x15aCEW\x83\x94\x95\x96_\x91aB\xA7W[P`\x01`\x01`\xA0\x1B\x03\x94\x85\x91\x01Q\x16\x93\x82\x16\x80\x94\x03aB\x89W\x80_R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x01\x91\x82\x84R\x85_ \x85_R\x84R`\xFF\x86_ T\x16aBbWP_R\x81R\x82_ \x91_RR_ `\x01`\xFF\x19\x82T\x16\x17\x90UV[\x85Qc\x99\xECH\xD9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16`$\x82\x01R`D\x90\xFD[\x84Qc\r\x86\xF5!`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R3`$\x82\x01R`D\x90\xFD[\x93PP=\x80_\x85>aB\xB9\x81\x85a5\nV[\x83\x01\x84\x84\x82\x03\x12a\t\x1CW\x83Q`\x01`\x01`@\x1B\x03\x94\x85\x82\x11a\t\x1CW\x01`\x80\x81\x83\x03\x12a\t\x1CW\x86Q\x91aB\xED\x83a4\x8BV[aB\xF6\x82a9]V[\x83RaC\x03\x87\x83\x01a9]V[\x87\x84\x01R\x87\x82\x01Q\x86\x81\x11a\t\x1CW\x81aC\x1E\x91\x84\x01aH1V[\x88\x84\x01R``\x82\x01Q\x95\x86\x11a\t\x1CW\x86\x95aC:\x92\x01aH1V[``\x82\x01R_aA\xF3V[\x84Q=_\x82>=\x90\xFD[\x84Qc\x15>7{`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x90\xFD[aC\x86\x91P\x85=\x87\x11a\x08\xFEWa\x08\xF0\x81\x83a5\nV[_aA\xB7V[\x86Q=_\x82>=\x90\xFD[`@Q_aC\xA3\x82a4pV[`\x01\x90`\x01\x83R` 6\x81\x85\x017`!\x83\x01\x82[aC\xC2W[PPP\x90V[_\x19\x01\x90`\n\x90o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x82\x82\x06\x1A\x83S\x04\x91\x82\x15aC\xF3W\x91\x90\x82aC\xB7V[aC\xBCV[`\xFF\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T\x16aD#WV[`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x90\xFD[\x91\x90\x82\x01\x80\x92\x11a2qWV[` \x81\x01\x80Q\x15aD\xD9W\x80Qa\x01m\x90\x81\x81\x11aD\xBBWPP\x81QB\x81\x11a\x12IWP\x81Q\x90Qb\x01Q\x80\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a2qWaD\x8A\x90B\x92aD5V[\x10aD\x92WPV[`@Qb\xC0\xD2\x01`\xE6\x1B\x81RB`\x04\x82\x01R\x81Q`$\x82\x01R` \x90\x91\x01Q`D\x82\x01R`d\x90\xFD[`D\x92P`@Q\x91c2\x95\x18c`\xE0\x1B\x83R`\x04\x83\x01R`$\x82\x01R\xFD[`@Qc\xDE(Y\xC1`\xE0\x1B\x81R`\x04\x90\xFD[\x80Q\x15a8\xAFW` \x01\x90V[\x80Q\x82\x10\x15a8\xAFW` \x91`\x05\x1B\x01\x01\x90V[\x90_[\x82Q\x81\x10\x15a9\xF7W`\x01`\x01`\xA0\x1B\x03\x80aE+\x83\x86aD\xF8V[Q\x16\x90\x83\x16\x14aE=W`\x01\x01aE\x0FV[PPP`\x01\x90V[\x90aEO\x82a;\xC7V[aE\\`@Q\x91\x82a5\nV[\x82\x81R\x80\x92aEm`\x1F\x19\x91a;\xC7V[\x01\x90` 6\x91\x017V[\x92\x91\x90\x80\x15aF\x88WaE\x89\x81aEEV[\x93_\x92_[\x83\x81\x10aE\xA8WPPPPa\x08\0\x90\x81\x81\x11a\x0B\xEDWPPV[aE\xB3\x81\x85\x85a;%V[5` \x95\x86aE\xC3\x84\x88\x88a;%V[\x015\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\t\x1CW`\x01`\x01`@\x1B\x03\x83`\x10\x1C\x16\x855\x80\x82\x03aFbWPPaF\x01\x90a\xFF\xFFa\t\xBBa\t\xB6\x86aH\xB4V[\x96\x84\x01aF\x1C\x82a\x03(aF\x15\x84\x89a;\xDEV[6\x91a<gV[\x15aF8WPP\x90`\x01\x91aF1\x82\x8AaD\xF8V[R\x01aE\x8EV[\x90aFFa\x08'\x92\x86a;\xDEV[`@Qc\xA4\xC3\x03\x91`\xE0\x1B\x81R\x93\x84\x93\x90\x92\x90`\x04\x85\x01a=\x02V[`@QcJ\xC8t\x8B`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x92\x90\x92R`D\x82\x01R`d\x90\xFD[`@Qc\xA6\xA6\xCB!`\xE0\x1B\x81R`\x04\x90\xFD[`\x01\x90`\x01\x81Q\x11\x15aGGW\x90\x80\x92\x91` \x80aF\xB7\x83aD\xEBV[Q\x01Q\x90`\x01\x95[aF\xCCW[PPPP\x90PV[\x82Q\x86\x10\x15aGBW\x81\x81aF\xE1\x88\x86aD\xF8V[Q\x01Q\x03aF\xF3W\x94\x83\x01\x94\x83aF\xBFV[\x82aG\x08\x87aG\x01\x83aD\xEBV[Q\x92aD\xF8V[Qa\x08'`@\x91aG0\x83Q\x94\x85\x94c\xCF\xAE\x92\x1F`\xE0\x1B\x86R`\x04\x86\x01R`D\x85\x01\x90a>\xF9V[\x83\x81\x03`\x03\x19\x01`$\x85\x01R\x90a>\xF9V[aF\xC4V[PPV[s3\xE0\xC7\xA0=+\x04\x0BQ\x85\x80\xC3e\xF4\xB3\xBD\xE7\xCCNn\x90\x81;\x15a\t\x1CW`\x01`\x01`\xA0\x1B\x03`$_\x92\x83`@Q\x95\x86\x94\x85\x93c\x98\x8A--`\xE0\x1B\x85R\x16`\x04\x84\x01RZ\xF1\x80\x15a\x0B\xB4WaG\x9CWPV[a8~\x90a4\xA6V[a=\"\x90aG\xB1aK\xB7V[a\x04\xB7aH\x11aG\xBFaL)V[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01\x90\x81R\x91\x81\x01\x95\x90\x95R``\x85\x01\x91\x90\x91RF`\x80\x85\x01R0`\xA0\x85\x01R\x92\x91\x82\x90`\xC0\x82\x01\x90V[Q\x90 `B\x91`@Q\x91a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R \x90V[\x81`\x1F\x82\x01\x12\x15a\t\x1CW\x80QaHG\x81a5+V[\x92aHU`@Q\x94\x85a5\nV[\x81\x84R` \x82\x84\x01\x01\x11a\t\x1CWa=\"\x91` \x80\x85\x01\x91\x01a3ZV[`\xFF\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`@\x1C\x16\x15aH\xA2WV[`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x90\xFD[`\x08\x1C`\xFF\x16`S\x81\x11aH\xE2W`T\x81\x10\x15aH\xCEW\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`$\x90`@Q\x90cd\x19P\xD7`\xE0\x1B\x82R`\x04\x82\x01R\xFD[`T\x81\x10\x15aH\xCEW\x80aI\x0EWP`\x02\x90V[`\x02\x81\x03aI\x1CWP`\x08\x90V[`\x03\x81\x03aI*WP`\x10\x90V[`\x04\x81\x03aI8WP` \x90V[`\x05\x81\x03aIFWP`@\x90V[`\x06\x81\x03aITWP`\x80\x90V[`\x07\x81\x03aIbWP`\xA0\x90V[`\x08\x81\x03aIqWPa\x01\0\x90V[`$\x90`@Q\x90c\xBEx0\xB1`\xE0\x1B\x82R`\x04\x82\x01R\xFD[\x80Q` \x80\x92\x01\x91_[\x82\x81\x10aI\xA1WPPPP\x90V[\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01aI\x93V[\x81Q\x91\x90`A\x83\x03aI\xEEWaI\xE7\x92P` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90aJ\xD2V[\x91\x92\x90\x91\x90V[PP_\x91`\x02\x91\x90V[`\x04\x81\x10\x15aH\xCEW\x80aJ\nWPPV[`\x01\x81\x03aJ$W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x90\xFD[`\x02\x81\x03aJEW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`\x03\x14aJOWPV[`$\x90`@Q\x90c5\xE2\xF3\x83`\xE2\x1B\x82R`\x04\x82\x01R\xFD[\x90a=\"\x91aJtaK\xB7V[aH\x11aJ\x7FaL)V[`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F` \x82\x01\x90\x81R\x91\x81\x01\x94\x90\x94R``\x84\x01\x91\x90\x91R`\x80\x83\x01\x93\x90\x93R0`\xA0\x83\x01R\x81`\xC0\x81\x01a\x04\xB7V[\x91\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11aKIW\x91` \x93`\x80\x92`\xFF_\x95`@Q\x94\x85R\x16\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x0B\xB4W_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15aK?W\x90_\x90_\x90V[P_\x90`\x01\x90_\x90V[PPP_\x91`\x03\x91\x90V[\x90aK{WP\x80Q\x15aKiW` \x81Q\x91\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x90\xFD[\x81Q\x15\x80aK\xAEW[aK\x8CWP\x90V[`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[P\x80;\x15aK\x84V[`@QaK\xC7\x81a\x1AU\x81a66V[\x80Q\x90\x81\x15aK\xD7W` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0T\x80\x15aL\x04W\x90V[P\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x90V[`@QaL9\x81a\x1AU\x81a7\x0CV[\x80Q\x90\x81\x15aLIW` \x01 \x90V[PP\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x01T\x80\x15aL\x04W\x90V",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
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
        impl From<u8> for FheType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<FheType> for u8 {
            fn from(value: FheType) -> Self {
                value.into_underlying()
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
    #[derive(serde::Serialize, serde::Deserialize)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct HandleEntry { bytes32 handle; address contractAddress; address ownerAddress; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HandleEntry {
        #[allow(missing_docs)]
        pub handle: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub contractAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ownerAddress: alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<HandleEntry> for UnderlyingRustTuple<'_> {
            fn from(value: HandleEntry) -> Self {
                (value.handle, value.contractAddress, value.ownerAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HandleEntry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    handle: tuple.0,
                    contractAddress: tuple.1,
                    ownerAddress: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for HandleEntry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for HandleEntry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.handle),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.contractAddress,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.ownerAddress,
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
        impl alloy_sol_types::SolType for HandleEntry {
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
        impl alloy_sol_types::SolStruct for HandleEntry {
            const NAME: &'static str = "HandleEntry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "HandleEntry(bytes32 handle,address contractAddress,address ownerAddress)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.handle)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.contractAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.ownerAddress,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for HandleEntry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.handle,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.contractAddress,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.ownerAddress,
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
                    &rust.handle,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.contractAddress,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.ownerAddress,
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
    /**Custom error with signature `ContractAddressesMaxLengthExceeded(uint256,uint256)` and selector `0xaf1f0495`.
```solidity
error ContractAddressesMaxLengthExceeded(uint256 maxLength, uint256 actualLength);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ContractAddressesMaxLengthExceeded {
        #[allow(missing_docs)]
        pub maxLength: alloy::sol_types::private::primitives::aliases::U256,
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
            const SIGNATURE: &'static str = "ContractAddressesMaxLengthExceeded(uint256,uint256)";
            const SELECTOR: [u8; 4] = [175u8, 31u8, 4u8, 149u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.maxLength),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.actualLength),
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
    /**Custom error with signature `CoprocessorSignerDoesNotMatchTxSender(address,address)` and selector `0xe134bf62`.
```solidity
error CoprocessorSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CoprocessorSignerDoesNotMatchTxSender {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<CoprocessorSignerDoesNotMatchTxSender>
        for UnderlyingRustTuple<'_> {
            fn from(value: CoprocessorSignerDoesNotMatchTxSender) -> Self {
                (value.signerAddress, value.txSenderAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CoprocessorSignerDoesNotMatchTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signerAddress: tuple.0,
                    txSenderAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CoprocessorSignerDoesNotMatchTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CoprocessorSignerDoesNotMatchTxSender(address,address)";
            const SELECTOR: [u8; 4] = [225u8, 52u8, 191u8, 98u8];
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
    /**Custom error with signature `CtHandleChainIdDiffersFromContractChainId(bytes32,uint256,uint256)` and selector `0x9590e916`.
```solidity
error CtHandleChainIdDiffersFromContractChainId(bytes32 ctHandle, uint256 chainId, uint256 contractChainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CtHandleChainIdDiffersFromContractChainId {
        #[allow(missing_docs)]
        pub ctHandle: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub contractChainId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<CtHandleChainIdDiffersFromContractChainId>
        for UnderlyingRustTuple<'_> {
            fn from(value: CtHandleChainIdDiffersFromContractChainId) -> Self {
                (value.ctHandle, value.chainId, value.contractChainId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CtHandleChainIdDiffersFromContractChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    ctHandle: tuple.0,
                    chainId: tuple.1,
                    contractChainId: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CtHandleChainIdDiffersFromContractChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CtHandleChainIdDiffersFromContractChainId(bytes32,uint256,uint256)";
            const SELECTOR: [u8; 4] = [149u8, 144u8, 233u8, 22u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandle),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.contractChainId),
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
    /**Custom error with signature `DecryptionNotRequested(uint256)` and selector `0xd48af942`.
```solidity
error DecryptionNotRequested(uint256 decryptionId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DecryptionNotRequested {
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
        impl ::core::convert::From<DecryptionNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: DecryptionNotRequested) -> Self {
                (value.decryptionId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DecryptionNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { decryptionId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DecryptionNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DecryptionNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [212u8, 138u8, 249u8, 66u8];
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
    /**Custom error with signature `EmptyContractAddresses()` and selector `0x57cfa217`.
```solidity
error EmptyContractAddresses();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyContractAddresses;
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
        impl ::core::convert::From<EmptyContractAddresses> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyContractAddresses) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyContractAddresses {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyContractAddresses {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyContractAddresses()";
            const SELECTOR: [u8; 4] = [87u8, 207u8, 162u8, 23u8];
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
    /**Custom error with signature `EmptyCtHandleContractPairs()` and selector `0xa6a6cb21`.
```solidity
error EmptyCtHandleContractPairs();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCtHandleContractPairs;
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
                Self
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
    /**Custom error with signature `EmptyCtHandles()` and selector `0x2de75438`.
```solidity
error EmptyCtHandles();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyCtHandles;
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
                Self
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
    /**Custom error with signature `EmptyHandles()` and selector `0x240e9309`.
```solidity
error EmptyHandles();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyHandles;
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
        impl ::core::convert::From<EmptyHandles> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyHandles) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyHandles {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyHandles {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyHandles()";
            const SELECTOR: [u8; 4] = [36u8, 14u8, 147u8, 9u8];
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
    /**Custom error with signature `EnforcedPause()` and selector `0xd93c0665`.
```solidity
error EnforcedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EnforcedPause;
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
        impl ::core::convert::From<EnforcedPause> for UnderlyingRustTuple<'_> {
            fn from(value: EnforcedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EnforcedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EnforcedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EnforcedPause()";
            const SELECTOR: [u8; 4] = [217u8, 60u8, 6u8, 101u8];
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
    /**Custom error with signature `ExpectedPause()` and selector `0x8dfc202b`.
```solidity
error ExpectedPause();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpectedPause;
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
        impl ::core::convert::From<ExpectedPause> for UnderlyingRustTuple<'_> {
            fn from(value: ExpectedPause) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpectedPause {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpectedPause {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpectedPause()";
            const SELECTOR: [u8; 4] = [141u8, 252u8, 32u8, 43u8];
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
    /**Custom error with signature `InvalidExtraDataLength(uint256,uint256)` and selector `0x93548a66`.
```solidity
error InvalidExtraDataLength(uint256 length, uint256 minimumLength);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidExtraDataLength {
        #[allow(missing_docs)]
        pub length: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub minimumLength: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<InvalidExtraDataLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidExtraDataLength) -> Self {
                (value.length, value.minimumLength)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidExtraDataLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    length: tuple.0,
                    minimumLength: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidExtraDataLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidExtraDataLength(uint256,uint256)";
            const SELECTOR: [u8; 4] = [147u8, 84u8, 138u8, 102u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumLength),
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
    /**Custom error with signature `InvalidNullDurationDays()` and selector `0xde2859c1`.
```solidity
error InvalidNullDurationDays();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullDurationDays;
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
                Self
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
    /**Custom error with signature `InvalidNullDurationSeconds()` and selector `0x48a788dc`.
```solidity
error InvalidNullDurationSeconds();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullDurationSeconds;
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
        impl ::core::convert::From<InvalidNullDurationSeconds>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullDurationSeconds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidNullDurationSeconds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullDurationSeconds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullDurationSeconds()";
            const SELECTOR: [u8; 4] = [72u8, 167u8, 136u8, 220u8];
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
    /**Custom error with signature `KmsSignerDoesNotMatchTxSender(address,address)` and selector `0x0d86f521`.
```solidity
error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsSignerDoesNotMatchTxSender {
        #[allow(missing_docs)]
        pub signerAddress: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsSignerDoesNotMatchTxSender>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsSignerDoesNotMatchTxSender) -> Self {
                (value.signerAddress, value.txSenderAddress)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsSignerDoesNotMatchTxSender {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signerAddress: tuple.0,
                    txSenderAddress: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsSignerDoesNotMatchTxSender {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsSignerDoesNotMatchTxSender(address,address)";
            const SELECTOR: [u8; 4] = [13u8, 134u8, 245u8, 33u8];
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
    /**Custom error with signature `MaxDurationSecondsExceeded(uint256,uint256)` and selector `0xae52eb12`.
```solidity
error MaxDurationSecondsExceeded(uint256 maxValue, uint256 actualValue);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MaxDurationSecondsExceeded {
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
        impl ::core::convert::From<MaxDurationSecondsExceeded>
        for UnderlyingRustTuple<'_> {
            fn from(value: MaxDurationSecondsExceeded) -> Self {
                (value.maxValue, value.actualValue)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MaxDurationSecondsExceeded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    maxValue: tuple.0,
                    actualValue: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MaxDurationSecondsExceeded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MaxDurationSecondsExceeded(uint256,uint256)";
            const SELECTOR: [u8; 4] = [174u8, 82u8, 235u8, 18u8];
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
    /**Custom error with signature `NotOwnerOrGatewayConfig(address)` and selector `0xe19166ee`.
```solidity
error NotOwnerOrGatewayConfig(address notOwnerOrGatewayConfig);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotOwnerOrGatewayConfig {
        #[allow(missing_docs)]
        pub notOwnerOrGatewayConfig: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotOwnerOrGatewayConfig> for UnderlyingRustTuple<'_> {
            fn from(value: NotOwnerOrGatewayConfig) -> Self {
                (value.notOwnerOrGatewayConfig,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotOwnerOrGatewayConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    notOwnerOrGatewayConfig: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotOwnerOrGatewayConfig {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotOwnerOrGatewayConfig(address)";
            const SELECTOR: [u8; 4] = [225u8, 145u8, 102u8, 238u8];
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
                        &self.notOwnerOrGatewayConfig,
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
    /**Custom error with signature `NotPauserOrGatewayConfig(address)` and selector `0x388916bb`.
```solidity
error NotPauserOrGatewayConfig(address notPauserOrGatewayConfig);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotPauserOrGatewayConfig {
        #[allow(missing_docs)]
        pub notPauserOrGatewayConfig: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<NotPauserOrGatewayConfig>
        for UnderlyingRustTuple<'_> {
            fn from(value: NotPauserOrGatewayConfig) -> Self {
                (value.notPauserOrGatewayConfig,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NotPauserOrGatewayConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    notPauserOrGatewayConfig: tuple.0,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotPauserOrGatewayConfig {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotPauserOrGatewayConfig(address)";
            const SELECTOR: [u8; 4] = [56u8, 137u8, 22u8, 187u8];
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
                        &self.notPauserOrGatewayConfig,
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
    /**Custom error with signature `UnsupportedExtraDataVersion(uint8)` and selector `0x2139cc2c`.
```solidity
error UnsupportedExtraDataVersion(uint8 version);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UnsupportedExtraDataVersion {
        #[allow(missing_docs)]
        pub version: u8,
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
        impl ::core::convert::From<UnsupportedExtraDataVersion>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnsupportedExtraDataVersion) -> Self {
                (value.version,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnsupportedExtraDataVersion {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { version: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnsupportedExtraDataVersion {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnsupportedExtraDataVersion(uint8)";
            const SELECTOR: [u8; 4] = [33u8, 57u8, 204u8, 44u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
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
    /**Custom error with signature `UserDecryptionRequestExpiredSeconds(uint256,(uint256,uint256))` and selector `0x678fcfce`.
```solidity
error UserDecryptionRequestExpiredSeconds(uint256 currentTimestamp, IDecryption.RequestValiditySeconds requestValidity);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserDecryptionRequestExpiredSeconds {
        #[allow(missing_docs)]
        pub currentTimestamp: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
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
            IDecryption::RequestValiditySeconds,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            <IDecryption::RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<UserDecryptionRequestExpiredSeconds>
        for UnderlyingRustTuple<'_> {
            fn from(value: UserDecryptionRequestExpiredSeconds) -> Self {
                (value.currentTimestamp, value.requestValidity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UserDecryptionRequestExpiredSeconds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentTimestamp: tuple.0,
                    requestValidity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UserDecryptionRequestExpiredSeconds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UserDecryptionRequestExpiredSeconds(uint256,(uint256,uint256))";
            const SELECTOR: [u8; 4] = [103u8, 143u8, 207u8, 206u8];
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
                    <IDecryption::RequestValiditySeconds as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
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
    /**Event with signature `Paused(address)` and selector `0x62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a258`.
```solidity
event Paused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Paused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
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
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],bytes)` and selector `0x22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff`.
```solidity
event PublicDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, bytes extraData);
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
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8, 219u8, 72u8, 10u8, 57u8, 189u8, 114u8, 85u8, 100u8, 56u8, 170u8,
                219u8, 74u8, 50u8, 163u8, 210u8, 166u8, 99u8, 139u8, 135u8, 192u8, 59u8,
                190u8, 197u8, 254u8, 246u8, 153u8, 126u8, 16u8, 149u8, 135u8, 255u8,
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
                    extraData: data.1,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionResponse(uint256,bytes,bytes[],bytes)` and selector `0xd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f`.
```solidity
event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures, bytes extraData);
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
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionResponse(uint256,bytes,bytes[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                215u8, 229u8, 138u8, 54u8, 122u8, 10u8, 108u8, 41u8, 142u8, 118u8, 173u8,
                93u8, 36u8, 0u8, 4u8, 227u8, 39u8, 170u8, 20u8, 35u8, 203u8, 228u8,
                189u8, 127u8, 248u8, 93u8, 76u8, 113u8, 94u8, 248u8, 209u8, 95u8,
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
                    extraData: data.2,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionResponseCall(uint256,bytes,bytes,address,bytes)` and selector `0x4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e5`.
```solidity
event PublicDecryptionResponseCall(uint256 indexed decryptionId, bytes decryptedResult, bytes signature, address kmsTxSender, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionResponseCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decryptedResult: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for PublicDecryptionResponseCall {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
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
            const SIGNATURE: &'static str = "PublicDecryptionResponseCall(uint256,bytes,bytes,address,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                77u8, 123u8, 29u8, 186u8, 73u8, 233u8, 232u8, 70u8, 33u8, 94u8, 22u8,
                33u8, 245u8, 115u8, 124u8, 129u8, 216u8, 97u8, 76u8, 79u8, 38u8, 132u8,
                148u8, 216u8, 183u8, 135u8, 99u8, 44u8, 78u8, 89u8, 240u8, 229u8,
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
                    signature: data.1,
                    kmsTxSender: data.2,
                    extraData: data.3,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionResponseCall {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionResponseCall> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionResponseCall,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address)` and selector `0x5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa`.
```solidity
event Unpaused(address account);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Unpaused(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { account: data.0 }
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.account,
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
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes,bytes)` and selector `0xf9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b`.
```solidity
event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionRequest_0 {
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
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for UserDecryptionRequest_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<SnsCiphertextMaterial>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                249u8, 1u8, 27u8, 214u8, 186u8, 13u8, 166u8, 4u8, 156u8, 82u8, 13u8,
                112u8, 254u8, 89u8, 113u8, 241u8, 126u8, 215u8, 171u8, 121u8, 84u8,
                134u8, 5u8, 37u8, 68u8, 181u8, 16u8, 25u8, 137u8, 108u8, 89u8, 107u8,
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
                    extraData: data.3,
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData for UserDecryptionRequest_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionRequest_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserDecryptionRequest_0,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],(bytes32,address,address)[],address,bytes,address[],(uint256,uint256),bytes,bytes)` and selector `0x044711c64cf18abf4960220e3e1ecf01059c72b8152c8415b58ea1fdd84a2b4b`.
```solidity
event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, HandleEntry[] handles, address userAddress, bytes publicKey, address[] allowedContracts, IDecryption.RequestValiditySeconds requestValidity, bytes signature, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionRequest_1 {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub snsCtMaterials: alloy::sol_types::private::Vec<
            <SnsCiphertextMaterial as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub handles: alloy::sol_types::private::Vec<
            <HandleEntry as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub allowedContracts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for UserDecryptionRequest_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<SnsCiphertextMaterial>,
                alloy::sol_types::sol_data::Array<HandleEntry>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                IDecryption::RequestValiditySeconds,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],(bytes32,address,address)[],address,bytes,address[],(uint256,uint256),bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                4u8, 71u8, 17u8, 198u8, 76u8, 241u8, 138u8, 191u8, 73u8, 96u8, 34u8,
                14u8, 62u8, 30u8, 207u8, 1u8, 5u8, 156u8, 114u8, 184u8, 21u8, 44u8,
                132u8, 21u8, 181u8, 142u8, 161u8, 253u8, 216u8, 74u8, 43u8, 75u8,
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
                    handles: data.1,
                    userAddress: data.2,
                    publicKey: data.3,
                    allowedContracts: data.4,
                    requestValidity: data.5,
                    signature: data.6,
                    extraData: data.7,
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
                    <alloy::sol_types::sol_data::Array<
                        HandleEntry,
                    > as alloy_sol_types::SolType>::tokenize(&self.handles),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.allowedContracts),
                    <IDecryption::RequestValiditySeconds as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData for UserDecryptionRequest_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionRequest_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserDecryptionRequest_1,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionResponse(uint256,uint256,bytes,bytes,bytes)` and selector `0x7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b2`.
```solidity
event UserDecryptionResponse(uint256 indexed decryptionId, uint256 indexShare, bytes userDecryptedShare, bytes signature, bytes extraData);
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
        pub indexShare: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub userDecryptedShare: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionResponse(uint256,uint256,bytes,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8, 205u8, 251u8, 83u8, 129u8, 145u8, 127u8, 85u8, 74u8, 113u8, 125u8,
                10u8, 84u8, 112u8, 163u8, 63u8, 90u8, 73u8, 186u8, 100u8, 69u8, 240u8,
                94u8, 196u8, 60u8, 116u8, 192u8, 188u8, 44u8, 198u8, 8u8, 178u8,
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
                    indexShare: data.0,
                    userDecryptedShare: data.1,
                    signature: data.2,
                    extraData: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.indexShare),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.userDecryptedShare,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `UserDecryptionResponseThresholdReached(uint256)` and selector `0xe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed25`.
```solidity
event UserDecryptionResponseThresholdReached(uint256 indexed decryptionId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionResponseThresholdReached {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for UserDecryptionResponseThresholdReached {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionResponseThresholdReached(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8, 151u8, 82u8, 190u8, 14u8, 205u8, 182u8, 139u8, 42u8, 110u8, 181u8,
                239u8, 26u8, 137u8, 16u8, 57u8, 224u8, 233u8, 42u8, 227u8, 200u8, 166u8,
                34u8, 116u8, 197u8, 136u8, 30u8, 72u8, 238u8, 161u8, 237u8, 37u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { decryptionId: topics.1 }
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
        impl alloy_sol_types::private::IntoLogData
        for UserDecryptionResponseThresholdReached {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionResponseThresholdReached>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserDecryptionResponseThresholdReached,
            ) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),(uint256,address[]),bytes,bytes,bytes)` and selector `0x9fad5a2f`.
```solidity
function delegatedUserDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, IDecryption.DelegationAccounts memory delegationAccounts, IDecryption.ContractsInfo memory contractsInfo, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
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
        pub delegationAccounts: <IDecryption::DelegationAccounts as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),(uint256,address[]),bytes,bytes,bytes)`](delegatedUserDecryptionRequestCall) function.
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
                IDecryption::DelegationAccounts,
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
                <IDecryption::DelegationAccounts as alloy::sol_types::SolType>::RustType,
                <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
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
                        value.contractsInfo,
                        value.publicKey,
                        value.signature,
                        value.extraData,
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
                        contractsInfo: tuple.3,
                        publicKey: tuple.4,
                        signature: tuple.5,
                        extraData: tuple.6,
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
        impl delegatedUserDecryptionRequestReturn {
            fn _tokenize(
                &self,
            ) -> <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegatedUserDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                IDecryption::DelegationAccounts,
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Bytes,
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
            const SIGNATURE: &'static str = "delegatedUserDecryptionRequest((bytes32,address)[],(uint256,uint256),(address,address),(uint256,address[]),bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [159u8, 173u8, 90u8, 47u8];
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
                    <IDecryption::DelegationAccounts as alloy_sol_types::SolType>::tokenize(
                        &self.delegationAccounts,
                    ),
                    <IDecryption::ContractsInfo as alloy_sol_types::SolType>::tokenize(
                        &self.contractsInfo,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                delegatedUserDecryptionRequestReturn::_tokenize(ret)
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
    /**Function with signature `getDecryptionConsensusTxSenders(uint256)` and selector `0x0900cc69`.
```solidity
function getDecryptionConsensusTxSenders(uint256 decryptionId) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDecryptionConsensusTxSendersCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getDecryptionConsensusTxSenders(uint256)`](getDecryptionConsensusTxSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDecryptionConsensusTxSendersReturn {
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
            impl ::core::convert::From<getDecryptionConsensusTxSendersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDecryptionConsensusTxSendersCall) -> Self {
                    (value.decryptionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDecryptionConsensusTxSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { decryptionId: tuple.0 }
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
            impl ::core::convert::From<getDecryptionConsensusTxSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDecryptionConsensusTxSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDecryptionConsensusTxSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDecryptionConsensusTxSendersCall {
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
            const SIGNATURE: &'static str = "getDecryptionConsensusTxSenders(uint256)";
            const SELECTOR: [u8; 4] = [9u8, 0u8, 204u8, 105u8];
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
                        let r: getDecryptionConsensusTxSendersReturn = r.into();
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
                        let r: getDecryptionConsensusTxSendersReturn = r.into();
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
    /**Function with signature `isDecryptionDone(uint256)` and selector `0x58f5b8ab`.
```solidity
function isDecryptionDone(uint256 decryptionId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDecryptionDoneCall {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isDecryptionDone(uint256)`](isDecryptionDoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDecryptionDoneReturn {
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
            impl ::core::convert::From<isDecryptionDoneCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDecryptionDoneCall) -> Self {
                    (value.decryptionId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDecryptionDoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { decryptionId: tuple.0 }
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
            impl ::core::convert::From<isDecryptionDoneReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDecryptionDoneReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDecryptionDoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDecryptionDoneCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDecryptionDone(uint256)";
            const SELECTOR: [u8; 4] = [88u8, 245u8, 184u8, 171u8];
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
                        let r: isDecryptionDoneReturn = r.into();
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
                        let r: isDecryptionDoneReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isDelegatedUserDecryptionReady((bytes32,address)[],bytes)` and selector `0x76227eed`.
```solidity
function isDelegatedUserDecryptionReady(CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedUserDecryptionReadyCall {
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isDelegatedUserDecryptionReady((bytes32,address)[],bytes)`](isDelegatedUserDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isDelegatedUserDecryptionReadyReturn {
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
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isDelegatedUserDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedUserDecryptionReadyCall) -> Self {
                    (value.ctHandleContractPairs, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDelegatedUserDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandleContractPairs: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isDelegatedUserDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isDelegatedUserDecryptionReadyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isDelegatedUserDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isDelegatedUserDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isDelegatedUserDecryptionReady((bytes32,address)[],bytes)";
            const SELECTOR: [u8; 4] = [118u8, 34u8, 126u8, 237u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
                        let r: isDelegatedUserDecryptionReadyReturn = r.into();
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
                        let r: isDelegatedUserDecryptionReadyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isPublicDecryptionReady(bytes32[],bytes)` and selector `0x4014c4cd`.
```solidity
function isPublicDecryptionReady(bytes32[] memory ctHandles, bytes memory) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPublicDecryptionReadyCall {
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isPublicDecryptionReady(bytes32[],bytes)`](isPublicDecryptionReadyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isPublicDecryptionReadyReturn {
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
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<isPublicDecryptionReadyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isPublicDecryptionReadyCall) -> Self {
                    (value.ctHandles, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isPublicDecryptionReadyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandles: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isPublicDecryptionReadyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isPublicDecryptionReadyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isPublicDecryptionReadyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isPublicDecryptionReadyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isPublicDecryptionReady(bytes32[],bytes)";
            const SELECTOR: [u8; 4] = [64u8, 20u8, 196u8, 205u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
                        let r: isPublicDecryptionReadyReturn = r.into();
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
                        let r: isPublicDecryptionReadyReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isUserDecryptionReady((bytes32,address,address)[],bytes)` and selector `0x410bf0ba`.
```solidity
function isUserDecryptionReady(HandleEntry[] memory handles, bytes memory) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReady_0Call {
        #[allow(missing_docs)]
        pub handles: alloy::sol_types::private::Vec<
            <HandleEntry as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isUserDecryptionReady((bytes32,address,address)[],bytes)`](isUserDecryptionReady_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReady_0Return {
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
                alloy::sol_types::sol_data::Array<HandleEntry>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <HandleEntry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isUserDecryptionReady_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReady_0Call) -> Self {
                    (value.handles, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReady_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        handles: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isUserDecryptionReady_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReady_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReady_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isUserDecryptionReady_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<HandleEntry>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isUserDecryptionReady((bytes32,address,address)[],bytes)";
            const SELECTOR: [u8; 4] = [65u8, 11u8, 240u8, 186u8];
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
                        HandleEntry,
                    > as alloy_sol_types::SolType>::tokenize(&self.handles),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
                        let r: isUserDecryptionReady_0Return = r.into();
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
                        let r: isUserDecryptionReady_0Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isUserDecryptionReady((bytes32,address)[],bytes)` and selector `0xe22d1b26`.
```solidity
function isUserDecryptionReady(CtHandleContractPair[] memory ctHandleContractPairs, bytes memory) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReady_1Call {
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isUserDecryptionReady((bytes32,address)[],bytes)`](isUserDecryptionReady_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReady_1Return {
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
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isUserDecryptionReady_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReady_1Call) -> Self {
                    (value.ctHandleContractPairs, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReady_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandleContractPairs: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isUserDecryptionReady_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReady_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReady_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isUserDecryptionReady_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isUserDecryptionReady((bytes32,address)[],bytes)";
            const SELECTOR: [u8; 4] = [226u8, 45u8, 27u8, 38u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
                        let r: isUserDecryptionReady_1Return = r.into();
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
                        let r: isUserDecryptionReady_1Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isUserDecryptionReady(address,(bytes32,address)[],bytes)` and selector `0xfbb83259`.
```solidity
function isUserDecryptionReady(address, CtHandleContractPair[] memory ctHandleContractPairs, bytes memory extraData) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReady_2Call {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isUserDecryptionReady(address,(bytes32,address)[],bytes)`](isUserDecryptionReady_2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isUserDecryptionReady_2Return {
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isUserDecryptionReady_2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReady_2Call) -> Self {
                    (value._0, value.ctHandleContractPairs, value.extraData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReady_2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        ctHandleContractPairs: tuple.1,
                        extraData: tuple.2,
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
            impl ::core::convert::From<isUserDecryptionReady_2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: isUserDecryptionReady_2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isUserDecryptionReady_2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isUserDecryptionReady_2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isUserDecryptionReady(address,(bytes32,address)[],bytes)";
            const SELECTOR: [u8; 4] = [251u8, 184u8, 50u8, 89u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        CtHandleContractPair,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.ctHandleContractPairs,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
                        let r: isUserDecryptionReady_2Return = r.into();
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
                        let r: isUserDecryptionReady_2Return = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pause()` and selector `0x8456cb59`.
```solidity
function pause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall;
    ///Container type for the return parameters of the [`pause()`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
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
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseReturn {
            fn _tokenize(
                &self,
            ) -> <pauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause()";
            const SELECTOR: [u8; 4] = [132u8, 86u8, 203u8, 89u8];
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
                pauseReturn::_tokenize(ret)
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
    /**Function with signature `paused()` and selector `0x5c975abb`.
```solidity
function paused() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](pausedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pausedReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<pausedCall> for UnderlyingRustTuple<'_> {
                fn from(value: pausedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<pausedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pausedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pausedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pausedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
                        let r: pausedReturn = r.into();
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
                        let r: pausedReturn = r.into();
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
    /**Function with signature `publicDecryptionRequest(bytes32[],bytes)` and selector `0xd8998f45`.
```solidity
function publicDecryptionRequest(bytes32[] memory ctHandles, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct publicDecryptionRequestCall {
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`publicDecryptionRequest(bytes32[],bytes)`](publicDecryptionRequestCall) function.
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<publicDecryptionRequestCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: publicDecryptionRequestCall) -> Self {
                    (value.ctHandles, value.extraData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for publicDecryptionRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandles: tuple.0,
                        extraData: tuple.1,
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
        impl publicDecryptionRequestReturn {
            fn _tokenize(
                &self,
            ) -> <publicDecryptionRequestCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for publicDecryptionRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = publicDecryptionRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "publicDecryptionRequest(bytes32[],bytes)";
            const SELECTOR: [u8; 4] = [216u8, 153u8, 143u8, 69u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                publicDecryptionRequestReturn::_tokenize(ret)
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
    /**Function with signature `publicDecryptionResponse(uint256,bytes,bytes,bytes)` and selector `0x6f8913bc`.
```solidity
function publicDecryptionResponse(uint256 decryptionId, bytes memory decryptedResult, bytes memory signature, bytes memory extraData) external;
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
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`publicDecryptionResponse(uint256,bytes,bytes,bytes)`](publicDecryptionResponseCall) function.
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
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
                    (
                        value.decryptionId,
                        value.decryptedResult,
                        value.signature,
                        value.extraData,
                    )
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
                        extraData: tuple.3,
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
        impl publicDecryptionResponseReturn {
            fn _tokenize(
                &self,
            ) -> <publicDecryptionResponseCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for publicDecryptionResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
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
            const SIGNATURE: &'static str = "publicDecryptionResponse(uint256,bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [111u8, 137u8, 19u8, 188u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                publicDecryptionResponseReturn::_tokenize(ret)
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
    /**Function with signature `reinitializeV4()` and selector `0x123abb28`.
```solidity
function reinitializeV4() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV4Call;
    ///Container type for the return parameters of the [`reinitializeV4()`](reinitializeV4Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV4Return {}
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
            impl ::core::convert::From<reinitializeV4Call> for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV4Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV4Call {
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
            impl ::core::convert::From<reinitializeV4Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV4Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reinitializeV4Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reinitializeV4Return {
            fn _tokenize(
                &self,
            ) -> <reinitializeV4Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reinitializeV4Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV4Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV4()";
            const SELECTOR: [u8; 4] = [18u8, 58u8, 187u8, 40u8];
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
                reinitializeV4Return::_tokenize(ret)
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
    /**Function with signature `unpause()` and selector `0x3f4ba83a`.
```solidity
function unpause() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall;
    ///Container type for the return parameters of the [`unpause()`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
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
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl unpauseReturn {
            fn _tokenize(
                &self,
            ) -> <unpauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause()";
            const SELECTOR: [u8; 4] = [63u8, 75u8, 168u8, 58u8];
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
                unpauseReturn::_tokenize(ret)
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
    /**Function with signature `userDecryptionRequest((bytes32,address,address)[],address,bytes,address[],(uint256,uint256),bytes,bytes)` and selector `0xb4de2c37`.
```solidity
function userDecryptionRequest(HandleEntry[] memory handles, address userAddress, bytes memory publicKey, address[] memory allowedContracts, IDecryption.RequestValiditySeconds memory requestValidity, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequest_0Call {
        #[allow(missing_docs)]
        pub handles: alloy::sol_types::private::Vec<
            <HandleEntry as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub allowedContracts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`userDecryptionRequest((bytes32,address,address)[],address,bytes,address[],(uint256,uint256),bytes,bytes)`](userDecryptionRequest_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequest_0Return {}
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
                alloy::sol_types::sol_data::Array<HandleEntry>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                IDecryption::RequestValiditySeconds,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <HandleEntry as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                <IDecryption::RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<userDecryptionRequest_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequest_0Call) -> Self {
                    (
                        value.handles,
                        value.userAddress,
                        value.publicKey,
                        value.allowedContracts,
                        value.requestValidity,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequest_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        handles: tuple.0,
                        userAddress: tuple.1,
                        publicKey: tuple.2,
                        allowedContracts: tuple.3,
                        requestValidity: tuple.4,
                        signature: tuple.5,
                        extraData: tuple.6,
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
            impl ::core::convert::From<userDecryptionRequest_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequest_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequest_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl userDecryptionRequest_0Return {
            fn _tokenize(
                &self,
            ) -> <userDecryptionRequest_0Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDecryptionRequest_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<HandleEntry>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                IDecryption::RequestValiditySeconds,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userDecryptionRequest_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userDecryptionRequest((bytes32,address,address)[],address,bytes,address[],(uint256,uint256),bytes,bytes)";
            const SELECTOR: [u8; 4] = [180u8, 222u8, 44u8, 55u8];
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
                        HandleEntry,
                    > as alloy_sol_types::SolType>::tokenize(&self.handles),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.allowedContracts),
                    <IDecryption::RequestValiditySeconds as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                userDecryptionRequest_0Return::_tokenize(ret)
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
    /**Function with signature `userDecryptionRequest((bytes32,address)[],(uint256,uint256),(uint256,address[]),address,bytes,bytes,bytes)` and selector `0xf1b57adb`.
```solidity
function userDecryptionRequest(CtHandleContractPair[] memory ctHandleContractPairs, IDecryption.RequestValidity memory requestValidity, IDecryption.ContractsInfo memory contractsInfo, address userAddress, bytes memory publicKey, bytes memory signature, bytes memory extraData) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequest_1Call {
        #[allow(missing_docs)]
        pub ctHandleContractPairs: alloy::sol_types::private::Vec<
            <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`userDecryptionRequest((bytes32,address)[],(uint256,uint256),(uint256,address[]),address,bytes,bytes,bytes)`](userDecryptionRequest_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDecryptionRequest_1Return {}
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
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
                >,
                <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
                <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<userDecryptionRequest_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequest_1Call) -> Self {
                    (
                        value.ctHandleContractPairs,
                        value.requestValidity,
                        value.contractsInfo,
                        value.userAddress,
                        value.publicKey,
                        value.signature,
                        value.extraData,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequest_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        ctHandleContractPairs: tuple.0,
                        requestValidity: tuple.1,
                        contractsInfo: tuple.2,
                        userAddress: tuple.3,
                        publicKey: tuple.4,
                        signature: tuple.5,
                        extraData: tuple.6,
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
            impl ::core::convert::From<userDecryptionRequest_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDecryptionRequest_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDecryptionRequest_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl userDecryptionRequest_1Return {
            fn _tokenize(
                &self,
            ) -> <userDecryptionRequest_1Call as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDecryptionRequest_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<CtHandleContractPair>,
                IDecryption::RequestValidity,
                IDecryption::ContractsInfo,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userDecryptionRequest_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userDecryptionRequest((bytes32,address)[],(uint256,uint256),(uint256,address[]),address,bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [241u8, 181u8, 122u8, 219u8];
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
                    <IDecryption::ContractsInfo as alloy_sol_types::SolType>::tokenize(
                        &self.contractsInfo,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                userDecryptionRequest_1Return::_tokenize(ret)
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
    /**Function with signature `userDecryptionResponse(uint256,bytes,bytes,bytes)` and selector `0x046f9eb3`.
```solidity
function userDecryptionResponse(uint256 decryptionId, bytes memory userDecryptedShare, bytes memory signature, bytes memory extraData) external;
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
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`userDecryptionResponse(uint256,bytes,bytes,bytes)`](userDecryptionResponseCall) function.
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
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
                    (
                        value.decryptionId,
                        value.userDecryptedShare,
                        value.signature,
                        value.extraData,
                    )
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
                        extraData: tuple.3,
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
        impl userDecryptionResponseReturn {
            fn _tokenize(
                &self,
            ) -> <userDecryptionResponseCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDecryptionResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
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
            const SIGNATURE: &'static str = "userDecryptionResponse(uint256,bytes,bytes,bytes)";
            const SELECTOR: [u8; 4] = [4u8, 111u8, 158u8, 179u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                userDecryptionResponseReturn::_tokenize(ret)
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
    ///Container for all the [`Decryption`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum DecryptionCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        delegatedUserDecryptionRequest(delegatedUserDecryptionRequestCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getDecryptionConsensusTxSenders(getDecryptionConsensusTxSendersCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isDecryptionDone(isDecryptionDoneCall),
        #[allow(missing_docs)]
        isDelegatedUserDecryptionReady(isDelegatedUserDecryptionReadyCall),
        #[allow(missing_docs)]
        isPublicDecryptionReady(isPublicDecryptionReadyCall),
        #[allow(missing_docs)]
        isUserDecryptionReady_0(isUserDecryptionReady_0Call),
        #[allow(missing_docs)]
        isUserDecryptionReady_1(isUserDecryptionReady_1Call),
        #[allow(missing_docs)]
        isUserDecryptionReady_2(isUserDecryptionReady_2Call),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        paused(pausedCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        publicDecryptionRequest(publicDecryptionRequestCall),
        #[allow(missing_docs)]
        publicDecryptionResponse(publicDecryptionResponseCall),
        #[allow(missing_docs)]
        reinitializeV4(reinitializeV4Call),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
        #[allow(missing_docs)]
        userDecryptionRequest_0(userDecryptionRequest_0Call),
        #[allow(missing_docs)]
        userDecryptionRequest_1(userDecryptionRequest_1Call),
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
            [4u8, 111u8, 158u8, 179u8],
            [9u8, 0u8, 204u8, 105u8],
            [13u8, 142u8, 110u8, 44u8],
            [18u8, 58u8, 187u8, 40u8],
            [57u8, 247u8, 56u8, 16u8],
            [63u8, 75u8, 168u8, 58u8],
            [64u8, 20u8, 196u8, 205u8],
            [65u8, 11u8, 240u8, 186u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [88u8, 245u8, 184u8, 171u8],
            [92u8, 151u8, 90u8, 187u8],
            [111u8, 137u8, 19u8, 188u8],
            [118u8, 34u8, 126u8, 237u8],
            [132u8, 86u8, 203u8, 89u8],
            [132u8, 176u8, 25u8, 110u8],
            [159u8, 173u8, 90u8, 47u8],
            [173u8, 60u8, 177u8, 204u8],
            [180u8, 222u8, 44u8, 55u8],
            [216u8, 153u8, 143u8, 69u8],
            [226u8, 45u8, 27u8, 38u8],
            [241u8, 181u8, 122u8, 219u8],
            [251u8, 184u8, 50u8, 89u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionCalls {
        const NAME: &'static str = "DecryptionCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegatedUserDecryptionRequest(_) => {
                    <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDecryptionConsensusTxSenders(_) => {
                    <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDecryptionDone(_) => {
                    <isDecryptionDoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isDelegatedUserDecryptionReady(_) => {
                    <isDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isPublicDecryptionReady(_) => {
                    <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isUserDecryptionReady_0(_) => {
                    <isUserDecryptionReady_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isUserDecryptionReady_1(_) => {
                    <isUserDecryptionReady_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isUserDecryptionReady_2(_) => {
                    <isUserDecryptionReady_2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused(_) => <pausedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionRequest(_) => {
                    <publicDecryptionRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::publicDecryptionResponse(_) => {
                    <publicDecryptionResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV4(_) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::upgradeToAndCall(_) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userDecryptionRequest_0(_) => {
                    <userDecryptionRequest_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userDecryptionRequest_1(_) => {
                    <userDecryptionRequest_1Call as alloy_sol_types::SolCall>::SELECTOR
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
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<DecryptionCalls>] = &[
                {
                    fn userDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionResponse)
                    }
                    userDecryptionResponse
                },
                {
                    fn getDecryptionConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::getDecryptionConsensusTxSenders)
                    }
                    getDecryptionConsensusTxSenders
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionCalls::unpause)
                    }
                    unpause
                },
                {
                    fn isPublicDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isPublicDecryptionReady)
                    }
                    isPublicDecryptionReady
                },
                {
                    fn isUserDecryptionReady_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReady_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady_0)
                    }
                    isUserDecryptionReady_0
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn isDecryptionDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isDecryptionDone)
                    }
                    isDecryptionDone
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionCalls::paused)
                    }
                    paused
                },
                {
                    fn publicDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionResponse)
                    }
                    publicDecryptionResponse
                },
                {
                    fn isDelegatedUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isDelegatedUserDecryptionReady)
                    }
                    isDelegatedUserDecryptionReady
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(DecryptionCalls::pause)
                    }
                    pause
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn delegatedUserDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::delegatedUserDecryptionRequest)
                    }
                    delegatedUserDecryptionRequest
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn userDecryptionRequest_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionRequest_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionRequest_0)
                    }
                    userDecryptionRequest_0
                },
                {
                    fn publicDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionRequest)
                    }
                    publicDecryptionRequest
                },
                {
                    fn isUserDecryptionReady_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReady_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady_1)
                    }
                    isUserDecryptionReady_1
                },
                {
                    fn userDecryptionRequest_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionRequest_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionRequest_1)
                    }
                    userDecryptionRequest_1
                },
                {
                    fn isUserDecryptionReady_2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReady_2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady_2)
                    }
                    isUserDecryptionReady_2
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
            ) -> alloy_sol_types::Result<DecryptionCalls>] = &[
                {
                    fn userDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionResponse)
                    }
                    userDecryptionResponse
                },
                {
                    fn getDecryptionConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::getDecryptionConsensusTxSenders)
                    }
                    getDecryptionConsensusTxSenders
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn reinitializeV4(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV4Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV4)
                    }
                    reinitializeV4
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::unpause)
                    }
                    unpause
                },
                {
                    fn isPublicDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isPublicDecryptionReady)
                    }
                    isPublicDecryptionReady
                },
                {
                    fn isUserDecryptionReady_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReady_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady_0)
                    }
                    isUserDecryptionReady_0
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn isDecryptionDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isDecryptionDone)
                    }
                    isDecryptionDone
                },
                {
                    fn paused(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pausedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::paused)
                    }
                    paused
                },
                {
                    fn publicDecryptionResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionResponse)
                    }
                    publicDecryptionResponse
                },
                {
                    fn isDelegatedUserDecryptionReady(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isDelegatedUserDecryptionReady)
                    }
                    isDelegatedUserDecryptionReady
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<DecryptionCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::pause)
                    }
                    pause
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn delegatedUserDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::delegatedUserDecryptionRequest)
                    }
                    delegatedUserDecryptionRequest
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn userDecryptionRequest_0(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionRequest_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionRequest_0)
                    }
                    userDecryptionRequest_0
                },
                {
                    fn publicDecryptionRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <publicDecryptionRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::publicDecryptionRequest)
                    }
                    publicDecryptionRequest
                },
                {
                    fn isUserDecryptionReady_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReady_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady_1)
                    }
                    isUserDecryptionReady_1
                },
                {
                    fn userDecryptionRequest_1(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <userDecryptionRequest_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::userDecryptionRequest_1)
                    }
                    userDecryptionRequest_1
                },
                {
                    fn isUserDecryptionReady_2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <isUserDecryptionReady_2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::isUserDecryptionReady_2)
                    }
                    isUserDecryptionReady_2
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
                Self::getDecryptionConsensusTxSenders(inner) => {
                    <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isDecryptionDone(inner) => {
                    <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isDelegatedUserDecryptionReady(inner) => {
                    <isDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isPublicDecryptionReady(inner) => {
                    <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isUserDecryptionReady_0(inner) => {
                    <isUserDecryptionReady_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isUserDecryptionReady_1(inner) => {
                    <isUserDecryptionReady_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isUserDecryptionReady_2(inner) => {
                    <isUserDecryptionReady_2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::reinitializeV4(inner) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userDecryptionRequest_0(inner) => {
                    <userDecryptionRequest_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userDecryptionRequest_1(inner) => {
                    <userDecryptionRequest_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getDecryptionConsensusTxSenders(inner) => {
                    <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isDecryptionDone(inner) => {
                    <isDecryptionDoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isDelegatedUserDecryptionReady(inner) => {
                    <isDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isPublicDecryptionReady(inner) => {
                    <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isUserDecryptionReady_0(inner) => {
                    <isUserDecryptionReady_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isUserDecryptionReady_1(inner) => {
                    <isUserDecryptionReady_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isUserDecryptionReady_2(inner) => {
                    <isUserDecryptionReady_2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused(inner) => {
                    <pausedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::reinitializeV4(inner) => {
                    <reinitializeV4Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::upgradeToAndCall(inner) => {
                    <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userDecryptionRequest_0(inner) => {
                    <userDecryptionRequest_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userDecryptionRequest_1(inner) => {
                    <userDecryptionRequest_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum DecryptionErrors {
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        ContractAddressesMaxLengthExceeded(ContractAddressesMaxLengthExceeded),
        #[allow(missing_docs)]
        ContractNotInContractAddresses(ContractNotInContractAddresses),
        #[allow(missing_docs)]
        CoprocessorSignerDoesNotMatchTxSender(CoprocessorSignerDoesNotMatchTxSender),
        #[allow(missing_docs)]
        CtHandleChainIdDiffersFromContractChainId(
            CtHandleChainIdDiffersFromContractChainId,
        ),
        #[allow(missing_docs)]
        DecryptionNotRequested(DecryptionNotRequested),
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
        EmptyContractAddresses(EmptyContractAddresses),
        #[allow(missing_docs)]
        EmptyCtHandleContractPairs(EmptyCtHandleContractPairs),
        #[allow(missing_docs)]
        EmptyCtHandles(EmptyCtHandles),
        #[allow(missing_docs)]
        EmptyHandles(EmptyHandles),
        #[allow(missing_docs)]
        EnforcedPause(EnforcedPause),
        #[allow(missing_docs)]
        ExpectedPause(ExpectedPause),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        HostChainNotRegistered(HostChainNotRegistered),
        #[allow(missing_docs)]
        InvalidExtraDataLength(InvalidExtraDataLength),
        #[allow(missing_docs)]
        InvalidFHEType(InvalidFHEType),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidNullDurationDays(InvalidNullDurationDays),
        #[allow(missing_docs)]
        InvalidNullDurationSeconds(InvalidNullDurationSeconds),
        #[allow(missing_docs)]
        InvalidUserSignature(InvalidUserSignature),
        #[allow(missing_docs)]
        KmsNodeAlreadySigned(KmsNodeAlreadySigned),
        #[allow(missing_docs)]
        KmsSignerDoesNotMatchTxSender(KmsSignerDoesNotMatchTxSender),
        #[allow(missing_docs)]
        MaxDecryptionRequestBitSizeExceeded(MaxDecryptionRequestBitSizeExceeded),
        #[allow(missing_docs)]
        MaxDurationDaysExceeded(MaxDurationDaysExceeded),
        #[allow(missing_docs)]
        MaxDurationSecondsExceeded(MaxDurationSecondsExceeded),
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
        NotOwnerOrGatewayConfig(NotOwnerOrGatewayConfig),
        #[allow(missing_docs)]
        NotPauserOrGatewayConfig(NotPauserOrGatewayConfig),
        #[allow(missing_docs)]
        StartTimestampInFuture(StartTimestampInFuture),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnsupportedExtraDataVersion(UnsupportedExtraDataVersion),
        #[allow(missing_docs)]
        UnsupportedFHEType(UnsupportedFHEType),
        #[allow(missing_docs)]
        UserAddressInContractAddresses(UserAddressInContractAddresses),
        #[allow(missing_docs)]
        UserDecryptionRequestExpired(UserDecryptionRequestExpired),
        #[allow(missing_docs)]
        UserDecryptionRequestExpiredSeconds(UserDecryptionRequestExpiredSeconds),
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
            [13u8, 134u8, 245u8, 33u8],
            [14u8, 86u8, 207u8, 61u8],
            [33u8, 57u8, 204u8, 44u8],
            [36u8, 14u8, 147u8, 9u8],
            [38u8, 205u8, 117u8, 220u8],
            [42u8, 124u8, 110u8, 246u8],
            [42u8, 135u8, 61u8, 39u8],
            [45u8, 231u8, 84u8, 56u8],
            [48u8, 52u8, 128u8, 64u8],
            [50u8, 149u8, 24u8, 99u8],
            [56u8, 137u8, 22u8, 187u8],
            [57u8, 22u8, 114u8, 167u8],
            [72u8, 167u8, 136u8, 220u8],
            [76u8, 156u8, 140u8, 227u8],
            [82u8, 215u8, 37u8, 245u8],
            [87u8, 207u8, 162u8, 23u8],
            [100u8, 25u8, 80u8, 215u8],
            [103u8, 143u8, 207u8, 206u8],
            [111u8, 79u8, 115u8, 31u8],
            [141u8, 252u8, 32u8, 43u8],
            [147u8, 84u8, 138u8, 102u8],
            [149u8, 144u8, 233u8, 22u8],
            [153u8, 150u8, 179u8, 21u8],
            [153u8, 236u8, 72u8, 217u8],
            [164u8, 195u8, 3u8, 145u8],
            [166u8, 166u8, 203u8, 33u8],
            [170u8, 29u8, 73u8, 164u8],
            [174u8, 82u8, 235u8, 18u8],
            [174u8, 232u8, 99u8, 35u8],
            [175u8, 31u8, 4u8, 149u8],
            [179u8, 152u8, 151u8, 159u8],
            [182u8, 103u8, 156u8, 59u8],
            [190u8, 120u8, 48u8, 177u8],
            [195u8, 68u8, 106u8, 199u8],
            [207u8, 174u8, 146u8, 31u8],
            [212u8, 138u8, 249u8, 66u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [217u8, 60u8, 6u8, 101u8],
            [220u8, 77u8, 120u8, 177u8],
            [222u8, 40u8, 89u8, 193u8],
            [224u8, 124u8, 141u8, 186u8],
            [225u8, 52u8, 191u8, 98u8],
            [225u8, 145u8, 102u8, 238u8],
            [231u8, 244u8, 137u8, 93u8],
            [242u8, 76u8, 8u8, 135u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 36u8, 160u8, 207u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DecryptionErrors {
        const NAME: &'static str = "DecryptionErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 51usize;
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
                Self::CoprocessorSignerDoesNotMatchTxSender(_) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CtHandleChainIdDiffersFromContractChainId(_) => {
                    <CtHandleChainIdDiffersFromContractChainId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DecryptionNotRequested(_) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyContractAddresses(_) => {
                    <EmptyContractAddresses as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCtHandleContractPairs(_) => {
                    <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyCtHandles(_) => {
                    <EmptyCtHandles as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyHandles(_) => {
                    <EmptyHandles as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EnforcedPause(_) => {
                    <EnforcedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpectedPause(_) => {
                    <ExpectedPause as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::HostChainNotRegistered(_) => {
                    <HostChainNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidExtraDataLength(_) => {
                    <InvalidExtraDataLength as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidNullDurationSeconds(_) => {
                    <InvalidNullDurationSeconds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidUserSignature(_) => {
                    <InvalidUserSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsNodeAlreadySigned(_) => {
                    <KmsNodeAlreadySigned as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsSignerDoesNotMatchTxSender(_) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDecryptionRequestBitSizeExceeded(_) => {
                    <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDurationDaysExceeded(_) => {
                    <MaxDurationDaysExceeded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MaxDurationSecondsExceeded(_) => {
                    <MaxDurationSecondsExceeded as alloy_sol_types::SolError>::SELECTOR
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
                Self::NotOwnerOrGatewayConfig(_) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotPauserOrGatewayConfig(_) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::SELECTOR
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
                Self::UnsupportedExtraDataVersion(_) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::SELECTOR
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
                Self::UserDecryptionRequestExpiredSeconds(_) => {
                    <UserDecryptionRequestExpiredSeconds as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<DecryptionErrors>] = &[
                {
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
                },
                {
                    fn NotGatewayOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotGatewayOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotGatewayOwner)
                    }
                    NotGatewayOwner
                },
                {
                    fn UnsupportedExtraDataVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UnsupportedExtraDataVersion)
                    }
                    UnsupportedExtraDataVersion
                },
                {
                    fn EmptyHandles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyHandles as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DecryptionErrors::EmptyHandles)
                    }
                    EmptyHandles
                },
                {
                    fn NotCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorSigner)
                    }
                    NotCoprocessorSigner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DecryptionErrors::NotKmsSigner)
                    }
                    NotKmsSigner
                },
                {
                    fn InvalidUserSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidUserSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidUserSignature)
                    }
                    InvalidUserSignature
                },
                {
                    fn EmptyCtHandles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandles as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandles)
                    }
                    EmptyCtHandles
                },
                {
                    fn UserDecryptionRequestExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UserDecryptionRequestExpired)
                    }
                    UserDecryptionRequestExpired
                },
                {
                    fn MaxDurationDaysExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::MaxDurationDaysExceeded)
                    }
                    MaxDurationDaysExceeded
                },
                {
                    fn NotPauserOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotPauserOrGatewayConfig)
                    }
                    NotPauserOrGatewayConfig
                },
                {
                    fn NotCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianSigner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianSigner)
                    }
                    NotCustodianSigner
                },
                {
                    fn InvalidNullDurationSeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullDurationSeconds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullDurationSeconds)
                    }
                    InvalidNullDurationSeconds
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
                },
                {
                    fn EmptyContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EmptyContractAddresses)
                    }
                    EmptyContractAddresses
                },
                {
                    fn InvalidFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidFHEType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidFHEType)
                    }
                    InvalidFHEType
                },
                {
                    fn UserDecryptionRequestExpiredSeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserDecryptionRequestExpiredSeconds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UserDecryptionRequestExpiredSeconds)
                    }
                    UserDecryptionRequestExpiredSeconds
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn InvalidExtraDataLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidExtraDataLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidExtraDataLength)
                    }
                    InvalidExtraDataLength
                },
                {
                    fn CtHandleChainIdDiffersFromContractChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <CtHandleChainIdDiffersFromContractChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                DecryptionErrors::CtHandleChainIdDiffersFromContractChainId,
                            )
                    }
                    CtHandleChainIdDiffersFromContractChainId
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn KmsNodeAlreadySigned(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::KmsNodeAlreadySigned)
                    }
                    KmsNodeAlreadySigned
                },
                {
                    fn ContractNotInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ContractNotInContractAddresses)
                    }
                    ContractNotInContractAddresses
                },
                {
                    fn EmptyCtHandleContractPairs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandleContractPairs)
                    }
                    EmptyCtHandleContractPairs
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn MaxDurationSecondsExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDurationSecondsExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::MaxDurationSecondsExceeded)
                    }
                    MaxDurationSecondsExceeded
                },
                {
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
                },
                {
                    fn ContractAddressesMaxLengthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ContractAddressesMaxLengthExceeded)
                    }
                    ContractAddressesMaxLengthExceeded
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn HostChainNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <HostChainNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::HostChainNotRegistered)
                    }
                    HostChainNotRegistered
                },
                {
                    fn UnsupportedFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UnsupportedFHEType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UnsupportedFHEType)
                    }
                    UnsupportedFHEType
                },
                {
                    fn DelegatorAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DelegatorAddressInContractAddresses)
                    }
                    DelegatorAddressInContractAddresses
                },
                {
                    fn DifferentKeyIdsNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DifferentKeyIdsNotAllowed)
                    }
                    DifferentKeyIdsNotAllowed
                },
                {
                    fn DecryptionNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionNotRequested)
                    }
                    DecryptionNotRequested
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(DecryptionErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn UserAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UserAddressInContractAddresses)
                    }
                    UserAddressInContractAddresses
                },
                {
                    fn InvalidNullDurationDays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullDurationDays)
                    }
                    InvalidNullDurationDays
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn CoprocessorSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::CoprocessorSignerDoesNotMatchTxSender)
                    }
                    CoprocessorSignerDoesNotMatchTxSender
                },
                {
                    fn NotOwnerOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotOwnerOrGatewayConfig)
                    }
                    NotOwnerOrGatewayConfig
                },
                {
                    fn MaxDecryptionRequestBitSizeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::MaxDecryptionRequestBitSizeExceeded)
                    }
                    MaxDecryptionRequestBitSizeExceeded
                },
                {
                    fn StartTimestampInFuture(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <StartTimestampInFuture as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::StartTimestampInFuture)
                    }
                    StartTimestampInFuture
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn NotCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianTxSender)
                    }
                    NotCustodianTxSender
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
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
            ) -> alloy_sol_types::Result<DecryptionErrors>] = &[
                {
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
                },
                {
                    fn NotGatewayOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotGatewayOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotGatewayOwner)
                    }
                    NotGatewayOwner
                },
                {
                    fn UnsupportedExtraDataVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UnsupportedExtraDataVersion)
                    }
                    UnsupportedExtraDataVersion
                },
                {
                    fn EmptyHandles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyHandles as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EmptyHandles)
                    }
                    EmptyHandles
                },
                {
                    fn NotCoprocessorSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorSigner)
                    }
                    NotCoprocessorSigner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotKmsSigner)
                    }
                    NotKmsSigner
                },
                {
                    fn InvalidUserSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidUserSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidUserSignature)
                    }
                    InvalidUserSignature
                },
                {
                    fn EmptyCtHandles(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandles as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandles)
                    }
                    EmptyCtHandles
                },
                {
                    fn UserDecryptionRequestExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserDecryptionRequestExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UserDecryptionRequestExpired)
                    }
                    UserDecryptionRequestExpired
                },
                {
                    fn MaxDurationDaysExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDurationDaysExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::MaxDurationDaysExceeded)
                    }
                    MaxDurationDaysExceeded
                },
                {
                    fn NotPauserOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotPauserOrGatewayConfig)
                    }
                    NotPauserOrGatewayConfig
                },
                {
                    fn NotCustodianSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianSigner)
                    }
                    NotCustodianSigner
                },
                {
                    fn InvalidNullDurationSeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullDurationSeconds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullDurationSeconds)
                    }
                    InvalidNullDurationSeconds
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotCoprocessorTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCoprocessorTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCoprocessorTxSender)
                    }
                    NotCoprocessorTxSender
                },
                {
                    fn EmptyContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EmptyContractAddresses)
                    }
                    EmptyContractAddresses
                },
                {
                    fn InvalidFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidFHEType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidFHEType)
                    }
                    InvalidFHEType
                },
                {
                    fn UserDecryptionRequestExpiredSeconds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserDecryptionRequestExpiredSeconds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UserDecryptionRequestExpiredSeconds)
                    }
                    UserDecryptionRequestExpiredSeconds
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn ExpectedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ExpectedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ExpectedPause)
                    }
                    ExpectedPause
                },
                {
                    fn InvalidExtraDataLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidExtraDataLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidExtraDataLength)
                    }
                    InvalidExtraDataLength
                },
                {
                    fn CtHandleChainIdDiffersFromContractChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <CtHandleChainIdDiffersFromContractChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                DecryptionErrors::CtHandleChainIdDiffersFromContractChainId,
                            )
                    }
                    CtHandleChainIdDiffersFromContractChainId
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn KmsNodeAlreadySigned(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <KmsNodeAlreadySigned as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::KmsNodeAlreadySigned)
                    }
                    KmsNodeAlreadySigned
                },
                {
                    fn ContractNotInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractNotInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ContractNotInContractAddresses)
                    }
                    ContractNotInContractAddresses
                },
                {
                    fn EmptyCtHandleContractPairs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EmptyCtHandleContractPairs)
                    }
                    EmptyCtHandleContractPairs
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn MaxDurationSecondsExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDurationSecondsExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::MaxDurationSecondsExceeded)
                    }
                    MaxDurationSecondsExceeded
                },
                {
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
                },
                {
                    fn ContractAddressesMaxLengthExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ContractAddressesMaxLengthExceeded)
                    }
                    ContractAddressesMaxLengthExceeded
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn HostChainNotRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <HostChainNotRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::HostChainNotRegistered)
                    }
                    HostChainNotRegistered
                },
                {
                    fn UnsupportedFHEType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UnsupportedFHEType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UnsupportedFHEType)
                    }
                    UnsupportedFHEType
                },
                {
                    fn DelegatorAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DelegatorAddressInContractAddresses)
                    }
                    DelegatorAddressInContractAddresses
                },
                {
                    fn DifferentKeyIdsNotAllowed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DifferentKeyIdsNotAllowed)
                    }
                    DifferentKeyIdsNotAllowed
                },
                {
                    fn DecryptionNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionNotRequested)
                    }
                    DecryptionNotRequested
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn EnforcedPause(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <EnforcedPause as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::EnforcedPause)
                    }
                    EnforcedPause
                },
                {
                    fn UserAddressInContractAddresses(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UserAddressInContractAddresses as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UserAddressInContractAddresses)
                    }
                    UserAddressInContractAddresses
                },
                {
                    fn InvalidNullDurationDays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullDurationDays as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullDurationDays)
                    }
                    InvalidNullDurationDays
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn CoprocessorSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::CoprocessorSignerDoesNotMatchTxSender)
                    }
                    CoprocessorSignerDoesNotMatchTxSender
                },
                {
                    fn NotOwnerOrGatewayConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotOwnerOrGatewayConfig)
                    }
                    NotOwnerOrGatewayConfig
                },
                {
                    fn MaxDecryptionRequestBitSizeExceeded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::MaxDecryptionRequestBitSizeExceeded)
                    }
                    MaxDecryptionRequestBitSizeExceeded
                },
                {
                    fn StartTimestampInFuture(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <StartTimestampInFuture as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::StartTimestampInFuture)
                    }
                    StartTimestampInFuture
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn NotCustodianTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <NotCustodianTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::NotCustodianTxSender)
                    }
                    NotCustodianTxSender
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
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
                Self::CoprocessorSignerDoesNotMatchTxSender(inner) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CtHandleChainIdDiffersFromContractChainId(inner) => {
                    <CtHandleChainIdDiffersFromContractChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DecryptionNotRequested(inner) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyContractAddresses(inner) => {
                    <EmptyContractAddresses as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyHandles(inner) => {
                    <EmptyHandles as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::HostChainNotRegistered(inner) => {
                    <HostChainNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidExtraDataLength(inner) => {
                    <InvalidExtraDataLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::InvalidNullDurationSeconds(inner) => {
                    <InvalidNullDurationSeconds as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::KmsSignerDoesNotMatchTxSender(inner) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::MaxDurationSecondsExceeded(inner) => {
                    <MaxDurationSecondsExceeded as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotOwnerOrGatewayConfig(inner) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotPauserOrGatewayConfig(inner) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::UnsupportedExtraDataVersion(inner) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::UserDecryptionRequestExpiredSeconds(inner) => {
                    <UserDecryptionRequestExpiredSeconds as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::CoprocessorSignerDoesNotMatchTxSender(inner) => {
                    <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CtHandleChainIdDiffersFromContractChainId(inner) => {
                    <CtHandleChainIdDiffersFromContractChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DecryptionNotRequested(inner) => {
                    <DecryptionNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyContractAddresses(inner) => {
                    <EmptyContractAddresses as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyHandles(inner) => {
                    <EmptyHandles as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EnforcedPause(inner) => {
                    <EnforcedPause as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ExpectedPause(inner) => {
                    <ExpectedPause as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidExtraDataLength(inner) => {
                    <InvalidExtraDataLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::InvalidNullDurationSeconds(inner) => {
                    <InvalidNullDurationSeconds as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::KmsSignerDoesNotMatchTxSender(inner) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::MaxDurationSecondsExceeded(inner) => {
                    <MaxDurationSecondsExceeded as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotOwnerOrGatewayConfig(inner) => {
                    <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotPauserOrGatewayConfig(inner) => {
                    <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UnsupportedExtraDataVersion(inner) => {
                    <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::UserDecryptionRequestExpiredSeconds(inner) => {
                    <UserDecryptionRequestExpiredSeconds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`Decryption`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum DecryptionEvents {
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        PublicDecryptionRequest(PublicDecryptionRequest),
        #[allow(missing_docs)]
        PublicDecryptionResponse(PublicDecryptionResponse),
        #[allow(missing_docs)]
        PublicDecryptionResponseCall(PublicDecryptionResponseCall),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
        #[allow(missing_docs)]
        UserDecryptionRequest_0(UserDecryptionRequest_0),
        #[allow(missing_docs)]
        UserDecryptionRequest_1(UserDecryptionRequest_1),
        #[allow(missing_docs)]
        UserDecryptionResponse(UserDecryptionResponse),
        #[allow(missing_docs)]
        UserDecryptionResponseThresholdReached(UserDecryptionResponseThresholdReached),
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
                4u8, 71u8, 17u8, 198u8, 76u8, 241u8, 138u8, 191u8, 73u8, 96u8, 34u8,
                14u8, 62u8, 30u8, 207u8, 1u8, 5u8, 156u8, 114u8, 184u8, 21u8, 44u8,
                132u8, 21u8, 181u8, 142u8, 161u8, 253u8, 216u8, 74u8, 43u8, 75u8,
            ],
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                34u8, 219u8, 72u8, 10u8, 57u8, 189u8, 114u8, 85u8, 100u8, 56u8, 170u8,
                219u8, 74u8, 50u8, 163u8, 210u8, 166u8, 99u8, 139u8, 135u8, 192u8, 59u8,
                190u8, 197u8, 254u8, 246u8, 153u8, 126u8, 16u8, 149u8, 135u8, 255u8,
            ],
            [
                77u8, 123u8, 29u8, 186u8, 73u8, 233u8, 232u8, 70u8, 33u8, 94u8, 22u8,
                33u8, 245u8, 115u8, 124u8, 129u8, 216u8, 97u8, 76u8, 79u8, 38u8, 132u8,
                148u8, 216u8, 183u8, 135u8, 99u8, 44u8, 78u8, 89u8, 240u8, 229u8,
            ],
            [
                93u8, 185u8, 238u8, 10u8, 73u8, 91u8, 242u8, 230u8, 255u8, 156u8, 145u8,
                167u8, 131u8, 76u8, 27u8, 164u8, 253u8, 210u8, 68u8, 165u8, 232u8, 170u8,
                78u8, 83u8, 123u8, 211u8, 138u8, 234u8, 228u8, 176u8, 115u8, 170u8,
            ],
            [
                98u8, 231u8, 140u8, 234u8, 1u8, 190u8, 227u8, 32u8, 205u8, 78u8, 66u8,
                2u8, 112u8, 181u8, 234u8, 116u8, 0u8, 13u8, 17u8, 176u8, 201u8, 247u8,
                71u8, 84u8, 235u8, 219u8, 252u8, 84u8, 75u8, 5u8, 162u8, 88u8,
            ],
            [
                127u8, 205u8, 251u8, 83u8, 129u8, 145u8, 127u8, 85u8, 74u8, 113u8, 125u8,
                10u8, 84u8, 112u8, 163u8, 63u8, 90u8, 73u8, 186u8, 100u8, 69u8, 240u8,
                94u8, 196u8, 60u8, 116u8, 192u8, 188u8, 44u8, 198u8, 8u8, 178u8,
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
                215u8, 229u8, 138u8, 54u8, 122u8, 10u8, 108u8, 41u8, 142u8, 118u8, 173u8,
                93u8, 36u8, 0u8, 4u8, 227u8, 39u8, 170u8, 20u8, 35u8, 203u8, 228u8,
                189u8, 127u8, 248u8, 93u8, 76u8, 113u8, 94u8, 248u8, 209u8, 95u8,
            ],
            [
                232u8, 151u8, 82u8, 190u8, 14u8, 205u8, 182u8, 139u8, 42u8, 110u8, 181u8,
                239u8, 26u8, 137u8, 16u8, 57u8, 224u8, 233u8, 42u8, 227u8, 200u8, 166u8,
                34u8, 116u8, 197u8, 136u8, 30u8, 72u8, 238u8, 161u8, 237u8, 37u8,
            ],
            [
                249u8, 1u8, 27u8, 214u8, 186u8, 13u8, 166u8, 4u8, 156u8, 82u8, 13u8,
                112u8, 254u8, 89u8, 113u8, 241u8, 126u8, 215u8, 171u8, 121u8, 84u8,
                134u8, 5u8, 37u8, 68u8, 181u8, 16u8, 25u8, 137u8, 108u8, 89u8, 107u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DecryptionEvents {
        const NAME: &'static str = "DecryptionEvents";
        const COUNT: usize = 12usize;
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
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(
                    <PublicDecryptionRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionRequest)
                }
                Some(
                    <PublicDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionResponse)
                }
                Some(
                    <PublicDecryptionResponseCall as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionResponseCall as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionResponseCall)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
                }
                Some(<Upgraded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Upgraded as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Upgraded)
                }
                Some(
                    <UserDecryptionRequest_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionRequest_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionRequest_0)
                }
                Some(
                    <UserDecryptionRequest_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionRequest_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionRequest_1)
                }
                Some(
                    <UserDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionResponse)
                }
                Some(
                    <UserDecryptionResponseThresholdReached as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionResponseThresholdReached as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionResponseThresholdReached)
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
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionResponseCall(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionRequest_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionRequest_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionResponseThresholdReached(inner) => {
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
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionResponseCall(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionRequest_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionRequest_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionResponseThresholdReached(inner) => {
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
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DecryptionInstance<P, N> {
        DecryptionInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<DecryptionInstance<P, N>>,
    > {
        DecryptionInstance::<P, N>::deploy(provider)
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
        DecryptionInstance::<P, N>::deploy_builder(provider)
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
    pub struct DecryptionInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for DecryptionInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DecryptionInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<P, N> {
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
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<DecryptionInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> DecryptionInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DecryptionInstance<P, N> {
            DecryptionInstance {
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
    > DecryptionInstance<P, N> {
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
        ///Creates a new call builder for the [`delegatedUserDecryptionRequest`] function.
        pub fn delegatedUserDecryptionRequest(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
            delegationAccounts: <IDecryption::DelegationAccounts as alloy::sol_types::SolType>::RustType,
            contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
            publicKey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, delegatedUserDecryptionRequestCall, N> {
            self.call_builder(
                &delegatedUserDecryptionRequestCall {
                    ctHandleContractPairs,
                    requestValidity,
                    delegationAccounts,
                    contractsInfo,
                    publicKey,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`getDecryptionConsensusTxSenders`] function.
        pub fn getDecryptionConsensusTxSenders(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getDecryptionConsensusTxSendersCall, N> {
            self.call_builder(
                &getDecryptionConsensusTxSendersCall {
                    decryptionId,
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
        ///Creates a new call builder for the [`isDecryptionDone`] function.
        pub fn isDecryptionDone(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isDecryptionDoneCall, N> {
            self.call_builder(
                &isDecryptionDoneCall {
                    decryptionId,
                },
            )
        }
        ///Creates a new call builder for the [`isDelegatedUserDecryptionReady`] function.
        pub fn isDelegatedUserDecryptionReady(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, isDelegatedUserDecryptionReadyCall, N> {
            self.call_builder(
                &isDelegatedUserDecryptionReadyCall {
                    ctHandleContractPairs,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`isPublicDecryptionReady`] function.
        pub fn isPublicDecryptionReady(
            &self,
            ctHandles: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, isPublicDecryptionReadyCall, N> {
            self.call_builder(
                &isPublicDecryptionReadyCall {
                    ctHandles,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`isUserDecryptionReady_0`] function.
        pub fn isUserDecryptionReady_0(
            &self,
            handles: alloy::sol_types::private::Vec<
                <HandleEntry as alloy::sol_types::SolType>::RustType,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, isUserDecryptionReady_0Call, N> {
            self.call_builder(
                &isUserDecryptionReady_0Call {
                    handles,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`isUserDecryptionReady_1`] function.
        pub fn isUserDecryptionReady_1(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, isUserDecryptionReady_1Call, N> {
            self.call_builder(
                &isUserDecryptionReady_1Call {
                    ctHandleContractPairs,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`isUserDecryptionReady_2`] function.
        pub fn isUserDecryptionReady_2(
            &self,
            _0: alloy::sol_types::private::Address,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, isUserDecryptionReady_2Call, N> {
            self.call_builder(
                &isUserDecryptionReady_2Call {
                    _0,
                    ctHandleContractPairs,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(&self) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall)
        }
        ///Creates a new call builder for the [`paused`] function.
        pub fn paused(&self) -> alloy_contract::SolCallBuilder<&P, pausedCall, N> {
            self.call_builder(&pausedCall)
        }
        ///Creates a new call builder for the [`proxiableUUID`] function.
        pub fn proxiableUUID(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, proxiableUUIDCall, N> {
            self.call_builder(&proxiableUUIDCall)
        }
        ///Creates a new call builder for the [`publicDecryptionRequest`] function.
        pub fn publicDecryptionRequest(
            &self,
            ctHandles: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, publicDecryptionRequestCall, N> {
            self.call_builder(
                &publicDecryptionRequestCall {
                    ctHandles,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`publicDecryptionResponse`] function.
        pub fn publicDecryptionResponse(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
            decryptedResult: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, publicDecryptionResponseCall, N> {
            self.call_builder(
                &publicDecryptionResponseCall {
                    decryptionId,
                    decryptedResult,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`reinitializeV4`] function.
        pub fn reinitializeV4(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV4Call, N> {
            self.call_builder(&reinitializeV4Call)
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(&self) -> alloy_contract::SolCallBuilder<&P, unpauseCall, N> {
            self.call_builder(&unpauseCall)
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
        ///Creates a new call builder for the [`userDecryptionRequest_0`] function.
        pub fn userDecryptionRequest_0(
            &self,
            handles: alloy::sol_types::private::Vec<
                <HandleEntry as alloy::sol_types::SolType>::RustType,
            >,
            userAddress: alloy::sol_types::private::Address,
            publicKey: alloy::sol_types::private::Bytes,
            allowedContracts: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            requestValidity: <IDecryption::RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, userDecryptionRequest_0Call, N> {
            self.call_builder(
                &userDecryptionRequest_0Call {
                    handles,
                    userAddress,
                    publicKey,
                    allowedContracts,
                    requestValidity,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`userDecryptionRequest_1`] function.
        pub fn userDecryptionRequest_1(
            &self,
            ctHandleContractPairs: alloy::sol_types::private::Vec<
                <CtHandleContractPair as alloy::sol_types::SolType>::RustType,
            >,
            requestValidity: <IDecryption::RequestValidity as alloy::sol_types::SolType>::RustType,
            contractsInfo: <IDecryption::ContractsInfo as alloy::sol_types::SolType>::RustType,
            userAddress: alloy::sol_types::private::Address,
            publicKey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, userDecryptionRequest_1Call, N> {
            self.call_builder(
                &userDecryptionRequest_1Call {
                    ctHandleContractPairs,
                    requestValidity,
                    contractsInfo,
                    userAddress,
                    publicKey,
                    signature,
                    extraData,
                },
            )
        }
        ///Creates a new call builder for the [`userDecryptionResponse`] function.
        pub fn userDecryptionResponse(
            &self,
            decryptionId: alloy::sol_types::private::primitives::aliases::U256,
            userDecryptedShare: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            extraData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, userDecryptionResponseCall, N> {
            self.call_builder(
                &userDecryptionResponseCall {
                    decryptionId,
                    userDecryptedShare,
                    signature,
                    extraData,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<P, N> {
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
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`PublicDecryptionRequest`] event.
        pub fn PublicDecryptionRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionRequest, N> {
            self.event_filter::<PublicDecryptionRequest>()
        }
        ///Creates a new event filter for the [`PublicDecryptionResponse`] event.
        pub fn PublicDecryptionResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionResponse, N> {
            self.event_filter::<PublicDecryptionResponse>()
        }
        ///Creates a new event filter for the [`PublicDecryptionResponseCall`] event.
        pub fn PublicDecryptionResponseCall_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionResponseCall, N> {
            self.event_filter::<PublicDecryptionResponseCall>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
        ///Creates a new event filter for the [`UserDecryptionRequest_0`] event.
        pub fn UserDecryptionRequest_0_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionRequest_0, N> {
            self.event_filter::<UserDecryptionRequest_0>()
        }
        ///Creates a new event filter for the [`UserDecryptionRequest_1`] event.
        pub fn UserDecryptionRequest_1_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionRequest_1, N> {
            self.event_filter::<UserDecryptionRequest_1>()
        }
        ///Creates a new event filter for the [`UserDecryptionResponse`] event.
        pub fn UserDecryptionResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionResponse, N> {
            self.event_filter::<UserDecryptionResponse>()
        }
        ///Creates a new event filter for the [`UserDecryptionResponseThresholdReached`] event.
        pub fn UserDecryptionResponseThresholdReached_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionResponseThresholdReached, N> {
            self.event_filter::<UserDecryptionResponseThresholdReached>()
        }
    }
}
