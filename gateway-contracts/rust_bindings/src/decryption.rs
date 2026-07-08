///Module containing a contract's types and functions.
/**

```solidity
library IDecryption {
    struct ContractsInfo { uint256 chainId; address[] addresses; }
    struct DelegationAccounts { address delegatorAddress; address delegateAddress; }
    struct RequestValidity { uint256 startTimestamp; uint256 durationDays; }
    struct RequestValiditySeconds { uint256 startTimestamp; uint256 durationSeconds; }
    struct UserDecryptionRequestPayload { address userAddress; bytes publicKey; address[] allowedContracts; RequestValiditySeconds requestValidity; bytes extraData; bytes signature; }
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
        #[allow(dead_code)]
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
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
struct UserDecryptionRequestPayload { address userAddress; bytes publicKey; address[] allowedContracts; RequestValiditySeconds requestValidity; bytes extraData; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserDecryptionRequestPayload {
        #[allow(missing_docs)]
        pub userAddress: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub publicKey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub allowedContracts: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub requestValidity: <RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub extraData: alloy::sol_types::private::Bytes,
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
        #[allow(dead_code)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            RequestValiditySeconds,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            <RequestValiditySeconds as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<UserDecryptionRequestPayload>
        for UnderlyingRustTuple<'_> {
            fn from(value: UserDecryptionRequestPayload) -> Self {
                (
                    value.userAddress,
                    value.publicKey,
                    value.allowedContracts,
                    value.requestValidity,
                    value.extraData,
                    value.signature,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UserDecryptionRequestPayload {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    userAddress: tuple.0,
                    publicKey: tuple.1,
                    allowedContracts: tuple.2,
                    requestValidity: tuple.3,
                    extraData: tuple.4,
                    signature: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UserDecryptionRequestPayload {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for UserDecryptionRequestPayload {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.userAddress,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.publicKey,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.allowedContracts),
                    <RequestValiditySeconds as alloy_sol_types::SolType>::tokenize(
                        &self.requestValidity,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
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
        impl alloy_sol_types::SolType for UserDecryptionRequestPayload {
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
        impl alloy_sol_types::SolStruct for UserDecryptionRequestPayload {
            const NAME: &'static str = "UserDecryptionRequestPayload";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UserDecryptionRequestPayload(address userAddress,bytes publicKey,address[] allowedContracts,RequestValiditySeconds requestValidity,bytes extraData,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <RequestValiditySeconds as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <RequestValiditySeconds as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.userAddress,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.publicKey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.allowedContracts,
                        )
                        .0,
                    <RequestValiditySeconds as alloy_sol_types::SolType>::eip712_data_word(
                            &self.requestValidity,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.extraData,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UserDecryptionRequestPayload {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.userAddress,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.publicKey,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.allowedContracts,
                    )
                    + <RequestValiditySeconds as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.requestValidity,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.extraData,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.publicKey,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.allowedContracts,
                    out,
                );
                <RequestValiditySeconds as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.requestValidity,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.extraData,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
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
        __provider: P,
    ) -> IDecryptionInstance<P, N> {
        IDecryptionInstance::<P, N>::new(address, __provider)
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
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IDecryptionInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IDecryption`](self) contract instance.

See the [wrapper's documentation](`IDecryptionInstance`) for more details.*/
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
    struct UserDecryptionRequestPayload {
        address userAddress;
        bytes publicKey;
        address[] allowedContracts;
        RequestValiditySeconds requestValidity;
        bytes extraData;
        bytes signature;
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
    error DecryptionContextMismatch(uint256 decryptionId, uint256 requestContextId, uint256 responseContextId);
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
    error HostChainDisabled(uint256 chainId);
    error HostChainNotRegistered(uint256 chainId);
    error InvalidExtraDataLength(uint256 length, uint256 minimumLength);
    error InvalidFHEType(uint8 fheTypeUint8);
    error InvalidInitialization();
    error InvalidKmsContext(uint256 kmsContextId);
    error InvalidNullContextId();
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
    event PublicDecryptionRequest(uint256 indexed decryptionId, bytes32[] ctHandles, bytes extraData);
    event PublicDecryptionResponse(uint256 indexed decryptionId, bytes decryptedResult, bytes[] signatures, bytes extraData);
    event PublicDecryptionResponseCall(uint256 indexed decryptionId, bytes decryptedResult, bytes signature, address kmsTxSender, bytes extraData);
    event Unpaused(address account);
    event Upgraded(address indexed implementation);
    event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, address userAddress, bytes publicKey, bytes extraData);
    event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, HandleEntry[] handles, IDecryption.UserDecryptionRequestPayload payload);
    event UserDecryptionRequest(uint256 indexed decryptionId, bytes32[] ctHandles, address userAddress, bytes publicKey, bytes extraData);
    event UserDecryptionRequest(uint256 indexed decryptionId, HandleEntry[] handles, IDecryption.UserDecryptionRequestPayload payload);
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
    function reinitializeV7() external;
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
    "name": "reinitializeV7",
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
    "name": "PublicDecryptionRequest",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "indexed": false,
        "internalType": "bytes32[]"
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
        "name": "payload",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDecryption.UserDecryptionRequestPayload",
        "components": [
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
            "name": "extraData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
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
    "name": "UserDecryptionRequest",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "ctHandles",
        "type": "bytes32[]",
        "indexed": false,
        "internalType": "bytes32[]"
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
        "name": "payload",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IDecryption.UserDecryptionRequestPayload",
        "components": [
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
            "name": "extraData",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "signature",
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
    "name": "DecryptionContextMismatch",
    "inputs": [
      {
        "name": "decryptionId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "requestContextId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "responseContextId",
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
    "name": "HostChainDisabled",
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
    "name": "InvalidNullContextId",
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
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b608051615d5c620001005f395f8181612830015281816128590152612a650152615d5c5ff3fe608060405260043610610178575f3560e01c80636f8913bc116100d1578063b4de2c371161007c578063f1b57adb11610057578063f1b57adb1461044b578063fa2106b81461046a578063fbb832591461047e575f80fd5b8063b4de2c37146103ee578063d8998f451461040d578063e22d1b261461042c575f80fd5b806384b0196e116100ac57806384b0196e146103605780639fad5a2f14610387578063ad3cb1cc146103a6575f80fd5b80636f8913bc1461030e57806376227eed1461032d5780638456cb591461034c575f80fd5b80634014c4cd1161013157806352d1902d1161010c57806352d1902d1461027c57806358f5b8ab1461029e5780635c975abb146102d8575f80fd5b80634014c4cd1461021b578063410bf0ba1461024a5780634f1ef28614610269575f80fd5b80630d8e6e2c116101615780630d8e6e2c146101d257806339f73810146101f35780633f4ba83a14610207575f80fd5b8063046f9eb31461017c5780630900cc691461019d575b5f80fd5b348015610187575f80fd5b5061019b610196366004614594565b61049d565b005b3480156101a8575f80fd5b506101bc6101b7366004614630565b610806565b6040516101c99190614647565b60405180910390f35b3480156101dd575f80fd5b506101e66108d2565b6040516101c991906146e0565b3480156101fe575f80fd5b5061019b61093a565b348015610212575f80fd5b5061019b610ac9565b348015610226575f80fd5b5061023a610235366004614733565b610b9c565b60405190151581526020016101c9565b348015610255575f80fd5b5061023a6102643660046147db565b610c68565b61019b6102773660046148c5565b610d28565b348015610287575f80fd5b50610290610d47565b6040519081526020016101c9565b3480156102a9575f80fd5b5061023a6102b8366004614630565b5f9081525f80516020615d3c833981519152602052604090205460ff1690565b3480156102e3575f80fd5b507fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff1661023a565b348015610319575f80fd5b5061019b610328366004614594565b610d75565b348015610338575f80fd5b5061023a610347366004614993565b61108b565b348015610357575f80fd5b5061019b61114b565b34801561036b575f80fd5b50610374611205565b6040516101c997969594939291906149c9565b348015610392575f80fd5b5061019b6103a1366004614a76565b6112c9565b3480156103b1575f80fd5b506101e66040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b3480156103f9575f80fd5b5061019b610408366004614b7e565b611869565b348015610418575f80fd5b5061019b610427366004614733565b611a11565b348015610437575f80fd5b5061023a610446366004614993565b611c0f565b348015610456575f80fd5b5061019b610465366004614ca5565b611ccf565b348015610475575f80fd5b5061019b6121e0565b348015610489575f80fd5b5061023a610498366004614d92565b612291565b5f80516020615d3c833981519152600160f91b881115806104c15750806008015488115b156104e757604051636a457ca160e11b8152600481018990526024015b60405180910390fd5b5f888152600782016020526040808220815180830190925280548290829061050e90614e23565b80601f016020809104026020016040519081016040528092919081815260200182805461053a90614e23565b80156105855780601f1061055c57610100808354040283529160200191610585565b820191905f5260205f20905b81548152906001019060200180831161056857829003601f168201915b50505050508152602001600182018054806020026020016040519081016040528092919081815260200182805480156105db57602002820191905f5260205f20905b8154815260200190600101908083116105c7575b50505050508152505090505f6040518060800160405280835f01518152602001836020015181526020018a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250604080516020601f8901819004810282018101909252878152918101919088908890819084018382808284375f920182905250939094525092935091506106859050826122a8565b5f8c81526009860160205260408120549192506106a28888612382565b9050815f036106b3578091506106e4565b8181146106e4576040516355dafa4360e11b8152600481018e905260248101839052604481018290526064016104de565b506106f2818d848c8c61254b565b5f8c815260028601602090815260408083208380528252822080546001818101835582855292909320909201805473ffffffffffffffffffffffffffffffffffffffff19163317905581548e917f7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b29161076b9190614e69565b8e8e8e8e8e8e6040516107849796959493929190614ea4565b60405180910390a25f8d81526020879052604090205460ff161580156107b2575080546107b2908390612638565b156107f7575f8d815260208790526040808220805460ff19166001179055518e917fe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed2591a25b50505050505050505050505050565b5f8181527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70360209081526040808320547f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee702835281842081855283529281902080548251818502810185019093528083526060945f80516020615d3c8339815191529490939291908301828280156108c457602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116108a6575b505050505092505050919050565b60606040518060400160405280600a8152602001692232b1b93cb83a34b7b760b11b8152506109005f6126ba565b61090a60076126ba565b6109135f6126ba565b6040516020016109269493929190614ef3565b604051602081830303815290604052905090565b610942612758565b67ffffffffffffffff1660011461096c57604051636f4f731f60e01b815260040160405180910390fd5b60085f610977612771565b8054909150600160401b900460ff168061099f5750805467ffffffffffffffff808416911610155b156109bd5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff191667ffffffffffffffff831617600160401b178155604080518082018252600a8152692232b1b93cb83a34b7b760b11b602080830191909152825180840190935260018352603160f81b90830152610a2191612799565b610a296127ab565b600160f81b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70655600160f91b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70855805468ff00000000000000001916815560405167ffffffffffffffff831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2906020015b60405180910390a15050565b73d582ec82a1758322907df80da8a754e12a5acb956001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b19573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b3d9190614f70565b6001600160a01b0316336001600160a01b031614158015610b7257503373d582ec82a1758322907df80da8a754e12a5acb9514155b15610b92576040516370c8b37760e11b81523360048201526024016104de565b610b9a6127b3565b565b5f838103610bab57505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f878784818110610be157610be1614f8b565b905060200201356040518263ffffffff1660e01b8152600401610c0691815260200190565b602060405180830381865afa158015610c21573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c459190614f9f565b610c52575f915050610c60565b600101610bad565b50600190505b949350505050565b5f838103610c7757505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f878784818110610cad57610cad614f8b565b9050606002015f01356040518263ffffffff1660e01b8152600401610cd491815260200190565b602060405180830381865afa158015610cef573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d139190614f9f565b610d20575f915050610c60565b600101610c79565b610d30612825565b610d39826128dc565b610d438282612986565b5050565b5f610d50612a5a565b507f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc90565b5f80516020615d3c833981519152600160f81b88111580610d995750806006015488115b15610dba57604051636a457ca160e11b8152600481018990526024016104de565b604080515f8a81526005840160209081528382208054608092810285018301909552606084018581529294849392840182828015610e1557602002820191905f5260205f20905b815481526020019060010190808311610e01575b5050505050815260200189898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250604080516020601f8801819004810282018101909252868152918101919087908790819084018382808284375f92018290525093909452509293509150610e9f905082612aa3565b5f8b8152600985016020526040812054919250610ebc8787612382565b9050815f03610ecd57809150610efe565b818114610efe576040516355dafa4360e11b8152600481018d905260248101839052604481018290526064016104de565b610f0b828d858c8c61254b565b5f8c815260048601602090815260408083208684528252822080546001810182558184529190922001610f3f8a8c83615002565b50856002015f8e81526020019081526020015f205f8581526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a8154816001600160a01b0302191690836001600160a01b031602179055508c7f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e58d8d8d8d338e8e604051610fe297969594939291906150bc565b60405180910390a25f8d81526020879052604090205460ff1615801561101057508054611010908490612b4a565b156107f7575f8d815260208781526040808320805460ff191660011790556003890190915290819020859055518d907fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f90611074908f908f9086908e908e90615106565b60405180910390a250505050505050505050505050565b5f83810361109a57505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f8787848181106110d0576110d0614f8b565b9050604002015f01356040518263ffffffff1660e01b81526004016110f791815260200190565b602060405180830381865afa158015611112573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111369190614f9f565b611143575f915050610c60565b60010161109c565b60405163237dfb4760e11b815233600482015273d582ec82a1758322907df80da8a754e12a5acb95906346fbf68e90602401602060405180830381865afa158015611198573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111bc9190614f9f565b1580156111dd57503373d582ec82a1758322907df80da8a754e12a5acb9514155b156111fd5760405163388916bb60e01b81523360048201526024016104de565b610b9a612b86565b5f60608082808083817fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100805490915015801561124357506001810154155b61128f5760405162461bcd60e51b815260206004820152601560248201527f4549503731323a20556e696e697469616c697a6564000000000000000000000060448201526064016104de565b611297612be1565b61129f612cb4565b604080515f80825260208201909252600f60f81b9c939b5091995046985030975095509350915050565b6112d1612d05565b604051635ff9d55d60e11b81528735600482018190529073d582ec82a1758322907df80da8a754e12a5acb959063bff3aaba90602401602060405180830381865afa158015611322573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113469190614f9f565b6113665760405163b6679c3b60e01b8152600481018290526024016104de565b60405163666286dd60e11b81526004810182905273d582ec82a1758322907df80da8a754e12a5acb959063ccc50dba90602401602060405180830381865afa1580156113b4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113d89190614f9f565b156113f95760405163180d9a3160e21b8152600481018290526024016104de565b61140660208901896151f8565b90505f03611427576040516357cfa21760e01b815260040160405180910390fd5b600a61143660208a018a6151f8565b9050111561147257600a61144d60208a018a6151f8565b60405163af1f049560e01b815260ff90931660048401526024830152506044016104de565b611489611484368c90038c018c61528b565b612d48565b6114dc61149960208a018a6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152506114d79250505060208c018c6152a5565b612e14565b15611517576114ee60208a018a6152a5565b6114fb60208a018a6151f8565b60405163c3446ac760e01b81526004016104de939291906152c0565b5f6115238d8d8b612e6d565b90505f6040518060c001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050509082525060209081019061157b908d018d6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152505050908252506020908101906115c0908e018e6152a5565b6001600160a01b031681526020018d5f013581526020018d60200135815260200186868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505091525090506116378161162e60408e0160208f016152a5565b89898e35613064565b5060405163a14f897160e01b81525f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f897190611671908590600401615355565b5f60405180830381865afa15801561168b573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526116b2919081019061538a565b90506116bd816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70880545f80516020615d3c833981519152915f6116f9836154d4565b909155505060088101546040805160606020601f8e01819004028201810183529181018c815290918291908e908e90819085018382808284375f920182905250938552505050602091820187905283815260078501909152604090208151819061176390826154ec565b50602082810151805161177c9260018501920190614462565b509050505f61178b8888612382565b9050611796816131a9565b5f8281526009840160205260409020556117af3361323b565b807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848f60200160208101906117e591906152a5565b8e8e8c8c6040516117fb9695949392919061566e565b60405180910390a2807fc71e32c80f3109c673a9adfcded0b3b2093212ab284084a0e03a281dee2bdd5f858f602001602081019061183991906152a5565b8e8e8c8c60405161184f969594939291906156c4565b60405180910390a250505050505050505050505050505050565b611871612d05565b5f8b90036118925760405163240e930960e01b815260040160405180910390fd5b600a8611156118be5760405163af1f049560e01b8152600a6004820152602481018790526044016104de565b6118d56118d03687900387018761528b565b6132a7565b6118dd6144ab565b6001600160a01b038b168152604080516020601f8c018190048102820181019092528a8152908b908b90819084018382808284375f92019190915250505050602080830191909152604080518983028181018401909252898152918a918a918291908501908490808284375f9201919091525050505060408201526119673687900387018761528b565b6060820152604080516020601f85018190048102820181019092528381529084908490819084018382808284375f920191909152505050506080820152604080516020601f87018190048102820181019092528581529086908690819084018382808284375f92018290525060a0860194909452506119ea915085905084612382565b90506119f53361323b565b611a018e8e848461337e565b5050505050505050505050505050565b611a19612d05565b5f839003611a3a576040516305bcea8760e31b815260040160405180910390fd5b611a758484808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061353f92505050565b60405163a14f897160e01b81525f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f897190611ab0908890889060040161571f565b5f60405180830381865afa158015611aca573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611af1919081019061538a565b9050611afc816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70680545f80516020615d3c833981519152915f611b38836154d4565b909155505060068101545f8181526005830160205260409020611b5c908888614502565b505f611b688686612382565b9050611b73816131a9565b5f828152600984016020526040902055611b8c336135c6565b807f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff848787604051611bc093929190615732565b60405180910390a2807f9bb088a53f1fe13ec84fa0a1d7b578b589e193268baaccc5e23f7ab785aa3dae88888888604051611bfe9493929190615757565b60405180910390a250505050505050565b5f838103611c1e57505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f878784818110611c5457611c54614f8b565b9050604002015f01356040518263ffffffff1660e01b8152600401611c7b91815260200190565b602060405180830381865afa158015611c96573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cba9190614f9f565b611cc7575f915050610c60565b600101611c20565b611cd7612d05565b604051635ff9d55d60e11b81528835600482018190529073d582ec82a1758322907df80da8a754e12a5acb959063bff3aaba90602401602060405180830381865afa158015611d28573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d4c9190614f9f565b611d6c5760405163b6679c3b60e01b8152600481018290526024016104de565b60405163666286dd60e11b81526004810182905273d582ec82a1758322907df80da8a754e12a5acb959063ccc50dba90602401602060405180830381865afa158015611dba573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611dde9190614f9f565b15611dff5760405163180d9a3160e21b8152600481018290526024016104de565b611e0c60208a018a6151f8565b90505f03611e2d576040516357cfa21760e01b815260040160405180910390fd5b600a611e3c60208b018b6151f8565b90501115611e5357600a61144d60208b018b6151f8565b611e65611484368c90038c018c61528b565b611ead611e7560208b018b6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508c9250612e14915050565b15611edc5787611ec060208b018b6151f8565b60405163dc4d78b160e01b81526004016104de939291906152c0565b5f611ee88d8d8c612e6d565b90505f6040518060a001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250602090810190611f40908e018e6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152505050908252508d356020808301919091528e8101356040808401919091528051601f89018390048302810183019091528781526060909201919088908890819084018382808284375f9201919091525050509152509050611fd2818b89898f35613606565b60405163a14f897160e01b81525f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f89719061200b908690600401615355565b5f60405180830381865afa158015612025573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261204c919081019061538a565b9050612057816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70880545f80516020615d3c833981519152915f612093836154d4565b909155505060088101546040805160606020601f8f01819004028201810183529181018d815290918291908f908f90819085018382808284375f92018290525093855250505060209182018890528381526007850190915260409020815181906120fd90826154ec565b5060208281015180516121169260018501920190614462565b509050505f6121258989612382565b9050612130816131a9565b5f8281526009840160205260409020556121493361323b565b807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848f8f8f8d8d6040516121839695949392919061566e565b60405180910390a2807fc71e32c80f3109c673a9adfcded0b3b2093212ab284084a0e03a281dee2bdd5f868f8f8f8d8d6040516121c5969594939291906156c4565b60405180910390a25050505050505050505050505050505050565b60085f6121eb612771565b8054909150600160401b900460ff16806122135750805467ffffffffffffffff808416911610155b156122315760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff191667ffffffffffffffff8316908117600160401b1768ff0000000000000000191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602001610abd565b5f61229e85858585611c0f565b9695505050505050565b5f61237c6040518060a00160405280606d8152602001615b4b606d913980519060200120835f01518051906020012084602001516040516020016122ec9190615788565b60405160208183030381529060405280519060200120856040015180519060200120866060015160405160200161232391906157bd565b60408051601f198184030181528282528051602091820120908301969096528101939093526060830191909152608082015260a081019190915260c0015b60405160208183030381529060405280519060200120613611565b92915050565b5f8181036124055773d582ec82a1758322907df80da8a754e12a5acb956001600160a01b031663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156123da573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123fe91906157d8565b905061237c565b5f83835f81811061241857612418614f8b565b919091013560f81c9150505f8190036124a75773d582ec82a1758322907df80da8a754e12a5acb956001600160a01b031663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561247b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061249f91906157d8565b91505061237c565b8060ff16600114806124bc57508060ff166002145b1561252d5760218310156124ed576040516349aa453360e11b815260048101849052602160248201526044016104de565b6124fb6021600185876157ef565b61250491615816565b91505f8290036125275760405163cb17b7a560e01b815260040160405180910390fd5b5061237c565b60405163084e730b60e21b815260ff821660048201526024016104de565b5f5f80516020615d3c83398151915290505f61259c8585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061363d92505050565b90506125a9878233613665565b5f86815260018301602090815260408083206001600160a01b038516845290915290205460ff1615612600576040516399ec48d960e01b8152600481018790526001600160a01b03821660248201526044016104de565b5f9586526001918201602090815260408088206001600160a01b039093168852919052909420805460ff191690941790935550505050565b60405163140f45ff60e11b8152600481018390525f90819073d582ec82a1758322907df80da8a754e12a5acb959063281e8bfe906024015b602060405180830381865afa15801561268b573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126af91906157d8565b909210159392505050565b60605f6126c6836137d3565b60010190505f8167ffffffffffffffff8111156126e5576126e5614830565b6040519080825280601f01601f19166020018201604052801561270f576020820181803683370190505b5090508181016020015b5f19017f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8504945084612719575b509392505050565b5f612761612771565b5467ffffffffffffffff16919050565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0061237c565b6127a16138b4565b610d4382826138d9565b610b9a6138b4565b6127bb61394b565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300805460ff191681557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a150565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806128be57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166128b27f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546001600160a01b031690565b6001600160a01b031614155b15610b9a5760405163703e46dd60e11b815260040160405180910390fd5b73d582ec82a1758322907df80da8a754e12a5acb956001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561292c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129509190614f70565b6001600160a01b0316336001600160a01b03161461298357604051630e56cf3d60e01b81523360048201526024016104de565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156129e0575060408051601f3d908101601f191682019092526129dd918101906157d8565b60015b612a0857604051634c9c8ce360e01b81526001600160a01b03831660048201526024016104de565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8114612a4b57604051632a87526960e21b8152600481018290526024016104de565b612a55838361398d565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610b9a5760405163703e46dd60e11b815260040160405180910390fd5b5f61237c604051806080016040528060548152602001615bb860549139805160209182012084516040519192612ad99201615788565b604051602081830303815290604052805190602001208460200151805190602001208560400151604051602001612b1091906157bd565b60408051601f198184030181528282528051602091820120908301959095528101929092526060820152608081019190915260a001612361565b6040516361d5552d60e11b8152600481018390525f90819073d582ec82a1758322907df80da8a754e12a5acb959063c3aaaa5a90602401612670565b612b8e612d05565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300805460ff191660011781557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a25833612807565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10280546060917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10091612c3290614e23565b80601f0160208091040260200160405190810160405280929190818152602001828054612c5e90614e23565b8015612ca95780601f10612c8057610100808354040283529160200191612ca9565b820191905f5260205f20905b815481529060010190602001808311612c8c57829003601f168201915b505050505091505090565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10380546060917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10091612c3290614e23565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff1615610b9a5760405163d93c066560e01b815260040160405180910390fd5b80602001515f03612d6c5760405163de2859c160e01b815260040160405180910390fd5b602081015161016d1015612da4576020810151604051633295186360e01b815261016d600482015260248101919091526044016104de565b8051421015612dd257805160405163f24c088760e01b815242600482015260248101919091526044016104de565b42816020015162015180612de69190615833565b8251612df2919061584a565b101561298357428160405162c0d20160e61b81526004016104de92919061585d565b5f805b8351811015612e6457826001600160a01b0316848281518110612e3c57612e3c614f8b565b60200260200101516001600160a01b031603612e5c57600191505061237c565b600101612e17565b505f9392505050565b60605f839003612e905760405163a6a6cb2160e01b815260040160405180910390fd5b8267ffffffffffffffff811115612ea957612ea9614830565b604051908082528060200260200182016040528015612ed2578160200160208202803683370190505b5090505f805b84811015613035575f868683818110612ef357612ef3614f8b565b9050604002015f013590505f878784818110612f1157612f11614f8b565b9050604002016020016020810190612f2991906152a5565b905067ffffffffffffffff601083901c1686358114612f6c57604051634ac8748b60e11b81526004810184905260248101829052873560448201526064016104de565b5f612f76846139e2565b9050612f8181613a2e565b612f8f9061ffff168761584a565b9550612fd9612fa160208a018a6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250879250612e14915050565b6130075782612feb60208a018a6151f8565b60405163a4c3039160e01b81526004016104de939291906152c0565b8387868151811061301a5761301a614f8b565b6020908102919091010152505060019092019150612ed89050565b506108008111156127505760405163e7f4895d60e01b81526108006004820152602481018290526044016104de565b5f61306f8683613b57565b90505f6130b18286868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061363d92505050565b9050856001600160a01b0316816001600160a01b0316146130e9578484604051632a873d2760e01b81526004016104de92919061587b565b50505050505050565b60018151116130fe5750565b5f815f8151811061311157613111614f8b565b60200260200101516020015190505f600190505b8251811015612a55578183828151811061314157613141614f8b565b602002602001015160200151146131a157825f8151811061316457613164614f8b565b602002602001015183828151811061317e5761317e614f8b565b602002602001015160405163cfae921f60e01b81526004016104de92919061588e565b600101613125565b6040516317f362d960e31b81526004810182905273d582ec82a1758322907df80da8a754e12a5acb959063bf9b16c890602401602060405180830381865afa1580156131f7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061321b9190614f9f565b612983576040516377ddbe8160e01b8152600481018290526024016104de565b60405163988a2d2d60e01b81526001600160a01b038216600482015273817a285f1fca3bb4084cbfc77d4babc238ad609c9063988a2d2d906024015b5f604051808303815f87803b15801561328e575f80fd5b505af11580156132a0573d5f803e3d5ffd5b5050505050565b80602001515f036132cb57604051631229e23760e21b815260040160405180910390fd5b6132da61016d62015180615833565b8160200151111561331b576132f461016d62015180615833565b6020820151604051635729758960e11b8152600481019290925260248201526044016104de565b805142101561334957805160405163f24c088760e01b815242600482015260248101919091526044016104de565b60208101518151429161335b9161584a565b10156129835742816040516333c7e7e760e11b81526004016104de92919061585d565b5f6133898585613c49565b60405163a14f897160e01b81529091505f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f8971906133c5908590600401615355565b5f60405180830381865afa1580156133df573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613406919081019061538a565b9050613411816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70880545f80516020615d3c833981519152915f61344d836154d4565b9091555050600881015460408051808201825260208089015182528082018790525f84815260078601909152919091208151819061348b90826154ec565b5060208281015180516134a49260018501920190614462565b5050505f818152600983016020526040908190208690555181907f1f80a47b51979837976f999a7735fdccbbe570e0d40081644ec88f8ed76c9612906134f19086908c908c908c906159a1565b60405180910390a2807f8ed4fab8b1d050676cd7d7d9fa448d5d22113375fab8ad4dc690e54593c527bf89898960405161352d939291906159da565b60405180910390a25050505050505050565b5f805b8251811015613597575f83828151811061355e5761355e614f8b565b602002602001015190505f613572826139e2565b905061357d81613a2e565b61358b9061ffff168561584a565b93505050600101613542565b50610800811115610d435760405163e7f4895d60e01b81526108006004820152602481018290526044016104de565b60405163247bac9f60e21b81526001600160a01b038216600482015273817a285f1fca3bb4084cbfc77d4babc238ad609c906391eeb27c90602401613277565b5f61306f8683613e43565b5f61237c61361d613f01565b8360405161190160f01b8152600281019290925260228201526042902090565b5f805f8061364b8686613f0f565b92509250925061365b8282613f58565b5090949350505050565b604051632511f3f560e21b8152600481018490526001600160a01b038316602482015273d582ec82a1758322907df80da8a754e12a5acb9590639447cfd490604401602060405180830381865afa1580156136c2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136e69190614f9f565b61370e5760405163153e377b60e11b81526001600160a01b03831660048201526024016104de565b60405163063fe83960e31b8152600481018490526001600160a01b03828116602483015283169073d582ec82a1758322907df80da8a754e12a5acb95906331ff41c8906044015f60405180830381865afa15801561376e573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526137959190810190615a41565b602001516001600160a01b031614612a5557604051630d86f52160e01b81526001600160a01b038084166004830152821660248201526044016104de565b5f807a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061381b577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000830492506040015b6d04ee2d6d415b85acef81000000008310613847576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc10000831061386557662386f26fc10000830492506010015b6305f5e100831061387d576305f5e100830492506008015b612710831061389157612710830492506004015b606483106138a3576064830492506002015b600a831061237c5760010192915050565b6138bc614010565b610b9a57604051631afcd79f60e31b815260040160405180910390fd5b6138e16138b4565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1007fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10261392d84826154ec565b506003810161393c83826154ec565b505f8082556001909101555050565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff16610b9a57604051638dfc202b60e01b815260040160405180910390fd5b61399682614029565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156139da57612a5582826140ac565b610d4361411e565b5f600882901c60ff166053811115613a125760405163641950d760e01b815260ff821660048201526024016104de565b8060ff166053811115613a2757613a27614e0f565b9392505050565b5f80826053811115613a4257613a42614e0f565b03613a4f57506002919050565b6002826053811115613a6357613a63614e0f565b03613a7057506008919050565b6003826053811115613a8457613a84614e0f565b03613a9157506010919050565b6004826053811115613aa557613aa5614e0f565b03613ab257506020919050565b6005826053811115613ac657613ac6614e0f565b03613ad357506040919050565b6006826053811115613ae757613ae7614e0f565b03613af457506080919050565b6007826053811115613b0857613b08614e0f565b03613b15575060a0919050565b6008826053811115613b2957613b29614e0f565b03613b375750610100919050565b8160405163be7830b160e01b81526004016104de9190615af2565b919050565b5f806040518060e0016040528060a98152602001615c9360a9913980519060200120845f0151805190602001208560200151604051602001613b999190615b18565b604051602081830303815290604052805190602001208660400151876060015188608001518960a00151604051602001613bd391906157bd565b60408051601f1981840301815282825280516020918201209083019890985281019590955260608501939093526001600160a01b03909116608084015260a083015260c082015260e0810191909152610100015b604051602081830303815290604052805190602001209050610c60838261413d565b60608167ffffffffffffffff811115613c6457613c64614830565b604051908082528060200260200182016040528015613c8d578160200160208202803683370190505b5090505f613cc084845f818110613ca657613ca6614f8b565b606002919091013560101c67ffffffffffffffff16919050565b604051635ff9d55d60e11b81526004810182905290915073d582ec82a1758322907df80da8a754e12a5acb959063bff3aaba90602401602060405180830381865afa158015613d11573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d359190614f9f565b613d555760405163b6679c3b60e01b8152600481018290526024016104de565b5f805b84811015613e0c575f868683818110613d7357613d73614f8b565b606002919091013591505067ffffffffffffffff601082901c16848114613dbe57604051634ac8748b60e11b81526004810183905260248101829052604481018690526064016104de565b5f613dc8836139e2565b9050613dd381613a2e565b613de19061ffff168661584a565b945082878581518110613df657613df6614f8b565b6020908102919091010152505050600101613d58565b50610800811115613e3b5760405163e7f4895d60e01b81526108006004820152602481018290526044016104de565b505092915050565b5f806040518060c0016040528060878152602001615c0c6087913980519060200120845f0151805190602001208560200151604051602001613e859190615b18565b60405160208183030381529060405280519060200120866040015187606001518860800151604051602001613eba91906157bd565b60408051601f198184030181528282528051602091820120908301979097528101949094526060840192909252608083015260a082015260c081019190915260e001613c27565b5f613f0a6141d3565b905090565b5f805f8351604103613f46576020840151604085015160608601515f1a613f3888828585614246565b955095509550505050613f51565b505081515f91506002905b9250925092565b5f826003811115613f6b57613f6b614e0f565b03613f74575050565b6001826003811115613f8857613f88614e0f565b03613fa65760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115613fba57613fba614e0f565b03613fdb5760405163fce698f760e01b8152600481018290526024016104de565b6003826003811115613fef57613fef614e0f565b03610d43576040516335e2f38360e21b8152600481018290526024016104de565b5f614019612771565b54600160401b900460ff16919050565b806001600160a01b03163b5f0361405e57604051634c9c8ce360e01b81526001600160a01b03821660048201526024016104de565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516140c891906157bd565b5f60405180830381855af49150503d805f8114614100576040519150601f19603f3d011682016040523d82523d5f602084013e614105565b606091505b509150915061411585838361430e565b95945050505050565b3415610b9a5760405163b398979f60e01b815260040160405180910390fd5b5f807f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61416861436a565b6141706143e5565b6040805160208101949094528301919091526060820152608081018590523060a082015260c001604051602081830303815290604052805190602001209050610c60818460405161190160f01b8152600281019290925260228201526042902090565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6141fd61436a565b6142056143e5565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561427f57505f91506003905082614304565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa1580156142d0573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b0381166142fb57505f925060019150829050614304565b92505f91508190505b9450945094915050565b6060826143235761431e8261443a565b613a27565b815115801561433a57506001600160a01b0384163b155b1561436357604051639996b31560e01b81526001600160a01b03851660048201526024016104de565b5092915050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10081614395612be1565b8051909150156143ad57805160209091012092915050565b815480156143bc579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10081614410612cb4565b80519091501561442857805160209091012092915050565b600182015480156143bc579392505050565b80511561444957805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b828054828255905f5260205f2090810192821561449b579160200282015b8281111561449b578251825591602001919060010190614480565b506144a792915061453b565b5090565b6040518060c001604052805f6001600160a01b0316815260200160608152602001606081526020016144ee60405180604001604052805f81526020015f81525090565b815260200160608152602001606081525090565b828054828255905f5260205f2090810192821561449b579160200282015b8281111561449b578235825591602001919060010190614520565b5b808211156144a7575f815560010161453c565b5f8083601f84011261455f575f80fd5b50813567ffffffffffffffff811115614576575f80fd5b60208301915083602082850101111561458d575f80fd5b9250929050565b5f805f805f805f6080888a0312156145aa575f80fd5b87359650602088013567ffffffffffffffff808211156145c8575f80fd5b6145d48b838c0161454f565b909850965060408a01359150808211156145ec575f80fd5b6145f88b838c0161454f565b909650945060608a0135915080821115614610575f80fd5b5061461d8a828b0161454f565b989b979a50959850939692959293505050565b5f60208284031215614640575f80fd5b5035919050565b602080825282518282018190525f9190848201906040850190845b818110156146875783516001600160a01b031683529284019291840191600101614662565b50909695505050505050565b5f5b838110156146ad578181015183820152602001614695565b50505f910152565b5f81518084526146cc816020860160208601614693565b601f01601f19169290920160200192915050565b602081525f613a2760208301846146b5565b5f8083601f840112614702575f80fd5b50813567ffffffffffffffff811115614719575f80fd5b6020830191508360208260051b850101111561458d575f80fd5b5f805f8060408587031215614746575f80fd5b843567ffffffffffffffff8082111561475d575f80fd5b614769888389016146f2565b90965094506020870135915080821115614781575f80fd5b5061478e8782880161454f565b95989497509550505050565b5f8083601f8401126147aa575f80fd5b50813567ffffffffffffffff8111156147c1575f80fd5b60208301915083602060608302850101111561458d575f80fd5b5f805f80604085870312156147ee575f80fd5b843567ffffffffffffffff80821115614805575f80fd5b6147698883890161479a565b6001600160a01b0381168114612983575f80fd5b8035613b5281614811565b634e487b7160e01b5f52604160045260245ffd5b6040516080810167ffffffffffffffff8111828210171561486757614867614830565b60405290565b604051601f8201601f1916810167ffffffffffffffff8111828210171561489657614896614830565b604052919050565b5f67ffffffffffffffff8211156148b7576148b7614830565b50601f01601f191660200190565b5f80604083850312156148d6575f80fd5b82356148e181614811565b9150602083013567ffffffffffffffff8111156148fc575f80fd5b8301601f8101851361490c575f80fd5b803561491f61491a8261489e565b61486d565b818152866020838501011115614933575f80fd5b816020840160208301375f602083830101528093505050509250929050565b5f8083601f840112614962575f80fd5b50813567ffffffffffffffff811115614979575f80fd5b6020830191508360208260061b850101111561458d575f80fd5b5f805f80604085870312156149a6575f80fd5b843567ffffffffffffffff808211156149bd575f80fd5b61476988838901614952565b60ff60f81b881681525f602060e060208401526149e960e084018a6146b5565b83810360408501526149fb818a6146b5565b606085018990526001600160a01b038816608086015260a0850187905284810360c0860152855180825260208088019350909101905f5b81811015614a4e57835183529284019291840191600101614a32565b50909c9b505050505050505050505050565b5f60408284031215614a70575f80fd5b50919050565b5f805f805f805f805f805f6101208c8e031215614a91575f80fd5b67ffffffffffffffff808d351115614aa7575f80fd5b614ab48e8e358f01614952565b909c509a50614ac68e60208f01614a60565b9950614ad58e60608f01614a60565b98508060a08e01351115614ae7575f80fd5b614af78e60a08f01358f01614a60565b97508060c08e01351115614b09575f80fd5b614b198e60c08f01358f0161454f565b909750955060e08d0135811015614b2e575f80fd5b614b3e8e60e08f01358f0161454f565b90955093506101008d0135811015614b54575f80fd5b50614b668d6101008e01358e0161454f565b81935080925050509295989b509295989b9093969950565b5f805f805f805f805f805f806101008d8f031215614b9a575f80fd5b67ffffffffffffffff8d351115614baf575f80fd5b614bbc8e8e358f0161479a565b909c509a50614bcd60208e01614825565b995067ffffffffffffffff60408e01351115614be7575f80fd5b614bf78e60408f01358f0161454f565b909950975067ffffffffffffffff60608e01351115614c14575f80fd5b614c248e60608f01358f016146f2565b9097509550614c368e60808f01614a60565b945067ffffffffffffffff60c08e01351115614c50575f80fd5b614c608e60c08f01358f0161454f565b909450925067ffffffffffffffff60e08e01351115614c7d575f80fd5b614c8d8e60e08f01358f0161454f565b81935080925050509295989b509295989b509295989b565b5f805f805f805f805f805f6101008c8e031215614cc0575f80fd5b67ffffffffffffffff808d351115614cd6575f80fd5b614ce38e8e358f01614952565b909c509a50614cf58e60208f01614a60565b99508060608e01351115614d07575f80fd5b614d178e60608f01358f01614a60565b9850614d2560808e01614825565b97508060a08e01351115614d37575f80fd5b614d478e60a08f01358f0161454f565b909750955060c08d0135811015614d5c575f80fd5b614d6c8e60c08f01358f0161454f565b909550935060e08d0135811015614d81575f80fd5b50614b668d60e08e01358e0161454f565b5f805f805f60608688031215614da6575f80fd5b8535614db181614811565b9450602086013567ffffffffffffffff80821115614dcd575f80fd5b614dd989838a01614952565b90965094506040880135915080821115614df1575f80fd5b50614dfe8882890161454f565b969995985093965092949392505050565b634e487b7160e01b5f52602160045260245ffd5b600181811c90821680614e3757607f821691505b602082108103614a7057634e487b7160e01b5f52602260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561237c5761237c614e55565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b878152608060208201525f614ebd60808301888a614e7c565b8281036040840152614ed0818789614e7c565b90508281036060840152614ee5818587614e7c565b9a9950505050505050505050565b5f8551614f04818460208a01614693565b61103b60f11b9083019081528551614f23816002840160208a01614693565b808201915050601760f91b8060028301528551614f47816003850160208a01614693565b60039201918201528351614f62816004840160208801614693565b016004019695505050505050565b5f60208284031215614f80575f80fd5b8151613a2781614811565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614faf575f80fd5b81518015158114613a27575f80fd5b601f821115612a5557805f5260205f20601f840160051c81016020851015614fe35750805b601f840160051c820191505b818110156132a0575f8155600101614fef565b67ffffffffffffffff83111561501a5761501a614830565b61502e836150288354614e23565b83614fbe565b5f601f84116001811461505f575f85156150485750838201355b5f19600387901b1c1916600186901b1783556132a0565b5f83815260208120601f198716915b8281101561508e578685013582556020948501946001909201910161506e565b50868210156150aa575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b608081525f6150cf60808301898b614e7c565b82810360208401526150e281888a614e7c565b90506001600160a01b03861660408401528281036060840152614ee5818587614e7c565b606081525f615119606083018789614e7c565b60208382038185015281875480845282840191506005838260051b8601018a5f52845f205f5b848110156151d257601f198884030186525f825461515c81614e23565b808652600182811680156151775760018114615190576151bb565b60ff198416888d0152821515891b88018c0194506151bb565b865f528b5f205f5b848110156151b35781548a82018f0152908301908d01615198565b89018d019550505b50988a01989295505050919091019060010161513f565b505087810360408901526151e7818a8c614e7c565b9d9c50505050505050505050505050565b5f808335601e1984360301811261520d575f80fd5b83018035915067ffffffffffffffff821115615227575f80fd5b6020019150600581901b360382131561458d575f80fd5b5f6040828403121561524e575f80fd5b6040516040810181811067ffffffffffffffff8211171561527157615271614830565b604052823581526020928301359281019290925250919050565b5f6040828403121561529b575f80fd5b613a27838361523e565b5f602082840312156152b5575f80fd5b8135613a2781614811565b6001600160a01b038481168252604060208084018290529083018490525f91859160608501845b8781101561530e5784356152fa81614811565b8416825293820193908201906001016152e7565b5098975050505050505050565b5f815180845260208085019450602084015f5b8381101561534a5781518752958201959082019060010161532e565b509495945050505050565b602081525f613a27602083018461531b565b5f67ffffffffffffffff82111561538057615380614830565b5060051b60200190565b5f602080838503121561539b575f80fd5b825167ffffffffffffffff808211156153b2575f80fd5b818501915085601f8301126153c5575f80fd5b81516153d361491a82615367565b81815260059190911b830184019084810190888311156153f1575f80fd5b8585015b8381101561530e5780518581111561540b575f80fd5b86016080818c03601f19011215615420575f80fd5b615428614844565b8882015181526040808301518a830152606083015181830152608083015188811115615452575f80fd5b8084019350508c603f840112615466575f80fd5b8983015161547661491a82615367565b81815260059190911b84018201908b8101908f831115615494575f80fd5b948301945b828610156154be57855193506154ae84614811565b838252948c0194908c0190615499565b60608501525050508452509186019186016153f5565b5f600182016154e5576154e5614e55565b5060010190565b815167ffffffffffffffff81111561550657615506614830565b61551a816155148454614e23565b84614fbe565b602080601f83116001811461554d575f84156155365750858301515b5f19600386901b1c1916600185901b1785556155a4565b5f85815260208120601f198616915b8281101561557b5788860151825594840194600190910190840161555c565b508582101561559857878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b5f815180845260208085019450602084015f5b8381101561534a5781516001600160a01b0316875295820195908201906001016155bf565b8051825260208101516020830152604081015160408301525f606082015160806060850152610c6060808501826155ac565b5f8282518085526020808601955060208260051b840101602086015f5b8481101561566157601f1986840301895261564f8383516155e4565b98840198925090830190600101615633565b5090979650505050505050565b608081525f6156806080830189615616565b6001600160a01b038816602084015282810360408401526156a2818789614e7c565b905082810360608401526156b7818587614e7c565b9998505050505050505050565b608081525f615680608083018961531b565b8183525f7f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115615706575f80fd5b8260051b80836020870137939093016020019392505050565b602081525f610c606020830184866156d6565b604081525f6157446040830186615616565b828103602084015261229e818587614e7c565b604081525f61576a6040830186886156d6565b828103602084015261577d818587614e7c565b979650505050505050565b81515f9082906020808601845b838110156157b157815185529382019390820190600101615795565b50929695505050505050565b5f82516157ce818460208701614693565b9190910192915050565b5f602082840312156157e8575f80fd5b5051919050565b5f80858511156157fd575f80fd5b83861115615809575f80fd5b5050820193919092039150565b8035602083101561237c575f19602084900360031b1b1692915050565b808202811582820484141761237c5761237c614e55565b8082018082111561237c5761237c614e55565b82815260608101613a27602083018480518252602090810151910152565b602081525f610c60602083018486614e7c565b604081525f6158a060408301856155e4565b828103602084015261411581856155e4565b8183525f60208085019450825f5b8581101561534a5781358752828201356158d981614811565b6001600160a01b0390811688850152604090838201356158f881614811565b169088015260609687019691909101906001016158c0565b6001600160a01b0381511682525f602082015160e0602085015261593760e08501826146b5565b90506040830151848203604086015261595082826155ac565b915050606083015161596f606086018280518252602090810151910152565b50608083015184820360a086015261598782826146b5565b91505060a083015184820360c086015261411582826146b5565b606081525f6159b36060830187615616565b82810360208401526159c68186886158b2565b9050828103604084015261577d8185615910565b604081525f6159ed6040830185876158b2565b828103602084015261229e8185615910565b5f82601f830112615a0e575f80fd5b8151615a1c61491a8261489e565b818152846020838601011115615a30575f80fd5b610c60826020830160208701614693565b5f60208284031215615a51575f80fd5b815167ffffffffffffffff80821115615a68575f80fd5b9083019060808286031215615a7b575f80fd5b615a83614844565b8251615a8e81614811565b81526020830151615a9e81614811565b6020820152604083015182811115615ab4575f80fd5b615ac0878286016159ff565b604083015250606083015182811115615ad7575f80fd5b615ae3878286016159ff565b60608301525095945050505050565b6020810160548310615b1257634e487b7160e01b5f52602160045260245ffd5b91905290565b81515f9082906020808601845b838110156157b15781516001600160a01b031685529382019390820190600101615b2556fe5573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c6279746573207573657244656372797074656453686172652c627974657320657874726144617461295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c742c62797465732065787472614461746129557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c6279746573206578747261446174612944656c656761746564557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c616464726573732064656c656761746f72416464726573732c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c6279746573206578747261446174612968113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80Qa]\\b\0\x01\0_9_\x81\x81a(0\x01R\x81\x81a(Y\x01Ra*e\x01Ra]\\_\xF3\xFE`\x80`@R`\x046\x10a\x01xW_5`\xE0\x1C\x80co\x89\x13\xBC\x11a\0\xD1W\x80c\xB4\xDE,7\x11a\0|W\x80c\xF1\xB5z\xDB\x11a\0WW\x80c\xF1\xB5z\xDB\x14a\x04KW\x80c\xFA!\x06\xB8\x14a\x04jW\x80c\xFB\xB82Y\x14a\x04~W_\x80\xFD[\x80c\xB4\xDE,7\x14a\x03\xEEW\x80c\xD8\x99\x8FE\x14a\x04\rW\x80c\xE2-\x1B&\x14a\x04,W_\x80\xFD[\x80c\x84\xB0\x19n\x11a\0\xACW\x80c\x84\xB0\x19n\x14a\x03`W\x80c\x9F\xADZ/\x14a\x03\x87W\x80c\xAD<\xB1\xCC\x14a\x03\xA6W_\x80\xFD[\x80co\x89\x13\xBC\x14a\x03\x0EW\x80cv\"~\xED\x14a\x03-W\x80c\x84V\xCBY\x14a\x03LW_\x80\xFD[\x80c@\x14\xC4\xCD\x11a\x011W\x80cR\xD1\x90-\x11a\x01\x0CW\x80cR\xD1\x90-\x14a\x02|W\x80cX\xF5\xB8\xAB\x14a\x02\x9EW\x80c\\\x97Z\xBB\x14a\x02\xD8W_\x80\xFD[\x80c@\x14\xC4\xCD\x14a\x02\x1BW\x80cA\x0B\xF0\xBA\x14a\x02JW\x80cO\x1E\xF2\x86\x14a\x02iW_\x80\xFD[\x80c\r\x8En,\x11a\x01aW\x80c\r\x8En,\x14a\x01\xD2W\x80c9\xF78\x10\x14a\x01\xF3W\x80c?K\xA8:\x14a\x02\x07W_\x80\xFD[\x80c\x04o\x9E\xB3\x14a\x01|W\x80c\t\0\xCCi\x14a\x01\x9DW[_\x80\xFD[4\x80\x15a\x01\x87W_\x80\xFD[Pa\x01\x9Ba\x01\x966`\x04aE\x94V[a\x04\x9DV[\0[4\x80\x15a\x01\xA8W_\x80\xFD[Pa\x01\xBCa\x01\xB76`\x04aF0V[a\x08\x06V[`@Qa\x01\xC9\x91\x90aFGV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDDW_\x80\xFD[Pa\x01\xE6a\x08\xD2V[`@Qa\x01\xC9\x91\x90aF\xE0V[4\x80\x15a\x01\xFEW_\x80\xFD[Pa\x01\x9Ba\t:V[4\x80\x15a\x02\x12W_\x80\xFD[Pa\x01\x9Ba\n\xC9V[4\x80\x15a\x02&W_\x80\xFD[Pa\x02:a\x0256`\x04aG3V[a\x0B\x9CV[`@Q\x90\x15\x15\x81R` \x01a\x01\xC9V[4\x80\x15a\x02UW_\x80\xFD[Pa\x02:a\x02d6`\x04aG\xDBV[a\x0ChV[a\x01\x9Ba\x02w6`\x04aH\xC5V[a\r(V[4\x80\x15a\x02\x87W_\x80\xFD[Pa\x02\x90a\rGV[`@Q\x90\x81R` \x01a\x01\xC9V[4\x80\x15a\x02\xA9W_\x80\xFD[Pa\x02:a\x02\xB86`\x04aF0V[_\x90\x81R_\x80Q` a]<\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x90V[4\x80\x15a\x02\xE3W_\x80\xFD[P\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x16a\x02:V[4\x80\x15a\x03\x19W_\x80\xFD[Pa\x01\x9Ba\x03(6`\x04aE\x94V[a\ruV[4\x80\x15a\x038W_\x80\xFD[Pa\x02:a\x03G6`\x04aI\x93V[a\x10\x8BV[4\x80\x15a\x03WW_\x80\xFD[Pa\x01\x9Ba\x11KV[4\x80\x15a\x03kW_\x80\xFD[Pa\x03ta\x12\x05V[`@Qa\x01\xC9\x97\x96\x95\x94\x93\x92\x91\x90aI\xC9V[4\x80\x15a\x03\x92W_\x80\xFD[Pa\x01\x9Ba\x03\xA16`\x04aJvV[a\x12\xC9V[4\x80\x15a\x03\xB1W_\x80\xFD[Pa\x01\xE6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x03\xF9W_\x80\xFD[Pa\x01\x9Ba\x04\x086`\x04aK~V[a\x18iV[4\x80\x15a\x04\x18W_\x80\xFD[Pa\x01\x9Ba\x04'6`\x04aG3V[a\x1A\x11V[4\x80\x15a\x047W_\x80\xFD[Pa\x02:a\x04F6`\x04aI\x93V[a\x1C\x0FV[4\x80\x15a\x04VW_\x80\xFD[Pa\x01\x9Ba\x04e6`\x04aL\xA5V[a\x1C\xCFV[4\x80\x15a\x04uW_\x80\xFD[Pa\x01\x9Ba!\xE0V[4\x80\x15a\x04\x89W_\x80\xFD[Pa\x02:a\x04\x986`\x04aM\x92V[a\"\x91V[_\x80Q` a]<\x839\x81Q\x91R`\x01`\xF9\x1B\x88\x11\x15\x80a\x04\xC1WP\x80`\x08\x01T\x88\x11[\x15a\x04\xE7W`@QcjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[_\x88\x81R`\x07\x82\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\x05\x0E\x90aN#V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05:\x90aN#V[\x80\x15a\x05\x85W\x80`\x1F\x10a\x05\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x85V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05hW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xDBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xC7W[PPPPP\x81RPP\x90P_`@Q\x80`\x80\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x89\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x87\x81R\x91\x81\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91Pa\x06\x85\x90P\x82a\"\xA8V[_\x8C\x81R`\t\x86\x01` R`@\x81 T\x91\x92Pa\x06\xA2\x88\x88a#\x82V[\x90P\x81_\x03a\x06\xB3W\x80\x91Pa\x06\xE4V[\x81\x81\x14a\x06\xE4W`@QcU\xDA\xFAC`\xE1\x1B\x81R`\x04\x81\x01\x8E\x90R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R`d\x01a\x04\xDEV[Pa\x06\xF2\x81\x8D\x84\x8C\x8Ca%KV[_\x8C\x81R`\x02\x86\x01` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x82 \x80T`\x01\x81\x81\x01\x83U\x82\x85R\x92\x90\x93 \x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x163\x17\x90U\x81T\x8E\x91\x7F\x7F\xCD\xFBS\x81\x91\x7FUJq}\nTp\xA3?ZI\xBAdE\xF0^\xC4<t\xC0\xBC,\xC6\x08\xB2\x91a\x07k\x91\x90aNiV[\x8E\x8E\x8E\x8E\x8E\x8E`@Qa\x07\x84\x97\x96\x95\x94\x93\x92\x91\x90aN\xA4V[`@Q\x80\x91\x03\x90\xA2_\x8D\x81R` \x87\x90R`@\x90 T`\xFF\x16\x15\x80\x15a\x07\xB2WP\x80Ta\x07\xB2\x90\x83\x90a&8V[\x15a\x07\xF7W_\x8D\x81R` \x87\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x8E\x91\x7F\xE8\x97R\xBE\x0E\xCD\xB6\x8B*n\xB5\xEF\x1A\x89\x109\xE0\xE9*\xE3\xC8\xA6\"t\xC5\x88\x1EH\xEE\xA1\xED%\x91\xA2[PPPPPPPPPPPPPV[_\x81\x81R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x03` \x90\x81R`@\x80\x83 T\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02\x83R\x81\x84 \x81\x85R\x83R\x92\x81\x90 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94_\x80Q` a]<\x839\x81Q\x91R\x94\x90\x93\x92\x91\x90\x83\x01\x82\x82\x80\x15a\x08\xC4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xA6W[PPPPP\x92PPP\x91\x90PV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\"2\xB1\xB9<\xB8:4\xB7\xB7`\xB1\x1B\x81RPa\t\0_a&\xBAV[a\t\n`\x07a&\xBAV[a\t\x13_a&\xBAV[`@Q` \x01a\t&\x94\x93\x92\x91\x90aN\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\tBa'XV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14a\tlW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08_a\twa'qV[\x80T\x90\x91P`\x01`@\x1B\x90\x04`\xFF\x16\x80a\t\x9FWP\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x91\x16\x10\x15[\x15a\t\xBDW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"2\xB1\xB9<\xB8:4\xB7\xB7`\xB1\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\n!\x91a'\x99V[a\n)a'\xABV[`\x01`\xF8\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06U`\x01`\xF9\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x81U`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x19W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B=\x91\x90aOpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x0BrWP3s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x14\x15[\x15a\x0B\x92W`@Qcp\xC8\xB3w`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\xDEV[a\x0B\x9Aa'\xB3V[V[_\x83\x81\x03a\x0B\xABWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x0B\xE1Wa\x0B\xE1aO\x8BV[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C!W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CE\x91\x90aO\x9FV[a\x0CRW_\x91PPa\x0C`V[`\x01\x01a\x0B\xADV[P`\x01\x90P[\x94\x93PPPPV[_\x83\x81\x03a\x0CwWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x0C\xADWa\x0C\xADaO\x8BV[\x90P``\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x13\x91\x90aO\x9FV[a\r W_\x91PPa\x0C`V[`\x01\x01a\x0CyV[a\r0a(%V[a\r9\x82a(\xDCV[a\rC\x82\x82a)\x86V[PPV[_a\rPa*ZV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[_\x80Q` a]<\x839\x81Q\x91R`\x01`\xF8\x1B\x88\x11\x15\x80a\r\x99WP\x80`\x06\x01T\x88\x11[\x15a\r\xBAW`@QcjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x01a\x04\xDEV[`@\x80Q_\x8A\x81R`\x05\x84\x01` \x90\x81R\x83\x82 \x80T`\x80\x92\x81\x02\x85\x01\x83\x01\x90\x95R``\x84\x01\x85\x81R\x92\x94\x84\x93\x92\x84\x01\x82\x82\x80\x15a\x0E\x15W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\x01W[PPPPP\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x88\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x86\x81R\x91\x81\x01\x91\x90\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91Pa\x0E\x9F\x90P\x82a*\xA3V[_\x8B\x81R`\t\x85\x01` R`@\x81 T\x91\x92Pa\x0E\xBC\x87\x87a#\x82V[\x90P\x81_\x03a\x0E\xCDW\x80\x91Pa\x0E\xFEV[\x81\x81\x14a\x0E\xFEW`@QcU\xDA\xFAC`\xE1\x1B\x81R`\x04\x81\x01\x8D\x90R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R`d\x01a\x04\xDEV[a\x0F\x0B\x82\x8D\x85\x8C\x8Ca%KV[_\x8C\x81R`\x04\x86\x01` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\x01\x81\x01\x82U\x81\x84R\x91\x90\x92 \x01a\x0F?\x8A\x8C\x83aP\x02V[P\x85`\x02\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x85\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x8C\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x8D\x8D\x8D\x8D3\x8E\x8E`@Qa\x0F\xE2\x97\x96\x95\x94\x93\x92\x91\x90aP\xBCV[`@Q\x80\x91\x03\x90\xA2_\x8D\x81R` \x87\x90R`@\x90 T`\xFF\x16\x15\x80\x15a\x10\x10WP\x80Ta\x10\x10\x90\x84\x90a+JV[\x15a\x07\xF7W_\x8D\x81R` \x87\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x03\x89\x01\x90\x91R\x90\x81\x90 \x85\x90UQ\x8D\x90\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x90a\x10t\x90\x8F\x90\x8F\x90\x86\x90\x8E\x90\x8E\x90aQ\x06V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPV[_\x83\x81\x03a\x10\x9AWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x10\xD0Wa\x10\xD0aO\x8BV[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xF7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x12W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x116\x91\x90aO\x9FV[a\x11CW_\x91PPa\x0C`V[`\x01\x01a\x10\x9CV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x98W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xBC\x91\x90aO\x9FV[\x15\x80\x15a\x11\xDDWP3s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x14\x15[\x15a\x11\xFDW`@Qc8\x89\x16\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\xDEV[a\x0B\x9Aa+\x86V[_``\x80\x82\x80\x80\x83\x81\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x80T\x90\x91P\x15\x80\x15a\x12CWP`\x01\x81\x01T\x15[a\x12\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\xDEV[a\x12\x97a+\xE1V[a\x12\x9Fa,\xB4V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x91\x99PF\x98P0\x97P\x95P\x93P\x91PPV[a\x12\xD1a-\x05V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R\x875`\x04\x82\x01\x81\x90R\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\xF3\xAA\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13F\x91\x90aO\x9FV[a\x13fW`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`@Qcfb\x86\xDD`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xCC\xC5\r\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD8\x91\x90aO\x9FV[\x15a\x13\xF9W`@Qc\x18\r\x9A1`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[a\x14\x06` \x89\x01\x89aQ\xF8V[\x90P_\x03a\x14'W`@QcW\xCF\xA2\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\na\x146` \x8A\x01\x8AaQ\xF8V[\x90P\x11\x15a\x14rW`\na\x14M` \x8A\x01\x8AaQ\xF8V[`@Qc\xAF\x1F\x04\x95`\xE0\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01R`$\x83\x01RP`D\x01a\x04\xDEV[a\x14\x89a\x14\x846\x8C\x90\x03\x8C\x01\x8CaR\x8BV[a-HV[a\x14\xDCa\x14\x99` \x8A\x01\x8AaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x14\xD7\x92PPP` \x8C\x01\x8CaR\xA5V[a.\x14V[\x15a\x15\x17Wa\x14\xEE` \x8A\x01\x8AaR\xA5V[a\x14\xFB` \x8A\x01\x8AaQ\xF8V[`@Qc\xC3Dj\xC7`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x93\x92\x91\x90aR\xC0V[_a\x15#\x8D\x8D\x8Ba.mV[\x90P_`@Q\x80`\xC0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x15{\x90\x8D\x01\x8DaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x15\xC0\x90\x8E\x01\x8EaR\xA5V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D_\x015\x81R` \x01\x8D` \x015\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x90Pa\x167\x81a\x16.`@\x8E\x01` \x8F\x01aR\xA5V[\x89\x89\x8E5a0dV[P`@Qc\xA1O\x89q`\xE0\x1B\x81R_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a\x16q\x90\x85\x90`\x04\x01aSUV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\xB2\x91\x90\x81\x01\x90aS\x8AV[\x90Pa\x16\xBD\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a\x16\xF9\x83aT\xD4V[\x90\x91UPP`\x08\x81\x01T`@\x80Q``` `\x1F\x8E\x01\x81\x90\x04\x02\x82\x01\x81\x01\x83R\x91\x81\x01\x8C\x81R\x90\x91\x82\x91\x90\x8E\x90\x8E\x90\x81\x90\x85\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP` \x91\x82\x01\x87\x90R\x83\x81R`\x07\x85\x01\x90\x91R`@\x90 \x81Q\x81\x90a\x17c\x90\x82aT\xECV[P` \x82\x81\x01Q\x80Qa\x17|\x92`\x01\x85\x01\x92\x01\x90aDbV[P\x90PP_a\x17\x8B\x88\x88a#\x82V[\x90Pa\x17\x96\x81a1\xA9V[_\x82\x81R`\t\x84\x01` R`@\x90 Ua\x17\xAF3a2;V[\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8F` \x01` \x81\x01\x90a\x17\xE5\x91\x90aR\xA5V[\x8E\x8E\x8C\x8C`@Qa\x17\xFB\x96\x95\x94\x93\x92\x91\x90aVnV[`@Q\x80\x91\x03\x90\xA2\x80\x7F\xC7\x1E2\xC8\x0F1\t\xC6s\xA9\xAD\xFC\xDE\xD0\xB3\xB2\t2\x12\xAB(@\x84\xA0\xE0:(\x1D\xEE+\xDD_\x85\x8F` \x01` \x81\x01\x90a\x189\x91\x90aR\xA5V[\x8E\x8E\x8C\x8C`@Qa\x18O\x96\x95\x94\x93\x92\x91\x90aV\xC4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[a\x18qa-\x05V[_\x8B\x90\x03a\x18\x92W`@Qc$\x0E\x93\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x86\x11\x15a\x18\xBEW`@Qc\xAF\x1F\x04\x95`\xE0\x1B\x81R`\n`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x04\xDEV[a\x18\xD5a\x18\xD06\x87\x90\x03\x87\x01\x87aR\x8BV[a2\xA7V[a\x18\xDDaD\xABV[`\x01`\x01`\xA0\x1B\x03\x8B\x16\x81R`@\x80Q` `\x1F\x8C\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8A\x81R\x90\x8B\x90\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP` \x80\x83\x01\x91\x90\x91R`@\x80Q\x89\x83\x02\x81\x81\x01\x84\x01\x90\x92R\x89\x81R\x91\x8A\x91\x8A\x91\x82\x91\x90\x85\x01\x90\x84\x90\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP`@\x82\x01Ra\x19g6\x87\x90\x03\x87\x01\x87aR\x8BV[``\x82\x01R`@\x80Q` `\x1F\x85\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP`\x80\x82\x01R`@\x80Q` `\x1F\x87\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x85\x81R\x90\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP`\xA0\x86\x01\x94\x90\x94RPa\x19\xEA\x91P\x85\x90P\x84a#\x82V[\x90Pa\x19\xF53a2;V[a\x1A\x01\x8E\x8E\x84\x84a3~V[PPPPPPPPPPPPPPV[a\x1A\x19a-\x05V[_\x83\x90\x03a\x1A:W`@Qc\x05\xBC\xEA\x87`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Au\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5?\x92PPPV[`@Qc\xA1O\x89q`\xE0\x1B\x81R_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a\x1A\xB0\x90\x88\x90\x88\x90`\x04\x01aW\x1FV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xCAW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\xF1\x91\x90\x81\x01\x90aS\x8AV[\x90Pa\x1A\xFC\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a\x1B8\x83aT\xD4V[\x90\x91UPP`\x06\x81\x01T_\x81\x81R`\x05\x83\x01` R`@\x90 a\x1B\\\x90\x88\x88aE\x02V[P_a\x1Bh\x86\x86a#\x82V[\x90Pa\x1Bs\x81a1\xA9V[_\x82\x81R`\t\x84\x01` R`@\x90 Ua\x1B\x8C3a5\xC6V[\x80\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x84\x87\x87`@Qa\x1B\xC0\x93\x92\x91\x90aW2V[`@Q\x80\x91\x03\x90\xA2\x80\x7F\x9B\xB0\x88\xA5?\x1F\xE1>\xC8O\xA0\xA1\xD7\xB5x\xB5\x89\xE1\x93&\x8B\xAA\xCC\xC5\xE2?z\xB7\x85\xAA=\xAE\x88\x88\x88\x88`@Qa\x1B\xFE\x94\x93\x92\x91\x90aWWV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_\x83\x81\x03a\x1C\x1EWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x1CTWa\x1CTaO\x8BV[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C{\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x96W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xBA\x91\x90aO\x9FV[a\x1C\xC7W_\x91PPa\x0C`V[`\x01\x01a\x1C V[a\x1C\xD7a-\x05V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R\x885`\x04\x82\x01\x81\x90R\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\xF3\xAA\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DL\x91\x90aO\x9FV[a\x1DlW`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`@Qcfb\x86\xDD`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xCC\xC5\r\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xBAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xDE\x91\x90aO\x9FV[\x15a\x1D\xFFW`@Qc\x18\r\x9A1`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[a\x1E\x0C` \x8A\x01\x8AaQ\xF8V[\x90P_\x03a\x1E-W`@QcW\xCF\xA2\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\na\x1E<` \x8B\x01\x8BaQ\xF8V[\x90P\x11\x15a\x1ESW`\na\x14M` \x8B\x01\x8BaQ\xF8V[a\x1Eea\x14\x846\x8C\x90\x03\x8C\x01\x8CaR\x8BV[a\x1E\xADa\x1Eu` \x8B\x01\x8BaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8C\x92Pa.\x14\x91PPV[\x15a\x1E\xDCW\x87a\x1E\xC0` \x8B\x01\x8BaQ\xF8V[`@Qc\xDCMx\xB1`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x93\x92\x91\x90aR\xC0V[_a\x1E\xE8\x8D\x8D\x8Ca.mV[\x90P_`@Q\x80`\xA0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x1F@\x90\x8E\x01\x8EaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP\x8D5` \x80\x83\x01\x91\x90\x91R\x8E\x81\x015`@\x80\x84\x01\x91\x90\x91R\x80Q`\x1F\x89\x01\x83\x90\x04\x83\x02\x81\x01\x83\x01\x90\x91R\x87\x81R``\x90\x92\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x90Pa\x1F\xD2\x81\x8B\x89\x89\x8F5a6\x06V[`@Qc\xA1O\x89q`\xE0\x1B\x81R_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a \x0B\x90\x86\x90`\x04\x01aSUV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a %W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra L\x91\x90\x81\x01\x90aS\x8AV[\x90Pa W\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a \x93\x83aT\xD4V[\x90\x91UPP`\x08\x81\x01T`@\x80Q``` `\x1F\x8F\x01\x81\x90\x04\x02\x82\x01\x81\x01\x83R\x91\x81\x01\x8D\x81R\x90\x91\x82\x91\x90\x8F\x90\x8F\x90\x81\x90\x85\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP` \x91\x82\x01\x88\x90R\x83\x81R`\x07\x85\x01\x90\x91R`@\x90 \x81Q\x81\x90a \xFD\x90\x82aT\xECV[P` \x82\x81\x01Q\x80Qa!\x16\x92`\x01\x85\x01\x92\x01\x90aDbV[P\x90PP_a!%\x89\x89a#\x82V[\x90Pa!0\x81a1\xA9V[_\x82\x81R`\t\x84\x01` R`@\x90 Ua!I3a2;V[\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8F\x8F\x8F\x8D\x8D`@Qa!\x83\x96\x95\x94\x93\x92\x91\x90aVnV[`@Q\x80\x91\x03\x90\xA2\x80\x7F\xC7\x1E2\xC8\x0F1\t\xC6s\xA9\xAD\xFC\xDE\xD0\xB3\xB2\t2\x12\xAB(@\x84\xA0\xE0:(\x1D\xEE+\xDD_\x86\x8F\x8F\x8F\x8D\x8D`@Qa!\xC5\x96\x95\x94\x93\x92\x91\x90aV\xC4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPPV[`\x08_a!\xEBa'qV[\x80T\x90\x91P`\x01`@\x1B\x90\x04`\xFF\x16\x80a\"\x13WP\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x91\x16\x10\x15[\x15a\"1W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17`\x01`@\x1B\x17h\xFF\0\0\0\0\0\0\0\0\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\n\xBDV[_a\"\x9E\x85\x85\x85\x85a\x1C\x0FV[\x96\x95PPPPPPV[_a#|`@Q\x80`\xA0\x01`@R\x80`m\x81R` \x01a[K`m\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a\"\xEC\x91\x90aW\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 \x86``\x01Q`@Q` \x01a##\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x96\x90\x96R\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6\x11V[\x92\x91PPV[_\x81\x81\x03a$\x05Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xFE\x91\x90aW\xD8V[\x90Pa#|V[_\x83\x83_\x81\x81\x10a$\x18Wa$\x18aO\x8BV[\x91\x90\x91\x015`\xF8\x1C\x91PP_\x81\x90\x03a$\xA7Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a${W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x9F\x91\x90aW\xD8V[\x91PPa#|V[\x80`\xFF\x16`\x01\x14\x80a$\xBCWP\x80`\xFF\x16`\x02\x14[\x15a%-W`!\x83\x10\x15a$\xEDW`@QcI\xAAE3`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`!`$\x82\x01R`D\x01a\x04\xDEV[a$\xFB`!`\x01\x85\x87aW\xEFV[a%\x04\x91aX\x16V[\x91P_\x82\x90\x03a%'W`@Qc\xCB\x17\xB7\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa#|V[`@Qc\x08Ns\x0B`\xE2\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x04\xDEV[__\x80Q` a]<\x839\x81Q\x91R\x90P_a%\x9C\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa6=\x92PPPV[\x90Pa%\xA9\x87\x823a6eV[_\x86\x81R`\x01\x83\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a&\0W`@Qc\x99\xECH\xD9`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x04\xDEV[_\x95\x86R`\x01\x91\x82\x01` \x90\x81R`@\x80\x88 `\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x88R\x91\x90R\x90\x94 \x80T`\xFF\x19\x16\x90\x94\x17\x90\x93UPPPPV[`@Qc\x14\x0FE\xFF`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R_\x90\x81\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c(\x1E\x8B\xFE\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x8BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xAF\x91\x90aW\xD8V[\x90\x92\x10\x15\x93\x92PPPV[``_a&\xC6\x83a7\xD3V[`\x01\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xE5Wa&\xE5aH0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a'\x0FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a'\x19W[P\x93\x92PPPV[_a'aa'qV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90PV[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a#|V[a'\xA1a8\xB4V[a\rC\x82\x82a8\xD9V[a\x0B\x9Aa8\xB4V[a'\xBBa9KV[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x80T`\xFF\x19\x16\x81U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a(\xBEWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(\xB2\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0B\x9AW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a),W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)P\x91\x90aOpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a)\x83W`@Qc\x0EV\xCF=`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\xDEV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a)\xE0WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra)\xDD\x91\x81\x01\x90aW\xD8V[`\x01[a*\x08W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\xDEV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a*KW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[a*U\x83\x83a9\x8DV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B\x9AW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a#|`@Q\x80`\x80\x01`@R\x80`T\x81R` \x01a[\xB8`T\x919\x80Q` \x91\x82\x01 \x84Q`@Q\x91\x92a*\xD9\x92\x01aW\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 \x85`@\x01Q`@Q` \x01a+\x10\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x95\x90\x95R\x81\x01\x92\x90\x92R``\x82\x01R`\x80\x81\x01\x91\x90\x91R`\xA0\x01a#aV[`@Qca\xD5U-`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R_\x90\x81\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xC3\xAA\xAAZ\x90`$\x01a&pV[a+\x8Ea-\x05V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x80T`\xFF\x19\x16`\x01\x17\x81U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X3a(\x07V[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x80T``\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x91a,2\x90aN#V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,^\x90aN#V[\x80\x15a,\xA9W\x80`\x1F\x10a,\x80Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a,\xA9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x80T``\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x91a,2\x90aN#V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x16\x15a\x0B\x9AW`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q_\x03a-lW`@Qc\xDE(Y\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x01m\x10\x15a-\xA4W` \x81\x01Q`@Qc2\x95\x18c`\xE0\x1B\x81Ra\x01m`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x04\xDEV[\x80QB\x10\x15a-\xD2W\x80Q`@Qc\xF2L\x08\x87`\xE0\x1B\x81RB`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x04\xDEV[B\x81` \x01Qb\x01Q\x80a-\xE6\x91\x90aX3V[\x82Qa-\xF2\x91\x90aXJV[\x10\x15a)\x83WB\x81`@Qb\xC0\xD2\x01`\xE6\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX]V[_\x80[\x83Q\x81\x10\x15a.dW\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a.<Wa.<aO\x8BV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a.\\W`\x01\x91PPa#|V[`\x01\x01a.\x17V[P_\x93\x92PPPV[``_\x83\x90\x03a.\x90W`@Qc\xA6\xA6\xCB!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xA9Wa.\xA9aH0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a.\xD2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x84\x81\x10\x15a05W_\x86\x86\x83\x81\x81\x10a.\xF3Wa.\xF3aO\x8BV[\x90P`@\x02\x01_\x015\x90P_\x87\x87\x84\x81\x81\x10a/\x11Wa/\x11aO\x8BV[\x90P`@\x02\x01` \x01` \x81\x01\x90a/)\x91\x90aR\xA5V[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x10\x83\x90\x1C\x16\x865\x81\x14a/lW`@QcJ\xC8t\x8B`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R\x875`D\x82\x01R`d\x01a\x04\xDEV[_a/v\x84a9\xE2V[\x90Pa/\x81\x81a:.V[a/\x8F\x90a\xFF\xFF\x16\x87aXJV[\x95Pa/\xD9a/\xA1` \x8A\x01\x8AaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa.\x14\x91PPV[a0\x07W\x82a/\xEB` \x8A\x01\x8AaQ\xF8V[`@Qc\xA4\xC3\x03\x91`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x93\x92\x91\x90aR\xC0V[\x83\x87\x86\x81Q\x81\x10a0\x1AWa0\x1AaO\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x90\x92\x01\x91Pa.\xD8\x90PV[Pa\x08\0\x81\x11\x15a'PW`@Qc\xE7\xF4\x89]`\xE0\x1B\x81Ra\x08\0`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x04\xDEV[_a0o\x86\x83a;WV[\x90P_a0\xB1\x82\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa6=\x92PPPV[\x90P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a0\xE9W\x84\x84`@Qc*\x87='`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX{V[PPPPPPPV[`\x01\x81Q\x11a0\xFEWPV[_\x81_\x81Q\x81\x10a1\x11Wa1\x11aO\x8BV[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a*UW\x81\x83\x82\x81Q\x81\x10a1AWa1AaO\x8BV[` \x02` \x01\x01Q` \x01Q\x14a1\xA1W\x82_\x81Q\x81\x10a1dWa1daO\x8BV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a1~Wa1~aO\x8BV[` \x02` \x01\x01Q`@Qc\xCF\xAE\x92\x1F`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX\x8EV[`\x01\x01a1%V[`@Qc\x17\xF3b\xD9`\xE3\x1B\x81R`\x04\x81\x01\x82\x90Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\x9B\x16\xC8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xF7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x1B\x91\x90aO\x9FV[a)\x83W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`@Qc\x98\x8A--`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\x81z(_\x1F\xCA;\xB4\x08L\xBF\xC7}K\xAB\xC28\xAD`\x9C\x90c\x98\x8A--\x90`$\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\x8EW_\x80\xFD[PZ\xF1\x15\x80\x15a2\xA0W=_\x80>=_\xFD[PPPPPV[\x80` \x01Q_\x03a2\xCBW`@Qc\x12)\xE27`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2\xDAa\x01mb\x01Q\x80aX3V[\x81` \x01Q\x11\x15a3\x1BWa2\xF4a\x01mb\x01Q\x80aX3V[` \x82\x01Q`@QcW)u\x89`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04\xDEV[\x80QB\x10\x15a3IW\x80Q`@Qc\xF2L\x08\x87`\xE0\x1B\x81RB`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x04\xDEV[` \x81\x01Q\x81QB\x91a3[\x91aXJV[\x10\x15a)\x83WB\x81`@Qc3\xC7\xE7\xE7`\xE1\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX]V[_a3\x89\x85\x85a<IV[`@Qc\xA1O\x89q`\xE0\x1B\x81R\x90\x91P_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a3\xC5\x90\x85\x90`\x04\x01aSUV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xDFW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra4\x06\x91\x90\x81\x01\x90aS\x8AV[\x90Pa4\x11\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a4M\x83aT\xD4V[\x90\x91UPP`\x08\x81\x01T`@\x80Q\x80\x82\x01\x82R` \x80\x89\x01Q\x82R\x80\x82\x01\x87\x90R_\x84\x81R`\x07\x86\x01\x90\x91R\x91\x90\x91 \x81Q\x81\x90a4\x8B\x90\x82aT\xECV[P` \x82\x81\x01Q\x80Qa4\xA4\x92`\x01\x85\x01\x92\x01\x90aDbV[PPP_\x81\x81R`\t\x83\x01` R`@\x90\x81\x90 \x86\x90UQ\x81\x90\x7F\x1F\x80\xA4{Q\x97\x987\x97o\x99\x9Aw5\xFD\xCC\xBB\xE5p\xE0\xD4\0\x81dN\xC8\x8F\x8E\xD7l\x96\x12\x90a4\xF1\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90aY\xA1V[`@Q\x80\x91\x03\x90\xA2\x80\x7F\x8E\xD4\xFA\xB8\xB1\xD0Pgl\xD7\xD7\xD9\xFAD\x8D]\"\x113u\xFA\xB8\xADM\xC6\x90\xE5E\x93\xC5'\xBF\x89\x89\x89`@Qa5-\x93\x92\x91\x90aY\xDAV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[_\x80[\x82Q\x81\x10\x15a5\x97W_\x83\x82\x81Q\x81\x10a5^Wa5^aO\x8BV[` \x02` \x01\x01Q\x90P_a5r\x82a9\xE2V[\x90Pa5}\x81a:.V[a5\x8B\x90a\xFF\xFF\x16\x85aXJV[\x93PPP`\x01\x01a5BV[Pa\x08\0\x81\x11\x15a\rCW`@Qc\xE7\xF4\x89]`\xE0\x1B\x81Ra\x08\0`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x04\xDEV[`@Qc${\xAC\x9F`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\x81z(_\x1F\xCA;\xB4\x08L\xBF\xC7}K\xAB\xC28\xAD`\x9C\x90c\x91\xEE\xB2|\x90`$\x01a2wV[_a0o\x86\x83a>CV[_a#|a6\x1Da?\x01V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80_\x80a6K\x86\x86a?\x0FV[\x92P\x92P\x92Pa6[\x82\x82a?XV[P\x90\x94\x93PPPPV[`@Qc%\x11\xF3\xF5`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\x94G\xCF\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xC2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE6\x91\x90aO\x9FV[a7\x0EW`@Qc\x15>7{`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\xDEV[`@Qc\x06?\xE89`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R\x83\x16\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c1\xFFA\xC8\x90`D\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7nW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7\x95\x91\x90\x81\x01\x90aZAV[` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a*UW`@Qc\r\x86\xF5!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x04\xDEV[_\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a8\x1BWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a8GWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a8eWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a8}Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a8\x91Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a8\xA3W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a#|W`\x01\x01\x92\x91PPV[a8\xBCa@\x10V[a\x0B\x9AW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a8\xE1a8\xB4V[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02a9-\x84\x82aT\xECV[P`\x03\x81\x01a9<\x83\x82aT\xECV[P_\x80\x82U`\x01\x90\x91\x01UPPV[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x16a\x0B\x9AW`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a9\x96\x82a@)V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a9\xDAWa*U\x82\x82a@\xACV[a\rCaA\x1EV[_`\x08\x82\x90\x1C`\xFF\x16`S\x81\x11\x15a:\x12W`@Qcd\x19P\xD7`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x04\xDEV[\x80`\xFF\x16`S\x81\x11\x15a:'Wa:'aN\x0FV[\x93\x92PPPV[_\x80\x82`S\x81\x11\x15a:BWa:BaN\x0FV[\x03a:OWP`\x02\x91\x90PV[`\x02\x82`S\x81\x11\x15a:cWa:caN\x0FV[\x03a:pWP`\x08\x91\x90PV[`\x03\x82`S\x81\x11\x15a:\x84Wa:\x84aN\x0FV[\x03a:\x91WP`\x10\x91\x90PV[`\x04\x82`S\x81\x11\x15a:\xA5Wa:\xA5aN\x0FV[\x03a:\xB2WP` \x91\x90PV[`\x05\x82`S\x81\x11\x15a:\xC6Wa:\xC6aN\x0FV[\x03a:\xD3WP`@\x91\x90PV[`\x06\x82`S\x81\x11\x15a:\xE7Wa:\xE7aN\x0FV[\x03a:\xF4WP`\x80\x91\x90PV[`\x07\x82`S\x81\x11\x15a;\x08Wa;\x08aN\x0FV[\x03a;\x15WP`\xA0\x91\x90PV[`\x08\x82`S\x81\x11\x15a;)Wa;)aN\x0FV[\x03a;7WPa\x01\0\x91\x90PV[\x81`@Qc\xBEx0\xB1`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x91\x90aZ\xF2V[\x91\x90PV[_\x80`@Q\x80`\xE0\x01`@R\x80`\xA9\x81R` \x01a\\\x93`\xA9\x919\x80Q\x90` \x01 \x84_\x01Q\x80Q\x90` \x01 \x85` \x01Q`@Q` \x01a;\x99\x91\x90a[\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q`@Q` \x01a;\xD3\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x98\x90\x98R\x81\x01\x95\x90\x95R``\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0C`\x83\x82aA=V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<dWa<daH0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_a<\xC0\x84\x84_\x81\x81\x10a<\xA6Wa<\xA6aO\x8BV[``\x02\x91\x90\x91\x015`\x10\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90PV[`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\xF3\xAA\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x11W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=5\x91\x90aO\x9FV[a=UW`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[_\x80[\x84\x81\x10\x15a>\x0CW_\x86\x86\x83\x81\x81\x10a=sWa=saO\x8BV[``\x02\x91\x90\x91\x015\x91PPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x10\x82\x90\x1C\x16\x84\x81\x14a=\xBEW`@QcJ\xC8t\x8B`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x81\x01\x86\x90R`d\x01a\x04\xDEV[_a=\xC8\x83a9\xE2V[\x90Pa=\xD3\x81a:.V[a=\xE1\x90a\xFF\xFF\x16\x86aXJV[\x94P\x82\x87\x85\x81Q\x81\x10a=\xF6Wa=\xF6aO\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a=XV[Pa\x08\0\x81\x11\x15a>;W`@Qc\xE7\xF4\x89]`\xE0\x1B\x81Ra\x08\0`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x04\xDEV[PP\x92\x91PPV[_\x80`@Q\x80`\xC0\x01`@R\x80`\x87\x81R` \x01a\\\x0C`\x87\x919\x80Q\x90` \x01 \x84_\x01Q\x80Q\x90` \x01 \x85` \x01Q`@Q` \x01a>\x85\x91\x90a[\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q`@Q` \x01a>\xBA\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R``\x84\x01\x92\x90\x92R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01a<'V[_a?\naA\xD3V[\x90P\x90V[_\x80_\x83Q`A\x03a?FW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa?8\x88\x82\x85\x85aBFV[\x95P\x95P\x95PPPPa?QV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a?kWa?kaN\x0FV[\x03a?tWPPV[`\x01\x82`\x03\x81\x11\x15a?\x88Wa?\x88aN\x0FV[\x03a?\xA6W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a?\xBAWa?\xBAaN\x0FV[\x03a?\xDBW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`\x03\x82`\x03\x81\x11\x15a?\xEFWa?\xEFaN\x0FV[\x03a\rCW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[_a@\x19a'qV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a@^W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04\xDEV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa@\xC8\x91\x90aW\xBDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aA\0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aA\x05V[``\x91P[P\x91P\x91PaA\x15\x85\x83\x83aC\x0EV[\x95\x94PPPPPV[4\x15a\x0B\x9AW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaAhaCjV[aApaC\xE5V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x81\x01\x85\x90R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0C`\x81\x84`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaA\xFDaCjV[aB\x05aC\xE5V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15aB\x7FWP_\x91P`\x03\x90P\x82aC\x04V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aB\xD0W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aB\xFBWP_\x92P`\x01\x91P\x82\x90PaC\x04V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aC#WaC\x1E\x82aD:V[a:'V[\x81Q\x15\x80\x15aC:WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aCcW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\xDEV[P\x92\x91PPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x81aC\x95a+\xE1V[\x80Q\x90\x91P\x15aC\xADW\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15aC\xBCW\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x81aD\x10a,\xB4V[\x80Q\x90\x91P\x15aD(W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15aC\xBCW\x93\x92PPPV[\x80Q\x15aDIW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\x9BW\x91` \x02\x82\x01[\x82\x81\x11\x15aD\x9BW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aD\x80V[PaD\xA7\x92\x91PaE;V[P\x90V[`@Q\x80`\xC0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01aD\xEE`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\x9BW\x91` \x02\x82\x01[\x82\x81\x11\x15aD\x9BW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aE V[[\x80\x82\x11\x15aD\xA7W_\x81U`\x01\x01aE<V[_\x80\x83`\x1F\x84\x01\x12aE_W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEvW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aE\xAAW_\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aE\xC8W_\x80\xFD[aE\xD4\x8B\x83\x8C\x01aEOV[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15aE\xECW_\x80\xFD[aE\xF8\x8B\x83\x8C\x01aEOV[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15aF\x10W_\x80\xFD[PaF\x1D\x8A\x82\x8B\x01aEOV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_` \x82\x84\x03\x12\x15aF@W_\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\x87W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aFbV[P\x90\x96\x95PPPPPPV[_[\x83\x81\x10\x15aF\xADW\x81\x81\x01Q\x83\x82\x01R` \x01aF\x95V[PP_\x91\x01RV[_\x81Q\x80\x84RaF\xCC\x81` \x86\x01` \x86\x01aF\x93V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a:'` \x83\x01\x84aF\xB5V[_\x80\x83`\x1F\x84\x01\x12aG\x02W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\x19W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[_\x80_\x80`@\x85\x87\x03\x12\x15aGFW_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aG]W_\x80\xFD[aGi\x88\x83\x89\x01aF\xF2V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aG\x81W_\x80\xFD[PaG\x8E\x87\x82\x88\x01aEOV[\x95\x98\x94\x97P\x95PPPPV[_\x80\x83`\x1F\x84\x01\x12aG\xAAW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xC1W_\x80\xFD[` \x83\x01\x91P\x83` ``\x83\x02\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[_\x80_\x80`@\x85\x87\x03\x12\x15aG\xEEW_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aH\x05W_\x80\xFD[aGi\x88\x83\x89\x01aG\x9AV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)\x83W_\x80\xFD[\x805a;R\x81aH\x11V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aHgWaHgaH0V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aH\x96WaH\x96aH0V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aH\xB7WaH\xB7aH0V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x80`@\x83\x85\x03\x12\x15aH\xD6W_\x80\xFD[\x825aH\xE1\x81aH\x11V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xFCW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x0CW_\x80\xFD[\x805aI\x1FaI\x1A\x82aH\x9EV[aHmV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aI3W_\x80\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aIbW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIyW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[_\x80_\x80`@\x85\x87\x03\x12\x15aI\xA6W_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aI\xBDW_\x80\xFD[aGi\x88\x83\x89\x01aIRV[`\xFF`\xF8\x1B\x88\x16\x81R_` `\xE0` \x84\x01RaI\xE9`\xE0\x84\x01\x8AaF\xB5V[\x83\x81\x03`@\x85\x01RaI\xFB\x81\x8AaF\xB5V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R` \x80\x88\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aJNW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJ2V[P\x90\x9C\x9BPPPPPPPPPPPPV[_`@\x82\x84\x03\x12\x15aJpW_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x80_\x80_\x80_a\x01 \x8C\x8E\x03\x12\x15aJ\x91W_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8D5\x11\x15aJ\xA7W_\x80\xFD[aJ\xB4\x8E\x8E5\x8F\x01aIRV[\x90\x9CP\x9APaJ\xC6\x8E` \x8F\x01aJ`V[\x99PaJ\xD5\x8E``\x8F\x01aJ`V[\x98P\x80`\xA0\x8E\x015\x11\x15aJ\xE7W_\x80\xFD[aJ\xF7\x8E`\xA0\x8F\x015\x8F\x01aJ`V[\x97P\x80`\xC0\x8E\x015\x11\x15aK\tW_\x80\xFD[aK\x19\x8E`\xC0\x8F\x015\x8F\x01aEOV[\x90\x97P\x95P`\xE0\x8D\x015\x81\x10\x15aK.W_\x80\xFD[aK>\x8E`\xE0\x8F\x015\x8F\x01aEOV[\x90\x95P\x93Pa\x01\0\x8D\x015\x81\x10\x15aKTW_\x80\xFD[PaKf\x8Da\x01\0\x8E\x015\x8E\x01aEOV[\x81\x93P\x80\x92PPP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x80_\x80_\x80_\x80_\x80_\x80a\x01\0\x8D\x8F\x03\x12\x15aK\x9AW_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D5\x11\x15aK\xAFW_\x80\xFD[aK\xBC\x8E\x8E5\x8F\x01aG\x9AV[\x90\x9CP\x9APaK\xCD` \x8E\x01aH%V[\x99Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x8E\x015\x11\x15aK\xE7W_\x80\xFD[aK\xF7\x8E`@\x8F\x015\x8F\x01aEOV[\x90\x99P\x97Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x8E\x015\x11\x15aL\x14W_\x80\xFD[aL$\x8E``\x8F\x015\x8F\x01aF\xF2V[\x90\x97P\x95PaL6\x8E`\x80\x8F\x01aJ`V[\x94Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x8E\x015\x11\x15aLPW_\x80\xFD[aL`\x8E`\xC0\x8F\x015\x8F\x01aEOV[\x90\x94P\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xE0\x8E\x015\x11\x15aL}W_\x80\xFD[aL\x8D\x8E`\xE0\x8F\x015\x8F\x01aEOV[\x81\x93P\x80\x92PPP\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[_\x80_\x80_\x80_\x80_\x80_a\x01\0\x8C\x8E\x03\x12\x15aL\xC0W_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8D5\x11\x15aL\xD6W_\x80\xFD[aL\xE3\x8E\x8E5\x8F\x01aIRV[\x90\x9CP\x9APaL\xF5\x8E` \x8F\x01aJ`V[\x99P\x80``\x8E\x015\x11\x15aM\x07W_\x80\xFD[aM\x17\x8E``\x8F\x015\x8F\x01aJ`V[\x98PaM%`\x80\x8E\x01aH%V[\x97P\x80`\xA0\x8E\x015\x11\x15aM7W_\x80\xFD[aMG\x8E`\xA0\x8F\x015\x8F\x01aEOV[\x90\x97P\x95P`\xC0\x8D\x015\x81\x10\x15aM\\W_\x80\xFD[aMl\x8E`\xC0\x8F\x015\x8F\x01aEOV[\x90\x95P\x93P`\xE0\x8D\x015\x81\x10\x15aM\x81W_\x80\xFD[PaKf\x8D`\xE0\x8E\x015\x8E\x01aEOV[_\x80_\x80_``\x86\x88\x03\x12\x15aM\xA6W_\x80\xFD[\x855aM\xB1\x81aH\x11V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aM\xCDW_\x80\xFD[aM\xD9\x89\x83\x8A\x01aIRV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aM\xF1W_\x80\xFD[PaM\xFE\x88\x82\x89\x01aEOV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80aN7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aJpWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a#|Wa#|aNUV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x87\x81R`\x80` \x82\x01R_aN\xBD`\x80\x83\x01\x88\x8AaN|V[\x82\x81\x03`@\x84\x01RaN\xD0\x81\x87\x89aN|V[\x90P\x82\x81\x03``\x84\x01RaN\xE5\x81\x85\x87aN|V[\x9A\x99PPPPPPPPPPV[_\x85QaO\x04\x81\x84` \x8A\x01aF\x93V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaO#\x81`\x02\x84\x01` \x8A\x01aF\x93V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaOG\x81`\x03\x85\x01` \x8A\x01aF\x93V[`\x03\x92\x01\x91\x82\x01R\x83QaOb\x81`\x04\x84\x01` \x88\x01aF\x93V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aO\x80W_\x80\xFD[\x81Qa:'\x81aH\x11V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aO\xAFW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a:'W_\x80\xFD[`\x1F\x82\x11\x15a*UW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aO\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a2\xA0W_\x81U`\x01\x01aO\xEFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aP\x1AWaP\x1AaH0V[aP.\x83aP(\x83TaN#V[\x83aO\xBEV[_`\x1F\x84\x11`\x01\x81\x14aP_W_\x85\x15aPHWP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua2\xA0V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15aP\x8EW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aPnV[P\x86\x82\x10\x15aP\xAAW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[`\x80\x81R_aP\xCF`\x80\x83\x01\x89\x8BaN|V[\x82\x81\x03` \x84\x01RaP\xE2\x81\x88\x8AaN|V[\x90P`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x84\x01R\x82\x81\x03``\x84\x01RaN\xE5\x81\x85\x87aN|V[``\x81R_aQ\x19``\x83\x01\x87\x89aN|V[` \x83\x82\x03\x81\x85\x01R\x81\x87T\x80\x84R\x82\x84\x01\x91P`\x05\x83\x82`\x05\x1B\x86\x01\x01\x8A_R\x84_ _[\x84\x81\x10\x15aQ\xD2W`\x1F\x19\x88\x84\x03\x01\x86R_\x82TaQ\\\x81aN#V[\x80\x86R`\x01\x82\x81\x16\x80\x15aQwW`\x01\x81\x14aQ\x90WaQ\xBBV[`\xFF\x19\x84\x16\x88\x8D\x01R\x82\x15\x15\x89\x1B\x88\x01\x8C\x01\x94PaQ\xBBV[\x86_R\x8B_ _[\x84\x81\x10\x15aQ\xB3W\x81T\x8A\x82\x01\x8F\x01R\x90\x83\x01\x90\x8D\x01aQ\x98V[\x89\x01\x8D\x01\x95PP[P\x98\x8A\x01\x98\x92\x95PPP\x91\x90\x91\x01\x90`\x01\x01aQ?V[PP\x87\x81\x03`@\x89\x01RaQ\xE7\x81\x8A\x8CaN|V[\x9D\x9CPPPPPPPPPPPPPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aR\rW_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aR'W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aE\x8DW_\x80\xFD[_`@\x82\x84\x03\x12\x15aRNW_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aRqWaRqaH0V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`@\x82\x84\x03\x12\x15aR\x9BW_\x80\xFD[a:'\x83\x83aR>V[_` \x82\x84\x03\x12\x15aR\xB5W_\x80\xFD[\x815a:'\x81aH\x11V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x90\x83\x01\x84\x90R_\x91\x85\x91``\x85\x01\x84[\x87\x81\x10\x15aS\x0EW\x845aR\xFA\x81aH\x11V[\x84\x16\x82R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aR\xE7V[P\x98\x97PPPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aSJW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aS.V[P\x94\x95\x94PPPPPV[` \x81R_a:'` \x83\x01\x84aS\x1BV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aS\x80WaS\x80aH0V[P`\x05\x1B` \x01\x90V[_` \x80\x83\x85\x03\x12\x15aS\x9BW_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aS\xB2W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aS\xC5W_\x80\xFD[\x81QaS\xD3aI\x1A\x82aSgV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aS\xF1W_\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aS\x0EW\x80Q\x85\x81\x11\x15aT\x0BW_\x80\xFD[\x86\x01`\x80\x81\x8C\x03`\x1F\x19\x01\x12\x15aT W_\x80\xFD[aT(aHDV[\x88\x82\x01Q\x81R`@\x80\x83\x01Q\x8A\x83\x01R``\x83\x01Q\x81\x83\x01R`\x80\x83\x01Q\x88\x81\x11\x15aTRW_\x80\xFD[\x80\x84\x01\x93PP\x8C`?\x84\x01\x12aTfW_\x80\xFD[\x89\x83\x01QaTvaI\x1A\x82aSgV[\x81\x81R`\x05\x91\x90\x91\x1B\x84\x01\x82\x01\x90\x8B\x81\x01\x90\x8F\x83\x11\x15aT\x94W_\x80\xFD[\x94\x83\x01\x94[\x82\x86\x10\x15aT\xBEW\x85Q\x93PaT\xAE\x84aH\x11V[\x83\x82R\x94\x8C\x01\x94\x90\x8C\x01\x90aT\x99V[``\x85\x01RPPP\x84RP\x91\x86\x01\x91\x86\x01aS\xF5V[_`\x01\x82\x01aT\xE5WaT\xE5aNUV[P`\x01\x01\x90V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\x06WaU\x06aH0V[aU\x1A\x81aU\x14\x84TaN#V[\x84aO\xBEV[` \x80`\x1F\x83\x11`\x01\x81\x14aUMW_\x84\x15aU6WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaU\xA4V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aU{W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aU\\V[P\x85\x82\x10\x15aU\x98W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aSJW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\xBFV[\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R_``\x82\x01Q`\x80``\x85\x01Ra\x0C``\x80\x85\x01\x82aU\xACV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aVaW`\x1F\x19\x86\x84\x03\x01\x89RaVO\x83\x83QaU\xE4V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aV3V[P\x90\x97\x96PPPPPPPV[`\x80\x81R_aV\x80`\x80\x83\x01\x89aV\x16V[`\x01`\x01`\xA0\x1B\x03\x88\x16` \x84\x01R\x82\x81\x03`@\x84\x01RaV\xA2\x81\x87\x89aN|V[\x90P\x82\x81\x03``\x84\x01RaV\xB7\x81\x85\x87aN|V[\x99\x98PPPPPPPPPV[`\x80\x81R_aV\x80`\x80\x83\x01\x89aS\x1BV[\x81\x83R_\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aW\x06W_\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[` \x81R_a\x0C`` \x83\x01\x84\x86aV\xD6V[`@\x81R_aWD`@\x83\x01\x86aV\x16V[\x82\x81\x03` \x84\x01Ra\"\x9E\x81\x85\x87aN|V[`@\x81R_aWj`@\x83\x01\x86\x88aV\xD6V[\x82\x81\x03` \x84\x01RaW}\x81\x85\x87aN|V[\x97\x96PPPPPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aW\xB1W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aW\x95V[P\x92\x96\x95PPPPPPV[_\x82QaW\xCE\x81\x84` \x87\x01aF\x93V[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aW\xE8W_\x80\xFD[PQ\x91\x90PV[_\x80\x85\x85\x11\x15aW\xFDW_\x80\xFD[\x83\x86\x11\x15aX\tW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a#|W_\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a#|Wa#|aNUV[\x80\x82\x01\x80\x82\x11\x15a#|Wa#|aNUV[\x82\x81R``\x81\x01a:'` \x83\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[` \x81R_a\x0C`` \x83\x01\x84\x86aN|V[`@\x81R_aX\xA0`@\x83\x01\x85aU\xE4V[\x82\x81\x03` \x84\x01RaA\x15\x81\x85aU\xE4V[\x81\x83R_` \x80\x85\x01\x94P\x82_[\x85\x81\x10\x15aSJW\x815\x87R\x82\x82\x015aX\xD9\x81aH\x11V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x88\x85\x01R`@\x90\x83\x82\x015aX\xF8\x81aH\x11V[\x16\x90\x88\x01R``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aX\xC0V[`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R_` \x82\x01Q`\xE0` \x85\x01RaY7`\xE0\x85\x01\x82aF\xB5V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaYP\x82\x82aU\xACV[\x91PP``\x83\x01QaYo``\x86\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P`\x80\x83\x01Q\x84\x82\x03`\xA0\x86\x01RaY\x87\x82\x82aF\xB5V[\x91PP`\xA0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaA\x15\x82\x82aF\xB5V[``\x81R_aY\xB3``\x83\x01\x87aV\x16V[\x82\x81\x03` \x84\x01RaY\xC6\x81\x86\x88aX\xB2V[\x90P\x82\x81\x03`@\x84\x01RaW}\x81\x85aY\x10V[`@\x81R_aY\xED`@\x83\x01\x85\x87aX\xB2V[\x82\x81\x03` \x84\x01Ra\"\x9E\x81\x85aY\x10V[_\x82`\x1F\x83\x01\x12aZ\x0EW_\x80\xFD[\x81QaZ\x1CaI\x1A\x82aH\x9EV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aZ0W_\x80\xFD[a\x0C`\x82` \x83\x01` \x87\x01aF\x93V[_` \x82\x84\x03\x12\x15aZQW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aZhW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15aZ{W_\x80\xFD[aZ\x83aHDV[\x82QaZ\x8E\x81aH\x11V[\x81R` \x83\x01QaZ\x9E\x81aH\x11V[` \x82\x01R`@\x83\x01Q\x82\x81\x11\x15aZ\xB4W_\x80\xFD[aZ\xC0\x87\x82\x86\x01aY\xFFV[`@\x83\x01RP``\x83\x01Q\x82\x81\x11\x15aZ\xD7W_\x80\xFD[aZ\xE3\x87\x82\x86\x01aY\xFFV[``\x83\x01RP\x95\x94PPPPPV[` \x81\x01`T\x83\x10a[\x12WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aW\xB1W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a[%V\xFEUserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare,bytes extraData)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 startTimestamp,uint256 durationDays,bytes extraData)DelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 startTimestamp,uint256 durationDays,bytes extraData)h\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610178575f3560e01c80636f8913bc116100d1578063b4de2c371161007c578063f1b57adb11610057578063f1b57adb1461044b578063fa2106b81461046a578063fbb832591461047e575f80fd5b8063b4de2c37146103ee578063d8998f451461040d578063e22d1b261461042c575f80fd5b806384b0196e116100ac57806384b0196e146103605780639fad5a2f14610387578063ad3cb1cc146103a6575f80fd5b80636f8913bc1461030e57806376227eed1461032d5780638456cb591461034c575f80fd5b80634014c4cd1161013157806352d1902d1161010c57806352d1902d1461027c57806358f5b8ab1461029e5780635c975abb146102d8575f80fd5b80634014c4cd1461021b578063410bf0ba1461024a5780634f1ef28614610269575f80fd5b80630d8e6e2c116101615780630d8e6e2c146101d257806339f73810146101f35780633f4ba83a14610207575f80fd5b8063046f9eb31461017c5780630900cc691461019d575b5f80fd5b348015610187575f80fd5b5061019b610196366004614594565b61049d565b005b3480156101a8575f80fd5b506101bc6101b7366004614630565b610806565b6040516101c99190614647565b60405180910390f35b3480156101dd575f80fd5b506101e66108d2565b6040516101c991906146e0565b3480156101fe575f80fd5b5061019b61093a565b348015610212575f80fd5b5061019b610ac9565b348015610226575f80fd5b5061023a610235366004614733565b610b9c565b60405190151581526020016101c9565b348015610255575f80fd5b5061023a6102643660046147db565b610c68565b61019b6102773660046148c5565b610d28565b348015610287575f80fd5b50610290610d47565b6040519081526020016101c9565b3480156102a9575f80fd5b5061023a6102b8366004614630565b5f9081525f80516020615d3c833981519152602052604090205460ff1690565b3480156102e3575f80fd5b507fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff1661023a565b348015610319575f80fd5b5061019b610328366004614594565b610d75565b348015610338575f80fd5b5061023a610347366004614993565b61108b565b348015610357575f80fd5b5061019b61114b565b34801561036b575f80fd5b50610374611205565b6040516101c997969594939291906149c9565b348015610392575f80fd5b5061019b6103a1366004614a76565b6112c9565b3480156103b1575f80fd5b506101e66040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b3480156103f9575f80fd5b5061019b610408366004614b7e565b611869565b348015610418575f80fd5b5061019b610427366004614733565b611a11565b348015610437575f80fd5b5061023a610446366004614993565b611c0f565b348015610456575f80fd5b5061019b610465366004614ca5565b611ccf565b348015610475575f80fd5b5061019b6121e0565b348015610489575f80fd5b5061023a610498366004614d92565b612291565b5f80516020615d3c833981519152600160f91b881115806104c15750806008015488115b156104e757604051636a457ca160e11b8152600481018990526024015b60405180910390fd5b5f888152600782016020526040808220815180830190925280548290829061050e90614e23565b80601f016020809104026020016040519081016040528092919081815260200182805461053a90614e23565b80156105855780601f1061055c57610100808354040283529160200191610585565b820191905f5260205f20905b81548152906001019060200180831161056857829003601f168201915b50505050508152602001600182018054806020026020016040519081016040528092919081815260200182805480156105db57602002820191905f5260205f20905b8154815260200190600101908083116105c7575b50505050508152505090505f6040518060800160405280835f01518152602001836020015181526020018a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250604080516020601f8901819004810282018101909252878152918101919088908890819084018382808284375f920182905250939094525092935091506106859050826122a8565b5f8c81526009860160205260408120549192506106a28888612382565b9050815f036106b3578091506106e4565b8181146106e4576040516355dafa4360e11b8152600481018e905260248101839052604481018290526064016104de565b506106f2818d848c8c61254b565b5f8c815260028601602090815260408083208380528252822080546001818101835582855292909320909201805473ffffffffffffffffffffffffffffffffffffffff19163317905581548e917f7fcdfb5381917f554a717d0a5470a33f5a49ba6445f05ec43c74c0bc2cc608b29161076b9190614e69565b8e8e8e8e8e8e6040516107849796959493929190614ea4565b60405180910390a25f8d81526020879052604090205460ff161580156107b2575080546107b2908390612638565b156107f7575f8d815260208790526040808220805460ff19166001179055518e917fe89752be0ecdb68b2a6eb5ef1a891039e0e92ae3c8a62274c5881e48eea1ed2591a25b50505050505050505050505050565b5f8181527f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70360209081526040808320547f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee702835281842081855283529281902080548251818502810185019093528083526060945f80516020615d3c8339815191529490939291908301828280156108c457602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116108a6575b505050505092505050919050565b60606040518060400160405280600a8152602001692232b1b93cb83a34b7b760b11b8152506109005f6126ba565b61090a60076126ba565b6109135f6126ba565b6040516020016109269493929190614ef3565b604051602081830303815290604052905090565b610942612758565b67ffffffffffffffff1660011461096c57604051636f4f731f60e01b815260040160405180910390fd5b60085f610977612771565b8054909150600160401b900460ff168061099f5750805467ffffffffffffffff808416911610155b156109bd5760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff191667ffffffffffffffff831617600160401b178155604080518082018252600a8152692232b1b93cb83a34b7b760b11b602080830191909152825180840190935260018352603160f81b90830152610a2191612799565b610a296127ab565b600160f81b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70655600160f91b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70855805468ff00000000000000001916815560405167ffffffffffffffff831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2906020015b60405180910390a15050565b73d582ec82a1758322907df80da8a754e12a5acb956001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b19573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b3d9190614f70565b6001600160a01b0316336001600160a01b031614158015610b7257503373d582ec82a1758322907df80da8a754e12a5acb9514155b15610b92576040516370c8b37760e11b81523360048201526024016104de565b610b9a6127b3565b565b5f838103610bab57505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f878784818110610be157610be1614f8b565b905060200201356040518263ffffffff1660e01b8152600401610c0691815260200190565b602060405180830381865afa158015610c21573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c459190614f9f565b610c52575f915050610c60565b600101610bad565b50600190505b949350505050565b5f838103610c7757505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f878784818110610cad57610cad614f8b565b9050606002015f01356040518263ffffffff1660e01b8152600401610cd491815260200190565b602060405180830381865afa158015610cef573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d139190614f9f565b610d20575f915050610c60565b600101610c79565b610d30612825565b610d39826128dc565b610d438282612986565b5050565b5f610d50612a5a565b507f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc90565b5f80516020615d3c833981519152600160f81b88111580610d995750806006015488115b15610dba57604051636a457ca160e11b8152600481018990526024016104de565b604080515f8a81526005840160209081528382208054608092810285018301909552606084018581529294849392840182828015610e1557602002820191905f5260205f20905b815481526020019060010190808311610e01575b5050505050815260200189898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250604080516020601f8801819004810282018101909252868152918101919087908790819084018382808284375f92018290525093909452509293509150610e9f905082612aa3565b5f8b8152600985016020526040812054919250610ebc8787612382565b9050815f03610ecd57809150610efe565b818114610efe576040516355dafa4360e11b8152600481018d905260248101839052604481018290526064016104de565b610f0b828d858c8c61254b565b5f8c815260048601602090815260408083208684528252822080546001810182558184529190922001610f3f8a8c83615002565b50856002015f8e81526020019081526020015f205f8581526020019081526020015f2033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a8154816001600160a01b0302191690836001600160a01b031602179055508c7f4d7b1dba49e9e846215e1621f5737c81d8614c4f268494d8b787632c4e59f0e58d8d8d8d338e8e604051610fe297969594939291906150bc565b60405180910390a25f8d81526020879052604090205460ff1615801561101057508054611010908490612b4a565b156107f7575f8d815260208781526040808320805460ff191660011790556003890190915290819020859055518d907fd7e58a367a0a6c298e76ad5d240004e327aa1423cbe4bd7ff85d4c715ef8d15f90611074908f908f9086908e908e90615106565b60405180910390a250505050505050505050505050565b5f83810361109a57505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f8787848181106110d0576110d0614f8b565b9050604002015f01356040518263ffffffff1660e01b81526004016110f791815260200190565b602060405180830381865afa158015611112573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111369190614f9f565b611143575f915050610c60565b60010161109c565b60405163237dfb4760e11b815233600482015273d582ec82a1758322907df80da8a754e12a5acb95906346fbf68e90602401602060405180830381865afa158015611198573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111bc9190614f9f565b1580156111dd57503373d582ec82a1758322907df80da8a754e12a5acb9514155b156111fd5760405163388916bb60e01b81523360048201526024016104de565b610b9a612b86565b5f60608082808083817fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100805490915015801561124357506001810154155b61128f5760405162461bcd60e51b815260206004820152601560248201527f4549503731323a20556e696e697469616c697a6564000000000000000000000060448201526064016104de565b611297612be1565b61129f612cb4565b604080515f80825260208201909252600f60f81b9c939b5091995046985030975095509350915050565b6112d1612d05565b604051635ff9d55d60e11b81528735600482018190529073d582ec82a1758322907df80da8a754e12a5acb959063bff3aaba90602401602060405180830381865afa158015611322573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113469190614f9f565b6113665760405163b6679c3b60e01b8152600481018290526024016104de565b60405163666286dd60e11b81526004810182905273d582ec82a1758322907df80da8a754e12a5acb959063ccc50dba90602401602060405180830381865afa1580156113b4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113d89190614f9f565b156113f95760405163180d9a3160e21b8152600481018290526024016104de565b61140660208901896151f8565b90505f03611427576040516357cfa21760e01b815260040160405180910390fd5b600a61143660208a018a6151f8565b9050111561147257600a61144d60208a018a6151f8565b60405163af1f049560e01b815260ff90931660048401526024830152506044016104de565b611489611484368c90038c018c61528b565b612d48565b6114dc61149960208a018a6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152506114d79250505060208c018c6152a5565b612e14565b15611517576114ee60208a018a6152a5565b6114fb60208a018a6151f8565b60405163c3446ac760e01b81526004016104de939291906152c0565b5f6115238d8d8b612e6d565b90505f6040518060c001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525050509082525060209081019061157b908d018d6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152505050908252506020908101906115c0908e018e6152a5565b6001600160a01b031681526020018d5f013581526020018d60200135815260200186868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505091525090506116378161162e60408e0160208f016152a5565b89898e35613064565b5060405163a14f897160e01b81525f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f897190611671908590600401615355565b5f60405180830381865afa15801561168b573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526116b2919081019061538a565b90506116bd816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70880545f80516020615d3c833981519152915f6116f9836154d4565b909155505060088101546040805160606020601f8e01819004028201810183529181018c815290918291908e908e90819085018382808284375f920182905250938552505050602091820187905283815260078501909152604090208151819061176390826154ec565b50602082810151805161177c9260018501920190614462565b509050505f61178b8888612382565b9050611796816131a9565b5f8281526009840160205260409020556117af3361323b565b807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848f60200160208101906117e591906152a5565b8e8e8c8c6040516117fb9695949392919061566e565b60405180910390a2807fc71e32c80f3109c673a9adfcded0b3b2093212ab284084a0e03a281dee2bdd5f858f602001602081019061183991906152a5565b8e8e8c8c60405161184f969594939291906156c4565b60405180910390a250505050505050505050505050505050565b611871612d05565b5f8b90036118925760405163240e930960e01b815260040160405180910390fd5b600a8611156118be5760405163af1f049560e01b8152600a6004820152602481018790526044016104de565b6118d56118d03687900387018761528b565b6132a7565b6118dd6144ab565b6001600160a01b038b168152604080516020601f8c018190048102820181019092528a8152908b908b90819084018382808284375f92019190915250505050602080830191909152604080518983028181018401909252898152918a918a918291908501908490808284375f9201919091525050505060408201526119673687900387018761528b565b6060820152604080516020601f85018190048102820181019092528381529084908490819084018382808284375f920191909152505050506080820152604080516020601f87018190048102820181019092528581529086908690819084018382808284375f92018290525060a0860194909452506119ea915085905084612382565b90506119f53361323b565b611a018e8e848461337e565b5050505050505050505050505050565b611a19612d05565b5f839003611a3a576040516305bcea8760e31b815260040160405180910390fd5b611a758484808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061353f92505050565b60405163a14f897160e01b81525f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f897190611ab0908890889060040161571f565b5f60405180830381865afa158015611aca573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611af1919081019061538a565b9050611afc816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70680545f80516020615d3c833981519152915f611b38836154d4565b909155505060068101545f8181526005830160205260409020611b5c908888614502565b505f611b688686612382565b9050611b73816131a9565b5f828152600984016020526040902055611b8c336135c6565b807f22db480a39bd72556438aadb4a32a3d2a6638b87c03bbec5fef6997e109587ff848787604051611bc093929190615732565b60405180910390a2807f9bb088a53f1fe13ec84fa0a1d7b578b589e193268baaccc5e23f7ab785aa3dae88888888604051611bfe9493929190615757565b60405180910390a250505050505050565b5f838103611c1e57505f610c60565b5f5b84811015610c5a5773c7d45661a345ec5ca0e8521cfef7e32fda0daa68632ddc9a6f878784818110611c5457611c54614f8b565b9050604002015f01356040518263ffffffff1660e01b8152600401611c7b91815260200190565b602060405180830381865afa158015611c96573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611cba9190614f9f565b611cc7575f915050610c60565b600101611c20565b611cd7612d05565b604051635ff9d55d60e11b81528835600482018190529073d582ec82a1758322907df80da8a754e12a5acb959063bff3aaba90602401602060405180830381865afa158015611d28573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d4c9190614f9f565b611d6c5760405163b6679c3b60e01b8152600481018290526024016104de565b60405163666286dd60e11b81526004810182905273d582ec82a1758322907df80da8a754e12a5acb959063ccc50dba90602401602060405180830381865afa158015611dba573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611dde9190614f9f565b15611dff5760405163180d9a3160e21b8152600481018290526024016104de565b611e0c60208a018a6151f8565b90505f03611e2d576040516357cfa21760e01b815260040160405180910390fd5b600a611e3c60208b018b6151f8565b90501115611e5357600a61144d60208b018b6151f8565b611e65611484368c90038c018c61528b565b611ead611e7560208b018b6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152508c9250612e14915050565b15611edc5787611ec060208b018b6151f8565b60405163dc4d78b160e01b81526004016104de939291906152c0565b5f611ee88d8d8c612e6d565b90505f6040518060a001604052808a8a8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250505090825250602090810190611f40908e018e6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152505050908252508d356020808301919091528e8101356040808401919091528051601f89018390048302810183019091528781526060909201919088908890819084018382808284375f9201919091525050509152509050611fd2818b89898f35613606565b60405163a14f897160e01b81525f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f89719061200b908690600401615355565b5f60405180830381865afa158015612025573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261204c919081019061538a565b9050612057816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70880545f80516020615d3c833981519152915f612093836154d4565b909155505060088101546040805160606020601f8f01819004028201810183529181018d815290918291908f908f90819085018382808284375f92018290525093855250505060209182018890528381526007850190915260409020815181906120fd90826154ec565b5060208281015180516121169260018501920190614462565b509050505f6121258989612382565b9050612130816131a9565b5f8281526009840160205260409020556121493361323b565b807ff9011bd6ba0da6049c520d70fe5971f17ed7ab795486052544b51019896c596b848f8f8f8d8d6040516121839695949392919061566e565b60405180910390a2807fc71e32c80f3109c673a9adfcded0b3b2093212ab284084a0e03a281dee2bdd5f868f8f8f8d8d6040516121c5969594939291906156c4565b60405180910390a25050505050505050505050505050505050565b60085f6121eb612771565b8054909150600160401b900460ff16806122135750805467ffffffffffffffff808416911610155b156122315760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff191667ffffffffffffffff8316908117600160401b1768ff0000000000000000191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d290602001610abd565b5f61229e85858585611c0f565b9695505050505050565b5f61237c6040518060a00160405280606d8152602001615b4b606d913980519060200120835f01518051906020012084602001516040516020016122ec9190615788565b60405160208183030381529060405280519060200120856040015180519060200120866060015160405160200161232391906157bd565b60408051601f198184030181528282528051602091820120908301969096528101939093526060830191909152608082015260a081019190915260c0015b60405160208183030381529060405280519060200120613611565b92915050565b5f8181036124055773d582ec82a1758322907df80da8a754e12a5acb956001600160a01b031663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa1580156123da573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123fe91906157d8565b905061237c565b5f83835f81811061241857612418614f8b565b919091013560f81c9150505f8190036124a75773d582ec82a1758322907df80da8a754e12a5acb956001600160a01b031663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa15801561247b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061249f91906157d8565b91505061237c565b8060ff16600114806124bc57508060ff166002145b1561252d5760218310156124ed576040516349aa453360e11b815260048101849052602160248201526044016104de565b6124fb6021600185876157ef565b61250491615816565b91505f8290036125275760405163cb17b7a560e01b815260040160405180910390fd5b5061237c565b60405163084e730b60e21b815260ff821660048201526024016104de565b5f5f80516020615d3c83398151915290505f61259c8585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061363d92505050565b90506125a9878233613665565b5f86815260018301602090815260408083206001600160a01b038516845290915290205460ff1615612600576040516399ec48d960e01b8152600481018790526001600160a01b03821660248201526044016104de565b5f9586526001918201602090815260408088206001600160a01b039093168852919052909420805460ff191690941790935550505050565b60405163140f45ff60e11b8152600481018390525f90819073d582ec82a1758322907df80da8a754e12a5acb959063281e8bfe906024015b602060405180830381865afa15801561268b573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906126af91906157d8565b909210159392505050565b60605f6126c6836137d3565b60010190505f8167ffffffffffffffff8111156126e5576126e5614830565b6040519080825280601f01601f19166020018201604052801561270f576020820181803683370190505b5090508181016020015b5f19017f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a8504945084612719575b509392505050565b5f612761612771565b5467ffffffffffffffff16919050565b5f807ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a0061237c565b6127a16138b4565b610d4382826138d9565b610b9a6138b4565b6127bb61394b565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300805460ff191681557f5db9ee0a495bf2e6ff9c91a7834c1ba4fdd244a5e8aa4e537bd38aeae4b073aa335b6040516001600160a01b03909116815260200160405180910390a150565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614806128be57507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166128b27f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc546001600160a01b031690565b6001600160a01b031614155b15610b9a5760405163703e46dd60e11b815260040160405180910390fd5b73d582ec82a1758322907df80da8a754e12a5acb956001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561292c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129509190614f70565b6001600160a01b0316336001600160a01b03161461298357604051630e56cf3d60e01b81523360048201526024016104de565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa9250505080156129e0575060408051601f3d908101601f191682019092526129dd918101906157d8565b60015b612a0857604051634c9c8ce360e01b81526001600160a01b03831660048201526024016104de565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc8114612a4b57604051632a87526960e21b8152600481018290526024016104de565b612a55838361398d565b505050565b306001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610b9a5760405163703e46dd60e11b815260040160405180910390fd5b5f61237c604051806080016040528060548152602001615bb860549139805160209182012084516040519192612ad99201615788565b604051602081830303815290604052805190602001208460200151805190602001208560400151604051602001612b1091906157bd565b60408051601f198184030181528282528051602091820120908301959095528101929092526060820152608081019190915260a001612361565b6040516361d5552d60e11b8152600481018390525f90819073d582ec82a1758322907df80da8a754e12a5acb959063c3aaaa5a90602401612670565b612b8e612d05565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f03300805460ff191660011781557f62e78cea01bee320cd4e420270b5ea74000d11b0c9f74754ebdbfc544b05a25833612807565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10280546060917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10091612c3290614e23565b80601f0160208091040260200160405190810160405280929190818152602001828054612c5e90614e23565b8015612ca95780601f10612c8057610100808354040283529160200191612ca9565b820191905f5260205f20905b815481529060010190602001808311612c8c57829003601f168201915b505050505091505090565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10380546060917fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10091612c3290614e23565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff1615610b9a5760405163d93c066560e01b815260040160405180910390fd5b80602001515f03612d6c5760405163de2859c160e01b815260040160405180910390fd5b602081015161016d1015612da4576020810151604051633295186360e01b815261016d600482015260248101919091526044016104de565b8051421015612dd257805160405163f24c088760e01b815242600482015260248101919091526044016104de565b42816020015162015180612de69190615833565b8251612df2919061584a565b101561298357428160405162c0d20160e61b81526004016104de92919061585d565b5f805b8351811015612e6457826001600160a01b0316848281518110612e3c57612e3c614f8b565b60200260200101516001600160a01b031603612e5c57600191505061237c565b600101612e17565b505f9392505050565b60605f839003612e905760405163a6a6cb2160e01b815260040160405180910390fd5b8267ffffffffffffffff811115612ea957612ea9614830565b604051908082528060200260200182016040528015612ed2578160200160208202803683370190505b5090505f805b84811015613035575f868683818110612ef357612ef3614f8b565b9050604002015f013590505f878784818110612f1157612f11614f8b565b9050604002016020016020810190612f2991906152a5565b905067ffffffffffffffff601083901c1686358114612f6c57604051634ac8748b60e11b81526004810184905260248101829052873560448201526064016104de565b5f612f76846139e2565b9050612f8181613a2e565b612f8f9061ffff168761584a565b9550612fd9612fa160208a018a6151f8565b808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250879250612e14915050565b6130075782612feb60208a018a6151f8565b60405163a4c3039160e01b81526004016104de939291906152c0565b8387868151811061301a5761301a614f8b565b6020908102919091010152505060019092019150612ed89050565b506108008111156127505760405163e7f4895d60e01b81526108006004820152602481018290526044016104de565b5f61306f8683613b57565b90505f6130b18286868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061363d92505050565b9050856001600160a01b0316816001600160a01b0316146130e9578484604051632a873d2760e01b81526004016104de92919061587b565b50505050505050565b60018151116130fe5750565b5f815f8151811061311157613111614f8b565b60200260200101516020015190505f600190505b8251811015612a55578183828151811061314157613141614f8b565b602002602001015160200151146131a157825f8151811061316457613164614f8b565b602002602001015183828151811061317e5761317e614f8b565b602002602001015160405163cfae921f60e01b81526004016104de92919061588e565b600101613125565b6040516317f362d960e31b81526004810182905273d582ec82a1758322907df80da8a754e12a5acb959063bf9b16c890602401602060405180830381865afa1580156131f7573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061321b9190614f9f565b612983576040516377ddbe8160e01b8152600481018290526024016104de565b60405163988a2d2d60e01b81526001600160a01b038216600482015273817a285f1fca3bb4084cbfc77d4babc238ad609c9063988a2d2d906024015b5f604051808303815f87803b15801561328e575f80fd5b505af11580156132a0573d5f803e3d5ffd5b5050505050565b80602001515f036132cb57604051631229e23760e21b815260040160405180910390fd5b6132da61016d62015180615833565b8160200151111561331b576132f461016d62015180615833565b6020820151604051635729758960e11b8152600481019290925260248201526044016104de565b805142101561334957805160405163f24c088760e01b815242600482015260248101919091526044016104de565b60208101518151429161335b9161584a565b10156129835742816040516333c7e7e760e11b81526004016104de92919061585d565b5f6133898585613c49565b60405163a14f897160e01b81529091505f9073c7d45661a345ec5ca0e8521cfef7e32fda0daa689063a14f8971906133c5908590600401615355565b5f60405180830381865afa1580156133df573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052613406919081019061538a565b9050613411816130f2565b7f68113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee70880545f80516020615d3c833981519152915f61344d836154d4565b9091555050600881015460408051808201825260208089015182528082018790525f84815260078601909152919091208151819061348b90826154ec565b5060208281015180516134a49260018501920190614462565b5050505f818152600983016020526040908190208690555181907f1f80a47b51979837976f999a7735fdccbbe570e0d40081644ec88f8ed76c9612906134f19086908c908c908c906159a1565b60405180910390a2807f8ed4fab8b1d050676cd7d7d9fa448d5d22113375fab8ad4dc690e54593c527bf89898960405161352d939291906159da565b60405180910390a25050505050505050565b5f805b8251811015613597575f83828151811061355e5761355e614f8b565b602002602001015190505f613572826139e2565b905061357d81613a2e565b61358b9061ffff168561584a565b93505050600101613542565b50610800811115610d435760405163e7f4895d60e01b81526108006004820152602481018290526044016104de565b60405163247bac9f60e21b81526001600160a01b038216600482015273817a285f1fca3bb4084cbfc77d4babc238ad609c906391eeb27c90602401613277565b5f61306f8683613e43565b5f61237c61361d613f01565b8360405161190160f01b8152600281019290925260228201526042902090565b5f805f8061364b8686613f0f565b92509250925061365b8282613f58565b5090949350505050565b604051632511f3f560e21b8152600481018490526001600160a01b038316602482015273d582ec82a1758322907df80da8a754e12a5acb9590639447cfd490604401602060405180830381865afa1580156136c2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136e69190614f9f565b61370e5760405163153e377b60e11b81526001600160a01b03831660048201526024016104de565b60405163063fe83960e31b8152600481018490526001600160a01b03828116602483015283169073d582ec82a1758322907df80da8a754e12a5acb95906331ff41c8906044015f60405180830381865afa15801561376e573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526137959190810190615a41565b602001516001600160a01b031614612a5557604051630d86f52160e01b81526001600160a01b038084166004830152821660248201526044016104de565b5f807a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061381b577a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000830492506040015b6d04ee2d6d415b85acef81000000008310613847576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc10000831061386557662386f26fc10000830492506010015b6305f5e100831061387d576305f5e100830492506008015b612710831061389157612710830492506004015b606483106138a3576064830492506002015b600a831061237c5760010192915050565b6138bc614010565b610b9a57604051631afcd79f60e31b815260040160405180910390fd5b6138e16138b4565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d1007fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10261392d84826154ec565b506003810161393c83826154ec565b505f8082556001909101555050565b7fcd5ed15c6e187e77e9aee88184c21f4f2182ab5827cb3b7e07fbedcd63f033005460ff16610b9a57604051638dfc202b60e01b815260040160405180910390fd5b61399682614029565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156139da57612a5582826140ac565b610d4361411e565b5f600882901c60ff166053811115613a125760405163641950d760e01b815260ff821660048201526024016104de565b8060ff166053811115613a2757613a27614e0f565b9392505050565b5f80826053811115613a4257613a42614e0f565b03613a4f57506002919050565b6002826053811115613a6357613a63614e0f565b03613a7057506008919050565b6003826053811115613a8457613a84614e0f565b03613a9157506010919050565b6004826053811115613aa557613aa5614e0f565b03613ab257506020919050565b6005826053811115613ac657613ac6614e0f565b03613ad357506040919050565b6006826053811115613ae757613ae7614e0f565b03613af457506080919050565b6007826053811115613b0857613b08614e0f565b03613b15575060a0919050565b6008826053811115613b2957613b29614e0f565b03613b375750610100919050565b8160405163be7830b160e01b81526004016104de9190615af2565b919050565b5f806040518060e0016040528060a98152602001615c9360a9913980519060200120845f0151805190602001208560200151604051602001613b999190615b18565b604051602081830303815290604052805190602001208660400151876060015188608001518960a00151604051602001613bd391906157bd565b60408051601f1981840301815282825280516020918201209083019890985281019590955260608501939093526001600160a01b03909116608084015260a083015260c082015260e0810191909152610100015b604051602081830303815290604052805190602001209050610c60838261413d565b60608167ffffffffffffffff811115613c6457613c64614830565b604051908082528060200260200182016040528015613c8d578160200160208202803683370190505b5090505f613cc084845f818110613ca657613ca6614f8b565b606002919091013560101c67ffffffffffffffff16919050565b604051635ff9d55d60e11b81526004810182905290915073d582ec82a1758322907df80da8a754e12a5acb959063bff3aaba90602401602060405180830381865afa158015613d11573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613d359190614f9f565b613d555760405163b6679c3b60e01b8152600481018290526024016104de565b5f805b84811015613e0c575f868683818110613d7357613d73614f8b565b606002919091013591505067ffffffffffffffff601082901c16848114613dbe57604051634ac8748b60e11b81526004810183905260248101829052604481018690526064016104de565b5f613dc8836139e2565b9050613dd381613a2e565b613de19061ffff168661584a565b945082878581518110613df657613df6614f8b565b6020908102919091010152505050600101613d58565b50610800811115613e3b5760405163e7f4895d60e01b81526108006004820152602481018290526044016104de565b505092915050565b5f806040518060c0016040528060878152602001615c0c6087913980519060200120845f0151805190602001208560200151604051602001613e859190615b18565b60405160208183030381529060405280519060200120866040015187606001518860800151604051602001613eba91906157bd565b60408051601f198184030181528282528051602091820120908301979097528101949094526060840192909252608083015260a082015260c081019190915260e001613c27565b5f613f0a6141d3565b905090565b5f805f8351604103613f46576020840151604085015160608601515f1a613f3888828585614246565b955095509550505050613f51565b505081515f91506002905b9250925092565b5f826003811115613f6b57613f6b614e0f565b03613f74575050565b6001826003811115613f8857613f88614e0f565b03613fa65760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115613fba57613fba614e0f565b03613fdb5760405163fce698f760e01b8152600481018290526024016104de565b6003826003811115613fef57613fef614e0f565b03610d43576040516335e2f38360e21b8152600481018290526024016104de565b5f614019612771565b54600160401b900460ff16919050565b806001600160a01b03163b5f0361405e57604051634c9c8ce360e01b81526001600160a01b03821660048201526024016104de565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc805473ffffffffffffffffffffffffffffffffffffffff19166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516140c891906157bd565b5f60405180830381855af49150503d805f8114614100576040519150601f19603f3d011682016040523d82523d5f602084013e614105565b606091505b509150915061411585838361430e565b95945050505050565b3415610b9a5760405163b398979f60e01b815260040160405180910390fd5b5f807f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61416861436a565b6141706143e5565b6040805160208101949094528301919091526060820152608081018590523060a082015260c001604051602081830303815290604052805190602001209050610c60818460405161190160f01b8152600281019290925260228201526042902090565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6141fd61436a565b6142056143e5565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561427f57505f91506003905082614304565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa1580156142d0573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b0381166142fb57505f925060019150829050614304565b92505f91508190505b9450945094915050565b6060826143235761431e8261443a565b613a27565b815115801561433a57506001600160a01b0384163b155b1561436357604051639996b31560e01b81526001600160a01b03851660048201526024016104de565b5092915050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10081614395612be1565b8051909150156143ad57805160209091012092915050565b815480156143bc579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10081614410612cb4565b80519091501561442857805160209091012092915050565b600182015480156143bc579392505050565b80511561444957805160208201fd5b60405163d6bda27560e01b815260040160405180910390fd5b828054828255905f5260205f2090810192821561449b579160200282015b8281111561449b578251825591602001919060010190614480565b506144a792915061453b565b5090565b6040518060c001604052805f6001600160a01b0316815260200160608152602001606081526020016144ee60405180604001604052805f81526020015f81525090565b815260200160608152602001606081525090565b828054828255905f5260205f2090810192821561449b579160200282015b8281111561449b578235825591602001919060010190614520565b5b808211156144a7575f815560010161453c565b5f8083601f84011261455f575f80fd5b50813567ffffffffffffffff811115614576575f80fd5b60208301915083602082850101111561458d575f80fd5b9250929050565b5f805f805f805f6080888a0312156145aa575f80fd5b87359650602088013567ffffffffffffffff808211156145c8575f80fd5b6145d48b838c0161454f565b909850965060408a01359150808211156145ec575f80fd5b6145f88b838c0161454f565b909650945060608a0135915080821115614610575f80fd5b5061461d8a828b0161454f565b989b979a50959850939692959293505050565b5f60208284031215614640575f80fd5b5035919050565b602080825282518282018190525f9190848201906040850190845b818110156146875783516001600160a01b031683529284019291840191600101614662565b50909695505050505050565b5f5b838110156146ad578181015183820152602001614695565b50505f910152565b5f81518084526146cc816020860160208601614693565b601f01601f19169290920160200192915050565b602081525f613a2760208301846146b5565b5f8083601f840112614702575f80fd5b50813567ffffffffffffffff811115614719575f80fd5b6020830191508360208260051b850101111561458d575f80fd5b5f805f8060408587031215614746575f80fd5b843567ffffffffffffffff8082111561475d575f80fd5b614769888389016146f2565b90965094506020870135915080821115614781575f80fd5b5061478e8782880161454f565b95989497509550505050565b5f8083601f8401126147aa575f80fd5b50813567ffffffffffffffff8111156147c1575f80fd5b60208301915083602060608302850101111561458d575f80fd5b5f805f80604085870312156147ee575f80fd5b843567ffffffffffffffff80821115614805575f80fd5b6147698883890161479a565b6001600160a01b0381168114612983575f80fd5b8035613b5281614811565b634e487b7160e01b5f52604160045260245ffd5b6040516080810167ffffffffffffffff8111828210171561486757614867614830565b60405290565b604051601f8201601f1916810167ffffffffffffffff8111828210171561489657614896614830565b604052919050565b5f67ffffffffffffffff8211156148b7576148b7614830565b50601f01601f191660200190565b5f80604083850312156148d6575f80fd5b82356148e181614811565b9150602083013567ffffffffffffffff8111156148fc575f80fd5b8301601f8101851361490c575f80fd5b803561491f61491a8261489e565b61486d565b818152866020838501011115614933575f80fd5b816020840160208301375f602083830101528093505050509250929050565b5f8083601f840112614962575f80fd5b50813567ffffffffffffffff811115614979575f80fd5b6020830191508360208260061b850101111561458d575f80fd5b5f805f80604085870312156149a6575f80fd5b843567ffffffffffffffff808211156149bd575f80fd5b61476988838901614952565b60ff60f81b881681525f602060e060208401526149e960e084018a6146b5565b83810360408501526149fb818a6146b5565b606085018990526001600160a01b038816608086015260a0850187905284810360c0860152855180825260208088019350909101905f5b81811015614a4e57835183529284019291840191600101614a32565b50909c9b505050505050505050505050565b5f60408284031215614a70575f80fd5b50919050565b5f805f805f805f805f805f6101208c8e031215614a91575f80fd5b67ffffffffffffffff808d351115614aa7575f80fd5b614ab48e8e358f01614952565b909c509a50614ac68e60208f01614a60565b9950614ad58e60608f01614a60565b98508060a08e01351115614ae7575f80fd5b614af78e60a08f01358f01614a60565b97508060c08e01351115614b09575f80fd5b614b198e60c08f01358f0161454f565b909750955060e08d0135811015614b2e575f80fd5b614b3e8e60e08f01358f0161454f565b90955093506101008d0135811015614b54575f80fd5b50614b668d6101008e01358e0161454f565b81935080925050509295989b509295989b9093969950565b5f805f805f805f805f805f806101008d8f031215614b9a575f80fd5b67ffffffffffffffff8d351115614baf575f80fd5b614bbc8e8e358f0161479a565b909c509a50614bcd60208e01614825565b995067ffffffffffffffff60408e01351115614be7575f80fd5b614bf78e60408f01358f0161454f565b909950975067ffffffffffffffff60608e01351115614c14575f80fd5b614c248e60608f01358f016146f2565b9097509550614c368e60808f01614a60565b945067ffffffffffffffff60c08e01351115614c50575f80fd5b614c608e60c08f01358f0161454f565b909450925067ffffffffffffffff60e08e01351115614c7d575f80fd5b614c8d8e60e08f01358f0161454f565b81935080925050509295989b509295989b509295989b565b5f805f805f805f805f805f6101008c8e031215614cc0575f80fd5b67ffffffffffffffff808d351115614cd6575f80fd5b614ce38e8e358f01614952565b909c509a50614cf58e60208f01614a60565b99508060608e01351115614d07575f80fd5b614d178e60608f01358f01614a60565b9850614d2560808e01614825565b97508060a08e01351115614d37575f80fd5b614d478e60a08f01358f0161454f565b909750955060c08d0135811015614d5c575f80fd5b614d6c8e60c08f01358f0161454f565b909550935060e08d0135811015614d81575f80fd5b50614b668d60e08e01358e0161454f565b5f805f805f60608688031215614da6575f80fd5b8535614db181614811565b9450602086013567ffffffffffffffff80821115614dcd575f80fd5b614dd989838a01614952565b90965094506040880135915080821115614df1575f80fd5b50614dfe8882890161454f565b969995985093965092949392505050565b634e487b7160e01b5f52602160045260245ffd5b600181811c90821680614e3757607f821691505b602082108103614a7057634e487b7160e01b5f52602260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b8181038181111561237c5761237c614e55565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b878152608060208201525f614ebd60808301888a614e7c565b8281036040840152614ed0818789614e7c565b90508281036060840152614ee5818587614e7c565b9a9950505050505050505050565b5f8551614f04818460208a01614693565b61103b60f11b9083019081528551614f23816002840160208a01614693565b808201915050601760f91b8060028301528551614f47816003850160208a01614693565b60039201918201528351614f62816004840160208801614693565b016004019695505050505050565b5f60208284031215614f80575f80fd5b8151613a2781614811565b634e487b7160e01b5f52603260045260245ffd5b5f60208284031215614faf575f80fd5b81518015158114613a27575f80fd5b601f821115612a5557805f5260205f20601f840160051c81016020851015614fe35750805b601f840160051c820191505b818110156132a0575f8155600101614fef565b67ffffffffffffffff83111561501a5761501a614830565b61502e836150288354614e23565b83614fbe565b5f601f84116001811461505f575f85156150485750838201355b5f19600387901b1c1916600186901b1783556132a0565b5f83815260208120601f198716915b8281101561508e578685013582556020948501946001909201910161506e565b50868210156150aa575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b608081525f6150cf60808301898b614e7c565b82810360208401526150e281888a614e7c565b90506001600160a01b03861660408401528281036060840152614ee5818587614e7c565b606081525f615119606083018789614e7c565b60208382038185015281875480845282840191506005838260051b8601018a5f52845f205f5b848110156151d257601f198884030186525f825461515c81614e23565b808652600182811680156151775760018114615190576151bb565b60ff198416888d0152821515891b88018c0194506151bb565b865f528b5f205f5b848110156151b35781548a82018f0152908301908d01615198565b89018d019550505b50988a01989295505050919091019060010161513f565b505087810360408901526151e7818a8c614e7c565b9d9c50505050505050505050505050565b5f808335601e1984360301811261520d575f80fd5b83018035915067ffffffffffffffff821115615227575f80fd5b6020019150600581901b360382131561458d575f80fd5b5f6040828403121561524e575f80fd5b6040516040810181811067ffffffffffffffff8211171561527157615271614830565b604052823581526020928301359281019290925250919050565b5f6040828403121561529b575f80fd5b613a27838361523e565b5f602082840312156152b5575f80fd5b8135613a2781614811565b6001600160a01b038481168252604060208084018290529083018490525f91859160608501845b8781101561530e5784356152fa81614811565b8416825293820193908201906001016152e7565b5098975050505050505050565b5f815180845260208085019450602084015f5b8381101561534a5781518752958201959082019060010161532e565b509495945050505050565b602081525f613a27602083018461531b565b5f67ffffffffffffffff82111561538057615380614830565b5060051b60200190565b5f602080838503121561539b575f80fd5b825167ffffffffffffffff808211156153b2575f80fd5b818501915085601f8301126153c5575f80fd5b81516153d361491a82615367565b81815260059190911b830184019084810190888311156153f1575f80fd5b8585015b8381101561530e5780518581111561540b575f80fd5b86016080818c03601f19011215615420575f80fd5b615428614844565b8882015181526040808301518a830152606083015181830152608083015188811115615452575f80fd5b8084019350508c603f840112615466575f80fd5b8983015161547661491a82615367565b81815260059190911b84018201908b8101908f831115615494575f80fd5b948301945b828610156154be57855193506154ae84614811565b838252948c0194908c0190615499565b60608501525050508452509186019186016153f5565b5f600182016154e5576154e5614e55565b5060010190565b815167ffffffffffffffff81111561550657615506614830565b61551a816155148454614e23565b84614fbe565b602080601f83116001811461554d575f84156155365750858301515b5f19600386901b1c1916600185901b1785556155a4565b5f85815260208120601f198616915b8281101561557b5788860151825594840194600190910190840161555c565b508582101561559857878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b5f815180845260208085019450602084015f5b8381101561534a5781516001600160a01b0316875295820195908201906001016155bf565b8051825260208101516020830152604081015160408301525f606082015160806060850152610c6060808501826155ac565b5f8282518085526020808601955060208260051b840101602086015f5b8481101561566157601f1986840301895261564f8383516155e4565b98840198925090830190600101615633565b5090979650505050505050565b608081525f6156806080830189615616565b6001600160a01b038816602084015282810360408401526156a2818789614e7c565b905082810360608401526156b7818587614e7c565b9998505050505050505050565b608081525f615680608083018961531b565b8183525f7f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115615706575f80fd5b8260051b80836020870137939093016020019392505050565b602081525f610c606020830184866156d6565b604081525f6157446040830186615616565b828103602084015261229e818587614e7c565b604081525f61576a6040830186886156d6565b828103602084015261577d818587614e7c565b979650505050505050565b81515f9082906020808601845b838110156157b157815185529382019390820190600101615795565b50929695505050505050565b5f82516157ce818460208701614693565b9190910192915050565b5f602082840312156157e8575f80fd5b5051919050565b5f80858511156157fd575f80fd5b83861115615809575f80fd5b5050820193919092039150565b8035602083101561237c575f19602084900360031b1b1692915050565b808202811582820484141761237c5761237c614e55565b8082018082111561237c5761237c614e55565b82815260608101613a27602083018480518252602090810151910152565b602081525f610c60602083018486614e7c565b604081525f6158a060408301856155e4565b828103602084015261411581856155e4565b8183525f60208085019450825f5b8581101561534a5781358752828201356158d981614811565b6001600160a01b0390811688850152604090838201356158f881614811565b169088015260609687019691909101906001016158c0565b6001600160a01b0381511682525f602082015160e0602085015261593760e08501826146b5565b90506040830151848203604086015261595082826155ac565b915050606083015161596f606086018280518252602090810151910152565b50608083015184820360a086015261598782826146b5565b91505060a083015184820360c086015261411582826146b5565b606081525f6159b36060830187615616565b82810360208401526159c68186886158b2565b9050828103604084015261577d8185615910565b604081525f6159ed6040830185876158b2565b828103602084015261229e8185615910565b5f82601f830112615a0e575f80fd5b8151615a1c61491a8261489e565b818152846020838601011115615a30575f80fd5b610c60826020830160208701614693565b5f60208284031215615a51575f80fd5b815167ffffffffffffffff80821115615a68575f80fd5b9083019060808286031215615a7b575f80fd5b615a83614844565b8251615a8e81614811565b81526020830151615a9e81614811565b6020820152604083015182811115615ab4575f80fd5b615ac0878286016159ff565b604083015250606083015182811115615ad7575f80fd5b615ae3878286016159ff565b60608301525095945050505050565b6020810160548310615b1257634e487b7160e01b5f52602160045260245ffd5b91905290565b81515f9082906020808601845b838110156157b15781516001600160a01b031685529382019390820190600101615b2556fe5573657244656372797074526573706f6e7365566572696669636174696f6e286279746573207075626c69634b65792c627974657333325b5d20637448616e646c65732c6279746573207573657244656372797074656453686172652c627974657320657874726144617461295075626c696344656372797074566572696669636174696f6e28627974657333325b5d20637448616e646c65732c627974657320646563727970746564526573756c742c62797465732065787472614461746129557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c6279746573206578747261446174612944656c656761746564557365724465637279707452657175657374566572696669636174696f6e286279746573207075626c69634b65792c616464726573735b5d20636f6e74726163744164647265737365732c616464726573732064656c656761746f72416464726573732c75696e7432353620737461727454696d657374616d702c75696e74323536206475726174696f6e446179732c6279746573206578747261446174612968113e68af494c6efd0210fc4bf9ba748d1ffadaa4718217fdf63548c4aee700
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01xW_5`\xE0\x1C\x80co\x89\x13\xBC\x11a\0\xD1W\x80c\xB4\xDE,7\x11a\0|W\x80c\xF1\xB5z\xDB\x11a\0WW\x80c\xF1\xB5z\xDB\x14a\x04KW\x80c\xFA!\x06\xB8\x14a\x04jW\x80c\xFB\xB82Y\x14a\x04~W_\x80\xFD[\x80c\xB4\xDE,7\x14a\x03\xEEW\x80c\xD8\x99\x8FE\x14a\x04\rW\x80c\xE2-\x1B&\x14a\x04,W_\x80\xFD[\x80c\x84\xB0\x19n\x11a\0\xACW\x80c\x84\xB0\x19n\x14a\x03`W\x80c\x9F\xADZ/\x14a\x03\x87W\x80c\xAD<\xB1\xCC\x14a\x03\xA6W_\x80\xFD[\x80co\x89\x13\xBC\x14a\x03\x0EW\x80cv\"~\xED\x14a\x03-W\x80c\x84V\xCBY\x14a\x03LW_\x80\xFD[\x80c@\x14\xC4\xCD\x11a\x011W\x80cR\xD1\x90-\x11a\x01\x0CW\x80cR\xD1\x90-\x14a\x02|W\x80cX\xF5\xB8\xAB\x14a\x02\x9EW\x80c\\\x97Z\xBB\x14a\x02\xD8W_\x80\xFD[\x80c@\x14\xC4\xCD\x14a\x02\x1BW\x80cA\x0B\xF0\xBA\x14a\x02JW\x80cO\x1E\xF2\x86\x14a\x02iW_\x80\xFD[\x80c\r\x8En,\x11a\x01aW\x80c\r\x8En,\x14a\x01\xD2W\x80c9\xF78\x10\x14a\x01\xF3W\x80c?K\xA8:\x14a\x02\x07W_\x80\xFD[\x80c\x04o\x9E\xB3\x14a\x01|W\x80c\t\0\xCCi\x14a\x01\x9DW[_\x80\xFD[4\x80\x15a\x01\x87W_\x80\xFD[Pa\x01\x9Ba\x01\x966`\x04aE\x94V[a\x04\x9DV[\0[4\x80\x15a\x01\xA8W_\x80\xFD[Pa\x01\xBCa\x01\xB76`\x04aF0V[a\x08\x06V[`@Qa\x01\xC9\x91\x90aFGV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xDDW_\x80\xFD[Pa\x01\xE6a\x08\xD2V[`@Qa\x01\xC9\x91\x90aF\xE0V[4\x80\x15a\x01\xFEW_\x80\xFD[Pa\x01\x9Ba\t:V[4\x80\x15a\x02\x12W_\x80\xFD[Pa\x01\x9Ba\n\xC9V[4\x80\x15a\x02&W_\x80\xFD[Pa\x02:a\x0256`\x04aG3V[a\x0B\x9CV[`@Q\x90\x15\x15\x81R` \x01a\x01\xC9V[4\x80\x15a\x02UW_\x80\xFD[Pa\x02:a\x02d6`\x04aG\xDBV[a\x0ChV[a\x01\x9Ba\x02w6`\x04aH\xC5V[a\r(V[4\x80\x15a\x02\x87W_\x80\xFD[Pa\x02\x90a\rGV[`@Q\x90\x81R` \x01a\x01\xC9V[4\x80\x15a\x02\xA9W_\x80\xFD[Pa\x02:a\x02\xB86`\x04aF0V[_\x90\x81R_\x80Q` a]<\x839\x81Q\x91R` R`@\x90 T`\xFF\x16\x90V[4\x80\x15a\x02\xE3W_\x80\xFD[P\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x16a\x02:V[4\x80\x15a\x03\x19W_\x80\xFD[Pa\x01\x9Ba\x03(6`\x04aE\x94V[a\ruV[4\x80\x15a\x038W_\x80\xFD[Pa\x02:a\x03G6`\x04aI\x93V[a\x10\x8BV[4\x80\x15a\x03WW_\x80\xFD[Pa\x01\x9Ba\x11KV[4\x80\x15a\x03kW_\x80\xFD[Pa\x03ta\x12\x05V[`@Qa\x01\xC9\x97\x96\x95\x94\x93\x92\x91\x90aI\xC9V[4\x80\x15a\x03\x92W_\x80\xFD[Pa\x01\x9Ba\x03\xA16`\x04aJvV[a\x12\xC9V[4\x80\x15a\x03\xB1W_\x80\xFD[Pa\x01\xE6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[4\x80\x15a\x03\xF9W_\x80\xFD[Pa\x01\x9Ba\x04\x086`\x04aK~V[a\x18iV[4\x80\x15a\x04\x18W_\x80\xFD[Pa\x01\x9Ba\x04'6`\x04aG3V[a\x1A\x11V[4\x80\x15a\x047W_\x80\xFD[Pa\x02:a\x04F6`\x04aI\x93V[a\x1C\x0FV[4\x80\x15a\x04VW_\x80\xFD[Pa\x01\x9Ba\x04e6`\x04aL\xA5V[a\x1C\xCFV[4\x80\x15a\x04uW_\x80\xFD[Pa\x01\x9Ba!\xE0V[4\x80\x15a\x04\x89W_\x80\xFD[Pa\x02:a\x04\x986`\x04aM\x92V[a\"\x91V[_\x80Q` a]<\x839\x81Q\x91R`\x01`\xF9\x1B\x88\x11\x15\x80a\x04\xC1WP\x80`\x08\x01T\x88\x11[\x15a\x04\xE7W`@QcjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x01[`@Q\x80\x91\x03\x90\xFD[_\x88\x81R`\x07\x82\x01` R`@\x80\x82 \x81Q\x80\x83\x01\x90\x92R\x80T\x82\x90\x82\x90a\x05\x0E\x90aN#V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05:\x90aN#V[\x80\x15a\x05\x85W\x80`\x1F\x10a\x05\\Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x85V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05hW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05\xDBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\xC7W[PPPPP\x81RPP\x90P_`@Q\x80`\x80\x01`@R\x80\x83_\x01Q\x81R` \x01\x83` \x01Q\x81R` \x01\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x89\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x87\x81R\x91\x81\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91Pa\x06\x85\x90P\x82a\"\xA8V[_\x8C\x81R`\t\x86\x01` R`@\x81 T\x91\x92Pa\x06\xA2\x88\x88a#\x82V[\x90P\x81_\x03a\x06\xB3W\x80\x91Pa\x06\xE4V[\x81\x81\x14a\x06\xE4W`@QcU\xDA\xFAC`\xE1\x1B\x81R`\x04\x81\x01\x8E\x90R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R`d\x01a\x04\xDEV[Pa\x06\xF2\x81\x8D\x84\x8C\x8Ca%KV[_\x8C\x81R`\x02\x86\x01` \x90\x81R`@\x80\x83 \x83\x80R\x82R\x82 \x80T`\x01\x81\x81\x01\x83U\x82\x85R\x92\x90\x93 \x90\x92\x01\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x163\x17\x90U\x81T\x8E\x91\x7F\x7F\xCD\xFBS\x81\x91\x7FUJq}\nTp\xA3?ZI\xBAdE\xF0^\xC4<t\xC0\xBC,\xC6\x08\xB2\x91a\x07k\x91\x90aNiV[\x8E\x8E\x8E\x8E\x8E\x8E`@Qa\x07\x84\x97\x96\x95\x94\x93\x92\x91\x90aN\xA4V[`@Q\x80\x91\x03\x90\xA2_\x8D\x81R` \x87\x90R`@\x90 T`\xFF\x16\x15\x80\x15a\x07\xB2WP\x80Ta\x07\xB2\x90\x83\x90a&8V[\x15a\x07\xF7W_\x8D\x81R` \x87\x90R`@\x80\x82 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x8E\x91\x7F\xE8\x97R\xBE\x0E\xCD\xB6\x8B*n\xB5\xEF\x1A\x89\x109\xE0\xE9*\xE3\xC8\xA6\"t\xC5\x88\x1EH\xEE\xA1\xED%\x91\xA2[PPPPPPPPPPPPPV[_\x81\x81R\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x03` \x90\x81R`@\x80\x83 T\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x02\x83R\x81\x84 \x81\x85R\x83R\x92\x81\x90 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94_\x80Q` a]<\x839\x81Q\x91R\x94\x90\x93\x92\x91\x90\x83\x01\x82\x82\x80\x15a\x08\xC4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\xA6W[PPPPP\x92PPP\x91\x90PV[```@Q\x80`@\x01`@R\x80`\n\x81R` \x01i\"2\xB1\xB9<\xB8:4\xB7\xB7`\xB1\x1B\x81RPa\t\0_a&\xBAV[a\t\n`\x07a&\xBAV[a\t\x13_a&\xBAV[`@Q` \x01a\t&\x94\x93\x92\x91\x90aN\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\tBa'XV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14a\tlW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08_a\twa'qV[\x80T\x90\x91P`\x01`@\x1B\x90\x04`\xFF\x16\x80a\t\x9FWP\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x91\x16\x10\x15[\x15a\t\xBDW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"2\xB1\xB9<\xB8:4\xB7\xB7`\xB1\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\n!\x91a'\x99V[a\n)a'\xABV[`\x01`\xF8\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06U`\x01`\xF9\x1B\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08U\x80Th\xFF\0\0\0\0\0\0\0\0\x19\x16\x81U`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPV[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x19W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B=\x91\x90aOpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x15\x80\x15a\x0BrWP3s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x14\x15[\x15a\x0B\x92W`@Qcp\xC8\xB3w`\xE1\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\xDEV[a\x0B\x9Aa'\xB3V[V[_\x83\x81\x03a\x0B\xABWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x0B\xE1Wa\x0B\xE1aO\x8BV[\x90P` \x02\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\x06\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C!W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CE\x91\x90aO\x9FV[a\x0CRW_\x91PPa\x0C`V[`\x01\x01a\x0B\xADV[P`\x01\x90P[\x94\x93PPPPV[_\x83\x81\x03a\x0CwWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x0C\xADWa\x0C\xADaO\x8BV[\x90P``\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xD4\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x13\x91\x90aO\x9FV[a\r W_\x91PPa\x0C`V[`\x01\x01a\x0CyV[a\r0a(%V[a\r9\x82a(\xDCV[a\rC\x82\x82a)\x86V[PPV[_a\rPa*ZV[P\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x90V[_\x80Q` a]<\x839\x81Q\x91R`\x01`\xF8\x1B\x88\x11\x15\x80a\r\x99WP\x80`\x06\x01T\x88\x11[\x15a\r\xBAW`@QcjE|\xA1`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x01a\x04\xDEV[`@\x80Q_\x8A\x81R`\x05\x84\x01` \x90\x81R\x83\x82 \x80T`\x80\x92\x81\x02\x85\x01\x83\x01\x90\x95R``\x84\x01\x85\x81R\x92\x94\x84\x93\x92\x84\x01\x82\x82\x80\x15a\x0E\x15W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0E\x01W[PPPPP\x81R` \x01\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`@\x80Q` `\x1F\x88\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x86\x81R\x91\x81\x01\x91\x90\x87\x90\x87\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x91Pa\x0E\x9F\x90P\x82a*\xA3V[_\x8B\x81R`\t\x85\x01` R`@\x81 T\x91\x92Pa\x0E\xBC\x87\x87a#\x82V[\x90P\x81_\x03a\x0E\xCDW\x80\x91Pa\x0E\xFEV[\x81\x81\x14a\x0E\xFEW`@QcU\xDA\xFAC`\xE1\x1B\x81R`\x04\x81\x01\x8D\x90R`$\x81\x01\x83\x90R`D\x81\x01\x82\x90R`d\x01a\x04\xDEV[a\x0F\x0B\x82\x8D\x85\x8C\x8Ca%KV[_\x8C\x81R`\x04\x86\x01` \x90\x81R`@\x80\x83 \x86\x84R\x82R\x82 \x80T`\x01\x81\x01\x82U\x81\x84R\x91\x90\x92 \x01a\x0F?\x8A\x8C\x83aP\x02V[P\x85`\x02\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x85\x81R` \x01\x90\x81R` \x01_ 3\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x8C\x7FM{\x1D\xBAI\xE9\xE8F!^\x16!\xF5s|\x81\xD8aLO&\x84\x94\xD8\xB7\x87c,NY\xF0\xE5\x8D\x8D\x8D\x8D3\x8E\x8E`@Qa\x0F\xE2\x97\x96\x95\x94\x93\x92\x91\x90aP\xBCV[`@Q\x80\x91\x03\x90\xA2_\x8D\x81R` \x87\x90R`@\x90 T`\xFF\x16\x15\x80\x15a\x10\x10WP\x80Ta\x10\x10\x90\x84\x90a+JV[\x15a\x07\xF7W_\x8D\x81R` \x87\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x03\x89\x01\x90\x91R\x90\x81\x90 \x85\x90UQ\x8D\x90\x7F\xD7\xE5\x8A6z\nl)\x8Ev\xAD]$\0\x04\xE3'\xAA\x14#\xCB\xE4\xBD\x7F\xF8]Lq^\xF8\xD1_\x90a\x10t\x90\x8F\x90\x8F\x90\x86\x90\x8E\x90\x8E\x90aQ\x06V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPV[_\x83\x81\x03a\x10\x9AWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x10\xD0Wa\x10\xD0aO\x8BV[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xF7\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x12W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x116\x91\x90aO\x9FV[a\x11CW_\x91PPa\x0C`V[`\x01\x01a\x10\x9CV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x98W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xBC\x91\x90aO\x9FV[\x15\x80\x15a\x11\xDDWP3s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x14\x15[\x15a\x11\xFDW`@Qc8\x89\x16\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\xDEV[a\x0B\x9Aa+\x86V[_``\x80\x82\x80\x80\x83\x81\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x80T\x90\x91P\x15\x80\x15a\x12CWP`\x01\x81\x01T\x15[a\x12\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01R\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\xDEV[a\x12\x97a+\xE1V[a\x12\x9Fa,\xB4V[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x91\x99PF\x98P0\x97P\x95P\x93P\x91PPV[a\x12\xD1a-\x05V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R\x875`\x04\x82\x01\x81\x90R\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\xF3\xAA\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13F\x91\x90aO\x9FV[a\x13fW`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`@Qcfb\x86\xDD`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xCC\xC5\r\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xB4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD8\x91\x90aO\x9FV[\x15a\x13\xF9W`@Qc\x18\r\x9A1`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[a\x14\x06` \x89\x01\x89aQ\xF8V[\x90P_\x03a\x14'W`@QcW\xCF\xA2\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\na\x146` \x8A\x01\x8AaQ\xF8V[\x90P\x11\x15a\x14rW`\na\x14M` \x8A\x01\x8AaQ\xF8V[`@Qc\xAF\x1F\x04\x95`\xE0\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01R`$\x83\x01RP`D\x01a\x04\xDEV[a\x14\x89a\x14\x846\x8C\x90\x03\x8C\x01\x8CaR\x8BV[a-HV[a\x14\xDCa\x14\x99` \x8A\x01\x8AaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x14\xD7\x92PPP` \x8C\x01\x8CaR\xA5V[a.\x14V[\x15a\x15\x17Wa\x14\xEE` \x8A\x01\x8AaR\xA5V[a\x14\xFB` \x8A\x01\x8AaQ\xF8V[`@Qc\xC3Dj\xC7`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x93\x92\x91\x90aR\xC0V[_a\x15#\x8D\x8D\x8Ba.mV[\x90P_`@Q\x80`\xC0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x15{\x90\x8D\x01\x8DaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x15\xC0\x90\x8E\x01\x8EaR\xA5V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8D_\x015\x81R` \x01\x8D` \x015\x81R` \x01\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x90Pa\x167\x81a\x16.`@\x8E\x01` \x8F\x01aR\xA5V[\x89\x89\x8E5a0dV[P`@Qc\xA1O\x89q`\xE0\x1B\x81R_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a\x16q\x90\x85\x90`\x04\x01aSUV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x16\xB2\x91\x90\x81\x01\x90aS\x8AV[\x90Pa\x16\xBD\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a\x16\xF9\x83aT\xD4V[\x90\x91UPP`\x08\x81\x01T`@\x80Q``` `\x1F\x8E\x01\x81\x90\x04\x02\x82\x01\x81\x01\x83R\x91\x81\x01\x8C\x81R\x90\x91\x82\x91\x90\x8E\x90\x8E\x90\x81\x90\x85\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP` \x91\x82\x01\x87\x90R\x83\x81R`\x07\x85\x01\x90\x91R`@\x90 \x81Q\x81\x90a\x17c\x90\x82aT\xECV[P` \x82\x81\x01Q\x80Qa\x17|\x92`\x01\x85\x01\x92\x01\x90aDbV[P\x90PP_a\x17\x8B\x88\x88a#\x82V[\x90Pa\x17\x96\x81a1\xA9V[_\x82\x81R`\t\x84\x01` R`@\x90 Ua\x17\xAF3a2;V[\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8F` \x01` \x81\x01\x90a\x17\xE5\x91\x90aR\xA5V[\x8E\x8E\x8C\x8C`@Qa\x17\xFB\x96\x95\x94\x93\x92\x91\x90aVnV[`@Q\x80\x91\x03\x90\xA2\x80\x7F\xC7\x1E2\xC8\x0F1\t\xC6s\xA9\xAD\xFC\xDE\xD0\xB3\xB2\t2\x12\xAB(@\x84\xA0\xE0:(\x1D\xEE+\xDD_\x85\x8F` \x01` \x81\x01\x90a\x189\x91\x90aR\xA5V[\x8E\x8E\x8C\x8C`@Qa\x18O\x96\x95\x94\x93\x92\x91\x90aV\xC4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPV[a\x18qa-\x05V[_\x8B\x90\x03a\x18\x92W`@Qc$\x0E\x93\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\n\x86\x11\x15a\x18\xBEW`@Qc\xAF\x1F\x04\x95`\xE0\x1B\x81R`\n`\x04\x82\x01R`$\x81\x01\x87\x90R`D\x01a\x04\xDEV[a\x18\xD5a\x18\xD06\x87\x90\x03\x87\x01\x87aR\x8BV[a2\xA7V[a\x18\xDDaD\xABV[`\x01`\x01`\xA0\x1B\x03\x8B\x16\x81R`@\x80Q` `\x1F\x8C\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8A\x81R\x90\x8B\x90\x8B\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP` \x80\x83\x01\x91\x90\x91R`@\x80Q\x89\x83\x02\x81\x81\x01\x84\x01\x90\x92R\x89\x81R\x91\x8A\x91\x8A\x91\x82\x91\x90\x85\x01\x90\x84\x90\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP`@\x82\x01Ra\x19g6\x87\x90\x03\x87\x01\x87aR\x8BV[``\x82\x01R`@\x80Q` `\x1F\x85\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x83\x81R\x90\x84\x90\x84\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPPP`\x80\x82\x01R`@\x80Q` `\x1F\x87\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x85\x81R\x90\x86\x90\x86\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP`\xA0\x86\x01\x94\x90\x94RPa\x19\xEA\x91P\x85\x90P\x84a#\x82V[\x90Pa\x19\xF53a2;V[a\x1A\x01\x8E\x8E\x84\x84a3~V[PPPPPPPPPPPPPPV[a\x1A\x19a-\x05V[_\x83\x90\x03a\x1A:W`@Qc\x05\xBC\xEA\x87`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Au\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5?\x92PPPV[`@Qc\xA1O\x89q`\xE0\x1B\x81R_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a\x1A\xB0\x90\x88\x90\x88\x90`\x04\x01aW\x1FV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xCAW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1A\xF1\x91\x90\x81\x01\x90aS\x8AV[\x90Pa\x1A\xFC\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x06\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a\x1B8\x83aT\xD4V[\x90\x91UPP`\x06\x81\x01T_\x81\x81R`\x05\x83\x01` R`@\x90 a\x1B\\\x90\x88\x88aE\x02V[P_a\x1Bh\x86\x86a#\x82V[\x90Pa\x1Bs\x81a1\xA9V[_\x82\x81R`\t\x84\x01` R`@\x90 Ua\x1B\x8C3a5\xC6V[\x80\x7F\"\xDBH\n9\xBDrUd8\xAA\xDBJ2\xA3\xD2\xA6c\x8B\x87\xC0;\xBE\xC5\xFE\xF6\x99~\x10\x95\x87\xFF\x84\x87\x87`@Qa\x1B\xC0\x93\x92\x91\x90aW2V[`@Q\x80\x91\x03\x90\xA2\x80\x7F\x9B\xB0\x88\xA5?\x1F\xE1>\xC8O\xA0\xA1\xD7\xB5x\xB5\x89\xE1\x93&\x8B\xAA\xCC\xC5\xE2?z\xB7\x85\xAA=\xAE\x88\x88\x88\x88`@Qa\x1B\xFE\x94\x93\x92\x91\x90aWWV[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[_\x83\x81\x03a\x1C\x1EWP_a\x0C`V[_[\x84\x81\x10\x15a\x0CZWs\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAhc-\xDC\x9Ao\x87\x87\x84\x81\x81\x10a\x1CTWa\x1CTaO\x8BV[\x90P`@\x02\x01_\x015`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1C{\x91\x81R` \x01\x90V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x96W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xBA\x91\x90aO\x9FV[a\x1C\xC7W_\x91PPa\x0C`V[`\x01\x01a\x1C V[a\x1C\xD7a-\x05V[`@Qc_\xF9\xD5]`\xE1\x1B\x81R\x885`\x04\x82\x01\x81\x90R\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\xF3\xAA\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1DL\x91\x90aO\x9FV[a\x1DlW`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`@Qcfb\x86\xDD`\xE1\x1B\x81R`\x04\x81\x01\x82\x90Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xCC\xC5\r\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xBAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xDE\x91\x90aO\x9FV[\x15a\x1D\xFFW`@Qc\x18\r\x9A1`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[a\x1E\x0C` \x8A\x01\x8AaQ\xF8V[\x90P_\x03a\x1E-W`@QcW\xCF\xA2\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\na\x1E<` \x8B\x01\x8BaQ\xF8V[\x90P\x11\x15a\x1ESW`\na\x14M` \x8B\x01\x8BaQ\xF8V[a\x1Eea\x14\x846\x8C\x90\x03\x8C\x01\x8CaR\x8BV[a\x1E\xADa\x1Eu` \x8B\x01\x8BaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8C\x92Pa.\x14\x91PPV[\x15a\x1E\xDCW\x87a\x1E\xC0` \x8B\x01\x8BaQ\xF8V[`@Qc\xDCMx\xB1`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x93\x92\x91\x90aR\xC0V[_a\x1E\xE8\x8D\x8D\x8Ca.mV[\x90P_`@Q\x80`\xA0\x01`@R\x80\x8A\x8A\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x1F@\x90\x8E\x01\x8EaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP\x8D5` \x80\x83\x01\x91\x90\x91R\x8E\x81\x015`@\x80\x84\x01\x91\x90\x91R\x80Q`\x1F\x89\x01\x83\x90\x04\x83\x02\x81\x01\x83\x01\x90\x91R\x87\x81R``\x90\x92\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x91RP\x90Pa\x1F\xD2\x81\x8B\x89\x89\x8F5a6\x06V[`@Qc\xA1O\x89q`\xE0\x1B\x81R_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a \x0B\x90\x86\x90`\x04\x01aSUV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a %W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra L\x91\x90\x81\x01\x90aS\x8AV[\x90Pa W\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a \x93\x83aT\xD4V[\x90\x91UPP`\x08\x81\x01T`@\x80Q``` `\x1F\x8F\x01\x81\x90\x04\x02\x82\x01\x81\x01\x83R\x91\x81\x01\x8D\x81R\x90\x91\x82\x91\x90\x8F\x90\x8F\x90\x81\x90\x85\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP` \x91\x82\x01\x88\x90R\x83\x81R`\x07\x85\x01\x90\x91R`@\x90 \x81Q\x81\x90a \xFD\x90\x82aT\xECV[P` \x82\x81\x01Q\x80Qa!\x16\x92`\x01\x85\x01\x92\x01\x90aDbV[P\x90PP_a!%\x89\x89a#\x82V[\x90Pa!0\x81a1\xA9V[_\x82\x81R`\t\x84\x01` R`@\x90 Ua!I3a2;V[\x80\x7F\xF9\x01\x1B\xD6\xBA\r\xA6\x04\x9CR\rp\xFEYq\xF1~\xD7\xAByT\x86\x05%D\xB5\x10\x19\x89lYk\x84\x8F\x8F\x8F\x8D\x8D`@Qa!\x83\x96\x95\x94\x93\x92\x91\x90aVnV[`@Q\x80\x91\x03\x90\xA2\x80\x7F\xC7\x1E2\xC8\x0F1\t\xC6s\xA9\xAD\xFC\xDE\xD0\xB3\xB2\t2\x12\xAB(@\x84\xA0\xE0:(\x1D\xEE+\xDD_\x86\x8F\x8F\x8F\x8D\x8D`@Qa!\xC5\x96\x95\x94\x93\x92\x91\x90aV\xC4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPPPPPPPPPPV[`\x08_a!\xEBa'qV[\x80T\x90\x91P`\x01`@\x1B\x90\x04`\xFF\x16\x80a\"\x13WP\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x91\x16\x10\x15[\x15a\"1W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17`\x01`@\x1B\x17h\xFF\0\0\0\0\0\0\0\0\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\n\xBDV[_a\"\x9E\x85\x85\x85\x85a\x1C\x0FV[\x96\x95PPPPPPV[_a#|`@Q\x80`\xA0\x01`@R\x80`m\x81R` \x01a[K`m\x919\x80Q\x90` \x01 \x83_\x01Q\x80Q\x90` \x01 \x84` \x01Q`@Q` \x01a\"\xEC\x91\x90aW\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85`@\x01Q\x80Q\x90` \x01 \x86``\x01Q`@Q` \x01a##\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x96\x90\x96R\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a6\x11V[\x92\x91PPV[_\x81\x81\x03a$\x05Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xDAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xFE\x91\x90aW\xD8V[\x90Pa#|V[_\x83\x83_\x81\x81\x10a$\x18Wa$\x18aO\x8BV[\x91\x90\x91\x015`\xF8\x1C\x91PP_\x81\x90\x03a$\xA7Ws\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a${W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$\x9F\x91\x90aW\xD8V[\x91PPa#|V[\x80`\xFF\x16`\x01\x14\x80a$\xBCWP\x80`\xFF\x16`\x02\x14[\x15a%-W`!\x83\x10\x15a$\xEDW`@QcI\xAAE3`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`!`$\x82\x01R`D\x01a\x04\xDEV[a$\xFB`!`\x01\x85\x87aW\xEFV[a%\x04\x91aX\x16V[\x91P_\x82\x90\x03a%'W`@Qc\xCB\x17\xB7\xA5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Pa#|V[`@Qc\x08Ns\x0B`\xE2\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x04\xDEV[__\x80Q` a]<\x839\x81Q\x91R\x90P_a%\x9C\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa6=\x92PPPV[\x90Pa%\xA9\x87\x823a6eV[_\x86\x81R`\x01\x83\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a&\0W`@Qc\x99\xECH\xD9`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x04\xDEV[_\x95\x86R`\x01\x91\x82\x01` \x90\x81R`@\x80\x88 `\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x88R\x91\x90R\x90\x94 \x80T`\xFF\x19\x16\x90\x94\x17\x90\x93UPPPPV[`@Qc\x14\x0FE\xFF`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R_\x90\x81\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c(\x1E\x8B\xFE\x90`$\x01[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x8BW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&\xAF\x91\x90aW\xD8V[\x90\x92\x10\x15\x93\x92PPPV[``_a&\xC6\x83a7\xD3V[`\x01\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&\xE5Wa&\xE5aH0V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a'\x0FW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a'\x19W[P\x93\x92PPPV[_a'aa'qV[Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90PV[_\x80\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0a#|V[a'\xA1a8\xB4V[a\rC\x82\x82a8\xD9V[a\x0B\x9Aa8\xB4V[a'\xBBa9KV[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x80T`\xFF\x19\x16\x81U\x7F]\xB9\xEE\nI[\xF2\xE6\xFF\x9C\x91\xA7\x83L\x1B\xA4\xFD\xD2D\xA5\xE8\xAANS{\xD3\x8A\xEA\xE4\xB0s\xAA3[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA1PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a(\xBEWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a(\xB2\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0B\x9AW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a),W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)P\x91\x90aOpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a)\x83W`@Qc\x0EV\xCF=`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x04\xDEV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a)\xE0WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra)\xDD\x91\x81\x01\x90aW\xD8V[`\x01[a*\x08W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\xDEV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x81\x14a*KW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[a*U\x83\x83a9\x8DV[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0B\x9AW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a#|`@Q\x80`\x80\x01`@R\x80`T\x81R` \x01a[\xB8`T\x919\x80Q` \x91\x82\x01 \x84Q`@Q\x91\x92a*\xD9\x92\x01aW\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x01Q\x80Q\x90` \x01 \x85`@\x01Q`@Q` \x01a+\x10\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x95\x90\x95R\x81\x01\x92\x90\x92R``\x82\x01R`\x80\x81\x01\x91\x90\x91R`\xA0\x01a#aV[`@Qca\xD5U-`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R_\x90\x81\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xC3\xAA\xAAZ\x90`$\x01a&pV[a+\x8Ea-\x05V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0\x80T`\xFF\x19\x16`\x01\x17\x81U\x7Fb\xE7\x8C\xEA\x01\xBE\xE3 \xCDNB\x02p\xB5\xEAt\0\r\x11\xB0\xC9\xF7GT\xEB\xDB\xFCTK\x05\xA2X3a(\x07V[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x80T``\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x91a,2\x90aN#V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta,^\x90aN#V[\x80\x15a,\xA9W\x80`\x1F\x10a,\x80Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a,\xA9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a,\x8CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x80T``\x91\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x91a,2\x90aN#V[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x16\x15a\x0B\x9AW`@Qc\xD9<\x06e`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q_\x03a-lW`@Qc\xDE(Y\xC1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x01m\x10\x15a-\xA4W` \x81\x01Q`@Qc2\x95\x18c`\xE0\x1B\x81Ra\x01m`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x04\xDEV[\x80QB\x10\x15a-\xD2W\x80Q`@Qc\xF2L\x08\x87`\xE0\x1B\x81RB`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x04\xDEV[B\x81` \x01Qb\x01Q\x80a-\xE6\x91\x90aX3V[\x82Qa-\xF2\x91\x90aXJV[\x10\x15a)\x83WB\x81`@Qb\xC0\xD2\x01`\xE6\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX]V[_\x80[\x83Q\x81\x10\x15a.dW\x82`\x01`\x01`\xA0\x1B\x03\x16\x84\x82\x81Q\x81\x10a.<Wa.<aO\x8BV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a.\\W`\x01\x91PPa#|V[`\x01\x01a.\x17V[P_\x93\x92PPPV[``_\x83\x90\x03a.\x90W`@Qc\xA6\xA6\xCB!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.\xA9Wa.\xA9aH0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a.\xD2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x84\x81\x10\x15a05W_\x86\x86\x83\x81\x81\x10a.\xF3Wa.\xF3aO\x8BV[\x90P`@\x02\x01_\x015\x90P_\x87\x87\x84\x81\x81\x10a/\x11Wa/\x11aO\x8BV[\x90P`@\x02\x01` \x01` \x81\x01\x90a/)\x91\x90aR\xA5V[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x10\x83\x90\x1C\x16\x865\x81\x14a/lW`@QcJ\xC8t\x8B`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x82\x90R\x875`D\x82\x01R`d\x01a\x04\xDEV[_a/v\x84a9\xE2V[\x90Pa/\x81\x81a:.V[a/\x8F\x90a\xFF\xFF\x16\x87aXJV[\x95Pa/\xD9a/\xA1` \x8A\x01\x8AaQ\xF8V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa.\x14\x91PPV[a0\x07W\x82a/\xEB` \x8A\x01\x8AaQ\xF8V[`@Qc\xA4\xC3\x03\x91`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x93\x92\x91\x90aR\xC0V[\x83\x87\x86\x81Q\x81\x10a0\x1AWa0\x1AaO\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x90\x92\x01\x91Pa.\xD8\x90PV[Pa\x08\0\x81\x11\x15a'PW`@Qc\xE7\xF4\x89]`\xE0\x1B\x81Ra\x08\0`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x04\xDEV[_a0o\x86\x83a;WV[\x90P_a0\xB1\x82\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa6=\x92PPPV[\x90P\x85`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14a0\xE9W\x84\x84`@Qc*\x87='`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX{V[PPPPPPPV[`\x01\x81Q\x11a0\xFEWPV[_\x81_\x81Q\x81\x10a1\x11Wa1\x11aO\x8BV[` \x02` \x01\x01Q` \x01Q\x90P_`\x01\x90P[\x82Q\x81\x10\x15a*UW\x81\x83\x82\x81Q\x81\x10a1AWa1AaO\x8BV[` \x02` \x01\x01Q` \x01Q\x14a1\xA1W\x82_\x81Q\x81\x10a1dWa1daO\x8BV[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a1~Wa1~aO\x8BV[` \x02` \x01\x01Q`@Qc\xCF\xAE\x92\x1F`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX\x8EV[`\x01\x01a1%V[`@Qc\x17\xF3b\xD9`\xE3\x1B\x81R`\x04\x81\x01\x82\x90Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\x9B\x16\xC8\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xF7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2\x1B\x91\x90aO\x9FV[a)\x83W`@Qcw\xDD\xBE\x81`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`@Qc\x98\x8A--`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\x81z(_\x1F\xCA;\xB4\x08L\xBF\xC7}K\xAB\xC28\xAD`\x9C\x90c\x98\x8A--\x90`$\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a2\x8EW_\x80\xFD[PZ\xF1\x15\x80\x15a2\xA0W=_\x80>=_\xFD[PPPPPV[\x80` \x01Q_\x03a2\xCBW`@Qc\x12)\xE27`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2\xDAa\x01mb\x01Q\x80aX3V[\x81` \x01Q\x11\x15a3\x1BWa2\xF4a\x01mb\x01Q\x80aX3V[` \x82\x01Q`@QcW)u\x89`\xE1\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x04\xDEV[\x80QB\x10\x15a3IW\x80Q`@Qc\xF2L\x08\x87`\xE0\x1B\x81RB`\x04\x82\x01R`$\x81\x01\x91\x90\x91R`D\x01a\x04\xDEV[` \x81\x01Q\x81QB\x91a3[\x91aXJV[\x10\x15a)\x83WB\x81`@Qc3\xC7\xE7\xE7`\xE1\x1B\x81R`\x04\x01a\x04\xDE\x92\x91\x90aX]V[_a3\x89\x85\x85a<IV[`@Qc\xA1O\x89q`\xE0\x1B\x81R\x90\x91P_\x90s\xC7\xD4Va\xA3E\xEC\\\xA0\xE8R\x1C\xFE\xF7\xE3/\xDA\r\xAAh\x90c\xA1O\x89q\x90a3\xC5\x90\x85\x90`\x04\x01aSUV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3\xDFW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra4\x06\x91\x90\x81\x01\x90aS\x8AV[\x90Pa4\x11\x81a0\xF2V[\x7Fh\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\x08\x80T_\x80Q` a]<\x839\x81Q\x91R\x91_a4M\x83aT\xD4V[\x90\x91UPP`\x08\x81\x01T`@\x80Q\x80\x82\x01\x82R` \x80\x89\x01Q\x82R\x80\x82\x01\x87\x90R_\x84\x81R`\x07\x86\x01\x90\x91R\x91\x90\x91 \x81Q\x81\x90a4\x8B\x90\x82aT\xECV[P` \x82\x81\x01Q\x80Qa4\xA4\x92`\x01\x85\x01\x92\x01\x90aDbV[PPP_\x81\x81R`\t\x83\x01` R`@\x90\x81\x90 \x86\x90UQ\x81\x90\x7F\x1F\x80\xA4{Q\x97\x987\x97o\x99\x9Aw5\xFD\xCC\xBB\xE5p\xE0\xD4\0\x81dN\xC8\x8F\x8E\xD7l\x96\x12\x90a4\xF1\x90\x86\x90\x8C\x90\x8C\x90\x8C\x90aY\xA1V[`@Q\x80\x91\x03\x90\xA2\x80\x7F\x8E\xD4\xFA\xB8\xB1\xD0Pgl\xD7\xD7\xD9\xFAD\x8D]\"\x113u\xFA\xB8\xADM\xC6\x90\xE5E\x93\xC5'\xBF\x89\x89\x89`@Qa5-\x93\x92\x91\x90aY\xDAV[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[_\x80[\x82Q\x81\x10\x15a5\x97W_\x83\x82\x81Q\x81\x10a5^Wa5^aO\x8BV[` \x02` \x01\x01Q\x90P_a5r\x82a9\xE2V[\x90Pa5}\x81a:.V[a5\x8B\x90a\xFF\xFF\x16\x85aXJV[\x93PPP`\x01\x01a5BV[Pa\x08\0\x81\x11\x15a\rCW`@Qc\xE7\xF4\x89]`\xE0\x1B\x81Ra\x08\0`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x04\xDEV[`@Qc${\xAC\x9F`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Rs\x81z(_\x1F\xCA;\xB4\x08L\xBF\xC7}K\xAB\xC28\xAD`\x9C\x90c\x91\xEE\xB2|\x90`$\x01a2wV[_a0o\x86\x83a>CV[_a#|a6\x1Da?\x01V[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80_\x80a6K\x86\x86a?\x0FV[\x92P\x92P\x92Pa6[\x82\x82a?XV[P\x90\x94\x93PPPPV[`@Qc%\x11\xF3\xF5`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01Rs\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\x94G\xCF\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xC2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xE6\x91\x90aO\x9FV[a7\x0EW`@Qc\x15>7{`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x04\xDEV[`@Qc\x06?\xE89`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`$\x83\x01R\x83\x16\x90s\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c1\xFFA\xC8\x90`D\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7nW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra7\x95\x91\x90\x81\x01\x90aZAV[` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14a*UW`@Qc\r\x86\xF5!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x04\xDEV[_\x80z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10a8\x1BWz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a8GWm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a8eWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a8}Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a8\x91Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a8\xA3W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a#|W`\x01\x01\x92\x91PPV[a8\xBCa@\x10V[a\x0B\x9AW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a8\xE1a8\xB4V[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02a9-\x84\x82aT\xECV[P`\x03\x81\x01a9<\x83\x82aT\xECV[P_\x80\x82U`\x01\x90\x91\x01UPPV[\x7F\xCD^\xD1\\n\x18~w\xE9\xAE\xE8\x81\x84\xC2\x1FO!\x82\xABX'\xCB;~\x07\xFB\xED\xCDc\xF03\0T`\xFF\x16a\x0B\x9AW`@Qc\x8D\xFC +`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a9\x96\x82a@)V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a9\xDAWa*U\x82\x82a@\xACV[a\rCaA\x1EV[_`\x08\x82\x90\x1C`\xFF\x16`S\x81\x11\x15a:\x12W`@Qcd\x19P\xD7`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x04\xDEV[\x80`\xFF\x16`S\x81\x11\x15a:'Wa:'aN\x0FV[\x93\x92PPPV[_\x80\x82`S\x81\x11\x15a:BWa:BaN\x0FV[\x03a:OWP`\x02\x91\x90PV[`\x02\x82`S\x81\x11\x15a:cWa:caN\x0FV[\x03a:pWP`\x08\x91\x90PV[`\x03\x82`S\x81\x11\x15a:\x84Wa:\x84aN\x0FV[\x03a:\x91WP`\x10\x91\x90PV[`\x04\x82`S\x81\x11\x15a:\xA5Wa:\xA5aN\x0FV[\x03a:\xB2WP` \x91\x90PV[`\x05\x82`S\x81\x11\x15a:\xC6Wa:\xC6aN\x0FV[\x03a:\xD3WP`@\x91\x90PV[`\x06\x82`S\x81\x11\x15a:\xE7Wa:\xE7aN\x0FV[\x03a:\xF4WP`\x80\x91\x90PV[`\x07\x82`S\x81\x11\x15a;\x08Wa;\x08aN\x0FV[\x03a;\x15WP`\xA0\x91\x90PV[`\x08\x82`S\x81\x11\x15a;)Wa;)aN\x0FV[\x03a;7WPa\x01\0\x91\x90PV[\x81`@Qc\xBEx0\xB1`\xE0\x1B\x81R`\x04\x01a\x04\xDE\x91\x90aZ\xF2V[\x91\x90PV[_\x80`@Q\x80`\xE0\x01`@R\x80`\xA9\x81R` \x01a\\\x93`\xA9\x919\x80Q\x90` \x01 \x84_\x01Q\x80Q\x90` \x01 \x85` \x01Q`@Q` \x01a;\x99\x91\x90a[\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q\x89`\xA0\x01Q`@Q` \x01a;\xD3\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x98\x90\x98R\x81\x01\x95\x90\x95R``\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x81\x01\x91\x90\x91Ra\x01\0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0C`\x83\x82aA=V[``\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<dWa<daH0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_a<\xC0\x84\x84_\x81\x81\x10a<\xA6Wa<\xA6aO\x8BV[``\x02\x91\x90\x91\x015`\x10\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90PV[`@Qc_\xF9\xD5]`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xD5\x82\xEC\x82\xA1u\x83\"\x90}\xF8\r\xA8\xA7T\xE1*Z\xCB\x95\x90c\xBF\xF3\xAA\xBA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\x11W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=5\x91\x90aO\x9FV[a=UW`@Qc\xB6g\x9C;`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[_\x80[\x84\x81\x10\x15a>\x0CW_\x86\x86\x83\x81\x81\x10a=sWa=saO\x8BV[``\x02\x91\x90\x91\x015\x91PPg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x10\x82\x90\x1C\x16\x84\x81\x14a=\xBEW`@QcJ\xC8t\x8B`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x81\x01\x86\x90R`d\x01a\x04\xDEV[_a=\xC8\x83a9\xE2V[\x90Pa=\xD3\x81a:.V[a=\xE1\x90a\xFF\xFF\x16\x86aXJV[\x94P\x82\x87\x85\x81Q\x81\x10a=\xF6Wa=\xF6aO\x8BV[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a=XV[Pa\x08\0\x81\x11\x15a>;W`@Qc\xE7\xF4\x89]`\xE0\x1B\x81Ra\x08\0`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x01a\x04\xDEV[PP\x92\x91PPV[_\x80`@Q\x80`\xC0\x01`@R\x80`\x87\x81R` \x01a\\\x0C`\x87\x919\x80Q\x90` \x01 \x84_\x01Q\x80Q\x90` \x01 \x85` \x01Q`@Q` \x01a>\x85\x91\x90a[\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86`@\x01Q\x87``\x01Q\x88`\x80\x01Q`@Q` \x01a>\xBA\x91\x90aW\xBDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x90\x83\x01\x97\x90\x97R\x81\x01\x94\x90\x94R``\x84\x01\x92\x90\x92R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x81\x01\x91\x90\x91R`\xE0\x01a<'V[_a?\naA\xD3V[\x90P\x90V[_\x80_\x83Q`A\x03a?FW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1Aa?8\x88\x82\x85\x85aBFV[\x95P\x95P\x95PPPPa?QV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15a?kWa?kaN\x0FV[\x03a?tWPPV[`\x01\x82`\x03\x81\x11\x15a?\x88Wa?\x88aN\x0FV[\x03a?\xA6W`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15a?\xBAWa?\xBAaN\x0FV[\x03a?\xDBW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[`\x03\x82`\x03\x81\x11\x15a?\xEFWa?\xEFaN\x0FV[\x03a\rCW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x04\xDEV[_a@\x19a'qV[T`\x01`@\x1B\x90\x04`\xFF\x16\x91\x90PV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a@^W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x04\xDEV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa@\xC8\x91\x90aW\xBDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aA\0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aA\x05V[``\x91P[P\x91P\x91PaA\x15\x85\x83\x83aC\x0EV[\x95\x94PPPPPV[4\x15a\x0B\x9AW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaAhaCjV[aApaC\xE5V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x81\x01\x85\x90R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0C`\x81\x84`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaA\xFDaCjV[aB\x05aC\xE5V[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15aB\x7FWP_\x91P`\x03\x90P\x82aC\x04V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aB\xD0W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aB\xFBWP_\x92P`\x01\x91P\x82\x90PaC\x04V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aC#WaC\x1E\x82aD:V[a:'V[\x81Q\x15\x80\x15aC:WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aCcW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x04\xDEV[P\x92\x91PPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x81aC\x95a+\xE1V[\x80Q\x90\x91P\x15aC\xADW\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15aC\xBCW\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x81aD\x10a,\xB4V[\x80Q\x90\x91P\x15aD(W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15aC\xBCW\x93\x92PPPV[\x80Q\x15aDIW\x80Q` \x82\x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\x9BW\x91` \x02\x82\x01[\x82\x81\x11\x15aD\x9BW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aD\x80V[PaD\xA7\x92\x91PaE;V[P\x90V[`@Q\x80`\xC0\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01aD\xEE`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aD\x9BW\x91` \x02\x82\x01[\x82\x81\x11\x15aD\x9BW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90aE V[[\x80\x82\x11\x15aD\xA7W_\x81U`\x01\x01aE<V[_\x80\x83`\x1F\x84\x01\x12aE_W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEvW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x80_`\x80\x88\x8A\x03\x12\x15aE\xAAW_\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aE\xC8W_\x80\xFD[aE\xD4\x8B\x83\x8C\x01aEOV[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15aE\xECW_\x80\xFD[aE\xF8\x8B\x83\x8C\x01aEOV[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15aF\x10W_\x80\xFD[PaF\x1D\x8A\x82\x8B\x01aEOV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[_` \x82\x84\x03\x12\x15aF@W_\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\x87W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aFbV[P\x90\x96\x95PPPPPPV[_[\x83\x81\x10\x15aF\xADW\x81\x81\x01Q\x83\x82\x01R` \x01aF\x95V[PP_\x91\x01RV[_\x81Q\x80\x84RaF\xCC\x81` \x86\x01` \x86\x01aF\x93V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a:'` \x83\x01\x84aF\xB5V[_\x80\x83`\x1F\x84\x01\x12aG\x02W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\x19W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[_\x80_\x80`@\x85\x87\x03\x12\x15aGFW_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aG]W_\x80\xFD[aGi\x88\x83\x89\x01aF\xF2V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15aG\x81W_\x80\xFD[PaG\x8E\x87\x82\x88\x01aEOV[\x95\x98\x94\x97P\x95PPPPV[_\x80\x83`\x1F\x84\x01\x12aG\xAAW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xC1W_\x80\xFD[` \x83\x01\x91P\x83` ``\x83\x02\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[_\x80_\x80`@\x85\x87\x03\x12\x15aG\xEEW_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aH\x05W_\x80\xFD[aGi\x88\x83\x89\x01aG\x9AV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)\x83W_\x80\xFD[\x805a;R\x81aH\x11V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aHgWaHgaH0V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aH\x96WaH\x96aH0V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aH\xB7WaH\xB7aH0V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x80`@\x83\x85\x03\x12\x15aH\xD6W_\x80\xFD[\x825aH\xE1\x81aH\x11V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xFCW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x0CW_\x80\xFD[\x805aI\x1FaI\x1A\x82aH\x9EV[aHmV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aI3W_\x80\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aIbW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIyW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aE\x8DW_\x80\xFD[_\x80_\x80`@\x85\x87\x03\x12\x15aI\xA6W_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aI\xBDW_\x80\xFD[aGi\x88\x83\x89\x01aIRV[`\xFF`\xF8\x1B\x88\x16\x81R_` `\xE0` \x84\x01RaI\xE9`\xE0\x84\x01\x8AaF\xB5V[\x83\x81\x03`@\x85\x01RaI\xFB\x81\x8AaF\xB5V[``\x85\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x88\x16`\x80\x86\x01R`\xA0\x85\x01\x87\x90R\x84\x81\x03`\xC0\x86\x01R\x85Q\x80\x82R` \x80\x88\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aJNW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aJ2V[P\x90\x9C\x9BPPPPPPPPPPPPV[_`@\x82\x84\x03\x12\x15aJpW_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x80_\x80_\x80_a\x01 \x8C\x8E\x03\x12\x15aJ\x91W_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8D5\x11\x15aJ\xA7W_\x80\xFD[aJ\xB4\x8E\x8E5\x8F\x01aIRV[\x90\x9CP\x9APaJ\xC6\x8E` \x8F\x01aJ`V[\x99PaJ\xD5\x8E``\x8F\x01aJ`V[\x98P\x80`\xA0\x8E\x015\x11\x15aJ\xE7W_\x80\xFD[aJ\xF7\x8E`\xA0\x8F\x015\x8F\x01aJ`V[\x97P\x80`\xC0\x8E\x015\x11\x15aK\tW_\x80\xFD[aK\x19\x8E`\xC0\x8F\x015\x8F\x01aEOV[\x90\x97P\x95P`\xE0\x8D\x015\x81\x10\x15aK.W_\x80\xFD[aK>\x8E`\xE0\x8F\x015\x8F\x01aEOV[\x90\x95P\x93Pa\x01\0\x8D\x015\x81\x10\x15aKTW_\x80\xFD[PaKf\x8Da\x01\0\x8E\x015\x8E\x01aEOV[\x81\x93P\x80\x92PPP\x92\x95\x98\x9BP\x92\x95\x98\x9B\x90\x93\x96\x99PV[_\x80_\x80_\x80_\x80_\x80_\x80a\x01\0\x8D\x8F\x03\x12\x15aK\x9AW_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D5\x11\x15aK\xAFW_\x80\xFD[aK\xBC\x8E\x8E5\x8F\x01aG\x9AV[\x90\x9CP\x9APaK\xCD` \x8E\x01aH%V[\x99Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x8E\x015\x11\x15aK\xE7W_\x80\xFD[aK\xF7\x8E`@\x8F\x015\x8F\x01aEOV[\x90\x99P\x97Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x8E\x015\x11\x15aL\x14W_\x80\xFD[aL$\x8E``\x8F\x015\x8F\x01aF\xF2V[\x90\x97P\x95PaL6\x8E`\x80\x8F\x01aJ`V[\x94Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xC0\x8E\x015\x11\x15aLPW_\x80\xFD[aL`\x8E`\xC0\x8F\x015\x8F\x01aEOV[\x90\x94P\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xE0\x8E\x015\x11\x15aL}W_\x80\xFD[aL\x8D\x8E`\xE0\x8F\x015\x8F\x01aEOV[\x81\x93P\x80\x92PPP\x92\x95\x98\x9BP\x92\x95\x98\x9BP\x92\x95\x98\x9BV[_\x80_\x80_\x80_\x80_\x80_a\x01\0\x8C\x8E\x03\x12\x15aL\xC0W_\x80\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8D5\x11\x15aL\xD6W_\x80\xFD[aL\xE3\x8E\x8E5\x8F\x01aIRV[\x90\x9CP\x9APaL\xF5\x8E` \x8F\x01aJ`V[\x99P\x80``\x8E\x015\x11\x15aM\x07W_\x80\xFD[aM\x17\x8E``\x8F\x015\x8F\x01aJ`V[\x98PaM%`\x80\x8E\x01aH%V[\x97P\x80`\xA0\x8E\x015\x11\x15aM7W_\x80\xFD[aMG\x8E`\xA0\x8F\x015\x8F\x01aEOV[\x90\x97P\x95P`\xC0\x8D\x015\x81\x10\x15aM\\W_\x80\xFD[aMl\x8E`\xC0\x8F\x015\x8F\x01aEOV[\x90\x95P\x93P`\xE0\x8D\x015\x81\x10\x15aM\x81W_\x80\xFD[PaKf\x8D`\xE0\x8E\x015\x8E\x01aEOV[_\x80_\x80_``\x86\x88\x03\x12\x15aM\xA6W_\x80\xFD[\x855aM\xB1\x81aH\x11V[\x94P` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aM\xCDW_\x80\xFD[aM\xD9\x89\x83\x8A\x01aIRV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aM\xF1W_\x80\xFD[PaM\xFE\x88\x82\x89\x01aEOV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80aN7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aJpWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a#|Wa#|aNUV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x87\x81R`\x80` \x82\x01R_aN\xBD`\x80\x83\x01\x88\x8AaN|V[\x82\x81\x03`@\x84\x01RaN\xD0\x81\x87\x89aN|V[\x90P\x82\x81\x03``\x84\x01RaN\xE5\x81\x85\x87aN|V[\x9A\x99PPPPPPPPPPV[_\x85QaO\x04\x81\x84` \x8A\x01aF\x93V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaO#\x81`\x02\x84\x01` \x8A\x01aF\x93V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaOG\x81`\x03\x85\x01` \x8A\x01aF\x93V[`\x03\x92\x01\x91\x82\x01R\x83QaOb\x81`\x04\x84\x01` \x88\x01aF\x93V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aO\x80W_\x80\xFD[\x81Qa:'\x81aH\x11V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aO\xAFW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a:'W_\x80\xFD[`\x1F\x82\x11\x15a*UW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aO\xE3WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a2\xA0W_\x81U`\x01\x01aO\xEFV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aP\x1AWaP\x1AaH0V[aP.\x83aP(\x83TaN#V[\x83aO\xBEV[_`\x1F\x84\x11`\x01\x81\x14aP_W_\x85\x15aPHWP\x83\x82\x015[_\x19`\x03\x87\x90\x1B\x1C\x19\x16`\x01\x86\x90\x1B\x17\x83Ua2\xA0V[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15aP\x8EW\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aPnV[P\x86\x82\x10\x15aP\xAAW_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[`\x80\x81R_aP\xCF`\x80\x83\x01\x89\x8BaN|V[\x82\x81\x03` \x84\x01RaP\xE2\x81\x88\x8AaN|V[\x90P`\x01`\x01`\xA0\x1B\x03\x86\x16`@\x84\x01R\x82\x81\x03``\x84\x01RaN\xE5\x81\x85\x87aN|V[``\x81R_aQ\x19``\x83\x01\x87\x89aN|V[` \x83\x82\x03\x81\x85\x01R\x81\x87T\x80\x84R\x82\x84\x01\x91P`\x05\x83\x82`\x05\x1B\x86\x01\x01\x8A_R\x84_ _[\x84\x81\x10\x15aQ\xD2W`\x1F\x19\x88\x84\x03\x01\x86R_\x82TaQ\\\x81aN#V[\x80\x86R`\x01\x82\x81\x16\x80\x15aQwW`\x01\x81\x14aQ\x90WaQ\xBBV[`\xFF\x19\x84\x16\x88\x8D\x01R\x82\x15\x15\x89\x1B\x88\x01\x8C\x01\x94PaQ\xBBV[\x86_R\x8B_ _[\x84\x81\x10\x15aQ\xB3W\x81T\x8A\x82\x01\x8F\x01R\x90\x83\x01\x90\x8D\x01aQ\x98V[\x89\x01\x8D\x01\x95PP[P\x98\x8A\x01\x98\x92\x95PPP\x91\x90\x91\x01\x90`\x01\x01aQ?V[PP\x87\x81\x03`@\x89\x01RaQ\xE7\x81\x8A\x8CaN|V[\x9D\x9CPPPPPPPPPPPPPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aR\rW_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aR'W_\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aE\x8DW_\x80\xFD[_`@\x82\x84\x03\x12\x15aRNW_\x80\xFD[`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aRqWaRqaH0V[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`@\x82\x84\x03\x12\x15aR\x9BW_\x80\xFD[a:'\x83\x83aR>V[_` \x82\x84\x03\x12\x15aR\xB5W_\x80\xFD[\x815a:'\x81aH\x11V[`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x82R`@` \x80\x84\x01\x82\x90R\x90\x83\x01\x84\x90R_\x91\x85\x91``\x85\x01\x84[\x87\x81\x10\x15aS\x0EW\x845aR\xFA\x81aH\x11V[\x84\x16\x82R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aR\xE7V[P\x98\x97PPPPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aSJW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aS.V[P\x94\x95\x94PPPPPV[` \x81R_a:'` \x83\x01\x84aS\x1BV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aS\x80WaS\x80aH0V[P`\x05\x1B` \x01\x90V[_` \x80\x83\x85\x03\x12\x15aS\x9BW_\x80\xFD[\x82Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aS\xB2W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12aS\xC5W_\x80\xFD[\x81QaS\xD3aI\x1A\x82aSgV[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x88\x83\x11\x15aS\xF1W_\x80\xFD[\x85\x85\x01[\x83\x81\x10\x15aS\x0EW\x80Q\x85\x81\x11\x15aT\x0BW_\x80\xFD[\x86\x01`\x80\x81\x8C\x03`\x1F\x19\x01\x12\x15aT W_\x80\xFD[aT(aHDV[\x88\x82\x01Q\x81R`@\x80\x83\x01Q\x8A\x83\x01R``\x83\x01Q\x81\x83\x01R`\x80\x83\x01Q\x88\x81\x11\x15aTRW_\x80\xFD[\x80\x84\x01\x93PP\x8C`?\x84\x01\x12aTfW_\x80\xFD[\x89\x83\x01QaTvaI\x1A\x82aSgV[\x81\x81R`\x05\x91\x90\x91\x1B\x84\x01\x82\x01\x90\x8B\x81\x01\x90\x8F\x83\x11\x15aT\x94W_\x80\xFD[\x94\x83\x01\x94[\x82\x86\x10\x15aT\xBEW\x85Q\x93PaT\xAE\x84aH\x11V[\x83\x82R\x94\x8C\x01\x94\x90\x8C\x01\x90aT\x99V[``\x85\x01RPPP\x84RP\x91\x86\x01\x91\x86\x01aS\xF5V[_`\x01\x82\x01aT\xE5WaT\xE5aNUV[P`\x01\x01\x90V[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aU\x06WaU\x06aH0V[aU\x1A\x81aU\x14\x84TaN#V[\x84aO\xBEV[` \x80`\x1F\x83\x11`\x01\x81\x14aUMW_\x84\x15aU6WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85UaU\xA4V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aU{W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aU\\V[P\x85\x82\x10\x15aU\x98W\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aSJW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aU\xBFV[\x80Q\x82R` \x81\x01Q` \x83\x01R`@\x81\x01Q`@\x83\x01R_``\x82\x01Q`\x80``\x85\x01Ra\x0C``\x80\x85\x01\x82aU\xACV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aVaW`\x1F\x19\x86\x84\x03\x01\x89RaVO\x83\x83QaU\xE4V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aV3V[P\x90\x97\x96PPPPPPPV[`\x80\x81R_aV\x80`\x80\x83\x01\x89aV\x16V[`\x01`\x01`\xA0\x1B\x03\x88\x16` \x84\x01R\x82\x81\x03`@\x84\x01RaV\xA2\x81\x87\x89aN|V[\x90P\x82\x81\x03``\x84\x01RaV\xB7\x81\x85\x87aN|V[\x99\x98PPPPPPPPPV[`\x80\x81R_aV\x80`\x80\x83\x01\x89aS\x1BV[\x81\x83R_\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15aW\x06W_\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[` \x81R_a\x0C`` \x83\x01\x84\x86aV\xD6V[`@\x81R_aWD`@\x83\x01\x86aV\x16V[\x82\x81\x03` \x84\x01Ra\"\x9E\x81\x85\x87aN|V[`@\x81R_aWj`@\x83\x01\x86\x88aV\xD6V[\x82\x81\x03` \x84\x01RaW}\x81\x85\x87aN|V[\x97\x96PPPPPPPV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aW\xB1W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aW\x95V[P\x92\x96\x95PPPPPPV[_\x82QaW\xCE\x81\x84` \x87\x01aF\x93V[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aW\xE8W_\x80\xFD[PQ\x91\x90PV[_\x80\x85\x85\x11\x15aW\xFDW_\x80\xFD[\x83\x86\x11\x15aX\tW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[\x805` \x83\x10\x15a#|W_\x19` \x84\x90\x03`\x03\x1B\x1B\x16\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a#|Wa#|aNUV[\x80\x82\x01\x80\x82\x11\x15a#|Wa#|aNUV[\x82\x81R``\x81\x01a:'` \x83\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[` \x81R_a\x0C`` \x83\x01\x84\x86aN|V[`@\x81R_aX\xA0`@\x83\x01\x85aU\xE4V[\x82\x81\x03` \x84\x01RaA\x15\x81\x85aU\xE4V[\x81\x83R_` \x80\x85\x01\x94P\x82_[\x85\x81\x10\x15aSJW\x815\x87R\x82\x82\x015aX\xD9\x81aH\x11V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x88\x85\x01R`@\x90\x83\x82\x015aX\xF8\x81aH\x11V[\x16\x90\x88\x01R``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aX\xC0V[`\x01`\x01`\xA0\x1B\x03\x81Q\x16\x82R_` \x82\x01Q`\xE0` \x85\x01RaY7`\xE0\x85\x01\x82aF\xB5V[\x90P`@\x83\x01Q\x84\x82\x03`@\x86\x01RaYP\x82\x82aU\xACV[\x91PP``\x83\x01QaYo``\x86\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P`\x80\x83\x01Q\x84\x82\x03`\xA0\x86\x01RaY\x87\x82\x82aF\xB5V[\x91PP`\xA0\x83\x01Q\x84\x82\x03`\xC0\x86\x01RaA\x15\x82\x82aF\xB5V[``\x81R_aY\xB3``\x83\x01\x87aV\x16V[\x82\x81\x03` \x84\x01RaY\xC6\x81\x86\x88aX\xB2V[\x90P\x82\x81\x03`@\x84\x01RaW}\x81\x85aY\x10V[`@\x81R_aY\xED`@\x83\x01\x85\x87aX\xB2V[\x82\x81\x03` \x84\x01Ra\"\x9E\x81\x85aY\x10V[_\x82`\x1F\x83\x01\x12aZ\x0EW_\x80\xFD[\x81QaZ\x1CaI\x1A\x82aH\x9EV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aZ0W_\x80\xFD[a\x0C`\x82` \x83\x01` \x87\x01aF\x93V[_` \x82\x84\x03\x12\x15aZQW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aZhW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15aZ{W_\x80\xFD[aZ\x83aHDV[\x82QaZ\x8E\x81aH\x11V[\x81R` \x83\x01QaZ\x9E\x81aH\x11V[` \x82\x01R`@\x83\x01Q\x82\x81\x11\x15aZ\xB4W_\x80\xFD[aZ\xC0\x87\x82\x86\x01aY\xFFV[`@\x83\x01RP``\x83\x01Q\x82\x81\x11\x15aZ\xD7W_\x80\xFD[aZ\xE3\x87\x82\x86\x01aY\xFFV[``\x83\x01RP\x95\x94PPPPPV[` \x81\x01`T\x83\x10a[\x12WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aW\xB1W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a[%V\xFEUserDecryptResponseVerification(bytes publicKey,bytes32[] ctHandles,bytes userDecryptedShare,bytes extraData)PublicDecryptVerification(bytes32[] ctHandles,bytes decryptedResult,bytes extraData)UserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,uint256 startTimestamp,uint256 durationDays,bytes extraData)DelegatedUserDecryptRequestVerification(bytes publicKey,address[] contractAddresses,address delegatorAddress,uint256 startTimestamp,uint256 durationDays,bytes extraData)h\x11>h\xAFILn\xFD\x02\x10\xFCK\xF9\xBAt\x8D\x1F\xFA\xDA\xA4q\x82\x17\xFD\xF65H\xC4\xAE\xE7\0",
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    /**Custom error with signature `DecryptionContextMismatch(uint256,uint256,uint256)` and selector `0xabb5f486`.
```solidity
error DecryptionContextMismatch(uint256 decryptionId, uint256 requestContextId, uint256 responseContextId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DecryptionContextMismatch {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub requestContextId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub responseContextId: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<DecryptionContextMismatch>
        for UnderlyingRustTuple<'_> {
            fn from(value: DecryptionContextMismatch) -> Self {
                (value.decryptionId, value.requestContextId, value.responseContextId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DecryptionContextMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    decryptionId: tuple.0,
                    requestContextId: tuple.1,
                    responseContextId: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DecryptionContextMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DecryptionContextMismatch(uint256,uint256,uint256)";
            const SELECTOR: [u8; 4] = [171u8, 181u8, 244u8, 134u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.requestContextId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.responseContextId),
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    /**Custom error with signature `HostChainDisabled(uint256)` and selector `0x603668c4`.
```solidity
error HostChainDisabled(uint256 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HostChainDisabled {
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
        impl ::core::convert::From<HostChainDisabled> for UnderlyingRustTuple<'_> {
            fn from(value: HostChainDisabled) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HostChainDisabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for HostChainDisabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "HostChainDisabled(uint256)";
            const SELECTOR: [u8; 4] = [96u8, 54u8, 104u8, 196u8];
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
    /**Custom error with signature `InvalidNullContextId()` and selector `0xcb17b7a5`.
```solidity
error InvalidNullContextId();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNullContextId;
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
        impl ::core::convert::From<InvalidNullContextId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNullContextId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNullContextId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNullContextId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNullContextId()";
            const SELECTOR: [u8; 4] = [203u8, 23u8, 183u8, 165u8];
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
        #[allow(dead_code)]
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
    pub struct PublicDecryptionRequest_0 {
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
        impl alloy_sol_types::SolEvent for PublicDecryptionRequest_0 {
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
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionRequest_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionRequest_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionRequest_0,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PublicDecryptionRequest(uint256,bytes32[],bytes)` and selector `0x9bb088a53f1fe13ec84fa0a1d7b578b589e193268baaccc5e23f7ab785aa3dae`.
```solidity
event PublicDecryptionRequest(uint256 indexed decryptionId, bytes32[] ctHandles, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PublicDecryptionRequest_1 {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for PublicDecryptionRequest_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "PublicDecryptionRequest(uint256,bytes32[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                155u8, 176u8, 136u8, 165u8, 63u8, 31u8, 225u8, 62u8, 200u8, 79u8, 160u8,
                161u8, 215u8, 181u8, 120u8, 181u8, 137u8, 225u8, 147u8, 38u8, 139u8,
                170u8, 204u8, 197u8, 226u8, 63u8, 122u8, 183u8, 133u8, 170u8, 61u8, 174u8,
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
                    ctHandles: data.0,
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
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandles),
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
        impl alloy_sol_types::private::IntoLogData for PublicDecryptionRequest_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PublicDecryptionRequest_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PublicDecryptionRequest_1,
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
    #[derive()]
    /**Event with signature `UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],(bytes32,address,address)[],(address,bytes,address[],(uint256,uint256),bytes,bytes))` and selector `0x1f80a47b51979837976f999a7735fdccbbe570e0d40081644ec88f8ed76c9612`.
```solidity
event UserDecryptionRequest(uint256 indexed decryptionId, SnsCiphertextMaterial[] snsCtMaterials, HandleEntry[] handles, IDecryption.UserDecryptionRequestPayload payload);
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
        pub payload: <IDecryption::UserDecryptionRequestPayload as alloy::sol_types::SolType>::RustType,
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
                IDecryption::UserDecryptionRequestPayload,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionRequest(uint256,(bytes32,uint256,bytes32,address[])[],(bytes32,address,address)[],(address,bytes,address[],(uint256,uint256),bytes,bytes))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                31u8, 128u8, 164u8, 123u8, 81u8, 151u8, 152u8, 55u8, 151u8, 111u8, 153u8,
                154u8, 119u8, 53u8, 253u8, 204u8, 187u8, 229u8, 112u8, 224u8, 212u8, 0u8,
                129u8, 100u8, 78u8, 200u8, 143u8, 142u8, 215u8, 108u8, 150u8, 18u8,
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
                    payload: data.2,
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
                    <IDecryption::UserDecryptionRequestPayload as alloy_sol_types::SolType>::tokenize(
                        &self.payload,
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
    /**Event with signature `UserDecryptionRequest(uint256,bytes32[],address,bytes,bytes)` and selector `0xc71e32c80f3109c673a9adfcded0b3b2093212ab284084a0e03a281dee2bdd5f`.
```solidity
event UserDecryptionRequest(uint256 indexed decryptionId, bytes32[] ctHandles, address userAddress, bytes publicKey, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionRequest_2 {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub ctHandles: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for UserDecryptionRequest_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
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
            const SIGNATURE: &'static str = "UserDecryptionRequest(uint256,bytes32[],address,bytes,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                199u8, 30u8, 50u8, 200u8, 15u8, 49u8, 9u8, 198u8, 115u8, 169u8, 173u8,
                252u8, 222u8, 208u8, 179u8, 178u8, 9u8, 50u8, 18u8, 171u8, 40u8, 64u8,
                132u8, 160u8, 224u8, 58u8, 40u8, 29u8, 238u8, 43u8, 221u8, 95u8,
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
                    ctHandles: data.0,
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
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.ctHandles),
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
        impl alloy_sol_types::private::IntoLogData for UserDecryptionRequest_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionRequest_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserDecryptionRequest_2,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Event with signature `UserDecryptionRequest(uint256,(bytes32,address,address)[],(address,bytes,address[],(uint256,uint256),bytes,bytes))` and selector `0x8ed4fab8b1d050676cd7d7d9fa448d5d22113375fab8ad4dc690e54593c527bf`.
```solidity
event UserDecryptionRequest(uint256 indexed decryptionId, HandleEntry[] handles, IDecryption.UserDecryptionRequestPayload payload);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UserDecryptionRequest_3 {
        #[allow(missing_docs)]
        pub decryptionId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub handles: alloy::sol_types::private::Vec<
            <HandleEntry as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub payload: <IDecryption::UserDecryptionRequestPayload as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for UserDecryptionRequest_3 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<HandleEntry>,
                IDecryption::UserDecryptionRequestPayload,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "UserDecryptionRequest(uint256,(bytes32,address,address)[],(address,bytes,address[],(uint256,uint256),bytes,bytes))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                142u8, 212u8, 250u8, 184u8, 177u8, 208u8, 80u8, 103u8, 108u8, 215u8,
                215u8, 217u8, 250u8, 68u8, 141u8, 93u8, 34u8, 17u8, 51u8, 117u8, 250u8,
                184u8, 173u8, 77u8, 198u8, 144u8, 229u8, 69u8, 147u8, 197u8, 39u8, 191u8,
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
                    handles: data.0,
                    payload: data.1,
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
                        HandleEntry,
                    > as alloy_sol_types::SolType>::tokenize(&self.handles),
                    <IDecryption::UserDecryptionRequestPayload as alloy_sol_types::SolType>::tokenize(
                        &self.payload,
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
        impl alloy_sol_types::private::IntoLogData for UserDecryptionRequest_3 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UserDecryptionRequest_3> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &UserDecryptionRequest_3,
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    /**Function with signature `reinitializeV7()` and selector `0xfa2106b8`.
```solidity
function reinitializeV7() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV7Call;
    ///Container type for the return parameters of the [`reinitializeV7()`](reinitializeV7Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV7Return {}
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
            impl ::core::convert::From<reinitializeV7Call> for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV7Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV7Call {
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
            impl ::core::convert::From<reinitializeV7Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: reinitializeV7Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for reinitializeV7Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl reinitializeV7Return {
            fn _tokenize(
                &self,
            ) -> <reinitializeV7Call as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for reinitializeV7Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV7Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV7()";
            const SELECTOR: [u8; 4] = [250u8, 33u8, 6u8, 184u8];
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
                reinitializeV7Return::_tokenize(ret)
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
            #[allow(dead_code)]
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
    #[derive(Clone)]
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
        reinitializeV7(reinitializeV7Call),
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
            [250u8, 33u8, 6u8, 184u8],
            [251u8, 184u8, 50u8, 89u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(userDecryptionResponse),
            ::core::stringify!(getDecryptionConsensusTxSenders),
            ::core::stringify!(getVersion),
            ::core::stringify!(initializeFromEmptyProxy),
            ::core::stringify!(unpause),
            ::core::stringify!(isPublicDecryptionReady),
            ::core::stringify!(isUserDecryptionReady_0),
            ::core::stringify!(upgradeToAndCall),
            ::core::stringify!(proxiableUUID),
            ::core::stringify!(isDecryptionDone),
            ::core::stringify!(paused),
            ::core::stringify!(publicDecryptionResponse),
            ::core::stringify!(isDelegatedUserDecryptionReady),
            ::core::stringify!(pause),
            ::core::stringify!(eip712Domain),
            ::core::stringify!(delegatedUserDecryptionRequest),
            ::core::stringify!(UPGRADE_INTERFACE_VERSION),
            ::core::stringify!(userDecryptionRequest_0),
            ::core::stringify!(publicDecryptionRequest),
            ::core::stringify!(isUserDecryptionReady_1),
            ::core::stringify!(userDecryptionRequest_1),
            ::core::stringify!(reinitializeV7),
            ::core::stringify!(isUserDecryptionReady_2),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <userDecryptionResponseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getDecryptionConsensusTxSendersCall as alloy_sol_types::SolCall>::SIGNATURE,
            <getVersionCall as alloy_sol_types::SolCall>::SIGNATURE,
            <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <unpauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isPublicDecryptionReadyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isUserDecryptionReady_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <upgradeToAndCallCall as alloy_sol_types::SolCall>::SIGNATURE,
            <proxiableUUIDCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isDecryptionDoneCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pausedCall as alloy_sol_types::SolCall>::SIGNATURE,
            <publicDecryptionResponseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isDelegatedUserDecryptionReadyCall as alloy_sol_types::SolCall>::SIGNATURE,
            <pauseCall as alloy_sol_types::SolCall>::SIGNATURE,
            <eip712DomainCall as alloy_sol_types::SolCall>::SIGNATURE,
            <delegatedUserDecryptionRequestCall as alloy_sol_types::SolCall>::SIGNATURE,
            <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SIGNATURE,
            <userDecryptionRequest_0Call as alloy_sol_types::SolCall>::SIGNATURE,
            <publicDecryptionRequestCall as alloy_sol_types::SolCall>::SIGNATURE,
            <isUserDecryptionReady_1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <userDecryptionRequest_1Call as alloy_sol_types::SolCall>::SIGNATURE,
            <reinitializeV7Call as alloy_sol_types::SolCall>::SIGNATURE,
            <isUserDecryptionReady_2Call as alloy_sol_types::SolCall>::SIGNATURE,
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
                Self::reinitializeV7(_) => {
                    <reinitializeV7Call as alloy_sol_types::SolCall>::SELECTOR
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
                    fn reinitializeV7(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV7Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV7)
                    }
                    reinitializeV7
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
                    fn reinitializeV7(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionCalls> {
                        <reinitializeV7Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionCalls::reinitializeV7)
                    }
                    reinitializeV7
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
                Self::reinitializeV7(inner) => {
                    <reinitializeV7Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::reinitializeV7(inner) => {
                    <reinitializeV7Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
    #[derive(Clone)]
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
        DecryptionContextMismatch(DecryptionContextMismatch),
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
        HostChainDisabled(HostChainDisabled),
        #[allow(missing_docs)]
        HostChainNotRegistered(HostChainNotRegistered),
        #[allow(missing_docs)]
        InvalidExtraDataLength(InvalidExtraDataLength),
        #[allow(missing_docs)]
        InvalidFHEType(InvalidFHEType),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        InvalidKmsContext(InvalidKmsContext),
        #[allow(missing_docs)]
        InvalidNullContextId(InvalidNullContextId),
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
            [72u8, 167u8, 136u8, 220u8],
            [76u8, 156u8, 140u8, 227u8],
            [82u8, 215u8, 37u8, 245u8],
            [87u8, 207u8, 162u8, 23u8],
            [96u8, 54u8, 104u8, 196u8],
            [100u8, 25u8, 80u8, 215u8],
            [103u8, 143u8, 207u8, 206u8],
            [111u8, 79u8, 115u8, 31u8],
            [119u8, 221u8, 190u8, 129u8],
            [141u8, 252u8, 32u8, 43u8],
            [147u8, 84u8, 138u8, 102u8],
            [149u8, 144u8, 233u8, 22u8],
            [153u8, 150u8, 179u8, 21u8],
            [153u8, 236u8, 72u8, 217u8],
            [164u8, 195u8, 3u8, 145u8],
            [166u8, 166u8, 203u8, 33u8],
            [170u8, 29u8, 73u8, 164u8],
            [171u8, 181u8, 244u8, 134u8],
            [174u8, 82u8, 235u8, 18u8],
            [174u8, 232u8, 99u8, 35u8],
            [175u8, 31u8, 4u8, 149u8],
            [179u8, 152u8, 151u8, 159u8],
            [182u8, 103u8, 156u8, 59u8],
            [190u8, 120u8, 48u8, 177u8],
            [195u8, 68u8, 106u8, 199u8],
            [203u8, 23u8, 183u8, 165u8],
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
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
        ];
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(KmsSignerDoesNotMatchTxSender),
            ::core::stringify!(NotGatewayOwner),
            ::core::stringify!(UnsupportedExtraDataVersion),
            ::core::stringify!(EmptyHandles),
            ::core::stringify!(NotCoprocessorSigner),
            ::core::stringify!(NotKmsSigner),
            ::core::stringify!(InvalidUserSignature),
            ::core::stringify!(EmptyCtHandles),
            ::core::stringify!(UserDecryptionRequestExpired),
            ::core::stringify!(MaxDurationDaysExceeded),
            ::core::stringify!(NotPauserOrGatewayConfig),
            ::core::stringify!(InvalidNullDurationSeconds),
            ::core::stringify!(ERC1967InvalidImplementation),
            ::core::stringify!(NotCoprocessorTxSender),
            ::core::stringify!(EmptyContractAddresses),
            ::core::stringify!(HostChainDisabled),
            ::core::stringify!(InvalidFHEType),
            ::core::stringify!(UserDecryptionRequestExpiredSeconds),
            ::core::stringify!(NotInitializingFromEmptyProxy),
            ::core::stringify!(InvalidKmsContext),
            ::core::stringify!(ExpectedPause),
            ::core::stringify!(InvalidExtraDataLength),
            ::core::stringify!(CtHandleChainIdDiffersFromContractChainId),
            ::core::stringify!(AddressEmptyCode),
            ::core::stringify!(KmsNodeAlreadySigned),
            ::core::stringify!(ContractNotInContractAddresses),
            ::core::stringify!(EmptyCtHandleContractPairs),
            ::core::stringify!(UUPSUnsupportedProxiableUUID),
            ::core::stringify!(DecryptionContextMismatch),
            ::core::stringify!(MaxDurationSecondsExceeded),
            ::core::stringify!(NotKmsTxSender),
            ::core::stringify!(ContractAddressesMaxLengthExceeded),
            ::core::stringify!(ERC1967NonPayable),
            ::core::stringify!(HostChainNotRegistered),
            ::core::stringify!(UnsupportedFHEType),
            ::core::stringify!(DelegatorAddressInContractAddresses),
            ::core::stringify!(InvalidNullContextId),
            ::core::stringify!(DifferentKeyIdsNotAllowed),
            ::core::stringify!(DecryptionNotRequested),
            ::core::stringify!(FailedCall),
            ::core::stringify!(ECDSAInvalidSignatureS),
            ::core::stringify!(NotInitializing),
            ::core::stringify!(EnforcedPause),
            ::core::stringify!(UserAddressInContractAddresses),
            ::core::stringify!(InvalidNullDurationDays),
            ::core::stringify!(UUPSUnauthorizedCallContext),
            ::core::stringify!(CoprocessorSignerDoesNotMatchTxSender),
            ::core::stringify!(NotOwnerOrGatewayConfig),
            ::core::stringify!(MaxDecryptionRequestBitSizeExceeded),
            ::core::stringify!(StartTimestampInFuture),
            ::core::stringify!(ECDSAInvalidSignature),
            ::core::stringify!(InvalidInitialization),
            ::core::stringify!(ECDSAInvalidSignatureLength),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <NotGatewayOwner as alloy_sol_types::SolError>::SIGNATURE,
            <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyHandles as alloy_sol_types::SolError>::SIGNATURE,
            <NotCoprocessorSigner as alloy_sol_types::SolError>::SIGNATURE,
            <NotKmsSigner as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidUserSignature as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyCtHandles as alloy_sol_types::SolError>::SIGNATURE,
            <UserDecryptionRequestExpired as alloy_sol_types::SolError>::SIGNATURE,
            <MaxDurationDaysExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <NotPauserOrGatewayConfig as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidNullDurationSeconds as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967InvalidImplementation as alloy_sol_types::SolError>::SIGNATURE,
            <NotCoprocessorTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyContractAddresses as alloy_sol_types::SolError>::SIGNATURE,
            <HostChainDisabled as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidFHEType as alloy_sol_types::SolError>::SIGNATURE,
            <UserDecryptionRequestExpiredSeconds as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidKmsContext as alloy_sol_types::SolError>::SIGNATURE,
            <ExpectedPause as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidExtraDataLength as alloy_sol_types::SolError>::SIGNATURE,
            <CtHandleChainIdDiffersFromContractChainId as alloy_sol_types::SolError>::SIGNATURE,
            <AddressEmptyCode as alloy_sol_types::SolError>::SIGNATURE,
            <KmsNodeAlreadySigned as alloy_sol_types::SolError>::SIGNATURE,
            <ContractNotInContractAddresses as alloy_sol_types::SolError>::SIGNATURE,
            <EmptyCtHandleContractPairs as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::SIGNATURE,
            <DecryptionContextMismatch as alloy_sol_types::SolError>::SIGNATURE,
            <MaxDurationSecondsExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <NotKmsTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <ContractAddressesMaxLengthExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <ERC1967NonPayable as alloy_sol_types::SolError>::SIGNATURE,
            <HostChainNotRegistered as alloy_sol_types::SolError>::SIGNATURE,
            <UnsupportedFHEType as alloy_sol_types::SolError>::SIGNATURE,
            <DelegatorAddressInContractAddresses as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidNullContextId as alloy_sol_types::SolError>::SIGNATURE,
            <DifferentKeyIdsNotAllowed as alloy_sol_types::SolError>::SIGNATURE,
            <DecryptionNotRequested as alloy_sol_types::SolError>::SIGNATURE,
            <FailedCall as alloy_sol_types::SolError>::SIGNATURE,
            <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::SIGNATURE,
            <NotInitializing as alloy_sol_types::SolError>::SIGNATURE,
            <EnforcedPause as alloy_sol_types::SolError>::SIGNATURE,
            <UserAddressInContractAddresses as alloy_sol_types::SolError>::SIGNATURE,
            <InvalidNullDurationDays as alloy_sol_types::SolError>::SIGNATURE,
            <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::SIGNATURE,
            <CoprocessorSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SIGNATURE,
            <NotOwnerOrGatewayConfig as alloy_sol_types::SolError>::SIGNATURE,
            <MaxDecryptionRequestBitSizeExceeded as alloy_sol_types::SolError>::SIGNATURE,
            <StartTimestampInFuture as alloy_sol_types::SolError>::SIGNATURE,
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
    impl alloy_sol_types::SolInterface for DecryptionErrors {
        const NAME: &'static str = "DecryptionErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 53usize;
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
                Self::DecryptionContextMismatch(_) => {
                    <DecryptionContextMismatch as alloy_sol_types::SolError>::SELECTOR
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
                Self::HostChainDisabled(_) => {
                    <HostChainDisabled as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidKmsContext(_) => {
                    <InvalidKmsContext as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNullContextId(_) => {
                    <InvalidNullContextId as alloy_sol_types::SolError>::SELECTOR
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
                    fn HostChainDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <HostChainDisabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::HostChainDisabled)
                    }
                    HostChainDisabled
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
                    fn InvalidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidKmsContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidKmsContext)
                    }
                    InvalidKmsContext
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
                    fn DecryptionContextMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionContextMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionContextMismatch)
                    }
                    DecryptionContextMismatch
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
                    fn InvalidNullContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullContextId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullContextId)
                    }
                    InvalidNullContextId
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
                    fn HostChainDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <HostChainDisabled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::HostChainDisabled)
                    }
                    HostChainDisabled
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
                    fn InvalidKmsContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidKmsContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidKmsContext)
                    }
                    InvalidKmsContext
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
                    fn DecryptionContextMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <DecryptionContextMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::DecryptionContextMismatch)
                    }
                    DecryptionContextMismatch
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
                    fn InvalidNullContextId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<DecryptionErrors> {
                        <InvalidNullContextId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(DecryptionErrors::InvalidNullContextId)
                    }
                    InvalidNullContextId
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
                Self::DecryptionContextMismatch(inner) => {
                    <DecryptionContextMismatch as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::HostChainDisabled(inner) => {
                    <HostChainDisabled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::InvalidKmsContext(inner) => {
                    <InvalidKmsContext as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNullContextId(inner) => {
                    <InvalidNullContextId as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::DecryptionContextMismatch(inner) => {
                    <DecryptionContextMismatch as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::HostChainDisabled(inner) => {
                    <HostChainDisabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::InvalidKmsContext(inner) => {
                    <InvalidKmsContext as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidNullContextId(inner) => {
                    <InvalidNullContextId as alloy_sol_types::SolError>::abi_encode_raw(
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
    #[derive(Clone)]
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
        PublicDecryptionRequest_0(PublicDecryptionRequest_0),
        #[allow(missing_docs)]
        PublicDecryptionRequest_1(PublicDecryptionRequest_1),
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
        UserDecryptionRequest_2(UserDecryptionRequest_2),
        #[allow(missing_docs)]
        UserDecryptionRequest_3(UserDecryptionRequest_3),
        #[allow(missing_docs)]
        UserDecryptionResponse(UserDecryptionResponse),
        #[allow(missing_docs)]
        UserDecryptionResponseThresholdReached(UserDecryptionResponseThresholdReached),
    }
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
                31u8, 128u8, 164u8, 123u8, 81u8, 151u8, 152u8, 55u8, 151u8, 111u8, 153u8,
                154u8, 119u8, 53u8, 253u8, 204u8, 187u8, 229u8, 112u8, 224u8, 212u8, 0u8,
                129u8, 100u8, 78u8, 200u8, 143u8, 142u8, 215u8, 108u8, 150u8, 18u8,
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
                142u8, 212u8, 250u8, 184u8, 177u8, 208u8, 80u8, 103u8, 108u8, 215u8,
                215u8, 217u8, 250u8, 68u8, 141u8, 93u8, 34u8, 17u8, 51u8, 117u8, 250u8,
                184u8, 173u8, 77u8, 198u8, 144u8, 229u8, 69u8, 147u8, 197u8, 39u8, 191u8,
            ],
            [
                155u8, 176u8, 136u8, 165u8, 63u8, 31u8, 225u8, 62u8, 200u8, 79u8, 160u8,
                161u8, 215u8, 181u8, 120u8, 181u8, 137u8, 225u8, 147u8, 38u8, 139u8,
                170u8, 204u8, 197u8, 226u8, 63u8, 122u8, 183u8, 133u8, 170u8, 61u8, 174u8,
            ],
            [
                188u8, 124u8, 215u8, 90u8, 32u8, 238u8, 39u8, 253u8, 154u8, 222u8, 186u8,
                179u8, 32u8, 65u8, 247u8, 85u8, 33u8, 77u8, 188u8, 107u8, 255u8, 169u8,
                12u8, 192u8, 34u8, 91u8, 57u8, 218u8, 46u8, 92u8, 45u8, 59u8,
            ],
            [
                199u8, 30u8, 50u8, 200u8, 15u8, 49u8, 9u8, 198u8, 115u8, 169u8, 173u8,
                252u8, 222u8, 208u8, 179u8, 178u8, 9u8, 50u8, 18u8, 171u8, 40u8, 64u8,
                132u8, 160u8, 224u8, 58u8, 40u8, 29u8, 238u8, 43u8, 221u8, 95u8,
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
        /// The names of the variants in the same order as `SELECTORS`.
        pub const VARIANT_NAMES: &'static [&'static str] = &[
            ::core::stringify!(EIP712DomainChanged),
            ::core::stringify!(UserDecryptionRequest_1),
            ::core::stringify!(PublicDecryptionRequest_0),
            ::core::stringify!(PublicDecryptionResponseCall),
            ::core::stringify!(Unpaused),
            ::core::stringify!(Paused),
            ::core::stringify!(UserDecryptionResponse),
            ::core::stringify!(UserDecryptionRequest_3),
            ::core::stringify!(PublicDecryptionRequest_1),
            ::core::stringify!(Upgraded),
            ::core::stringify!(UserDecryptionRequest_2),
            ::core::stringify!(Initialized),
            ::core::stringify!(PublicDecryptionResponse),
            ::core::stringify!(UserDecryptionResponseThresholdReached),
            ::core::stringify!(UserDecryptionRequest_0),
        ];
        /// The signatures in the same order as `SELECTORS`.
        pub const SIGNATURES: &'static [&'static str] = &[
            <EIP712DomainChanged as alloy_sol_types::SolEvent>::SIGNATURE,
            <UserDecryptionRequest_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <PublicDecryptionRequest_0 as alloy_sol_types::SolEvent>::SIGNATURE,
            <PublicDecryptionResponseCall as alloy_sol_types::SolEvent>::SIGNATURE,
            <Unpaused as alloy_sol_types::SolEvent>::SIGNATURE,
            <Paused as alloy_sol_types::SolEvent>::SIGNATURE,
            <UserDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE,
            <UserDecryptionRequest_3 as alloy_sol_types::SolEvent>::SIGNATURE,
            <PublicDecryptionRequest_1 as alloy_sol_types::SolEvent>::SIGNATURE,
            <Upgraded as alloy_sol_types::SolEvent>::SIGNATURE,
            <UserDecryptionRequest_2 as alloy_sol_types::SolEvent>::SIGNATURE,
            <Initialized as alloy_sol_types::SolEvent>::SIGNATURE,
            <PublicDecryptionResponse as alloy_sol_types::SolEvent>::SIGNATURE,
            <UserDecryptionResponseThresholdReached as alloy_sol_types::SolEvent>::SIGNATURE,
            <UserDecryptionRequest_0 as alloy_sol_types::SolEvent>::SIGNATURE,
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
    impl alloy_sol_types::SolEventInterface for DecryptionEvents {
        const NAME: &'static str = "DecryptionEvents";
        const COUNT: usize = 15usize;
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
                    <PublicDecryptionRequest_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionRequest_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionRequest_0)
                }
                Some(
                    <PublicDecryptionRequest_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PublicDecryptionRequest_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PublicDecryptionRequest_1)
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
                    <UserDecryptionRequest_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionRequest_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionRequest_2)
                }
                Some(
                    <UserDecryptionRequest_3 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UserDecryptionRequest_3 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::UserDecryptionRequest_3)
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
                Self::PublicDecryptionRequest_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PublicDecryptionRequest_1(inner) => {
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
                Self::UserDecryptionRequest_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UserDecryptionRequest_3(inner) => {
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
                Self::PublicDecryptionRequest_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PublicDecryptionRequest_1(inner) => {
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
                Self::UserDecryptionRequest_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UserDecryptionRequest_3(inner) => {
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
        __provider: P,
    ) -> DecryptionInstance<P, N> {
        DecryptionInstance::<P, N>::new(address, __provider)
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
        Output = alloy_contract::Result<DecryptionInstance<P, N>>,
    > {
        DecryptionInstance::<P, N>::deploy(__provider)
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
        DecryptionInstance::<P, N>::deploy_builder(__provider)
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
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > DecryptionInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`Decryption`](self) contract instance.

See the [wrapper's documentation](`DecryptionInstance`) for more details.*/
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
        ) -> alloy_contract::Result<DecryptionInstance<P, N>> {
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
        ///Creates a new call builder for the [`reinitializeV7`] function.
        pub fn reinitializeV7(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV7Call, N> {
            self.call_builder(&reinitializeV7Call)
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
        ///Creates a new event filter for the [`PublicDecryptionRequest_0`] event.
        pub fn PublicDecryptionRequest_0_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionRequest_0, N> {
            self.event_filter::<PublicDecryptionRequest_0>()
        }
        ///Creates a new event filter for the [`PublicDecryptionRequest_1`] event.
        pub fn PublicDecryptionRequest_1_filter(
            &self,
        ) -> alloy_contract::Event<&P, PublicDecryptionRequest_1, N> {
            self.event_filter::<PublicDecryptionRequest_1>()
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
        ///Creates a new event filter for the [`UserDecryptionRequest_2`] event.
        pub fn UserDecryptionRequest_2_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionRequest_2, N> {
            self.event_filter::<UserDecryptionRequest_2>()
        }
        ///Creates a new event filter for the [`UserDecryptionRequest_3`] event.
        pub fn UserDecryptionRequest_3_filter(
            &self,
        ) -> alloy_contract::Event<&P, UserDecryptionRequest_3, N> {
            self.event_filter::<UserDecryptionRequest_3>()
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
