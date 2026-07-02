///Module containing a contract's types and functions.
/**

```solidity
library IKMSGeneration {
    type KeyType is uint8;
    type ParamsType is uint8;
    struct HostChainCutover { uint256 chainId; uint64 cutoverBlock; }
    struct KeyDigest { KeyType keyType; bytes digest; }
    struct KeyInfo { uint256 prepKeygenId; uint256 keyId; ParamsType paramsType; KeyDigest[] keyDigests; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IKMSGeneration {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<KeyType> for u8 {
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
        impl KeyType {
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
        impl From<u8> for KeyType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<KeyType> for u8 {
            fn from(value: KeyType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for KeyType {
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
        impl alloy_sol_types::EventTopic for KeyType {
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
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ParamsType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<ParamsType> for u8 {
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
        impl ParamsType {
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
        impl From<u8> for ParamsType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<ParamsType> for u8 {
            fn from(value: ParamsType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ParamsType {
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
        impl alloy_sol_types::EventTopic for ParamsType {
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
struct HostChainCutover { uint256 chainId; uint64 cutoverBlock; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct HostChainCutover {
        #[allow(missing_docs)]
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub cutoverBlock: u64,
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
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<HostChainCutover> for UnderlyingRustTuple<'_> {
            fn from(value: HostChainCutover) -> Self {
                (value.chainId, value.cutoverBlock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for HostChainCutover {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    chainId: tuple.0,
                    cutoverBlock: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for HostChainCutover {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for HostChainCutover {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainId),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.cutoverBlock),
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
        impl alloy_sol_types::SolType for HostChainCutover {
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
        impl alloy_sol_types::SolStruct for HostChainCutover {
            const NAME: &'static str = "HostChainCutover";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "HostChainCutover(uint256 chainId,uint64 cutoverBlock)",
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
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.cutoverBlock)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for HostChainCutover {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.chainId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cutoverBlock,
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
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cutoverBlock,
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
struct KeyDigest { KeyType keyType; bytes digest; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyDigest {
        #[allow(missing_docs)]
        pub keyType: <KeyType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub digest: alloy::sol_types::private::Bytes,
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
        type UnderlyingSolTuple<'a> = (KeyType, alloy::sol_types::sol_data::Bytes);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <KeyType as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<KeyDigest> for UnderlyingRustTuple<'_> {
            fn from(value: KeyDigest) -> Self {
                (value.keyType, value.digest)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyDigest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    keyType: tuple.0,
                    digest: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for KeyDigest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for KeyDigest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <KeyType as alloy_sol_types::SolType>::tokenize(&self.keyType),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.digest,
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
        impl alloy_sol_types::SolType for KeyDigest {
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
        impl alloy_sol_types::SolStruct for KeyDigest {
            const NAME: &'static str = "KeyDigest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "KeyDigest(uint8 keyType,bytes digest)",
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
                    <KeyType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.keyType,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.digest,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KeyDigest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <KeyType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keyType,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.digest,
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
                <KeyType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyType,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.digest,
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
struct KeyInfo { uint256 prepKeygenId; uint256 keyId; ParamsType paramsType; KeyDigest[] keyDigests; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyInfo {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <ParamsType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <KeyDigest as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<256>,
            ParamsType,
            alloy::sol_types::sol_data::Array<KeyDigest>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::U256,
            <ParamsType as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<
                <KeyDigest as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<KeyInfo> for UnderlyingRustTuple<'_> {
            fn from(value: KeyInfo) -> Self {
                (value.prepKeygenId, value.keyId, value.paramsType, value.keyDigests)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    prepKeygenId: tuple.0,
                    keyId: tuple.1,
                    paramsType: tuple.2,
                    keyDigests: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for KeyInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for KeyInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <ParamsType as alloy_sol_types::SolType>::tokenize(&self.paramsType),
                    <alloy::sol_types::sol_data::Array<
                        KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
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
        impl alloy_sol_types::SolType for KeyInfo {
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
        impl alloy_sol_types::SolStruct for KeyInfo {
            const NAME: &'static str = "KeyInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "KeyInfo(uint256 prepKeygenId,uint256 keyId,uint8 paramsType,KeyDigest[] keyDigests)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<KeyDigest as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <KeyDigest as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.prepKeygenId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.keyId)
                        .0,
                    <ParamsType as alloy_sol_types::SolType>::eip712_data_word(
                            &self.paramsType,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        KeyDigest,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.keyDigests)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for KeyInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prepKeygenId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.keyId)
                    + <ParamsType as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.paramsType,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        KeyDigest,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.keyDigests,
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
                    &rust.prepKeygenId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyId,
                    out,
                );
                <ParamsType as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.paramsType,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    KeyDigest,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.keyDigests,
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
    /**Creates a new wrapper around an on-chain [`IKMSGeneration`](self) contract instance.

See the [wrapper's documentation](`IKMSGenerationInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IKMSGenerationInstance<P, N> {
        IKMSGenerationInstance::<P, N>::new(address, provider)
    }
    /**A [`IKMSGeneration`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IKMSGeneration`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IKMSGenerationInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IKMSGenerationInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IKMSGenerationInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > IKMSGenerationInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`IKMSGeneration`](self) contract instance.

See the [wrapper's documentation](`IKMSGenerationInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IKMSGenerationInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IKMSGenerationInstance<P, N> {
            IKMSGenerationInstance {
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
    > IKMSGenerationInstance<P, N> {
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
    > IKMSGenerationInstance<P, N> {
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
library IKMSGeneration {
    type KeyType is uint8;
    type ParamsType is uint8;
    struct HostChainCutover {
        uint256 chainId;
        uint64 cutoverBlock;
    }
    struct KeyDigest {
        KeyType keyType;
        bytes digest;
    }
    struct KeyInfo {
        uint256 prepKeygenId;
        uint256 keyId;
        ParamsType paramsType;
        KeyDigest[] keyDigests;
    }
}

interface KMSGeneration {
    error AbortCrsgenAlreadyDone(uint256 crsId);
    error AbortCrsgenInvalidId(uint256 crsId);
    error AbortKeygenAlreadyDone(uint256 prepKeygenId);
    error AbortKeygenInvalidId(uint256 prepKeygenId);
    error AddressEmptyCode(address target);
    error CompressedKeyCutoverAlreadyScheduled(uint256 keyId);
    error CompressedKeyMaterialsAlreadyAdded(uint256 keyId);
    error CompressedKeyMaterialsNotAdded(uint256 keyId);
    error CrsAborted(uint256 crsId);
    error CrsNotGenerated(uint256 crsId);
    error CrsgenNotRequested(uint256 crsId);
    error CrsgenOngoing(uint256 crsId);
    error DeserializingExtraDataFail();
    error DuplicateCutoverChainId(uint256 chainId);
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyHostChainCutovers();
    error EmptyKeyDigests(uint256 keyId);
    error FailedCall();
    error InvalidCutoverBlock();
    error InvalidInitialization();
    error KeyAborted(uint256 keyId);
    error KeyManagementRequestPending();
    error KeyNotGenerated(uint256 keyId);
    error KeygenNotRequested(uint256 keyId);
    error KeygenOngoing(uint256 keyId);
    error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsSigner(address signerAddress);
    error NotKmsTxSender(address txSenderAddress);
    error PrepKeygenNotRequested(uint256 prepKeygenId);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnsupportedExtraDataVersion(uint8 version);
    error WrongKeygenResponseEndpoint(uint256 requestId);

    event AbortCrsgen(uint256 crsId);
    event AbortKeygen(uint256 prepKeygenId);
    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
    event CompressedKeyCutoverScheduled(uint256 keyId, IKMSGeneration.HostChainCutover[] hostChainCutovers, uint64 gatewayCutoverBlock);
    event CompressedKeyMaterialAdded(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
    event CompressedKeyMaterialResponse(uint256 migrationRequestId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
    event CompressedKeyMigrationKeygenRequest(uint256 prepKeygenId, uint256 migrationRequestId, uint256 keyId, bytes extraData);
    event CompressedKeyMigrationPrepKeygenRequest(uint256 prepKeygenId, uint256 keyId, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);
    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);
    event KeygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
    event PrepKeygenRequest(uint256 prepKeygenId, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function abortCrsgen(uint256 crsId) external;
    function abortKeygen(uint256 prepKeygenId) external;
    function addCompressedKeyMaterials(uint256 migrationRequestId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
    function compressedKeyMigrationKeygen(uint256 keyId) external;
    function crsgenRequest(uint256 maxBitLength, IKMSGeneration.ParamsType paramsType) external;
    function crsgenResponse(uint256 crsId, bytes memory crsDigest, bytes memory signature) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getActiveCrsId() external view returns (uint256);
    function getActiveKeyId() external view returns (uint256);
    function getCompletedCrsIds() external view returns (uint256[] memory);
    function getCompletedKeyIds() external view returns (uint256[] memory);
    function getCompressedKeyCutoverSchedule(uint256 keyId) external view returns (bool, IKMSGeneration.HostChainCutover[] memory, uint64);
    function getCompressedKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);
    function getCrsCounter() external view returns (uint256);
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);
    function getCrsParamsType(uint256 crsId) external view returns (IKMSGeneration.ParamsType);
    function getKeyCounter() external view returns (uint256);
    function getKeyInfo(uint256 keyId) external view returns (IKMSGeneration.KeyInfo memory);
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
    function getKeyParamsType(uint256 keyId) external view returns (IKMSGeneration.ParamsType);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function isRequestDone(uint256 requestId) external view returns (bool);
    function keygen(IKMSGeneration.ParamsType paramsType) external;
    function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
    function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV3() external;
    function scheduleCompressedKeyCutover(uint256 keyId, IKMSGeneration.HostChainCutover[] memory hostChainCutovers, uint64 gatewayCutoverBlock) external;
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
    "name": "abortCrsgen",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "abortKeygen",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addCompressedKeyMaterials",
    "inputs": [
      {
        "name": "migrationRequestId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
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
    "name": "compressedKeyMigrationKeygen",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "crsgenRequest",
    "inputs": [
      {
        "name": "maxBitLength",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "crsgenResponse",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "crsDigest",
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
    "name": "getActiveCrsId",
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
    "name": "getActiveKeyId",
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
    "name": "getCompletedCrsIds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCompletedKeyIds",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCompressedKeyCutoverSchedule",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.HostChainCutover[]",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "cutoverBlock",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCompressedKeyMaterials",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "string[]",
        "internalType": "string[]"
      },
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getConsensusTxSenders",
    "inputs": [
      {
        "name": "requestId",
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
    "name": "getCrsCounter",
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
    "name": "getCrsMaterials",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "string[]",
        "internalType": "string[]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCrsParamsType",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeyCounter",
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
    "name": "getKeyInfo",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IKMSGeneration.KeyInfo",
        "components": [
          {
            "name": "prepKeygenId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "keyId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "paramsType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.ParamsType"
          },
          {
            "name": "keyDigests",
            "type": "tuple[]",
            "internalType": "struct IKMSGeneration.KeyDigest[]",
            "components": [
              {
                "name": "keyType",
                "type": "uint8",
                "internalType": "enum IKMSGeneration.KeyType"
              },
              {
                "name": "digest",
                "type": "bytes",
                "internalType": "bytes"
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
    "name": "getKeyMaterials",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "string[]",
        "internalType": "string[]"
      },
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getKeyParamsType",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
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
    "name": "isRequestDone",
    "inputs": [
      {
        "name": "requestId",
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
    "name": "keygen",
    "inputs": [
      {
        "name": "paramsType",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "keygenResponse",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
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
    "name": "prepKeygenResponse",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "scheduleCompressedKeyCutover",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "hostChainCutovers",
        "type": "tuple[]",
        "internalType": "struct IKMSGeneration.HostChainCutover[]",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "cutoverBlock",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "gatewayCutoverBlock",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "AbortCrsgen",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "AbortKeygen",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ActivateCrs",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "indexed": false,
        "internalType": "string[]"
      },
      {
        "name": "crsDigest",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ActivateKey",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "indexed": false,
        "internalType": "string[]"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
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
    "name": "CompressedKeyCutoverScheduled",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "hostChainCutovers",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IKMSGeneration.HostChainCutover[]",
        "components": [
          {
            "name": "chainId",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "cutoverBlock",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      },
      {
        "name": "gatewayCutoverBlock",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CompressedKeyMaterialAdded",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "indexed": false,
        "internalType": "string[]"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
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
    "name": "CompressedKeyMaterialResponse",
    "inputs": [
      {
        "name": "migrationRequestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
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
        "name": "kmsTxSender",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CompressedKeyMigrationKeygenRequest",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "migrationRequestId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "CompressedKeyMigrationPrepKeygenRequest",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKMSGeneration.ParamsType"
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
    "name": "CrsgenRequest",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "maxBitLength",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKMSGeneration.ParamsType"
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
    "name": "CrsgenResponse",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "crsDigest",
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
      }
    ],
    "anonymous": false
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
    "name": "KeygenRequest",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "KeygenResponse",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "keyDigests",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IKMSGeneration.KeyDigest[]",
        "components": [
          {
            "name": "keyType",
            "type": "uint8",
            "internalType": "enum IKMSGeneration.KeyType"
          },
          {
            "name": "digest",
            "type": "bytes",
            "internalType": "bytes"
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
        "name": "kmsTxSender",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PrepKeygenRequest",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "paramsType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKMSGeneration.ParamsType"
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
    "name": "PrepKeygenResponse",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "AbortCrsgenAlreadyDone",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "AbortCrsgenInvalidId",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "AbortKeygenAlreadyDone",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "AbortKeygenInvalidId",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
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
    "name": "CompressedKeyCutoverAlreadyScheduled",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CompressedKeyMaterialsAlreadyAdded",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CompressedKeyMaterialsNotAdded",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CrsAborted",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CrsNotGenerated",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CrsgenNotRequested",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "CrsgenOngoing",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "DeserializingExtraDataFail",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DuplicateCutoverChainId",
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
    "name": "EmptyHostChainCutovers",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyKeyDigests",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "InvalidCutoverBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidInitialization",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyAborted",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KeyManagementRequestPending",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyNotGenerated",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KeygenNotRequested",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KeygenOngoing",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsAlreadySignedForCrsgen",
    "inputs": [
      {
        "name": "crsId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "kmsSigner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsAlreadySignedForKeygen",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "kmsSigner",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "KmsAlreadySignedForPrepKeygen",
    "inputs": [
      {
        "name": "prepKeygenId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "kmsSigner",
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
    "name": "PrepKeygenNotRequested",
    "inputs": [
      {
        "name": "prepKeygenId",
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
    "name": "WrongKeygenResponseEndpoint",
    "inputs": [
      {
        "name": "requestId",
        "type": "uint256",
        "internalType": "uint256"
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
pub mod KMSGeneration {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523060805234801562000014575f80fd5b506200001f62000025565b620000d9565b7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00805468010000000000000000900460ff1615620000765760405163f92ee8a960e01b815260040160405180910390fd5b80546001600160401b0390811614620000d65780546001600160401b0319166001600160401b0390811782556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200160405180910390a15b50565b6080516158a1620001005f395f8181613686015281816136af015261389701526158a15ff3fe6080604052600436106101db575f3560e01c80637609152d116100fd578063c1252ca911610092578063d52f10eb11610062578063d52f10eb14610587578063dabd732f1461059b578063e410117e146105bc578063e711c9e7146105d0575f80fd5b8063c1252ca9146104fd578063c2c1faee1461051c578063c55b87241461053b578063caa367db14610568575f80fd5b8063ad3cb1cc116100cd578063ad3cb1cc14610486578063bac22bb8146104b6578063baff211e146104ca578063bc5404a0146104de575f80fd5b80637609152d146103e557806384b0196e146104135780639081fe131461043a578063936608ae14610459575f80fd5b80633d5ec7e31161017357806352d1902d1161014357806352d1902d14610367578063589adb0e1461037b5780636294f4621461039a57806362978787146103c6575f80fd5b80633d5ec7e3146102e757806345af261b146103165780634610ffe8146103355780634f1ef28614610354575f80fd5b806319f4f632116101ae57806319f4f6321461027457806339f73810146102a05780633ac50072146102b45780633c02f834146102c8575f80fd5b80630b680733146101df5780630d8e6e2c1461020657806316c713d9146102275780631703c61a14610253575b5f80fd5b3480156101ea575f80fd5b506101f36105ef565b6040519081526020015b60405180910390f35b348015610211575f80fd5b5061021a610603565b6040516101fd9190614651565b348015610232575f80fd5b50610246610241366004614663565b61066e565b6040516101fd919061467a565b34801561025e575f80fd5b5061027261026d366004614663565b6106fc565b005b34801561027f575f80fd5b5061029361028e366004614663565b610877565b6040516101fd91906146f6565b3480156102ab575f80fd5b5061027261090f565b3480156102bf575f80fd5b506101f3610a77565b3480156102d3575f80fd5b506102726102e2366004614715565b610a8b565b3480156102f2575f80fd5b50610306610301366004614663565b610cca565b60405190151581526020016101fd565b348015610321575f80fd5b50610293610330366004614663565b610ceb565b348015610340575f80fd5b5061027261034f366004614787565b610d71565b6102726103623660046148ce565b611100565b348015610372575f80fd5b506101f361111f565b348015610386575f80fd5b5061027261039536600461495a565b61113a565b3480156103a5575f80fd5b506103b96103b4366004614663565b61139e565b6040516101fd9190614a16565b3480156103d1575f80fd5b506102726103e0366004614a61565b61158b565b3480156103f0575f80fd5b506104046103ff366004614663565b611840565b6040516101fd93929190614ab6565b34801561041e575f80fd5b506104276118fc565b6040516101fd9796959493929190614b66565b348015610445575f80fd5b50610272610454366004614663565b6119a5565b348015610464575f80fd5b50610478610473366004614663565b611cfa565b6040516101fd929190614c20565b348015610491575f80fd5b5061021a604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156104c1575f80fd5b50610272611fc2565b3480156104d5575f80fd5b506101f3612070565b3480156104e9575f80fd5b506102726104f8366004614c58565b612084565b348015610508575f80fd5b50610272610517366004614787565b6123a3565b348015610527575f80fd5b50610272610536366004614663565b612781565b348015610546575f80fd5b5061055a610555366004614663565b612926565b6040516101fd929190614ce2565b348015610573575f80fd5b50610272610582366004614d06565b612ae3565b348015610592575f80fd5b506101f3612d4f565b3480156105a6575f80fd5b506105af612d63565b6040516101fd9190614d21565b3480156105c7575f80fd5b506105af612dc2565b3480156105db575f80fd5b506104786105ea366004614663565b612e1f565b5f806105f9613041565b6005015492915050565b60606040518060400160405280600d81526020016c25a6a9a3b2b732b930ba34b7b760991b8152506106345f613065565b61063e6003613065565b6106475f613065565b60405160200161065a9493929190614d33565b604051602081830303815290604052905090565b60605f610679613041565b5f848152600382016020908152604080832054600285018352818420818552835292819020805482518185028101850190935280835294955092939092918301828280156106ee57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116106d0575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561074c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107709190614db0565b6001600160a01b0316336001600160a01b0316146107a85760405163021bfda160e41b81523360048201526024015b60405180910390fd5b5f6107b1613041565b905080600901548211806107c95750600560f81b8211155b156107ea576040516365f4932b60e11b81526004810183905260240161079f565b5f82815260018201602052604090205460ff161561081e5760405163df0db5fb60e01b81526004810183905260240161079f565b5f8281526001828101602052604091829020805460ff19169091179055517f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e9061086b9084815260200190565b60405180910390a15050565b5f80610881613041565b5f84815260018201602052604090205490915060ff166108b7576040516384de133160e01b81526004810184905260240161079f565b5f8381526003820160205260409020546108e7576040516383f1833560e01b81526004810184905260240161079f565b5f9283526006810160209081526040808520548552600d90920190529091205460ff16919050565b5f80516020615881833981519152546001600160401b03166001600160401b031660011461095057604051636f4f731f60e01b815260040160405180910390fd5b5f80516020615881833981519152805460049190600160401b900460ff1680610986575080546001600160401b03808416911610155b156109a45760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b178155604080518082018252600d81526c25a6a9a3b2b732b930ba34b7b760991b602080830191909152825180840190935260018352603160f81b90830152610a0a916130f4565b5f610a13613041565b600360f81b6004820155600160fa1b6005820155600560f81b60099091015550805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200161086b565b5f80610a81613041565b6009015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610adb573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610aff9190614db0565b6001600160a01b0316336001600160a01b031614610b325760405163021bfda160e41b815233600482015260240161079f565b5f610b3b613041565b6009810154909150600560f81b8114801590610b6757505f81815260018301602052604090205460ff16155b15610b885760405163061ac61d60e01b81526004810182905260240161079f565b600982018054905f610b9983614dcb565b909155505060098201545f818152600a840160209081526040808320889055600d86019091529020805485919060ff191660018381811115610bdd57610bdd6146c6565b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015610c33573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c579190614def565b915091505f610c668383613106565b5f858152600e880160205260409020909150610c828282614ea8565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d84898984604051610cb89493929190614f62565b60405180910390a15050505050505050565b5f80610cd4613041565b5f9384526001016020525050604090205460ff1690565b5f80610cf5613041565b5f84815260018201602052604090205490915060ff16610d2b5760405163da32d00f60e01b81526004810184905260240161079f565b5f838152600382016020526040902054610d5b5760405163d5fd3cd760e01b81526004810184905260240161079f565b5f928352600d0160205250604090205460ff1690565b5f610d7a613041565b90508060050154861180610d925750600160fa1b8611155b15610db357604051632b7eae4160e21b81526004810187905260240161079f565b5f86815260118201602052604090205415610de457604051635a240b5b60e11b81526004810187905260240161079f565b5f849003610e085760405163e6f9083b60e01b81526004810187905260240161079f565b5f80610e138861313e565b5f8a815260068601602090815260408083205480845260018901909252909120549294509092509060ff16610e5b57604051636fbcdd2b60e01b815260040160405180910390fd5b5f610e69828b8b8b8861328f565b90505f610e7884838a8a613465565b5f8c8152602088815260408083206001600160a01b038516845290915290205490915060ff1615610ece576040516398fb957d60e01b8152600481018c90526001600160a01b038216602482015260440161079f565b5f8b8152602087815260408083206001600160a01b03851684528252808320805460ff191660019081179091558e845260028a0183528184208685528352818420805491820181558085529290932090920180546001600160a01b03191633908117909155915190917f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c791610f6c918f918f918f918f918f91615074565b60405180910390a15f8c815260018801602052604090205460ff16158015610f9c57508054610f9c9086906134bc565b156110f2575f8c8152600188810160205260408220805460ff191690911790555b8a811015611023575f8d815260078901602052604090208c8c83818110610fe657610fe66150bc565b9050602002810190610ff891906150d0565b81546001810183555f928352602090922090916002020161101982826151e4565b5050600101610fbd565b505f8c8152600388016020908152604080832086905560088a018f9055600f8a018054600181018255908452828420018f905583548151818402810184019092528082526110b19289929186918301828280156110a757602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611089575b505050505061353d565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8d828e8e6040516110e894939291906152ed565b60405180910390a1505b505050505050505050505050565b61110861367b565b61111182613721565b61111b82826137cb565b5050565b5f61112861388c565b505f8051602061567b83398151915290565b5f611143613041565b9050806004015484118061115b5750600360f81b8411155b1561117c57604051630ab7f68760e01b81526004810185905260240161079f565b5f806111878661313e565b915091505f61119687846138d5565b90505f6111a583838989613465565b5f898152602087815260408083206001600160a01b038516845290915290205490915060ff16156111fb576040516333ca1fe360e01b8152600481018990526001600160a01b038216602482015260440161079f565b5f888152602086815260408083206001600160a01b03851684528252808320805460ff191660019081179091558b84526002890183528184208685528352818420805491820181558085529290932090920180546001600160a01b03191633908117909155915190917f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c91611295918c918c918c91615318565b60405180910390a15f89815260018701602052604090205460ff161580156112c5575080546112c59085906134bc565b15611393575f898152600187810160209081526040808420805460ff19169093179092556003890181528183208690556006890181528183205480845260118a019091529120548015611354577fb07c5a1b25df43bfe6d702bac06c3d6e151a71da406a6cba07ffa41700dcabe18b83838a604051611347949392919061534a565b60405180910390a1611390565b7f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8b83896040516113879392919061536e565b60405180910390a15b50505b505050505050505050565b6113a66145ce565b5f6113af613041565b5f84815260018201602052604090205490915060ff166113e5576040516384de133160e01b81526004810184905260240161079f565b5f838152600382016020526040902054611415576040516383f1833560e01b81526004810184905260240161079f565b5f8381526006820160209081526040808320548151608081018352818152808401889052818552600d860190935292819020549082019060ff166001811115611460576114606146c6565b81525f86815260078501602090815260408083208054825181850281018501909352808352948301949193909284015b8282101561157d575f8481526020902060408051808201909152600284029091018054829060ff1660018111156114c9576114c96146c6565b60018111156114da576114da6146c6565b81526020016001820180546114ee90614e11565b80601f016020809104026020016040519081016040528092919081815260200182805461151a90614e11565b80156115655780601f1061153c57610100808354040283529160200191611565565b820191905f5260205f20905b81548152906001019060200180831161154857829003601f168201915b50505050508152505081526020019060010190611490565b505050915250949350505050565b5f611594613041565b905080600901548611806115ac5750600560f81b8611155b156115cd576040516346c64a0560e11b81526004810187905260240161079f565b5f806115d88861313e565b915091505f6115fd8985600a015f8c81526020019081526020015f20548a8a87613925565b90505f61160c83838989613465565b5f8b8152602087815260408083206001600160a01b038516845290915290205490915060ff16156116625760405163fcf5a6e960e01b8152600481018b90526001600160a01b038216602482015260440161079f565b5f8a8152602086815260408083206001600160a01b03851684528252808320805460ff191660019081179091558d84526002890183528184208685528352818420805491820181558085529290932090920180546001600160a01b03191633908117909155915190917f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd91611700918e918e918e918e918e9161538c565b60405180910390a15f8b815260018701602052604090205460ff16158015611730575080546117309085906134bc565b15611390575f8b8152600187810160209081526040808420805460ff1916909317909255600b8901905290206117678a8c83615130565b505f8b81526003870160209081526040808320869055600c89018e9055601089018054600181018255908452828420018e905583548151818402810184019092528082526117f39288929186918301828280156110a757602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161108957505050505061353d565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d60405161182a94939291906153a5565b60405180910390a1505050505050505050505050565b5f60605f8061184d613041565b5f86815260168201602090815260408083205460148501835281842060158601845282852054815484518187028101870190955280855296975060ff9092169590946001600160401b039092169385929084015b828210156118e8575f8481526020908190206040805180820190915260028502909101805482526001908101546001600160401b03168284015290835290920191016118a1565b505050509150935093509350509193909250565b5f60608082808083815f8051602061565b833981519152805490915015801561192757506001810154155b61196b5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b604482015260640161079f565b6119736139b1565b61197b613a68565b604080515f80825260208201909252600f60f81b9c939b5091995046985030975095509350915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156119f5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a199190614db0565b6001600160a01b0316336001600160a01b031614611a4c5760405163021bfda160e41b815233600482015260240161079f565b5f611a55613041565b5f83815260018201602052604090205490915060ff161580611a7a5750806005015482115b80611a895750600160fa1b8211155b15611aaa576040516384de133160e01b81526004810183905260240161079f565b5f828152600382016020526040902054611ada576040516383f1833560e01b81526004810183905260240161079f565b5f82815260128201602052604090205415611b0b57604051632231dc3d60e21b81526004810183905260240161079f565b6005810154600160fa1b8114801590611b3457505f81815260018301602052604090205460ff16155b15611b5557604051630770a7b560e31b81526004810182905260240161079f565b600482018054905f611b6683614dcb565b90915550506004820154600583018054905f611b8183614dcb565b909155505060058301545f8281526006850160208181526040808420859055848452808420869055601188018252808420899055888452918152818320548352600d87019052808220548483529120805460ff9092169182919060ff191660018381811115611bf257611bf26146c6565b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015611c48573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c6c9190614def565b915091505f611c7b8383613106565b5f878152600e8a0160205260409020909150611c978282614ea8565b505f858152600e890160205260409020611cb18282614ea8565b507f2869c51322e90d2ce2dfe55ec29d180f36bffa15b434fc0a8dd0e5d3ce3c174c868a8684604051611ce79493929190614f62565b60405180910390a1505050505050505050565b6060805f611d06613041565b5f85815260018201602052604090205490915060ff16611d3c576040516384de133160e01b81526004810185905260240161079f565b5f84815260038201602052604090205480611d6d576040516383f1833560e01b81526004810186905260240161079f565b5f8581526002830160209081526040808320848452825280832080548251818502810185019093528083529192909190830182828015611dd457602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611db6575b505050505090505f611e7e84600e015f8981526020019081526020015f208054611dfd90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054611e2990614e11565b8015611e745780601f10611e4b57610100808354040283529160200191611e74565b820191905f5260205f20905b815481529060010190602001808311611e5757829003601f168201915b5050505050613aa6565b90505f611e8b828461353d565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015611fae575f8481526020902060408051808201909152600284029091018054829060ff166001811115611efa57611efa6146c6565b6001811115611f0b57611f0b6146c6565b8152602001600182018054611f1f90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054611f4b90614e11565b8015611f965780601f10611f6d57610100808354040283529160200191611f96565b820191905f5260205f20905b815481529060010190602001808311611f7957829003601f168201915b50505050508152505081526020019060010190611ec1565b505050509050965096505050505050915091565b5f80516020615881833981519152805460049190600160401b900460ff1680611ff8575080546001600160401b03808416911610155b156120165760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1760ff60401b191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200161086b565b5f8061207a613041565b600c015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120d4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120f89190614db0565b6001600160a01b0316336001600160a01b03161461212b5760405163021bfda160e41b815233600482015260240161079f565b5f612134613041565b5f8681526012820160205260408120549192500361216857604051637c8b772160e11b81526004810186905260240161079f565b5f85815260168201602052604090205460ff161561219c57604051632031495b60e11b81526004810186905260240161079f565b5f8390036121bd576040516304a6b4b160e51b815260040160405180910390fd5b816001600160401b03165f036121e65760405163246bc3fb60e11b815260040160405180910390fd5b5f5b8381101561231d57848482818110612202576122026150bc565b905060400201602001602081019061221a91906153d0565b6001600160401b03165f036122425760405163246bc3fb60e11b815260040160405180910390fd5b5f5b818110156122c45785858381811061225e5761225e6150bc565b9050604002015f0135868683818110612279576122796150bc565b9050604002015f0135036122bc57858583818110612299576122996150bc565b60408051634a056b6b60e11b81529102929092013560048301525060240161079f565b600101612244565b505f86815260148301602052604090208585838181106122e6576122e66150bc565b83546001810185555f948552602090942060409091029290920192600202909101905061231382826153eb565b50506001016121e8565b505f8581526015820160209081526040808320805467ffffffffffffffff19166001600160401b0387161790556016840190915290819020805460ff19166001179055517f78cec34fd2715b935e4af4ae695f75a2c57cbd9655f1e7e3cc86c97ce79fa29f90612394908790879087908790615421565b60405180910390a15050505050565b5f6123ac613041565b905080600501548611806123c45750600160fa1b8611155b156123e557604051632b7eae4160e21b81526004810187905260240161079f565b5f8681526011820160205260408120549081900361241957604051635a240b5b60e11b81526004810188905260240161079f565b5f85900361243d5760405163e6f9083b60e01b81526004810188905260240161079f565b5f806124488961313e565b5f8b815260068701602090815260408083205480845260018a01909252909120549294509092509060ff1661249057604051636fbcdd2b60e01b815260040160405180910390fd5b5f61249f828c878d8d89613c10565b90505f6124ae84838b8b613465565b5f8d8152602089815260408083206001600160a01b038516845290915290205490915060ff1615612504576040516398fb957d60e01b8152600481018d90526001600160a01b038216602482015260440161079f565b6001875f015f8e81526020019081526020015f205f836001600160a01b03166001600160a01b031681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f876002015f8e81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a8154816001600160a01b0302191690836001600160a01b031602179055507f03c88abb63cb1a33cba1e37393e7fc01f203bb037b92f5ef84d52e5c63ac7be78d8d8d8d8d336040516125f296959493929190615074565b60405180910390a15f8d815260018901602052604090205460ff16158015612622575080546126229086906134bc565b15612772575f8d8152600189810160209081526040808420805460ff191690931790925560038b01905281208490555b8b8110156126b8575f88815260138a01602052604090208d8d8381811061267b5761267b6150bc565b905060200281019061268d91906150d0565b81546001810183555f92835260209092209091600202016126ae82826151e4565b5050600101612652565b508c886012015f8981526020019081526020015f20819055505f61273186838054806020026020016040519081016040528092919081815260200182805480156110a757602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161108957505050505061353d565b90507f80ebc2a4e183000f6837fab1e36970e8bc4a1b19223054c32769db663a4ce34688828f8f60405161276894939291906152ed565b60405180910390a1505b50505050505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127d1573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127f59190614db0565b6001600160a01b0316336001600160a01b0316146128285760405163021bfda160e41b815233600482015260240161079f565b5f612831613041565b905080600401548211806128495750600360f81b8211155b1561286a57604051637e796dbd60e11b81526004810183905260240161079f565b5f828152600682016020908152604080832054808452600185019092529091205460ff16156128af576040516392789b6760e01b81526004810184905260240161079f565b5f83815260018381016020526040909120805460ff1916909117905580156128ee575f81815260018381016020526040909120805460ff191690911790555b6040518381527f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe32649060200160405180910390a1505050565b6060805f612932613041565b5f85815260018201602052604090205490915060ff166129685760405163da32d00f60e01b81526004810185905260240161079f565b5f848152600382016020526040902054806129995760405163d5fd3cd760e01b81526004810186905260240161079f565b5f8581526002830160209081526040808320848452825280832080548251818502810185019093528083529192909190830182828015612a0057602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116129e2575b505050505090505f612a2984600e015f8981526020019081526020015f208054611dfd90614e11565b90505f612a36828461353d565b5f898152600b870160205260409020805491925082918190612a5790614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054612a8390614e11565b8015612ace5780601f10612aa557610100808354040283529160200191612ace565b820191905f5260205f20905b815481529060010190602001808311612ab157829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612b33573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b579190614db0565b6001600160a01b0316336001600160a01b031614612b8a5760405163021bfda160e41b815233600482015260240161079f565b5f612b93613041565b6005810154909150600160fa1b8114801590612bbf57505f81815260018301602052604090205460ff16155b15612be057604051630770a7b560e31b81526004810182905260240161079f565b600482018054905f612bf183614dcb565b90915550506004820154600583018054905f612c0c83614dcb565b909155505060058301545f8281526006850160209081526040808320849055838352808320859055848352600d87019091529020805486919060ff191660018381811115612c5c57612c5c6146c6565b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015612cb2573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cd69190614def565b915091505f612ce58383613106565b5f868152600e890160205260409020909150612d018282614ea8565b505f848152600e880160205260409020612d1b8282614ea8565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe91858983604051610cb893929190615495565b5f80612d59613041565b6008015492915050565b60605f612d6e613041565b60108101805460408051602080840282018101909252828152939450830182828015612db757602002820191905f5260205f20905b815481526020019060010190808311612da3575b505050505091505090565b60605f612dcd613041565b600f8101805460408051602080840282018101909252828152939450830182828015612db757602002820191905f5260205f2090815481526020019060010190808311612da357505050505091505090565b6060805f612e2b613041565b5f858152601282016020526040812054919250819003612e6157604051637c8b772160e11b81526004810186905260240161079f565b5f81815260038301602090815260408083205460028601835281842081855283528184208054835181860281018601909452808452919493909190830182828015612ed357602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311612eb5575b505050505090505f612efc85600e015f8681526020019081526020015f208054611dfd90614e11565b90505f612f09828461353d565b905080866013015f8b81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b8282101561302c575f8481526020902060408051808201909152600284029091018054829060ff166001811115612f7857612f786146c6565b6001811115612f8957612f896146c6565b8152602001600182018054612f9d90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054612fc990614e11565b80156130145780601f10612feb57610100808354040283529160200191613014565b820191905f5260205f20905b815481529060010190602001808311612ff757829003601f168201915b50505050508152505081526020019060010190612f3f565b50505050905097509750505050505050915091565b7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db0090565b60605f61307183613dd6565b60010190505f816001600160401b0381111561308f5761308f61483c565b6040519080825280601f01601f1916602001820160405280156130b9576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a85049450846130c357509392505050565b6130fc613ead565b61111b8282613ee3565b604051600160f91b60208201526021810183905260418101829052606090606101604051602081830303815290604052905092915050565b60605f8061314a613041565b5f858152600e82016020526040902080549192509061316890614e11565b80601f016020809104026020016040519081016040528092919081815260200182805461319490614e11565b80156131df5780601f106131b6576101008083540402835291602001916131df565b820191905f5260205f20905b8154815290600101906020018083116131c257829003601f168201915b505050505092506131ef83613aa6565b6040516346c5bbbd60e01b8152600481018290523360248201529092507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac906346c5bbbd90604401602060405180830381865afa158015613246573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061326a91906154bc565b6132895760405163aee8632360e01b815233600482015260240161079f565b50915091565b5f80836001600160401b038111156132a9576132a961483c565b6040519080825280602002602001820160405280156132d2578160200160208202803683370190505b5090505f5b848110156133c35760405180606001604052806025815260200161585c6025913980519060200120868683818110613311576133116150bc565b905060200281019061332391906150d0565b613331906020810190614d06565b878784818110613343576133436150bc565b905060200281019061335591906150d0565b6133639060208101906150ee565b6040516133719291906154db565b6040519081900381206133889392916020016154ea565b604051602081830303815290604052805190602001208282815181106133b0576133b06150bc565b60209081029190910101526001016132d7565b5061345a6040518060c00160405280608281526020016157da60829139805190602001208888846040516020016133fa919061550b565b60408051601f1981840301815282825280516020918201208a518b83012091840196909652908201939093526060810191909152608081019290925260a082015260c0015b60405160208183030381529060405280519060200120613f42565b979650505050505050565b5f806134a68585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250613f6e92505050565b90506134b3868233613f96565b95945050505050565b60405163106b41a760e21b8152600481018390525f9081907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac906341ad069c90602401602060405180830381865afa15801561350e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135329190615540565b909210159392505050565b80516060905f816001600160401b0381111561355b5761355b61483c565b60405190808252806020026020018201604052801561358e57816020015b60608152602001906001900390816135795790505b5090505f5b82811015613672577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166331ff41c8878784815181106135d1576135d16150bc565b60200260200101516040518363ffffffff1660e01b81526004016136089291909182526001600160a01b0316602082015260400190565b5f60405180830381865afa158015613622573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136499190810190615599565b6060015182828151811061365f5761365f6150bc565b6020908102919091010152600101613593565b50949350505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061370157507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166136f55f8051602061567b833981519152546001600160a01b031690565b6001600160a01b031614155b1561371f5760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613771573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137959190614db0565b6001600160a01b0316336001600160a01b0316146137c85760405163021bfda160e41b815233600482015260240161079f565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613825575060408051601f3d908101601f1916820190925261382291810190615540565b60015b61384d57604051634c9c8ce360e01b81526001600160a01b038316600482015260240161079f565b5f8051602061567b833981519152811461387d57604051632a87526960e21b81526004810182905260240161079f565b6138878383614115565b505050565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461371f5760405163703e46dd60e11b815260040160405180910390fd5b5f61391e6040518060600160405280603c815260200161569b603c913980516020918201208451858301206040805193840192909252908201869052606082015260800161343f565b9392505050565b5f6139a76040518060800160405280605681526020016156d760569139805190602001208787878760405160200161395e9291906154db565b60408051601f19818403018152828252805160209182012089518a83012091840196909652908201939093526060810191909152608081019290925260a082015260c00161343f565b9695505050505050565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10280546060915f8051602061565b833981519152916139ef90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054613a1b90614e11565b8015612db75780601f10613a3d57610100808354040283529160200191612db7565b820191905f5260205f20905b815481529060010190602001808311613a495750939695505050505050565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10380546060915f8051602061565b833981519152916139ef90614e11565b5f81515f1480613acd5750815f81518110613ac357613ac36150bc565b016020015160f81c155b15613b4c577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b031663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613b22573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b469190615540565b92915050565b5f825f81518110613b5f57613b5f6150bc565b016020015160f81c905060018114801590613b7e575060ff8116600214155b15613ba15760405163084e730b60e21b815260ff8216600482015260240161079f565b60ff81166001148015613bb657508251602114155b15613bd457604051630459245b60e51b815260040160405180910390fd5b60ff81166002148015613be957508251604114155b15613c0757604051630459245b60e51b815260040160405180910390fd5b50506021015190565b5f80836001600160401b03811115613c2a57613c2a61483c565b604051908082528060200260200182016040528015613c53578160200160208202803683370190505b5090505f5b84811015613d445760405180606001604052806025815260200161585c6025913980519060200120868683818110613c9257613c926150bc565b9050602002810190613ca491906150d0565b613cb2906020810190614d06565b878784818110613cc457613cc46150bc565b9050602002810190613cd691906150d0565b613ce49060208101906150ee565b604051613cf29291906154db565b604051908190038120613d099392916020016154ea565b60405160208183030381529060405280519060200120828281518110613d3157613d316150bc565b6020908102919091010152600101613c58565b50613dca6040518060e0016040528060ad815260200161572d60ad91398051906020012089898985604051602001613d7c919061550b565b60408051601f1981840301815282825280516020918201208b518c83012091840197909752908201949094526060810192909252608082015260a081019290925260c082015260e00161343f565b98975050505050505050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b8310613e145772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef81000000008310613e40576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310613e5e57662386f26fc10000830492506010015b6305f5e1008310613e76576305f5e100830492506008015b6127108310613e8a57612710830492506004015b60648310613e9c576064830492506002015b600a8310613b465760010192915050565b5f8051602061588183398151915254600160401b900460ff1661371f57604051631afcd79f60e31b815260040160405180910390fd5b613eeb613ead565b5f8051602061565b8339815191527fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d102613f248482614ea8565b5060038101613f338382614ea8565b505f8082556001909101555050565b5f613b46613f4e61416a565b8360405161190160f01b8152600281019290925260228201526042902090565b5f805f80613f7c8686614178565b925092509250613f8c82826141c1565b5090949350505050565b604051632511f3f560e21b8152600481018490526001600160a01b03831660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac90639447cfd490604401602060405180830381865afa158015613ff3573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061401791906154bc565b61403f5760405163153e377b60e11b81526001600160a01b038316600482015260240161079f565b60405163063fe83960e31b8152600481018490526001600160a01b03821660248201525f907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac906331ff41c8906044015f60405180830381865afa15801561409d573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526140c49190810190615599565b9050826001600160a01b031681602001516001600160a01b03161461410f57604051630d86f52160e01b81526001600160a01b0380851660048301528316602482015260440161079f565b50505050565b61411e82614279565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156141625761388782826142dc565b61111b614345565b5f614173614364565b905090565b5f805f83516041036141af576020840151604085015160608601515f1a6141a1888285856143d7565b9550955095505050506141ba565b505081515f91506002905b9250925092565b5f8260038111156141d4576141d46146c6565b036141dd575050565b60018260038111156141f1576141f16146c6565b0361420f5760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115614223576142236146c6565b036142445760405163fce698f760e01b81526004810182905260240161079f565b6003826003811115614258576142586146c6565b0361111b576040516335e2f38360e21b81526004810182905260240161079f565b806001600160a01b03163b5f036142ae57604051634c9c8ce360e01b81526001600160a01b038216600482015260240161079f565b5f8051602061567b83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516142f89190615649565b5f60405180830381855af49150503d805f8114614330576040519150601f19603f3d011682016040523d82523d5f602084013e614335565b606091505b50915091506134b385838361449f565b341561371f5760405163b398979f60e01b815260040160405180910390fd5b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61438e6144fb565b614396614563565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561441057505f91506003905082614495565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015614461573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661448c57505f925060019150829050614495565b92505f91508190505b9450945094915050565b6060826144b4576144af826145a5565b61391e565b81511580156144cb57506001600160a01b0384163b155b156144f457604051639996b31560e01b81526001600160a01b038516600482015260240161079f565b5092915050565b5f5f8051602061565b833981519152816145136139b1565b80519091501561452b57805160209091012092915050565b8154801561453a579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f5f8051602061565b8339815191528161457b613a68565b80519091501561459357805160209091012092915050565b6001820154801561453a579392505050565b8051156145b55780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b60405180608001604052805f81526020015f81526020015f60018111156145f7576145f76146c6565b8152602001606081525090565b5f5b8381101561461e578181015183820152602001614606565b50505f910152565b5f815180845261463d816020860160208601614604565b601f01601f19169290920160200192915050565b602081525f61391e6020830184614626565b5f60208284031215614673575f80fd5b5035919050565b602080825282518282018190525f9190848201906040850190845b818110156146ba5783516001600160a01b031683529284019291840191600101614695565b50909695505050505050565b634e487b7160e01b5f52602160045260245ffd5b600281106137c857634e487b7160e01b5f52602160045260245ffd5b60208101614703836146da565b91905290565b600281106137c8575f80fd5b5f8060408385031215614726575f80fd5b82359150602083013561473881614709565b809150509250929050565b5f8083601f840112614753575f80fd5b5081356001600160401b03811115614769575f80fd5b602083019150836020828501011115614780575f80fd5b9250929050565b5f805f805f6060868803121561479b575f80fd5b8535945060208601356001600160401b03808211156147b8575f80fd5b818801915088601f8301126147cb575f80fd5b8135818111156147d9575f80fd5b8960208260051b85010111156147ed575f80fd5b60208301965080955050604088013591508082111561480a575f80fd5b5061481788828901614743565b969995985093965092949392505050565b6001600160a01b03811681146137c8575f80fd5b634e487b7160e01b5f52604160045260245ffd5b604051608081016001600160401b03811182821017156148725761487261483c565b60405290565b604051601f8201601f191681016001600160401b03811182821017156148a0576148a061483c565b604052919050565b5f6001600160401b038211156148c0576148c061483c565b50601f01601f191660200190565b5f80604083850312156148df575f80fd5b82356148ea81614828565b915060208301356001600160401b03811115614904575f80fd5b8301601f81018513614914575f80fd5b8035614927614922826148a8565b614878565b81815286602083850101111561493b575f80fd5b816020840160208301375f602083830101528093505050509250929050565b5f805f6040848603121561496c575f80fd5b8335925060208401356001600160401b03811115614988575f80fd5b61499486828701614743565b9497909650939450505050565b5f82825180855260208086019550808260051b8401018186015f5b84811015614a0957601f198684030189528151604081516149dc816146da565b855290850151858501829052906149f581860183614626565b9a86019a94505050908301906001016149bc565b5090979650505050505050565b6020815281516020820152602082015160408201525f6040830151614a3a816146da565b806060840152506060830151608080840152614a5960a08401826149a1565b949350505050565b5f805f805f60608688031215614a75575f80fd5b8535945060208601356001600160401b0380821115614a92575f80fd5b614a9e89838a01614743565b9096509450604088013591508082111561480a575f80fd5b5f6060820185151583526020606060208501528186518084526080860191506020880193505f5b81811015614b0d578451805184528401516001600160401b03168484015293830193604090920191600101614add565b50508093505050506001600160401b0383166040830152949350505050565b5f815180845260208085019450602084015f5b83811015614b5b57815187529582019590820190600101614b3f565b509495945050505050565b60ff60f81b8816815260e060208201525f614b8460e0830189614626565b8281036040840152614b968189614626565b606084018890526001600160a01b038716608085015260a0840186905283810360c08501529050614bc78185614b2c565b9a9950505050505050505050565b5f8282518085526020808601955060208260051b840101602086015f5b84811015614a0957601f19868403018952614c0e838351614626565b98840198925090830190600101614bf2565b604081525f614c326040830185614bd5565b82810360208401526134b381856149a1565b6001600160401b03811681146137c8575f80fd5b5f805f8060608587031215614c6b575f80fd5b8435935060208501356001600160401b0380821115614c88575f80fd5b818701915087601f830112614c9b575f80fd5b813581811115614ca9575f80fd5b8860208260061b8501011115614cbd575f80fd5b6020830195508094505050506040850135614cd781614c44565b939692955090935050565b604081525f614cf46040830185614bd5565b82810360208401526134b38185614626565b5f60208284031215614d16575f80fd5b813561391e81614709565b602081525f61391e6020830184614b2c565b5f8551614d44818460208a01614604565b61103b60f11b9083019081528551614d63816002840160208a01614604565b808201915050601760f91b8060028301528551614d87816003850160208a01614604565b60039201918201528351614da2816004840160208801614604565b016004019695505050505050565b5f60208284031215614dc0575f80fd5b815161391e81614828565b5f60018201614de857634e487b7160e01b5f52601160045260245ffd5b5060010190565b5f8060408385031215614e00575f80fd5b505080516020909101519092909150565b600181811c90821680614e2557607f821691505b602082108103614e4357634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561388757805f5260205f20601f840160051c81016020851015614e6e5750805b601f840160051c820191505b81811015614e8d575f8155600101614e7a565b5050505050565b5f19600383901b1c191660019190911b1790565b81516001600160401b03811115614ec157614ec161483c565b614ed581614ecf8454614e11565b84614e49565b602080601f831160018114614f03575f8415614ef15750858301515b614efb8582614e94565b865550614f5a565b5f85815260208120601f198616915b82811015614f3157888601518255948401946001909101908401614f12565b5085821015614f4e57878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b848152836020820152614f74836146da565b826040820152608060608201525f6139a76080830184614626565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8383855260208086019550808560051b830101845f5b87811015614a0957848303601f19018952813536889003603e19018112614ff3575f80fd5b87016040813561500281614709565b61500b816146da565b85528186013536839003601e19018112615023575f80fd5b9091018581019190356001600160401b0381111561503f575f80fd5b80360383131561504d575f80fd5b818787015261505f8287018285614f8f565b9b87019b955050509184019150600101614fce565b868152608060208201525f61508d608083018789614fb7565b82810360408401526150a0818688614f8f565b91505060018060a01b0383166060830152979650505050505050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e198336030181126150e4575f80fd5b9190910192915050565b5f808335601e19843603018112615103575f80fd5b8301803591506001600160401b0382111561511c575f80fd5b602001915036819003821315614780575f80fd5b6001600160401b038311156151475761514761483c565b61515b836151558354614e11565b83614e49565b5f601f841160018114615187575f85156151755750838201355b61517f8682614e94565b845550614e8d565b5f83815260208120601f198716915b828110156151b65786850135825560209485019460019092019101615196565b50868210156151d2575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b81356151ef81614709565b6151f8816146da565b60ff1982541660ff82168117835550506001808201602080850135601e19863603018112615224575f80fd5b850180356001600160401b0381111561523b575f80fd5b803603838301131561524b575f80fd5b61525f816152598654614e11565b86614e49565b5f601f82116001811461528d575f831561527b57508382018501355b6152858482614e94565b875550611393565b5f86815260208120601f198516915b828110156152bb5786850188013582559387019390890190870161529c565b50848210156152d9575f1960f88660031b161c198785880101351681555b50505050600190811b019092555050505050565b848152606060208201525f6153056060830186614bd5565b828103604084015261345a818587614fb7565b848152606060208201525f615331606083018587614f8f565b905060018060a01b038316604083015295945050505050565b848152836020820152826040820152608060608201525f6139a76080830184614626565b838152826020820152606060408201525f6134b36060830184614626565b868152608060208201525f61508d608083018789614f8f565b848152606060208201525f6153bd6060830186614bd5565b828103604084015261345a818587614f8f565b5f602082840312156153e0575f80fd5b813561391e81614c44565b8135815560018101602083013561540181614c44565b815467ffffffffffffffff19166001600160401b03919091161790555050565b848152606060208083018290529082018490525f90859060808401835b8781101561547657833582528284013561545781614c44565b6001600160401b0316828401526040938401939091019060010161543e565b508093505050506001600160401b038316604083015295945050505050565b8381526154a1836146da565b826020820152606060408201525f6134b36060830184614626565b5f602082840312156154cc575f80fd5b8151801515811461391e575f80fd5b818382375f9101908152919050565b838152606081016154fa846146da565b602082019390935260400152919050565b81515f9082906020808601845b8381101561553457815185529382019390820190600101615518565b50929695505050505050565b5f60208284031215615550575f80fd5b5051919050565b5f82601f830112615566575f80fd5b8151615574614922826148a8565b818152846020838601011115615588575f80fd5b614a59826020830160208701614604565b5f602082840312156155a9575f80fd5b81516001600160401b03808211156155bf575f80fd5b90830190608082860312156155d2575f80fd5b6155da614850565b82516155e581614828565b815260208301516155f581614828565b602082015260408301518281111561560b575f80fd5b61561787828601615557565b60408301525060608301518281111561562e575f80fd5b61563a87828601615557565b60608301525095945050505050565b5f82516150e481846020870161460456fea16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c62797465732065787472614461746129436f6d707265737365644b65794d6174657269616c73566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206d6967726174696f6e5265717565737449642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0`\x80R4\x80\x15b\0\0\x14W_\x80\xFD[Pb\0\0\x1Fb\0\0%V[b\0\0\xD9V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80Th\x01\0\0\0\0\0\0\0\0\x90\x04`\xFF\x16\x15b\0\0vW`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80T`\x01`\x01`@\x1B\x03\x90\x81\x16\x14b\0\0\xD6W\x80T`\x01`\x01`@\x1B\x03\x19\x16`\x01`\x01`@\x1B\x03\x90\x81\x17\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PV[`\x80QaX\xA1b\0\x01\0_9_\x81\x81a6\x86\x01R\x81\x81a6\xAF\x01Ra8\x97\x01RaX\xA1_\xF3\xFE`\x80`@R`\x046\x10a\x01\xDBW_5`\xE0\x1C\x80cv\t\x15-\x11a\0\xFDW\x80c\xC1%,\xA9\x11a\0\x92W\x80c\xD5/\x10\xEB\x11a\0bW\x80c\xD5/\x10\xEB\x14a\x05\x87W\x80c\xDA\xBDs/\x14a\x05\x9BW\x80c\xE4\x10\x11~\x14a\x05\xBCW\x80c\xE7\x11\xC9\xE7\x14a\x05\xD0W_\x80\xFD[\x80c\xC1%,\xA9\x14a\x04\xFDW\x80c\xC2\xC1\xFA\xEE\x14a\x05\x1CW\x80c\xC5[\x87$\x14a\x05;W\x80c\xCA\xA3g\xDB\x14a\x05hW_\x80\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xCDW\x80c\xAD<\xB1\xCC\x14a\x04\x86W\x80c\xBA\xC2+\xB8\x14a\x04\xB6W\x80c\xBA\xFF!\x1E\x14a\x04\xCAW\x80c\xBCT\x04\xA0\x14a\x04\xDEW_\x80\xFD[\x80cv\t\x15-\x14a\x03\xE5W\x80c\x84\xB0\x19n\x14a\x04\x13W\x80c\x90\x81\xFE\x13\x14a\x04:W\x80c\x93f\x08\xAE\x14a\x04YW_\x80\xFD[\x80c=^\xC7\xE3\x11a\x01sW\x80cR\xD1\x90-\x11a\x01CW\x80cR\xD1\x90-\x14a\x03gW\x80cX\x9A\xDB\x0E\x14a\x03{W\x80cb\x94\xF4b\x14a\x03\x9AW\x80cb\x97\x87\x87\x14a\x03\xC6W_\x80\xFD[\x80c=^\xC7\xE3\x14a\x02\xE7W\x80cE\xAF&\x1B\x14a\x03\x16W\x80cF\x10\xFF\xE8\x14a\x035W\x80cO\x1E\xF2\x86\x14a\x03TW_\x80\xFD[\x80c\x19\xF4\xF62\x11a\x01\xAEW\x80c\x19\xF4\xF62\x14a\x02tW\x80c9\xF78\x10\x14a\x02\xA0W\x80c:\xC5\0r\x14a\x02\xB4W\x80c<\x02\xF84\x14a\x02\xC8W_\x80\xFD[\x80c\x0Bh\x073\x14a\x01\xDFW\x80c\r\x8En,\x14a\x02\x06W\x80c\x16\xC7\x13\xD9\x14a\x02'W\x80c\x17\x03\xC6\x1A\x14a\x02SW[_\x80\xFD[4\x80\x15a\x01\xEAW_\x80\xFD[Pa\x01\xF3a\x05\xEFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x11W_\x80\xFD[Pa\x02\x1Aa\x06\x03V[`@Qa\x01\xFD\x91\x90aFQV[4\x80\x15a\x022W_\x80\xFD[Pa\x02Fa\x02A6`\x04aFcV[a\x06nV[`@Qa\x01\xFD\x91\x90aFzV[4\x80\x15a\x02^W_\x80\xFD[Pa\x02ra\x02m6`\x04aFcV[a\x06\xFCV[\0[4\x80\x15a\x02\x7FW_\x80\xFD[Pa\x02\x93a\x02\x8E6`\x04aFcV[a\x08wV[`@Qa\x01\xFD\x91\x90aF\xF6V[4\x80\x15a\x02\xABW_\x80\xFD[Pa\x02ra\t\x0FV[4\x80\x15a\x02\xBFW_\x80\xFD[Pa\x01\xF3a\nwV[4\x80\x15a\x02\xD3W_\x80\xFD[Pa\x02ra\x02\xE26`\x04aG\x15V[a\n\x8BV[4\x80\x15a\x02\xF2W_\x80\xFD[Pa\x03\x06a\x03\x016`\x04aFcV[a\x0C\xCAV[`@Q\x90\x15\x15\x81R` \x01a\x01\xFDV[4\x80\x15a\x03!W_\x80\xFD[Pa\x02\x93a\x0306`\x04aFcV[a\x0C\xEBV[4\x80\x15a\x03@W_\x80\xFD[Pa\x02ra\x03O6`\x04aG\x87V[a\rqV[a\x02ra\x03b6`\x04aH\xCEV[a\x11\0V[4\x80\x15a\x03rW_\x80\xFD[Pa\x01\xF3a\x11\x1FV[4\x80\x15a\x03\x86W_\x80\xFD[Pa\x02ra\x03\x956`\x04aIZV[a\x11:V[4\x80\x15a\x03\xA5W_\x80\xFD[Pa\x03\xB9a\x03\xB46`\x04aFcV[a\x13\x9EV[`@Qa\x01\xFD\x91\x90aJ\x16V[4\x80\x15a\x03\xD1W_\x80\xFD[Pa\x02ra\x03\xE06`\x04aJaV[a\x15\x8BV[4\x80\x15a\x03\xF0W_\x80\xFD[Pa\x04\x04a\x03\xFF6`\x04aFcV[a\x18@V[`@Qa\x01\xFD\x93\x92\x91\x90aJ\xB6V[4\x80\x15a\x04\x1EW_\x80\xFD[Pa\x04'a\x18\xFCV[`@Qa\x01\xFD\x97\x96\x95\x94\x93\x92\x91\x90aKfV[4\x80\x15a\x04EW_\x80\xFD[Pa\x02ra\x04T6`\x04aFcV[a\x19\xA5V[4\x80\x15a\x04dW_\x80\xFD[Pa\x04xa\x04s6`\x04aFcV[a\x1C\xFAV[`@Qa\x01\xFD\x92\x91\x90aL V[4\x80\x15a\x04\x91W_\x80\xFD[Pa\x02\x1A`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\xC1W_\x80\xFD[Pa\x02ra\x1F\xC2V[4\x80\x15a\x04\xD5W_\x80\xFD[Pa\x01\xF3a pV[4\x80\x15a\x04\xE9W_\x80\xFD[Pa\x02ra\x04\xF86`\x04aLXV[a \x84V[4\x80\x15a\x05\x08W_\x80\xFD[Pa\x02ra\x05\x176`\x04aG\x87V[a#\xA3V[4\x80\x15a\x05'W_\x80\xFD[Pa\x02ra\x0566`\x04aFcV[a'\x81V[4\x80\x15a\x05FW_\x80\xFD[Pa\x05Za\x05U6`\x04aFcV[a)&V[`@Qa\x01\xFD\x92\x91\x90aL\xE2V[4\x80\x15a\x05sW_\x80\xFD[Pa\x02ra\x05\x826`\x04aM\x06V[a*\xE3V[4\x80\x15a\x05\x92W_\x80\xFD[Pa\x01\xF3a-OV[4\x80\x15a\x05\xA6W_\x80\xFD[Pa\x05\xAFa-cV[`@Qa\x01\xFD\x91\x90aM!V[4\x80\x15a\x05\xC7W_\x80\xFD[Pa\x05\xAFa-\xC2V[4\x80\x15a\x05\xDBW_\x80\xFD[Pa\x04xa\x05\xEA6`\x04aFcV[a.\x1FV[_\x80a\x05\xF9a0AV[`\x05\x01T\x92\x91PPV[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01l%\xA6\xA9\xA3\xB2\xB72\xB90\xBA4\xB7\xB7`\x99\x1B\x81RPa\x064_a0eV[a\x06>`\x03a0eV[a\x06G_a0eV[`@Q` \x01a\x06Z\x94\x93\x92\x91\x90aM3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x06ya0AV[_\x84\x81R`\x03\x82\x01` \x90\x81R`@\x80\x83 T`\x02\x85\x01\x83R\x81\x84 \x81\x85R\x83R\x92\x81\x90 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x82\x82\x80\x15a\x06\xEEW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06\xD0W[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07LW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07p\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xA8W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x07\xB1a0AV[\x90P\x80`\t\x01T\x82\x11\x80a\x07\xC9WP`\x05`\xF8\x1B\x82\x11\x15[\x15a\x07\xEAW`@Qce\xF4\x93+`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x01\x82\x01` R`@\x90 T`\xFF\x16\x15a\x08\x1EW`@Qc\xDF\r\xB5\xFB`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x01\x82\x81\x01` R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90UQ\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x90a\x08k\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x08\x81a0AV[_\x84\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x08\xB7W`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x03\x82\x01` R`@\x90 Ta\x08\xE7W`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x92\x83R`\x06\x81\x01` \x90\x81R`@\x80\x85 T\x85R`\r\x90\x92\x01\x90R\x90\x91 T`\xFF\x16\x91\x90PV[_\x80Q` aX\x81\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\tPW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` aX\x81\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\t\x86WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\t\xA4W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\r\x81Rl%\xA6\xA9\xA3\xB2\xB72\xB90\xBA4\xB7\xB7`\x99\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\n\n\x91a0\xF4V[_a\n\x13a0AV[`\x03`\xF8\x1B`\x04\x82\x01U`\x01`\xFA\x1B`\x05\x82\x01U`\x05`\xF8\x1B`\t\x90\x91\x01UP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x08kV[_\x80a\n\x81a0AV[`\t\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xFF\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B2W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a\x0B;a0AV[`\t\x81\x01T\x90\x91P`\x05`\xF8\x1B\x81\x14\x80\x15\x90a\x0BgWP_\x81\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16\x15[\x15a\x0B\x88W`@Qc\x06\x1A\xC6\x1D`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\t\x82\x01\x80T\x90_a\x0B\x99\x83aM\xCBV[\x90\x91UPP`\t\x82\x01T_\x81\x81R`\n\x84\x01` \x90\x81R`@\x80\x83 \x88\x90U`\r\x86\x01\x90\x91R\x90 \x80T\x85\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x0B\xDDWa\x0B\xDDaF\xC6V[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CW\x91\x90aM\xEFV[\x91P\x91P_a\x0Cf\x83\x83a1\x06V[_\x85\x81R`\x0E\x88\x01` R`@\x90 \x90\x91Pa\x0C\x82\x82\x82aN\xA8V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x84\x89\x89\x84`@Qa\x0C\xB8\x94\x93\x92\x91\x90aObV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x0C\xD4a0AV[_\x93\x84R`\x01\x01` RPP`@\x90 T`\xFF\x16\x90V[_\x80a\x0C\xF5a0AV[_\x84\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\r+W`@Qc\xDA2\xD0\x0F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x03\x82\x01` R`@\x90 Ta\r[W`@Qc\xD5\xFD<\xD7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x92\x83R`\r\x01` RP`@\x90 T`\xFF\x16\x90V[_a\rza0AV[\x90P\x80`\x05\x01T\x86\x11\x80a\r\x92WP`\x01`\xFA\x1B\x86\x11\x15[\x15a\r\xB3W`@Qc+~\xAEA`\xE2\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x86\x81R`\x11\x82\x01` R`@\x90 T\x15a\r\xE4W`@QcZ$\x0B[`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x84\x90\x03a\x0E\x08W`@Qc\xE6\xF9\x08;`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x80a\x0E\x13\x88a1>V[_\x8A\x81R`\x06\x86\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x01\x89\x01\x90\x92R\x90\x91 T\x92\x94P\x90\x92P\x90`\xFF\x16a\x0E[W`@Qco\xBC\xDD+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0Ei\x82\x8B\x8B\x8B\x88a2\x8FV[\x90P_a\x0Ex\x84\x83\x8A\x8Aa4eV[_\x8C\x81R` \x88\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a\x0E\xCEW`@Qc\x98\xFB\x95}`\xE0\x1B\x81R`\x04\x81\x01\x8C\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[_\x8B\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x8E\x84R`\x02\x8A\x01\x83R\x81\x84 \x86\x85R\x83R\x81\x84 \x80T\x91\x82\x01\x81U\x80\x85R\x92\x90\x93 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x91Q\x90\x91\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x91a\x0Fl\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91aPtV[`@Q\x80\x91\x03\x90\xA1_\x8C\x81R`\x01\x88\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a\x0F\x9CWP\x80Ta\x0F\x9C\x90\x86\x90a4\xBCV[\x15a\x10\xF2W_\x8C\x81R`\x01\x88\x81\x01` R`@\x82 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[\x8A\x81\x10\x15a\x10#W_\x8D\x81R`\x07\x89\x01` R`@\x90 \x8C\x8C\x83\x81\x81\x10a\x0F\xE6Wa\x0F\xE6aP\xBCV[\x90P` \x02\x81\x01\x90a\x0F\xF8\x91\x90aP\xD0V[\x81T`\x01\x81\x01\x83U_\x92\x83R` \x90\x92 \x90\x91`\x02\x02\x01a\x10\x19\x82\x82aQ\xE4V[PP`\x01\x01a\x0F\xBDV[P_\x8C\x81R`\x03\x88\x01` \x90\x81R`@\x80\x83 \x86\x90U`\x08\x8A\x01\x8F\x90U`\x0F\x8A\x01\x80T`\x01\x81\x01\x82U\x90\x84R\x82\x84 \x01\x8F\x90U\x83T\x81Q\x81\x84\x02\x81\x01\x84\x01\x90\x92R\x80\x82Ra\x10\xB1\x92\x89\x92\x91\x86\x91\x83\x01\x82\x82\x80\x15a\x10\xA7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x10\x89W[PPPPPa5=V[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8D\x82\x8E\x8E`@Qa\x10\xE8\x94\x93\x92\x91\x90aR\xEDV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPPV[a\x11\x08a6{V[a\x11\x11\x82a7!V[a\x11\x1B\x82\x82a7\xCBV[PPV[_a\x11(a8\x8CV[P_\x80Q` aV{\x839\x81Q\x91R\x90V[_a\x11Ca0AV[\x90P\x80`\x04\x01T\x84\x11\x80a\x11[WP`\x03`\xF8\x1B\x84\x11\x15[\x15a\x11|W`@Qc\n\xB7\xF6\x87`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x07\x9FV[_\x80a\x11\x87\x86a1>V[\x91P\x91P_a\x11\x96\x87\x84a8\xD5V[\x90P_a\x11\xA5\x83\x83\x89\x89a4eV[_\x89\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a\x11\xFBW`@Qc3\xCA\x1F\xE3`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[_\x88\x81R` \x86\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x8B\x84R`\x02\x89\x01\x83R\x81\x84 \x86\x85R\x83R\x81\x84 \x80T\x91\x82\x01\x81U\x80\x85R\x92\x90\x93 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x91Q\x90\x91\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x91a\x12\x95\x91\x8C\x91\x8C\x91\x8C\x91aS\x18V[`@Q\x80\x91\x03\x90\xA1_\x89\x81R`\x01\x87\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a\x12\xC5WP\x80Ta\x12\xC5\x90\x85\x90a4\xBCV[\x15a\x13\x93W_\x89\x81R`\x01\x87\x81\x01` \x90\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U`\x03\x89\x01\x81R\x81\x83 \x86\x90U`\x06\x89\x01\x81R\x81\x83 T\x80\x84R`\x11\x8A\x01\x90\x91R\x91 T\x80\x15a\x13TW\x7F\xB0|Z\x1B%\xDFC\xBF\xE6\xD7\x02\xBA\xC0l=n\x15\x1Aq\xDA@jl\xBA\x07\xFF\xA4\x17\0\xDC\xAB\xE1\x8B\x83\x83\x8A`@Qa\x13G\x94\x93\x92\x91\x90aSJV[`@Q\x80\x91\x03\x90\xA1a\x13\x90V[\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8B\x83\x89`@Qa\x13\x87\x93\x92\x91\x90aSnV[`@Q\x80\x91\x03\x90\xA1[PP[PPPPPPPPPV[a\x13\xA6aE\xCEV[_a\x13\xAFa0AV[_\x84\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x13\xE5W`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x03\x82\x01` R`@\x90 Ta\x14\x15W`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x06\x82\x01` \x90\x81R`@\x80\x83 T\x81Q`\x80\x81\x01\x83R\x81\x81R\x80\x84\x01\x88\x90R\x81\x85R`\r\x86\x01\x90\x93R\x92\x81\x90 T\x90\x82\x01\x90`\xFF\x16`\x01\x81\x11\x15a\x14`Wa\x14`aF\xC6V[\x81R_\x86\x81R`\x07\x85\x01` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x94\x83\x01\x94\x91\x93\x90\x92\x84\x01[\x82\x82\x10\x15a\x15}W_\x84\x81R` \x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x84\x02\x90\x91\x01\x80T\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x14\xC9Wa\x14\xC9aF\xC6V[`\x01\x81\x11\x15a\x14\xDAWa\x14\xDAaF\xC6V[\x81R` \x01`\x01\x82\x01\x80Ta\x14\xEE\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x1A\x90aN\x11V[\x80\x15a\x15eW\x80`\x1F\x10a\x15<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15eV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15HW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14\x90V[PPP\x91RP\x94\x93PPPPV[_a\x15\x94a0AV[\x90P\x80`\t\x01T\x86\x11\x80a\x15\xACWP`\x05`\xF8\x1B\x86\x11\x15[\x15a\x15\xCDW`@QcF\xC6J\x05`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x80a\x15\xD8\x88a1>V[\x91P\x91P_a\x15\xFD\x89\x85`\n\x01_\x8C\x81R` \x01\x90\x81R` \x01_ T\x8A\x8A\x87a9%V[\x90P_a\x16\x0C\x83\x83\x89\x89a4eV[_\x8B\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a\x16bW`@Qc\xFC\xF5\xA6\xE9`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[_\x8A\x81R` \x86\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x8D\x84R`\x02\x89\x01\x83R\x81\x84 \x86\x85R\x83R\x81\x84 \x80T\x91\x82\x01\x81U\x80\x85R\x92\x90\x93 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x91Q\x90\x91\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x91a\x17\0\x91\x8E\x91\x8E\x91\x8E\x91\x8E\x91\x8E\x91aS\x8CV[`@Q\x80\x91\x03\x90\xA1_\x8B\x81R`\x01\x87\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a\x170WP\x80Ta\x170\x90\x85\x90a4\xBCV[\x15a\x13\x90W_\x8B\x81R`\x01\x87\x81\x01` \x90\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U`\x0B\x89\x01\x90R\x90 a\x17g\x8A\x8C\x83aQ0V[P_\x8B\x81R`\x03\x87\x01` \x90\x81R`@\x80\x83 \x86\x90U`\x0C\x89\x01\x8E\x90U`\x10\x89\x01\x80T`\x01\x81\x01\x82U\x90\x84R\x82\x84 \x01\x8E\x90U\x83T\x81Q\x81\x84\x02\x81\x01\x84\x01\x90\x92R\x80\x82Ra\x17\xF3\x92\x88\x92\x91\x86\x91\x83\x01\x82\x82\x80\x15a\x10\xA7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x10\x89WPPPPPa5=V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x18*\x94\x93\x92\x91\x90aS\xA5V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPPV[_``_\x80a\x18Ma0AV[_\x86\x81R`\x16\x82\x01` \x90\x81R`@\x80\x83 T`\x14\x85\x01\x83R\x81\x84 `\x15\x86\x01\x84R\x82\x85 T\x81T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x96\x97P`\xFF\x90\x92\x16\x95\x90\x94`\x01`\x01`@\x1B\x03\x90\x92\x16\x93\x85\x92\x90\x84\x01[\x82\x82\x10\x15a\x18\xE8W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x85\x02\x90\x91\x01\x80T\x82R`\x01\x90\x81\x01T`\x01`\x01`@\x1B\x03\x16\x82\x84\x01R\x90\x83R\x90\x92\x01\x91\x01a\x18\xA1V[PPPP\x91P\x93P\x93P\x93PP\x91\x93\x90\x92PV[_``\x80\x82\x80\x80\x83\x81_\x80Q` aV[\x839\x81Q\x91R\x80T\x90\x91P\x15\x80\x15a\x19'WP`\x01\x81\x01T\x15[a\x19kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07\x9FV[a\x19sa9\xB1V[a\x19{a:hV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x91\x99PF\x98P0\x97P\x95P\x93P\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x19\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1ALW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a\x1AUa0AV[_\x83\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16\x15\x80a\x1AzWP\x80`\x05\x01T\x82\x11[\x80a\x1A\x89WP`\x01`\xFA\x1B\x82\x11\x15[\x15a\x1A\xAAW`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x03\x82\x01` R`@\x90 Ta\x1A\xDAW`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x12\x82\x01` R`@\x90 T\x15a\x1B\x0BW`@Qc\"1\xDC=`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[`\x05\x81\x01T`\x01`\xFA\x1B\x81\x14\x80\x15\x90a\x1B4WP_\x81\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16\x15[\x15a\x1BUW`@Qc\x07p\xA7\xB5`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\x04\x82\x01\x80T\x90_a\x1Bf\x83aM\xCBV[\x90\x91UPP`\x04\x82\x01T`\x05\x83\x01\x80T\x90_a\x1B\x81\x83aM\xCBV[\x90\x91UPP`\x05\x83\x01T_\x82\x81R`\x06\x85\x01` \x81\x81R`@\x80\x84 \x85\x90U\x84\x84R\x80\x84 \x86\x90U`\x11\x88\x01\x82R\x80\x84 \x89\x90U\x88\x84R\x91\x81R\x81\x83 T\x83R`\r\x87\x01\x90R\x80\x82 T\x84\x83R\x91 \x80T`\xFF\x90\x92\x16\x91\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x1B\xF2Wa\x1B\xF2aF\xC6V[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CHW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cl\x91\x90aM\xEFV[\x91P\x91P_a\x1C{\x83\x83a1\x06V[_\x87\x81R`\x0E\x8A\x01` R`@\x90 \x90\x91Pa\x1C\x97\x82\x82aN\xA8V[P_\x85\x81R`\x0E\x89\x01` R`@\x90 a\x1C\xB1\x82\x82aN\xA8V[P\x7F(i\xC5\x13\"\xE9\r,\xE2\xDF\xE5^\xC2\x9D\x18\x0F6\xBF\xFA\x15\xB44\xFC\n\x8D\xD0\xE5\xD3\xCE<\x17L\x86\x8A\x86\x84`@Qa\x1C\xE7\x94\x93\x92\x91\x90aObV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[``\x80_a\x1D\x06a0AV[_\x85\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x1D<W`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x07\x9FV[_\x84\x81R`\x03\x82\x01` R`@\x90 T\x80a\x1DmW`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x85\x81R`\x02\x83\x01` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x1D\xD4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1D\xB6W[PPPPP\x90P_a\x1E~\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\xFD\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E)\x90aN\x11V[\x80\x15a\x1EtW\x80`\x1F\x10a\x1EKWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1EtV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1EWW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:\xA6V[\x90P_a\x1E\x8B\x82\x84a5=V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1F\xAEW_\x84\x81R` \x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x84\x02\x90\x91\x01\x80T\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x1E\xFAWa\x1E\xFAaF\xC6V[`\x01\x81\x11\x15a\x1F\x0BWa\x1F\x0BaF\xC6V[\x81R` \x01`\x01\x82\x01\x80Ta\x1F\x1F\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1FK\x90aN\x11V[\x80\x15a\x1F\x96W\x80`\x1F\x10a\x1FmWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\x96V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1FyW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xC1V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[_\x80Q` aX\x81\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x1F\xF8WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a \x16W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17`\xFF`@\x1B\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x08kV[_\x80a za0AV[`\x0C\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xD4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xF8\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a!+W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a!4a0AV[_\x86\x81R`\x12\x82\x01` R`@\x81 T\x91\x92P\x03a!hW`@Qc|\x8Bw!`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x85\x81R`\x16\x82\x01` R`@\x90 T`\xFF\x16\x15a!\x9CW`@Qc 1I[`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x83\x90\x03a!\xBDW`@Qc\x04\xA6\xB4\xB1`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16_\x03a!\xE6W`@Qc$k\xC3\xFB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a#\x1DW\x84\x84\x82\x81\x81\x10a\"\x02Wa\"\x02aP\xBCV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\"\x1A\x91\x90aS\xD0V[`\x01`\x01`@\x1B\x03\x16_\x03a\"BW`@Qc$k\xC3\xFB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\"\xC4W\x85\x85\x83\x81\x81\x10a\"^Wa\"^aP\xBCV[\x90P`@\x02\x01_\x015\x86\x86\x83\x81\x81\x10a\"yWa\"yaP\xBCV[\x90P`@\x02\x01_\x015\x03a\"\xBCW\x85\x85\x83\x81\x81\x10a\"\x99Wa\"\x99aP\xBCV[`@\x80QcJ\x05kk`\xE1\x1B\x81R\x91\x02\x92\x90\x92\x015`\x04\x83\x01RP`$\x01a\x07\x9FV[`\x01\x01a\"DV[P_\x86\x81R`\x14\x83\x01` R`@\x90 \x85\x85\x83\x81\x81\x10a\"\xE6Wa\"\xE6aP\xBCV[\x83T`\x01\x81\x01\x85U_\x94\x85R` \x90\x94 `@\x90\x91\x02\x92\x90\x92\x01\x92`\x02\x02\x90\x91\x01\x90Pa#\x13\x82\x82aS\xEBV[PP`\x01\x01a!\xE8V[P_\x85\x81R`\x15\x82\x01` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x87\x16\x17\x90U`\x16\x84\x01\x90\x91R\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fx\xCE\xC3O\xD2q[\x93^J\xF4\xAEi_u\xA2\xC5|\xBD\x96U\xF1\xE7\xE3\xCC\x86\xC9|\xE7\x9F\xA2\x9F\x90a#\x94\x90\x87\x90\x87\x90\x87\x90\x87\x90aT!V[`@Q\x80\x91\x03\x90\xA1PPPPPV[_a#\xACa0AV[\x90P\x80`\x05\x01T\x86\x11\x80a#\xC4WP`\x01`\xFA\x1B\x86\x11\x15[\x15a#\xE5W`@Qc+~\xAEA`\xE2\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x86\x81R`\x11\x82\x01` R`@\x81 T\x90\x81\x90\x03a$\x19W`@QcZ$\x0B[`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07\x9FV[_\x85\x90\x03a$=W`@Qc\xE6\xF9\x08;`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07\x9FV[_\x80a$H\x89a1>V[_\x8B\x81R`\x06\x87\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x01\x8A\x01\x90\x92R\x90\x91 T\x92\x94P\x90\x92P\x90`\xFF\x16a$\x90W`@Qco\xBC\xDD+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a$\x9F\x82\x8C\x87\x8D\x8D\x89a<\x10V[\x90P_a$\xAE\x84\x83\x8B\x8Ba4eV[_\x8D\x81R` \x89\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a%\x04W`@Qc\x98\xFB\x95}`\xE0\x1B\x81R`\x04\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[`\x01\x87_\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x87`\x02\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x03\xC8\x8A\xBBc\xCB\x1A3\xCB\xA1\xE3s\x93\xE7\xFC\x01\xF2\x03\xBB\x03{\x92\xF5\xEF\x84\xD5.\\c\xAC{\xE7\x8D\x8D\x8D\x8D\x8D3`@Qa%\xF2\x96\x95\x94\x93\x92\x91\x90aPtV[`@Q\x80\x91\x03\x90\xA1_\x8D\x81R`\x01\x89\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a&\"WP\x80Ta&\"\x90\x86\x90a4\xBCV[\x15a'rW_\x8D\x81R`\x01\x89\x81\x01` \x90\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U`\x03\x8B\x01\x90R\x81 \x84\x90U[\x8B\x81\x10\x15a&\xB8W_\x88\x81R`\x13\x8A\x01` R`@\x90 \x8D\x8D\x83\x81\x81\x10a&{Wa&{aP\xBCV[\x90P` \x02\x81\x01\x90a&\x8D\x91\x90aP\xD0V[\x81T`\x01\x81\x01\x83U_\x92\x83R` \x90\x92 \x90\x91`\x02\x02\x01a&\xAE\x82\x82aQ\xE4V[PP`\x01\x01a&RV[P\x8C\x88`\x12\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_a'1\x86\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x10\xA7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x10\x89WPPPPPa5=V[\x90P\x7F\x80\xEB\xC2\xA4\xE1\x83\0\x0Fh7\xFA\xB1\xE3ip\xE8\xBCJ\x1B\x19\"0T\xC3'i\xDBf:L\xE3F\x88\x82\x8F\x8F`@Qa'h\x94\x93\x92\x91\x90aR\xEDV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xD1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xF5\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a((W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a(1a0AV[\x90P\x80`\x04\x01T\x82\x11\x80a(IWP`\x03`\xF8\x1B\x82\x11\x15[\x15a(jW`@Qc~ym\xBD`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x06\x82\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x01\x85\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a(\xAFW`@Qc\x92x\x9Bg`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x01\x83\x81\x01` R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U\x80\x15a(\xEEW_\x81\x81R`\x01\x83\x81\x01` R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[`@Q\x83\x81R\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[``\x80_a)2a0AV[_\x85\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a)hW`@Qc\xDA2\xD0\x0F`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x07\x9FV[_\x84\x81R`\x03\x82\x01` R`@\x90 T\x80a)\x99W`@Qc\xD5\xFD<\xD7`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x85\x81R`\x02\x83\x01` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a*\0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a)\xE2W[PPPPP\x90P_a*)\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\xFD\x90aN\x11V[\x90P_a*6\x82\x84a5=V[_\x89\x81R`\x0B\x87\x01` R`@\x90 \x80T\x91\x92P\x82\x91\x81\x90a*W\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\x83\x90aN\x11V[\x80\x15a*\xCEW\x80`\x1F\x10a*\xA5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*\xCEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+W\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+\x8AW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a+\x93a0AV[`\x05\x81\x01T\x90\x91P`\x01`\xFA\x1B\x81\x14\x80\x15\x90a+\xBFWP_\x81\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16\x15[\x15a+\xE0W`@Qc\x07p\xA7\xB5`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\x04\x82\x01\x80T\x90_a+\xF1\x83aM\xCBV[\x90\x91UPP`\x04\x82\x01T`\x05\x83\x01\x80T\x90_a,\x0C\x83aM\xCBV[\x90\x91UPP`\x05\x83\x01T_\x82\x81R`\x06\x85\x01` \x90\x81R`@\x80\x83 \x84\x90U\x83\x83R\x80\x83 \x85\x90U\x84\x83R`\r\x87\x01\x90\x91R\x90 \x80T\x86\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a,\\Wa,\\aF\xC6V[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xB2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xD6\x91\x90aM\xEFV[\x91P\x91P_a,\xE5\x83\x83a1\x06V[_\x86\x81R`\x0E\x89\x01` R`@\x90 \x90\x91Pa-\x01\x82\x82aN\xA8V[P_\x84\x81R`\x0E\x88\x01` R`@\x90 a-\x1B\x82\x82aN\xA8V[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x85\x89\x83`@Qa\x0C\xB8\x93\x92\x91\x90aT\x95V[_\x80a-Ya0AV[`\x08\x01T\x92\x91PPV[``_a-na0AV[`\x10\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a-\xB7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xA3W[PPPPP\x91PP\x90V[``_a-\xCDa0AV[`\x0F\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a-\xB7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xA3WPPPPP\x91PP\x90V[``\x80_a.+a0AV[_\x85\x81R`\x12\x82\x01` R`@\x81 T\x91\x92P\x81\x90\x03a.aW`@Qc|\x8Bw!`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x81\x81R`\x03\x83\x01` \x90\x81R`@\x80\x83 T`\x02\x86\x01\x83R\x81\x84 \x81\x85R\x83R\x81\x84 \x80T\x83Q\x81\x86\x02\x81\x01\x86\x01\x90\x94R\x80\x84R\x91\x94\x93\x90\x91\x90\x83\x01\x82\x82\x80\x15a.\xD3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a.\xB5W[PPPPP\x90P_a.\xFC\x85`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\xFD\x90aN\x11V[\x90P_a/\t\x82\x84a5=V[\x90P\x80\x86`\x13\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a0,W_\x84\x81R` \x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x84\x02\x90\x91\x01\x80T\x82\x90`\xFF\x16`\x01\x81\x11\x15a/xWa/xaF\xC6V[`\x01\x81\x11\x15a/\x89Wa/\x89aF\xC6V[\x81R` \x01`\x01\x82\x01\x80Ta/\x9D\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xC9\x90aN\x11V[\x80\x15a0\x14W\x80`\x1F\x10a/\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\x14V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a/\xF7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a/?V[PPPP\x90P\x97P\x97PPPPPPP\x91P\x91V[\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90V[``_a0q\x83a=\xD6V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x8FWa0\x8FaH<V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a0\xB9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a0\xC3WP\x93\x92PPPV[a0\xFCa>\xADV[a\x11\x1B\x82\x82a>\xE3V[`@Q`\x01`\xF9\x1B` \x82\x01R`!\x81\x01\x83\x90R`A\x81\x01\x82\x90R``\x90`a\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_\x80a1Ja0AV[_\x85\x81R`\x0E\x82\x01` R`@\x90 \x80T\x91\x92P\x90a1h\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\x94\x90aN\x11V[\x80\x15a1\xDFW\x80`\x1F\x10a1\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1\xDFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1\xC2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x92Pa1\xEF\x83a:\xA6V[`@QcF\xC5\xBB\xBD`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R3`$\x82\x01R\x90\x92PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90cF\xC5\xBB\xBD\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2j\x91\x90aT\xBCV[a2\x89W`@Qc\xAE\xE8c#`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[P\x91P\x91V[_\x80\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xA9Wa2\xA9aH<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xD2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a3\xC3W`@Q\x80``\x01`@R\x80`%\x81R` \x01aX\\`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a3\x11Wa3\x11aP\xBCV[\x90P` \x02\x81\x01\x90a3#\x91\x90aP\xD0V[a31\x90` \x81\x01\x90aM\x06V[\x87\x87\x84\x81\x81\x10a3CWa3CaP\xBCV[\x90P` \x02\x81\x01\x90a3U\x91\x90aP\xD0V[a3c\x90` \x81\x01\x90aP\xEEV[`@Qa3q\x92\x91\x90aT\xDBV[`@Q\x90\x81\x90\x03\x81 a3\x88\x93\x92\x91` \x01aT\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a3\xB0Wa3\xB0aP\xBCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a2\xD7V[Pa4Z`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01aW\xDA`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a3\xFA\x91\x90aU\x0BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x8AQ\x8B\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a?BV[\x97\x96PPPPPPPV[_\x80a4\xA6\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa?n\x92PPPV[\x90Pa4\xB3\x86\x823a?\x96V[\x95\x94PPPPPV[`@Qc\x10kA\xA7`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R_\x90\x81\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90cA\xAD\x06\x9C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x0EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a52\x91\x90aU@V[\x90\x92\x10\x15\x93\x92PPPV[\x80Q``\x90_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a5[Wa5[aH<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\x8EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a5yW\x90P[P\x90P_[\x82\x81\x10\x15a6rWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a5\xD1Wa5\xD1aP\xBCV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\x08\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\"W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6I\x91\x90\x81\x01\x90aU\x99V[``\x01Q\x82\x82\x81Q\x81\x10a6_Wa6_aP\xBCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a5\x93V[P\x94\x93PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a7\x01WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a6\xF5_\x80Q` aV{\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a7\x1FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7qW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\x95\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a7\xC8W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a8%WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra8\"\x91\x81\x01\x90aU@V[`\x01[a8MW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\x9FV[_\x80Q` aV{\x839\x81Q\x91R\x81\x14a8}W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[a8\x87\x83\x83aA\x15V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a7\x1FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a9\x1E`@Q\x80``\x01`@R\x80`<\x81R` \x01aV\x9B`<\x919\x80Q` \x91\x82\x01 \x84Q\x85\x83\x01 `@\x80Q\x93\x84\x01\x92\x90\x92R\x90\x82\x01\x86\x90R``\x82\x01R`\x80\x01a4?V[\x93\x92PPPV[_a9\xA7`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01aV\xD7`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01a9^\x92\x91\x90aT\xDBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x89Q\x8A\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01a4?V[\x96\x95PPPPPPV[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x80T``\x91_\x80Q` aV[\x839\x81Q\x91R\x91a9\xEF\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta:\x1B\x90aN\x11V[\x80\x15a-\xB7W\x80`\x1F\x10a:=Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\xB7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a:IWP\x93\x96\x95PPPPPPV[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x80T``\x91_\x80Q` aV[\x839\x81Q\x91R\x91a9\xEF\x90aN\x11V[_\x81Q_\x14\x80a:\xCDWP\x81_\x81Q\x81\x10a:\xC3Wa:\xC3aP\xBCV[\x01` \x01Q`\xF8\x1C\x15[\x15a;LWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;F\x91\x90aU@V[\x92\x91PPV[_\x82_\x81Q\x81\x10a;_Wa;_aP\xBCV[\x01` \x01Q`\xF8\x1C\x90P`\x01\x81\x14\x80\x15\x90a;~WP`\xFF\x81\x16`\x02\x14\x15[\x15a;\xA1W`@Qc\x08Ns\x0B`\xE2\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x07\x9FV[`\xFF\x81\x16`\x01\x14\x80\x15a;\xB6WP\x82Q`!\x14\x15[\x15a;\xD4W`@Qc\x04Y$[`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16`\x02\x14\x80\x15a;\xE9WP\x82Q`A\x14\x15[\x15a<\x07W`@Qc\x04Y$[`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`!\x01Q\x90V[_\x80\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a<*Wa<*aH<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<SW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a=DW`@Q\x80``\x01`@R\x80`%\x81R` \x01aX\\`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a<\x92Wa<\x92aP\xBCV[\x90P` \x02\x81\x01\x90a<\xA4\x91\x90aP\xD0V[a<\xB2\x90` \x81\x01\x90aM\x06V[\x87\x87\x84\x81\x81\x10a<\xC4Wa<\xC4aP\xBCV[\x90P` \x02\x81\x01\x90a<\xD6\x91\x90aP\xD0V[a<\xE4\x90` \x81\x01\x90aP\xEEV[`@Qa<\xF2\x92\x91\x90aT\xDBV[`@Q\x90\x81\x90\x03\x81 a=\t\x93\x92\x91` \x01aT\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a=1Wa=1aP\xBCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a<XV[Pa=\xCA`@Q\x80`\xE0\x01`@R\x80`\xAD\x81R` \x01aW-`\xAD\x919\x80Q\x90` \x01 \x89\x89\x89\x85`@Q` \x01a=|\x91\x90aU\x0BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x8BQ\x8C\x83\x01 \x91\x84\x01\x97\x90\x97R\x90\x82\x01\x94\x90\x94R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R`\xC0\x82\x01R`\xE0\x01a4?V[\x98\x97PPPPPPPPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a>\x14Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a>@Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a>^Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a>vWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a>\x8AWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a>\x9CW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a;FW`\x01\x01\x92\x91PPV[_\x80Q` aX\x81\x839\x81Q\x91RT`\x01`@\x1B\x90\x04`\xFF\x16a7\x1FW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a>\xEBa>\xADV[_\x80Q` aV[\x839\x81Q\x91R\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02a?$\x84\x82aN\xA8V[P`\x03\x81\x01a?3\x83\x82aN\xA8V[P_\x80\x82U`\x01\x90\x91\x01UPPV[_a;Fa?NaAjV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80_\x80a?|\x86\x86aAxV[\x92P\x92P\x92Pa?\x8C\x82\x82aA\xC1V[P\x90\x94\x93PPPPV[`@Qc%\x11\xF3\xF5`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\x94G\xCF\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xF3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x17\x91\x90aT\xBCV[a@?W`@Qc\x15>7{`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\x9FV[`@Qc\x06?\xE89`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c1\xFFA\xC8\x90`D\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x9DW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra@\xC4\x91\x90\x81\x01\x90aU\x99V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14aA\x0FW`@Qc\r\x86\xF5!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`D\x01a\x07\x9FV[PPPPV[aA\x1E\x82aByV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15aAbWa8\x87\x82\x82aB\xDCV[a\x11\x1BaCEV[_aAsaCdV[\x90P\x90V[_\x80_\x83Q`A\x03aA\xAFW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1AaA\xA1\x88\x82\x85\x85aC\xD7V[\x95P\x95P\x95PPPPaA\xBAV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15aA\xD4WaA\xD4aF\xC6V[\x03aA\xDDWPPV[`\x01\x82`\x03\x81\x11\x15aA\xF1WaA\xF1aF\xC6V[\x03aB\x0FW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15aB#WaB#aF\xC6V[\x03aBDW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\x03\x82`\x03\x81\x11\x15aBXWaBXaF\xC6V[\x03a\x11\x1BW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03aB\xAEW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07\x9FV[_\x80Q` aV{\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@QaB\xF8\x91\x90aVIV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aC5V[``\x91P[P\x91P\x91Pa4\xB3\x85\x83\x83aD\x9FV[4\x15a7\x1FW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaC\x8EaD\xFBV[aC\x96aEcV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15aD\x10WP_\x91P`\x03\x90P\x82aD\x95V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aDaW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aD\x8CWP_\x92P`\x01\x91P\x82\x90PaD\x95V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aD\xB4WaD\xAF\x82aE\xA5V[a9\x1EV[\x81Q\x15\x80\x15aD\xCBWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aD\xF4W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07\x9FV[P\x92\x91PPV[__\x80Q` aV[\x839\x81Q\x91R\x81aE\x13a9\xB1V[\x80Q\x90\x91P\x15aE+W\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15aE:W\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[__\x80Q` aV[\x839\x81Q\x91R\x81aE{a:hV[\x80Q\x90\x91P\x15aE\x93W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15aE:W\x93\x92PPPV[\x80Q\x15aE\xB5W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01\x81\x11\x15aE\xF7WaE\xF7aF\xC6V[\x81R` \x01``\x81RP\x90V[_[\x83\x81\x10\x15aF\x1EW\x81\x81\x01Q\x83\x82\x01R` \x01aF\x06V[PP_\x91\x01RV[_\x81Q\x80\x84RaF=\x81` \x86\x01` \x86\x01aF\x04V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a9\x1E` \x83\x01\x84aF&V[_` \x82\x84\x03\x12\x15aFsW_\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\xBAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aF\x95V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a7\xC8WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01aG\x03\x83aF\xDAV[\x91\x90R\x90V[`\x02\x81\x10a7\xC8W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15aG&W_\x80\xFD[\x825\x91P` \x83\x015aG8\x81aG\tV[\x80\x91PP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aGSW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aGiW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\x80W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aG\x9BW_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG\xB8W_\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12aG\xCBW_\x80\xFD[\x815\x81\x81\x11\x15aG\xD9W_\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15aG\xEDW_\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x88\x015\x91P\x80\x82\x11\x15aH\nW_\x80\xFD[PaH\x17\x88\x82\x89\x01aGCV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a7\xC8W_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aHrWaHraH<V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\xA0WaH\xA0aH<V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aH\xC0WaH\xC0aH<V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x80`@\x83\x85\x03\x12\x15aH\xDFW_\x80\xFD[\x825aH\xEA\x81aH(V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x04W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x14W_\x80\xFD[\x805aI'aI\"\x82aH\xA8V[aHxV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aI;W_\x80\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_\x80_`@\x84\x86\x03\x12\x15aIlW_\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x88W_\x80\xFD[aI\x94\x86\x82\x87\x01aGCV[\x94\x97\x90\x96P\x93\x94PPPPV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01_[\x84\x81\x10\x15aJ\tW`\x1F\x19\x86\x84\x03\x01\x89R\x81Q`@\x81QaI\xDC\x81aF\xDAV[\x85R\x90\x85\x01Q\x85\x85\x01\x82\x90R\x90aI\xF5\x81\x86\x01\x83aF&V[\x9A\x86\x01\x9A\x94PPP\x90\x83\x01\x90`\x01\x01aI\xBCV[P\x90\x97\x96PPPPPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01R_`@\x83\x01QaJ:\x81aF\xDAV[\x80``\x84\x01RP``\x83\x01Q`\x80\x80\x84\x01RaJY`\xA0\x84\x01\x82aI\xA1V[\x94\x93PPPPV[_\x80_\x80_``\x86\x88\x03\x12\x15aJuW_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\x92W_\x80\xFD[aJ\x9E\x89\x83\x8A\x01aGCV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aH\nW_\x80\xFD[_``\x82\x01\x85\x15\x15\x83R` ``` \x85\x01R\x81\x86Q\x80\x84R`\x80\x86\x01\x91P` \x88\x01\x93P_[\x81\x81\x10\x15aK\rW\x84Q\x80Q\x84R\x84\x01Q`\x01`\x01`@\x1B\x03\x16\x84\x84\x01R\x93\x83\x01\x93`@\x90\x92\x01\x91`\x01\x01aJ\xDDV[PP\x80\x93PPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x94\x93PPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aK[W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aK?V[P\x94\x95\x94PPPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_aK\x84`\xE0\x83\x01\x89aF&V[\x82\x81\x03`@\x84\x01RaK\x96\x81\x89aF&V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90PaK\xC7\x81\x85aK,V[\x9A\x99PPPPPPPPPPV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aJ\tW`\x1F\x19\x86\x84\x03\x01\x89RaL\x0E\x83\x83QaF&V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aK\xF2V[`@\x81R_aL2`@\x83\x01\x85aK\xD5V[\x82\x81\x03` \x84\x01Ra4\xB3\x81\x85aI\xA1V[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a7\xC8W_\x80\xFD[_\x80_\x80``\x85\x87\x03\x12\x15aLkW_\x80\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL\x88W_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aL\x9BW_\x80\xFD[\x815\x81\x81\x11\x15aL\xA9W_\x80\xFD[\x88` \x82`\x06\x1B\x85\x01\x01\x11\x15aL\xBDW_\x80\xFD[` \x83\x01\x95P\x80\x94PPPP`@\x85\x015aL\xD7\x81aLDV[\x93\x96\x92\x95P\x90\x93PPV[`@\x81R_aL\xF4`@\x83\x01\x85aK\xD5V[\x82\x81\x03` \x84\x01Ra4\xB3\x81\x85aF&V[_` \x82\x84\x03\x12\x15aM\x16W_\x80\xFD[\x815a9\x1E\x81aG\tV[` \x81R_a9\x1E` \x83\x01\x84aK,V[_\x85QaMD\x81\x84` \x8A\x01aF\x04V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaMc\x81`\x02\x84\x01` \x8A\x01aF\x04V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaM\x87\x81`\x03\x85\x01` \x8A\x01aF\x04V[`\x03\x92\x01\x91\x82\x01R\x83QaM\xA2\x81`\x04\x84\x01` \x88\x01aF\x04V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aM\xC0W_\x80\xFD[\x81Qa9\x1E\x81aH(V[_`\x01\x82\x01aM\xE8WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[_\x80`@\x83\x85\x03\x12\x15aN\0W_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01\x81\x81\x1C\x90\x82\x16\x80aN%W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aNCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a8\x87W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aNnWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aN\x8DW_\x81U`\x01\x01aNzV[PPPPPV[_\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xC1WaN\xC1aH<V[aN\xD5\x81aN\xCF\x84TaN\x11V[\x84aNIV[` \x80`\x1F\x83\x11`\x01\x81\x14aO\x03W_\x84\x15aN\xF1WP\x85\x83\x01Q[aN\xFB\x85\x82aN\x94V[\x86UPaOZV[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aO1W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aO\x12V[P\x85\x82\x10\x15aONW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x84\x81R\x83` \x82\x01RaOt\x83aF\xDAV[\x82`@\x82\x01R`\x80``\x82\x01R_a9\xA7`\x80\x83\x01\x84aF&V[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aJ\tW\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`>\x19\x01\x81\x12aO\xF3W_\x80\xFD[\x87\x01`@\x815aP\x02\x81aG\tV[aP\x0B\x81aF\xDAV[\x85R\x81\x86\x0156\x83\x90\x03`\x1E\x19\x01\x81\x12aP#W_\x80\xFD[\x90\x91\x01\x85\x81\x01\x91\x905`\x01`\x01`@\x1B\x03\x81\x11\x15aP?W_\x80\xFD[\x806\x03\x83\x13\x15aPMW_\x80\xFD[\x81\x87\x87\x01RaP_\x82\x87\x01\x82\x85aO\x8FV[\x9B\x87\x01\x9B\x95PPP\x91\x84\x01\x91P`\x01\x01aO\xCEV[\x86\x81R`\x80` \x82\x01R_aP\x8D`\x80\x83\x01\x87\x89aO\xB7V[\x82\x81\x03`@\x84\x01RaP\xA0\x81\x86\x88aO\x8FV[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16``\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aP\xE4W_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aQ\x03W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aQ\x1CW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aG\x80W_\x80\xFD[`\x01`\x01`@\x1B\x03\x83\x11\x15aQGWaQGaH<V[aQ[\x83aQU\x83TaN\x11V[\x83aNIV[_`\x1F\x84\x11`\x01\x81\x14aQ\x87W_\x85\x15aQuWP\x83\x82\x015[aQ\x7F\x86\x82aN\x94V[\x84UPaN\x8DV[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15aQ\xB6W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aQ\x96V[P\x86\x82\x10\x15aQ\xD2W_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x815aQ\xEF\x81aG\tV[aQ\xF8\x81aF\xDAV[`\xFF\x19\x82T\x16`\xFF\x82\x16\x81\x17\x83UPP`\x01\x80\x82\x01` \x80\x85\x015`\x1E\x19\x866\x03\x01\x81\x12aR$W_\x80\xFD[\x85\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aR;W_\x80\xFD[\x806\x03\x83\x83\x01\x13\x15aRKW_\x80\xFD[aR_\x81aRY\x86TaN\x11V[\x86aNIV[_`\x1F\x82\x11`\x01\x81\x14aR\x8DW_\x83\x15aR{WP\x83\x82\x01\x85\x015[aR\x85\x84\x82aN\x94V[\x87UPa\x13\x93V[_\x86\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aR\xBBW\x86\x85\x01\x88\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01aR\x9CV[P\x84\x82\x10\x15aR\xD9W_\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x87\x85\x88\x01\x015\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90\x92UPPPPPV[\x84\x81R``` \x82\x01R_aS\x05``\x83\x01\x86aK\xD5V[\x82\x81\x03`@\x84\x01Ra4Z\x81\x85\x87aO\xB7V[\x84\x81R``` \x82\x01R_aS1``\x83\x01\x85\x87aO\x8FV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x95\x94PPPPPV[\x84\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R_a9\xA7`\x80\x83\x01\x84aF&V[\x83\x81R\x82` \x82\x01R```@\x82\x01R_a4\xB3``\x83\x01\x84aF&V[\x86\x81R`\x80` \x82\x01R_aP\x8D`\x80\x83\x01\x87\x89aO\x8FV[\x84\x81R``` \x82\x01R_aS\xBD``\x83\x01\x86aK\xD5V[\x82\x81\x03`@\x84\x01Ra4Z\x81\x85\x87aO\x8FV[_` \x82\x84\x03\x12\x15aS\xE0W_\x80\xFD[\x815a9\x1E\x81aLDV[\x815\x81U`\x01\x81\x01` \x83\x015aT\x01\x81aLDV[\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x17\x90UPPV[\x84\x81R``` \x80\x83\x01\x82\x90R\x90\x82\x01\x84\x90R_\x90\x85\x90`\x80\x84\x01\x83[\x87\x81\x10\x15aTvW\x835\x82R\x82\x84\x015aTW\x81aLDV[`\x01`\x01`@\x1B\x03\x16\x82\x84\x01R`@\x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aT>V[P\x80\x93PPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x95\x94PPPPPV[\x83\x81RaT\xA1\x83aF\xDAV[\x82` \x82\x01R```@\x82\x01R_a4\xB3``\x83\x01\x84aF&V[_` \x82\x84\x03\x12\x15aT\xCCW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a9\x1EW_\x80\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x83\x81R``\x81\x01aT\xFA\x84aF\xDAV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aU4W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aU\x18V[P\x92\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aUPW_\x80\xFD[PQ\x91\x90PV[_\x82`\x1F\x83\x01\x12aUfW_\x80\xFD[\x81QaUtaI\"\x82aH\xA8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aU\x88W_\x80\xFD[aJY\x82` \x83\x01` \x87\x01aF\x04V[_` \x82\x84\x03\x12\x15aU\xA9W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aU\xBFW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15aU\xD2W_\x80\xFD[aU\xDAaHPV[\x82QaU\xE5\x81aH(V[\x81R` \x83\x01QaU\xF5\x81aH(V[` \x82\x01R`@\x83\x01Q\x82\x81\x11\x15aV\x0BW_\x80\xFD[aV\x17\x87\x82\x86\x01aUWV[`@\x83\x01RP``\x83\x01Q\x82\x81\x11\x15aV.W_\x80\xFD[aV:\x87\x82\x86\x01aUWV[``\x83\x01RP\x95\x94PPPPPV[_\x82QaP\xE4\x81\x84` \x87\x01aF\x04V\xFE\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)CompressedKeyMaterialsVerification(uint256 prepKeygenId,uint256 migrationRequestId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101db575f3560e01c80637609152d116100fd578063c1252ca911610092578063d52f10eb11610062578063d52f10eb14610587578063dabd732f1461059b578063e410117e146105bc578063e711c9e7146105d0575f80fd5b8063c1252ca9146104fd578063c2c1faee1461051c578063c55b87241461053b578063caa367db14610568575f80fd5b8063ad3cb1cc116100cd578063ad3cb1cc14610486578063bac22bb8146104b6578063baff211e146104ca578063bc5404a0146104de575f80fd5b80637609152d146103e557806384b0196e146104135780639081fe131461043a578063936608ae14610459575f80fd5b80633d5ec7e31161017357806352d1902d1161014357806352d1902d14610367578063589adb0e1461037b5780636294f4621461039a57806362978787146103c6575f80fd5b80633d5ec7e3146102e757806345af261b146103165780634610ffe8146103355780634f1ef28614610354575f80fd5b806319f4f632116101ae57806319f4f6321461027457806339f73810146102a05780633ac50072146102b45780633c02f834146102c8575f80fd5b80630b680733146101df5780630d8e6e2c1461020657806316c713d9146102275780631703c61a14610253575b5f80fd5b3480156101ea575f80fd5b506101f36105ef565b6040519081526020015b60405180910390f35b348015610211575f80fd5b5061021a610603565b6040516101fd9190614651565b348015610232575f80fd5b50610246610241366004614663565b61066e565b6040516101fd919061467a565b34801561025e575f80fd5b5061027261026d366004614663565b6106fc565b005b34801561027f575f80fd5b5061029361028e366004614663565b610877565b6040516101fd91906146f6565b3480156102ab575f80fd5b5061027261090f565b3480156102bf575f80fd5b506101f3610a77565b3480156102d3575f80fd5b506102726102e2366004614715565b610a8b565b3480156102f2575f80fd5b50610306610301366004614663565b610cca565b60405190151581526020016101fd565b348015610321575f80fd5b50610293610330366004614663565b610ceb565b348015610340575f80fd5b5061027261034f366004614787565b610d71565b6102726103623660046148ce565b611100565b348015610372575f80fd5b506101f361111f565b348015610386575f80fd5b5061027261039536600461495a565b61113a565b3480156103a5575f80fd5b506103b96103b4366004614663565b61139e565b6040516101fd9190614a16565b3480156103d1575f80fd5b506102726103e0366004614a61565b61158b565b3480156103f0575f80fd5b506104046103ff366004614663565b611840565b6040516101fd93929190614ab6565b34801561041e575f80fd5b506104276118fc565b6040516101fd9796959493929190614b66565b348015610445575f80fd5b50610272610454366004614663565b6119a5565b348015610464575f80fd5b50610478610473366004614663565b611cfa565b6040516101fd929190614c20565b348015610491575f80fd5b5061021a604051806040016040528060058152602001640352e302e360dc1b81525081565b3480156104c1575f80fd5b50610272611fc2565b3480156104d5575f80fd5b506101f3612070565b3480156104e9575f80fd5b506102726104f8366004614c58565b612084565b348015610508575f80fd5b50610272610517366004614787565b6123a3565b348015610527575f80fd5b50610272610536366004614663565b612781565b348015610546575f80fd5b5061055a610555366004614663565b612926565b6040516101fd929190614ce2565b348015610573575f80fd5b50610272610582366004614d06565b612ae3565b348015610592575f80fd5b506101f3612d4f565b3480156105a6575f80fd5b506105af612d63565b6040516101fd9190614d21565b3480156105c7575f80fd5b506105af612dc2565b3480156105db575f80fd5b506104786105ea366004614663565b612e1f565b5f806105f9613041565b6005015492915050565b60606040518060400160405280600d81526020016c25a6a9a3b2b732b930ba34b7b760991b8152506106345f613065565b61063e6003613065565b6106475f613065565b60405160200161065a9493929190614d33565b604051602081830303815290604052905090565b60605f610679613041565b5f848152600382016020908152604080832054600285018352818420818552835292819020805482518185028101850190935280835294955092939092918301828280156106ee57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116106d0575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561074c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107709190614db0565b6001600160a01b0316336001600160a01b0316146107a85760405163021bfda160e41b81523360048201526024015b60405180910390fd5b5f6107b1613041565b905080600901548211806107c95750600560f81b8211155b156107ea576040516365f4932b60e11b81526004810183905260240161079f565b5f82815260018201602052604090205460ff161561081e5760405163df0db5fb60e01b81526004810183905260240161079f565b5f8281526001828101602052604091829020805460ff19169091179055517f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e9061086b9084815260200190565b60405180910390a15050565b5f80610881613041565b5f84815260018201602052604090205490915060ff166108b7576040516384de133160e01b81526004810184905260240161079f565b5f8381526003820160205260409020546108e7576040516383f1833560e01b81526004810184905260240161079f565b5f9283526006810160209081526040808520548552600d90920190529091205460ff16919050565b5f80516020615881833981519152546001600160401b03166001600160401b031660011461095057604051636f4f731f60e01b815260040160405180910390fd5b5f80516020615881833981519152805460049190600160401b900460ff1680610986575080546001600160401b03808416911610155b156109a45760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b03831617600160401b178155604080518082018252600d81526c25a6a9a3b2b732b930ba34b7b760991b602080830191909152825180840190935260018352603160f81b90830152610a0a916130f4565b5f610a13613041565b600360f81b6004820155600160fa1b6005820155600560f81b60099091015550805460ff60401b191681556040516001600160401b03831681527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200161086b565b5f80610a81613041565b6009015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610adb573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610aff9190614db0565b6001600160a01b0316336001600160a01b031614610b325760405163021bfda160e41b815233600482015260240161079f565b5f610b3b613041565b6009810154909150600560f81b8114801590610b6757505f81815260018301602052604090205460ff16155b15610b885760405163061ac61d60e01b81526004810182905260240161079f565b600982018054905f610b9983614dcb565b909155505060098201545f818152600a840160209081526040808320889055600d86019091529020805485919060ff191660018381811115610bdd57610bdd6146c6565b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015610c33573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c579190614def565b915091505f610c668383613106565b5f858152600e880160205260409020909150610c828282614ea8565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d84898984604051610cb89493929190614f62565b60405180910390a15050505050505050565b5f80610cd4613041565b5f9384526001016020525050604090205460ff1690565b5f80610cf5613041565b5f84815260018201602052604090205490915060ff16610d2b5760405163da32d00f60e01b81526004810184905260240161079f565b5f838152600382016020526040902054610d5b5760405163d5fd3cd760e01b81526004810184905260240161079f565b5f928352600d0160205250604090205460ff1690565b5f610d7a613041565b90508060050154861180610d925750600160fa1b8611155b15610db357604051632b7eae4160e21b81526004810187905260240161079f565b5f86815260118201602052604090205415610de457604051635a240b5b60e11b81526004810187905260240161079f565b5f849003610e085760405163e6f9083b60e01b81526004810187905260240161079f565b5f80610e138861313e565b5f8a815260068601602090815260408083205480845260018901909252909120549294509092509060ff16610e5b57604051636fbcdd2b60e01b815260040160405180910390fd5b5f610e69828b8b8b8861328f565b90505f610e7884838a8a613465565b5f8c8152602088815260408083206001600160a01b038516845290915290205490915060ff1615610ece576040516398fb957d60e01b8152600481018c90526001600160a01b038216602482015260440161079f565b5f8b8152602087815260408083206001600160a01b03851684528252808320805460ff191660019081179091558e845260028a0183528184208685528352818420805491820181558085529290932090920180546001600160a01b03191633908117909155915190917f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c791610f6c918f918f918f918f918f91615074565b60405180910390a15f8c815260018801602052604090205460ff16158015610f9c57508054610f9c9086906134bc565b156110f2575f8c8152600188810160205260408220805460ff191690911790555b8a811015611023575f8d815260078901602052604090208c8c83818110610fe657610fe66150bc565b9050602002810190610ff891906150d0565b81546001810183555f928352602090922090916002020161101982826151e4565b5050600101610fbd565b505f8c8152600388016020908152604080832086905560088a018f9055600f8a018054600181018255908452828420018f905583548151818402810184019092528082526110b19289929186918301828280156110a757602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611089575b505050505061353d565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8d828e8e6040516110e894939291906152ed565b60405180910390a1505b505050505050505050505050565b61110861367b565b61111182613721565b61111b82826137cb565b5050565b5f61112861388c565b505f8051602061567b83398151915290565b5f611143613041565b9050806004015484118061115b5750600360f81b8411155b1561117c57604051630ab7f68760e01b81526004810185905260240161079f565b5f806111878661313e565b915091505f61119687846138d5565b90505f6111a583838989613465565b5f898152602087815260408083206001600160a01b038516845290915290205490915060ff16156111fb576040516333ca1fe360e01b8152600481018990526001600160a01b038216602482015260440161079f565b5f888152602086815260408083206001600160a01b03851684528252808320805460ff191660019081179091558b84526002890183528184208685528352818420805491820181558085529290932090920180546001600160a01b03191633908117909155915190917f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c91611295918c918c918c91615318565b60405180910390a15f89815260018701602052604090205460ff161580156112c5575080546112c59085906134bc565b15611393575f898152600187810160209081526040808420805460ff19169093179092556003890181528183208690556006890181528183205480845260118a019091529120548015611354577fb07c5a1b25df43bfe6d702bac06c3d6e151a71da406a6cba07ffa41700dcabe18b83838a604051611347949392919061534a565b60405180910390a1611390565b7f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8b83896040516113879392919061536e565b60405180910390a15b50505b505050505050505050565b6113a66145ce565b5f6113af613041565b5f84815260018201602052604090205490915060ff166113e5576040516384de133160e01b81526004810184905260240161079f565b5f838152600382016020526040902054611415576040516383f1833560e01b81526004810184905260240161079f565b5f8381526006820160209081526040808320548151608081018352818152808401889052818552600d860190935292819020549082019060ff166001811115611460576114606146c6565b81525f86815260078501602090815260408083208054825181850281018501909352808352948301949193909284015b8282101561157d575f8481526020902060408051808201909152600284029091018054829060ff1660018111156114c9576114c96146c6565b60018111156114da576114da6146c6565b81526020016001820180546114ee90614e11565b80601f016020809104026020016040519081016040528092919081815260200182805461151a90614e11565b80156115655780601f1061153c57610100808354040283529160200191611565565b820191905f5260205f20905b81548152906001019060200180831161154857829003601f168201915b50505050508152505081526020019060010190611490565b505050915250949350505050565b5f611594613041565b905080600901548611806115ac5750600560f81b8611155b156115cd576040516346c64a0560e11b81526004810187905260240161079f565b5f806115d88861313e565b915091505f6115fd8985600a015f8c81526020019081526020015f20548a8a87613925565b90505f61160c83838989613465565b5f8b8152602087815260408083206001600160a01b038516845290915290205490915060ff16156116625760405163fcf5a6e960e01b8152600481018b90526001600160a01b038216602482015260440161079f565b5f8a8152602086815260408083206001600160a01b03851684528252808320805460ff191660019081179091558d84526002890183528184208685528352818420805491820181558085529290932090920180546001600160a01b03191633908117909155915190917f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd91611700918e918e918e918e918e9161538c565b60405180910390a15f8b815260018701602052604090205460ff16158015611730575080546117309085906134bc565b15611390575f8b8152600187810160209081526040808420805460ff1916909317909255600b8901905290206117678a8c83615130565b505f8b81526003870160209081526040808320869055600c89018e9055601089018054600181018255908452828420018e905583548151818402810184019092528082526117f39288929186918301828280156110a757602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161108957505050505061353d565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d60405161182a94939291906153a5565b60405180910390a1505050505050505050505050565b5f60605f8061184d613041565b5f86815260168201602090815260408083205460148501835281842060158601845282852054815484518187028101870190955280855296975060ff9092169590946001600160401b039092169385929084015b828210156118e8575f8481526020908190206040805180820190915260028502909101805482526001908101546001600160401b03168284015290835290920191016118a1565b505050509150935093509350509193909250565b5f60608082808083815f8051602061565b833981519152805490915015801561192757506001810154155b61196b5760405162461bcd60e51b81526020600482015260156024820152741152540dcc4c8e88155b9a5b9a5d1a585b1a5e9959605a1b604482015260640161079f565b6119736139b1565b61197b613a68565b604080515f80825260208201909252600f60f81b9c939b5091995046985030975095509350915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156119f5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a199190614db0565b6001600160a01b0316336001600160a01b031614611a4c5760405163021bfda160e41b815233600482015260240161079f565b5f611a55613041565b5f83815260018201602052604090205490915060ff161580611a7a5750806005015482115b80611a895750600160fa1b8211155b15611aaa576040516384de133160e01b81526004810183905260240161079f565b5f828152600382016020526040902054611ada576040516383f1833560e01b81526004810183905260240161079f565b5f82815260128201602052604090205415611b0b57604051632231dc3d60e21b81526004810183905260240161079f565b6005810154600160fa1b8114801590611b3457505f81815260018301602052604090205460ff16155b15611b5557604051630770a7b560e31b81526004810182905260240161079f565b600482018054905f611b6683614dcb565b90915550506004820154600583018054905f611b8183614dcb565b909155505060058301545f8281526006850160208181526040808420859055848452808420869055601188018252808420899055888452918152818320548352600d87019052808220548483529120805460ff9092169182919060ff191660018381811115611bf257611bf26146c6565b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015611c48573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c6c9190614def565b915091505f611c7b8383613106565b5f878152600e8a0160205260409020909150611c978282614ea8565b505f858152600e890160205260409020611cb18282614ea8565b507f2869c51322e90d2ce2dfe55ec29d180f36bffa15b434fc0a8dd0e5d3ce3c174c868a8684604051611ce79493929190614f62565b60405180910390a1505050505050505050565b6060805f611d06613041565b5f85815260018201602052604090205490915060ff16611d3c576040516384de133160e01b81526004810185905260240161079f565b5f84815260038201602052604090205480611d6d576040516383f1833560e01b81526004810186905260240161079f565b5f8581526002830160209081526040808320848452825280832080548251818502810185019093528083529192909190830182828015611dd457602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311611db6575b505050505090505f611e7e84600e015f8981526020019081526020015f208054611dfd90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054611e2990614e11565b8015611e745780601f10611e4b57610100808354040283529160200191611e74565b820191905f5260205f20905b815481529060010190602001808311611e5757829003601f168201915b5050505050613aa6565b90505f611e8b828461353d565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015611fae575f8481526020902060408051808201909152600284029091018054829060ff166001811115611efa57611efa6146c6565b6001811115611f0b57611f0b6146c6565b8152602001600182018054611f1f90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054611f4b90614e11565b8015611f965780601f10611f6d57610100808354040283529160200191611f96565b820191905f5260205f20905b815481529060010190602001808311611f7957829003601f168201915b50505050508152505081526020019060010190611ec1565b505050509050965096505050505050915091565b5f80516020615881833981519152805460049190600160401b900460ff1680611ff8575080546001600160401b03808416911610155b156120165760405163f92ee8a960e01b815260040160405180910390fd5b805468ffffffffffffffffff19166001600160401b038316908117600160401b1760ff60401b191682556040519081527fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d29060200161086b565b5f8061207a613041565b600c015492915050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156120d4573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906120f89190614db0565b6001600160a01b0316336001600160a01b03161461212b5760405163021bfda160e41b815233600482015260240161079f565b5f612134613041565b5f8681526012820160205260408120549192500361216857604051637c8b772160e11b81526004810186905260240161079f565b5f85815260168201602052604090205460ff161561219c57604051632031495b60e11b81526004810186905260240161079f565b5f8390036121bd576040516304a6b4b160e51b815260040160405180910390fd5b816001600160401b03165f036121e65760405163246bc3fb60e11b815260040160405180910390fd5b5f5b8381101561231d57848482818110612202576122026150bc565b905060400201602001602081019061221a91906153d0565b6001600160401b03165f036122425760405163246bc3fb60e11b815260040160405180910390fd5b5f5b818110156122c45785858381811061225e5761225e6150bc565b9050604002015f0135868683818110612279576122796150bc565b9050604002015f0135036122bc57858583818110612299576122996150bc565b60408051634a056b6b60e11b81529102929092013560048301525060240161079f565b600101612244565b505f86815260148301602052604090208585838181106122e6576122e66150bc565b83546001810185555f948552602090942060409091029290920192600202909101905061231382826153eb565b50506001016121e8565b505f8581526015820160209081526040808320805467ffffffffffffffff19166001600160401b0387161790556016840190915290819020805460ff19166001179055517f78cec34fd2715b935e4af4ae695f75a2c57cbd9655f1e7e3cc86c97ce79fa29f90612394908790879087908790615421565b60405180910390a15050505050565b5f6123ac613041565b905080600501548611806123c45750600160fa1b8611155b156123e557604051632b7eae4160e21b81526004810187905260240161079f565b5f8681526011820160205260408120549081900361241957604051635a240b5b60e11b81526004810188905260240161079f565b5f85900361243d5760405163e6f9083b60e01b81526004810188905260240161079f565b5f806124488961313e565b5f8b815260068701602090815260408083205480845260018a01909252909120549294509092509060ff1661249057604051636fbcdd2b60e01b815260040160405180910390fd5b5f61249f828c878d8d89613c10565b90505f6124ae84838b8b613465565b5f8d8152602089815260408083206001600160a01b038516845290915290205490915060ff1615612504576040516398fb957d60e01b8152600481018d90526001600160a01b038216602482015260440161079f565b6001875f015f8e81526020019081526020015f205f836001600160a01b03166001600160a01b031681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f876002015f8e81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a8154816001600160a01b0302191690836001600160a01b031602179055507f03c88abb63cb1a33cba1e37393e7fc01f203bb037b92f5ef84d52e5c63ac7be78d8d8d8d8d336040516125f296959493929190615074565b60405180910390a15f8d815260018901602052604090205460ff16158015612622575080546126229086906134bc565b15612772575f8d8152600189810160209081526040808420805460ff191690931790925560038b01905281208490555b8b8110156126b8575f88815260138a01602052604090208d8d8381811061267b5761267b6150bc565b905060200281019061268d91906150d0565b81546001810183555f92835260209092209091600202016126ae82826151e4565b5050600101612652565b508c886012015f8981526020019081526020015f20819055505f61273186838054806020026020016040519081016040528092919081815260200182805480156110a757602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161108957505050505061353d565b90507f80ebc2a4e183000f6837fab1e36970e8bc4a1b19223054c32769db663a4ce34688828f8f60405161276894939291906152ed565b60405180910390a1505b50505050505050505050505050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156127d1573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906127f59190614db0565b6001600160a01b0316336001600160a01b0316146128285760405163021bfda160e41b815233600482015260240161079f565b5f612831613041565b905080600401548211806128495750600360f81b8211155b1561286a57604051637e796dbd60e11b81526004810183905260240161079f565b5f828152600682016020908152604080832054808452600185019092529091205460ff16156128af576040516392789b6760e01b81526004810184905260240161079f565b5f83815260018381016020526040909120805460ff1916909117905580156128ee575f81815260018381016020526040909120805460ff191690911790555b6040518381527f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe32649060200160405180910390a1505050565b6060805f612932613041565b5f85815260018201602052604090205490915060ff166129685760405163da32d00f60e01b81526004810185905260240161079f565b5f848152600382016020526040902054806129995760405163d5fd3cd760e01b81526004810186905260240161079f565b5f8581526002830160209081526040808320848452825280832080548251818502810185019093528083529192909190830182828015612a0057602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116129e2575b505050505090505f612a2984600e015f8981526020019081526020015f208054611dfd90614e11565b90505f612a36828461353d565b5f898152600b870160205260409020805491925082918190612a5790614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054612a8390614e11565b8015612ace5780601f10612aa557610100808354040283529160200191612ace565b820191905f5260205f20905b815481529060010190602001808311612ab157829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612b33573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b579190614db0565b6001600160a01b0316336001600160a01b031614612b8a5760405163021bfda160e41b815233600482015260240161079f565b5f612b93613041565b6005810154909150600160fa1b8114801590612bbf57505f81815260018301602052604090205460ff16155b15612be057604051630770a7b560e31b81526004810182905260240161079f565b600482018054905f612bf183614dcb565b90915550506004820154600583018054905f612c0c83614dcb565b909155505060058301545f8281526006850160209081526040808320849055838352808320859055848352600d87019091529020805486919060ff191660018381811115612c5c57612c5c6146c6565b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015612cb2573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cd69190614def565b915091505f612ce58383613106565b5f868152600e890160205260409020909150612d018282614ea8565b505f848152600e880160205260409020612d1b8282614ea8565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe91858983604051610cb893929190615495565b5f80612d59613041565b6008015492915050565b60605f612d6e613041565b60108101805460408051602080840282018101909252828152939450830182828015612db757602002820191905f5260205f20905b815481526020019060010190808311612da3575b505050505091505090565b60605f612dcd613041565b600f8101805460408051602080840282018101909252828152939450830182828015612db757602002820191905f5260205f2090815481526020019060010190808311612da357505050505091505090565b6060805f612e2b613041565b5f858152601282016020526040812054919250819003612e6157604051637c8b772160e11b81526004810186905260240161079f565b5f81815260038301602090815260408083205460028601835281842081855283528184208054835181860281018601909452808452919493909190830182828015612ed357602002820191905f5260205f20905b81546001600160a01b03168152600190910190602001808311612eb5575b505050505090505f612efc85600e015f8681526020019081526020015f208054611dfd90614e11565b90505f612f09828461353d565b905080866013015f8b81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b8282101561302c575f8481526020902060408051808201909152600284029091018054829060ff166001811115612f7857612f786146c6565b6001811115612f8957612f896146c6565b8152602001600182018054612f9d90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054612fc990614e11565b80156130145780601f10612feb57610100808354040283529160200191613014565b820191905f5260205f20905b815481529060010190602001808311612ff757829003601f168201915b50505050508152505081526020019060010190612f3f565b50505050905097509750505050505050915091565b7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db0090565b60605f61307183613dd6565b60010190505f816001600160401b0381111561308f5761308f61483c565b6040519080825280601f01601f1916602001820160405280156130b9576020820181803683370190505b5090508181016020015b5f19016f181899199a1a9b1b9c1cb0b131b232b360811b600a86061a8153600a85049450846130c357509392505050565b6130fc613ead565b61111b8282613ee3565b604051600160f91b60208201526021810183905260418101829052606090606101604051602081830303815290604052905092915050565b60605f8061314a613041565b5f858152600e82016020526040902080549192509061316890614e11565b80601f016020809104026020016040519081016040528092919081815260200182805461319490614e11565b80156131df5780601f106131b6576101008083540402835291602001916131df565b820191905f5260205f20905b8154815290600101906020018083116131c257829003601f168201915b505050505092506131ef83613aa6565b6040516346c5bbbd60e01b8152600481018290523360248201529092507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac906346c5bbbd90604401602060405180830381865afa158015613246573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061326a91906154bc565b6132895760405163aee8632360e01b815233600482015260240161079f565b50915091565b5f80836001600160401b038111156132a9576132a961483c565b6040519080825280602002602001820160405280156132d2578160200160208202803683370190505b5090505f5b848110156133c35760405180606001604052806025815260200161585c6025913980519060200120868683818110613311576133116150bc565b905060200281019061332391906150d0565b613331906020810190614d06565b878784818110613343576133436150bc565b905060200281019061335591906150d0565b6133639060208101906150ee565b6040516133719291906154db565b6040519081900381206133889392916020016154ea565b604051602081830303815290604052805190602001208282815181106133b0576133b06150bc565b60209081029190910101526001016132d7565b5061345a6040518060c00160405280608281526020016157da60829139805190602001208888846040516020016133fa919061550b565b60408051601f1981840301815282825280516020918201208a518b83012091840196909652908201939093526060810191909152608081019290925260a082015260c0015b60405160208183030381529060405280519060200120613f42565b979650505050505050565b5f806134a68585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250613f6e92505050565b90506134b3868233613f96565b95945050505050565b60405163106b41a760e21b8152600481018390525f9081907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac906341ad069c90602401602060405180830381865afa15801561350e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906135329190615540565b909210159392505050565b80516060905f816001600160401b0381111561355b5761355b61483c565b60405190808252806020026020018201604052801561358e57816020015b60608152602001906001900390816135795790505b5090505f5b82811015613672577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b03166331ff41c8878784815181106135d1576135d16150bc565b60200260200101516040518363ffffffff1660e01b81526004016136089291909182526001600160a01b0316602082015260400190565b5f60405180830381865afa158015613622573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526136499190810190615599565b6060015182828151811061365f5761365f6150bc565b6020908102919091010152600101613593565b50949350505050565b306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148061370157507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166136f55f8051602061567b833981519152546001600160a01b031690565b6001600160a01b031614155b1561371f5760405163703e46dd60e11b815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d6001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613771573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906137959190614db0565b6001600160a01b0316336001600160a01b0316146137c85760405163021bfda160e41b815233600482015260240161079f565b50565b816001600160a01b03166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa925050508015613825575060408051601f3d908101601f1916820190925261382291810190615540565b60015b61384d57604051634c9c8ce360e01b81526001600160a01b038316600482015260240161079f565b5f8051602061567b833981519152811461387d57604051632a87526960e21b81526004810182905260240161079f565b6138878383614115565b505050565b306001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461371f5760405163703e46dd60e11b815260040160405180910390fd5b5f61391e6040518060600160405280603c815260200161569b603c913980516020918201208451858301206040805193840192909252908201869052606082015260800161343f565b9392505050565b5f6139a76040518060800160405280605681526020016156d760569139805190602001208787878760405160200161395e9291906154db565b60408051601f19818403018152828252805160209182012089518a83012091840196909652908201939093526060810191909152608081019290925260a082015260c00161343f565b9695505050505050565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10280546060915f8051602061565b833981519152916139ef90614e11565b80601f0160208091040260200160405190810160405280929190818152602001828054613a1b90614e11565b8015612db75780601f10613a3d57610100808354040283529160200191612db7565b820191905f5260205f20905b815481529060010190602001808311613a495750939695505050505050565b7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d10380546060915f8051602061565b833981519152916139ef90614e11565b5f81515f1480613acd5750815f81518110613ac357613ac36150bc565b016020015160f81c155b15613b4c577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac6001600160a01b031663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015613b22573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613b469190615540565b92915050565b5f825f81518110613b5f57613b5f6150bc565b016020015160f81c905060018114801590613b7e575060ff8116600214155b15613ba15760405163084e730b60e21b815260ff8216600482015260240161079f565b60ff81166001148015613bb657508251602114155b15613bd457604051630459245b60e51b815260040160405180910390fd5b60ff81166002148015613be957508251604114155b15613c0757604051630459245b60e51b815260040160405180910390fd5b50506021015190565b5f80836001600160401b03811115613c2a57613c2a61483c565b604051908082528060200260200182016040528015613c53578160200160208202803683370190505b5090505f5b84811015613d445760405180606001604052806025815260200161585c6025913980519060200120868683818110613c9257613c926150bc565b9050602002810190613ca491906150d0565b613cb2906020810190614d06565b878784818110613cc457613cc46150bc565b9050602002810190613cd691906150d0565b613ce49060208101906150ee565b604051613cf29291906154db565b604051908190038120613d099392916020016154ea565b60405160208183030381529060405280519060200120828281518110613d3157613d316150bc565b6020908102919091010152600101613c58565b50613dca6040518060e0016040528060ad815260200161572d60ad91398051906020012089898985604051602001613d7c919061550b565b60408051601f1981840301815282825280516020918201208b518c83012091840197909752908201949094526060810192909252608082015260a081019290925260c082015260e00161343f565b98975050505050505050565b5f8072184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b8310613e145772184f03e93ff9f4daa797ed6e38ed64bf6a1f0160401b830492506040015b6d04ee2d6d415b85acef81000000008310613e40576d04ee2d6d415b85acef8100000000830492506020015b662386f26fc100008310613e5e57662386f26fc10000830492506010015b6305f5e1008310613e76576305f5e100830492506008015b6127108310613e8a57612710830492506004015b60648310613e9c576064830492506002015b600a8310613b465760010192915050565b5f8051602061588183398151915254600160401b900460ff1661371f57604051631afcd79f60e31b815260040160405180910390fd5b613eeb613ead565b5f8051602061565b8339815191527fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d102613f248482614ea8565b5060038101613f338382614ea8565b505f8082556001909101555050565b5f613b46613f4e61416a565b8360405161190160f01b8152600281019290925260228201526042902090565b5f805f80613f7c8686614178565b925092509250613f8c82826141c1565b5090949350505050565b604051632511f3f560e21b8152600481018490526001600160a01b03831660248201527344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac90639447cfd490604401602060405180830381865afa158015613ff3573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061401791906154bc565b61403f5760405163153e377b60e11b81526001600160a01b038316600482015260240161079f565b60405163063fe83960e31b8152600481018490526001600160a01b03821660248201525f907344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac906331ff41c8906044015f60405180830381865afa15801561409d573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526140c49190810190615599565b9050826001600160a01b031681602001516001600160a01b03161461410f57604051630d86f52160e01b81526001600160a01b0380851660048301528316602482015260440161079f565b50505050565b61411e82614279565b6040516001600160a01b038316907fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b905f90a28051156141625761388782826142dc565b61111b614345565b5f614173614364565b905090565b5f805f83516041036141af576020840151604085015160608601515f1a6141a1888285856143d7565b9550955095505050506141ba565b505081515f91506002905b9250925092565b5f8260038111156141d4576141d46146c6565b036141dd575050565b60018260038111156141f1576141f16146c6565b0361420f5760405163f645eedf60e01b815260040160405180910390fd5b6002826003811115614223576142236146c6565b036142445760405163fce698f760e01b81526004810182905260240161079f565b6003826003811115614258576142586146c6565b0361111b576040516335e2f38360e21b81526004810182905260240161079f565b806001600160a01b03163b5f036142ae57604051634c9c8ce360e01b81526001600160a01b038216600482015260240161079f565b5f8051602061567b83398151915280546001600160a01b0319166001600160a01b0392909216919091179055565b60605f80846001600160a01b0316846040516142f89190615649565b5f60405180830381855af49150503d805f8114614330576040519150601f19603f3d011682016040523d82523d5f602084013e614335565b606091505b50915091506134b385838361449f565b341561371f5760405163b398979f60e01b815260040160405180910390fd5b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f61438e6144fb565b614396614563565b60408051602081019490945283019190915260608201524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b5f80807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a084111561441057505f91506003905082614495565b604080515f808252602082018084528a905260ff891692820192909252606081018790526080810186905260019060a0016020604051602081039080840390855afa158015614461573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b03811661448c57505f925060019150829050614495565b92505f91508190505b9450945094915050565b6060826144b4576144af826145a5565b61391e565b81511580156144cb57506001600160a01b0384163b155b156144f457604051639996b31560e01b81526001600160a01b038516600482015260240161079f565b5092915050565b5f5f8051602061565b833981519152816145136139b1565b80519091501561452b57805160209091012092915050565b8154801561453a579392505050565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470935050505090565b5f5f8051602061565b8339815191528161457b613a68565b80519091501561459357805160209091012092915050565b6001820154801561453a579392505050565b8051156145b55780518082602001fd5b60405163d6bda27560e01b815260040160405180910390fd5b60405180608001604052805f81526020015f81526020015f60018111156145f7576145f76146c6565b8152602001606081525090565b5f5b8381101561461e578181015183820152602001614606565b50505f910152565b5f815180845261463d816020860160208601614604565b601f01601f19169290920160200192915050565b602081525f61391e6020830184614626565b5f60208284031215614673575f80fd5b5035919050565b602080825282518282018190525f9190848201906040850190845b818110156146ba5783516001600160a01b031683529284019291840191600101614695565b50909695505050505050565b634e487b7160e01b5f52602160045260245ffd5b600281106137c857634e487b7160e01b5f52602160045260245ffd5b60208101614703836146da565b91905290565b600281106137c8575f80fd5b5f8060408385031215614726575f80fd5b82359150602083013561473881614709565b809150509250929050565b5f8083601f840112614753575f80fd5b5081356001600160401b03811115614769575f80fd5b602083019150836020828501011115614780575f80fd5b9250929050565b5f805f805f6060868803121561479b575f80fd5b8535945060208601356001600160401b03808211156147b8575f80fd5b818801915088601f8301126147cb575f80fd5b8135818111156147d9575f80fd5b8960208260051b85010111156147ed575f80fd5b60208301965080955050604088013591508082111561480a575f80fd5b5061481788828901614743565b969995985093965092949392505050565b6001600160a01b03811681146137c8575f80fd5b634e487b7160e01b5f52604160045260245ffd5b604051608081016001600160401b03811182821017156148725761487261483c565b60405290565b604051601f8201601f191681016001600160401b03811182821017156148a0576148a061483c565b604052919050565b5f6001600160401b038211156148c0576148c061483c565b50601f01601f191660200190565b5f80604083850312156148df575f80fd5b82356148ea81614828565b915060208301356001600160401b03811115614904575f80fd5b8301601f81018513614914575f80fd5b8035614927614922826148a8565b614878565b81815286602083850101111561493b575f80fd5b816020840160208301375f602083830101528093505050509250929050565b5f805f6040848603121561496c575f80fd5b8335925060208401356001600160401b03811115614988575f80fd5b61499486828701614743565b9497909650939450505050565b5f82825180855260208086019550808260051b8401018186015f5b84811015614a0957601f198684030189528151604081516149dc816146da565b855290850151858501829052906149f581860183614626565b9a86019a94505050908301906001016149bc565b5090979650505050505050565b6020815281516020820152602082015160408201525f6040830151614a3a816146da565b806060840152506060830151608080840152614a5960a08401826149a1565b949350505050565b5f805f805f60608688031215614a75575f80fd5b8535945060208601356001600160401b0380821115614a92575f80fd5b614a9e89838a01614743565b9096509450604088013591508082111561480a575f80fd5b5f6060820185151583526020606060208501528186518084526080860191506020880193505f5b81811015614b0d578451805184528401516001600160401b03168484015293830193604090920191600101614add565b50508093505050506001600160401b0383166040830152949350505050565b5f815180845260208085019450602084015f5b83811015614b5b57815187529582019590820190600101614b3f565b509495945050505050565b60ff60f81b8816815260e060208201525f614b8460e0830189614626565b8281036040840152614b968189614626565b606084018890526001600160a01b038716608085015260a0840186905283810360c08501529050614bc78185614b2c565b9a9950505050505050505050565b5f8282518085526020808601955060208260051b840101602086015f5b84811015614a0957601f19868403018952614c0e838351614626565b98840198925090830190600101614bf2565b604081525f614c326040830185614bd5565b82810360208401526134b381856149a1565b6001600160401b03811681146137c8575f80fd5b5f805f8060608587031215614c6b575f80fd5b8435935060208501356001600160401b0380821115614c88575f80fd5b818701915087601f830112614c9b575f80fd5b813581811115614ca9575f80fd5b8860208260061b8501011115614cbd575f80fd5b6020830195508094505050506040850135614cd781614c44565b939692955090935050565b604081525f614cf46040830185614bd5565b82810360208401526134b38185614626565b5f60208284031215614d16575f80fd5b813561391e81614709565b602081525f61391e6020830184614b2c565b5f8551614d44818460208a01614604565b61103b60f11b9083019081528551614d63816002840160208a01614604565b808201915050601760f91b8060028301528551614d87816003850160208a01614604565b60039201918201528351614da2816004840160208801614604565b016004019695505050505050565b5f60208284031215614dc0575f80fd5b815161391e81614828565b5f60018201614de857634e487b7160e01b5f52601160045260245ffd5b5060010190565b5f8060408385031215614e00575f80fd5b505080516020909101519092909150565b600181811c90821680614e2557607f821691505b602082108103614e4357634e487b7160e01b5f52602260045260245ffd5b50919050565b601f82111561388757805f5260205f20601f840160051c81016020851015614e6e5750805b601f840160051c820191505b81811015614e8d575f8155600101614e7a565b5050505050565b5f19600383901b1c191660019190911b1790565b81516001600160401b03811115614ec157614ec161483c565b614ed581614ecf8454614e11565b84614e49565b602080601f831160018114614f03575f8415614ef15750858301515b614efb8582614e94565b865550614f5a565b5f85815260208120601f198616915b82811015614f3157888601518255948401946001909101908401614f12565b5085821015614f4e57878501515f19600388901b60f8161c191681555b505060018460011b0185555b505050505050565b848152836020820152614f74836146da565b826040820152608060608201525f6139a76080830184614626565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8383855260208086019550808560051b830101845f5b87811015614a0957848303601f19018952813536889003603e19018112614ff3575f80fd5b87016040813561500281614709565b61500b816146da565b85528186013536839003601e19018112615023575f80fd5b9091018581019190356001600160401b0381111561503f575f80fd5b80360383131561504d575f80fd5b818787015261505f8287018285614f8f565b9b87019b955050509184019150600101614fce565b868152608060208201525f61508d608083018789614fb7565b82810360408401526150a0818688614f8f565b91505060018060a01b0383166060830152979650505050505050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e198336030181126150e4575f80fd5b9190910192915050565b5f808335601e19843603018112615103575f80fd5b8301803591506001600160401b0382111561511c575f80fd5b602001915036819003821315614780575f80fd5b6001600160401b038311156151475761514761483c565b61515b836151558354614e11565b83614e49565b5f601f841160018114615187575f85156151755750838201355b61517f8682614e94565b845550614e8d565b5f83815260208120601f198716915b828110156151b65786850135825560209485019460019092019101615196565b50868210156151d2575f1960f88860031b161c19848701351681555b505060018560011b0183555050505050565b81356151ef81614709565b6151f8816146da565b60ff1982541660ff82168117835550506001808201602080850135601e19863603018112615224575f80fd5b850180356001600160401b0381111561523b575f80fd5b803603838301131561524b575f80fd5b61525f816152598654614e11565b86614e49565b5f601f82116001811461528d575f831561527b57508382018501355b6152858482614e94565b875550611393565b5f86815260208120601f198516915b828110156152bb5786850188013582559387019390890190870161529c565b50848210156152d9575f1960f88660031b161c198785880101351681555b50505050600190811b019092555050505050565b848152606060208201525f6153056060830186614bd5565b828103604084015261345a818587614fb7565b848152606060208201525f615331606083018587614f8f565b905060018060a01b038316604083015295945050505050565b848152836020820152826040820152608060608201525f6139a76080830184614626565b838152826020820152606060408201525f6134b36060830184614626565b868152608060208201525f61508d608083018789614f8f565b848152606060208201525f6153bd6060830186614bd5565b828103604084015261345a818587614f8f565b5f602082840312156153e0575f80fd5b813561391e81614c44565b8135815560018101602083013561540181614c44565b815467ffffffffffffffff19166001600160401b03919091161790555050565b848152606060208083018290529082018490525f90859060808401835b8781101561547657833582528284013561545781614c44565b6001600160401b0316828401526040938401939091019060010161543e565b508093505050506001600160401b038316604083015295945050505050565b8381526154a1836146da565b826020820152606060408201525f6134b36060830184614626565b5f602082840312156154cc575f80fd5b8151801515811461391e575f80fd5b818382375f9101908152919050565b838152606081016154fa846146da565b602082019390935260400152919050565b81515f9082906020808601845b8381101561553457815185529382019390820190600101615518565b50929695505050505050565b5f60208284031215615550575f80fd5b5051919050565b5f82601f830112615566575f80fd5b8151615574614922826148a8565b818152846020838601011115615588575f80fd5b614a59826020830160208701614604565b5f602082840312156155a9575f80fd5b81516001600160401b03808211156155bf575f80fd5b90830190608082860312156155d2575f80fd5b6155da614850565b82516155e581614828565b815260208301516155f581614828565b602082015260408301518281111561560b575f80fd5b61561787828601615557565b60408301525060608301518281111561562e575f80fd5b61563a87828601615557565b60608301525095945050505050565b5f82516150e481846020870161460456fea16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c62797465732065787472614461746129436f6d707265737365644b65794d6174657269616c73566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206d6967726174696f6e5265717565737449642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429f0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xDBW_5`\xE0\x1C\x80cv\t\x15-\x11a\0\xFDW\x80c\xC1%,\xA9\x11a\0\x92W\x80c\xD5/\x10\xEB\x11a\0bW\x80c\xD5/\x10\xEB\x14a\x05\x87W\x80c\xDA\xBDs/\x14a\x05\x9BW\x80c\xE4\x10\x11~\x14a\x05\xBCW\x80c\xE7\x11\xC9\xE7\x14a\x05\xD0W_\x80\xFD[\x80c\xC1%,\xA9\x14a\x04\xFDW\x80c\xC2\xC1\xFA\xEE\x14a\x05\x1CW\x80c\xC5[\x87$\x14a\x05;W\x80c\xCA\xA3g\xDB\x14a\x05hW_\x80\xFD[\x80c\xAD<\xB1\xCC\x11a\0\xCDW\x80c\xAD<\xB1\xCC\x14a\x04\x86W\x80c\xBA\xC2+\xB8\x14a\x04\xB6W\x80c\xBA\xFF!\x1E\x14a\x04\xCAW\x80c\xBCT\x04\xA0\x14a\x04\xDEW_\x80\xFD[\x80cv\t\x15-\x14a\x03\xE5W\x80c\x84\xB0\x19n\x14a\x04\x13W\x80c\x90\x81\xFE\x13\x14a\x04:W\x80c\x93f\x08\xAE\x14a\x04YW_\x80\xFD[\x80c=^\xC7\xE3\x11a\x01sW\x80cR\xD1\x90-\x11a\x01CW\x80cR\xD1\x90-\x14a\x03gW\x80cX\x9A\xDB\x0E\x14a\x03{W\x80cb\x94\xF4b\x14a\x03\x9AW\x80cb\x97\x87\x87\x14a\x03\xC6W_\x80\xFD[\x80c=^\xC7\xE3\x14a\x02\xE7W\x80cE\xAF&\x1B\x14a\x03\x16W\x80cF\x10\xFF\xE8\x14a\x035W\x80cO\x1E\xF2\x86\x14a\x03TW_\x80\xFD[\x80c\x19\xF4\xF62\x11a\x01\xAEW\x80c\x19\xF4\xF62\x14a\x02tW\x80c9\xF78\x10\x14a\x02\xA0W\x80c:\xC5\0r\x14a\x02\xB4W\x80c<\x02\xF84\x14a\x02\xC8W_\x80\xFD[\x80c\x0Bh\x073\x14a\x01\xDFW\x80c\r\x8En,\x14a\x02\x06W\x80c\x16\xC7\x13\xD9\x14a\x02'W\x80c\x17\x03\xC6\x1A\x14a\x02SW[_\x80\xFD[4\x80\x15a\x01\xEAW_\x80\xFD[Pa\x01\xF3a\x05\xEFV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x11W_\x80\xFD[Pa\x02\x1Aa\x06\x03V[`@Qa\x01\xFD\x91\x90aFQV[4\x80\x15a\x022W_\x80\xFD[Pa\x02Fa\x02A6`\x04aFcV[a\x06nV[`@Qa\x01\xFD\x91\x90aFzV[4\x80\x15a\x02^W_\x80\xFD[Pa\x02ra\x02m6`\x04aFcV[a\x06\xFCV[\0[4\x80\x15a\x02\x7FW_\x80\xFD[Pa\x02\x93a\x02\x8E6`\x04aFcV[a\x08wV[`@Qa\x01\xFD\x91\x90aF\xF6V[4\x80\x15a\x02\xABW_\x80\xFD[Pa\x02ra\t\x0FV[4\x80\x15a\x02\xBFW_\x80\xFD[Pa\x01\xF3a\nwV[4\x80\x15a\x02\xD3W_\x80\xFD[Pa\x02ra\x02\xE26`\x04aG\x15V[a\n\x8BV[4\x80\x15a\x02\xF2W_\x80\xFD[Pa\x03\x06a\x03\x016`\x04aFcV[a\x0C\xCAV[`@Q\x90\x15\x15\x81R` \x01a\x01\xFDV[4\x80\x15a\x03!W_\x80\xFD[Pa\x02\x93a\x0306`\x04aFcV[a\x0C\xEBV[4\x80\x15a\x03@W_\x80\xFD[Pa\x02ra\x03O6`\x04aG\x87V[a\rqV[a\x02ra\x03b6`\x04aH\xCEV[a\x11\0V[4\x80\x15a\x03rW_\x80\xFD[Pa\x01\xF3a\x11\x1FV[4\x80\x15a\x03\x86W_\x80\xFD[Pa\x02ra\x03\x956`\x04aIZV[a\x11:V[4\x80\x15a\x03\xA5W_\x80\xFD[Pa\x03\xB9a\x03\xB46`\x04aFcV[a\x13\x9EV[`@Qa\x01\xFD\x91\x90aJ\x16V[4\x80\x15a\x03\xD1W_\x80\xFD[Pa\x02ra\x03\xE06`\x04aJaV[a\x15\x8BV[4\x80\x15a\x03\xF0W_\x80\xFD[Pa\x04\x04a\x03\xFF6`\x04aFcV[a\x18@V[`@Qa\x01\xFD\x93\x92\x91\x90aJ\xB6V[4\x80\x15a\x04\x1EW_\x80\xFD[Pa\x04'a\x18\xFCV[`@Qa\x01\xFD\x97\x96\x95\x94\x93\x92\x91\x90aKfV[4\x80\x15a\x04EW_\x80\xFD[Pa\x02ra\x04T6`\x04aFcV[a\x19\xA5V[4\x80\x15a\x04dW_\x80\xFD[Pa\x04xa\x04s6`\x04aFcV[a\x1C\xFAV[`@Qa\x01\xFD\x92\x91\x90aL V[4\x80\x15a\x04\x91W_\x80\xFD[Pa\x02\x1A`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\xC1W_\x80\xFD[Pa\x02ra\x1F\xC2V[4\x80\x15a\x04\xD5W_\x80\xFD[Pa\x01\xF3a pV[4\x80\x15a\x04\xE9W_\x80\xFD[Pa\x02ra\x04\xF86`\x04aLXV[a \x84V[4\x80\x15a\x05\x08W_\x80\xFD[Pa\x02ra\x05\x176`\x04aG\x87V[a#\xA3V[4\x80\x15a\x05'W_\x80\xFD[Pa\x02ra\x0566`\x04aFcV[a'\x81V[4\x80\x15a\x05FW_\x80\xFD[Pa\x05Za\x05U6`\x04aFcV[a)&V[`@Qa\x01\xFD\x92\x91\x90aL\xE2V[4\x80\x15a\x05sW_\x80\xFD[Pa\x02ra\x05\x826`\x04aM\x06V[a*\xE3V[4\x80\x15a\x05\x92W_\x80\xFD[Pa\x01\xF3a-OV[4\x80\x15a\x05\xA6W_\x80\xFD[Pa\x05\xAFa-cV[`@Qa\x01\xFD\x91\x90aM!V[4\x80\x15a\x05\xC7W_\x80\xFD[Pa\x05\xAFa-\xC2V[4\x80\x15a\x05\xDBW_\x80\xFD[Pa\x04xa\x05\xEA6`\x04aFcV[a.\x1FV[_\x80a\x05\xF9a0AV[`\x05\x01T\x92\x91PPV[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01l%\xA6\xA9\xA3\xB2\xB72\xB90\xBA4\xB7\xB7`\x99\x1B\x81RPa\x064_a0eV[a\x06>`\x03a0eV[a\x06G_a0eV[`@Q` \x01a\x06Z\x94\x93\x92\x91\x90aM3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x06ya0AV[_\x84\x81R`\x03\x82\x01` \x90\x81R`@\x80\x83 T`\x02\x85\x01\x83R\x81\x84 \x81\x85R\x83R\x92\x81\x90 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x94\x95P\x92\x93\x90\x92\x91\x83\x01\x82\x82\x80\x15a\x06\xEEW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x06\xD0W[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07LW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07p\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\xA8W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[_a\x07\xB1a0AV[\x90P\x80`\t\x01T\x82\x11\x80a\x07\xC9WP`\x05`\xF8\x1B\x82\x11\x15[\x15a\x07\xEAW`@Qce\xF4\x93+`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x01\x82\x01` R`@\x90 T`\xFF\x16\x15a\x08\x1EW`@Qc\xDF\r\xB5\xFB`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x01\x82\x81\x01` R`@\x91\x82\x90 \x80T`\xFF\x19\x16\x90\x91\x17\x90UQ\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x90a\x08k\x90\x84\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x08\x81a0AV[_\x84\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x08\xB7W`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x03\x82\x01` R`@\x90 Ta\x08\xE7W`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x92\x83R`\x06\x81\x01` \x90\x81R`@\x80\x85 T\x85R`\r\x90\x92\x01\x90R\x90\x91 T`\xFF\x16\x91\x90PV[_\x80Q` aX\x81\x839\x81Q\x91RT`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01\x14a\tPW`@QcoOs\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80Q` aX\x81\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\t\x86WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a\t\xA4W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x17`\x01`@\x1B\x17\x81U`@\x80Q\x80\x82\x01\x82R`\r\x81Rl%\xA6\xA9\xA3\xB2\xB72\xB90\xBA4\xB7\xB7`\x99\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x01\x83R`1`\xF8\x1B\x90\x83\x01Ra\n\n\x91a0\xF4V[_a\n\x13a0AV[`\x03`\xF8\x1B`\x04\x82\x01U`\x01`\xFA\x1B`\x05\x82\x01U`\x05`\xF8\x1B`\t\x90\x91\x01UP\x80T`\xFF`@\x1B\x19\x16\x81U`@Q`\x01`\x01`@\x1B\x03\x83\x16\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x08kV[_\x80a\n\x81a0AV[`\t\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xDBW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xFF\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B2W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a\x0B;a0AV[`\t\x81\x01T\x90\x91P`\x05`\xF8\x1B\x81\x14\x80\x15\x90a\x0BgWP_\x81\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16\x15[\x15a\x0B\x88W`@Qc\x06\x1A\xC6\x1D`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\t\x82\x01\x80T\x90_a\x0B\x99\x83aM\xCBV[\x90\x91UPP`\t\x82\x01T_\x81\x81R`\n\x84\x01` \x90\x81R`@\x80\x83 \x88\x90U`\r\x86\x01\x90\x91R\x90 \x80T\x85\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x0B\xDDWa\x0B\xDDaF\xC6V[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CW\x91\x90aM\xEFV[\x91P\x91P_a\x0Cf\x83\x83a1\x06V[_\x85\x81R`\x0E\x88\x01` R`@\x90 \x90\x91Pa\x0C\x82\x82\x82aN\xA8V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x84\x89\x89\x84`@Qa\x0C\xB8\x94\x93\x92\x91\x90aObV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x0C\xD4a0AV[_\x93\x84R`\x01\x01` RPP`@\x90 T`\xFF\x16\x90V[_\x80a\x0C\xF5a0AV[_\x84\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\r+W`@Qc\xDA2\xD0\x0F`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x03\x82\x01` R`@\x90 Ta\r[W`@Qc\xD5\xFD<\xD7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x92\x83R`\r\x01` RP`@\x90 T`\xFF\x16\x90V[_a\rza0AV[\x90P\x80`\x05\x01T\x86\x11\x80a\r\x92WP`\x01`\xFA\x1B\x86\x11\x15[\x15a\r\xB3W`@Qc+~\xAEA`\xE2\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x86\x81R`\x11\x82\x01` R`@\x90 T\x15a\r\xE4W`@QcZ$\x0B[`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x84\x90\x03a\x0E\x08W`@Qc\xE6\xF9\x08;`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x80a\x0E\x13\x88a1>V[_\x8A\x81R`\x06\x86\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x01\x89\x01\x90\x92R\x90\x91 T\x92\x94P\x90\x92P\x90`\xFF\x16a\x0E[W`@Qco\xBC\xDD+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0Ei\x82\x8B\x8B\x8B\x88a2\x8FV[\x90P_a\x0Ex\x84\x83\x8A\x8Aa4eV[_\x8C\x81R` \x88\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a\x0E\xCEW`@Qc\x98\xFB\x95}`\xE0\x1B\x81R`\x04\x81\x01\x8C\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[_\x8B\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x8E\x84R`\x02\x8A\x01\x83R\x81\x84 \x86\x85R\x83R\x81\x84 \x80T\x91\x82\x01\x81U\x80\x85R\x92\x90\x93 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x91Q\x90\x91\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x91a\x0Fl\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91\x8F\x91aPtV[`@Q\x80\x91\x03\x90\xA1_\x8C\x81R`\x01\x88\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a\x0F\x9CWP\x80Ta\x0F\x9C\x90\x86\x90a4\xBCV[\x15a\x10\xF2W_\x8C\x81R`\x01\x88\x81\x01` R`@\x82 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[\x8A\x81\x10\x15a\x10#W_\x8D\x81R`\x07\x89\x01` R`@\x90 \x8C\x8C\x83\x81\x81\x10a\x0F\xE6Wa\x0F\xE6aP\xBCV[\x90P` \x02\x81\x01\x90a\x0F\xF8\x91\x90aP\xD0V[\x81T`\x01\x81\x01\x83U_\x92\x83R` \x90\x92 \x90\x91`\x02\x02\x01a\x10\x19\x82\x82aQ\xE4V[PP`\x01\x01a\x0F\xBDV[P_\x8C\x81R`\x03\x88\x01` \x90\x81R`@\x80\x83 \x86\x90U`\x08\x8A\x01\x8F\x90U`\x0F\x8A\x01\x80T`\x01\x81\x01\x82U\x90\x84R\x82\x84 \x01\x8F\x90U\x83T\x81Q\x81\x84\x02\x81\x01\x84\x01\x90\x92R\x80\x82Ra\x10\xB1\x92\x89\x92\x91\x86\x91\x83\x01\x82\x82\x80\x15a\x10\xA7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x10\x89W[PPPPPa5=V[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8D\x82\x8E\x8E`@Qa\x10\xE8\x94\x93\x92\x91\x90aR\xEDV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPPV[a\x11\x08a6{V[a\x11\x11\x82a7!V[a\x11\x1B\x82\x82a7\xCBV[PPV[_a\x11(a8\x8CV[P_\x80Q` aV{\x839\x81Q\x91R\x90V[_a\x11Ca0AV[\x90P\x80`\x04\x01T\x84\x11\x80a\x11[WP`\x03`\xF8\x1B\x84\x11\x15[\x15a\x11|W`@Qc\n\xB7\xF6\x87`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x07\x9FV[_\x80a\x11\x87\x86a1>V[\x91P\x91P_a\x11\x96\x87\x84a8\xD5V[\x90P_a\x11\xA5\x83\x83\x89\x89a4eV[_\x89\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a\x11\xFBW`@Qc3\xCA\x1F\xE3`\xE0\x1B\x81R`\x04\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[_\x88\x81R` \x86\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x8B\x84R`\x02\x89\x01\x83R\x81\x84 \x86\x85R\x83R\x81\x84 \x80T\x91\x82\x01\x81U\x80\x85R\x92\x90\x93 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x91Q\x90\x91\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x91a\x12\x95\x91\x8C\x91\x8C\x91\x8C\x91aS\x18V[`@Q\x80\x91\x03\x90\xA1_\x89\x81R`\x01\x87\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a\x12\xC5WP\x80Ta\x12\xC5\x90\x85\x90a4\xBCV[\x15a\x13\x93W_\x89\x81R`\x01\x87\x81\x01` \x90\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U`\x03\x89\x01\x81R\x81\x83 \x86\x90U`\x06\x89\x01\x81R\x81\x83 T\x80\x84R`\x11\x8A\x01\x90\x91R\x91 T\x80\x15a\x13TW\x7F\xB0|Z\x1B%\xDFC\xBF\xE6\xD7\x02\xBA\xC0l=n\x15\x1Aq\xDA@jl\xBA\x07\xFF\xA4\x17\0\xDC\xAB\xE1\x8B\x83\x83\x8A`@Qa\x13G\x94\x93\x92\x91\x90aSJV[`@Q\x80\x91\x03\x90\xA1a\x13\x90V[\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8B\x83\x89`@Qa\x13\x87\x93\x92\x91\x90aSnV[`@Q\x80\x91\x03\x90\xA1[PP[PPPPPPPPPV[a\x13\xA6aE\xCEV[_a\x13\xAFa0AV[_\x84\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x13\xE5W`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x03\x82\x01` R`@\x90 Ta\x14\x15W`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x06\x82\x01` \x90\x81R`@\x80\x83 T\x81Q`\x80\x81\x01\x83R\x81\x81R\x80\x84\x01\x88\x90R\x81\x85R`\r\x86\x01\x90\x93R\x92\x81\x90 T\x90\x82\x01\x90`\xFF\x16`\x01\x81\x11\x15a\x14`Wa\x14`aF\xC6V[\x81R_\x86\x81R`\x07\x85\x01` \x90\x81R`@\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x94\x83\x01\x94\x91\x93\x90\x92\x84\x01[\x82\x82\x10\x15a\x15}W_\x84\x81R` \x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x84\x02\x90\x91\x01\x80T\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x14\xC9Wa\x14\xC9aF\xC6V[`\x01\x81\x11\x15a\x14\xDAWa\x14\xDAaF\xC6V[\x81R` \x01`\x01\x82\x01\x80Ta\x14\xEE\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x1A\x90aN\x11V[\x80\x15a\x15eW\x80`\x1F\x10a\x15<Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15eV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15HW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14\x90V[PPP\x91RP\x94\x93PPPPV[_a\x15\x94a0AV[\x90P\x80`\t\x01T\x86\x11\x80a\x15\xACWP`\x05`\xF8\x1B\x86\x11\x15[\x15a\x15\xCDW`@QcF\xC6J\x05`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x80a\x15\xD8\x88a1>V[\x91P\x91P_a\x15\xFD\x89\x85`\n\x01_\x8C\x81R` \x01\x90\x81R` \x01_ T\x8A\x8A\x87a9%V[\x90P_a\x16\x0C\x83\x83\x89\x89a4eV[_\x8B\x81R` \x87\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a\x16bW`@Qc\xFC\xF5\xA6\xE9`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[_\x8A\x81R` \x86\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x8D\x84R`\x02\x89\x01\x83R\x81\x84 \x86\x85R\x83R\x81\x84 \x80T\x91\x82\x01\x81U\x80\x85R\x92\x90\x93 \x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x90\x91U\x91Q\x90\x91\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x91a\x17\0\x91\x8E\x91\x8E\x91\x8E\x91\x8E\x91\x8E\x91aS\x8CV[`@Q\x80\x91\x03\x90\xA1_\x8B\x81R`\x01\x87\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a\x170WP\x80Ta\x170\x90\x85\x90a4\xBCV[\x15a\x13\x90W_\x8B\x81R`\x01\x87\x81\x01` \x90\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U`\x0B\x89\x01\x90R\x90 a\x17g\x8A\x8C\x83aQ0V[P_\x8B\x81R`\x03\x87\x01` \x90\x81R`@\x80\x83 \x86\x90U`\x0C\x89\x01\x8E\x90U`\x10\x89\x01\x80T`\x01\x81\x01\x82U\x90\x84R\x82\x84 \x01\x8E\x90U\x83T\x81Q\x81\x84\x02\x81\x01\x84\x01\x90\x92R\x80\x82Ra\x17\xF3\x92\x88\x92\x91\x86\x91\x83\x01\x82\x82\x80\x15a\x10\xA7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x10\x89WPPPPPa5=V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa\x18*\x94\x93\x92\x91\x90aS\xA5V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPPPPV[_``_\x80a\x18Ma0AV[_\x86\x81R`\x16\x82\x01` \x90\x81R`@\x80\x83 T`\x14\x85\x01\x83R\x81\x84 `\x15\x86\x01\x84R\x82\x85 T\x81T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x96\x97P`\xFF\x90\x92\x16\x95\x90\x94`\x01`\x01`@\x1B\x03\x90\x92\x16\x93\x85\x92\x90\x84\x01[\x82\x82\x10\x15a\x18\xE8W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x85\x02\x90\x91\x01\x80T\x82R`\x01\x90\x81\x01T`\x01`\x01`@\x1B\x03\x16\x82\x84\x01R\x90\x83R\x90\x92\x01\x91\x01a\x18\xA1V[PPPP\x91P\x93P\x93P\x93PP\x91\x93\x90\x92PV[_``\x80\x82\x80\x80\x83\x81_\x80Q` aV[\x839\x81Q\x91R\x80T\x90\x91P\x15\x80\x15a\x19'WP`\x01\x81\x01T\x15[a\x19kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11RT\r\xCCL\x8E\x88\x15[\x9A[\x9A]\x1AX[\x1A^\x99Y`Z\x1B`D\x82\x01R`d\x01a\x07\x9FV[a\x19sa9\xB1V[a\x19{a:hV[`@\x80Q_\x80\x82R` \x82\x01\x90\x92R`\x0F`\xF8\x1B\x9C\x93\x9BP\x91\x99PF\x98P0\x97P\x95P\x93P\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x19\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1ALW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a\x1AUa0AV[_\x83\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16\x15\x80a\x1AzWP\x80`\x05\x01T\x82\x11[\x80a\x1A\x89WP`\x01`\xFA\x1B\x82\x11\x15[\x15a\x1A\xAAW`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x03\x82\x01` R`@\x90 Ta\x1A\xDAW`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x12\x82\x01` R`@\x90 T\x15a\x1B\x0BW`@Qc\"1\xDC=`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[`\x05\x81\x01T`\x01`\xFA\x1B\x81\x14\x80\x15\x90a\x1B4WP_\x81\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16\x15[\x15a\x1BUW`@Qc\x07p\xA7\xB5`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\x04\x82\x01\x80T\x90_a\x1Bf\x83aM\xCBV[\x90\x91UPP`\x04\x82\x01T`\x05\x83\x01\x80T\x90_a\x1B\x81\x83aM\xCBV[\x90\x91UPP`\x05\x83\x01T_\x82\x81R`\x06\x85\x01` \x81\x81R`@\x80\x84 \x85\x90U\x84\x84R\x80\x84 \x86\x90U`\x11\x88\x01\x82R\x80\x84 \x89\x90U\x88\x84R\x91\x81R\x81\x83 T\x83R`\r\x87\x01\x90R\x80\x82 T\x84\x83R\x91 \x80T`\xFF\x90\x92\x16\x91\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a\x1B\xF2Wa\x1B\xF2aF\xC6V[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1CHW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Cl\x91\x90aM\xEFV[\x91P\x91P_a\x1C{\x83\x83a1\x06V[_\x87\x81R`\x0E\x8A\x01` R`@\x90 \x90\x91Pa\x1C\x97\x82\x82aN\xA8V[P_\x85\x81R`\x0E\x89\x01` R`@\x90 a\x1C\xB1\x82\x82aN\xA8V[P\x7F(i\xC5\x13\"\xE9\r,\xE2\xDF\xE5^\xC2\x9D\x18\x0F6\xBF\xFA\x15\xB44\xFC\n\x8D\xD0\xE5\xD3\xCE<\x17L\x86\x8A\x86\x84`@Qa\x1C\xE7\x94\x93\x92\x91\x90aObV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[``\x80_a\x1D\x06a0AV[_\x85\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a\x1D<W`@Qc\x84\xDE\x131`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x07\x9FV[_\x84\x81R`\x03\x82\x01` R`@\x90 T\x80a\x1DmW`@Qc\x83\xF1\x835`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x85\x81R`\x02\x83\x01` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x1D\xD4W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x1D\xB6W[PPPPP\x90P_a\x1E~\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\xFD\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E)\x90aN\x11V[\x80\x15a\x1EtW\x80`\x1F\x10a\x1EKWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1EtV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1EWW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:\xA6V[\x90P_a\x1E\x8B\x82\x84a5=V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1F\xAEW_\x84\x81R` \x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x84\x02\x90\x91\x01\x80T\x82\x90`\xFF\x16`\x01\x81\x11\x15a\x1E\xFAWa\x1E\xFAaF\xC6V[`\x01\x81\x11\x15a\x1F\x0BWa\x1F\x0BaF\xC6V[\x81R` \x01`\x01\x82\x01\x80Ta\x1F\x1F\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1FK\x90aN\x11V[\x80\x15a\x1F\x96W\x80`\x1F\x10a\x1FmWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\x96V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1FyW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xC1V[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[_\x80Q` aX\x81\x839\x81Q\x91R\x80T`\x04\x91\x90`\x01`@\x1B\x90\x04`\xFF\x16\x80a\x1F\xF8WP\x80T`\x01`\x01`@\x1B\x03\x80\x84\x16\x91\x16\x10\x15[\x15a \x16W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x83\x16\x90\x81\x17`\x01`@\x1B\x17`\xFF`@\x1B\x19\x16\x82U`@Q\x90\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01a\x08kV[_\x80a za0AV[`\x0C\x01T\x92\x91PPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xD4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xF8\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a!+W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a!4a0AV[_\x86\x81R`\x12\x82\x01` R`@\x81 T\x91\x92P\x03a!hW`@Qc|\x8Bw!`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x85\x81R`\x16\x82\x01` R`@\x90 T`\xFF\x16\x15a!\x9CW`@Qc 1I[`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x83\x90\x03a!\xBDW`@Qc\x04\xA6\xB4\xB1`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\x01`\x01`@\x1B\x03\x16_\x03a!\xE6W`@Qc$k\xC3\xFB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83\x81\x10\x15a#\x1DW\x84\x84\x82\x81\x81\x10a\"\x02Wa\"\x02aP\xBCV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\"\x1A\x91\x90aS\xD0V[`\x01`\x01`@\x1B\x03\x16_\x03a\"BW`@Qc$k\xC3\xFB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\"\xC4W\x85\x85\x83\x81\x81\x10a\"^Wa\"^aP\xBCV[\x90P`@\x02\x01_\x015\x86\x86\x83\x81\x81\x10a\"yWa\"yaP\xBCV[\x90P`@\x02\x01_\x015\x03a\"\xBCW\x85\x85\x83\x81\x81\x10a\"\x99Wa\"\x99aP\xBCV[`@\x80QcJ\x05kk`\xE1\x1B\x81R\x91\x02\x92\x90\x92\x015`\x04\x83\x01RP`$\x01a\x07\x9FV[`\x01\x01a\"DV[P_\x86\x81R`\x14\x83\x01` R`@\x90 \x85\x85\x83\x81\x81\x10a\"\xE6Wa\"\xE6aP\xBCV[\x83T`\x01\x81\x01\x85U_\x94\x85R` \x90\x94 `@\x90\x91\x02\x92\x90\x92\x01\x92`\x02\x02\x90\x91\x01\x90Pa#\x13\x82\x82aS\xEBV[PP`\x01\x01a!\xE8V[P_\x85\x81R`\x15\x82\x01` \x90\x81R`@\x80\x83 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x87\x16\x17\x90U`\x16\x84\x01\x90\x91R\x90\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7Fx\xCE\xC3O\xD2q[\x93^J\xF4\xAEi_u\xA2\xC5|\xBD\x96U\xF1\xE7\xE3\xCC\x86\xC9|\xE7\x9F\xA2\x9F\x90a#\x94\x90\x87\x90\x87\x90\x87\x90\x87\x90aT!V[`@Q\x80\x91\x03\x90\xA1PPPPPV[_a#\xACa0AV[\x90P\x80`\x05\x01T\x86\x11\x80a#\xC4WP`\x01`\xFA\x1B\x86\x11\x15[\x15a#\xE5W`@Qc+~\xAEA`\xE2\x1B\x81R`\x04\x81\x01\x87\x90R`$\x01a\x07\x9FV[_\x86\x81R`\x11\x82\x01` R`@\x81 T\x90\x81\x90\x03a$\x19W`@QcZ$\x0B[`\xE1\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07\x9FV[_\x85\x90\x03a$=W`@Qc\xE6\xF9\x08;`\xE0\x1B\x81R`\x04\x81\x01\x88\x90R`$\x01a\x07\x9FV[_\x80a$H\x89a1>V[_\x8B\x81R`\x06\x87\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x01\x8A\x01\x90\x92R\x90\x91 T\x92\x94P\x90\x92P\x90`\xFF\x16a$\x90W`@Qco\xBC\xDD+`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a$\x9F\x82\x8C\x87\x8D\x8D\x89a<\x10V[\x90P_a$\xAE\x84\x83\x8B\x8Ba4eV[_\x8D\x81R` \x89\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 T\x90\x91P`\xFF\x16\x15a%\x04W`@Qc\x98\xFB\x95}`\xE0\x1B\x81R`\x04\x81\x01\x8D\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R`D\x01a\x07\x9FV[`\x01\x87_\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x83`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x87`\x02\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x03\xC8\x8A\xBBc\xCB\x1A3\xCB\xA1\xE3s\x93\xE7\xFC\x01\xF2\x03\xBB\x03{\x92\xF5\xEF\x84\xD5.\\c\xAC{\xE7\x8D\x8D\x8D\x8D\x8D3`@Qa%\xF2\x96\x95\x94\x93\x92\x91\x90aPtV[`@Q\x80\x91\x03\x90\xA1_\x8D\x81R`\x01\x89\x01` R`@\x90 T`\xFF\x16\x15\x80\x15a&\"WP\x80Ta&\"\x90\x86\x90a4\xBCV[\x15a'rW_\x8D\x81R`\x01\x89\x81\x01` \x90\x81R`@\x80\x84 \x80T`\xFF\x19\x16\x90\x93\x17\x90\x92U`\x03\x8B\x01\x90R\x81 \x84\x90U[\x8B\x81\x10\x15a&\xB8W_\x88\x81R`\x13\x8A\x01` R`@\x90 \x8D\x8D\x83\x81\x81\x10a&{Wa&{aP\xBCV[\x90P` \x02\x81\x01\x90a&\x8D\x91\x90aP\xD0V[\x81T`\x01\x81\x01\x83U_\x92\x83R` \x90\x92 \x90\x91`\x02\x02\x01a&\xAE\x82\x82aQ\xE4V[PP`\x01\x01a&RV[P\x8C\x88`\x12\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_a'1\x86\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x10\xA7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x10\x89WPPPPPa5=V[\x90P\x7F\x80\xEB\xC2\xA4\xE1\x83\0\x0Fh7\xFA\xB1\xE3ip\xE8\xBCJ\x1B\x19\"0T\xC3'i\xDBf:L\xE3F\x88\x82\x8F\x8F`@Qa'h\x94\x93\x92\x91\x90aR\xEDV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPPPV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\xD1W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\xF5\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a((W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a(1a0AV[\x90P\x80`\x04\x01T\x82\x11\x80a(IWP`\x03`\xF8\x1B\x82\x11\x15[\x15a(jW`@Qc~ym\xBD`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x01a\x07\x9FV[_\x82\x81R`\x06\x82\x01` \x90\x81R`@\x80\x83 T\x80\x84R`\x01\x85\x01\x90\x92R\x90\x91 T`\xFF\x16\x15a(\xAFW`@Qc\x92x\x9Bg`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x07\x9FV[_\x83\x81R`\x01\x83\x81\x01` R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U\x80\x15a(\xEEW_\x81\x81R`\x01\x83\x81\x01` R`@\x90\x91 \x80T`\xFF\x19\x16\x90\x91\x17\x90U[`@Q\x83\x81R\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPV[``\x80_a)2a0AV[_\x85\x81R`\x01\x82\x01` R`@\x90 T\x90\x91P`\xFF\x16a)hW`@Qc\xDA2\xD0\x0F`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R`$\x01a\x07\x9FV[_\x84\x81R`\x03\x82\x01` R`@\x90 T\x80a)\x99W`@Qc\xD5\xFD<\xD7`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x85\x81R`\x02\x83\x01` \x90\x81R`@\x80\x83 \x84\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a*\0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a)\xE2W[PPPPP\x90P_a*)\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\xFD\x90aN\x11V[\x90P_a*6\x82\x84a5=V[_\x89\x81R`\x0B\x87\x01` R`@\x90 \x80T\x91\x92P\x82\x91\x81\x90a*W\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta*\x83\x90aN\x11V[\x80\x15a*\xCEW\x80`\x1F\x10a*\xA5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a*\xCEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a*\xB1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+W\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+\x8AW`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[_a+\x93a0AV[`\x05\x81\x01T\x90\x91P`\x01`\xFA\x1B\x81\x14\x80\x15\x90a+\xBFWP_\x81\x81R`\x01\x83\x01` R`@\x90 T`\xFF\x16\x15[\x15a+\xE0W`@Qc\x07p\xA7\xB5`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\x04\x82\x01\x80T\x90_a+\xF1\x83aM\xCBV[\x90\x91UPP`\x04\x82\x01T`\x05\x83\x01\x80T\x90_a,\x0C\x83aM\xCBV[\x90\x91UPP`\x05\x83\x01T_\x82\x81R`\x06\x85\x01` \x90\x81R`@\x80\x83 \x84\x90U\x83\x83R\x80\x83 \x85\x90U\x84\x83R`\r\x87\x01\x90\x91R\x90 \x80T\x86\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a,\\Wa,\\aF\xC6V[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xB2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xD6\x91\x90aM\xEFV[\x91P\x91P_a,\xE5\x83\x83a1\x06V[_\x86\x81R`\x0E\x89\x01` R`@\x90 \x90\x91Pa-\x01\x82\x82aN\xA8V[P_\x84\x81R`\x0E\x88\x01` R`@\x90 a-\x1B\x82\x82aN\xA8V[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x85\x89\x83`@Qa\x0C\xB8\x93\x92\x91\x90aT\x95V[_\x80a-Ya0AV[`\x08\x01T\x92\x91PPV[``_a-na0AV[`\x10\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a-\xB7W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xA3W[PPPPP\x91PP\x90V[``_a-\xCDa0AV[`\x0F\x81\x01\x80T`@\x80Q` \x80\x84\x02\x82\x01\x81\x01\x90\x92R\x82\x81R\x93\x94P\x83\x01\x82\x82\x80\x15a-\xB7W` \x02\x82\x01\x91\x90_R` _ \x90\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a-\xA3WPPPPP\x91PP\x90V[``\x80_a.+a0AV[_\x85\x81R`\x12\x82\x01` R`@\x81 T\x91\x92P\x81\x90\x03a.aW`@Qc|\x8Bw!`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R`$\x01a\x07\x9FV[_\x81\x81R`\x03\x83\x01` \x90\x81R`@\x80\x83 T`\x02\x86\x01\x83R\x81\x84 \x81\x85R\x83R\x81\x84 \x80T\x83Q\x81\x86\x02\x81\x01\x86\x01\x90\x94R\x80\x84R\x91\x94\x93\x90\x91\x90\x83\x01\x82\x82\x80\x15a.\xD3W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a.\xB5W[PPPPP\x90P_a.\xFC\x85`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x80Ta\x1D\xFD\x90aN\x11V[\x90P_a/\t\x82\x84a5=V[\x90P\x80\x86`\x13\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a0,W_\x84\x81R` \x90 `@\x80Q\x80\x82\x01\x90\x91R`\x02\x84\x02\x90\x91\x01\x80T\x82\x90`\xFF\x16`\x01\x81\x11\x15a/xWa/xaF\xC6V[`\x01\x81\x11\x15a/\x89Wa/\x89aF\xC6V[\x81R` \x01`\x01\x82\x01\x80Ta/\x9D\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta/\xC9\x90aN\x11V[\x80\x15a0\x14W\x80`\x1F\x10a/\xEBWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a0\x14V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a/\xF7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a/?V[PPPP\x90P\x97P\x97PPPPPPP\x91P\x91V[\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90V[``_a0q\x83a=\xD6V[`\x01\x01\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x8FWa0\x8FaH<V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a0\xB9W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a0\xC3WP\x93\x92PPPV[a0\xFCa>\xADV[a\x11\x1B\x82\x82a>\xE3V[`@Q`\x01`\xF9\x1B` \x82\x01R`!\x81\x01\x83\x90R`A\x81\x01\x82\x90R``\x90`a\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_\x80a1Ja0AV[_\x85\x81R`\x0E\x82\x01` R`@\x90 \x80T\x91\x92P\x90a1h\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta1\x94\x90aN\x11V[\x80\x15a1\xDFW\x80`\x1F\x10a1\xB6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a1\xDFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a1\xC2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x92Pa1\xEF\x83a:\xA6V[`@QcF\xC5\xBB\xBD`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R3`$\x82\x01R\x90\x92PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90cF\xC5\xBB\xBD\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a2j\x91\x90aT\xBCV[a2\x89W`@Qc\xAE\xE8c#`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[P\x91P\x91V[_\x80\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xA9Wa2\xA9aH<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xD2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a3\xC3W`@Q\x80``\x01`@R\x80`%\x81R` \x01aX\\`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a3\x11Wa3\x11aP\xBCV[\x90P` \x02\x81\x01\x90a3#\x91\x90aP\xD0V[a31\x90` \x81\x01\x90aM\x06V[\x87\x87\x84\x81\x81\x10a3CWa3CaP\xBCV[\x90P` \x02\x81\x01\x90a3U\x91\x90aP\xD0V[a3c\x90` \x81\x01\x90aP\xEEV[`@Qa3q\x92\x91\x90aT\xDBV[`@Q\x90\x81\x90\x03\x81 a3\x88\x93\x92\x91` \x01aT\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a3\xB0Wa3\xB0aP\xBCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a2\xD7V[Pa4Z`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01aW\xDA`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a3\xFA\x91\x90aU\x0BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x8AQ\x8B\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 a?BV[\x97\x96PPPPPPPV[_\x80a4\xA6\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa?n\x92PPPV[\x90Pa4\xB3\x86\x823a?\x96V[\x95\x94PPPPPV[`@Qc\x10kA\xA7`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R_\x90\x81\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90cA\xAD\x06\x9C\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a5\x0EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a52\x91\x90aU@V[\x90\x92\x10\x15\x93\x92PPPV[\x80Q``\x90_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a5[Wa5[aH<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\x8EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a5yW\x90P[P\x90P_[\x82\x81\x10\x15a6rWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a5\xD1Wa5\xD1aP\xBCV[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a6\x08\x92\x91\x90\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R`@\x01\x90V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\"W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra6I\x91\x90\x81\x01\x90aU\x99V[``\x01Q\x82\x82\x81Q\x81\x10a6_Wa6_aP\xBCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a5\x93V[P\x94\x93PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a7\x01WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a6\xF5_\x80Q` aV{\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a7\x1FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7qW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\x95\x91\x90aM\xB0V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a7\xC8W`@Qc\x02\x1B\xFD\xA1`\xE4\x1B\x81R3`\x04\x82\x01R`$\x01a\x07\x9FV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a8%WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra8\"\x91\x81\x01\x90aU@V[`\x01[a8MW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\x9FV[_\x80Q` aV{\x839\x81Q\x91R\x81\x14a8}W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[a8\x87\x83\x83aA\x15V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a7\x1FW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a9\x1E`@Q\x80``\x01`@R\x80`<\x81R` \x01aV\x9B`<\x919\x80Q` \x91\x82\x01 \x84Q\x85\x83\x01 `@\x80Q\x93\x84\x01\x92\x90\x92R\x90\x82\x01\x86\x90R``\x82\x01R`\x80\x01a4?V[\x93\x92PPPV[_a9\xA7`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01aV\xD7`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01a9^\x92\x91\x90aT\xDBV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x89Q\x8A\x83\x01 \x91\x84\x01\x96\x90\x96R\x90\x82\x01\x93\x90\x93R``\x81\x01\x91\x90\x91R`\x80\x81\x01\x92\x90\x92R`\xA0\x82\x01R`\xC0\x01a4?V[\x96\x95PPPPPPV[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02\x80T``\x91_\x80Q` aV[\x839\x81Q\x91R\x91a9\xEF\x90aN\x11V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta:\x1B\x90aN\x11V[\x80\x15a-\xB7W\x80`\x1F\x10a:=Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a-\xB7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a:IWP\x93\x96\x95PPPPPPV[\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x03\x80T``\x91_\x80Q` aV[\x839\x81Q\x91R\x91a9\xEF\x90aN\x11V[_\x81Q_\x14\x80a:\xCDWP\x81_\x81Q\x81\x10a:\xC3Wa:\xC3aP\xBCV[\x01` \x01Q`\xF8\x1C\x15[\x15a;LWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC`\x01`\x01`\xA0\x1B\x03\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a;\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a;F\x91\x90aU@V[\x92\x91PPV[_\x82_\x81Q\x81\x10a;_Wa;_aP\xBCV[\x01` \x01Q`\xF8\x1C\x90P`\x01\x81\x14\x80\x15\x90a;~WP`\xFF\x81\x16`\x02\x14\x15[\x15a;\xA1W`@Qc\x08Ns\x0B`\xE2\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01a\x07\x9FV[`\xFF\x81\x16`\x01\x14\x80\x15a;\xB6WP\x82Q`!\x14\x15[\x15a;\xD4W`@Qc\x04Y$[`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xFF\x81\x16`\x02\x14\x80\x15a;\xE9WP\x82Q`A\x14\x15[\x15a<\x07W`@Qc\x04Y$[`\xE5\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`!\x01Q\x90V[_\x80\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a<*Wa<*aH<V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<SW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a=DW`@Q\x80``\x01`@R\x80`%\x81R` \x01aX\\`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a<\x92Wa<\x92aP\xBCV[\x90P` \x02\x81\x01\x90a<\xA4\x91\x90aP\xD0V[a<\xB2\x90` \x81\x01\x90aM\x06V[\x87\x87\x84\x81\x81\x10a<\xC4Wa<\xC4aP\xBCV[\x90P` \x02\x81\x01\x90a<\xD6\x91\x90aP\xD0V[a<\xE4\x90` \x81\x01\x90aP\xEEV[`@Qa<\xF2\x92\x91\x90aT\xDBV[`@Q\x90\x81\x90\x03\x81 a=\t\x93\x92\x91` \x01aT\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a=1Wa=1aP\xBCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a<XV[Pa=\xCA`@Q\x80`\xE0\x01`@R\x80`\xAD\x81R` \x01aW-`\xAD\x919\x80Q\x90` \x01 \x89\x89\x89\x85`@Q` \x01a=|\x91\x90aU\x0BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x8BQ\x8C\x83\x01 \x91\x84\x01\x97\x90\x97R\x90\x82\x01\x94\x90\x94R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x81\x01\x92\x90\x92R`\xC0\x82\x01R`\xE0\x01a4?V[\x98\x97PPPPPPPPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a>\x14Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a>@Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a>^Wf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a>vWc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a>\x8AWa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a>\x9CW`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a;FW`\x01\x01\x92\x91PPV[_\x80Q` aX\x81\x839\x81Q\x91RT`\x01`@\x1B\x90\x04`\xFF\x16a7\x1FW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a>\xEBa>\xADV[_\x80Q` aV[\x839\x81Q\x91R\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x02a?$\x84\x82aN\xA8V[P`\x03\x81\x01a?3\x83\x82aN\xA8V[P_\x80\x82U`\x01\x90\x91\x01UPPV[_a;Fa?NaAjV[\x83`@Qa\x19\x01`\xF0\x1B\x81R`\x02\x81\x01\x92\x90\x92R`\"\x82\x01R`B\x90 \x90V[_\x80_\x80a?|\x86\x86aAxV[\x92P\x92P\x92Pa?\x8C\x82\x82aA\xC1V[P\x90\x94\x93PPPPV[`@Qc%\x11\xF3\xF5`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01RsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c\x94G\xCF\xD4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?\xF3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x17\x91\x90aT\xBCV[a@?W`@Qc\x15>7{`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x07\x9FV[`@Qc\x06?\xE89`\xE3\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16`$\x82\x01R_\x90sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xAC\x90c1\xFFA\xC8\x90`D\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@\x9DW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra@\xC4\x91\x90\x81\x01\x90aU\x99V[\x90P\x82`\x01`\x01`\xA0\x1B\x03\x16\x81` \x01Q`\x01`\x01`\xA0\x1B\x03\x16\x14aA\x0FW`@Qc\r\x86\xF5!`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x04\x83\x01R\x83\x16`$\x82\x01R`D\x01a\x07\x9FV[PPPPV[aA\x1E\x82aByV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15aAbWa8\x87\x82\x82aB\xDCV[a\x11\x1BaCEV[_aAsaCdV[\x90P\x90V[_\x80_\x83Q`A\x03aA\xAFW` \x84\x01Q`@\x85\x01Q``\x86\x01Q_\x1AaA\xA1\x88\x82\x85\x85aC\xD7V[\x95P\x95P\x95PPPPaA\xBAV[PP\x81Q_\x91P`\x02\x90[\x92P\x92P\x92V[_\x82`\x03\x81\x11\x15aA\xD4WaA\xD4aF\xC6V[\x03aA\xDDWPPV[`\x01\x82`\x03\x81\x11\x15aA\xF1WaA\xF1aF\xC6V[\x03aB\x0FW`@Qc\xF6E\xEE\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82`\x03\x81\x11\x15aB#WaB#aF\xC6V[\x03aBDW`@Qc\xFC\xE6\x98\xF7`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[`\x03\x82`\x03\x81\x11\x15aBXWaBXaF\xC6V[\x03a\x11\x1BW`@Qc5\xE2\xF3\x83`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x07\x9FV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03aB\xAEW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x07\x9FV[_\x80Q` aV{\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@QaB\xF8\x91\x90aVIV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aC0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aC5V[``\x91P[P\x91P\x91Pa4\xB3\x85\x83\x83aD\x9FV[4\x15a7\x1FW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaC\x8EaD\xFBV[aC\x96aEcV[`@\x80Q` \x81\x01\x94\x90\x94R\x83\x01\x91\x90\x91R``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11\x15aD\x10WP_\x91P`\x03\x90P\x82aD\x95V[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x8A\x90R`\xFF\x89\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x87\x90R`\x80\x81\x01\x86\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aDaW=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16aD\x8CWP_\x92P`\x01\x91P\x82\x90PaD\x95V[\x92P_\x91P\x81\x90P[\x94P\x94P\x94\x91PPV[``\x82aD\xB4WaD\xAF\x82aE\xA5V[a9\x1EV[\x81Q\x15\x80\x15aD\xCBWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15aD\xF4W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x07\x9FV[P\x92\x91PPV[__\x80Q` aV[\x839\x81Q\x91R\x81aE\x13a9\xB1V[\x80Q\x90\x91P\x15aE+W\x80Q` \x90\x91\x01 \x92\x91PPV[\x81T\x80\x15aE:W\x93\x92PPPV[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP\x90V[__\x80Q` aV[\x839\x81Q\x91R\x81aE{a:hV[\x80Q\x90\x91P\x15aE\x93W\x80Q` \x90\x91\x01 \x92\x91PPV[`\x01\x82\x01T\x80\x15aE:W\x93\x92PPPV[\x80Q\x15aE\xB5W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01\x81\x11\x15aE\xF7WaE\xF7aF\xC6V[\x81R` \x01``\x81RP\x90V[_[\x83\x81\x10\x15aF\x1EW\x81\x81\x01Q\x83\x82\x01R` \x01aF\x06V[PP_\x91\x01RV[_\x81Q\x80\x84RaF=\x81` \x86\x01` \x86\x01aF\x04V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a9\x1E` \x83\x01\x84aF&V[_` \x82\x84\x03\x12\x15aFsW_\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aF\xBAW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aF\x95V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x02\x81\x10a7\xC8WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01aG\x03\x83aF\xDAV[\x91\x90R\x90V[`\x02\x81\x10a7\xC8W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15aG&W_\x80\xFD[\x825\x91P` \x83\x015aG8\x81aG\tV[\x80\x91PP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aGSW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aGiW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aG\x80W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aG\x9BW_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG\xB8W_\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12aG\xCBW_\x80\xFD[\x815\x81\x81\x11\x15aG\xD9W_\x80\xFD[\x89` \x82`\x05\x1B\x85\x01\x01\x11\x15aG\xEDW_\x80\xFD[` \x83\x01\x96P\x80\x95PP`@\x88\x015\x91P\x80\x82\x11\x15aH\nW_\x80\xFD[PaH\x17\x88\x82\x89\x01aGCV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a7\xC8W_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aHrWaHraH<V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\xA0WaH\xA0aH<V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aH\xC0WaH\xC0aH<V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x80`@\x83\x85\x03\x12\x15aH\xDFW_\x80\xFD[\x825aH\xEA\x81aH(V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x04W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aI\x14W_\x80\xFD[\x805aI'aI\"\x82aH\xA8V[aHxV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15aI;W_\x80\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[_\x80_`@\x84\x86\x03\x12\x15aIlW_\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x88W_\x80\xFD[aI\x94\x86\x82\x87\x01aGCV[\x94\x97\x90\x96P\x93\x94PPPPV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P\x80\x82`\x05\x1B\x84\x01\x01\x81\x86\x01_[\x84\x81\x10\x15aJ\tW`\x1F\x19\x86\x84\x03\x01\x89R\x81Q`@\x81QaI\xDC\x81aF\xDAV[\x85R\x90\x85\x01Q\x85\x85\x01\x82\x90R\x90aI\xF5\x81\x86\x01\x83aF&V[\x9A\x86\x01\x9A\x94PPP\x90\x83\x01\x90`\x01\x01aI\xBCV[P\x90\x97\x96PPPPPPPV[` \x81R\x81Q` \x82\x01R` \x82\x01Q`@\x82\x01R_`@\x83\x01QaJ:\x81aF\xDAV[\x80``\x84\x01RP``\x83\x01Q`\x80\x80\x84\x01RaJY`\xA0\x84\x01\x82aI\xA1V[\x94\x93PPPPV[_\x80_\x80_``\x86\x88\x03\x12\x15aJuW_\x80\xFD[\x855\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aJ\x92W_\x80\xFD[aJ\x9E\x89\x83\x8A\x01aGCV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15aH\nW_\x80\xFD[_``\x82\x01\x85\x15\x15\x83R` ``` \x85\x01R\x81\x86Q\x80\x84R`\x80\x86\x01\x91P` \x88\x01\x93P_[\x81\x81\x10\x15aK\rW\x84Q\x80Q\x84R\x84\x01Q`\x01`\x01`@\x1B\x03\x16\x84\x84\x01R\x93\x83\x01\x93`@\x90\x92\x01\x91`\x01\x01aJ\xDDV[PP\x80\x93PPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x94\x93PPPPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15aK[W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aK?V[P\x94\x95\x94PPPPPV[`\xFF`\xF8\x1B\x88\x16\x81R`\xE0` \x82\x01R_aK\x84`\xE0\x83\x01\x89aF&V[\x82\x81\x03`@\x84\x01RaK\x96\x81\x89aF&V[``\x84\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x90PaK\xC7\x81\x85aK,V[\x9A\x99PPPPPPPPPPV[_\x82\x82Q\x80\x85R` \x80\x86\x01\x95P` \x82`\x05\x1B\x84\x01\x01` \x86\x01_[\x84\x81\x10\x15aJ\tW`\x1F\x19\x86\x84\x03\x01\x89RaL\x0E\x83\x83QaF&V[\x98\x84\x01\x98\x92P\x90\x83\x01\x90`\x01\x01aK\xF2V[`@\x81R_aL2`@\x83\x01\x85aK\xD5V[\x82\x81\x03` \x84\x01Ra4\xB3\x81\x85aI\xA1V[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a7\xC8W_\x80\xFD[_\x80_\x80``\x85\x87\x03\x12\x15aLkW_\x80\xFD[\x845\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aL\x88W_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12aL\x9BW_\x80\xFD[\x815\x81\x81\x11\x15aL\xA9W_\x80\xFD[\x88` \x82`\x06\x1B\x85\x01\x01\x11\x15aL\xBDW_\x80\xFD[` \x83\x01\x95P\x80\x94PPPP`@\x85\x015aL\xD7\x81aLDV[\x93\x96\x92\x95P\x90\x93PPV[`@\x81R_aL\xF4`@\x83\x01\x85aK\xD5V[\x82\x81\x03` \x84\x01Ra4\xB3\x81\x85aF&V[_` \x82\x84\x03\x12\x15aM\x16W_\x80\xFD[\x815a9\x1E\x81aG\tV[` \x81R_a9\x1E` \x83\x01\x84aK,V[_\x85QaMD\x81\x84` \x8A\x01aF\x04V[a\x10;`\xF1\x1B\x90\x83\x01\x90\x81R\x85QaMc\x81`\x02\x84\x01` \x8A\x01aF\x04V[\x80\x82\x01\x91PP`\x17`\xF9\x1B\x80`\x02\x83\x01R\x85QaM\x87\x81`\x03\x85\x01` \x8A\x01aF\x04V[`\x03\x92\x01\x91\x82\x01R\x83QaM\xA2\x81`\x04\x84\x01` \x88\x01aF\x04V[\x01`\x04\x01\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aM\xC0W_\x80\xFD[\x81Qa9\x1E\x81aH(V[_`\x01\x82\x01aM\xE8WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[P`\x01\x01\x90V[_\x80`@\x83\x85\x03\x12\x15aN\0W_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01\x81\x81\x1C\x90\x82\x16\x80aN%W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03aNCWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a8\x87W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aNnWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15aN\x8DW_\x81U`\x01\x01aNzV[PPPPPV[_\x19`\x03\x83\x90\x1B\x1C\x19\x16`\x01\x91\x90\x91\x1B\x17\x90V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xC1WaN\xC1aH<V[aN\xD5\x81aN\xCF\x84TaN\x11V[\x84aNIV[` \x80`\x1F\x83\x11`\x01\x81\x14aO\x03W_\x84\x15aN\xF1WP\x85\x83\x01Q[aN\xFB\x85\x82aN\x94V[\x86UPaOZV[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15aO1W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01aO\x12V[P\x85\x82\x10\x15aONW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x84\x81R\x83` \x82\x01RaOt\x83aF\xDAV[\x82`@\x82\x01R`\x80``\x82\x01R_a9\xA7`\x80\x83\x01\x84aF&V[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84_[\x87\x81\x10\x15aJ\tW\x84\x83\x03`\x1F\x19\x01\x89R\x8156\x88\x90\x03`>\x19\x01\x81\x12aO\xF3W_\x80\xFD[\x87\x01`@\x815aP\x02\x81aG\tV[aP\x0B\x81aF\xDAV[\x85R\x81\x86\x0156\x83\x90\x03`\x1E\x19\x01\x81\x12aP#W_\x80\xFD[\x90\x91\x01\x85\x81\x01\x91\x905`\x01`\x01`@\x1B\x03\x81\x11\x15aP?W_\x80\xFD[\x806\x03\x83\x13\x15aPMW_\x80\xFD[\x81\x87\x87\x01RaP_\x82\x87\x01\x82\x85aO\x8FV[\x9B\x87\x01\x9B\x95PPP\x91\x84\x01\x91P`\x01\x01aO\xCEV[\x86\x81R`\x80` \x82\x01R_aP\x8D`\x80\x83\x01\x87\x89aO\xB7V[\x82\x81\x03`@\x84\x01RaP\xA0\x81\x86\x88aO\x8FV[\x91PP`\x01\x80`\xA0\x1B\x03\x83\x16``\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aP\xE4W_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aQ\x03W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aQ\x1CW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aG\x80W_\x80\xFD[`\x01`\x01`@\x1B\x03\x83\x11\x15aQGWaQGaH<V[aQ[\x83aQU\x83TaN\x11V[\x83aNIV[_`\x1F\x84\x11`\x01\x81\x14aQ\x87W_\x85\x15aQuWP\x83\x82\x015[aQ\x7F\x86\x82aN\x94V[\x84UPaN\x8DV[_\x83\x81R` \x81 `\x1F\x19\x87\x16\x91[\x82\x81\x10\x15aQ\xB6W\x86\x85\x015\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aQ\x96V[P\x86\x82\x10\x15aQ\xD2W_\x19`\xF8\x88`\x03\x1B\x16\x1C\x19\x84\x87\x015\x16\x81U[PP`\x01\x85`\x01\x1B\x01\x83UPPPPPV[\x815aQ\xEF\x81aG\tV[aQ\xF8\x81aF\xDAV[`\xFF\x19\x82T\x16`\xFF\x82\x16\x81\x17\x83UPP`\x01\x80\x82\x01` \x80\x85\x015`\x1E\x19\x866\x03\x01\x81\x12aR$W_\x80\xFD[\x85\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aR;W_\x80\xFD[\x806\x03\x83\x83\x01\x13\x15aRKW_\x80\xFD[aR_\x81aRY\x86TaN\x11V[\x86aNIV[_`\x1F\x82\x11`\x01\x81\x14aR\x8DW_\x83\x15aR{WP\x83\x82\x01\x85\x015[aR\x85\x84\x82aN\x94V[\x87UPa\x13\x93V[_\x86\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aR\xBBW\x86\x85\x01\x88\x015\x82U\x93\x87\x01\x93\x90\x89\x01\x90\x87\x01aR\x9CV[P\x84\x82\x10\x15aR\xD9W_\x19`\xF8\x86`\x03\x1B\x16\x1C\x19\x87\x85\x88\x01\x015\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90\x92UPPPPPV[\x84\x81R``` \x82\x01R_aS\x05``\x83\x01\x86aK\xD5V[\x82\x81\x03`@\x84\x01Ra4Z\x81\x85\x87aO\xB7V[\x84\x81R``` \x82\x01R_aS1``\x83\x01\x85\x87aO\x8FV[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16`@\x83\x01R\x95\x94PPPPPV[\x84\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R_a9\xA7`\x80\x83\x01\x84aF&V[\x83\x81R\x82` \x82\x01R```@\x82\x01R_a4\xB3``\x83\x01\x84aF&V[\x86\x81R`\x80` \x82\x01R_aP\x8D`\x80\x83\x01\x87\x89aO\x8FV[\x84\x81R``` \x82\x01R_aS\xBD``\x83\x01\x86aK\xD5V[\x82\x81\x03`@\x84\x01Ra4Z\x81\x85\x87aO\x8FV[_` \x82\x84\x03\x12\x15aS\xE0W_\x80\xFD[\x815a9\x1E\x81aLDV[\x815\x81U`\x01\x81\x01` \x83\x015aT\x01\x81aLDV[\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x91\x90\x91\x16\x17\x90UPPV[\x84\x81R``` \x80\x83\x01\x82\x90R\x90\x82\x01\x84\x90R_\x90\x85\x90`\x80\x84\x01\x83[\x87\x81\x10\x15aTvW\x835\x82R\x82\x84\x015aTW\x81aLDV[`\x01`\x01`@\x1B\x03\x16\x82\x84\x01R`@\x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01aT>V[P\x80\x93PPPP`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x95\x94PPPPPV[\x83\x81RaT\xA1\x83aF\xDAV[\x82` \x82\x01R```@\x82\x01R_a4\xB3``\x83\x01\x84aF&V[_` \x82\x84\x03\x12\x15aT\xCCW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a9\x1EW_\x80\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x83\x81R``\x81\x01aT\xFA\x84aF\xDAV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15aU4W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01aU\x18V[P\x92\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aUPW_\x80\xFD[PQ\x91\x90PV[_\x82`\x1F\x83\x01\x12aUfW_\x80\xFD[\x81QaUtaI\"\x82aH\xA8V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15aU\x88W_\x80\xFD[aJY\x82` \x83\x01` \x87\x01aF\x04V[_` \x82\x84\x03\x12\x15aU\xA9W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aU\xBFW_\x80\xFD[\x90\x83\x01\x90`\x80\x82\x86\x03\x12\x15aU\xD2W_\x80\xFD[aU\xDAaHPV[\x82QaU\xE5\x81aH(V[\x81R` \x83\x01QaU\xF5\x81aH(V[` \x82\x01R`@\x83\x01Q\x82\x81\x11\x15aV\x0BW_\x80\xFD[aV\x17\x87\x82\x86\x01aUWV[`@\x83\x01RP``\x83\x01Q\x82\x81\x11\x15aV.W_\x80\xFD[aV:\x87\x82\x86\x01aUWV[``\x83\x01RP\x95\x94PPPPPV[_\x82QaP\xE4\x81\x84` \x87\x01aF\x04V\xFE\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\x006\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)CompressedKeyMaterialsVerification(uint256 prepKeygenId,uint256 migrationRequestId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0",
    );
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `AbortCrsgenAlreadyDone(uint256)` and selector `0xdf0db5fb`.
```solidity
error AbortCrsgenAlreadyDone(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortCrsgenAlreadyDone {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AbortCrsgenAlreadyDone> for UnderlyingRustTuple<'_> {
            fn from(value: AbortCrsgenAlreadyDone) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortCrsgenAlreadyDone {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortCrsgenAlreadyDone {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortCrsgenAlreadyDone(uint256)";
            const SELECTOR: [u8; 4] = [223u8, 13u8, 181u8, 251u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `AbortCrsgenInvalidId(uint256)` and selector `0xcbe92656`.
```solidity
error AbortCrsgenInvalidId(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortCrsgenInvalidId {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AbortCrsgenInvalidId> for UnderlyingRustTuple<'_> {
            fn from(value: AbortCrsgenInvalidId) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortCrsgenInvalidId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortCrsgenInvalidId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortCrsgenInvalidId(uint256)";
            const SELECTOR: [u8; 4] = [203u8, 233u8, 38u8, 86u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `AbortKeygenAlreadyDone(uint256)` and selector `0x92789b67`.
```solidity
error AbortKeygenAlreadyDone(uint256 prepKeygenId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortKeygenAlreadyDone {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AbortKeygenAlreadyDone> for UnderlyingRustTuple<'_> {
            fn from(value: AbortKeygenAlreadyDone) -> Self {
                (value.prepKeygenId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortKeygenAlreadyDone {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { prepKeygenId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortKeygenAlreadyDone {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortKeygenAlreadyDone(uint256)";
            const SELECTOR: [u8; 4] = [146u8, 120u8, 155u8, 103u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
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
    /**Custom error with signature `AbortKeygenInvalidId(uint256)` and selector `0xfcf2db7a`.
```solidity
error AbortKeygenInvalidId(uint256 prepKeygenId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AbortKeygenInvalidId {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<AbortKeygenInvalidId> for UnderlyingRustTuple<'_> {
            fn from(value: AbortKeygenInvalidId) -> Self {
                (value.prepKeygenId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AbortKeygenInvalidId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { prepKeygenId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AbortKeygenInvalidId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AbortKeygenInvalidId(uint256)";
            const SELECTOR: [u8; 4] = [252u8, 242u8, 219u8, 122u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
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
    /**Custom error with signature `CompressedKeyCutoverAlreadyScheduled(uint256)` and selector `0x406292b6`.
```solidity
error CompressedKeyCutoverAlreadyScheduled(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CompressedKeyCutoverAlreadyScheduled {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CompressedKeyCutoverAlreadyScheduled>
        for UnderlyingRustTuple<'_> {
            fn from(value: CompressedKeyCutoverAlreadyScheduled) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CompressedKeyCutoverAlreadyScheduled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CompressedKeyCutoverAlreadyScheduled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CompressedKeyCutoverAlreadyScheduled(uint256)";
            const SELECTOR: [u8; 4] = [64u8, 98u8, 146u8, 182u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `CompressedKeyMaterialsAlreadyAdded(uint256)` and selector `0x88c770f4`.
```solidity
error CompressedKeyMaterialsAlreadyAdded(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CompressedKeyMaterialsAlreadyAdded {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CompressedKeyMaterialsAlreadyAdded>
        for UnderlyingRustTuple<'_> {
            fn from(value: CompressedKeyMaterialsAlreadyAdded) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CompressedKeyMaterialsAlreadyAdded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CompressedKeyMaterialsAlreadyAdded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CompressedKeyMaterialsAlreadyAdded(uint256)";
            const SELECTOR: [u8; 4] = [136u8, 199u8, 112u8, 244u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `CompressedKeyMaterialsNotAdded(uint256)` and selector `0xf916ee42`.
```solidity
error CompressedKeyMaterialsNotAdded(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CompressedKeyMaterialsNotAdded {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CompressedKeyMaterialsNotAdded>
        for UnderlyingRustTuple<'_> {
            fn from(value: CompressedKeyMaterialsNotAdded) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CompressedKeyMaterialsNotAdded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CompressedKeyMaterialsNotAdded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CompressedKeyMaterialsNotAdded(uint256)";
            const SELECTOR: [u8; 4] = [249u8, 22u8, 238u8, 66u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `CrsAborted(uint256)` and selector `0xd5fd3cd7`.
```solidity
error CrsAborted(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsAborted {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CrsAborted> for UnderlyingRustTuple<'_> {
            fn from(value: CrsAborted) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsAborted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsAborted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsAborted(uint256)";
            const SELECTOR: [u8; 4] = [213u8, 253u8, 60u8, 215u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `CrsNotGenerated(uint256)` and selector `0xda32d00f`.
```solidity
error CrsNotGenerated(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsNotGenerated {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CrsNotGenerated> for UnderlyingRustTuple<'_> {
            fn from(value: CrsNotGenerated) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsNotGenerated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsNotGenerated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsNotGenerated(uint256)";
            const SELECTOR: [u8; 4] = [218u8, 50u8, 208u8, 15u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `CrsgenNotRequested(uint256)` and selector `0x8d8c940a`.
```solidity
error CrsgenNotRequested(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsgenNotRequested {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CrsgenNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: CrsgenNotRequested) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsgenNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsgenNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsgenNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [141u8, 140u8, 148u8, 10u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `CrsgenOngoing(uint256)` and selector `0x061ac61d`.
```solidity
error CrsgenOngoing(uint256 crsId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CrsgenOngoing {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<CrsgenOngoing> for UnderlyingRustTuple<'_> {
            fn from(value: CrsgenOngoing) -> Self {
                (value.crsId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CrsgenOngoing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { crsId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CrsgenOngoing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CrsgenOngoing(uint256)";
            const SELECTOR: [u8; 4] = [6u8, 26u8, 198u8, 29u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
    /**Custom error with signature `DeserializingExtraDataFail()` and selector `0x8b248b60`.
```solidity
error DeserializingExtraDataFail();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeserializingExtraDataFail;
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
        impl ::core::convert::From<DeserializingExtraDataFail>
        for UnderlyingRustTuple<'_> {
            fn from(value: DeserializingExtraDataFail) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for DeserializingExtraDataFail {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DeserializingExtraDataFail {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DeserializingExtraDataFail()";
            const SELECTOR: [u8; 4] = [139u8, 36u8, 139u8, 96u8];
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
    /**Custom error with signature `DuplicateCutoverChainId(uint256)` and selector `0x940ad6d6`.
```solidity
error DuplicateCutoverChainId(uint256 chainId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DuplicateCutoverChainId {
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
        impl ::core::convert::From<DuplicateCutoverChainId> for UnderlyingRustTuple<'_> {
            fn from(value: DuplicateCutoverChainId) -> Self {
                (value.chainId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DuplicateCutoverChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { chainId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DuplicateCutoverChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DuplicateCutoverChainId(uint256)";
            const SELECTOR: [u8; 4] = [148u8, 10u8, 214u8, 214u8];
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
    /**Custom error with signature `EmptyHostChainCutovers()` and selector `0x94d69620`.
```solidity
error EmptyHostChainCutovers();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyHostChainCutovers;
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
        impl ::core::convert::From<EmptyHostChainCutovers> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyHostChainCutovers) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyHostChainCutovers {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyHostChainCutovers {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyHostChainCutovers()";
            const SELECTOR: [u8; 4] = [148u8, 214u8, 150u8, 32u8];
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
    /**Custom error with signature `EmptyKeyDigests(uint256)` and selector `0xe6f9083b`.
```solidity
error EmptyKeyDigests(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyKeyDigests {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<EmptyKeyDigests> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyKeyDigests) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyKeyDigests {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyKeyDigests {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyKeyDigests(uint256)";
            const SELECTOR: [u8; 4] = [230u8, 249u8, 8u8, 59u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Custom error with signature `InvalidCutoverBlock()` and selector `0x48d787f6`.
```solidity
error InvalidCutoverBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCutoverBlock;
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
        impl ::core::convert::From<InvalidCutoverBlock> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCutoverBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCutoverBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCutoverBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCutoverBlock()";
            const SELECTOR: [u8; 4] = [72u8, 215u8, 135u8, 246u8];
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
    /**Custom error with signature `KeyAborted(uint256)` and selector `0x83f18335`.
```solidity
error KeyAborted(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyAborted {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KeyAborted> for UnderlyingRustTuple<'_> {
            fn from(value: KeyAborted) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyAborted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyAborted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyAborted(uint256)";
            const SELECTOR: [u8; 4] = [131u8, 241u8, 131u8, 53u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `KeyManagementRequestPending()` and selector `0x6fbcdd2b`.
```solidity
error KeyManagementRequestPending();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyManagementRequestPending;
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
        impl ::core::convert::From<KeyManagementRequestPending>
        for UnderlyingRustTuple<'_> {
            fn from(value: KeyManagementRequestPending) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KeyManagementRequestPending {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyManagementRequestPending {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyManagementRequestPending()";
            const SELECTOR: [u8; 4] = [111u8, 188u8, 221u8, 43u8];
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
    /**Custom error with signature `KeyNotGenerated(uint256)` and selector `0x84de1331`.
```solidity
error KeyNotGenerated(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyNotGenerated {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KeyNotGenerated> for UnderlyingRustTuple<'_> {
            fn from(value: KeyNotGenerated) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyNotGenerated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyNotGenerated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyNotGenerated(uint256)";
            const SELECTOR: [u8; 4] = [132u8, 222u8, 19u8, 49u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `KeygenNotRequested(uint256)` and selector `0xadfab904`.
```solidity
error KeygenNotRequested(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeygenNotRequested {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KeygenNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: KeygenNotRequested) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeygenNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeygenNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeygenNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [173u8, 250u8, 185u8, 4u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `KeygenOngoing(uint256)` and selector `0x3b853da8`.
```solidity
error KeygenOngoing(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeygenOngoing {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<KeygenOngoing> for UnderlyingRustTuple<'_> {
            fn from(value: KeygenOngoing) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeygenOngoing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeygenOngoing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeygenOngoing(uint256)";
            const SELECTOR: [u8; 4] = [59u8, 133u8, 61u8, 168u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
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
    /**Custom error with signature `KmsAlreadySignedForCrsgen(uint256,address)` and selector `0xfcf5a6e9`.
```solidity
error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsAlreadySignedForCrsgen {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsSigner: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsAlreadySignedForCrsgen>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsAlreadySignedForCrsgen) -> Self {
                (value.crsId, value.kmsSigner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsAlreadySignedForCrsgen {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    crsId: tuple.0,
                    kmsSigner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsAlreadySignedForCrsgen {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsAlreadySignedForCrsgen(uint256,address)";
            const SELECTOR: [u8; 4] = [252u8, 245u8, 166u8, 233u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsSigner,
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
    /**Custom error with signature `KmsAlreadySignedForKeygen(uint256,address)` and selector `0x98fb957d`.
```solidity
error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsAlreadySignedForKeygen {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsSigner: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsAlreadySignedForKeygen>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsAlreadySignedForKeygen) -> Self {
                (value.keyId, value.kmsSigner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsAlreadySignedForKeygen {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    keyId: tuple.0,
                    kmsSigner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsAlreadySignedForKeygen {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsAlreadySignedForKeygen(uint256,address)";
            const SELECTOR: [u8; 4] = [152u8, 251u8, 149u8, 125u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsSigner,
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
    /**Custom error with signature `KmsAlreadySignedForPrepKeygen(uint256,address)` and selector `0x33ca1fe3`.
```solidity
error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KmsAlreadySignedForPrepKeygen {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsSigner: alloy::sol_types::private::Address,
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
        impl ::core::convert::From<KmsAlreadySignedForPrepKeygen>
        for UnderlyingRustTuple<'_> {
            fn from(value: KmsAlreadySignedForPrepKeygen) -> Self {
                (value.prepKeygenId, value.kmsSigner)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for KmsAlreadySignedForPrepKeygen {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    prepKeygenId: tuple.0,
                    kmsSigner: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KmsAlreadySignedForPrepKeygen {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KmsAlreadySignedForPrepKeygen(uint256,address)";
            const SELECTOR: [u8; 4] = [51u8, 202u8, 31u8, 227u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsSigner,
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
    /**Custom error with signature `PrepKeygenNotRequested(uint256)` and selector `0x0ab7f687`.
```solidity
error PrepKeygenNotRequested(uint256 prepKeygenId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PrepKeygenNotRequested {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<PrepKeygenNotRequested> for UnderlyingRustTuple<'_> {
            fn from(value: PrepKeygenNotRequested) -> Self {
                (value.prepKeygenId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PrepKeygenNotRequested {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { prepKeygenId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PrepKeygenNotRequested {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PrepKeygenNotRequested(uint256)";
            const SELECTOR: [u8; 4] = [10u8, 183u8, 246u8, 135u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
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
    /**Custom error with signature `WrongKeygenResponseEndpoint(uint256)` and selector `0xb44816b6`.
```solidity
error WrongKeygenResponseEndpoint(uint256 requestId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WrongKeygenResponseEndpoint {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<WrongKeygenResponseEndpoint>
        for UnderlyingRustTuple<'_> {
            fn from(value: WrongKeygenResponseEndpoint) -> Self {
                (value.requestId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for WrongKeygenResponseEndpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { requestId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongKeygenResponseEndpoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongKeygenResponseEndpoint(uint256)";
            const SELECTOR: [u8; 4] = [180u8, 72u8, 22u8, 182u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
    /**Event with signature `AbortCrsgen(uint256)` and selector `0x384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e`.
```solidity
event AbortCrsgen(uint256 crsId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AbortCrsgen {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for AbortCrsgen {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AbortCrsgen(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                56u8, 79u8, 144u8, 254u8, 251u8, 207u8, 170u8, 104u8, 242u8, 46u8, 0u8,
                9u8, 74u8, 234u8, 165u8, 43u8, 43u8, 198u8, 147u8, 147u8, 109u8, 44u8,
                225u8, 175u8, 237u8, 18u8, 18u8, 82u8, 11u8, 89u8, 181u8, 142u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { crsId: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
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
        impl alloy_sol_types::private::IntoLogData for AbortCrsgen {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AbortCrsgen> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AbortCrsgen) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `AbortKeygen(uint256)` and selector `0x2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe3264`.
```solidity
event AbortKeygen(uint256 prepKeygenId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AbortKeygen {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for AbortKeygen {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AbortKeygen(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                43u8, 8u8, 123u8, 136u8, 75u8, 53u8, 168u8, 29u8, 118u8, 157u8, 26u8,
                30u8, 9u8, 40u8, 128u8, 241u8, 218u8, 86u8, 222u8, 150u8, 78u8, 75u8,
                51u8, 158u8, 171u8, 203u8, 31u8, 69u8, 245u8, 254u8, 50u8, 100u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { prepKeygenId: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
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
        impl alloy_sol_types::private::IntoLogData for AbortKeygen {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AbortKeygen> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AbortKeygen) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ActivateCrs(uint256,string[],bytes)` and selector `0x2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a2`.
```solidity
event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActivateCrs {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub crsDigest: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for ActivateCrs {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ActivateCrs(uint256,string[],bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                34u8, 88u8, 183u8, 63u8, 174u8, 211u8, 63u8, 178u8, 226u8, 234u8, 69u8,
                68u8, 3u8, 190u8, 249u8, 116u8, 146u8, 12u8, 175u8, 104u8, 42u8, 179u8,
                167u8, 35u8, 72u8, 79u8, 207u8, 103u8, 85u8, 59u8, 22u8, 162u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    crsId: data.0,
                    kmsNodeStorageUrls: data.1,
                    crsDigest: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsDigest,
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
        impl alloy_sol_types::private::IntoLogData for ActivateCrs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActivateCrs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActivateCrs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Event with signature `ActivateKey(uint256,string[],(uint8,bytes)[])` and selector `0xeb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b`.
```solidity
event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActivateKey {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for ActivateKey {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ActivateKey(uint256,string[],(uint8,bytes)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8, 133u8, 194u8, 109u8, 188u8, 173u8, 70u8, 184u8, 10u8, 104u8,
                160u8, 242u8, 76u8, 206u8, 124u8, 44u8, 144u8, 240u8, 161u8, 250u8,
                222u8, 216u8, 65u8, 132u8, 19u8, 136u8, 57u8, 252u8, 158u8, 128u8, 162u8,
                91u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    keyId: data.0,
                    kmsNodeStorageUrls: data.1,
                    keyDigests: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
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
        impl alloy_sol_types::private::IntoLogData for ActivateKey {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActivateKey> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActivateKey) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CompressedKeyCutoverScheduled(uint256,(uint256,uint64)[],uint64)` and selector `0x78cec34fd2715b935e4af4ae695f75a2c57cbd9655f1e7e3cc86c97ce79fa29f`.
```solidity
event CompressedKeyCutoverScheduled(uint256 keyId, IKMSGeneration.HostChainCutover[] hostChainCutovers, uint64 gatewayCutoverBlock);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CompressedKeyCutoverScheduled {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hostChainCutovers: alloy::sol_types::private::Vec<
            <IKMSGeneration::HostChainCutover as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub gatewayCutoverBlock: u64,
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
        impl alloy_sol_types::SolEvent for CompressedKeyCutoverScheduled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::HostChainCutover>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CompressedKeyCutoverScheduled(uint256,(uint256,uint64)[],uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                120u8, 206u8, 195u8, 79u8, 210u8, 113u8, 91u8, 147u8, 94u8, 74u8, 244u8,
                174u8, 105u8, 95u8, 117u8, 162u8, 197u8, 124u8, 189u8, 150u8, 85u8,
                241u8, 231u8, 227u8, 204u8, 134u8, 201u8, 124u8, 231u8, 159u8, 162u8,
                159u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    keyId: data.0,
                    hostChainCutovers: data.1,
                    gatewayCutoverBlock: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::HostChainCutover,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostChainCutovers),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.gatewayCutoverBlock),
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
        impl alloy_sol_types::private::IntoLogData for CompressedKeyCutoverScheduled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CompressedKeyCutoverScheduled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CompressedKeyCutoverScheduled,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Event with signature `CompressedKeyMaterialAdded(uint256,string[],(uint8,bytes)[])` and selector `0x80ebc2a4e183000f6837fab1e36970e8bc4a1b19223054c32769db663a4ce346`.
```solidity
event CompressedKeyMaterialAdded(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CompressedKeyMaterialAdded {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for CompressedKeyMaterialAdded {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CompressedKeyMaterialAdded(uint256,string[],(uint8,bytes)[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                128u8, 235u8, 194u8, 164u8, 225u8, 131u8, 0u8, 15u8, 104u8, 55u8, 250u8,
                177u8, 227u8, 105u8, 112u8, 232u8, 188u8, 74u8, 27u8, 25u8, 34u8, 48u8,
                84u8, 195u8, 39u8, 105u8, 219u8, 102u8, 58u8, 76u8, 227u8, 70u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    keyId: data.0,
                    kmsNodeStorageUrls: data.1,
                    keyDigests: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
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
        impl alloy_sol_types::private::IntoLogData for CompressedKeyMaterialAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CompressedKeyMaterialAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CompressedKeyMaterialAdded,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Event with signature `CompressedKeyMaterialResponse(uint256,(uint8,bytes)[],bytes,address)` and selector `0x03c88abb63cb1a33cba1e37393e7fc01f203bb037b92f5ef84d52e5c63ac7be7`.
```solidity
event CompressedKeyMaterialResponse(uint256 migrationRequestId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CompressedKeyMaterialResponse {
        #[allow(missing_docs)]
        pub migrationRequestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for CompressedKeyMaterialResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CompressedKeyMaterialResponse(uint256,(uint8,bytes)[],bytes,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                3u8, 200u8, 138u8, 187u8, 99u8, 203u8, 26u8, 51u8, 203u8, 161u8, 227u8,
                115u8, 147u8, 231u8, 252u8, 1u8, 242u8, 3u8, 187u8, 3u8, 123u8, 146u8,
                245u8, 239u8, 132u8, 213u8, 46u8, 92u8, 99u8, 172u8, 123u8, 231u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    migrationRequestId: data.0,
                    keyDigests: data.1,
                    signature: data.2,
                    kmsTxSender: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.migrationRequestId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for CompressedKeyMaterialResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CompressedKeyMaterialResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CompressedKeyMaterialResponse,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CompressedKeyMigrationKeygenRequest(uint256,uint256,uint256,bytes)` and selector `0xb07c5a1b25df43bfe6d702bac06c3d6e151a71da406a6cba07ffa41700dcabe1`.
```solidity
event CompressedKeyMigrationKeygenRequest(uint256 prepKeygenId, uint256 migrationRequestId, uint256 keyId, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CompressedKeyMigrationKeygenRequest {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub migrationRequestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for CompressedKeyMigrationKeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CompressedKeyMigrationKeygenRequest(uint256,uint256,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                176u8, 124u8, 90u8, 27u8, 37u8, 223u8, 67u8, 191u8, 230u8, 215u8, 2u8,
                186u8, 192u8, 108u8, 61u8, 110u8, 21u8, 26u8, 113u8, 218u8, 64u8, 106u8,
                108u8, 186u8, 7u8, 255u8, 164u8, 23u8, 0u8, 220u8, 171u8, 225u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    migrationRequestId: data.1,
                    keyId: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.migrationRequestId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData
        for CompressedKeyMigrationKeygenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CompressedKeyMigrationKeygenRequest>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CompressedKeyMigrationKeygenRequest,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CompressedKeyMigrationPrepKeygenRequest(uint256,uint256,uint8,bytes)` and selector `0x2869c51322e90d2ce2dfe55ec29d180f36bffa15b434fc0a8dd0e5d3ce3c174c`.
```solidity
event CompressedKeyMigrationPrepKeygenRequest(uint256 prepKeygenId, uint256 keyId, IKMSGeneration.ParamsType paramsType, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CompressedKeyMigrationPrepKeygenRequest {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for CompressedKeyMigrationPrepKeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CompressedKeyMigrationPrepKeygenRequest(uint256,uint256,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8, 105u8, 197u8, 19u8, 34u8, 233u8, 13u8, 44u8, 226u8, 223u8, 229u8,
                94u8, 194u8, 157u8, 24u8, 15u8, 54u8, 191u8, 250u8, 21u8, 180u8, 52u8,
                252u8, 10u8, 141u8, 208u8, 229u8, 211u8, 206u8, 60u8, 23u8, 76u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    keyId: data.1,
                    paramsType: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData
        for CompressedKeyMigrationPrepKeygenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CompressedKeyMigrationPrepKeygenRequest>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &CompressedKeyMigrationPrepKeygenRequest,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CrsgenRequest(uint256,uint256,uint8,bytes)` and selector `0x8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d`.
```solidity
event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CrsgenRequest {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub maxBitLength: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for CrsgenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CrsgenRequest(uint256,uint256,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8, 240u8, 21u8, 19u8, 147u8, 248u8, 79u8, 214u8, 148u8, 197u8, 227u8,
                21u8, 203u8, 116u8, 204u8, 5u8, 178u8, 71u8, 222u8, 10u8, 69u8, 79u8,
                217u8, 233u8, 18u8, 156u8, 102u8, 30u8, 253u8, 249u8, 64u8, 29u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    crsId: data.0,
                    maxBitLength: data.1,
                    paramsType: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitLength),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData for CrsgenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CrsgenRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CrsgenRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CrsgenResponse(uint256,bytes,bytes,address)` and selector `0x7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd`.
```solidity
event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CrsgenResponse {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub crsDigest: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for CrsgenResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "CrsgenResponse(uint256,bytes,bytes,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                123u8, 241u8, 180u8, 44u8, 16u8, 233u8, 73u8, 124u8, 135u8, 150u8, 32u8,
                197u8, 183u8, 175u8, 206u8, 209u8, 11u8, 218u8, 23u8, 216u8, 201u8, 11u8,
                34u8, 240u8, 227u8, 188u8, 107u8, 47u8, 214u8, 206u8, 208u8, 189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    crsId: data.0,
                    crsDigest: data.1,
                    signature: data.2,
                    kmsTxSender: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsDigest,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for CrsgenResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CrsgenResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CrsgenResponse) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
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
    /**Event with signature `KeygenRequest(uint256,uint256,bytes)` and selector `0x3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee`.
```solidity
event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeygenRequest {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeygenRequest(uint256,uint256,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                58u8, 17u8, 97u8, 32u8, 204u8, 165u8, 212u8, 240u8, 115u8, 204u8, 31u8,
                195u8, 31u8, 242u8, 97u8, 51u8, 171u8, 123u8, 4u8, 153u8, 242u8, 113u8,
                47u8, 160u8, 16u8, 2u8, 59u8, 135u8, 213u8, 161u8, 249u8, 238u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    keyId: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData for KeygenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeygenRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeygenRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    /**Event with signature `KeygenResponse(uint256,(uint8,bytes)[],bytes,address)` and selector `0x2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c7`.
```solidity
event KeygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeygenResponse {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for KeygenResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeygenResponse(uint256,(uint8,bytes)[],bytes,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                42u8, 254u8, 100u8, 251u8, 58u8, 253u8, 232u8, 226u8, 103u8, 138u8,
                234u8, 132u8, 207u8, 54u8, 34u8, 63u8, 51u8, 14u8, 47u8, 177u8, 40u8,
                109u8, 55u8, 174u8, 213u8, 115u8, 171u8, 156u8, 209u8, 219u8, 71u8, 199u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    keyId: data.0,
                    keyDigests: data.1,
                    signature: data.2,
                    kmsTxSender: data.3,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for KeygenResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeygenResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeygenResponse) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PrepKeygenRequest(uint256,uint8,bytes)` and selector `0xfbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe91`.
```solidity
event PrepKeygenRequest(uint256 prepKeygenId, IKMSGeneration.ParamsType paramsType, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PrepKeygenRequest {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for PrepKeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PrepKeygenRequest(uint256,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8, 245u8, 39u8, 72u8, 16u8, 185u8, 79u8, 134u8, 151u8, 12u8, 17u8,
                71u8, 232u8, 255u8, 174u8, 190u8, 210u8, 70u8, 238u8, 151u8, 119u8,
                214u8, 149u8, 166u8, 144u8, 4u8, 220u8, 98u8, 86u8, 209u8, 254u8, 145u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    paramsType: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.extraData,
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
        impl alloy_sol_types::private::IntoLogData for PrepKeygenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PrepKeygenRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PrepKeygenRequest) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `PrepKeygenResponse(uint256,bytes,address)` and selector `0x4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c`.
```solidity
event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PrepKeygenResponse {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub kmsTxSender: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PrepKeygenResponse {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PrepKeygenResponse(uint256,bytes,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                76u8, 113u8, 92u8, 87u8, 52u8, 206u8, 92u8, 24u8, 201u8, 193u8, 46u8,
                132u8, 150u8, 229u8, 61u8, 42u8, 101u8, 241u8, 236u8, 56u8, 29u8, 71u8,
                105u8, 87u8, 240u8, 245u8, 150u8, 179u8, 100u8, 165u8, 155u8, 12u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prepKeygenId: data.0,
                    signature: data.1,
                    kmsTxSender: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.kmsTxSender,
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
        impl alloy_sol_types::private::IntoLogData for PrepKeygenResponse {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PrepKeygenResponse> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PrepKeygenResponse) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `abortCrsgen(uint256)` and selector `0x1703c61a`.
```solidity
function abortCrsgen(uint256 crsId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortCrsgenCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`abortCrsgen(uint256)`](abortCrsgenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortCrsgenReturn {}
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
            impl ::core::convert::From<abortCrsgenCall> for UnderlyingRustTuple<'_> {
                fn from(value: abortCrsgenCall) -> Self {
                    (value.crsId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortCrsgenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { crsId: tuple.0 }
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
            impl ::core::convert::From<abortCrsgenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: abortCrsgenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortCrsgenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl abortCrsgenReturn {
            fn _tokenize(
                &self,
            ) -> <abortCrsgenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for abortCrsgenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = abortCrsgenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "abortCrsgen(uint256)";
            const SELECTOR: [u8; 4] = [23u8, 3u8, 198u8, 26u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                abortCrsgenReturn::_tokenize(ret)
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
    /**Function with signature `abortKeygen(uint256)` and selector `0xc2c1faee`.
```solidity
function abortKeygen(uint256 prepKeygenId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortKeygenCall {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`abortKeygen(uint256)`](abortKeygenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct abortKeygenReturn {}
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
            impl ::core::convert::From<abortKeygenCall> for UnderlyingRustTuple<'_> {
                fn from(value: abortKeygenCall) -> Self {
                    (value.prepKeygenId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortKeygenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { prepKeygenId: tuple.0 }
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
            impl ::core::convert::From<abortKeygenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: abortKeygenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for abortKeygenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl abortKeygenReturn {
            fn _tokenize(
                &self,
            ) -> <abortKeygenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for abortKeygenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = abortKeygenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "abortKeygen(uint256)";
            const SELECTOR: [u8; 4] = [194u8, 193u8, 250u8, 238u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                abortKeygenReturn::_tokenize(ret)
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
    #[derive()]
    /**Function with signature `addCompressedKeyMaterials(uint256,(uint8,bytes)[],bytes)` and selector `0xc1252ca9`.
```solidity
function addCompressedKeyMaterials(uint256 migrationRequestId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addCompressedKeyMaterialsCall {
        #[allow(missing_docs)]
        pub migrationRequestId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`addCompressedKeyMaterials(uint256,(uint8,bytes)[],bytes)`](addCompressedKeyMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addCompressedKeyMaterialsReturn {}
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
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addCompressedKeyMaterialsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addCompressedKeyMaterialsCall) -> Self {
                    (value.migrationRequestId, value.keyDigests, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addCompressedKeyMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        migrationRequestId: tuple.0,
                        keyDigests: tuple.1,
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
            impl ::core::convert::From<addCompressedKeyMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addCompressedKeyMaterialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addCompressedKeyMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addCompressedKeyMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <addCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addCompressedKeyMaterialsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addCompressedKeyMaterialsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addCompressedKeyMaterials(uint256,(uint8,bytes)[],bytes)";
            const SELECTOR: [u8; 4] = [193u8, 37u8, 44u8, 169u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.migrationRequestId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addCompressedKeyMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `compressedKeyMigrationKeygen(uint256)` and selector `0x9081fe13`.
```solidity
function compressedKeyMigrationKeygen(uint256 keyId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct compressedKeyMigrationKeygenCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`compressedKeyMigrationKeygen(uint256)`](compressedKeyMigrationKeygenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct compressedKeyMigrationKeygenReturn {}
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
            impl ::core::convert::From<compressedKeyMigrationKeygenCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: compressedKeyMigrationKeygenCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for compressedKeyMigrationKeygenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
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
            impl ::core::convert::From<compressedKeyMigrationKeygenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: compressedKeyMigrationKeygenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for compressedKeyMigrationKeygenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl compressedKeyMigrationKeygenReturn {
            fn _tokenize(
                &self,
            ) -> <compressedKeyMigrationKeygenCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for compressedKeyMigrationKeygenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = compressedKeyMigrationKeygenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "compressedKeyMigrationKeygen(uint256)";
            const SELECTOR: [u8; 4] = [144u8, 129u8, 254u8, 19u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                compressedKeyMigrationKeygenReturn::_tokenize(ret)
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
    /**Function with signature `crsgenRequest(uint256,uint8)` and selector `0x3c02f834`.
```solidity
function crsgenRequest(uint256 maxBitLength, IKMSGeneration.ParamsType paramsType) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenRequestCall {
        #[allow(missing_docs)]
        pub maxBitLength: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`crsgenRequest(uint256,uint8)`](crsgenRequestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenRequestReturn {}
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
                IKMSGeneration::ParamsType,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<crsgenRequestCall> for UnderlyingRustTuple<'_> {
                fn from(value: crsgenRequestCall) -> Self {
                    (value.maxBitLength, value.paramsType)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crsgenRequestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maxBitLength: tuple.0,
                        paramsType: tuple.1,
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
            impl ::core::convert::From<crsgenRequestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: crsgenRequestReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crsgenRequestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl crsgenRequestReturn {
            fn _tokenize(
                &self,
            ) -> <crsgenRequestCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for crsgenRequestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IKMSGeneration::ParamsType,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = crsgenRequestReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "crsgenRequest(uint256,uint8)";
            const SELECTOR: [u8; 4] = [60u8, 2u8, 248u8, 52u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.maxBitLength),
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                crsgenRequestReturn::_tokenize(ret)
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
    /**Function with signature `crsgenResponse(uint256,bytes,bytes)` and selector `0x62978787`.
```solidity
function crsgenResponse(uint256 crsId, bytes memory crsDigest, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenResponseCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub crsDigest: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`crsgenResponse(uint256,bytes,bytes)`](crsgenResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct crsgenResponseReturn {}
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
            impl ::core::convert::From<crsgenResponseCall> for UnderlyingRustTuple<'_> {
                fn from(value: crsgenResponseCall) -> Self {
                    (value.crsId, value.crsDigest, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for crsgenResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        crsId: tuple.0,
                        crsDigest: tuple.1,
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
            impl ::core::convert::From<crsgenResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: crsgenResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for crsgenResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl crsgenResponseReturn {
            fn _tokenize(
                &self,
            ) -> <crsgenResponseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for crsgenResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = crsgenResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "crsgenResponse(uint256,bytes,bytes)";
            const SELECTOR: [u8; 4] = [98u8, 151u8, 135u8, 135u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.crsDigest,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                crsgenResponseReturn::_tokenize(ret)
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
    /**Function with signature `getActiveCrsId()` and selector `0xbaff211e`.
```solidity
function getActiveCrsId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveCrsIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveCrsId()`](getActiveCrsIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveCrsIdReturn {
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
            impl ::core::convert::From<getActiveCrsIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveCrsIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveCrsIdCall {
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
            impl ::core::convert::From<getActiveCrsIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveCrsIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveCrsIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveCrsIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveCrsId()";
            const SELECTOR: [u8; 4] = [186u8, 255u8, 33u8, 30u8];
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
                        let r: getActiveCrsIdReturn = r.into();
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
                        let r: getActiveCrsIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getActiveKeyId()` and selector `0xd52f10eb`.
```solidity
function getActiveKeyId() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveKeyIdCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveKeyId()`](getActiveKeyIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveKeyIdReturn {
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
            impl ::core::convert::From<getActiveKeyIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveKeyIdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveKeyIdCall {
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
            impl ::core::convert::From<getActiveKeyIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveKeyIdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveKeyIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveKeyIdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveKeyId()";
            const SELECTOR: [u8; 4] = [213u8, 47u8, 16u8, 235u8];
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
                        let r: getActiveKeyIdReturn = r.into();
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
                        let r: getActiveKeyIdReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCompletedCrsIds()` and selector `0xdabd732f`.
```solidity
function getCompletedCrsIds() external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompletedCrsIdsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCompletedCrsIds()`](getCompletedCrsIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompletedCrsIdsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getCompletedCrsIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompletedCrsIdsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompletedCrsIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getCompletedCrsIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompletedCrsIdsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompletedCrsIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCompletedCrsIdsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCompletedCrsIds()";
            const SELECTOR: [u8; 4] = [218u8, 189u8, 115u8, 47u8];
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCompletedCrsIdsReturn = r.into();
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
                        let r: getCompletedCrsIdsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCompletedKeyIds()` and selector `0xe410117e`.
```solidity
function getCompletedKeyIds() external view returns (uint256[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompletedKeyIdsCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCompletedKeyIds()`](getCompletedKeyIdsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompletedKeyIdsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getCompletedKeyIdsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompletedKeyIdsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompletedKeyIdsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getCompletedKeyIdsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompletedKeyIdsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompletedKeyIdsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCompletedKeyIdsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCompletedKeyIds()";
            const SELECTOR: [u8; 4] = [228u8, 16u8, 17u8, 126u8];
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getCompletedKeyIdsReturn = r.into();
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
                        let r: getCompletedKeyIdsReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCompressedKeyCutoverSchedule(uint256)` and selector `0x7609152d`.
```solidity
function getCompressedKeyCutoverSchedule(uint256 keyId) external view returns (bool, IKMSGeneration.HostChainCutover[] memory, uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompressedKeyCutoverScheduleCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCompressedKeyCutoverSchedule(uint256)`](getCompressedKeyCutoverScheduleCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompressedKeyCutoverScheduleReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            <IKMSGeneration::HostChainCutover as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub _2: u64,
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
            impl ::core::convert::From<getCompressedKeyCutoverScheduleCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompressedKeyCutoverScheduleCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompressedKeyCutoverScheduleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Array<IKMSGeneration::HostChainCutover>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::HostChainCutover as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCompressedKeyCutoverScheduleReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompressedKeyCutoverScheduleReturn) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompressedKeyCutoverScheduleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
                }
            }
        }
        impl getCompressedKeyCutoverScheduleReturn {
            fn _tokenize(
                &self,
            ) -> <getCompressedKeyCutoverScheduleCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::HostChainCutover,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCompressedKeyCutoverScheduleCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCompressedKeyCutoverScheduleReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Array<IKMSGeneration::HostChainCutover>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCompressedKeyCutoverSchedule(uint256)";
            const SELECTOR: [u8; 4] = [118u8, 9u8, 21u8, 45u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getCompressedKeyCutoverScheduleReturn::_tokenize(ret)
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
    /**Function with signature `getCompressedKeyMaterials(uint256)` and selector `0xe711c9e7`.
```solidity
function getCompressedKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompressedKeyMaterialsCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`getCompressedKeyMaterials(uint256)`](getCompressedKeyMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCompressedKeyMaterialsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCompressedKeyMaterialsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompressedKeyMaterialsCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompressedKeyMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCompressedKeyMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCompressedKeyMaterialsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCompressedKeyMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getCompressedKeyMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <getCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCompressedKeyMaterialsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCompressedKeyMaterialsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCompressedKeyMaterials(uint256)";
            const SELECTOR: [u8; 4] = [231u8, 17u8, 201u8, 231u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getCompressedKeyMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `getConsensusTxSenders(uint256)` and selector `0x16c713d9`.
```solidity
function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConsensusTxSendersCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getConsensusTxSenders(uint256)`](getConsensusTxSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConsensusTxSendersReturn {
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
            impl ::core::convert::From<getConsensusTxSendersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConsensusTxSendersCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConsensusTxSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
            impl ::core::convert::From<getConsensusTxSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getConsensusTxSendersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getConsensusTxSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getConsensusTxSendersCall {
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
            const SIGNATURE: &'static str = "getConsensusTxSenders(uint256)";
            const SELECTOR: [u8; 4] = [22u8, 199u8, 19u8, 217u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
                        let r: getConsensusTxSendersReturn = r.into();
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
                        let r: getConsensusTxSendersReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCrsCounter()` and selector `0x3ac50072`.
```solidity
function getCrsCounter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsCounterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCrsCounter()`](getCrsCounterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsCounterReturn {
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
            impl ::core::convert::From<getCrsCounterCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCrsCounterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCrsCounterCall {
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
            impl ::core::convert::From<getCrsCounterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCrsCounterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCrsCounterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCrsCounterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCrsCounter()";
            const SELECTOR: [u8; 4] = [58u8, 197u8, 0u8, 114u8];
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
                        let r: getCrsCounterReturn = r.into();
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
                        let r: getCrsCounterReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCrsMaterials(uint256)` and selector `0xc55b8724`.
```solidity
function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsMaterialsCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCrsMaterials(uint256)`](getCrsMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsMaterialsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<getCrsMaterialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCrsMaterialsCall) -> Self {
                    (value.crsId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCrsMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { crsId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<getCrsMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCrsMaterialsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCrsMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getCrsMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <getCrsMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCrsMaterialsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCrsMaterialsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCrsMaterials(uint256)";
            const SELECTOR: [u8; 4] = [197u8, 91u8, 135u8, 36u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getCrsMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `getCrsParamsType(uint256)` and selector `0x45af261b`.
```solidity
function getCrsParamsType(uint256 crsId) external view returns (IKMSGeneration.ParamsType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsParamsTypeCall {
        #[allow(missing_docs)]
        pub crsId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCrsParamsType(uint256)`](getCrsParamsTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCrsParamsTypeReturn {
        #[allow(missing_docs)]
        pub _0: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCrsParamsTypeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCrsParamsTypeCall) -> Self {
                    (value.crsId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCrsParamsTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { crsId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IKMSGeneration::ParamsType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCrsParamsTypeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCrsParamsTypeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCrsParamsTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCrsParamsTypeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IKMSGeneration::ParamsType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCrsParamsType(uint256)";
            const SELECTOR: [u8; 4] = [69u8, 175u8, 38u8, 27u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.crsId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
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
                        let r: getCrsParamsTypeReturn = r.into();
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
                        let r: getCrsParamsTypeReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyCounter()` and selector `0x0b680733`.
```solidity
function getKeyCounter() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyCounterCall;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyCounter()`](getKeyCounterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyCounterReturn {
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
            impl ::core::convert::From<getKeyCounterCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyCounterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyCounterCall {
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
            impl ::core::convert::From<getKeyCounterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyCounterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyCounterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyCounterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyCounter()";
            const SELECTOR: [u8; 4] = [11u8, 104u8, 7u8, 51u8];
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
                        let r: getKeyCounterReturn = r.into();
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
                        let r: getKeyCounterReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyInfo(uint256)` and selector `0x6294f462`.
```solidity
function getKeyInfo(uint256 keyId) external view returns (IKMSGeneration.KeyInfo memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyInfoCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`getKeyInfo(uint256)`](getKeyInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyInfoReturn {
        #[allow(missing_docs)]
        pub _0: <IKMSGeneration::KeyInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKeyInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyInfoCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IKMSGeneration::KeyInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::KeyInfo as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IKMSGeneration::KeyInfo as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IKMSGeneration::KeyInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyInfo(uint256)";
            const SELECTOR: [u8; 4] = [98u8, 148u8, 244u8, 98u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<IKMSGeneration::KeyInfo as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(|r| {
                        let r: getKeyInfoReturn = r.into();
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
                        let r: getKeyInfoReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyMaterials(uint256)` and selector `0x936608ae`.
```solidity
function getKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialsCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    ///Container type for the return parameters of the [`getKeyMaterials(uint256)`](getKeyMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialsReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKeyMaterialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialsCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKeyMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        impl getKeyMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <getKeyMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyMaterialsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getKeyMaterialsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyMaterials(uint256)";
            const SELECTOR: [u8; 4] = [147u8, 102u8, 8u8, 174u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getKeyMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `getKeyParamsType(uint256)` and selector `0x19f4f632`.
```solidity
function getKeyParamsType(uint256 keyId) external view returns (IKMSGeneration.ParamsType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyParamsTypeCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyParamsType(uint256)`](getKeyParamsTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyParamsTypeReturn {
        #[allow(missing_docs)]
        pub _0: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKeyParamsTypeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyParamsTypeCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyParamsTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IKMSGeneration::ParamsType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getKeyParamsTypeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyParamsTypeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyParamsTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyParamsTypeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IKMSGeneration::ParamsType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyParamsType(uint256)";
            const SELECTOR: [u8; 4] = [25u8, 244u8, 246u8, 50u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
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
                        let r: getKeyParamsTypeReturn = r.into();
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
                        let r: getKeyParamsTypeReturn = r.into();
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
    /**Function with signature `isRequestDone(uint256)` and selector `0x3d5ec7e3`.
```solidity
function isRequestDone(uint256 requestId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRequestDoneCall {
        #[allow(missing_docs)]
        pub requestId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isRequestDone(uint256)`](isRequestDoneCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRequestDoneReturn {
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
            impl ::core::convert::From<isRequestDoneCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRequestDoneCall) -> Self {
                    (value.requestId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRequestDoneCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requestId: tuple.0 }
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
            impl ::core::convert::From<isRequestDoneReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRequestDoneReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRequestDoneReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRequestDoneCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRequestDone(uint256)";
            const SELECTOR: [u8; 4] = [61u8, 94u8, 199u8, 227u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.requestId),
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
                        let r: isRequestDoneReturn = r.into();
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
                        let r: isRequestDoneReturn = r.into();
                        r._0
                    })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `keygen(uint8)` and selector `0xcaa367db`.
```solidity
function keygen(IKMSGeneration.ParamsType paramsType) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenCall {
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`keygen(uint8)`](keygenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenReturn {}
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
            type UnderlyingSolTuple<'a> = (IKMSGeneration::ParamsType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<keygenCall> for UnderlyingRustTuple<'_> {
                fn from(value: keygenCall) -> Self {
                    (value.paramsType,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { paramsType: tuple.0 }
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
            impl ::core::convert::From<keygenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: keygenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl keygenReturn {
            fn _tokenize(
                &self,
            ) -> <keygenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keygenCall {
            type Parameters<'a> = (IKMSGeneration::ParamsType,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = keygenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keygen(uint8)";
            const SELECTOR: [u8; 4] = [202u8, 163u8, 103u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IKMSGeneration::ParamsType as alloy_sol_types::SolType>::tokenize(
                        &self.paramsType,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                keygenReturn::_tokenize(ret)
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
    #[derive()]
    /**Function with signature `keygenResponse(uint256,(uint8,bytes)[],bytes)` and selector `0x4610ffe8`.
```solidity
function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenResponseCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`keygenResponse(uint256,(uint8,bytes)[],bytes)`](keygenResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keygenResponseReturn {}
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
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<keygenResponseCall> for UnderlyingRustTuple<'_> {
                fn from(value: keygenResponseCall) -> Self {
                    (value.keyId, value.keyDigests, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keygenResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        keyId: tuple.0,
                        keyDigests: tuple.1,
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
            impl ::core::convert::From<keygenResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: keygenResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for keygenResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl keygenResponseReturn {
            fn _tokenize(
                &self,
            ) -> <keygenResponseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keygenResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = keygenResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keygenResponse(uint256,(uint8,bytes)[],bytes)";
            const SELECTOR: [u8; 4] = [70u8, 16u8, 255u8, 232u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                keygenResponseReturn::_tokenize(ret)
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
    /**Function with signature `prepKeygenResponse(uint256,bytes)` and selector `0x589adb0e`.
```solidity
function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prepKeygenResponseCall {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`prepKeygenResponse(uint256,bytes)`](prepKeygenResponseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct prepKeygenResponseReturn {}
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<prepKeygenResponseCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: prepKeygenResponseCall) -> Self {
                    (value.prepKeygenId, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for prepKeygenResponseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        prepKeygenId: tuple.0,
                        signature: tuple.1,
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
            impl ::core::convert::From<prepKeygenResponseReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: prepKeygenResponseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for prepKeygenResponseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl prepKeygenResponseReturn {
            fn _tokenize(
                &self,
            ) -> <prepKeygenResponseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for prepKeygenResponseCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = prepKeygenResponseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "prepKeygenResponse(uint256,bytes)";
            const SELECTOR: [u8; 4] = [88u8, 154u8, 219u8, 14u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.prepKeygenId),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                prepKeygenResponseReturn::_tokenize(ret)
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `scheduleCompressedKeyCutover(uint256,(uint256,uint64)[],uint64)` and selector `0xbc5404a0`.
```solidity
function scheduleCompressedKeyCutover(uint256 keyId, IKMSGeneration.HostChainCutover[] memory hostChainCutovers, uint64 gatewayCutoverBlock) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleCompressedKeyCutoverCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hostChainCutovers: alloy::sol_types::private::Vec<
            <IKMSGeneration::HostChainCutover as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub gatewayCutoverBlock: u64,
    }
    ///Container type for the return parameters of the [`scheduleCompressedKeyCutover(uint256,(uint256,uint64)[],uint64)`](scheduleCompressedKeyCutoverCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleCompressedKeyCutoverReturn {}
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
                alloy::sol_types::sol_data::Array<IKMSGeneration::HostChainCutover>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::HostChainCutover as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<scheduleCompressedKeyCutoverCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleCompressedKeyCutoverCall) -> Self {
                    (value.keyId, value.hostChainCutovers, value.gatewayCutoverBlock)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleCompressedKeyCutoverCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        keyId: tuple.0,
                        hostChainCutovers: tuple.1,
                        gatewayCutoverBlock: tuple.2,
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
            impl ::core::convert::From<scheduleCompressedKeyCutoverReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleCompressedKeyCutoverReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleCompressedKeyCutoverReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scheduleCompressedKeyCutoverReturn {
            fn _tokenize(
                &self,
            ) -> <scheduleCompressedKeyCutoverCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduleCompressedKeyCutoverCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::HostChainCutover>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scheduleCompressedKeyCutoverReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scheduleCompressedKeyCutover(uint256,(uint256,uint64)[],uint64)";
            const SELECTOR: [u8; 4] = [188u8, 84u8, 4u8, 160u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::HostChainCutover,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostChainCutovers),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.gatewayCutoverBlock),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scheduleCompressedKeyCutoverReturn::_tokenize(ret)
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
    ///Container for all the [`KMSGeneration`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum KMSGenerationCalls {
        #[allow(missing_docs)]
        UPGRADE_INTERFACE_VERSION(UPGRADE_INTERFACE_VERSIONCall),
        #[allow(missing_docs)]
        abortCrsgen(abortCrsgenCall),
        #[allow(missing_docs)]
        abortKeygen(abortKeygenCall),
        #[allow(missing_docs)]
        addCompressedKeyMaterials(addCompressedKeyMaterialsCall),
        #[allow(missing_docs)]
        compressedKeyMigrationKeygen(compressedKeyMigrationKeygenCall),
        #[allow(missing_docs)]
        crsgenRequest(crsgenRequestCall),
        #[allow(missing_docs)]
        crsgenResponse(crsgenResponseCall),
        #[allow(missing_docs)]
        eip712Domain(eip712DomainCall),
        #[allow(missing_docs)]
        getActiveCrsId(getActiveCrsIdCall),
        #[allow(missing_docs)]
        getActiveKeyId(getActiveKeyIdCall),
        #[allow(missing_docs)]
        getCompletedCrsIds(getCompletedCrsIdsCall),
        #[allow(missing_docs)]
        getCompletedKeyIds(getCompletedKeyIdsCall),
        #[allow(missing_docs)]
        getCompressedKeyCutoverSchedule(getCompressedKeyCutoverScheduleCall),
        #[allow(missing_docs)]
        getCompressedKeyMaterials(getCompressedKeyMaterialsCall),
        #[allow(missing_docs)]
        getConsensusTxSenders(getConsensusTxSendersCall),
        #[allow(missing_docs)]
        getCrsCounter(getCrsCounterCall),
        #[allow(missing_docs)]
        getCrsMaterials(getCrsMaterialsCall),
        #[allow(missing_docs)]
        getCrsParamsType(getCrsParamsTypeCall),
        #[allow(missing_docs)]
        getKeyCounter(getKeyCounterCall),
        #[allow(missing_docs)]
        getKeyInfo(getKeyInfoCall),
        #[allow(missing_docs)]
        getKeyMaterials(getKeyMaterialsCall),
        #[allow(missing_docs)]
        getKeyParamsType(getKeyParamsTypeCall),
        #[allow(missing_docs)]
        getVersion(getVersionCall),
        #[allow(missing_docs)]
        initializeFromEmptyProxy(initializeFromEmptyProxyCall),
        #[allow(missing_docs)]
        isRequestDone(isRequestDoneCall),
        #[allow(missing_docs)]
        keygen(keygenCall),
        #[allow(missing_docs)]
        keygenResponse(keygenResponseCall),
        #[allow(missing_docs)]
        prepKeygenResponse(prepKeygenResponseCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV3(reinitializeV3Call),
        #[allow(missing_docs)]
        scheduleCompressedKeyCutover(scheduleCompressedKeyCutoverCall),
        #[allow(missing_docs)]
        upgradeToAndCall(upgradeToAndCallCall),
    }
    #[automatically_derived]
    impl KMSGenerationCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 104u8, 7u8, 51u8],
            [13u8, 142u8, 110u8, 44u8],
            [22u8, 199u8, 19u8, 217u8],
            [23u8, 3u8, 198u8, 26u8],
            [25u8, 244u8, 246u8, 50u8],
            [57u8, 247u8, 56u8, 16u8],
            [58u8, 197u8, 0u8, 114u8],
            [60u8, 2u8, 248u8, 52u8],
            [61u8, 94u8, 199u8, 227u8],
            [69u8, 175u8, 38u8, 27u8],
            [70u8, 16u8, 255u8, 232u8],
            [79u8, 30u8, 242u8, 134u8],
            [82u8, 209u8, 144u8, 45u8],
            [88u8, 154u8, 219u8, 14u8],
            [98u8, 148u8, 244u8, 98u8],
            [98u8, 151u8, 135u8, 135u8],
            [118u8, 9u8, 21u8, 45u8],
            [132u8, 176u8, 25u8, 110u8],
            [144u8, 129u8, 254u8, 19u8],
            [147u8, 102u8, 8u8, 174u8],
            [173u8, 60u8, 177u8, 204u8],
            [186u8, 194u8, 43u8, 184u8],
            [186u8, 255u8, 33u8, 30u8],
            [188u8, 84u8, 4u8, 160u8],
            [193u8, 37u8, 44u8, 169u8],
            [194u8, 193u8, 250u8, 238u8],
            [197u8, 91u8, 135u8, 36u8],
            [202u8, 163u8, 103u8, 219u8],
            [213u8, 47u8, 16u8, 235u8],
            [218u8, 189u8, 115u8, 47u8],
            [228u8, 16u8, 17u8, 126u8],
            [231u8, 17u8, 201u8, 231u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationCalls {
        const NAME: &'static str = "KMSGenerationCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 32usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::UPGRADE_INTERFACE_VERSION(_) => {
                    <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::abortCrsgen(_) => {
                    <abortCrsgenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::abortKeygen(_) => {
                    <abortKeygenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addCompressedKeyMaterials(_) => {
                    <addCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::compressedKeyMigrationKeygen(_) => {
                    <compressedKeyMigrationKeygenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::crsgenRequest(_) => {
                    <crsgenRequestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::crsgenResponse(_) => {
                    <crsgenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveCrsId(_) => {
                    <getActiveCrsIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveKeyId(_) => {
                    <getActiveKeyIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCompletedCrsIds(_) => {
                    <getCompletedCrsIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCompletedKeyIds(_) => {
                    <getCompletedKeyIdsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCompressedKeyCutoverSchedule(_) => {
                    <getCompressedKeyCutoverScheduleCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCompressedKeyMaterials(_) => {
                    <getCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getConsensusTxSenders(_) => {
                    <getConsensusTxSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCrsCounter(_) => {
                    <getCrsCounterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCrsMaterials(_) => {
                    <getCrsMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCrsParamsType(_) => {
                    <getCrsParamsTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyCounter(_) => {
                    <getKeyCounterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyInfo(_) => {
                    <getKeyInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyMaterials(_) => {
                    <getKeyMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyParamsType(_) => {
                    <getKeyParamsTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getVersion(_) => {
                    <getVersionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeFromEmptyProxy(_) => {
                    <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRequestDone(_) => {
                    <isRequestDoneCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::keygen(_) => <keygenCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::keygenResponse(_) => {
                    <keygenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::prepKeygenResponse(_) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV3(_) => {
                    <reinitializeV3Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scheduleCompressedKeyCutover(_) => {
                    <scheduleCompressedKeyCutoverCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<KMSGenerationCalls>] = &[
                {
                    fn getKeyCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyCounterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyCounter)
                    }
                    getKeyCounter
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn getConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getConsensusTxSenders)
                    }
                    getConsensusTxSenders
                },
                {
                    fn abortCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortCrsgenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::abortCrsgen)
                    }
                    abortCrsgen
                },
                {
                    fn getKeyParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyParamsType)
                    }
                    getKeyParamsType
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getCrsCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsCounterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsCounter)
                    }
                    getCrsCounter
                },
                {
                    fn crsgenRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenRequestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenRequest)
                    }
                    crsgenRequest
                },
                {
                    fn isRequestDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <isRequestDoneCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::isRequestDone)
                    }
                    isRequestDone
                },
                {
                    fn getCrsParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsParamsType)
                    }
                    getCrsParamsType
                },
                {
                    fn keygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::keygenResponse)
                    }
                    keygenResponse
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn prepKeygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::prepKeygenResponse)
                    }
                    prepKeygenResponse
                },
                {
                    fn getKeyInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyInfo)
                    }
                    getKeyInfo
                },
                {
                    fn crsgenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenResponse)
                    }
                    crsgenResponse
                },
                {
                    fn getCompressedKeyCutoverSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompressedKeyCutoverScheduleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompressedKeyCutoverSchedule)
                    }
                    getCompressedKeyCutoverSchedule
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn compressedKeyMigrationKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <compressedKeyMigrationKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::compressedKeyMigrationKeygen)
                    }
                    compressedKeyMigrationKeygen
                },
                {
                    fn getKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterials)
                    }
                    getKeyMaterials
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV3)
                    }
                    reinitializeV3
                },
                {
                    fn getActiveCrsId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveCrsId)
                    }
                    getActiveCrsId
                },
                {
                    fn scheduleCompressedKeyCutover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <scheduleCompressedKeyCutoverCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::scheduleCompressedKeyCutover)
                    }
                    scheduleCompressedKeyCutover
                },
                {
                    fn addCompressedKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <addCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::addCompressedKeyMaterials)
                    }
                    addCompressedKeyMaterials
                },
                {
                    fn abortKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::abortKeygen)
                    }
                    abortKeygen
                },
                {
                    fn getCrsMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsMaterials)
                    }
                    getCrsMaterials
                },
                {
                    fn keygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KMSGenerationCalls::keygen)
                    }
                    keygen
                },
                {
                    fn getActiveKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveKeyId)
                    }
                    getActiveKeyId
                },
                {
                    fn getCompletedCrsIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompletedCrsIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompletedCrsIds)
                    }
                    getCompletedCrsIds
                },
                {
                    fn getCompletedKeyIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompletedKeyIdsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompletedKeyIds)
                    }
                    getCompletedKeyIds
                },
                {
                    fn getCompressedKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompressedKeyMaterials)
                    }
                    getCompressedKeyMaterials
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
            ) -> alloy_sol_types::Result<KMSGenerationCalls>] = &[
                {
                    fn getKeyCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyCounterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyCounter)
                    }
                    getKeyCounter
                },
                {
                    fn getVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getVersion)
                    }
                    getVersion
                },
                {
                    fn getConsensusTxSenders(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getConsensusTxSenders)
                    }
                    getConsensusTxSenders
                },
                {
                    fn abortCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortCrsgenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::abortCrsgen)
                    }
                    abortCrsgen
                },
                {
                    fn getKeyParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyParamsType)
                    }
                    getKeyParamsType
                },
                {
                    fn initializeFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <initializeFromEmptyProxyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::initializeFromEmptyProxy)
                    }
                    initializeFromEmptyProxy
                },
                {
                    fn getCrsCounter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsCounterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsCounter)
                    }
                    getCrsCounter
                },
                {
                    fn crsgenRequest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenRequestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenRequest)
                    }
                    crsgenRequest
                },
                {
                    fn isRequestDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <isRequestDoneCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::isRequestDone)
                    }
                    isRequestDone
                },
                {
                    fn getCrsParamsType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsParamsType)
                    }
                    getCrsParamsType
                },
                {
                    fn keygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::keygenResponse)
                    }
                    keygenResponse
                },
                {
                    fn upgradeToAndCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <upgradeToAndCallCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::upgradeToAndCall)
                    }
                    upgradeToAndCall
                },
                {
                    fn proxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <proxiableUUIDCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::proxiableUUID)
                    }
                    proxiableUUID
                },
                {
                    fn prepKeygenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::prepKeygenResponse)
                    }
                    prepKeygenResponse
                },
                {
                    fn getKeyInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyInfo)
                    }
                    getKeyInfo
                },
                {
                    fn crsgenResponse(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <crsgenResponseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::crsgenResponse)
                    }
                    crsgenResponse
                },
                {
                    fn getCompressedKeyCutoverSchedule(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompressedKeyCutoverScheduleCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompressedKeyCutoverSchedule)
                    }
                    getCompressedKeyCutoverSchedule
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn compressedKeyMigrationKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <compressedKeyMigrationKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::compressedKeyMigrationKeygen)
                    }
                    compressedKeyMigrationKeygen
                },
                {
                    fn getKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterials)
                    }
                    getKeyMaterials
                },
                {
                    fn UPGRADE_INTERFACE_VERSION(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <UPGRADE_INTERFACE_VERSIONCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::UPGRADE_INTERFACE_VERSION)
                    }
                    UPGRADE_INTERFACE_VERSION
                },
                {
                    fn reinitializeV3(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV3Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV3)
                    }
                    reinitializeV3
                },
                {
                    fn getActiveCrsId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveCrsId)
                    }
                    getActiveCrsId
                },
                {
                    fn scheduleCompressedKeyCutover(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <scheduleCompressedKeyCutoverCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::scheduleCompressedKeyCutover)
                    }
                    scheduleCompressedKeyCutover
                },
                {
                    fn addCompressedKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <addCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::addCompressedKeyMaterials)
                    }
                    addCompressedKeyMaterials
                },
                {
                    fn abortKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <abortKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::abortKeygen)
                    }
                    abortKeygen
                },
                {
                    fn getCrsMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCrsMaterials)
                    }
                    getCrsMaterials
                },
                {
                    fn keygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <keygenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::keygen)
                    }
                    keygen
                },
                {
                    fn getActiveKeyId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getActiveKeyId)
                    }
                    getActiveKeyId
                },
                {
                    fn getCompletedCrsIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompletedCrsIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompletedCrsIds)
                    }
                    getCompletedCrsIds
                },
                {
                    fn getCompletedKeyIds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompletedKeyIdsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompletedKeyIds)
                    }
                    getCompletedKeyIds
                },
                {
                    fn getCompressedKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getCompressedKeyMaterials)
                    }
                    getCompressedKeyMaterials
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
                Self::abortCrsgen(inner) => {
                    <abortCrsgenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::abortKeygen(inner) => {
                    <abortKeygenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addCompressedKeyMaterials(inner) => {
                    <addCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::compressedKeyMigrationKeygen(inner) => {
                    <compressedKeyMigrationKeygenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::crsgenRequest(inner) => {
                    <crsgenRequestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::crsgenResponse(inner) => {
                    <crsgenResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveCrsId(inner) => {
                    <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveKeyId(inner) => {
                    <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCompletedCrsIds(inner) => {
                    <getCompletedCrsIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCompletedKeyIds(inner) => {
                    <getCompletedKeyIdsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCompressedKeyCutoverSchedule(inner) => {
                    <getCompressedKeyCutoverScheduleCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCompressedKeyMaterials(inner) => {
                    <getCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getConsensusTxSenders(inner) => {
                    <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCrsCounter(inner) => {
                    <getCrsCounterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCrsMaterials(inner) => {
                    <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCrsParamsType(inner) => {
                    <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyCounter(inner) => {
                    <getKeyCounterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyInfo(inner) => {
                    <getKeyInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getKeyMaterials(inner) => {
                    <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyParamsType(inner) => {
                    <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isRequestDone(inner) => {
                    <isRequestDoneCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::keygen(inner) => {
                    <keygenCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::keygenResponse(inner) => {
                    <keygenResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::prepKeygenResponse(inner) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::scheduleCompressedKeyCutover(inner) => {
                    <scheduleCompressedKeyCutoverCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::abortCrsgen(inner) => {
                    <abortCrsgenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::abortKeygen(inner) => {
                    <abortKeygenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addCompressedKeyMaterials(inner) => {
                    <addCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::compressedKeyMigrationKeygen(inner) => {
                    <compressedKeyMigrationKeygenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::crsgenRequest(inner) => {
                    <crsgenRequestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::crsgenResponse(inner) => {
                    <crsgenResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getActiveCrsId(inner) => {
                    <getActiveCrsIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveKeyId(inner) => {
                    <getActiveKeyIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCompletedCrsIds(inner) => {
                    <getCompletedCrsIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCompletedKeyIds(inner) => {
                    <getCompletedKeyIdsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCompressedKeyCutoverSchedule(inner) => {
                    <getCompressedKeyCutoverScheduleCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCompressedKeyMaterials(inner) => {
                    <getCompressedKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getConsensusTxSenders(inner) => {
                    <getConsensusTxSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCrsCounter(inner) => {
                    <getCrsCounterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCrsMaterials(inner) => {
                    <getCrsMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCrsParamsType(inner) => {
                    <getCrsParamsTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyCounter(inner) => {
                    <getKeyCounterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyInfo(inner) => {
                    <getKeyInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyMaterials(inner) => {
                    <getKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyParamsType(inner) => {
                    <getKeyParamsTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isRequestDone(inner) => {
                    <isRequestDoneCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::keygen(inner) => {
                    <keygenCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::keygenResponse(inner) => {
                    <keygenResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::prepKeygenResponse(inner) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::scheduleCompressedKeyCutover(inner) => {
                    <scheduleCompressedKeyCutoverCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`KMSGeneration`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum KMSGenerationErrors {
        #[allow(missing_docs)]
        AbortCrsgenAlreadyDone(AbortCrsgenAlreadyDone),
        #[allow(missing_docs)]
        AbortCrsgenInvalidId(AbortCrsgenInvalidId),
        #[allow(missing_docs)]
        AbortKeygenAlreadyDone(AbortKeygenAlreadyDone),
        #[allow(missing_docs)]
        AbortKeygenInvalidId(AbortKeygenInvalidId),
        #[allow(missing_docs)]
        AddressEmptyCode(AddressEmptyCode),
        #[allow(missing_docs)]
        CompressedKeyCutoverAlreadyScheduled(CompressedKeyCutoverAlreadyScheduled),
        #[allow(missing_docs)]
        CompressedKeyMaterialsAlreadyAdded(CompressedKeyMaterialsAlreadyAdded),
        #[allow(missing_docs)]
        CompressedKeyMaterialsNotAdded(CompressedKeyMaterialsNotAdded),
        #[allow(missing_docs)]
        CrsAborted(CrsAborted),
        #[allow(missing_docs)]
        CrsNotGenerated(CrsNotGenerated),
        #[allow(missing_docs)]
        CrsgenNotRequested(CrsgenNotRequested),
        #[allow(missing_docs)]
        CrsgenOngoing(CrsgenOngoing),
        #[allow(missing_docs)]
        DeserializingExtraDataFail(DeserializingExtraDataFail),
        #[allow(missing_docs)]
        DuplicateCutoverChainId(DuplicateCutoverChainId),
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
        EmptyHostChainCutovers(EmptyHostChainCutovers),
        #[allow(missing_docs)]
        EmptyKeyDigests(EmptyKeyDigests),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidCutoverBlock(InvalidCutoverBlock),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        KeyAborted(KeyAborted),
        #[allow(missing_docs)]
        KeyManagementRequestPending(KeyManagementRequestPending),
        #[allow(missing_docs)]
        KeyNotGenerated(KeyNotGenerated),
        #[allow(missing_docs)]
        KeygenNotRequested(KeygenNotRequested),
        #[allow(missing_docs)]
        KeygenOngoing(KeygenOngoing),
        #[allow(missing_docs)]
        KmsAlreadySignedForCrsgen(KmsAlreadySignedForCrsgen),
        #[allow(missing_docs)]
        KmsAlreadySignedForKeygen(KmsAlreadySignedForKeygen),
        #[allow(missing_docs)]
        KmsAlreadySignedForPrepKeygen(KmsAlreadySignedForPrepKeygen),
        #[allow(missing_docs)]
        KmsSignerDoesNotMatchTxSender(KmsSignerDoesNotMatchTxSender),
        #[allow(missing_docs)]
        NotHostOwner(NotHostOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
        #[allow(missing_docs)]
        NotKmsSigner(NotKmsSigner),
        #[allow(missing_docs)]
        NotKmsTxSender(NotKmsTxSender),
        #[allow(missing_docs)]
        PrepKeygenNotRequested(PrepKeygenNotRequested),
        #[allow(missing_docs)]
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        #[allow(missing_docs)]
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        #[allow(missing_docs)]
        UnsupportedExtraDataVersion(UnsupportedExtraDataVersion),
        #[allow(missing_docs)]
        WrongKeygenResponseEndpoint(WrongKeygenResponseEndpoint),
    }
    #[automatically_derived]
    impl KMSGenerationErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 26u8, 198u8, 29u8],
            [10u8, 183u8, 246u8, 135u8],
            [13u8, 134u8, 245u8, 33u8],
            [33u8, 57u8, 204u8, 44u8],
            [33u8, 191u8, 218u8, 16u8],
            [42u8, 124u8, 110u8, 246u8],
            [51u8, 202u8, 31u8, 227u8],
            [59u8, 133u8, 61u8, 168u8],
            [64u8, 98u8, 146u8, 182u8],
            [72u8, 215u8, 135u8, 246u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [111u8, 188u8, 221u8, 43u8],
            [131u8, 241u8, 131u8, 53u8],
            [132u8, 222u8, 19u8, 49u8],
            [136u8, 199u8, 112u8, 244u8],
            [139u8, 36u8, 139u8, 96u8],
            [141u8, 140u8, 148u8, 10u8],
            [146u8, 120u8, 155u8, 103u8],
            [148u8, 10u8, 214u8, 214u8],
            [148u8, 214u8, 150u8, 32u8],
            [152u8, 251u8, 149u8, 125u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [173u8, 250u8, 185u8, 4u8],
            [174u8, 232u8, 99u8, 35u8],
            [179u8, 152u8, 151u8, 159u8],
            [180u8, 72u8, 22u8, 182u8],
            [203u8, 233u8, 38u8, 86u8],
            [213u8, 253u8, 60u8, 215u8],
            [214u8, 189u8, 162u8, 117u8],
            [215u8, 139u8, 206u8, 12u8],
            [215u8, 230u8, 188u8, 248u8],
            [218u8, 50u8, 208u8, 15u8],
            [223u8, 13u8, 181u8, 251u8],
            [224u8, 124u8, 141u8, 186u8],
            [230u8, 249u8, 8u8, 59u8],
            [246u8, 69u8, 238u8, 223u8],
            [249u8, 22u8, 238u8, 66u8],
            [249u8, 46u8, 232u8, 169u8],
            [252u8, 230u8, 152u8, 247u8],
            [252u8, 242u8, 219u8, 122u8],
            [252u8, 245u8, 166u8, 233u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationErrors {
        const NAME: &'static str = "KMSGenerationErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 43usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AbortCrsgenAlreadyDone(_) => {
                    <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AbortCrsgenInvalidId(_) => {
                    <AbortCrsgenInvalidId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AbortKeygenAlreadyDone(_) => {
                    <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AbortKeygenInvalidId(_) => {
                    <AbortKeygenInvalidId as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AddressEmptyCode(_) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CompressedKeyCutoverAlreadyScheduled(_) => {
                    <CompressedKeyCutoverAlreadyScheduled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CompressedKeyMaterialsAlreadyAdded(_) => {
                    <CompressedKeyMaterialsAlreadyAdded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CompressedKeyMaterialsNotAdded(_) => {
                    <CompressedKeyMaterialsNotAdded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsAborted(_) => {
                    <CrsAborted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsNotGenerated(_) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsgenNotRequested(_) => {
                    <CrsgenNotRequested as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CrsgenOngoing(_) => {
                    <CrsgenOngoing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DeserializingExtraDataFail(_) => {
                    <DeserializingExtraDataFail as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DuplicateCutoverChainId(_) => {
                    <DuplicateCutoverChainId as alloy_sol_types::SolError>::SELECTOR
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
                Self::EmptyHostChainCutovers(_) => {
                    <EmptyHostChainCutovers as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyKeyDigests(_) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCutoverBlock(_) => {
                    <InvalidCutoverBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidInitialization(_) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyAborted(_) => {
                    <KeyAborted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyManagementRequestPending(_) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyNotGenerated(_) => {
                    <KeyNotGenerated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeygenNotRequested(_) => {
                    <KeygenNotRequested as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeygenOngoing(_) => {
                    <KeygenOngoing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForCrsgen(_) => {
                    <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForKeygen(_) => {
                    <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsAlreadySignedForPrepKeygen(_) => {
                    <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KmsSignerDoesNotMatchTxSender(_) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::SELECTOR
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
                Self::NotKmsSigner(_) => {
                    <NotKmsSigner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotKmsTxSender(_) => {
                    <NotKmsTxSender as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PrepKeygenNotRequested(_) => {
                    <PrepKeygenNotRequested as alloy_sol_types::SolError>::SELECTOR
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
                Self::WrongKeygenResponseEndpoint(_) => {
                    <WrongKeygenResponseEndpoint as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<KMSGenerationErrors>] = &[
                {
                    fn CrsgenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenOngoing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenOngoing)
                    }
                    CrsgenOngoing
                },
                {
                    fn PrepKeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::PrepKeygenNotRequested)
                    }
                    PrepKeygenNotRequested
                },
                {
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
                },
                {
                    fn UnsupportedExtraDataVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UnsupportedExtraDataVersion)
                    }
                    UnsupportedExtraDataVersion
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::NotKmsSigner)
                    }
                    NotKmsSigner
                },
                {
                    fn KmsAlreadySignedForPrepKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForPrepKeygen)
                    }
                    KmsAlreadySignedForPrepKeygen
                },
                {
                    fn KeygenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenOngoing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenOngoing)
                    }
                    KeygenOngoing
                },
                {
                    fn CompressedKeyCutoverAlreadyScheduled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CompressedKeyCutoverAlreadyScheduled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                KMSGenerationErrors::CompressedKeyCutoverAlreadyScheduled,
                            )
                    }
                    CompressedKeyCutoverAlreadyScheduled
                },
                {
                    fn InvalidCutoverBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidCutoverBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidCutoverBlock)
                    }
                    InvalidCutoverBlock
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn KeyManagementRequestPending(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyManagementRequestPending)
                    }
                    KeyManagementRequestPending
                },
                {
                    fn KeyAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyAborted as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::KeyAborted)
                    }
                    KeyAborted
                },
                {
                    fn KeyNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyNotGenerated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyNotGenerated)
                    }
                    KeyNotGenerated
                },
                {
                    fn CompressedKeyMaterialsAlreadyAdded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CompressedKeyMaterialsAlreadyAdded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CompressedKeyMaterialsAlreadyAdded)
                    }
                    CompressedKeyMaterialsAlreadyAdded
                },
                {
                    fn DeserializingExtraDataFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::DeserializingExtraDataFail)
                    }
                    DeserializingExtraDataFail
                },
                {
                    fn CrsgenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenNotRequested)
                    }
                    CrsgenNotRequested
                },
                {
                    fn AbortKeygenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenAlreadyDone)
                    }
                    AbortKeygenAlreadyDone
                },
                {
                    fn DuplicateCutoverChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <DuplicateCutoverChainId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::DuplicateCutoverChainId)
                    }
                    DuplicateCutoverChainId
                },
                {
                    fn EmptyHostChainCutovers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <EmptyHostChainCutovers as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::EmptyHostChainCutovers)
                    }
                    EmptyHostChainCutovers
                },
                {
                    fn KmsAlreadySignedForKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForKeygen)
                    }
                    KmsAlreadySignedForKeygen
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn KeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenNotRequested)
                    }
                    KeygenNotRequested
                },
                {
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn WrongKeygenResponseEndpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <WrongKeygenResponseEndpoint as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::WrongKeygenResponseEndpoint)
                    }
                    WrongKeygenResponseEndpoint
                },
                {
                    fn AbortCrsgenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenInvalidId)
                    }
                    AbortCrsgenInvalidId
                },
                {
                    fn CrsAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsAborted as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::CrsAborted)
                    }
                    CrsAborted
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KMSGenerationErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn CrsNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsNotGenerated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsNotGenerated)
                    }
                    CrsNotGenerated
                },
                {
                    fn AbortCrsgenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenAlreadyDone)
                    }
                    AbortCrsgenAlreadyDone
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn EmptyKeyDigests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <EmptyKeyDigests as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::EmptyKeyDigests)
                    }
                    EmptyKeyDigests
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn CompressedKeyMaterialsNotAdded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CompressedKeyMaterialsNotAdded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::CompressedKeyMaterialsNotAdded)
                    }
                    CompressedKeyMaterialsNotAdded
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
                {
                    fn AbortKeygenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenInvalidId)
                    }
                    AbortKeygenInvalidId
                },
                {
                    fn KmsAlreadySignedForCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForCrsgen)
                    }
                    KmsAlreadySignedForCrsgen
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
            ) -> alloy_sol_types::Result<KMSGenerationErrors>] = &[
                {
                    fn CrsgenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenOngoing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenOngoing)
                    }
                    CrsgenOngoing
                },
                {
                    fn PrepKeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::PrepKeygenNotRequested)
                    }
                    PrepKeygenNotRequested
                },
                {
                    fn KmsSignerDoesNotMatchTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsSignerDoesNotMatchTxSender)
                    }
                    KmsSignerDoesNotMatchTxSender
                },
                {
                    fn UnsupportedExtraDataVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UnsupportedExtraDataVersion as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UnsupportedExtraDataVersion)
                    }
                    UnsupportedExtraDataVersion
                },
                {
                    fn NotHostOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotHostOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotHostOwner)
                    }
                    NotHostOwner
                },
                {
                    fn NotKmsSigner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsSigner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotKmsSigner)
                    }
                    NotKmsSigner
                },
                {
                    fn KmsAlreadySignedForPrepKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForPrepKeygen)
                    }
                    KmsAlreadySignedForPrepKeygen
                },
                {
                    fn KeygenOngoing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenOngoing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenOngoing)
                    }
                    KeygenOngoing
                },
                {
                    fn CompressedKeyCutoverAlreadyScheduled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CompressedKeyCutoverAlreadyScheduled as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                KMSGenerationErrors::CompressedKeyCutoverAlreadyScheduled,
                            )
                    }
                    CompressedKeyCutoverAlreadyScheduled
                },
                {
                    fn InvalidCutoverBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidCutoverBlock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidCutoverBlock)
                    }
                    InvalidCutoverBlock
                },
                {
                    fn ERC1967InvalidImplementation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967InvalidImplementation as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967InvalidImplementation)
                    }
                    ERC1967InvalidImplementation
                },
                {
                    fn NotInitializingFromEmptyProxy(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializingFromEmptyProxy as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializingFromEmptyProxy)
                    }
                    NotInitializingFromEmptyProxy
                },
                {
                    fn KeyManagementRequestPending(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyManagementRequestPending)
                    }
                    KeyManagementRequestPending
                },
                {
                    fn KeyAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyAborted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyAborted)
                    }
                    KeyAborted
                },
                {
                    fn KeyNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyNotGenerated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyNotGenerated)
                    }
                    KeyNotGenerated
                },
                {
                    fn CompressedKeyMaterialsAlreadyAdded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CompressedKeyMaterialsAlreadyAdded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CompressedKeyMaterialsAlreadyAdded)
                    }
                    CompressedKeyMaterialsAlreadyAdded
                },
                {
                    fn DeserializingExtraDataFail(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::DeserializingExtraDataFail)
                    }
                    DeserializingExtraDataFail
                },
                {
                    fn CrsgenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsgenNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsgenNotRequested)
                    }
                    CrsgenNotRequested
                },
                {
                    fn AbortKeygenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenAlreadyDone)
                    }
                    AbortKeygenAlreadyDone
                },
                {
                    fn DuplicateCutoverChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <DuplicateCutoverChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::DuplicateCutoverChainId)
                    }
                    DuplicateCutoverChainId
                },
                {
                    fn EmptyHostChainCutovers(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <EmptyHostChainCutovers as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::EmptyHostChainCutovers)
                    }
                    EmptyHostChainCutovers
                },
                {
                    fn KmsAlreadySignedForKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForKeygen)
                    }
                    KmsAlreadySignedForKeygen
                },
                {
                    fn AddressEmptyCode(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AddressEmptyCode as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AddressEmptyCode)
                    }
                    AddressEmptyCode
                },
                {
                    fn UUPSUnsupportedProxiableUUID(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnsupportedProxiableUUID as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnsupportedProxiableUUID)
                    }
                    UUPSUnsupportedProxiableUUID
                },
                {
                    fn KeygenNotRequested(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeygenNotRequested as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeygenNotRequested)
                    }
                    KeygenNotRequested
                },
                {
                    fn NotKmsTxSender(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotKmsTxSender as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotKmsTxSender)
                    }
                    NotKmsTxSender
                },
                {
                    fn ERC1967NonPayable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ERC1967NonPayable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ERC1967NonPayable)
                    }
                    ERC1967NonPayable
                },
                {
                    fn WrongKeygenResponseEndpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <WrongKeygenResponseEndpoint as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::WrongKeygenResponseEndpoint)
                    }
                    WrongKeygenResponseEndpoint
                },
                {
                    fn AbortCrsgenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenInvalidId)
                    }
                    AbortCrsgenInvalidId
                },
                {
                    fn CrsAborted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsAborted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsAborted)
                    }
                    CrsAborted
                },
                {
                    fn FailedCall(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <FailedCall as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::FailedCall)
                    }
                    FailedCall
                },
                {
                    fn ECDSAInvalidSignatureS(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureS as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureS)
                    }
                    ECDSAInvalidSignatureS
                },
                {
                    fn NotInitializing(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <NotInitializing as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::NotInitializing)
                    }
                    NotInitializing
                },
                {
                    fn CrsNotGenerated(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CrsNotGenerated as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CrsNotGenerated)
                    }
                    CrsNotGenerated
                },
                {
                    fn AbortCrsgenAlreadyDone(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortCrsgenAlreadyDone)
                    }
                    AbortCrsgenAlreadyDone
                },
                {
                    fn UUPSUnauthorizedCallContext(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <UUPSUnauthorizedCallContext as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::UUPSUnauthorizedCallContext)
                    }
                    UUPSUnauthorizedCallContext
                },
                {
                    fn EmptyKeyDigests(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <EmptyKeyDigests as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::EmptyKeyDigests)
                    }
                    EmptyKeyDigests
                },
                {
                    fn ECDSAInvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignature)
                    }
                    ECDSAInvalidSignature
                },
                {
                    fn CompressedKeyMaterialsNotAdded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <CompressedKeyMaterialsNotAdded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::CompressedKeyMaterialsNotAdded)
                    }
                    CompressedKeyMaterialsNotAdded
                },
                {
                    fn InvalidInitialization(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <InvalidInitialization as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::InvalidInitialization)
                    }
                    InvalidInitialization
                },
                {
                    fn ECDSAInvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <ECDSAInvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::ECDSAInvalidSignatureLength)
                    }
                    ECDSAInvalidSignatureLength
                },
                {
                    fn AbortKeygenInvalidId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::AbortKeygenInvalidId)
                    }
                    AbortKeygenInvalidId
                },
                {
                    fn KmsAlreadySignedForCrsgen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KmsAlreadySignedForCrsgen)
                    }
                    KmsAlreadySignedForCrsgen
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
                Self::AbortCrsgenAlreadyDone(inner) => {
                    <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AbortCrsgenInvalidId(inner) => {
                    <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AbortKeygenAlreadyDone(inner) => {
                    <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AbortKeygenInvalidId(inner) => {
                    <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CompressedKeyCutoverAlreadyScheduled(inner) => {
                    <CompressedKeyCutoverAlreadyScheduled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CompressedKeyMaterialsAlreadyAdded(inner) => {
                    <CompressedKeyMaterialsAlreadyAdded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CompressedKeyMaterialsNotAdded(inner) => {
                    <CompressedKeyMaterialsNotAdded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CrsAborted(inner) => {
                    <CrsAborted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CrsNotGenerated(inner) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CrsgenNotRequested(inner) => {
                    <CrsgenNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CrsgenOngoing(inner) => {
                    <CrsgenOngoing as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::DeserializingExtraDataFail(inner) => {
                    <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DuplicateCutoverChainId(inner) => {
                    <DuplicateCutoverChainId as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyHostChainCutovers(inner) => {
                    <EmptyHostChainCutovers as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyKeyDigests(inner) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidCutoverBlock(inner) => {
                    <InvalidCutoverBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidInitialization(inner) => {
                    <InvalidInitialization as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeyAborted(inner) => {
                    <KeyAborted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::KeyManagementRequestPending(inner) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeyNotGenerated(inner) => {
                    <KeyNotGenerated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeygenNotRequested(inner) => {
                    <KeygenNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeygenOngoing(inner) => {
                    <KeygenOngoing as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::KmsAlreadySignedForCrsgen(inner) => {
                    <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsAlreadySignedForKeygen(inner) => {
                    <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsAlreadySignedForPrepKeygen(inner) => {
                    <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KmsSignerDoesNotMatchTxSender(inner) => {
                    <KmsSignerDoesNotMatchTxSender as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotKmsSigner(inner) => {
                    <NotKmsSigner as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotKmsTxSender(inner) => {
                    <NotKmsTxSender as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PrepKeygenNotRequested(inner) => {
                    <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::WrongKeygenResponseEndpoint(inner) => {
                    <WrongKeygenResponseEndpoint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AbortCrsgenAlreadyDone(inner) => {
                    <AbortCrsgenAlreadyDone as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AbortCrsgenInvalidId(inner) => {
                    <AbortCrsgenInvalidId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AbortKeygenAlreadyDone(inner) => {
                    <AbortKeygenAlreadyDone as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AbortKeygenInvalidId(inner) => {
                    <AbortKeygenInvalidId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AddressEmptyCode(inner) => {
                    <AddressEmptyCode as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CompressedKeyCutoverAlreadyScheduled(inner) => {
                    <CompressedKeyCutoverAlreadyScheduled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CompressedKeyMaterialsAlreadyAdded(inner) => {
                    <CompressedKeyMaterialsAlreadyAdded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CompressedKeyMaterialsNotAdded(inner) => {
                    <CompressedKeyMaterialsNotAdded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CrsAborted(inner) => {
                    <CrsAborted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::CrsNotGenerated(inner) => {
                    <CrsNotGenerated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CrsgenNotRequested(inner) => {
                    <CrsgenNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CrsgenOngoing(inner) => {
                    <CrsgenOngoing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DeserializingExtraDataFail(inner) => {
                    <DeserializingExtraDataFail as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DuplicateCutoverChainId(inner) => {
                    <DuplicateCutoverChainId as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::EmptyHostChainCutovers(inner) => {
                    <EmptyHostChainCutovers as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyKeyDigests(inner) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidCutoverBlock(inner) => {
                    <InvalidCutoverBlock as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::KeyAborted(inner) => {
                    <KeyAborted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::KeyManagementRequestPending(inner) => {
                    <KeyManagementRequestPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeyNotGenerated(inner) => {
                    <KeyNotGenerated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeygenNotRequested(inner) => {
                    <KeygenNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeygenOngoing(inner) => {
                    <KeygenOngoing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsAlreadySignedForCrsgen(inner) => {
                    <KmsAlreadySignedForCrsgen as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsAlreadySignedForKeygen(inner) => {
                    <KmsAlreadySignedForKeygen as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KmsAlreadySignedForPrepKeygen(inner) => {
                    <KmsAlreadySignedForPrepKeygen as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::PrepKeygenNotRequested(inner) => {
                    <PrepKeygenNotRequested as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::WrongKeygenResponseEndpoint(inner) => {
                    <WrongKeygenResponseEndpoint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`KMSGeneration`](self) events.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive()]
    pub enum KMSGenerationEvents {
        #[allow(missing_docs)]
        AbortCrsgen(AbortCrsgen),
        #[allow(missing_docs)]
        AbortKeygen(AbortKeygen),
        #[allow(missing_docs)]
        ActivateCrs(ActivateCrs),
        #[allow(missing_docs)]
        ActivateKey(ActivateKey),
        #[allow(missing_docs)]
        CompressedKeyCutoverScheduled(CompressedKeyCutoverScheduled),
        #[allow(missing_docs)]
        CompressedKeyMaterialAdded(CompressedKeyMaterialAdded),
        #[allow(missing_docs)]
        CompressedKeyMaterialResponse(CompressedKeyMaterialResponse),
        #[allow(missing_docs)]
        CompressedKeyMigrationKeygenRequest(CompressedKeyMigrationKeygenRequest),
        #[allow(missing_docs)]
        CompressedKeyMigrationPrepKeygenRequest(CompressedKeyMigrationPrepKeygenRequest),
        #[allow(missing_docs)]
        CrsgenRequest(CrsgenRequest),
        #[allow(missing_docs)]
        CrsgenResponse(CrsgenResponse),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KeygenRequest(KeygenRequest),
        #[allow(missing_docs)]
        KeygenResponse(KeygenResponse),
        #[allow(missing_docs)]
        PrepKeygenRequest(PrepKeygenRequest),
        #[allow(missing_docs)]
        PrepKeygenResponse(PrepKeygenResponse),
        #[allow(missing_docs)]
        Upgraded(Upgraded),
    }
    #[automatically_derived]
    impl KMSGenerationEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                3u8, 200u8, 138u8, 187u8, 99u8, 203u8, 26u8, 51u8, 203u8, 161u8, 227u8,
                115u8, 147u8, 231u8, 252u8, 1u8, 242u8, 3u8, 187u8, 3u8, 123u8, 146u8,
                245u8, 239u8, 132u8, 213u8, 46u8, 92u8, 99u8, 172u8, 123u8, 231u8,
            ],
            [
                10u8, 99u8, 135u8, 201u8, 234u8, 54u8, 40u8, 184u8, 138u8, 99u8, 59u8,
                180u8, 243u8, 177u8, 81u8, 119u8, 15u8, 112u8, 8u8, 81u8, 23u8, 161u8,
                95u8, 155u8, 243u8, 120u8, 124u8, 218u8, 83u8, 241u8, 61u8, 49u8,
            ],
            [
                34u8, 88u8, 183u8, 63u8, 174u8, 211u8, 63u8, 178u8, 226u8, 234u8, 69u8,
                68u8, 3u8, 190u8, 249u8, 116u8, 146u8, 12u8, 175u8, 104u8, 42u8, 179u8,
                167u8, 35u8, 72u8, 79u8, 207u8, 103u8, 85u8, 59u8, 22u8, 162u8,
            ],
            [
                40u8, 105u8, 197u8, 19u8, 34u8, 233u8, 13u8, 44u8, 226u8, 223u8, 229u8,
                94u8, 194u8, 157u8, 24u8, 15u8, 54u8, 191u8, 250u8, 21u8, 180u8, 52u8,
                252u8, 10u8, 141u8, 208u8, 229u8, 211u8, 206u8, 60u8, 23u8, 76u8,
            ],
            [
                42u8, 254u8, 100u8, 251u8, 58u8, 253u8, 232u8, 226u8, 103u8, 138u8,
                234u8, 132u8, 207u8, 54u8, 34u8, 63u8, 51u8, 14u8, 47u8, 177u8, 40u8,
                109u8, 55u8, 174u8, 213u8, 115u8, 171u8, 156u8, 209u8, 219u8, 71u8, 199u8,
            ],
            [
                43u8, 8u8, 123u8, 136u8, 75u8, 53u8, 168u8, 29u8, 118u8, 157u8, 26u8,
                30u8, 9u8, 40u8, 128u8, 241u8, 218u8, 86u8, 222u8, 150u8, 78u8, 75u8,
                51u8, 158u8, 171u8, 203u8, 31u8, 69u8, 245u8, 254u8, 50u8, 100u8,
            ],
            [
                56u8, 79u8, 144u8, 254u8, 251u8, 207u8, 170u8, 104u8, 242u8, 46u8, 0u8,
                9u8, 74u8, 234u8, 165u8, 43u8, 43u8, 198u8, 147u8, 147u8, 109u8, 44u8,
                225u8, 175u8, 237u8, 18u8, 18u8, 82u8, 11u8, 89u8, 181u8, 142u8,
            ],
            [
                58u8, 17u8, 97u8, 32u8, 204u8, 165u8, 212u8, 240u8, 115u8, 204u8, 31u8,
                195u8, 31u8, 242u8, 97u8, 51u8, 171u8, 123u8, 4u8, 153u8, 242u8, 113u8,
                47u8, 160u8, 16u8, 2u8, 59u8, 135u8, 213u8, 161u8, 249u8, 238u8,
            ],
            [
                76u8, 113u8, 92u8, 87u8, 52u8, 206u8, 92u8, 24u8, 201u8, 193u8, 46u8,
                132u8, 150u8, 229u8, 61u8, 42u8, 101u8, 241u8, 236u8, 56u8, 29u8, 71u8,
                105u8, 87u8, 240u8, 245u8, 150u8, 179u8, 100u8, 165u8, 155u8, 12u8,
            ],
            [
                120u8, 206u8, 195u8, 79u8, 210u8, 113u8, 91u8, 147u8, 94u8, 74u8, 244u8,
                174u8, 105u8, 95u8, 117u8, 162u8, 197u8, 124u8, 189u8, 150u8, 85u8,
                241u8, 231u8, 227u8, 204u8, 134u8, 201u8, 124u8, 231u8, 159u8, 162u8,
                159u8,
            ],
            [
                123u8, 241u8, 180u8, 44u8, 16u8, 233u8, 73u8, 124u8, 135u8, 150u8, 32u8,
                197u8, 183u8, 175u8, 206u8, 209u8, 11u8, 218u8, 23u8, 216u8, 201u8, 11u8,
                34u8, 240u8, 227u8, 188u8, 107u8, 47u8, 214u8, 206u8, 208u8, 189u8,
            ],
            [
                128u8, 235u8, 194u8, 164u8, 225u8, 131u8, 0u8, 15u8, 104u8, 55u8, 250u8,
                177u8, 227u8, 105u8, 112u8, 232u8, 188u8, 74u8, 27u8, 25u8, 34u8, 48u8,
                84u8, 195u8, 39u8, 105u8, 219u8, 102u8, 58u8, 76u8, 227u8, 70u8,
            ],
            [
                140u8, 240u8, 21u8, 19u8, 147u8, 248u8, 79u8, 214u8, 148u8, 197u8, 227u8,
                21u8, 203u8, 116u8, 204u8, 5u8, 178u8, 71u8, 222u8, 10u8, 69u8, 79u8,
                217u8, 233u8, 18u8, 156u8, 102u8, 30u8, 253u8, 249u8, 64u8, 29u8,
            ],
            [
                176u8, 124u8, 90u8, 27u8, 37u8, 223u8, 67u8, 191u8, 230u8, 215u8, 2u8,
                186u8, 192u8, 108u8, 61u8, 110u8, 21u8, 26u8, 113u8, 218u8, 64u8, 106u8,
                108u8, 186u8, 7u8, 255u8, 164u8, 23u8, 0u8, 220u8, 171u8, 225u8,
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
                235u8, 133u8, 194u8, 109u8, 188u8, 173u8, 70u8, 184u8, 10u8, 104u8,
                160u8, 242u8, 76u8, 206u8, 124u8, 44u8, 144u8, 240u8, 161u8, 250u8,
                222u8, 216u8, 65u8, 132u8, 19u8, 136u8, 57u8, 252u8, 158u8, 128u8, 162u8,
                91u8,
            ],
            [
                251u8, 245u8, 39u8, 72u8, 16u8, 185u8, 79u8, 134u8, 151u8, 12u8, 17u8,
                71u8, 232u8, 255u8, 174u8, 190u8, 210u8, 70u8, 238u8, 151u8, 119u8,
                214u8, 149u8, 166u8, 144u8, 4u8, 220u8, 98u8, 86u8, 209u8, 254u8, 145u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for KMSGenerationEvents {
        const NAME: &'static str = "KMSGenerationEvents";
        const COUNT: usize = 18usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AbortCrsgen as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AbortCrsgen as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::AbortCrsgen)
                }
                Some(<AbortKeygen as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AbortKeygen as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::AbortKeygen)
                }
                Some(<ActivateCrs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActivateCrs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActivateCrs)
                }
                Some(<ActivateKey as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ActivateKey as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::ActivateKey)
                }
                Some(
                    <CompressedKeyCutoverScheduled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CompressedKeyCutoverScheduled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CompressedKeyCutoverScheduled)
                }
                Some(
                    <CompressedKeyMaterialAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CompressedKeyMaterialAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CompressedKeyMaterialAdded)
                }
                Some(
                    <CompressedKeyMaterialResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CompressedKeyMaterialResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CompressedKeyMaterialResponse)
                }
                Some(
                    <CompressedKeyMigrationKeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CompressedKeyMigrationKeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CompressedKeyMigrationKeygenRequest)
                }
                Some(
                    <CompressedKeyMigrationPrepKeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CompressedKeyMigrationPrepKeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CompressedKeyMigrationPrepKeygenRequest)
                }
                Some(<CrsgenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CrsgenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CrsgenRequest)
                }
                Some(<CrsgenResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CrsgenResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::CrsgenResponse)
                }
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
                Some(<KeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeygenRequest)
                }
                Some(<KeygenResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeygenResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeygenResponse)
                }
                Some(
                    <PrepKeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PrepKeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PrepKeygenRequest)
                }
                Some(
                    <PrepKeygenResponse as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PrepKeygenResponse as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::PrepKeygenResponse)
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
    impl alloy_sol_types::private::IntoLogData for KMSGenerationEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AbortCrsgen(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AbortKeygen(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ActivateCrs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ActivateKey(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CompressedKeyCutoverScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CompressedKeyMaterialAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CompressedKeyMaterialResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CompressedKeyMigrationKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CompressedKeyMigrationPrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CrsgenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CrsgenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PrepKeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AbortCrsgen(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AbortKeygen(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ActivateCrs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ActivateKey(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CompressedKeyCutoverScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CompressedKeyMaterialAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CompressedKeyMaterialResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CompressedKeyMigrationKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CompressedKeyMigrationPrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CrsgenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CrsgenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EIP712DomainChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PrepKeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PrepKeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Upgraded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`KMSGeneration`](self) contract instance.

See the [wrapper's documentation](`KMSGenerationInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> KMSGenerationInstance<P, N> {
        KMSGenerationInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<KMSGenerationInstance<P, N>>,
    > {
        KMSGenerationInstance::<P, N>::deploy(provider)
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
        KMSGenerationInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`KMSGeneration`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`KMSGeneration`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct KMSGenerationInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for KMSGenerationInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("KMSGenerationInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > KMSGenerationInstance<P, N> {
        /**Creates a new wrapper around an on-chain [`KMSGeneration`](self) contract instance.

See the [wrapper's documentation](`KMSGenerationInstance`) for more details.*/
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
        ) -> alloy_contract::Result<KMSGenerationInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> KMSGenerationInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> KMSGenerationInstance<P, N> {
            KMSGenerationInstance {
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
    > KMSGenerationInstance<P, N> {
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
        ///Creates a new call builder for the [`abortCrsgen`] function.
        pub fn abortCrsgen(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, abortCrsgenCall, N> {
            self.call_builder(&abortCrsgenCall { crsId })
        }
        ///Creates a new call builder for the [`abortKeygen`] function.
        pub fn abortKeygen(
            &self,
            prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, abortKeygenCall, N> {
            self.call_builder(&abortKeygenCall { prepKeygenId })
        }
        ///Creates a new call builder for the [`addCompressedKeyMaterials`] function.
        pub fn addCompressedKeyMaterials(
            &self,
            migrationRequestId: alloy::sol_types::private::primitives::aliases::U256,
            keyDigests: alloy::sol_types::private::Vec<
                <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
            >,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, addCompressedKeyMaterialsCall, N> {
            self.call_builder(
                &addCompressedKeyMaterialsCall {
                    migrationRequestId,
                    keyDigests,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`compressedKeyMigrationKeygen`] function.
        pub fn compressedKeyMigrationKeygen(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, compressedKeyMigrationKeygenCall, N> {
            self.call_builder(
                &compressedKeyMigrationKeygenCall {
                    keyId,
                },
            )
        }
        ///Creates a new call builder for the [`crsgenRequest`] function.
        pub fn crsgenRequest(
            &self,
            maxBitLength: alloy::sol_types::private::primitives::aliases::U256,
            paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, crsgenRequestCall, N> {
            self.call_builder(
                &crsgenRequestCall {
                    maxBitLength,
                    paramsType,
                },
            )
        }
        ///Creates a new call builder for the [`crsgenResponse`] function.
        pub fn crsgenResponse(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
            crsDigest: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, crsgenResponseCall, N> {
            self.call_builder(
                &crsgenResponseCall {
                    crsId,
                    crsDigest,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall)
        }
        ///Creates a new call builder for the [`getActiveCrsId`] function.
        pub fn getActiveCrsId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveCrsIdCall, N> {
            self.call_builder(&getActiveCrsIdCall)
        }
        ///Creates a new call builder for the [`getActiveKeyId`] function.
        pub fn getActiveKeyId(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveKeyIdCall, N> {
            self.call_builder(&getActiveKeyIdCall)
        }
        ///Creates a new call builder for the [`getCompletedCrsIds`] function.
        pub fn getCompletedCrsIds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCompletedCrsIdsCall, N> {
            self.call_builder(&getCompletedCrsIdsCall)
        }
        ///Creates a new call builder for the [`getCompletedKeyIds`] function.
        pub fn getCompletedKeyIds(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCompletedKeyIdsCall, N> {
            self.call_builder(&getCompletedKeyIdsCall)
        }
        ///Creates a new call builder for the [`getCompressedKeyCutoverSchedule`] function.
        pub fn getCompressedKeyCutoverSchedule(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCompressedKeyCutoverScheduleCall, N> {
            self.call_builder(
                &getCompressedKeyCutoverScheduleCall {
                    keyId,
                },
            )
        }
        ///Creates a new call builder for the [`getCompressedKeyMaterials`] function.
        pub fn getCompressedKeyMaterials(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCompressedKeyMaterialsCall, N> {
            self.call_builder(
                &getCompressedKeyMaterialsCall {
                    keyId,
                },
            )
        }
        ///Creates a new call builder for the [`getConsensusTxSenders`] function.
        pub fn getConsensusTxSenders(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getConsensusTxSendersCall, N> {
            self.call_builder(
                &getConsensusTxSendersCall {
                    requestId,
                },
            )
        }
        ///Creates a new call builder for the [`getCrsCounter`] function.
        pub fn getCrsCounter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCrsCounterCall, N> {
            self.call_builder(&getCrsCounterCall)
        }
        ///Creates a new call builder for the [`getCrsMaterials`] function.
        pub fn getCrsMaterials(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCrsMaterialsCall, N> {
            self.call_builder(&getCrsMaterialsCall { crsId })
        }
        ///Creates a new call builder for the [`getCrsParamsType`] function.
        pub fn getCrsParamsType(
            &self,
            crsId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getCrsParamsTypeCall, N> {
            self.call_builder(&getCrsParamsTypeCall { crsId })
        }
        ///Creates a new call builder for the [`getKeyCounter`] function.
        pub fn getKeyCounter(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyCounterCall, N> {
            self.call_builder(&getKeyCounterCall)
        }
        ///Creates a new call builder for the [`getKeyInfo`] function.
        pub fn getKeyInfo(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyInfoCall, N> {
            self.call_builder(&getKeyInfoCall { keyId })
        }
        ///Creates a new call builder for the [`getKeyMaterials`] function.
        pub fn getKeyMaterials(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyMaterialsCall, N> {
            self.call_builder(&getKeyMaterialsCall { keyId })
        }
        ///Creates a new call builder for the [`getKeyParamsType`] function.
        pub fn getKeyParamsType(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyParamsTypeCall, N> {
            self.call_builder(&getKeyParamsTypeCall { keyId })
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
        ///Creates a new call builder for the [`isRequestDone`] function.
        pub fn isRequestDone(
            &self,
            requestId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isRequestDoneCall, N> {
            self.call_builder(&isRequestDoneCall { requestId })
        }
        ///Creates a new call builder for the [`keygen`] function.
        pub fn keygen(
            &self,
            paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, keygenCall, N> {
            self.call_builder(&keygenCall { paramsType })
        }
        ///Creates a new call builder for the [`keygenResponse`] function.
        pub fn keygenResponse(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
            keyDigests: alloy::sol_types::private::Vec<
                <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
            >,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, keygenResponseCall, N> {
            self.call_builder(
                &keygenResponseCall {
                    keyId,
                    keyDigests,
                    signature,
                },
            )
        }
        ///Creates a new call builder for the [`prepKeygenResponse`] function.
        pub fn prepKeygenResponse(
            &self,
            prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, prepKeygenResponseCall, N> {
            self.call_builder(
                &prepKeygenResponseCall {
                    prepKeygenId,
                    signature,
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
        ///Creates a new call builder for the [`scheduleCompressedKeyCutover`] function.
        pub fn scheduleCompressedKeyCutover(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
            hostChainCutovers: alloy::sol_types::private::Vec<
                <IKMSGeneration::HostChainCutover as alloy::sol_types::SolType>::RustType,
            >,
            gatewayCutoverBlock: u64,
        ) -> alloy_contract::SolCallBuilder<&P, scheduleCompressedKeyCutoverCall, N> {
            self.call_builder(
                &scheduleCompressedKeyCutoverCall {
                    keyId,
                    hostChainCutovers,
                    gatewayCutoverBlock,
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
    #[automatically_derived]
    impl<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    > KMSGenerationInstance<P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AbortCrsgen`] event.
        pub fn AbortCrsgen_filter(&self) -> alloy_contract::Event<&P, AbortCrsgen, N> {
            self.event_filter::<AbortCrsgen>()
        }
        ///Creates a new event filter for the [`AbortKeygen`] event.
        pub fn AbortKeygen_filter(&self) -> alloy_contract::Event<&P, AbortKeygen, N> {
            self.event_filter::<AbortKeygen>()
        }
        ///Creates a new event filter for the [`ActivateCrs`] event.
        pub fn ActivateCrs_filter(&self) -> alloy_contract::Event<&P, ActivateCrs, N> {
            self.event_filter::<ActivateCrs>()
        }
        ///Creates a new event filter for the [`ActivateKey`] event.
        pub fn ActivateKey_filter(&self) -> alloy_contract::Event<&P, ActivateKey, N> {
            self.event_filter::<ActivateKey>()
        }
        ///Creates a new event filter for the [`CompressedKeyCutoverScheduled`] event.
        pub fn CompressedKeyCutoverScheduled_filter(
            &self,
        ) -> alloy_contract::Event<&P, CompressedKeyCutoverScheduled, N> {
            self.event_filter::<CompressedKeyCutoverScheduled>()
        }
        ///Creates a new event filter for the [`CompressedKeyMaterialAdded`] event.
        pub fn CompressedKeyMaterialAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, CompressedKeyMaterialAdded, N> {
            self.event_filter::<CompressedKeyMaterialAdded>()
        }
        ///Creates a new event filter for the [`CompressedKeyMaterialResponse`] event.
        pub fn CompressedKeyMaterialResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, CompressedKeyMaterialResponse, N> {
            self.event_filter::<CompressedKeyMaterialResponse>()
        }
        ///Creates a new event filter for the [`CompressedKeyMigrationKeygenRequest`] event.
        pub fn CompressedKeyMigrationKeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, CompressedKeyMigrationKeygenRequest, N> {
            self.event_filter::<CompressedKeyMigrationKeygenRequest>()
        }
        ///Creates a new event filter for the [`CompressedKeyMigrationPrepKeygenRequest`] event.
        pub fn CompressedKeyMigrationPrepKeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, CompressedKeyMigrationPrepKeygenRequest, N> {
            self.event_filter::<CompressedKeyMigrationPrepKeygenRequest>()
        }
        ///Creates a new event filter for the [`CrsgenRequest`] event.
        pub fn CrsgenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, CrsgenRequest, N> {
            self.event_filter::<CrsgenRequest>()
        }
        ///Creates a new event filter for the [`CrsgenResponse`] event.
        pub fn CrsgenResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, CrsgenResponse, N> {
            self.event_filter::<CrsgenResponse>()
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
        ///Creates a new event filter for the [`KeygenRequest`] event.
        pub fn KeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeygenRequest, N> {
            self.event_filter::<KeygenRequest>()
        }
        ///Creates a new event filter for the [`KeygenResponse`] event.
        pub fn KeygenResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeygenResponse, N> {
            self.event_filter::<KeygenResponse>()
        }
        ///Creates a new event filter for the [`PrepKeygenRequest`] event.
        pub fn PrepKeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, PrepKeygenRequest, N> {
            self.event_filter::<PrepKeygenRequest>()
        }
        ///Creates a new event filter for the [`PrepKeygenResponse`] event.
        pub fn PrepKeygenResponse_filter(
            &self,
        ) -> alloy_contract::Event<&P, PrepKeygenResponse, N> {
            self.event_filter::<PrepKeygenResponse>()
        }
        ///Creates a new event filter for the [`Upgraded`] event.
        pub fn Upgraded_filter(&self) -> alloy_contract::Event<&P, Upgraded, N> {
            self.event_filter::<Upgraded>()
        }
    }
}
