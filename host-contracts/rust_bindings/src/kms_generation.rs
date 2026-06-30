///Module containing a contract's types and functions.
/**

```solidity
library IKMSGeneration {
    type KeyType is uint8;
    type ParamsType is uint8;
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
    error CrsAborted(uint256 crsId);
    error CrsNotGenerated(uint256 crsId);
    error CrsgenNotRequested(uint256 crsId);
    error CrsgenOngoing(uint256 crsId);
    error DeserializingExtraDataFail();
    error ECDSAInvalidSignature();
    error ECDSAInvalidSignatureLength(uint256 length);
    error ECDSAInvalidSignatureS(bytes32 s);
    error ERC1967InvalidImplementation(address implementation);
    error ERC1967NonPayable();
    error EmptyKeyDigests(uint256 keyId);
    error FailedCall();
    error InvalidInitialization();
    error KeyAborted(uint256 keyId);
    error KeyManagementRequestPending();
    error KeyMaterialNotPublished(uint256 keyId);
    error KeyNotGenerated(uint256 keyId);
    error KeygenNotRequested(uint256 keyId);
    error KeygenOngoing(uint256 keyId);
    error KmsAlreadySignedForCrsgen(uint256 crsId, address kmsSigner);
    error KmsAlreadySignedForKeygen(uint256 keyId, address kmsSigner);
    error KmsAlreadySignedForPrepKeygen(uint256 prepKeygenId, address kmsSigner);
    error KmsSignerDoesNotMatchTxSender(address signerAddress, address txSenderAddress);
    error MigrationKeyNotForExistingKey(uint256 migrationKeyId, uint256 existingKeyId);
    error MismatchedMigrationArrays();
    error NotHostOwner(address sender);
    error NotInitializing();
    error NotInitializingFromEmptyProxy();
    error NotKmsTxSender(address txSenderAddress);
    error PrepKeygenNotRequested(uint256 prepKeygenId);
    error UUPSUnauthorizedCallContext();
    error UUPSUnsupportedProxiableUUID(bytes32 slot);
    error UnsupportedExtraDataVersion(uint8 version);

    event AbortCrsgen(uint256 crsId);
    event AbortKeygen(uint256 prepKeygenId);
    event ActivateCrs(uint256 crsId, string[] kmsNodeStorageUrls, bytes crsDigest);
    event ActivateKey(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests);
    event CrsgenRequest(uint256 crsId, uint256 maxBitLength, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event CrsgenResponse(uint256 crsId, bytes crsDigest, bytes signature, address kmsTxSender);
    event EIP712DomainChanged();
    event Initialized(uint64 version);
    event KeyMaterialAdded(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests, uint256 materialVersion);
    event KeyMaterialMigrationScheduled(uint256 keyId, uint256[] hostChainIds, uint256[] hostMigrationBlocks, uint256 gatewayMigrationBlock, uint256 materialVersion);
    event KeygenRequest(uint256 prepKeygenId, uint256 keyId, bytes extraData);
    event KeygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] keyDigests, bytes signature, address kmsTxSender);
    event MigrationKeygenRequest(uint256 prepKeygenId, uint256 keyId, uint256 existingKeyId, bool copyToOriginal, bytes extraData);
    event PrepKeygenRequest(uint256 prepKeygenId, IKMSGeneration.ParamsType paramsType, bytes extraData);
    event PrepKeygenResponse(uint256 prepKeygenId, bytes signature, address kmsTxSender);
    event Upgraded(address indexed implementation);

    constructor();

    function UPGRADE_INTERFACE_VERSION() external view returns (string memory);
    function abortCrsgen(uint256 crsId) external;
    function abortKeygen(uint256 prepKeygenId) external;
    function addKeyMaterials(uint256 existingKeyId, uint256 migrationKeyId, IKMSGeneration.KeyDigest[] memory keyDigests, string[] memory kmsNodeStorageUrls) external;
    function crsgenRequest(uint256 maxBitLength, IKMSGeneration.ParamsType paramsType) external;
    function crsgenResponse(uint256 crsId, bytes memory crsDigest, bytes memory signature) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function getActiveCrsId() external view returns (uint256);
    function getActiveKeyId() external view returns (uint256);
    function getCompletedCrsIds() external view returns (uint256[] memory);
    function getCompletedKeyIds() external view returns (uint256[] memory);
    function getConsensusTxSenders(uint256 requestId) external view returns (address[] memory);
    function getCrsCounter() external view returns (uint256);
    function getCrsMaterials(uint256 crsId) external view returns (string[] memory, bytes memory);
    function getCrsParamsType(uint256 crsId) external view returns (IKMSGeneration.ParamsType);
    function getKeyCounter() external view returns (uint256);
    function getKeyInfo(uint256 keyId) external view returns (IKMSGeneration.KeyInfo memory);
    function getKeyMaterialVersion(uint256 keyId) external view returns (uint256);
    function getKeyMaterials(uint256 keyId) external view returns (string[] memory, IKMSGeneration.KeyDigest[] memory);
    function getKeyParamsType(uint256 keyId) external view returns (IKMSGeneration.ParamsType);
    function getVersion() external pure returns (string memory);
    function initializeFromEmptyProxy() external;
    function isRequestDone(uint256 requestId) external view returns (bool);
    function keygen(IKMSGeneration.ParamsType paramsType) external;
    function keygenResponse(uint256 keyId, IKMSGeneration.KeyDigest[] memory keyDigests, bytes memory signature) external;
    function migrationKeygen(IKMSGeneration.ParamsType paramsType, uint256 existingKeyId) external;
    function prepKeygenResponse(uint256 prepKeygenId, bytes memory signature) external;
    function proxiableUUID() external view returns (bytes32);
    function reinitializeV2() external;
    function scheduleKeyMaterialMigration(uint256 keyId, uint256[] memory hostChainIds, uint256[] memory hostMigrationBlocks, uint256 gatewayMigrationBlock) external;
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
    "name": "addKeyMaterials",
    "inputs": [
      {
        "name": "existingKeyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "migrationKeyId",
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
        "name": "kmsNodeStorageUrls",
        "type": "string[]",
        "internalType": "string[]"
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
    "name": "getKeyMaterialVersion",
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
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "migrationKeygen",
    "inputs": [
      {
        "name": "paramsType",
        "type": "uint8",
        "internalType": "enum IKMSGeneration.ParamsType"
      },
      {
        "name": "existingKeyId",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "reinitializeV2",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "scheduleKeyMaterialMigration",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "hostChainIds",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "hostMigrationBlocks",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "gatewayMigrationBlock",
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
    "name": "KeyMaterialAdded",
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
      },
      {
        "name": "materialVersion",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeyMaterialMigrationScheduled",
    "inputs": [
      {
        "name": "keyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "hostChainIds",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "hostMigrationBlocks",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "gatewayMigrationBlock",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "materialVersion",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "MigrationKeygenRequest",
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
        "name": "existingKeyId",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "copyToOriginal",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
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
    "name": "KeyMaterialNotPublished",
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
    "name": "MigrationKeyNotForExistingKey",
    "inputs": [
      {
        "name": "migrationKeyId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "existingKeyId",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "MismatchedMigrationArrays",
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
    ///0x60a06040523073ffffffffffffffffffffffffffffffffffffffff1660809073ffffffffffffffffffffffffffffffffffffffff1681525034801562000043575f80fd5b50620000546200005a60201b60201c565b620001c4565b5f6200006b6200015e60201b60201c565b9050805f0160089054906101000a900460ff1615620000b6576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b67ffffffffffffffff8016815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff16146200015b5767ffffffffffffffff815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d267ffffffffffffffff604051620001529190620001a9565b60405180910390a15b50565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b5f67ffffffffffffffff82169050919050565b620001a38162000185565b82525050565b5f602082019050620001be5f83018462000198565b92915050565b608051617bdd620001eb5f395f8181613de001528181613e3501526140d70152617bdd5ff3fe6080604052600436106101d7575f3560e01c80636294f46211610101578063c2c1faee11610094578063d52f10eb11610063578063d52f10eb14610685578063dabd732f146106af578063e410117e146106d9578063f0f8cbc614610703576101d7565b8063c2c1faee146105e2578063c41158741461060a578063c55b872414610620578063caa367db1461065d576101d7565b8063aaa47016116100d0578063aaa470161461053e578063ad3cb1cc14610566578063b53b3ccc14610590578063baff211e146105b8576101d7565b80636294f4621461046d57806362978787146104a957806384b0196e146104d1578063936608ae14610501576101d7565b80633c02f834116101795780634f1ef286116101485780634f1ef286146103d757806352d1902d146103f357806356a610b41461041d578063589adb0e14610445576101d7565b80633c02f8341461030f5780633d5ec7e31461033757806345af261b146103735780634610ffe8146103af576101d7565b80631703c61a116101b55780631703c61a1461026b57806319f4f6321461029357806339f73810146102cf5780633ac50072146102e5576101d7565b80630b680733146101db5780630d8e6e2c1461020557806316c713d91461022f575b5f80fd5b3480156101e6575f80fd5b506101ef61073f565b6040516101fc91906153d3565b60405180910390f35b348015610210575f80fd5b50610219610756565b6040516102269190615476565b60405180910390f35b34801561023a575f80fd5b50610255600480360381019061025091906154d1565b6107d1565b60405161026291906155e3565b60405180910390f35b348015610276575f80fd5b50610291600480360381019061028c91906154d1565b6108a2565b005b34801561029e575f80fd5b506102b960048036038101906102b491906154d1565b610ad0565b6040516102c69190615676565b60405180910390f35b3480156102da575f80fd5b506102e3610bd6565b005b3480156102f0575f80fd5b506102f9610e26565b60405161030691906153d3565b60405180910390f35b34801561031a575f80fd5b50610335600480360381019061033091906156b2565b610e3d565b005b348015610342575f80fd5b5061035d600480360381019061035891906154d1565b611134565b60405161036a919061570a565b60405180910390f35b34801561037e575f80fd5b50610399600480360381019061039491906154d1565b611168565b6040516103a69190615676565b60405180910390f35b3480156103ba575f80fd5b506103d560048036038101906103d091906157d9565b611256565b005b6103f160048036038101906103ec91906159bc565b6117a6565b005b3480156103fe575f80fd5b506104076117c5565b6040516104149190615a2e565b60405180910390f35b348015610428575f80fd5b50610443600480360381019061043e9190615a9c565b6117f6565b005b348015610450575f80fd5b5061046b60048036038101906104669190615b3f565b6119d4565b005b348015610478575f80fd5b50610493600480360381019061048e91906154d1565b611d90565b6040516104a09190615da7565b60405180910390f35b3480156104b4575f80fd5b506104cf60048036038101906104ca9190615dc7565b611ffd565b005b3480156104dc575f80fd5b506104e561242a565b6040516104f89796959493929190615f49565b60405180910390f35b34801561050c575f80fd5b50610527600480360381019061052291906154d1565b612533565b604051610535929190616151565b60405180910390f35b348015610549575f80fd5b50610564600480360381019061055f9190616186565b612899565b005b348015610571575f80fd5b5061057a612a5d565b6040516105879190615476565b60405180910390f35b34801561059b575f80fd5b506105b660048036038101906105b19190616219565b612a96565b005b3480156105c3575f80fd5b506105cc612cec565b6040516105d991906153d3565b60405180910390f35b3480156105ed575f80fd5b50610608600480360381019061060391906154d1565b612d03565b005b348015610615575f80fd5b5061061e612f7c565b005b34801561062b575f80fd5b50610646600480360381019061064191906154d1565b6131c5565b604051610654929190616304565b60405180910390f35b348015610668575f80fd5b50610683600480360381019061067e9190616339565b613496565b005b348015610690575f80fd5b50610699613594565b6040516106a691906153d3565b60405180910390f35b3480156106ba575f80fd5b506106c36135ab565b6040516106d09190616364565b60405180910390f35b3480156106e4575f80fd5b506106ed61360f565b6040516106fa9190616364565b60405180910390f35b34801561070e575f80fd5b50610729600480360381019061072491906154d1565b613673565b60405161073691906153d3565b60405180910390f35b5f806107496136a5565b9050806005015491505090565b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506107975f6136cc565b6107a160026136cc565b6107aa5f6136cc565b6040516020016107bd9493929190616452565b604051602081830303815290604052905090565b60605f6107dc6136a5565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561089457602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161084b575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108ff573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061092391906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461099257336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161098991906164ef565b60405180910390fd5b5f61099b6136a5565b905080600901548211806109c6575060f8600560088111156109c0576109bf615603565b5b901b8211155b15610a0857816040517fcbe926560000000000000000000000000000000000000000000000000000000081526004016109ff91906153d3565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff1615610a6a57816040517fdf0db5fb000000000000000000000000000000000000000000000000000000008152600401610a6191906153d3565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e82604051610ac491906153d3565b60405180910390a15050565b5f80610ada6136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610b3d57826040517f84de1331000000000000000000000000000000000000000000000000000000008152600401610b3491906153d3565b60405180910390fd5b5f801b816003015f8581526020019081526020015f205403610b9657826040517f83f18335000000000000000000000000000000000000000000000000000000008152600401610b8d91906153d3565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b6001610be0613796565b67ffffffffffffffff1614610c21576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610c2c6137ba565b9050805f0160089054906101000a900460ff1680610c7457508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610cab576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610d646040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152506137e1565b5f610d6d6136a5565b905060f860036008811115610d8557610d84615603565b5b901b816004018190555060f860046008811115610da557610da4615603565b5b901b816005018190555060f860056008811115610dc557610dc4615603565b5b901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610e1a919061652a565b60405180910390a15050565b5f80610e306136a5565b9050806009015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e9a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ebe91906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f2d57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610f2491906164ef565b60405180910390fd5b5f610f366136a5565b90505f8160090154905060f860056008811115610f5657610f55615603565b5b901b8114158015610f845750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610fc657806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610fbd91906153d3565b60405180910390fd5b816009015f815480929190610fda90616570565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff0219169083600181111561103457611033615603565b5b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015611097573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110bb91906165cb565b915091505f6110ca83836137f7565b90508086600e015f8681526020019081526020015f2090816110ec9190616803565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d8489898460405161112294939291906168d2565b60405180910390a15050505050505050565b5f8061113e6136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806111726136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166111d557826040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016111cc91906153d3565b60405180910390fd5b5f801b816003015f8581526020019081526020015f20540361122e57826040517fd5fd3cd700000000000000000000000000000000000000000000000000000000815260040161122591906153d3565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f61125f6136a5565b9050806005015486118061128a575060f86004600881111561128457611283615603565b5b901b8611155b156112cc57856040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016112c391906153d3565b60405180910390fd5b5f858590500361131357856040517fe6f9083b00000000000000000000000000000000000000000000000000000000815260040161130a91906153d3565b60405180910390fd5b5f8061131e88613826565b915091505f836006015f8a81526020019081526020015f20549050836001015f8281526020019081526020015f205f9054906101000a900460ff1661138f576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61139d828b8b8b886139b0565b90505f6113ac84838a8a613b91565b9050855f015f8c81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561144c578a816040517f98fb957d00000000000000000000000000000000000000000000000000000000815260040161144392919061691c565b60405180910390fd5b6001865f015f8d81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f866002015f8d81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78c8c8c8c8c3360405161156e96959493929190616b4f565b60405180910390a1866001015f8d81526020019081526020015f205f9054906101000a900460ff161580156115ad57506115ac858280549050613bf9565b5b15611798576001876001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8b8b905081101561166357876007015f8e81526020019081526020015f208c8c838181106116105761160f616ba4565b5b90506020028101906116229190616bdd565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816116549190616e10565b505080806001019150506115df565b5082876003015f8e81526020019081526020015f208190555086600f018c908060018154018082558091505060019003905f5260205f20015f90919091909150555f876011015f8e81526020019081526020015f205f015403611797578b87600801819055505f611756868380548060200260200160405190810160405280929190818152602001828054801561174c57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611703575b5050505050613c96565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8d828e8e60405161178d9493929190616e1e565b60405180910390a1505b5b505050505050505050505050565b6117ae613dde565b6117b782613ec4565b6117c18282613fb7565b5050565b5f6117ce6140d5565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611853573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061187791906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146118e657336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016118dd91906164ef565b60405180910390fd5b5f6118ef6136a5565b9050838390508686905014611930576040517f894b2ab300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f816012015f8981526020019081526020015f20540361198757866040517f05b083f200000000000000000000000000000000000000000000000000000000815260040161197e91906153d3565b60405180910390fd5b7f8bfa7d0ed6f87e526b62342918ee7bfa53952badd463dc934054d7dd940eafdc87878787878760016040516119c39796959493929190616ecb565b60405180910390a150505050505050565b5f6119dd6136a5565b90508060040154841180611a08575060f860036008811115611a0257611a01615603565b5b901b8411155b15611a4a57836040517f0ab7f687000000000000000000000000000000000000000000000000000000008152600401611a4191906153d3565b60405180910390fd5b5f80611a5586613826565b915091505f611a64878461415c565b90505f611a7383838989613b91565b9050845f015f8981526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611b135787816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611b0a92919061691c565b60405180910390fd5b6001855f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8a81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c89898933604051611c319493929190616f2e565b60405180910390a1856001015f8a81526020019081526020015f205f9054906101000a900460ff16158015611c705750611c6f848280549050613bf9565b5b15611d85576001866001015f8b81526020019081526020015f205f6101000a81548160ff02191690831515021790555082866003015f8b81526020019081526020015f20819055505f866006015f8b81526020019081526020015f205490505f876011015f8381526020019081526020015f2090505f815f015414611d46577fe453c29c46ccc7664c0398e8464d5bb421e995432daf5506a3fdbc6aa0966a938b83835f0154846001015f9054906101000a900460ff168b604051611d39959493929190616f6c565b60405180910390a1611d82565b7f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8b8389604051611d7993929190616fc4565b60405180910390a15b50505b505050505050505050565b611d98615384565b5f611da16136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16611e0457826040517f84de1331000000000000000000000000000000000000000000000000000000008152600401611dfb91906153d3565b60405180910390fd5b5f801b816003015f8581526020019081526020015f205403611e5d57826040517f83f18335000000000000000000000000000000000000000000000000000000008152600401611e5491906153d3565b60405180910390fd5b5f816006015f8581526020019081526020015f20549050604051806080016040528082815260200185815260200183600d015f8481526020019081526020015f205f9054906101000a900460ff166001811115611ebd57611ebc615603565b5b8152602001836007015f8781526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015611fed578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff166001811115611f3857611f37615603565b5b6001811115611f4a57611f49615603565b5b8152602001600182018054611f5e90616636565b80601f0160208091040260200160405190810160405280929190818152602001828054611f8a90616636565b8015611fd55780601f10611fac57610100808354040283529160200191611fd5565b820191905f5260205f20905b815481529060010190602001808311611fb857829003601f168201915b50505050508152505081526020019060010190611ef4565b5050505081525092505050919050565b5f6120066136a5565b90508060090154861180612031575060f86005600881111561202b5761202a615603565b5b901b8611155b1561207357856040517f8d8c940a00000000000000000000000000000000000000000000000000000000815260040161206a91906153d3565b60405180910390fd5b5f8061207e88613826565b915091505f6120a38985600a015f8c81526020019081526020015f20548a8a876141be565b90505f6120b283838989613b91565b9050845f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156121525789816040517ffcf5a6e900000000000000000000000000000000000000000000000000000000815260040161214992919061691c565b60405180910390fd5b6001855f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8b8b8b8b8b3360405161227496959493929190617000565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff161580156122b357506122b2848280549050613bf9565b5b1561241d576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f209182612305929190616cef565b5082866003015f8d81526020019081526020015f20819055508a86600c0181905550856010018b908060018154018082558091505060019003905f5260205f20015f90919091909150555f6123dc85838054806020026020016040519081016040528092919081815260200182805480156123d257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612389575b5050505050613c96565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d6040516124139493929190617055565b60405180910390a1505b5050505050505050505050565b5f6060805f805f60605f61243c61424f565b90505f801b815f015414801561245757505f801b8160010154145b612496576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161248d906170e4565b60405180910390fd5b61249e614276565b6124a6614314565b46305f801b5f67ffffffffffffffff8111156124c5576124c4615898565b5b6040519080825280602002602001820160405280156124f35781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f61253f6136a5565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166125a257836040517f84de133100000000000000000000000000000000000000000000000000000000815260040161259991906153d3565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b81036125ff57846040517f83f183350000000000000000000000000000000000000000000000000000000081526004016125f691906153d3565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561269f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612656575b505050505090505f61274984600e015f8981526020019081526020015f2080546126c890616636565b80601f01602080910402602001604051908101604052809291908181526020018280546126f490616636565b801561273f5780601f106127165761010080835404028352916020019161273f565b820191905f5260205f20905b81548152906001019060200180831161272257829003601f168201915b50505050506143b2565b90505f6127568284613c96565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015612885578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156127d0576127cf615603565b5b60018111156127e2576127e1615603565b5b81526020016001820180546127f690616636565b80601f016020809104026020016040519081016040528092919081815260200182805461282290616636565b801561286d5780601f106128445761010080835404028352916020019161286d565b820191905f5260205f20905b81548152906001019060200180831161285057829003601f168201915b5050505050815250508152602001906001019061278c565b505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156128f6573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061291a91906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461298957336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161298091906164ef565b60405180910390fd5b5f6129926136a5565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff166129f557816040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016129ec91906153d3565b60405180910390fd5b5f6129ff846145a1565b915050604051806040016040528084815260200160011515815250826011015f8381526020019081526020015f205f820151815f01556020820151816001015f6101000a81548160ff02191690831515021790555090505050505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612af3573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b1791906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612b8657336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612b7d91906164ef565b60405180910390fd5b5f612b8f6136a5565b905086816011015f8881526020019081526020015f205f015414612bec5785876040517f9431f34e000000000000000000000000000000000000000000000000000000008152600401612be3929190617102565b60405180910390fd5b806001015f8781526020019081526020015f205f9054906101000a900460ff16612c42576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8585905003612c8957866040517fe6f9083b000000000000000000000000000000000000000000000000000000008152600401612c8091906153d3565b60405180910390fd5b85816012015f8981526020019081526020015f20819055507fa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa8187848488886001604051612cdb96959493929190617255565b60405180910390a150505050505050565b5f80612cf66136a5565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d60573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d8491906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612df357336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612dea91906164ef565b60405180910390fd5b5f612dfc6136a5565b90508060040154821180612e27575060f860036008811115612e2157612e20615603565b5b901b8211155b15612e6957816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401612e6091906153d3565b60405180910390fd5b5f816006015f8481526020019081526020015f20549050816001015f8281526020019081526020015f205f9054906101000a900460ff1615612ee257826040517f92789b67000000000000000000000000000000000000000000000000000000008152600401612ed991906153d3565b60405180910390fd5b6001826001015f8581526020019081526020015f205f6101000a81548160ff0219169083151502179055505f8114612f40576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe326483604051612f6f91906153d3565b60405180910390a1505050565b60035f612f876137ba565b9050805f0160089054906101000a900460ff1680612fcf57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15613006576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f6130546136a5565b90505f600160f86004600881111561306f5761306e615603565b5b901b61307b91906172aa565b90505b816005015481116130e1575f801b826003015f8381526020019081526020015f2054146130ce5781600f0181908060018154018082558091505060019003905f5260205f20015f90919091909150555b80806130d990616570565b91505061307e565b505f600160f8600560088111156130fb576130fa615603565b5b901b61310791906172aa565b90505b8160090154811161316d575f801b826003015f8381526020019081526020015f20541461315a578160100181908060018154018082558091505060019003905f5260205f20015f90919091909150555b808061316590616570565b91505061310a565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516131b9919061652a565b60405180910390a15050565b6060805f6131d16136a5565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff1661323457836040517fda32d00f00000000000000000000000000000000000000000000000000000000815260040161322b91906153d3565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b810361329157846040517fd5fd3cd700000000000000000000000000000000000000000000000000000000815260040161328891906153d3565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561333157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116132e8575b505050505090505f6133db84600e015f8981526020019081526020015f20805461335a90616636565b80601f016020809104026020016040519081016040528092919081815260200182805461338690616636565b80156133d15780601f106133a8576101008083540402835291602001916133d1565b820191905f5260205f20905b8154815290600101906020018083116133b457829003601f168201915b50505050506143b2565b90505f6133e88284613c96565b90508085600b015f8a81526020019081526020015f2080805461340a90616636565b80601f016020809104026020016040519081016040528092919081815260200182805461343690616636565b80156134815780601f1061345857610100808354040283529160200191613481565b820191905f5260205f20905b81548152906001019060200180831161346457829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156134f3573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061351791906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461358657336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161357d91906164ef565b60405180910390fd5b61358f816145a1565b505050565b5f8061359e6136a5565b9050806008015491505090565b60605f6135b66136a5565b90508060100180548060200260200160405190810160405280929190818152602001828054801561360457602002820191905f5260205f20905b8154815260200190600101908083116135f0575b505050505091505090565b60605f61361a6136a5565b905080600f0180548060200260200160405190810160405280929190818152602001828054801561366857602002820191905f5260205f20905b815481526020019060010190808311613654575b505050505091505090565b5f8061367d6136a5565b6012015f8481526020019081526020015f20540361369b575f61369e565b60015b9050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b60605f60016136da84614800565b0190505f8167ffffffffffffffff8111156136f8576136f7615898565b5b6040519080825280601f01601f19166020018201604052801561372a5781602001600182028036833780820191505090505b5090505f82602001820190505b60011561378b578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816137805761377f6172dd565b5b0494505f8503613737575b819350505050919050565b5f61379f6137ba565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6137e9614951565b6137f38282614991565b5050565b60606002838360405160200161380f9392919061736a565b604051602081830303815290604052905092915050565b60605f806138326136a5565b905080600e015f8581526020019081526020015f20805461385290616636565b80601f016020809104026020016040519081016040528092919081815260200182805461387e90616636565b80156138c95780601f106138a0576101008083540402835291602001916138c9565b820191905f5260205f20905b8154815290600101906020018083116138ac57829003601f168201915b505050505092506138d9836143b2565b91507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd83336040518363ffffffff1660e01b815260040161392a92919061691c565b602060405180830381865afa158015613945573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061396991906173d0565b6139aa57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016139a191906164ef565b60405180910390fd5b50915091565b5f808484905067ffffffffffffffff8111156139cf576139ce615898565b5b6040519080825280602002602001820160405280156139fd5781602001602082028036833780820191505090505b5090505f5b85859050811015613b0157604051806060016040528060258152602001617bb86025913980519060200120868683818110613a4057613a3f616ba4565b5b9050602002810190613a529190616bdd565b5f016020810190613a6391906173fb565b878784818110613a7657613a75616ba4565b5b9050602002810190613a889190616bdd565b8060200190613a979190616c83565b604051613aa5929190617454565b6040518091039020604051602001613abf9392919061747b565b60405160208183030381529060405280519060200120828281518110613ae857613ae7616ba4565b5b6020026020010181815250508080600101915050613a02565b50613b856040518060c0016040528060828152602001617b366082913980519060200120888884604051602001613b389190617561565b604051602081830303815290604052805190602001208780519060200120604051602001613b6a959493929190617577565b604051602081830303815290604052805190602001206149e2565b91505095945050505050565b5f80613be08585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506149fb565b9050613bed868233614a25565b80915050949350505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166341ad069c856040518263ffffffff1660e01b8152600401613c4891906153d3565b602060405180830381865afa158015613c63573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c8791906175c8565b90508083101591505092915050565b60605f825190505f8167ffffffffffffffff811115613cb857613cb7615898565b5b604051908082528060200260200182016040528015613ceb57816020015b6060815260200190600190039081613cd65790505b5090505f5b82811015613dd2577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c887878481518110613d3c57613d3b616ba4565b5b60200260200101516040518363ffffffff1660e01b8152600401613d6192919061691c565b5f60405180830381865afa158015613d7b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190613da39190617746565b60600151828281518110613dba57613db9616ba4565b5b60200260200101819052508080600101915050613cf0565b50809250505092915050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480613e8b57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613e72614c06565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613ec2576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613f21573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f4591906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613fb457336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613fab91906164ef565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561401f57506040513d601f19601f8201168201806040525081019061401c91906177b7565b60015b61406057816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161405791906164ef565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146140c657806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016140bd9190615a2e565b60405180910390fd5b6140d08383614c59565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461415a576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6141b66040518060600160405280603c8152602001617aa4603c91398051906020012084848051906020012060405160200161419b939291906177e2565b604051602081830303815290604052805190602001206149e2565b905092915050565b5f614244604051806080016040528060568152602001617ae06056913980519060200120878787876040516020016141f7929190617454565b604051602081830303815290604052805190602001208680519060200120604051602001614229959493929190617577565b604051602081830303815290604052805190602001206149e2565b905095945050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f61428161424f565b905080600201805461429290616636565b80601f01602080910402602001604051908101604052809291908181526020018280546142be90616636565b80156143095780601f106142e057610100808354040283529160200191614309565b820191905f5260205f20905b8154815290600101906020018083116142ec57829003601f168201915b505050505091505090565b60605f61431f61424f565b905080600301805461433090616636565b80601f016020809104026020016040519081016040528092919081815260200182805461435c90616636565b80156143a75780601f1061437e576101008083540402835291602001916143a7565b820191905f5260205f20905b81548152906001019060200180831161438a57829003601f168201915b505050505091505090565b5f80825114806143e457505f825f815181106143d1576143d0616ba4565b5b602001015160f81c60f81b60f81c60ff16145b15614471577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015614446573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061446a91906175c8565b905061459c565b5f825f8151811061448557614484616ba4565b5b602001015160f81c60f81b60f81c9050600160ff168160ff16141580156144b35750600260ff168160ff1614155b156144f557806040517f2139cc2c0000000000000000000000000000000000000000000000000000000081526004016144ec9190617826565b60405180910390fd5b600160ff168160ff1614801561450d57506021835114155b15614544576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260ff168160ff1614801561455c57506041835114155b15614593576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60218301519150505b919050565b5f805f6145ac6136a5565b90505f8160050154905060f8600460088111156145cc576145cb615603565b5b901b81141580156145fa5750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561463c57806040517f3b853da800000000000000000000000000000000000000000000000000000000815260040161463391906153d3565b60405180910390fd5b816004015f81548092919061465090616570565b919050555081600401549350816005015f81548092919061467090616570565b91905055508160050154925082826006015f8681526020019081526020015f208190555083826006015f8581526020019081526020015f20819055508482600d015f8681526020019081526020015f205f6101000a81548160ff021916908360018111156146e1576146e0615603565b5b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015614744573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061476891906165cb565b915091505f61477783836137f7565b90508085600e015f8981526020019081526020015f2090816147999190616803565b508085600e015f8881526020019081526020015f2090816147ba9190616803565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe918789836040516147ee9392919061783f565b60405180910390a15050505050915091565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061485c577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381614852576148516172dd565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310614899576d04ee2d6d415b85acef8100000000838161488f5761488e6172dd565b5b0492506020810190505b662386f26fc1000083106148c857662386f26fc1000083816148be576148bd6172dd565b5b0492506010810190505b6305f5e10083106148f1576305f5e10083816148e7576148e66172dd565b5b0492506008810190505b612710831061491657612710838161490c5761490b6172dd565b5b0492506004810190505b60648310614939576064838161492f5761492e6172dd565b5b0492506002810190505b600a8310614948576001810190505b80915050919050565b614959614ccb565b61498f576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b614999614951565b5f6149a261424f565b9050828160020190816149b591906178d3565b50818160030190816149c791906178d3565b505f801b815f01819055505f801b8160010181905550505050565b5f6149f46149ee614ce9565b83614cf7565b9050919050565b5f805f80614a098686614d37565b925092509250614a198282614d8c565b82935050505092915050565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd484846040518363ffffffff1660e01b8152600401614a7492919061691c565b602060405180830381865afa158015614a8f573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ab391906173d0565b614af65781816040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614aed9291906179a2565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b8152600401614b4692919061691c565b5f60405180830381865afa158015614b60573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190614b889190617746565b90508273ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff1614614c005782826040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614bf79291906179a2565b60405180910390fd5b50505050565b5f614c327f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614eee565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b614c6282614ef7565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614cbe57614cb88282614fc0565b50614cc7565b614cc6615040565b5b5050565b5f614cd46137ba565b5f0160089054906101000a900460ff16905090565b5f614cf261507c565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614d77575f805f602087015192506040870151915060608701515f1a9050614d69888285856150df565b955095509550505050614d85565b5f600285515f1b9250925092505b9250925092565b5f6003811115614d9f57614d9e615603565b5b826003811115614db257614db1615603565b5b0315614eea5760016003811115614dcc57614dcb615603565b5b826003811115614ddf57614dde615603565b5b03614e16576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115614e2a57614e29615603565b5b826003811115614e3d57614e3c615603565b5b03614e8157805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614e7891906153d3565b60405180910390fd5b600380811115614e9457614e93615603565b5b826003811115614ea757614ea6615603565b5b03614ee957806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614ee09190615a2e565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614f5257806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614f4991906164ef565b60405180910390fd5b80614f7e7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614eee565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614fe991906179f9565b5f60405180830381855af49150503d805f8114615021576040519150601f19603f3d011682016040523d82523d5f602084013e615026565b606091505b50915091506150368583836151c6565b9250505092915050565b5f34111561507a576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6150a6615253565b6150ae6152c9565b46306040516020016150c4959493929190617a0f565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c111561511b575f6003859250925092506151bc565b5f6001888888886040515f815260200160405260405161513e9493929190617a60565b6020604051602081039080840390855afa15801561515e573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036151af575f60015f801b935093509350506151bc565b805f805f1b935093509350505b9450945094915050565b6060826151db576151d682615340565b61524b565b5f825114801561520157505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561524357836040517f9996b31500000000000000000000000000000000000000000000000000000000815260040161523a91906164ef565b60405180910390fd5b81905061524c565b5b9392505050565b5f8061525d61424f565b90505f615268614276565b90505f81511115615284578080519060200120925050506152c6565b5f825f015490505f801b811461529f578093505050506152c6565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806152d361424f565b90505f6152de614314565b90505f815111156152fa5780805190602001209250505061533d565b5f826001015490505f801b81146153165780935050505061533d565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156153525780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180608001604052805f81526020015f81526020015f60018111156153ae576153ad615603565b5b8152602001606081525090565b5f819050919050565b6153cd816153bb565b82525050565b5f6020820190506153e65f8301846153c4565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015615423578082015181840152602081019050615408565b5f8484015250505050565b5f601f19601f8301169050919050565b5f615448826153ec565b61545281856153f6565b9350615462818560208601615406565b61546b8161542e565b840191505092915050565b5f6020820190508181035f83015261548e818461543e565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b6154b0816153bb565b81146154ba575f80fd5b50565b5f813590506154cb816154a7565b92915050565b5f602082840312156154e6576154e561549f565b5b5f6154f3848285016154bd565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61554e82615525565b9050919050565b61555e81615544565b82525050565b5f61556f8383615555565b60208301905092915050565b5f602082019050919050565b5f615591826154fc565b61559b8185615506565b93506155a683615516565b805f5b838110156155d65781516155bd8882615564565b97506155c88361557b565b9250506001810190506155a9565b5085935050505092915050565b5f6020820190508181035f8301526155fb8184615587565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061564157615640615603565b5b50565b5f81905061565182615630565b919050565b5f61566082615644565b9050919050565b61567081615656565b82525050565b5f6020820190506156895f830184615667565b92915050565b6002811061569b575f80fd5b50565b5f813590506156ac8161568f565b92915050565b5f80604083850312156156c8576156c761549f565b5b5f6156d5858286016154bd565b92505060206156e68582860161569e565b9150509250929050565b5f8115159050919050565b615704816156f0565b82525050565b5f60208201905061571d5f8301846156fb565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261574457615743615723565b5b8235905067ffffffffffffffff81111561576157615760615727565b5b60208301915083602082028301111561577d5761577c61572b565b5b9250929050565b5f8083601f84011261579957615798615723565b5b8235905067ffffffffffffffff8111156157b6576157b5615727565b5b6020830191508360018202830111156157d2576157d161572b565b5b9250929050565b5f805f805f606086880312156157f2576157f161549f565b5b5f6157ff888289016154bd565b955050602086013567ffffffffffffffff8111156158205761581f6154a3565b5b61582c8882890161572f565b9450945050604086013567ffffffffffffffff81111561584f5761584e6154a3565b5b61585b88828901615784565b92509250509295509295909350565b61587381615544565b811461587d575f80fd5b50565b5f8135905061588e8161586a565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6158ce8261542e565b810181811067ffffffffffffffff821117156158ed576158ec615898565b5b80604052505050565b5f6158ff615496565b905061590b82826158c5565b919050565b5f67ffffffffffffffff82111561592a57615929615898565b5b6159338261542e565b9050602081019050919050565b828183375f83830152505050565b5f61596061595b84615910565b6158f6565b90508281526020810184848401111561597c5761597b615894565b5b615987848285615940565b509392505050565b5f82601f8301126159a3576159a2615723565b5b81356159b384826020860161594e565b91505092915050565b5f80604083850312156159d2576159d161549f565b5b5f6159df85828601615880565b925050602083013567ffffffffffffffff811115615a00576159ff6154a3565b5b615a0c8582860161598f565b9150509250929050565b5f819050919050565b615a2881615a16565b82525050565b5f602082019050615a415f830184615a1f565b92915050565b5f8083601f840112615a5c57615a5b615723565b5b8235905067ffffffffffffffff811115615a7957615a78615727565b5b602083019150836020820283011115615a9557615a9461572b565b5b9250929050565b5f805f805f8060808789031215615ab657615ab561549f565b5b5f615ac389828a016154bd565b965050602087013567ffffffffffffffff811115615ae457615ae36154a3565b5b615af089828a01615a47565b9550955050604087013567ffffffffffffffff811115615b1357615b126154a3565b5b615b1f89828a01615a47565b93509350506060615b3289828a016154bd565b9150509295509295509295565b5f805f60408486031215615b5657615b5561549f565b5b5f615b63868287016154bd565b935050602084013567ffffffffffffffff811115615b8457615b836154a3565b5b615b9086828701615784565b92509250509250925092565b615ba5816153bb565b82525050565b615bb481615656565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b60028110615bf457615bf3615603565b5b50565b5f819050615c0482615be3565b919050565b5f615c1382615bf7565b9050919050565b615c2381615c09565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f615c4d82615c29565b615c578185615c33565b9350615c67818560208601615406565b615c708161542e565b840191505092915050565b5f604083015f830151615c905f860182615c1a565b5060208301518482036020860152615ca88282615c43565b9150508091505092915050565b5f615cc08383615c7b565b905092915050565b5f602082019050919050565b5f615cde82615bba565b615ce88185615bc4565b935083602082028501615cfa85615bd4565b805f5b85811015615d355784840389528151615d168582615cb5565b9450615d2183615cc8565b925060208a01995050600181019050615cfd565b50829750879550505050505092915050565b5f608083015f830151615d5c5f860182615b9c565b506020830151615d6f6020860182615b9c565b506040830151615d826040860182615bab565b5060608301518482036060860152615d9a8282615cd4565b9150508091505092915050565b5f6020820190508181035f830152615dbf8184615d47565b905092915050565b5f805f805f60608688031215615de057615ddf61549f565b5b5f615ded888289016154bd565b955050602086013567ffffffffffffffff811115615e0e57615e0d6154a3565b5b615e1a88828901615784565b9450945050604086013567ffffffffffffffff811115615e3d57615e3c6154a3565b5b615e4988828901615784565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b615e8c81615e58565b82525050565b615e9b81615544565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f615ed58383615b9c565b60208301905092915050565b5f602082019050919050565b5f615ef782615ea1565b615f018185615eab565b9350615f0c83615ebb565b805f5b83811015615f3c578151615f238882615eca565b9750615f2e83615ee1565b925050600181019050615f0f565b5085935050505092915050565b5f60e082019050615f5c5f83018a615e83565b8181036020830152615f6e818961543e565b90508181036040830152615f82818861543e565b9050615f9160608301876153c4565b615f9e6080830186615e92565b615fab60a0830185615a1f565b81810360c0830152615fbd8184615eed565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f61600e826153ec565b6160188185615ff4565b9350616028818560208601615406565b6160318161542e565b840191505092915050565b5f6160478383616004565b905092915050565b5f602082019050919050565b5f61606582615fcb565b61606f8185615fd5565b93508360208202850161608185615fe5565b805f5b858110156160bc578484038952815161609d858261603c565b94506160a88361604f565b925060208a01995050600181019050616084565b50829750879550505050505092915050565b5f82825260208201905092915050565b5f6160e882615bba565b6160f281856160ce565b93508360208202850161610485615bd4565b805f5b8581101561613f57848403895281516161208582615cb5565b945061612b83615cc8565b925060208a01995050600181019050616107565b50829750879550505050505092915050565b5f6040820190508181035f830152616169818561605b565b9050818103602083015261617d81846160de565b90509392505050565b5f806040838503121561619c5761619b61549f565b5b5f6161a98582860161569e565b92505060206161ba858286016154bd565b9150509250929050565b5f8083601f8401126161d9576161d8615723565b5b8235905067ffffffffffffffff8111156161f6576161f5615727565b5b6020830191508360208202830111156162125761621161572b565b5b9250929050565b5f805f805f80608087890312156162335761623261549f565b5b5f61624089828a016154bd565b965050602061625189828a016154bd565b955050604087013567ffffffffffffffff811115616272576162716154a3565b5b61627e89828a0161572f565b9450945050606087013567ffffffffffffffff8111156162a1576162a06154a3565b5b6162ad89828a016161c4565b92509250509295509295509295565b5f82825260208201905092915050565b5f6162d682615c29565b6162e081856162bc565b93506162f0818560208601615406565b6162f98161542e565b840191505092915050565b5f6040820190508181035f83015261631c818561605b565b9050818103602083015261633081846162cc565b90509392505050565b5f6020828403121561634e5761634d61549f565b5b5f61635b8482850161569e565b91505092915050565b5f6020820190508181035f83015261637c8184615eed565b905092915050565b5f81905092915050565b5f616398826153ec565b6163a28185616384565b93506163b2818560208601615406565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6163f2600283616384565b91506163fd826163be565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f61643c600183616384565b915061644782616408565b600182019050919050565b5f61645d828761638e565b9150616468826163e6565b9150616474828661638e565b915061647f82616430565b915061648b828561638e565b915061649682616430565b91506164a2828461638e565b915081905095945050505050565b5f815190506164be8161586a565b92915050565b5f602082840312156164d9576164d861549f565b5b5f6164e6848285016164b0565b91505092915050565b5f6020820190506165025f830184615e92565b92915050565b5f67ffffffffffffffff82169050919050565b61652481616508565b82525050565b5f60208201905061653d5f83018461651b565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61657a826153bb565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036165ac576165ab616543565b5b600182019050919050565b5f815190506165c5816154a7565b92915050565b5f80604083850312156165e1576165e061549f565b5b5f6165ee858286016165b7565b92505060206165ff858286016165b7565b9150509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061664d57607f821691505b6020821081036166605761665f616609565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026166c27fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82616687565b6166cc8683616687565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6167076167026166fd846153bb565b6166e4565b6153bb565b9050919050565b5f819050919050565b616720836166ed565b61673461672c8261670e565b848454616693565b825550505050565b5f90565b61674861673c565b616753818484616717565b505050565b5b818110156167765761676b5f82616740565b600181019050616759565b5050565b601f8211156167bb5761678c81616666565b61679584616678565b810160208510156167a4578190505b6167b86167b085616678565b830182616758565b50505b505050565b5f82821c905092915050565b5f6167db5f19846008026167c0565b1980831691505092915050565b5f6167f383836167cc565b9150826002028217905092915050565b61680c82615c29565b67ffffffffffffffff81111561682557616824615898565b5b61682f8254616636565b61683a82828561677a565b5f60209050601f83116001811461686b575f8415616859578287015190505b61686385826167e8565b8655506168ca565b601f19841661687986616666565b5f5b828110156168a05784890151825560018201915060208501945060208101905061687b565b868310156168bd57848901516168b9601f8916826167cc565b8355505b6001600288020188555050505b505050505050565b5f6080820190506168e55f8301876153c4565b6168f260208301866153c4565b6168ff6040830185615667565b818103606083015261691181846162cc565b905095945050505050565b5f60408201905061692f5f8301856153c4565b61693c6020830184615e92565b9392505050565b5f819050919050565b60028110616958575f80fd5b50565b5f813590506169698161694c565b92915050565b5f61697d602084018461695b565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126169ad576169ac61698d565b5b83810192508235915060208301925067ffffffffffffffff8211156169d5576169d4616985565b5b6001820236038313156169eb576169ea616989565b5b509250929050565b5f6169fe8385615c33565b9350616a0b838584615940565b616a148361542e565b840190509392505050565b5f60408301616a305f84018461696f565b616a3c5f860182615c1a565b50616a4a6020840184616991565b8583036020870152616a5d8382846169f3565b925050508091505092915050565b5f616a768383616a1f565b905092915050565b5f82356001604003833603038112616a9957616a9861698d565b5b82810191505092915050565b5f602082019050919050565b5f616abc83856160ce565b935083602084028501616ace84616943565b805f5b87811015616b11578484038952616ae88284616a7e565b616af28582616a6b565b9450616afd83616aa5565b925060208a01995050600181019050616ad1565b50829750879450505050509392505050565b5f616b2e83856162bc565b9350616b3b838584615940565b616b448361542e565b840190509392505050565b5f608082019050616b625f8301896153c4565b8181036020830152616b75818789616ab1565b90508181036040830152616b8a818587616b23565b9050616b996060830184615e92565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f82356001604003833603038112616bf857616bf7616bd1565b5b80830191505092915050565b5f8135616c108161694c565b80915050919050565b5f815f1b9050919050565b5f60ff616c3084616c19565b9350801983169250808416831791505092915050565b5f616c5082615bf7565b9050919050565b5f819050919050565b616c6982616c46565b616c7c616c7582616c57565b8354616c24565b8255505050565b5f8083356001602003843603038112616c9f57616c9e616bd1565b5b80840192508235915067ffffffffffffffff821115616cc157616cc0616bd5565b5b602083019250600182023603831315616cdd57616cdc616bd9565b5b509250929050565b5f82905092915050565b616cf98383616ce5565b67ffffffffffffffff811115616d1257616d11615898565b5b616d1c8254616636565b616d2782828561677a565b5f601f831160018114616d54575f8415616d42578287013590505b616d4c85826167e8565b865550616db3565b601f198416616d6286616666565b5f5b82811015616d8957848901358255600182019150602085019450602081019050616d64565b86831015616da65784890135616da2601f8916826167cc565b8355505b6001600288020188555050505b50505050505050565b616dc7838383616cef565b505050565b5f81015f830180616ddc81616c04565b9050616de88184616c60565b5050506001810160208301616dfd8185616c83565b616e08818386616dbc565b505050505050565b616e1a8282616dcc565b5050565b5f606082019050616e315f8301876153c4565b8181036020830152616e43818661605b565b90508181036040830152616e58818486616ab1565b905095945050505050565b5f80fd5b82818337505050565b5f616e7b8385615eab565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115616eae57616ead616e63565b5b602083029250616ebf838584616e67565b82840190509392505050565b5f60a082019050616ede5f83018a6153c4565b8181036020830152616ef181888a616e70565b90508181036040830152616f06818688616e70565b9050616f1560608301856153c4565b616f2260808301846153c4565b98975050505050505050565b5f606082019050616f415f8301876153c4565b8181036020830152616f54818587616b23565b9050616f636040830184615e92565b95945050505050565b5f60a082019050616f7f5f8301886153c4565b616f8c60208301876153c4565b616f9960408301866153c4565b616fa660608301856156fb565b8181036080830152616fb881846162cc565b90509695505050505050565b5f606082019050616fd75f8301866153c4565b616fe460208301856153c4565b8181036040830152616ff681846162cc565b9050949350505050565b5f6080820190506170135f8301896153c4565b8181036020830152617026818789616b23565b9050818103604083015261703b818587616b23565b905061704a6060830184615e92565b979650505050505050565b5f6060820190506170685f8301876153c4565b818103602083015261707a818661605b565b9050818103604083015261708f818486616b23565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6170ce6015836153f6565b91506170d98261709a565b602082019050919050565b5f6020820190508181035f8301526170fb816170c2565b9050919050565b5f6040820190506171155f8301856153c4565b61712260208301846153c4565b9392505050565b5f819050919050565b5f61713d8385615ff4565b935061714a838584615940565b6171538361542e565b840190509392505050565b5f61716a848484617132565b90509392505050565b5f808335600160200384360303811261718f5761718e61698d565b5b83810192508235915060208301925067ffffffffffffffff8211156171b7576171b6616985565b5b6001820236038313156171cd576171cc616989565b5b509250929050565b5f602082019050919050565b5f6171ec8385615fd5565b9350836020840285016171fe84617129565b805f5b878110156172435784840389526172188284617173565b61722386828461715e565b955061722e846171d5565b935060208b019a505050600181019050617201565b50829750879450505050509392505050565b5f6080820190506172685f8301896153c4565b818103602083015261727b8187896171e1565b90508181036040830152617290818587616ab1565b905061729f60608301846153c4565b979650505050505050565b5f6172b4826153bb565b91506172bf836153bb565b92508282019050808211156172d7576172d6616543565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60ff82169050919050565b5f8160f81b9050919050565b5f61732c82617316565b9050919050565b61734461733f8261730a565b617322565b82525050565b5f819050919050565b61736461735f826153bb565b61734a565b82525050565b5f6173758286617333565b6001820191506173858285617353565b6020820191506173958284617353565b602082019150819050949350505050565b6173af816156f0565b81146173b9575f80fd5b50565b5f815190506173ca816173a6565b92915050565b5f602082840312156173e5576173e461549f565b5b5f6173f2848285016173bc565b91505092915050565b5f602082840312156174105761740f61549f565b5b5f61741d8482850161695b565b91505092915050565b5f81905092915050565b5f61743b8385617426565b9350617448838584615940565b82840190509392505050565b5f617460828486617430565b91508190509392505050565b61747581615c09565b82525050565b5f60608201905061748e5f830186615a1f565b61749b602083018561746c565b6174a86040830184615a1f565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b6174dc81615a16565b82525050565b5f6174ed83836174d3565b60208301905092915050565b5f602082019050919050565b5f61750f826174b0565b61751981856174ba565b9350617524836174c4565b805f5b8381101561755457815161753b88826174e2565b9750617546836174f9565b925050600181019050617527565b5085935050505092915050565b5f61756c8284617505565b915081905092915050565b5f60a08201905061758a5f830188615a1f565b61759760208301876153c4565b6175a460408301866153c4565b6175b16060830185615a1f565b6175be6080830184615a1f565b9695505050505050565b5f602082840312156175dd576175dc61549f565b5b5f6175ea848285016165b7565b91505092915050565b5f80fd5b5f80fd5b5f67ffffffffffffffff82111561761557617614615898565b5b61761e8261542e565b9050602081019050919050565b5f61763d617638846175fb565b6158f6565b90508281526020810184848401111561765957617658615894565b5b617664848285615406565b509392505050565b5f82601f8301126176805761767f615723565b5b815161769084826020860161762b565b91505092915050565b5f608082840312156176ae576176ad6175f3565b5b6176b860806158f6565b90505f6176c7848285016164b0565b5f8301525060206176da848285016164b0565b602083015250604082015167ffffffffffffffff8111156176fe576176fd6175f7565b5b61770a8482850161766c565b604083015250606082015167ffffffffffffffff81111561772e5761772d6175f7565b5b61773a8482850161766c565b60608301525092915050565b5f6020828403121561775b5761775a61549f565b5b5f82015167ffffffffffffffff811115617778576177776154a3565b5b61778484828501617699565b91505092915050565b61779681615a16565b81146177a0575f80fd5b50565b5f815190506177b18161778d565b92915050565b5f602082840312156177cc576177cb61549f565b5b5f6177d9848285016177a3565b91505092915050565b5f6060820190506177f55f830186615a1f565b61780260208301856153c4565b61780f6040830184615a1f565b949350505050565b6178208161730a565b82525050565b5f6020820190506178395f830184617817565b92915050565b5f6060820190506178525f8301866153c4565b61785f6020830185615667565b818103604083015261787181846162cc565b9050949350505050565b5f819050815f5260205f209050919050565b601f8211156178ce5761789f8161787b565b6178a884616678565b810160208510156178b7578190505b6178cb6178c385616678565b830182616758565b50505b505050565b6178dc826153ec565b67ffffffffffffffff8111156178f5576178f4615898565b5b6178ff8254616636565b61790a82828561788d565b5f60209050601f83116001811461793b575f8415617929578287015190505b61793385826167e8565b86555061799a565b601f1984166179498661787b565b5f5b828110156179705784890151825560018201915060208501945060208101905061794b565b8683101561798d5784890151617989601f8916826167cc565b8355505b6001600288020188555050505b505050505050565b5f6040820190506179b55f830185615e92565b6179c26020830184615e92565b9392505050565b5f6179d382615c29565b6179dd8185617426565b93506179ed818560208601615406565b80840191505092915050565b5f617a0482846179c9565b915081905092915050565b5f60a082019050617a225f830188615a1f565b617a2f6020830187615a1f565b617a3c6040830186615a1f565b617a4960608301856153c4565b617a566080830184615e92565b9695505050505050565b5f608082019050617a735f830187615a1f565b617a806020830186617817565b617a8d6040830185615a1f565b617a9a6060830184615a1f565b9594505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP4\x80\x15b\0\0CW_\x80\xFD[Pb\0\0Tb\0\0Z` \x1B` \x1CV[b\0\x01\xC4V[_b\0\0kb\0\x01^` \x1B` \x1CV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15b\0\0\xB6W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14b\0\x01[Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@Qb\0\x01R\x91\x90b\0\x01\xA9V[`@Q\x80\x91\x03\x90\xA1[PV[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[b\0\x01\xA3\x81b\0\x01\x85V[\x82RPPV[_` \x82\x01\x90Pb\0\x01\xBE_\x83\x01\x84b\0\x01\x98V[\x92\x91PPV[`\x80Qa{\xDDb\0\x01\xEB_9_\x81\x81a=\xE0\x01R\x81\x81a>5\x01Ra@\xD7\x01Ra{\xDD_\xF3\xFE`\x80`@R`\x046\x10a\x01\xD7W_5`\xE0\x1C\x80cb\x94\xF4b\x11a\x01\x01W\x80c\xC2\xC1\xFA\xEE\x11a\0\x94W\x80c\xD5/\x10\xEB\x11a\0cW\x80c\xD5/\x10\xEB\x14a\x06\x85W\x80c\xDA\xBDs/\x14a\x06\xAFW\x80c\xE4\x10\x11~\x14a\x06\xD9W\x80c\xF0\xF8\xCB\xC6\x14a\x07\x03Wa\x01\xD7V[\x80c\xC2\xC1\xFA\xEE\x14a\x05\xE2W\x80c\xC4\x11Xt\x14a\x06\nW\x80c\xC5[\x87$\x14a\x06 W\x80c\xCA\xA3g\xDB\x14a\x06]Wa\x01\xD7V[\x80c\xAA\xA4p\x16\x11a\0\xD0W\x80c\xAA\xA4p\x16\x14a\x05>W\x80c\xAD<\xB1\xCC\x14a\x05fW\x80c\xB5;<\xCC\x14a\x05\x90W\x80c\xBA\xFF!\x1E\x14a\x05\xB8Wa\x01\xD7V[\x80cb\x94\xF4b\x14a\x04mW\x80cb\x97\x87\x87\x14a\x04\xA9W\x80c\x84\xB0\x19n\x14a\x04\xD1W\x80c\x93f\x08\xAE\x14a\x05\x01Wa\x01\xD7V[\x80c<\x02\xF84\x11a\x01yW\x80cO\x1E\xF2\x86\x11a\x01HW\x80cO\x1E\xF2\x86\x14a\x03\xD7W\x80cR\xD1\x90-\x14a\x03\xF3W\x80cV\xA6\x10\xB4\x14a\x04\x1DW\x80cX\x9A\xDB\x0E\x14a\x04EWa\x01\xD7V[\x80c<\x02\xF84\x14a\x03\x0FW\x80c=^\xC7\xE3\x14a\x037W\x80cE\xAF&\x1B\x14a\x03sW\x80cF\x10\xFF\xE8\x14a\x03\xAFWa\x01\xD7V[\x80c\x17\x03\xC6\x1A\x11a\x01\xB5W\x80c\x17\x03\xC6\x1A\x14a\x02kW\x80c\x19\xF4\xF62\x14a\x02\x93W\x80c9\xF78\x10\x14a\x02\xCFW\x80c:\xC5\0r\x14a\x02\xE5Wa\x01\xD7V[\x80c\x0Bh\x073\x14a\x01\xDBW\x80c\r\x8En,\x14a\x02\x05W\x80c\x16\xC7\x13\xD9\x14a\x02/W[_\x80\xFD[4\x80\x15a\x01\xE6W_\x80\xFD[Pa\x01\xEFa\x07?V[`@Qa\x01\xFC\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x10W_\x80\xFD[Pa\x02\x19a\x07VV[`@Qa\x02&\x91\x90aTvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02:W_\x80\xFD[Pa\x02U`\x04\x806\x03\x81\x01\x90a\x02P\x91\x90aT\xD1V[a\x07\xD1V[`@Qa\x02b\x91\x90aU\xE3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02vW_\x80\xFD[Pa\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90aT\xD1V[a\x08\xA2V[\0[4\x80\x15a\x02\x9EW_\x80\xFD[Pa\x02\xB9`\x04\x806\x03\x81\x01\x90a\x02\xB4\x91\x90aT\xD1V[a\n\xD0V[`@Qa\x02\xC6\x91\x90aVvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xDAW_\x80\xFD[Pa\x02\xE3a\x0B\xD6V[\0[4\x80\x15a\x02\xF0W_\x80\xFD[Pa\x02\xF9a\x0E&V[`@Qa\x03\x06\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x1AW_\x80\xFD[Pa\x035`\x04\x806\x03\x81\x01\x90a\x030\x91\x90aV\xB2V[a\x0E=V[\0[4\x80\x15a\x03BW_\x80\xFD[Pa\x03]`\x04\x806\x03\x81\x01\x90a\x03X\x91\x90aT\xD1V[a\x114V[`@Qa\x03j\x91\x90aW\nV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03~W_\x80\xFD[Pa\x03\x99`\x04\x806\x03\x81\x01\x90a\x03\x94\x91\x90aT\xD1V[a\x11hV[`@Qa\x03\xA6\x91\x90aVvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xBAW_\x80\xFD[Pa\x03\xD5`\x04\x806\x03\x81\x01\x90a\x03\xD0\x91\x90aW\xD9V[a\x12VV[\0[a\x03\xF1`\x04\x806\x03\x81\x01\x90a\x03\xEC\x91\x90aY\xBCV[a\x17\xA6V[\0[4\x80\x15a\x03\xFEW_\x80\xFD[Pa\x04\x07a\x17\xC5V[`@Qa\x04\x14\x91\x90aZ.V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04(W_\x80\xFD[Pa\x04C`\x04\x806\x03\x81\x01\x90a\x04>\x91\x90aZ\x9CV[a\x17\xF6V[\0[4\x80\x15a\x04PW_\x80\xFD[Pa\x04k`\x04\x806\x03\x81\x01\x90a\x04f\x91\x90a[?V[a\x19\xD4V[\0[4\x80\x15a\x04xW_\x80\xFD[Pa\x04\x93`\x04\x806\x03\x81\x01\x90a\x04\x8E\x91\x90aT\xD1V[a\x1D\x90V[`@Qa\x04\xA0\x91\x90a]\xA7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xB4W_\x80\xFD[Pa\x04\xCF`\x04\x806\x03\x81\x01\x90a\x04\xCA\x91\x90a]\xC7V[a\x1F\xFDV[\0[4\x80\x15a\x04\xDCW_\x80\xFD[Pa\x04\xE5a$*V[`@Qa\x04\xF8\x97\x96\x95\x94\x93\x92\x91\x90a_IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x0CW_\x80\xFD[Pa\x05'`\x04\x806\x03\x81\x01\x90a\x05\"\x91\x90aT\xD1V[a%3V[`@Qa\x055\x92\x91\x90aaQV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05IW_\x80\xFD[Pa\x05d`\x04\x806\x03\x81\x01\x90a\x05_\x91\x90aa\x86V[a(\x99V[\0[4\x80\x15a\x05qW_\x80\xFD[Pa\x05za*]V[`@Qa\x05\x87\x91\x90aTvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x9BW_\x80\xFD[Pa\x05\xB6`\x04\x806\x03\x81\x01\x90a\x05\xB1\x91\x90ab\x19V[a*\x96V[\0[4\x80\x15a\x05\xC3W_\x80\xFD[Pa\x05\xCCa,\xECV[`@Qa\x05\xD9\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xEDW_\x80\xFD[Pa\x06\x08`\x04\x806\x03\x81\x01\x90a\x06\x03\x91\x90aT\xD1V[a-\x03V[\0[4\x80\x15a\x06\x15W_\x80\xFD[Pa\x06\x1Ea/|V[\0[4\x80\x15a\x06+W_\x80\xFD[Pa\x06F`\x04\x806\x03\x81\x01\x90a\x06A\x91\x90aT\xD1V[a1\xC5V[`@Qa\x06T\x92\x91\x90ac\x04V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06hW_\x80\xFD[Pa\x06\x83`\x04\x806\x03\x81\x01\x90a\x06~\x91\x90ac9V[a4\x96V[\0[4\x80\x15a\x06\x90W_\x80\xFD[Pa\x06\x99a5\x94V[`@Qa\x06\xA6\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xBAW_\x80\xFD[Pa\x06\xC3a5\xABV[`@Qa\x06\xD0\x91\x90acdV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xE4W_\x80\xFD[Pa\x06\xEDa6\x0FV[`@Qa\x06\xFA\x91\x90acdV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x0EW_\x80\xFD[Pa\x07)`\x04\x806\x03\x81\x01\x90a\x07$\x91\x90aT\xD1V[a6sV[`@Qa\x076\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[_\x80a\x07Ia6\xA5V[\x90P\x80`\x05\x01T\x91PP\x90V[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x07\x97_a6\xCCV[a\x07\xA1`\x02a6\xCCV[a\x07\xAA_a6\xCCV[`@Q` \x01a\x07\xBD\x94\x93\x92\x91\x90adRV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x07\xDCa6\xA5V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\x94W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08KW[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xFFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t#\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\x92W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x89\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a\t\x9Ba6\xA5V[\x90P\x80`\t\x01T\x82\x11\x80a\t\xC6WP`\xF8`\x05`\x08\x81\x11\x15a\t\xC0Wa\t\xBFaV\x03V[[\x90\x1B\x82\x11\x15[\x15a\n\x08W\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFF\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\njW\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\na\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\n\xC4\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\n\xDAa6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0B=W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B4\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x0B\x96W\x82`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\x8D\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x0B\xE0a7\x96V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C!W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x0C,a7\xBAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0CtWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0C\xABW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\rd`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7\xE1V[_a\rma6\xA5V[\x90P`\xF8`\x03`\x08\x81\x11\x15a\r\x85Wa\r\x84aV\x03V[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x08\x81\x11\x15a\r\xA5Wa\r\xA4aV\x03V[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\x08\x81\x11\x15a\r\xC5Wa\r\xC4aV\x03V[[\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0E\x1A\x91\x90ae*V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x0E0a6\xA5V[\x90P\x80`\t\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x9AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xBE\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F-W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F$\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a\x0F6a6\xA5V[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\x08\x81\x11\x15a\x0FVWa\x0FUaV\x03V[[\x90\x1B\x81\x14\x15\x80\x15a\x0F\x84WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0F\xC6W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xBD\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0F\xDA\x90aepV[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x104Wa\x103aV\x03V[[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x97W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xBB\x91\x90ae\xCBV[\x91P\x91P_a\x10\xCA\x83\x83a7\xF7V[\x90P\x80\x86`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a\x10\xEC\x91\x90ah\x03V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x84\x89\x89\x84`@Qa\x11\"\x94\x93\x92\x91\x90ah\xD2V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x11>a6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x11ra6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x11\xD5W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xCC\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x12.W\x82`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12%\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_a\x12_a6\xA5V[\x90P\x80`\x05\x01T\x86\x11\x80a\x12\x8AWP`\xF8`\x04`\x08\x81\x11\x15a\x12\x84Wa\x12\x83aV\x03V[[\x90\x1B\x86\x11\x15[\x15a\x12\xCCW\x85`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xC3\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x13\x13W\x85`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\n\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x13\x1E\x88a8&V[\x91P\x91P_\x83`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P\x83`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x13\x8FW`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x9D\x82\x8B\x8B\x8B\x88a9\xB0V[\x90P_a\x13\xAC\x84\x83\x8A\x8Aa;\x91V[\x90P\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x14LW\x8A\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14C\x92\x91\x90ai\x1CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x86_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x86`\x02\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8C\x8C\x8C\x8C\x8C3`@Qa\x15n\x96\x95\x94\x93\x92\x91\x90akOV[`@Q\x80\x91\x03\x90\xA1\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x15\xADWPa\x15\xAC\x85\x82\x80T\x90Pa;\xF9V[[\x15a\x17\x98W`\x01\x87`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8B\x8B\x90P\x81\x10\x15a\x16cW\x87`\x07\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x8C\x8C\x83\x81\x81\x10a\x16\x10Wa\x16\x0Fak\xA4V[[\x90P` \x02\x81\x01\x90a\x16\"\x91\x90ak\xDDV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x16T\x91\x90an\x10V[PP\x80\x80`\x01\x01\x91PPa\x15\xDFV[P\x82\x87`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86`\x0F\x01\x8C\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU_\x87`\x11\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x01T\x03a\x17\x97W\x8B\x87`\x08\x01\x81\x90UP_a\x17V\x86\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x17LW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x17\x03W[PPPPPa<\x96V[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8D\x82\x8E\x8E`@Qa\x17\x8D\x94\x93\x92\x91\x90an\x1EV[`@Q\x80\x91\x03\x90\xA1P[[PPPPPPPPPPPPV[a\x17\xAEa=\xDEV[a\x17\xB7\x82a>\xC4V[a\x17\xC1\x82\x82a?\xB7V[PPV[_a\x17\xCEa@\xD5V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18SW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18w\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xE6W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xDD\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a\x18\xEFa6\xA5V[\x90P\x83\x83\x90P\x86\x86\x90P\x14a\x190W`@Q\x7F\x89K*\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`\x12\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x03a\x19\x87W\x86`@Q\x7F\x05\xB0\x83\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19~\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x7F\x8B\xFA}\x0E\xD6\xF8~Rkb4)\x18\xEE{\xFAS\x95+\xAD\xD4c\xDC\x93@T\xD7\xDD\x94\x0E\xAF\xDC\x87\x87\x87\x87\x87\x87`\x01`@Qa\x19\xC3\x97\x96\x95\x94\x93\x92\x91\x90an\xCBV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_a\x19\xDDa6\xA5V[\x90P\x80`\x04\x01T\x84\x11\x80a\x1A\x08WP`\xF8`\x03`\x08\x81\x11\x15a\x1A\x02Wa\x1A\x01aV\x03V[[\x90\x1B\x84\x11\x15[\x15a\x1AJW\x83`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1AA\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x1AU\x86a8&V[\x91P\x91P_a\x1Ad\x87\x84aA\\V[\x90P_a\x1As\x83\x83\x89\x89a;\x91V[\x90P\x84_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1B\x13W\x87\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\n\x92\x91\x90ai\x1CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x89\x89\x893`@Qa\x1C1\x94\x93\x92\x91\x90ao.V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1CpWPa\x1Co\x84\x82\x80T\x90Pa;\xF9V[[\x15a\x1D\x85W`\x01\x86`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x86`\x03\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x86`\x06\x01_\x8B\x81R` \x01\x90\x81R` \x01_ T\x90P_\x87`\x11\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x90P_\x81_\x01T\x14a\x1DFW\x7F\xE4S\xC2\x9CF\xCC\xC7fL\x03\x98\xE8FM[\xB4!\xE9\x95C-\xAFU\x06\xA3\xFD\xBCj\xA0\x96j\x93\x8B\x83\x83_\x01T\x84`\x01\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x8B`@Qa\x1D9\x95\x94\x93\x92\x91\x90aolV[`@Q\x80\x91\x03\x90\xA1a\x1D\x82V[\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8B\x83\x89`@Qa\x1Dy\x93\x92\x91\x90ao\xC4V[`@Q\x80\x91\x03\x90\xA1[PP[PPPPPPPPPV[a\x1D\x98aS\x84V[_a\x1D\xA1a6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1E\x04W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xFB\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x1E]W\x82`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1ET\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P`@Q\x80`\x80\x01`@R\x80\x82\x81R` \x01\x85\x81R` \x01\x83`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1E\xBDWa\x1E\xBCaV\x03V[[\x81R` \x01\x83`\x07\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1F\xEDW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1F8Wa\x1F7aV\x03V[[`\x01\x81\x11\x15a\x1FJWa\x1FIaV\x03V[[\x81R` \x01`\x01\x82\x01\x80Ta\x1F^\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\x8A\x90af6V[\x80\x15a\x1F\xD5W\x80`\x1F\x10a\x1F\xACWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xD5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xF4V[PPPP\x81RP\x92PPP\x91\x90PV[_a \x06a6\xA5V[\x90P\x80`\t\x01T\x86\x11\x80a 1WP`\xF8`\x05`\x08\x81\x11\x15a +Wa *aV\x03V[[\x90\x1B\x86\x11\x15[\x15a sW\x85`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a j\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80a ~\x88a8&V[\x91P\x91P_a \xA3\x89\x85`\n\x01_\x8C\x81R` \x01\x90\x81R` \x01_ T\x8A\x8A\x87aA\xBEV[\x90P_a \xB2\x83\x83\x89\x89a;\x91V[\x90P\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a!RW\x89\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!I\x92\x91\x90ai\x1CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8B\x8B\x8B\x8B\x8B3`@Qa\"t\x96\x95\x94\x93\x92\x91\x90ap\0V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\"\xB3WPa\"\xB2\x84\x82\x80T\x90Pa;\xF9V[[\x15a$\x1DW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a#\x05\x92\x91\x90al\xEFV[P\x82\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP\x85`\x10\x01\x8B\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU_a#\xDC\x85\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a#\xD2W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a#\x89W[PPPPPa<\x96V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa$\x13\x94\x93\x92\x91\x90apUV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[_``\x80_\x80_``_a$<aBOV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a$WWP_\x80\x1B\x81`\x01\x01T\x14[a$\x96W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x8D\x90ap\xE4V[`@Q\x80\x91\x03\x90\xFD[a$\x9EaBvV[a$\xA6aC\x14V[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xC5Wa$\xC4aX\x98V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$\xF3W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a%?a6\xA5V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a%\xA2W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\x99\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a%\xFFW\x84`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xF6\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a&\x9FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a&VW[PPPPP\x90P_a'I\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta&\xC8\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xF4\x90af6V[\x80\x15a'?W\x80`\x1F\x10a'\x16Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'?V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaC\xB2V[\x90P_a'V\x82\x84a<\x96V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a(\x85W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a'\xD0Wa'\xCFaV\x03V[[`\x01\x81\x11\x15a'\xE2Wa'\xE1aV\x03V[[\x81R` \x01`\x01\x82\x01\x80Ta'\xF6\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\"\x90af6V[\x80\x15a(mW\x80`\x1F\x10a(DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(mV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(PW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a'\x8CV[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xF6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x1A\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)\x89W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\x80\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a)\x92a6\xA5V[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a)\xF5W\x81`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xEC\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_a)\xFF\x84aE\xA1V[\x91PP`@Q\x80`@\x01`@R\x80\x84\x81R` \x01`\x01\x15\x15\x81RP\x82`\x11\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x90PPPPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xF3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x17\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a+\x86W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+}\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a+\x8Fa6\xA5V[\x90P\x86\x81`\x11\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x01T\x14a+\xECW\x85\x87`@Q\x7F\x941\xF3N\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+\xE3\x92\x91\x90aq\x02V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a,BW`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a,\x89W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a,\x80\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x85\x81`\x12\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x7F\xA4vd\x86\x1A\xB5\x8C[\xD5\x04\x0E\x9C\xC4^h\xD0\xE4\x8E\xC0Cq\x03_\xD7P\x99\xE2\x17\xE0\xA6\xAA\x81\x87\x84\x84\x88\x88`\x01`@Qa,\xDB\x96\x95\x94\x93\x92\x91\x90arUV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a,\xF6a6\xA5V[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x84\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a-\xF3W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xEA\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a-\xFCa6\xA5V[\x90P\x80`\x04\x01T\x82\x11\x80a.'WP`\xF8`\x03`\x08\x81\x11\x15a.!Wa. aV\x03V[[\x90\x1B\x82\x11\x15[\x15a.iW\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.`\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a.\xE2W\x82`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xD9\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81\x14a/@W`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa/o\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xA1PPPV[`\x03_a/\x87a7\xBAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a/\xCFWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a0\x06W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a0Ta6\xA5V[\x90P_`\x01`\xF8`\x04`\x08\x81\x11\x15a0oWa0naV\x03V[[\x90\x1Ba0{\x91\x90ar\xAAV[\x90P[\x81`\x05\x01T\x81\x11a0\xE1W_\x80\x1B\x82`\x03\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x14a0\xCEW\x81`\x0F\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU[\x80\x80a0\xD9\x90aepV[\x91PPa0~V[P_`\x01`\xF8`\x05`\x08\x81\x11\x15a0\xFBWa0\xFAaV\x03V[[\x90\x1Ba1\x07\x91\x90ar\xAAV[\x90P[\x81`\t\x01T\x81\x11a1mW_\x80\x1B\x82`\x03\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x14a1ZW\x81`\x10\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU[\x80\x80a1e\x90aepV[\x91PPa1\nV[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa1\xB9\x91\x90ae*V[`@Q\x80\x91\x03\x90\xA1PPV[``\x80_a1\xD1a6\xA5V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a24W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2+\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a2\x91W\x84`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\x88\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a31W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a2\xE8W[PPPPP\x90P_a3\xDB\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta3Z\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3\x86\x90af6V[\x80\x15a3\xD1W\x80`\x1F\x10a3\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3\xD1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3\xB4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaC\xB2V[\x90P_a3\xE8\x82\x84a<\x96V[\x90P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta4\n\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta46\x90af6V[\x80\x15a4\x81W\x80`\x1F\x10a4XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4\x81V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a4dW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xF3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\x17\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a5\x86W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5}\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[a5\x8F\x81aE\xA1V[PPPV[_\x80a5\x9Ea6\xA5V[\x90P\x80`\x08\x01T\x91PP\x90V[``_a5\xB6a6\xA5V[\x90P\x80`\x10\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a6\x04W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a5\xF0W[PPPPP\x91PP\x90V[``_a6\x1Aa6\xA5V[\x90P\x80`\x0F\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a6hW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a6TW[PPPPP\x91PP\x90V[_\x80a6}a6\xA5V[`\x12\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x03a6\x9BW_a6\x9EV[`\x01[\x90P\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[``_`\x01a6\xDA\x84aH\0V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xF8Wa6\xF7aX\x98V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a7*W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a7\x8BW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a7\x80Wa7\x7Far\xDDV[[\x04\x94P_\x85\x03a77W[\x81\x93PPPP\x91\x90PV[_a7\x9Fa7\xBAV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a7\xE9aIQV[a7\xF3\x82\x82aI\x91V[PPV[```\x02\x83\x83`@Q` \x01a8\x0F\x93\x92\x91\x90asjV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_\x80a82a6\xA5V[\x90P\x80`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80Ta8R\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta8~\x90af6V[\x80\x15a8\xC9W\x80`\x1F\x10a8\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a8\xC9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a8\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x92Pa8\xD9\x83aC\xB2V[\x91PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9*\x92\x91\x90ai\x1CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9i\x91\x90as\xD0V[a9\xAAW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\xA1\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[P\x91P\x91V[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xCFWa9\xCEaX\x98V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9\xFDW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15a;\x01W`@Q\x80``\x01`@R\x80`%\x81R` \x01a{\xB8`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a:@Wa:?ak\xA4V[[\x90P` \x02\x81\x01\x90a:R\x91\x90ak\xDDV[_\x01` \x81\x01\x90a:c\x91\x90as\xFBV[\x87\x87\x84\x81\x81\x10a:vWa:uak\xA4V[[\x90P` \x02\x81\x01\x90a:\x88\x91\x90ak\xDDV[\x80` \x01\x90a:\x97\x91\x90al\x83V[`@Qa:\xA5\x92\x91\x90atTV[`@Q\x80\x91\x03\x90 `@Q` \x01a:\xBF\x93\x92\x91\x90at{V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a:\xE8Wa:\xE7ak\xA4V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa:\x02V[Pa;\x85`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01a{6`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a;8\x91\x90auaV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01a;j\x95\x94\x93\x92\x91\x90auwV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\xE2V[\x91PP\x95\x94PPPPPV[_\x80a;\xE0\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaI\xFBV[\x90Pa;\xED\x86\x823aJ%V[\x80\x91PP\x94\x93PPPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cA\xAD\x06\x9C\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<H\x91\x90aS\xD3V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<cW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x87\x91\x90au\xC8V[\x90P\x80\x83\x10\x15\x91PP\x92\x91PPV[``_\x82Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xB8Wa<\xB7aX\x98V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\xEBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a<\xD6W\x90P[P\x90P_[\x82\x81\x10\x15a=\xD2WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a=<Wa=;ak\xA4V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=a\x92\x91\x90ai\x1CV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a={W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xA3\x91\x90awFV[``\x01Q\x82\x82\x81Q\x81\x10a=\xBAWa=\xB9ak\xA4V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa<\xF0V[P\x80\x92PPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a>\x8BWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a>raL\x06V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a>\xC2W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?!W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?E\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a?\xB4W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xAB\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a@\x1FWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x1C\x91\x90aw\xB7V[`\x01[a@`W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@W\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a@\xC6W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@\xBD\x91\x90aZ.V[`@Q\x80\x91\x03\x90\xFD[a@\xD0\x83\x83aLYV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aAZW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aA\xB6`@Q\x80``\x01`@R\x80`<\x81R` \x01az\xA4`<\x919\x80Q\x90` \x01 \x84\x84\x80Q\x90` \x01 `@Q` \x01aA\x9B\x93\x92\x91\x90aw\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\xE2V[\x90P\x92\x91PPV[_aBD`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01az\xE0`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01aA\xF7\x92\x91\x90atTV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01aB)\x95\x94\x93\x92\x91\x90auwV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\xE2V[\x90P\x95\x94PPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_aB\x81aBOV[\x90P\x80`\x02\x01\x80TaB\x92\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaB\xBE\x90af6V[\x80\x15aC\tW\x80`\x1F\x10aB\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aC\tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aB\xECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aC\x1FaBOV[\x90P\x80`\x03\x01\x80TaC0\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaC\\\x90af6V[\x80\x15aC\xA7W\x80`\x1F\x10aC~Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aC\xA7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aC\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80\x82Q\x14\x80aC\xE4WP_\x82_\x81Q\x81\x10aC\xD1WaC\xD0ak\xA4V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x14[\x15aDqWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDj\x91\x90au\xC8V[\x90PaE\x9CV[_\x82_\x81Q\x81\x10aD\x85WaD\x84ak\xA4V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\x01`\xFF\x16\x81`\xFF\x16\x14\x15\x80\x15aD\xB3WP`\x02`\xFF\x16\x81`\xFF\x16\x14\x15[\x15aD\xF5W\x80`@Q\x7F!9\xCC,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aD\xEC\x91\x90ax&V[`@Q\x80\x91\x03\x90\xFD[`\x01`\xFF\x16\x81`\xFF\x16\x14\x80\x15aE\rWP`!\x83Q\x14\x15[\x15aEDW`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\xFF\x16\x81`\xFF\x16\x14\x80\x15aE\\WP`A\x83Q\x14\x15[\x15aE\x93W`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!\x83\x01Q\x91PP[\x91\x90PV[_\x80_aE\xACa6\xA5V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\x08\x81\x11\x15aE\xCCWaE\xCBaV\x03V[[\x90\x1B\x81\x14\x15\x80\x15aE\xFAWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15aF<W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aF3\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90aFP\x90aepV[\x91\x90PUP\x81`\x04\x01T\x93P\x81`\x05\x01_\x81T\x80\x92\x91\x90aFp\x90aepV[\x91\x90PUP\x81`\x05\x01T\x92P\x82\x82`\x06\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x82`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x84\x82`\r\x01_\x86\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15aF\xE1WaF\xE0aV\x03V[[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aGDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGh\x91\x90ae\xCBV[\x91P\x91P_aGw\x83\x83a7\xF7V[\x90P\x80\x85`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x90\x81aG\x99\x91\x90ah\x03V[P\x80\x85`\x0E\x01_\x88\x81R` \x01\x90\x81R` \x01_ \x90\x81aG\xBA\x91\x90ah\x03V[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x87\x89\x83`@QaG\xEE\x93\x92\x91\x90ax?V[`@Q\x80\x91\x03\x90\xA1PPPPP\x91P\x91V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aH\\Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aHRWaHQar\xDDV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aH\x99Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aH\x8FWaH\x8Ear\xDDV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aH\xC8Wf#\x86\xF2o\xC1\0\0\x83\x81aH\xBEWaH\xBDar\xDDV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aH\xF1Wc\x05\xF5\xE1\0\x83\x81aH\xE7WaH\xE6ar\xDDV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aI\x16Wa'\x10\x83\x81aI\x0CWaI\x0Bar\xDDV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aI9W`d\x83\x81aI/WaI.ar\xDDV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aIHW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aIYaL\xCBV[aI\x8FW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aI\x99aIQV[_aI\xA2aBOV[\x90P\x82\x81`\x02\x01\x90\x81aI\xB5\x91\x90ax\xD3V[P\x81\x81`\x03\x01\x90\x81aI\xC7\x91\x90ax\xD3V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_aI\xF4aI\xEEaL\xE9V[\x83aL\xF7V[\x90P\x91\x90PV[_\x80_\x80aJ\t\x86\x86aM7V[\x92P\x92P\x92PaJ\x19\x82\x82aM\x8CV[\x82\x93PPPP\x92\x91PPV[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJt\x92\x91\x90ai\x1CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\x8FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\xB3\x91\x90as\xD0V[aJ\xF6W\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\xED\x92\x91\x90ay\xA2V[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aKF\x92\x91\x90ai\x1CV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK`W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x88\x91\x90awFV[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aL\0W\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aK\xF7\x92\x91\x90ay\xA2V[`@Q\x80\x91\x03\x90\xFD[PPPPV[_aL2\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\xEEV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aLb\x82aN\xF7V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aL\xBEWaL\xB8\x82\x82aO\xC0V[PaL\xC7V[aL\xC6aP@V[[PPV[_aL\xD4a7\xBAV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_aL\xF2aP|V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aMwW_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaMi\x88\x82\x85\x85aP\xDFV[\x95P\x95P\x95PPPPaM\x85V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aM\x9FWaM\x9EaV\x03V[[\x82`\x03\x81\x11\x15aM\xB2WaM\xB1aV\x03V[[\x03\x15aN\xEAW`\x01`\x03\x81\x11\x15aM\xCCWaM\xCBaV\x03V[[\x82`\x03\x81\x11\x15aM\xDFWaM\xDEaV\x03V[[\x03aN\x16W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aN*WaN)aV\x03V[[\x82`\x03\x81\x11\x15aN=WaN<aV\x03V[[\x03aN\x81W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aNx\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aN\x94WaN\x93aV\x03V[[\x82`\x03\x81\x11\x15aN\xA7WaN\xA6aV\x03V[[\x03aN\xE9W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\xE0\x91\x90aZ.V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aORW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aOI\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[\x80aO~\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\xEEV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaO\xE9\x91\x90ay\xF9V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aP!W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aP&V[``\x91P[P\x91P\x91PaP6\x85\x83\x83aQ\xC6V[\x92PPP\x92\x91PPV[_4\x11\x15aPzW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaP\xA6aRSV[aP\xAEaR\xC9V[F0`@Q` \x01aP\xC4\x95\x94\x93\x92\x91\x90az\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aQ\x1BW_`\x03\x85\x92P\x92P\x92PaQ\xBCV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaQ>\x94\x93\x92\x91\x90az`V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aQ^W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aQ\xAFW_`\x01_\x80\x1B\x93P\x93P\x93PPaQ\xBCV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aQ\xDBWaQ\xD6\x82aS@V[aRKV[_\x82Q\x14\x80\x15aR\x01WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aRCW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aR:\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaRLV[[\x93\x92PPPV[_\x80aR]aBOV[\x90P_aRhaBvV[\x90P_\x81Q\x11\x15aR\x84W\x80\x80Q\x90` \x01 \x92PPPaR\xC6V[_\x82_\x01T\x90P_\x80\x1B\x81\x14aR\x9FW\x80\x93PPPPaR\xC6V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aR\xD3aBOV[\x90P_aR\xDEaC\x14V[\x90P_\x81Q\x11\x15aR\xFAW\x80\x80Q\x90` \x01 \x92PPPaS=V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aS\x16W\x80\x93PPPPaS=V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aSRW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01\x81\x11\x15aS\xAEWaS\xADaV\x03V[[\x81R` \x01``\x81RP\x90V[_\x81\x90P\x91\x90PV[aS\xCD\x81aS\xBBV[\x82RPPV[_` \x82\x01\x90PaS\xE6_\x83\x01\x84aS\xC4V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aT#W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaT\x08V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aTH\x82aS\xECV[aTR\x81\x85aS\xF6V[\x93PaTb\x81\x85` \x86\x01aT\x06V[aTk\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT\x8E\x81\x84aT>V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[aT\xB0\x81aS\xBBV[\x81\x14aT\xBAW_\x80\xFD[PV[_\x815\x90PaT\xCB\x81aT\xA7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aT\xE6WaT\xE5aT\x9FV[[_aT\xF3\x84\x82\x85\x01aT\xBDV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aUN\x82aU%V[\x90P\x91\x90PV[aU^\x81aUDV[\x82RPPV[_aUo\x83\x83aUUV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aU\x91\x82aT\xFCV[aU\x9B\x81\x85aU\x06V[\x93PaU\xA6\x83aU\x16V[\x80_[\x83\x81\x10\x15aU\xD6W\x81QaU\xBD\x88\x82aUdV[\x97PaU\xC8\x83aU{V[\x92PP`\x01\x81\x01\x90PaU\xA9V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU\xFB\x81\x84aU\x87V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aVAWaV@aV\x03V[[PV[_\x81\x90PaVQ\x82aV0V[\x91\x90PV[_aV`\x82aVDV[\x90P\x91\x90PV[aVp\x81aVVV[\x82RPPV[_` \x82\x01\x90PaV\x89_\x83\x01\x84aVgV[\x92\x91PPV[`\x02\x81\x10aV\x9BW_\x80\xFD[PV[_\x815\x90PaV\xAC\x81aV\x8FV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aV\xC8WaV\xC7aT\x9FV[[_aV\xD5\x85\x82\x86\x01aT\xBDV[\x92PP` aV\xE6\x85\x82\x86\x01aV\x9EV[\x91PP\x92P\x92\x90PV[_\x81\x15\x15\x90P\x91\x90PV[aW\x04\x81aV\xF0V[\x82RPPV[_` \x82\x01\x90PaW\x1D_\x83\x01\x84aV\xFBV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aWDWaWCaW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aWaWaW`aW'V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aW}WaW|aW+V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aW\x99WaW\x98aW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xB6WaW\xB5aW'V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aW\xD2WaW\xD1aW+V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aW\xF2WaW\xF1aT\x9FV[[_aW\xFF\x88\x82\x89\x01aT\xBDV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX WaX\x1FaT\xA3V[[aX,\x88\x82\x89\x01aW/V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aXOWaXNaT\xA3V[[aX[\x88\x82\x89\x01aW\x84V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[aXs\x81aUDV[\x81\x14aX}W_\x80\xFD[PV[_\x815\x90PaX\x8E\x81aXjV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aX\xCE\x82aT.V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aX\xEDWaX\xECaX\x98V[[\x80`@RPPPV[_aX\xFFaT\x96V[\x90PaY\x0B\x82\x82aX\xC5V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aY*WaY)aX\x98V[[aY3\x82aT.V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aY`aY[\x84aY\x10V[aX\xF6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aY|WaY{aX\x94V[[aY\x87\x84\x82\x85aY@V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aY\xA3WaY\xA2aW#V[[\x815aY\xB3\x84\x82` \x86\x01aYNV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aY\xD2WaY\xD1aT\x9FV[[_aY\xDF\x85\x82\x86\x01aX\x80V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\0WaY\xFFaT\xA3V[[aZ\x0C\x85\x82\x86\x01aY\x8FV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aZ(\x81aZ\x16V[\x82RPPV[_` \x82\x01\x90PaZA_\x83\x01\x84aZ\x1FV[\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aZ\\WaZ[aW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZyWaZxaW'V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aZ\x95WaZ\x94aW+V[[\x92P\x92\x90PV[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15aZ\xB6WaZ\xB5aT\x9FV[[_aZ\xC3\x89\x82\x8A\x01aT\xBDV[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xE4WaZ\xE3aT\xA3V[[aZ\xF0\x89\x82\x8A\x01aZGV[\x95P\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x13Wa[\x12aT\xA3V[[a[\x1F\x89\x82\x8A\x01aZGV[\x93P\x93PP``a[2\x89\x82\x8A\x01aT\xBDV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80_`@\x84\x86\x03\x12\x15a[VWa[UaT\x9FV[[_a[c\x86\x82\x87\x01aT\xBDV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x84Wa[\x83aT\xA3V[[a[\x90\x86\x82\x87\x01aW\x84V[\x92P\x92PP\x92P\x92P\x92V[a[\xA5\x81aS\xBBV[\x82RPPV[a[\xB4\x81aVVV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a[\xF4Wa[\xF3aV\x03V[[PV[_\x81\x90Pa\\\x04\x82a[\xE3V[\x91\x90PV[_a\\\x13\x82a[\xF7V[\x90P\x91\x90PV[a\\#\x81a\\\tV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\\M\x82a\\)V[a\\W\x81\x85a\\3V[\x93Pa\\g\x81\x85` \x86\x01aT\x06V[a\\p\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa\\\x90_\x86\x01\x82a\\\x1AV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra\\\xA8\x82\x82a\\CV[\x91PP\x80\x91PP\x92\x91PPV[_a\\\xC0\x83\x83a\\{V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\\\xDE\x82a[\xBAV[a\\\xE8\x81\x85a[\xC4V[\x93P\x83` \x82\x02\x85\x01a\\\xFA\x85a[\xD4V[\x80_[\x85\x81\x10\x15a]5W\x84\x84\x03\x89R\x81Qa]\x16\x85\x82a\\\xB5V[\x94Pa]!\x83a\\\xC8V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\\\xFDV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa]\\_\x86\x01\x82a[\x9CV[P` \x83\x01Qa]o` \x86\x01\x82a[\x9CV[P`@\x83\x01Qa]\x82`@\x86\x01\x82a[\xABV[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra]\x9A\x82\x82a\\\xD4V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xBF\x81\x84a]GV[\x90P\x92\x91PPV[_\x80_\x80_``\x86\x88\x03\x12\x15a]\xE0Wa]\xDFaT\x9FV[[_a]\xED\x88\x82\x89\x01aT\xBDV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x0EWa^\raT\xA3V[[a^\x1A\x88\x82\x89\x01aW\x84V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^=Wa^<aT\xA3V[[a^I\x88\x82\x89\x01aW\x84V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a^\x8C\x81a^XV[\x82RPPV[a^\x9B\x81aUDV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a^\xD5\x83\x83a[\x9CV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^\xF7\x82a^\xA1V[a_\x01\x81\x85a^\xABV[\x93Pa_\x0C\x83a^\xBBV[\x80_[\x83\x81\x10\x15a_<W\x81Qa_#\x88\x82a^\xCAV[\x97Pa_.\x83a^\xE1V[\x92PP`\x01\x81\x01\x90Pa_\x0FV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa_\\_\x83\x01\x8Aa^\x83V[\x81\x81\x03` \x83\x01Ra_n\x81\x89aT>V[\x90P\x81\x81\x03`@\x83\x01Ra_\x82\x81\x88aT>V[\x90Pa_\x91``\x83\x01\x87aS\xC4V[a_\x9E`\x80\x83\x01\x86a^\x92V[a_\xAB`\xA0\x83\x01\x85aZ\x1FV[\x81\x81\x03`\xC0\x83\x01Ra_\xBD\x81\x84a^\xEDV[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a`\x0E\x82aS\xECV[a`\x18\x81\x85a_\xF4V[\x93Pa`(\x81\x85` \x86\x01aT\x06V[a`1\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_a`G\x83\x83a`\x04V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a`e\x82a_\xCBV[a`o\x81\x85a_\xD5V[\x93P\x83` \x82\x02\x85\x01a`\x81\x85a_\xE5V[\x80_[\x85\x81\x10\x15a`\xBCW\x84\x84\x03\x89R\x81Qa`\x9D\x85\x82a`<V[\x94Pa`\xA8\x83a`OV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa`\x84V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a`\xE8\x82a[\xBAV[a`\xF2\x81\x85a`\xCEV[\x93P\x83` \x82\x02\x85\x01aa\x04\x85a[\xD4V[\x80_[\x85\x81\x10\x15aa?W\x84\x84\x03\x89R\x81Qaa \x85\x82a\\\xB5V[\x94Paa+\x83a\\\xC8V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Paa\x07V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Raai\x81\x85a`[V[\x90P\x81\x81\x03` \x83\x01Raa}\x81\x84a`\xDEV[\x90P\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aa\x9CWaa\x9BaT\x9FV[[_aa\xA9\x85\x82\x86\x01aV\x9EV[\x92PP` aa\xBA\x85\x82\x86\x01aT\xBDV[\x91PP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aa\xD9Waa\xD8aW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\xF6Waa\xF5aW'V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15ab\x12Wab\x11aW+V[[\x92P\x92\x90PV[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15ab3Wab2aT\x9FV[[_ab@\x89\x82\x8A\x01aT\xBDV[\x96PP` abQ\x89\x82\x8A\x01aT\xBDV[\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15abrWabqaT\xA3V[[ab~\x89\x82\x8A\x01aW/V[\x94P\x94PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab\xA1Wab\xA0aT\xA3V[[ab\xAD\x89\x82\x8A\x01aa\xC4V[\x92P\x92PP\x92\x95P\x92\x95P\x92\x95V[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ab\xD6\x82a\\)V[ab\xE0\x81\x85ab\xBCV[\x93Pab\xF0\x81\x85` \x86\x01aT\x06V[ab\xF9\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rac\x1C\x81\x85a`[V[\x90P\x81\x81\x03` \x83\x01Rac0\x81\x84ab\xCCV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15acNWacMaT\x9FV[[_ac[\x84\x82\x85\x01aV\x9EV[\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rac|\x81\x84a^\xEDV[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ac\x98\x82aS\xECV[ac\xA2\x81\x85ac\x84V[\x93Pac\xB2\x81\x85` \x86\x01aT\x06V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ac\xF2`\x02\x83ac\x84V[\x91Pac\xFD\x82ac\xBEV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ad<`\x01\x83ac\x84V[\x91PadG\x82ad\x08V[`\x01\x82\x01\x90P\x91\x90PV[_ad]\x82\x87ac\x8EV[\x91Padh\x82ac\xE6V[\x91Padt\x82\x86ac\x8EV[\x91Pad\x7F\x82ad0V[\x91Pad\x8B\x82\x85ac\x8EV[\x91Pad\x96\x82ad0V[\x91Pad\xA2\x82\x84ac\x8EV[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Pad\xBE\x81aXjV[\x92\x91PPV[_` \x82\x84\x03\x12\x15ad\xD9Wad\xD8aT\x9FV[[_ad\xE6\x84\x82\x85\x01ad\xB0V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pae\x02_\x83\x01\x84a^\x92V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[ae$\x81ae\x08V[\x82RPPV[_` \x82\x01\x90Pae=_\x83\x01\x84ae\x1BV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aez\x82aS\xBBV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03ae\xACWae\xABaeCV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pae\xC5\x81aT\xA7V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15ae\xE1Wae\xE0aT\x9FV[[_ae\xEE\x85\x82\x86\x01ae\xB7V[\x92PP` ae\xFF\x85\x82\x86\x01ae\xB7V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80afMW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03af`Waf_af\tV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02af\xC2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82af\x87V[af\xCC\x86\x83af\x87V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ag\x07ag\x02af\xFD\x84aS\xBBV[af\xE4V[aS\xBBV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ag \x83af\xEDV[ag4ag,\x82ag\x0EV[\x84\x84Taf\x93V[\x82UPPPPV[_\x90V[agHag<V[agS\x81\x84\x84ag\x17V[PPPV[[\x81\x81\x10\x15agvWagk_\x82ag@V[`\x01\x81\x01\x90PagYV[PPV[`\x1F\x82\x11\x15ag\xBBWag\x8C\x81affV[ag\x95\x84afxV[\x81\x01` \x85\x10\x15ag\xA4W\x81\x90P[ag\xB8ag\xB0\x85afxV[\x83\x01\x82agXV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ag\xDB_\x19\x84`\x08\x02ag\xC0V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_ag\xF3\x83\x83ag\xCCV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[ah\x0C\x82a\\)V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah%Wah$aX\x98V[[ah/\x82Taf6V[ah:\x82\x82\x85agzV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ahkW_\x84\x15ahYW\x82\x87\x01Q\x90P[ahc\x85\x82ag\xE8V[\x86UPah\xCAV[`\x1F\x19\x84\x16ahy\x86affV[_[\x82\x81\x10\x15ah\xA0W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pah{V[\x86\x83\x10\x15ah\xBDW\x84\x89\x01Qah\xB9`\x1F\x89\x16\x82ag\xCCV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90Pah\xE5_\x83\x01\x87aS\xC4V[ah\xF2` \x83\x01\x86aS\xC4V[ah\xFF`@\x83\x01\x85aVgV[\x81\x81\x03``\x83\x01Rai\x11\x81\x84ab\xCCV[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90Pai/_\x83\x01\x85aS\xC4V[ai<` \x83\x01\x84a^\x92V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10aiXW_\x80\xFD[PV[_\x815\x90Paii\x81aiLV[\x92\x91PPV[_ai}` \x84\x01\x84ai[V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ai\xADWai\xACai\x8DV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ai\xD5Wai\xD4ai\x85V[[`\x01\x82\x026\x03\x83\x13\x15ai\xEBWai\xEAai\x89V[[P\x92P\x92\x90PV[_ai\xFE\x83\x85a\\3V[\x93Paj\x0B\x83\x85\x84aY@V[aj\x14\x83aT.V[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aj0_\x84\x01\x84aioV[aj<_\x86\x01\x82a\\\x1AV[PajJ` \x84\x01\x84ai\x91V[\x85\x83\x03` \x87\x01Raj]\x83\x82\x84ai\xF3V[\x92PPP\x80\x91PP\x92\x91PPV[_ajv\x83\x83aj\x1FV[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aj\x99Waj\x98ai\x8DV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aj\xBC\x83\x85a`\xCEV[\x93P\x83` \x84\x02\x85\x01aj\xCE\x84aiCV[\x80_[\x87\x81\x10\x15ak\x11W\x84\x84\x03\x89Raj\xE8\x82\x84aj~V[aj\xF2\x85\x82ajkV[\x94Paj\xFD\x83aj\xA5V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Paj\xD1V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_ak.\x83\x85ab\xBCV[\x93Pak;\x83\x85\x84aY@V[akD\x83aT.V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pakb_\x83\x01\x89aS\xC4V[\x81\x81\x03` \x83\x01Raku\x81\x87\x89aj\xB1V[\x90P\x81\x81\x03`@\x83\x01Rak\x8A\x81\x85\x87ak#V[\x90Pak\x99``\x83\x01\x84a^\x92V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ak\xF8Wak\xF7ak\xD1V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815al\x10\x81aiLV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFal0\x84al\x19V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_alP\x82a[\xF7V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ali\x82alFV[al|alu\x82alWV[\x83Tal$V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12al\x9FWal\x9Eak\xD1V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15al\xC1Wal\xC0ak\xD5V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15al\xDDWal\xDCak\xD9V[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[al\xF9\x83\x83al\xE5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15am\x12Wam\x11aX\x98V[[am\x1C\x82Taf6V[am'\x82\x82\x85agzV[_`\x1F\x83\x11`\x01\x81\x14amTW_\x84\x15amBW\x82\x87\x015\x90P[amL\x85\x82ag\xE8V[\x86UPam\xB3V[`\x1F\x19\x84\x16amb\x86affV[_[\x82\x81\x10\x15am\x89W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PamdV[\x86\x83\x10\x15am\xA6W\x84\x89\x015am\xA2`\x1F\x89\x16\x82ag\xCCV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[am\xC7\x83\x83\x83al\xEFV[PPPV[_\x81\x01_\x83\x01\x80am\xDC\x81al\x04V[\x90Pam\xE8\x81\x84al`V[PPP`\x01\x81\x01` \x83\x01am\xFD\x81\x85al\x83V[an\x08\x81\x83\x86am\xBCV[PPPPPPV[an\x1A\x82\x82am\xCCV[PPV[_``\x82\x01\x90Pan1_\x83\x01\x87aS\xC4V[\x81\x81\x03` \x83\x01RanC\x81\x86a`[V[\x90P\x81\x81\x03`@\x83\x01RanX\x81\x84\x86aj\xB1V[\x90P\x95\x94PPPPPV[_\x80\xFD[\x82\x81\x837PPPV[_an{\x83\x85a^\xABV[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15an\xAEWan\xADancV[[` \x83\x02\x92Pan\xBF\x83\x85\x84angV[\x82\x84\x01\x90P\x93\x92PPPV[_`\xA0\x82\x01\x90Pan\xDE_\x83\x01\x8AaS\xC4V[\x81\x81\x03` \x83\x01Ran\xF1\x81\x88\x8AanpV[\x90P\x81\x81\x03`@\x83\x01Rao\x06\x81\x86\x88anpV[\x90Pao\x15``\x83\x01\x85aS\xC4V[ao\"`\x80\x83\x01\x84aS\xC4V[\x98\x97PPPPPPPPV[_``\x82\x01\x90PaoA_\x83\x01\x87aS\xC4V[\x81\x81\x03` \x83\x01RaoT\x81\x85\x87ak#V[\x90Paoc`@\x83\x01\x84a^\x92V[\x95\x94PPPPPV[_`\xA0\x82\x01\x90Pao\x7F_\x83\x01\x88aS\xC4V[ao\x8C` \x83\x01\x87aS\xC4V[ao\x99`@\x83\x01\x86aS\xC4V[ao\xA6``\x83\x01\x85aV\xFBV[\x81\x81\x03`\x80\x83\x01Rao\xB8\x81\x84ab\xCCV[\x90P\x96\x95PPPPPPV[_``\x82\x01\x90Pao\xD7_\x83\x01\x86aS\xC4V[ao\xE4` \x83\x01\x85aS\xC4V[\x81\x81\x03`@\x83\x01Rao\xF6\x81\x84ab\xCCV[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Pap\x13_\x83\x01\x89aS\xC4V[\x81\x81\x03` \x83\x01Rap&\x81\x87\x89ak#V[\x90P\x81\x81\x03`@\x83\x01Rap;\x81\x85\x87ak#V[\x90PapJ``\x83\x01\x84a^\x92V[\x97\x96PPPPPPPV[_``\x82\x01\x90Paph_\x83\x01\x87aS\xC4V[\x81\x81\x03` \x83\x01Rapz\x81\x86a`[V[\x90P\x81\x81\x03`@\x83\x01Rap\x8F\x81\x84\x86ak#V[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ap\xCE`\x15\x83aS\xF6V[\x91Pap\xD9\x82ap\x9AV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rap\xFB\x81ap\xC2V[\x90P\x91\x90PV[_`@\x82\x01\x90Paq\x15_\x83\x01\x85aS\xC4V[aq\"` \x83\x01\x84aS\xC4V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aq=\x83\x85a_\xF4V[\x93PaqJ\x83\x85\x84aY@V[aqS\x83aT.V[\x84\x01\x90P\x93\x92PPPV[_aqj\x84\x84\x84aq2V[\x90P\x93\x92PPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aq\x8FWaq\x8Eai\x8DV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aq\xB7Waq\xB6ai\x85V[[`\x01\x82\x026\x03\x83\x13\x15aq\xCDWaq\xCCai\x89V[[P\x92P\x92\x90PV[_` \x82\x01\x90P\x91\x90PV[_aq\xEC\x83\x85a_\xD5V[\x93P\x83` \x84\x02\x85\x01aq\xFE\x84aq)V[\x80_[\x87\x81\x10\x15arCW\x84\x84\x03\x89Rar\x18\x82\x84aqsV[ar#\x86\x82\x84aq^V[\x95Par.\x84aq\xD5V[\x93P` \x8B\x01\x9APPP`\x01\x81\x01\x90Par\x01V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`\x80\x82\x01\x90Parh_\x83\x01\x89aS\xC4V[\x81\x81\x03` \x83\x01Rar{\x81\x87\x89aq\xE1V[\x90P\x81\x81\x03`@\x83\x01Rar\x90\x81\x85\x87aj\xB1V[\x90Par\x9F``\x83\x01\x84aS\xC4V[\x97\x96PPPPPPPV[_ar\xB4\x82aS\xBBV[\x91Par\xBF\x83aS\xBBV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15ar\xD7War\xD6aeCV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_as,\x82as\x16V[\x90P\x91\x90PV[asDas?\x82as\nV[as\"V[\x82RPPV[_\x81\x90P\x91\x90PV[asdas_\x82aS\xBBV[asJV[\x82RPPV[_asu\x82\x86as3V[`\x01\x82\x01\x91Pas\x85\x82\x85asSV[` \x82\x01\x91Pas\x95\x82\x84asSV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[as\xAF\x81aV\xF0V[\x81\x14as\xB9W_\x80\xFD[PV[_\x81Q\x90Pas\xCA\x81as\xA6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15as\xE5Was\xE4aT\x9FV[[_as\xF2\x84\x82\x85\x01as\xBCV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15at\x10Wat\x0FaT\x9FV[[_at\x1D\x84\x82\x85\x01ai[V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_at;\x83\x85at&V[\x93PatH\x83\x85\x84aY@V[\x82\x84\x01\x90P\x93\x92PPPV[_at`\x82\x84\x86at0V[\x91P\x81\x90P\x93\x92PPPV[atu\x81a\\\tV[\x82RPPV[_``\x82\x01\x90Pat\x8E_\x83\x01\x86aZ\x1FV[at\x9B` \x83\x01\x85atlV[at\xA8`@\x83\x01\x84aZ\x1FV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[at\xDC\x81aZ\x16V[\x82RPPV[_at\xED\x83\x83at\xD3V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_au\x0F\x82at\xB0V[au\x19\x81\x85at\xBAV[\x93Pau$\x83at\xC4V[\x80_[\x83\x81\x10\x15auTW\x81Qau;\x88\x82at\xE2V[\x97PauF\x83at\xF9V[\x92PP`\x01\x81\x01\x90Pau'V[P\x85\x93PPPP\x92\x91PPV[_aul\x82\x84au\x05V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pau\x8A_\x83\x01\x88aZ\x1FV[au\x97` \x83\x01\x87aS\xC4V[au\xA4`@\x83\x01\x86aS\xC4V[au\xB1``\x83\x01\x85aZ\x1FV[au\xBE`\x80\x83\x01\x84aZ\x1FV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15au\xDDWau\xDCaT\x9FV[[_au\xEA\x84\x82\x85\x01ae\xB7V[\x91PP\x92\x91PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15av\x15Wav\x14aX\x98V[[av\x1E\x82aT.V[\x90P` \x81\x01\x90P\x91\x90PV[_av=av8\x84au\xFBV[aX\xF6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15avYWavXaX\x94V[[avd\x84\x82\x85aT\x06V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12av\x80Wav\x7FaW#V[[\x81Qav\x90\x84\x82` \x86\x01av+V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15av\xAEWav\xADau\xF3V[[av\xB8`\x80aX\xF6V[\x90P_av\xC7\x84\x82\x85\x01ad\xB0V[_\x83\x01RP` av\xDA\x84\x82\x85\x01ad\xB0V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15av\xFEWav\xFDau\xF7V[[aw\n\x84\x82\x85\x01avlV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aw.Waw-au\xF7V[[aw:\x84\x82\x85\x01avlV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aw[WawZaT\x9FV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15awxWawwaT\xA3V[[aw\x84\x84\x82\x85\x01av\x99V[\x91PP\x92\x91PPV[aw\x96\x81aZ\x16V[\x81\x14aw\xA0W_\x80\xFD[PV[_\x81Q\x90Paw\xB1\x81aw\x8DV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aw\xCCWaw\xCBaT\x9FV[[_aw\xD9\x84\x82\x85\x01aw\xA3V[\x91PP\x92\x91PPV[_``\x82\x01\x90Paw\xF5_\x83\x01\x86aZ\x1FV[ax\x02` \x83\x01\x85aS\xC4V[ax\x0F`@\x83\x01\x84aZ\x1FV[\x94\x93PPPPV[ax \x81as\nV[\x82RPPV[_` \x82\x01\x90Pax9_\x83\x01\x84ax\x17V[\x92\x91PPV[_``\x82\x01\x90PaxR_\x83\x01\x86aS\xC4V[ax_` \x83\x01\x85aVgV[\x81\x81\x03`@\x83\x01Raxq\x81\x84ab\xCCV[\x90P\x94\x93PPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ax\xCEWax\x9F\x81ax{V[ax\xA8\x84afxV[\x81\x01` \x85\x10\x15ax\xB7W\x81\x90P[ax\xCBax\xC3\x85afxV[\x83\x01\x82agXV[PP[PPPV[ax\xDC\x82aS\xECV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ax\xF5Wax\xF4aX\x98V[[ax\xFF\x82Taf6V[ay\n\x82\x82\x85ax\x8DV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ay;W_\x84\x15ay)W\x82\x87\x01Q\x90P[ay3\x85\x82ag\xE8V[\x86UPay\x9AV[`\x1F\x19\x84\x16ayI\x86ax{V[_[\x82\x81\x10\x15aypW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PayKV[\x86\x83\x10\x15ay\x8DW\x84\x89\x01Qay\x89`\x1F\x89\x16\x82ag\xCCV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pay\xB5_\x83\x01\x85a^\x92V[ay\xC2` \x83\x01\x84a^\x92V[\x93\x92PPPV[_ay\xD3\x82a\\)V[ay\xDD\x81\x85at&V[\x93Pay\xED\x81\x85` \x86\x01aT\x06V[\x80\x84\x01\x91PP\x92\x91PPV[_az\x04\x82\x84ay\xC9V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Paz\"_\x83\x01\x88aZ\x1FV[az/` \x83\x01\x87aZ\x1FV[az<`@\x83\x01\x86aZ\x1FV[azI``\x83\x01\x85aS\xC4V[azV`\x80\x83\x01\x84a^\x92V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pazs_\x83\x01\x87aZ\x1FV[az\x80` \x83\x01\x86ax\x17V[az\x8D`@\x83\x01\x85aZ\x1FV[az\x9A``\x83\x01\x84aZ\x1FV[\x95\x94PPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101d7575f3560e01c80636294f46211610101578063c2c1faee11610094578063d52f10eb11610063578063d52f10eb14610685578063dabd732f146106af578063e410117e146106d9578063f0f8cbc614610703576101d7565b8063c2c1faee146105e2578063c41158741461060a578063c55b872414610620578063caa367db1461065d576101d7565b8063aaa47016116100d0578063aaa470161461053e578063ad3cb1cc14610566578063b53b3ccc14610590578063baff211e146105b8576101d7565b80636294f4621461046d57806362978787146104a957806384b0196e146104d1578063936608ae14610501576101d7565b80633c02f834116101795780634f1ef286116101485780634f1ef286146103d757806352d1902d146103f357806356a610b41461041d578063589adb0e14610445576101d7565b80633c02f8341461030f5780633d5ec7e31461033757806345af261b146103735780634610ffe8146103af576101d7565b80631703c61a116101b55780631703c61a1461026b57806319f4f6321461029357806339f73810146102cf5780633ac50072146102e5576101d7565b80630b680733146101db5780630d8e6e2c1461020557806316c713d91461022f575b5f80fd5b3480156101e6575f80fd5b506101ef61073f565b6040516101fc91906153d3565b60405180910390f35b348015610210575f80fd5b50610219610756565b6040516102269190615476565b60405180910390f35b34801561023a575f80fd5b50610255600480360381019061025091906154d1565b6107d1565b60405161026291906155e3565b60405180910390f35b348015610276575f80fd5b50610291600480360381019061028c91906154d1565b6108a2565b005b34801561029e575f80fd5b506102b960048036038101906102b491906154d1565b610ad0565b6040516102c69190615676565b60405180910390f35b3480156102da575f80fd5b506102e3610bd6565b005b3480156102f0575f80fd5b506102f9610e26565b60405161030691906153d3565b60405180910390f35b34801561031a575f80fd5b50610335600480360381019061033091906156b2565b610e3d565b005b348015610342575f80fd5b5061035d600480360381019061035891906154d1565b611134565b60405161036a919061570a565b60405180910390f35b34801561037e575f80fd5b50610399600480360381019061039491906154d1565b611168565b6040516103a69190615676565b60405180910390f35b3480156103ba575f80fd5b506103d560048036038101906103d091906157d9565b611256565b005b6103f160048036038101906103ec91906159bc565b6117a6565b005b3480156103fe575f80fd5b506104076117c5565b6040516104149190615a2e565b60405180910390f35b348015610428575f80fd5b50610443600480360381019061043e9190615a9c565b6117f6565b005b348015610450575f80fd5b5061046b60048036038101906104669190615b3f565b6119d4565b005b348015610478575f80fd5b50610493600480360381019061048e91906154d1565b611d90565b6040516104a09190615da7565b60405180910390f35b3480156104b4575f80fd5b506104cf60048036038101906104ca9190615dc7565b611ffd565b005b3480156104dc575f80fd5b506104e561242a565b6040516104f89796959493929190615f49565b60405180910390f35b34801561050c575f80fd5b50610527600480360381019061052291906154d1565b612533565b604051610535929190616151565b60405180910390f35b348015610549575f80fd5b50610564600480360381019061055f9190616186565b612899565b005b348015610571575f80fd5b5061057a612a5d565b6040516105879190615476565b60405180910390f35b34801561059b575f80fd5b506105b660048036038101906105b19190616219565b612a96565b005b3480156105c3575f80fd5b506105cc612cec565b6040516105d991906153d3565b60405180910390f35b3480156105ed575f80fd5b50610608600480360381019061060391906154d1565b612d03565b005b348015610615575f80fd5b5061061e612f7c565b005b34801561062b575f80fd5b50610646600480360381019061064191906154d1565b6131c5565b604051610654929190616304565b60405180910390f35b348015610668575f80fd5b50610683600480360381019061067e9190616339565b613496565b005b348015610690575f80fd5b50610699613594565b6040516106a691906153d3565b60405180910390f35b3480156106ba575f80fd5b506106c36135ab565b6040516106d09190616364565b60405180910390f35b3480156106e4575f80fd5b506106ed61360f565b6040516106fa9190616364565b60405180910390f35b34801561070e575f80fd5b50610729600480360381019061072491906154d1565b613673565b60405161073691906153d3565b60405180910390f35b5f806107496136a5565b9050806005015491505090565b60606040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506107975f6136cc565b6107a160026136cc565b6107aa5f6136cc565b6040516020016107bd9493929190616452565b604051602081830303815290604052905090565b60605f6107dc6136a5565b90505f816003015f8581526020019081526020015f20549050816002015f8581526020019081526020015f205f8281526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561089457602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168152602001906001019080831161084b575b505050505092505050919050565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156108ff573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061092391906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461099257336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161098991906164ef565b60405180910390fd5b5f61099b6136a5565b905080600901548211806109c6575060f8600560088111156109c0576109bf615603565b5b901b8211155b15610a0857816040517fcbe926560000000000000000000000000000000000000000000000000000000081526004016109ff91906153d3565b60405180910390fd5b806001015f8381526020019081526020015f205f9054906101000a900460ff1615610a6a57816040517fdf0db5fb000000000000000000000000000000000000000000000000000000008152600401610a6191906153d3565b60405180910390fd5b6001816001015f8481526020019081526020015f205f6101000a81548160ff0219169083151502179055507f384f90fefbcfaa68f22e00094aeaa52b2bc693936d2ce1afed1212520b59b58e82604051610ac491906153d3565b60405180910390a15050565b5f80610ada6136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16610b3d57826040517f84de1331000000000000000000000000000000000000000000000000000000008152600401610b3491906153d3565b60405180910390fd5b5f801b816003015f8581526020019081526020015f205403610b9657826040517f83f18335000000000000000000000000000000000000000000000000000000008152600401610b8d91906153d3565b60405180910390fd5b5f816006015f8581526020019081526020015f2054905081600d015f8281526020019081526020015f205f9054906101000a900460ff1692505050919050565b6001610be0613796565b67ffffffffffffffff1614610c21576040517f6f4f731f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035f610c2c6137ba565b9050805f0160089054906101000a900460ff1680610c7457508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15610cab576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff021916908315150217905550610d646040518060400160405280600d81526020017f4b4d5347656e65726174696f6e000000000000000000000000000000000000008152506040518060400160405280600181526020017f31000000000000000000000000000000000000000000000000000000000000008152506137e1565b5f610d6d6136a5565b905060f860036008811115610d8557610d84615603565b5b901b816004018190555060f860046008811115610da557610da4615603565b5b901b816005018190555060f860056008811115610dc557610dc4615603565b5b901b8160090181905550505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d282604051610e1a919061652a565b60405180910390a15050565b5f80610e306136a5565b9050806009015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610e9a573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ebe91906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614610f2d57336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401610f2491906164ef565b60405180910390fd5b5f610f366136a5565b90505f8160090154905060f860056008811115610f5657610f55615603565b5b901b8114158015610f845750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b15610fc657806040517f061ac61d000000000000000000000000000000000000000000000000000000008152600401610fbd91906153d3565b60405180910390fd5b816009015f815480929190610fda90616570565b91905055505f826009015490508483600a015f8381526020019081526020015f20819055508383600d015f8381526020019081526020015f205f6101000a81548160ff0219169083600181111561103457611033615603565b5b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015611097573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110bb91906165cb565b915091505f6110ca83836137f7565b90508086600e015f8681526020019081526020015f2090816110ec9190616803565b507f8cf0151393f84fd694c5e315cb74cc05b247de0a454fd9e9129c661efdf9401d8489898460405161112294939291906168d2565b60405180910390a15050505050505050565b5f8061113e6136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f806111726136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff166111d557826040517fda32d00f0000000000000000000000000000000000000000000000000000000081526004016111cc91906153d3565b60405180910390fd5b5f801b816003015f8581526020019081526020015f20540361122e57826040517fd5fd3cd700000000000000000000000000000000000000000000000000000000815260040161122591906153d3565b60405180910390fd5b80600d015f8481526020019081526020015f205f9054906101000a900460ff16915050919050565b5f61125f6136a5565b9050806005015486118061128a575060f86004600881111561128457611283615603565b5b901b8611155b156112cc57856040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016112c391906153d3565b60405180910390fd5b5f858590500361131357856040517fe6f9083b00000000000000000000000000000000000000000000000000000000815260040161130a91906153d3565b60405180910390fd5b5f8061131e88613826565b915091505f836006015f8a81526020019081526020015f20549050836001015f8281526020019081526020015f205f9054906101000a900460ff1661138f576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61139d828b8b8b886139b0565b90505f6113ac84838a8a613b91565b9050855f015f8c81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff161561144c578a816040517f98fb957d00000000000000000000000000000000000000000000000000000000815260040161144392919061691c565b60405180910390fd5b6001865f015f8d81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f866002015f8d81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f2afe64fb3afde8e2678aea84cf36223f330e2fb1286d37aed573ab9cd1db47c78c8c8c8c8c3360405161156e96959493929190616b4f565b60405180910390a1866001015f8d81526020019081526020015f205f9054906101000a900460ff161580156115ad57506115ac858280549050613bf9565b5b15611798576001876001015f8e81526020019081526020015f205f6101000a81548160ff0219169083151502179055505f5b8b8b905081101561166357876007015f8e81526020019081526020015f208c8c838181106116105761160f616ba4565b5b90506020028101906116229190616bdd565b908060018154018082558091505060019003905f5260205f2090600202015f9091909190915081816116549190616e10565b505080806001019150506115df565b5082876003015f8e81526020019081526020015f208190555086600f018c908060018154018082558091505060019003905f5260205f20015f90919091909150555f876011015f8e81526020019081526020015f205f015403611797578b87600801819055505f611756868380548060200260200160405190810160405280929190818152602001828054801561174c57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311611703575b5050505050613c96565b90507feb85c26dbcad46b80a68a0f24cce7c2c90f0a1faded84184138839fc9e80a25b8d828e8e60405161178d9493929190616e1e565b60405180910390a1505b5b505050505050505050505050565b6117ae613dde565b6117b782613ec4565b6117c18282613fb7565b5050565b5f6117ce6140d5565b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b905090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611853573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061187791906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146118e657336040517f21bfda100000000000000000000000000000000000000000000000000000000081526004016118dd91906164ef565b60405180910390fd5b5f6118ef6136a5565b9050838390508686905014611930576040517f894b2ab300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f816012015f8981526020019081526020015f20540361198757866040517f05b083f200000000000000000000000000000000000000000000000000000000815260040161197e91906153d3565b60405180910390fd5b7f8bfa7d0ed6f87e526b62342918ee7bfa53952badd463dc934054d7dd940eafdc87878787878760016040516119c39796959493929190616ecb565b60405180910390a150505050505050565b5f6119dd6136a5565b90508060040154841180611a08575060f860036008811115611a0257611a01615603565b5b901b8411155b15611a4a57836040517f0ab7f687000000000000000000000000000000000000000000000000000000008152600401611a4191906153d3565b60405180910390fd5b5f80611a5586613826565b915091505f611a64878461415c565b90505f611a7383838989613b91565b9050845f015f8981526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff1615611b135787816040517f33ca1fe3000000000000000000000000000000000000000000000000000000008152600401611b0a92919061691c565b60405180910390fd5b6001855f015f8a81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8a81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f4c715c5734ce5c18c9c12e8496e53d2a65f1ec381d476957f0f596b364a59b0c89898933604051611c319493929190616f2e565b60405180910390a1856001015f8a81526020019081526020015f205f9054906101000a900460ff16158015611c705750611c6f848280549050613bf9565b5b15611d85576001866001015f8b81526020019081526020015f205f6101000a81548160ff02191690831515021790555082866003015f8b81526020019081526020015f20819055505f866006015f8b81526020019081526020015f205490505f876011015f8381526020019081526020015f2090505f815f015414611d46577fe453c29c46ccc7664c0398e8464d5bb421e995432daf5506a3fdbc6aa0966a938b83835f0154846001015f9054906101000a900460ff168b604051611d39959493929190616f6c565b60405180910390a1611d82565b7f3a116120cca5d4f073cc1fc31ff26133ab7b0499f2712fa010023b87d5a1f9ee8b8389604051611d7993929190616fc4565b60405180910390a15b50505b505050505050505050565b611d98615384565b5f611da16136a5565b9050806001015f8481526020019081526020015f205f9054906101000a900460ff16611e0457826040517f84de1331000000000000000000000000000000000000000000000000000000008152600401611dfb91906153d3565b60405180910390fd5b5f801b816003015f8581526020019081526020015f205403611e5d57826040517f83f18335000000000000000000000000000000000000000000000000000000008152600401611e5491906153d3565b60405180910390fd5b5f816006015f8581526020019081526020015f20549050604051806080016040528082815260200185815260200183600d015f8481526020019081526020015f205f9054906101000a900460ff166001811115611ebd57611ebc615603565b5b8152602001836007015f8781526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015611fed578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff166001811115611f3857611f37615603565b5b6001811115611f4a57611f49615603565b5b8152602001600182018054611f5e90616636565b80601f0160208091040260200160405190810160405280929190818152602001828054611f8a90616636565b8015611fd55780601f10611fac57610100808354040283529160200191611fd5565b820191905f5260205f20905b815481529060010190602001808311611fb857829003601f168201915b50505050508152505081526020019060010190611ef4565b5050505081525092505050919050565b5f6120066136a5565b90508060090154861180612031575060f86005600881111561202b5761202a615603565b5b901b8611155b1561207357856040517f8d8c940a00000000000000000000000000000000000000000000000000000000815260040161206a91906153d3565b60405180910390fd5b5f8061207e88613826565b915091505f6120a38985600a015f8c81526020019081526020015f20548a8a876141be565b90505f6120b283838989613b91565b9050845f015f8b81526020019081526020015f205f8273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f9054906101000a900460ff16156121525789816040517ffcf5a6e900000000000000000000000000000000000000000000000000000000815260040161214992919061691c565b60405180910390fd5b6001855f015f8c81526020019081526020015f205f8373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020015f205f6101000a81548160ff0219169083151502179055505f856002015f8c81526020019081526020015f205f8481526020019081526020015f2090508033908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055507f7bf1b42c10e9497c879620c5b7afced10bda17d8c90b22f0e3bc6b2fd6ced0bd8b8b8b8b8b3360405161227496959493929190617000565b60405180910390a1856001015f8c81526020019081526020015f205f9054906101000a900460ff161580156122b357506122b2848280549050613bf9565b5b1561241d576001866001015f8d81526020019081526020015f205f6101000a81548160ff021916908315150217905550898987600b015f8e81526020019081526020015f209182612305929190616cef565b5082866003015f8d81526020019081526020015f20819055508a86600c0181905550856010018b908060018154018082558091505060019003905f5260205f20015f90919091909150555f6123dc85838054806020026020016040519081016040528092919081815260200182805480156123d257602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612389575b5050505050613c96565b90507f2258b73faed33fb2e2ea454403bef974920caf682ab3a723484fcf67553b16a28c828d8d6040516124139493929190617055565b60405180910390a1505b5050505050505050505050565b5f6060805f805f60605f61243c61424f565b90505f801b815f015414801561245757505f801b8160010154145b612496576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161248d906170e4565b60405180910390fd5b61249e614276565b6124a6614314565b46305f801b5f67ffffffffffffffff8111156124c5576124c4615898565b5b6040519080825280602002602001820160405280156124f35781602001602082028036833780820191505090505b507f0f0000000000000000000000000000000000000000000000000000000000000095949392919097509750975097509750975097505090919293949596565b6060805f61253f6136a5565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff166125a257836040517f84de133100000000000000000000000000000000000000000000000000000000815260040161259991906153d3565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b81036125ff57846040517f83f183350000000000000000000000000000000000000000000000000000000081526004016125f691906153d3565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561269f57602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019060010190808311612656575b505050505090505f61274984600e015f8981526020019081526020015f2080546126c890616636565b80601f01602080910402602001604051908101604052809291908181526020018280546126f490616636565b801561273f5780601f106127165761010080835404028352916020019161273f565b820191905f5260205f20905b81548152906001019060200180831161272257829003601f168201915b50505050506143b2565b90505f6127568284613c96565b905080856007015f8a81526020019081526020015f2080805480602002602001604051908101604052809291908181526020015f905b82821015612885578382905f5260205f2090600202016040518060400160405290815f82015f9054906101000a900460ff1660018111156127d0576127cf615603565b5b60018111156127e2576127e1615603565b5b81526020016001820180546127f690616636565b80601f016020809104026020016040519081016040528092919081815260200182805461282290616636565b801561286d5780601f106128445761010080835404028352916020019161286d565b820191905f5260205f20905b81548152906001019060200180831161285057829003601f168201915b5050505050815250508152602001906001019061278c565b505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156128f6573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061291a91906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461298957336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161298091906164ef565b60405180910390fd5b5f6129926136a5565b9050806001015f8381526020019081526020015f205f9054906101000a900460ff166129f557816040517fadfab9040000000000000000000000000000000000000000000000000000000081526004016129ec91906153d3565b60405180910390fd5b5f6129ff846145a1565b915050604051806040016040528084815260200160011515815250826011015f8381526020019081526020015f205f820151815f01556020820151816001015f6101000a81548160ff02191690831515021790555090505050505050565b6040518060400160405280600581526020017f352e302e3000000000000000000000000000000000000000000000000000000081525081565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612af3573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612b1791906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612b8657336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612b7d91906164ef565b60405180910390fd5b5f612b8f6136a5565b905086816011015f8881526020019081526020015f205f015414612bec5785876040517f9431f34e000000000000000000000000000000000000000000000000000000008152600401612be3929190617102565b60405180910390fd5b806001015f8781526020019081526020015f205f9054906101000a900460ff16612c42576040517f6fbcdd2b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8585905003612c8957866040517fe6f9083b000000000000000000000000000000000000000000000000000000008152600401612c8091906153d3565b60405180910390fd5b85816012015f8981526020019081526020015f20819055507fa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa8187848488886001604051612cdb96959493929190617255565b60405180910390a150505050505050565b5f80612cf66136a5565b905080600c015491505090565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612d60573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d8491906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612df357336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401612dea91906164ef565b60405180910390fd5b5f612dfc6136a5565b90508060040154821180612e27575060f860036008811115612e2157612e20615603565b5b901b8211155b15612e6957816040517ffcf2db7a000000000000000000000000000000000000000000000000000000008152600401612e6091906153d3565b60405180910390fd5b5f816006015f8481526020019081526020015f20549050816001015f8281526020019081526020015f205f9054906101000a900460ff1615612ee257826040517f92789b67000000000000000000000000000000000000000000000000000000008152600401612ed991906153d3565b60405180910390fd5b6001826001015f8581526020019081526020015f205f6101000a81548160ff0219169083151502179055505f8114612f40576001826001015f8381526020019081526020015f205f6101000a81548160ff0219169083151502179055505b7f2b087b884b35a81d769d1a1e092880f1da56de964e4b339eabcb1f45f5fe326483604051612f6f91906153d3565b60405180910390a1505050565b60035f612f876137ba565b9050805f0160089054906101000a900460ff1680612fcf57508167ffffffffffffffff16815f015f9054906101000a900467ffffffffffffffff1667ffffffffffffffff1610155b15613006576040517ff92ee8a900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81815f015f6101000a81548167ffffffffffffffff021916908367ffffffffffffffff1602179055506001815f0160086101000a81548160ff0219169083151502179055505f6130546136a5565b90505f600160f86004600881111561306f5761306e615603565b5b901b61307b91906172aa565b90505b816005015481116130e1575f801b826003015f8381526020019081526020015f2054146130ce5781600f0181908060018154018082558091505060019003905f5260205f20015f90919091909150555b80806130d990616570565b91505061307e565b505f600160f8600560088111156130fb576130fa615603565b5b901b61310791906172aa565b90505b8160090154811161316d575f801b826003015f8381526020019081526020015f20541461315a578160100181908060018154018082558091505060019003905f5260205f20015f90919091909150555b808061316590616570565b91505061310a565b50505f815f0160086101000a81548160ff0219169083151502179055507fc7f505b2f371ae2175ee4913f4499e1f2633a7b5936321eed1cdaeb6115181d2826040516131b9919061652a565b60405180910390a15050565b6060805f6131d16136a5565b9050806001015f8581526020019081526020015f205f9054906101000a900460ff1661323457836040517fda32d00f00000000000000000000000000000000000000000000000000000000815260040161322b91906153d3565b60405180910390fd5b5f816003015f8681526020019081526020015f205490505f801b810361329157846040517fd5fd3cd700000000000000000000000000000000000000000000000000000000815260040161328891906153d3565b60405180910390fd5b5f826002015f8781526020019081526020015f205f8381526020019081526020015f2080548060200260200160405190810160405280929190818152602001828054801561333157602002820191905f5260205f20905b815f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190600101908083116132e8575b505050505090505f6133db84600e015f8981526020019081526020015f20805461335a90616636565b80601f016020809104026020016040519081016040528092919081815260200182805461338690616636565b80156133d15780601f106133a8576101008083540402835291602001916133d1565b820191905f5260205f20905b8154815290600101906020018083116133b457829003601f168201915b50505050506143b2565b90505f6133e88284613c96565b90508085600b015f8a81526020019081526020015f2080805461340a90616636565b80601f016020809104026020016040519081016040528092919081815260200182805461343690616636565b80156134815780601f1061345857610100808354040283529160200191613481565b820191905f5260205f20905b81548152906001019060200180831161346457829003601f168201915b50505050509050965096505050505050915091565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156134f3573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061351791906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161461358657336040517f21bfda1000000000000000000000000000000000000000000000000000000000815260040161357d91906164ef565b60405180910390fd5b61358f816145a1565b505050565b5f8061359e6136a5565b9050806008015491505090565b60605f6135b66136a5565b90508060100180548060200260200160405190810160405280929190818152602001828054801561360457602002820191905f5260205f20905b8154815260200190600101908083116135f0575b505050505091505090565b60605f61361a6136a5565b905080600f0180548060200260200160405190810160405280929190818152602001828054801561366857602002820191905f5260205f20905b815481526020019060010190808311613654575b505050505091505090565b5f8061367d6136a5565b6012015f8481526020019081526020015f20540361369b575f61369e565b60015b9050919050565b5f7f26fdaf8a2cb20d20b55e36218986905e534ee7a970dd2fa827946e4b7496db00905090565b60605f60016136da84614800565b0190505f8167ffffffffffffffff8111156136f8576136f7615898565b5b6040519080825280601f01601f19166020018201604052801561372a5781602001600182028036833780820191505090505b5090505f82602001820190505b60011561378b578080600190039150507f3031323334353637383961626364656600000000000000000000000000000000600a86061a8153600a85816137805761377f6172dd565b5b0494505f8503613737575b819350505050919050565b5f61379f6137ba565b5f015f9054906101000a900467ffffffffffffffff16905090565b5f7ff0c57e16840df040f15088dc2f81fe391c3923bec73e23a9662efc9c229c6a00905090565b6137e9614951565b6137f38282614991565b5050565b60606002838360405160200161380f9392919061736a565b604051602081830303815290604052905092915050565b60605f806138326136a5565b905080600e015f8581526020019081526020015f20805461385290616636565b80601f016020809104026020016040519081016040528092919081815260200182805461387e90616636565b80156138c95780601f106138a0576101008083540402835291602001916138c9565b820191905f5260205f20905b8154815290600101906020018083116138ac57829003601f168201915b505050505092506138d9836143b2565b91507344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166346c5bbbd83336040518363ffffffff1660e01b815260040161392a92919061691c565b602060405180830381865afa158015613945573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061396991906173d0565b6139aa57336040517faee863230000000000000000000000000000000000000000000000000000000081526004016139a191906164ef565b60405180910390fd5b50915091565b5f808484905067ffffffffffffffff8111156139cf576139ce615898565b5b6040519080825280602002602001820160405280156139fd5781602001602082028036833780820191505090505b5090505f5b85859050811015613b0157604051806060016040528060258152602001617bb86025913980519060200120868683818110613a4057613a3f616ba4565b5b9050602002810190613a529190616bdd565b5f016020810190613a6391906173fb565b878784818110613a7657613a75616ba4565b5b9050602002810190613a889190616bdd565b8060200190613a979190616c83565b604051613aa5929190617454565b6040518091039020604051602001613abf9392919061747b565b60405160208183030381529060405280519060200120828281518110613ae857613ae7616ba4565b5b6020026020010181815250508080600101915050613a02565b50613b856040518060c0016040528060828152602001617b366082913980519060200120888884604051602001613b389190617561565b604051602081830303815290604052805190602001208780519060200120604051602001613b6a959493929190617577565b604051602081830303815290604052805190602001206149e2565b91505095945050505050565b5f80613be08585858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f81840152601f19601f820116905080830192505050505050506149fb565b9050613bed868233614a25565b80915050949350505050565b5f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166341ad069c856040518263ffffffff1660e01b8152600401613c4891906153d3565b602060405180830381865afa158015613c63573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613c8791906175c8565b90508083101591505092915050565b60605f825190505f8167ffffffffffffffff811115613cb857613cb7615898565b5b604051908082528060200260200182016040528015613ceb57816020015b6060815260200190600190039081613cd65790505b5090505f5b82811015613dd2577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c887878481518110613d3c57613d3b616ba4565b5b60200260200101516040518363ffffffff1660e01b8152600401613d6192919061691c565b5f60405180830381865afa158015613d7b573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190613da39190617746565b60600151828281518110613dba57613db9616ba4565b5b60200260200101819052508080600101915050613cf0565b50809250505092915050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161480613e8b57507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16613e72614c06565b73ffffffffffffffffffffffffffffffffffffffff1614155b15613ec2576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b7350157cffd6bbfa2dece204a89ec419c23ef5755d73ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015613f21573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613f4591906164c4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614613fb457336040517f21bfda10000000000000000000000000000000000000000000000000000000008152600401613fab91906164ef565b60405180910390fd5b50565b8173ffffffffffffffffffffffffffffffffffffffff166352d1902d6040518163ffffffff1660e01b8152600401602060405180830381865afa92505050801561401f57506040513d601f19601f8201168201806040525081019061401c91906177b7565b60015b61406057816040517f4c9c8ce300000000000000000000000000000000000000000000000000000000815260040161405791906164ef565b60405180910390fd5b7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b81146140c657806040517faa1d49a40000000000000000000000000000000000000000000000000000000081526004016140bd9190615a2e565b60405180910390fd5b6140d08383614c59565b505050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163073ffffffffffffffffffffffffffffffffffffffff161461415a576040517fe07c8dba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f6141b66040518060600160405280603c8152602001617aa4603c91398051906020012084848051906020012060405160200161419b939291906177e2565b604051602081830303815290604052805190602001206149e2565b905092915050565b5f614244604051806080016040528060568152602001617ae06056913980519060200120878787876040516020016141f7929190617454565b604051602081830303815290604052805190602001208680519060200120604051602001614229959493929190617577565b604051602081830303815290604052805190602001206149e2565b905095945050505050565b5f7fa16a46d94261c7517cc8ff89f61c0ce93598e3c849801011dee649a6a557d100905090565b60605f61428161424f565b905080600201805461429290616636565b80601f01602080910402602001604051908101604052809291908181526020018280546142be90616636565b80156143095780601f106142e057610100808354040283529160200191614309565b820191905f5260205f20905b8154815290600101906020018083116142ec57829003601f168201915b505050505091505090565b60605f61431f61424f565b905080600301805461433090616636565b80601f016020809104026020016040519081016040528092919081815260200182805461435c90616636565b80156143a75780601f1061437e576101008083540402835291602001916143a7565b820191905f5260205f20905b81548152906001019060200180831161438a57829003601f168201915b505050505091505090565b5f80825114806143e457505f825f815181106143d1576143d0616ba4565b5b602001015160f81c60f81b60f81c60ff16145b15614471577344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff1663976f3eb96040518163ffffffff1660e01b8152600401602060405180830381865afa158015614446573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061446a91906175c8565b905061459c565b5f825f8151811061448557614484616ba4565b5b602001015160f81c60f81b60f81c9050600160ff168160ff16141580156144b35750600260ff168160ff1614155b156144f557806040517f2139cc2c0000000000000000000000000000000000000000000000000000000081526004016144ec9190617826565b60405180910390fd5b600160ff168160ff1614801561450d57506021835114155b15614544576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600260ff168160ff1614801561455c57506041835114155b15614593576040517f8b248b6000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60218301519150505b919050565b5f805f6145ac6136a5565b90505f8160050154905060f8600460088111156145cc576145cb615603565b5b901b81141580156145fa5750816001015f8281526020019081526020015f205f9054906101000a900460ff16155b1561463c57806040517f3b853da800000000000000000000000000000000000000000000000000000000815260040161463391906153d3565b60405180910390fd5b816004015f81548092919061465090616570565b919050555081600401549350816005015f81548092919061467090616570565b91905055508160050154925082826006015f8681526020019081526020015f208190555083826006015f8581526020019081526020015f20819055508482600d015f8681526020019081526020015f205f6101000a81548160ff021916908360018111156146e1576146e0615603565b5b02179055505f807344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166365b394af6040518163ffffffff1660e01b81526004016040805180830381865afa158015614744573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061476891906165cb565b915091505f61477783836137f7565b90508085600e015f8981526020019081526020015f2090816147999190616803565b508085600e015f8881526020019081526020015f2090816147ba9190616803565b507ffbf5274810b94f86970c1147e8ffaebed246ee9777d695a69004dc6256d1fe918789836040516147ee9392919061783f565b60405180910390a15050505050915091565b5f805f90507a184f03e93ff9f4daa797ed6e38ed64bf6a1f010000000000000000831061485c577a184f03e93ff9f4daa797ed6e38ed64bf6a1f0100000000000000008381614852576148516172dd565b5b0492506040810190505b6d04ee2d6d415b85acef81000000008310614899576d04ee2d6d415b85acef8100000000838161488f5761488e6172dd565b5b0492506020810190505b662386f26fc1000083106148c857662386f26fc1000083816148be576148bd6172dd565b5b0492506010810190505b6305f5e10083106148f1576305f5e10083816148e7576148e66172dd565b5b0492506008810190505b612710831061491657612710838161490c5761490b6172dd565b5b0492506004810190505b60648310614939576064838161492f5761492e6172dd565b5b0492506002810190505b600a8310614948576001810190505b80915050919050565b614959614ccb565b61498f576040517fd7e6bcf800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b614999614951565b5f6149a261424f565b9050828160020190816149b591906178d3565b50818160030190816149c791906178d3565b505f801b815f01819055505f801b8160010181905550505050565b5f6149f46149ee614ce9565b83614cf7565b9050919050565b5f805f80614a098686614d37565b925092509250614a198282614d8c565b82935050505092915050565b7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff16639447cfd484846040518363ffffffff1660e01b8152600401614a7492919061691c565b602060405180830381865afa158015614a8f573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190614ab391906173d0565b614af65781816040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614aed9291906179a2565b60405180910390fd5b5f7344aa028fd264c76bf4a8f8b4d8a5272f6ae25cac73ffffffffffffffffffffffffffffffffffffffff166331ff41c885846040518363ffffffff1660e01b8152600401614b4692919061691c565b5f60405180830381865afa158015614b60573d5f803e3d5ffd5b505050506040513d5f823e3d601f19601f82011682018060405250810190614b889190617746565b90508273ffffffffffffffffffffffffffffffffffffffff16816020015173ffffffffffffffffffffffffffffffffffffffff1614614c005782826040517f0d86f521000000000000000000000000000000000000000000000000000000008152600401614bf79291906179a2565b60405180910390fd5b50505050565b5f614c327f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614eee565b5f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16905090565b614c6282614ef7565b8173ffffffffffffffffffffffffffffffffffffffff167fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b60405160405180910390a25f81511115614cbe57614cb88282614fc0565b50614cc7565b614cc6615040565b5b5050565b5f614cd46137ba565b5f0160089054906101000a900460ff16905090565b5f614cf261507c565b905090565b5f6040517f190100000000000000000000000000000000000000000000000000000000000081528360028201528260228201526042812091505092915050565b5f805f6041845103614d77575f805f602087015192506040870151915060608701515f1a9050614d69888285856150df565b955095509550505050614d85565b5f600285515f1b9250925092505b9250925092565b5f6003811115614d9f57614d9e615603565b5b826003811115614db257614db1615603565b5b0315614eea5760016003811115614dcc57614dcb615603565b5b826003811115614ddf57614dde615603565b5b03614e16576040517ff645eedf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60026003811115614e2a57614e29615603565b5b826003811115614e3d57614e3c615603565b5b03614e8157805f1c6040517ffce698f7000000000000000000000000000000000000000000000000000000008152600401614e7891906153d3565b60405180910390fd5b600380811115614e9457614e93615603565b5b826003811115614ea757614ea6615603565b5b03614ee957806040517fd78bce0c000000000000000000000000000000000000000000000000000000008152600401614ee09190615a2e565b60405180910390fd5b5b5050565b5f819050919050565b5f8173ffffffffffffffffffffffffffffffffffffffff163b03614f5257806040517f4c9c8ce3000000000000000000000000000000000000000000000000000000008152600401614f4991906164ef565b60405180910390fd5b80614f7e7f360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc5f1b614eee565b5f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b60605f808473ffffffffffffffffffffffffffffffffffffffff1684604051614fe991906179f9565b5f60405180830381855af49150503d805f8114615021576040519150601f19603f3d011682016040523d82523d5f602084013e615026565b606091505b50915091506150368583836151c6565b9250505092915050565b5f34111561507a576040517fb398979f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f6150a6615253565b6150ae6152c9565b46306040516020016150c4959493929190617a0f565b60405160208183030381529060405280519060200120905090565b5f805f7f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a0845f1c111561511b575f6003859250925092506151bc565b5f6001888888886040515f815260200160405260405161513e9493929190617a60565b6020604051602081039080840390855afa15801561515e573d5f803e3d5ffd5b5050506020604051035190505f73ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16036151af575f60015f801b935093509350506151bc565b805f805f1b935093509350505b9450945094915050565b6060826151db576151d682615340565b61524b565b5f825114801561520157505f8473ffffffffffffffffffffffffffffffffffffffff163b145b1561524357836040517f9996b31500000000000000000000000000000000000000000000000000000000815260040161523a91906164ef565b60405180910390fd5b81905061524c565b5b9392505050565b5f8061525d61424f565b90505f615268614276565b90505f81511115615284578080519060200120925050506152c6565b5f825f015490505f801b811461529f578093505050506152c6565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f806152d361424f565b90505f6152de614314565b90505f815111156152fa5780805190602001209250505061533d565b5f826001015490505f801b81146153165780935050505061533d565b7fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47093505050505b90565b5f815111156153525780518082602001fd5b6040517fd6bda27500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60405180608001604052805f81526020015f81526020015f60018111156153ae576153ad615603565b5b8152602001606081525090565b5f819050919050565b6153cd816153bb565b82525050565b5f6020820190506153e65f8301846153c4565b92915050565b5f81519050919050565b5f82825260208201905092915050565b5f5b83811015615423578082015181840152602081019050615408565b5f8484015250505050565b5f601f19601f8301169050919050565b5f615448826153ec565b61545281856153f6565b9350615462818560208601615406565b61546b8161542e565b840191505092915050565b5f6020820190508181035f83015261548e818461543e565b905092915050565b5f604051905090565b5f80fd5b5f80fd5b6154b0816153bb565b81146154ba575f80fd5b50565b5f813590506154cb816154a7565b92915050565b5f602082840312156154e6576154e561549f565b5b5f6154f3848285016154bd565b91505092915050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61554e82615525565b9050919050565b61555e81615544565b82525050565b5f61556f8383615555565b60208301905092915050565b5f602082019050919050565b5f615591826154fc565b61559b8185615506565b93506155a683615516565b805f5b838110156155d65781516155bd8882615564565b97506155c88361557b565b9250506001810190506155a9565b5085935050505092915050565b5f6020820190508181035f8301526155fb8184615587565b905092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061564157615640615603565b5b50565b5f81905061565182615630565b919050565b5f61566082615644565b9050919050565b61567081615656565b82525050565b5f6020820190506156895f830184615667565b92915050565b6002811061569b575f80fd5b50565b5f813590506156ac8161568f565b92915050565b5f80604083850312156156c8576156c761549f565b5b5f6156d5858286016154bd565b92505060206156e68582860161569e565b9150509250929050565b5f8115159050919050565b615704816156f0565b82525050565b5f60208201905061571d5f8301846156fb565b92915050565b5f80fd5b5f80fd5b5f80fd5b5f8083601f84011261574457615743615723565b5b8235905067ffffffffffffffff81111561576157615760615727565b5b60208301915083602082028301111561577d5761577c61572b565b5b9250929050565b5f8083601f84011261579957615798615723565b5b8235905067ffffffffffffffff8111156157b6576157b5615727565b5b6020830191508360018202830111156157d2576157d161572b565b5b9250929050565b5f805f805f606086880312156157f2576157f161549f565b5b5f6157ff888289016154bd565b955050602086013567ffffffffffffffff8111156158205761581f6154a3565b5b61582c8882890161572f565b9450945050604086013567ffffffffffffffff81111561584f5761584e6154a3565b5b61585b88828901615784565b92509250509295509295909350565b61587381615544565b811461587d575f80fd5b50565b5f8135905061588e8161586a565b92915050565b5f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6158ce8261542e565b810181811067ffffffffffffffff821117156158ed576158ec615898565b5b80604052505050565b5f6158ff615496565b905061590b82826158c5565b919050565b5f67ffffffffffffffff82111561592a57615929615898565b5b6159338261542e565b9050602081019050919050565b828183375f83830152505050565b5f61596061595b84615910565b6158f6565b90508281526020810184848401111561597c5761597b615894565b5b615987848285615940565b509392505050565b5f82601f8301126159a3576159a2615723565b5b81356159b384826020860161594e565b91505092915050565b5f80604083850312156159d2576159d161549f565b5b5f6159df85828601615880565b925050602083013567ffffffffffffffff811115615a00576159ff6154a3565b5b615a0c8582860161598f565b9150509250929050565b5f819050919050565b615a2881615a16565b82525050565b5f602082019050615a415f830184615a1f565b92915050565b5f8083601f840112615a5c57615a5b615723565b5b8235905067ffffffffffffffff811115615a7957615a78615727565b5b602083019150836020820283011115615a9557615a9461572b565b5b9250929050565b5f805f805f8060808789031215615ab657615ab561549f565b5b5f615ac389828a016154bd565b965050602087013567ffffffffffffffff811115615ae457615ae36154a3565b5b615af089828a01615a47565b9550955050604087013567ffffffffffffffff811115615b1357615b126154a3565b5b615b1f89828a01615a47565b93509350506060615b3289828a016154bd565b9150509295509295509295565b5f805f60408486031215615b5657615b5561549f565b5b5f615b63868287016154bd565b935050602084013567ffffffffffffffff811115615b8457615b836154a3565b5b615b9086828701615784565b92509250509250925092565b615ba5816153bb565b82525050565b615bb481615656565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b60028110615bf457615bf3615603565b5b50565b5f819050615c0482615be3565b919050565b5f615c1382615bf7565b9050919050565b615c2381615c09565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f615c4d82615c29565b615c578185615c33565b9350615c67818560208601615406565b615c708161542e565b840191505092915050565b5f604083015f830151615c905f860182615c1a565b5060208301518482036020860152615ca88282615c43565b9150508091505092915050565b5f615cc08383615c7b565b905092915050565b5f602082019050919050565b5f615cde82615bba565b615ce88185615bc4565b935083602082028501615cfa85615bd4565b805f5b85811015615d355784840389528151615d168582615cb5565b9450615d2183615cc8565b925060208a01995050600181019050615cfd565b50829750879550505050505092915050565b5f608083015f830151615d5c5f860182615b9c565b506020830151615d6f6020860182615b9c565b506040830151615d826040860182615bab565b5060608301518482036060860152615d9a8282615cd4565b9150508091505092915050565b5f6020820190508181035f830152615dbf8184615d47565b905092915050565b5f805f805f60608688031215615de057615ddf61549f565b5b5f615ded888289016154bd565b955050602086013567ffffffffffffffff811115615e0e57615e0d6154a3565b5b615e1a88828901615784565b9450945050604086013567ffffffffffffffff811115615e3d57615e3c6154a3565b5b615e4988828901615784565b92509250509295509295909350565b5f7fff0000000000000000000000000000000000000000000000000000000000000082169050919050565b615e8c81615e58565b82525050565b615e9b81615544565b82525050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f615ed58383615b9c565b60208301905092915050565b5f602082019050919050565b5f615ef782615ea1565b615f018185615eab565b9350615f0c83615ebb565b805f5b83811015615f3c578151615f238882615eca565b9750615f2e83615ee1565b925050600181019050615f0f565b5085935050505092915050565b5f60e082019050615f5c5f83018a615e83565b8181036020830152615f6e818961543e565b90508181036040830152615f82818861543e565b9050615f9160608301876153c4565b615f9e6080830186615e92565b615fab60a0830185615a1f565b81810360c0830152615fbd8184615eed565b905098975050505050505050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f82825260208201905092915050565b5f61600e826153ec565b6160188185615ff4565b9350616028818560208601615406565b6160318161542e565b840191505092915050565b5f6160478383616004565b905092915050565b5f602082019050919050565b5f61606582615fcb565b61606f8185615fd5565b93508360208202850161608185615fe5565b805f5b858110156160bc578484038952815161609d858261603c565b94506160a88361604f565b925060208a01995050600181019050616084565b50829750879550505050505092915050565b5f82825260208201905092915050565b5f6160e882615bba565b6160f281856160ce565b93508360208202850161610485615bd4565b805f5b8581101561613f57848403895281516161208582615cb5565b945061612b83615cc8565b925060208a01995050600181019050616107565b50829750879550505050505092915050565b5f6040820190508181035f830152616169818561605b565b9050818103602083015261617d81846160de565b90509392505050565b5f806040838503121561619c5761619b61549f565b5b5f6161a98582860161569e565b92505060206161ba858286016154bd565b9150509250929050565b5f8083601f8401126161d9576161d8615723565b5b8235905067ffffffffffffffff8111156161f6576161f5615727565b5b6020830191508360208202830111156162125761621161572b565b5b9250929050565b5f805f805f80608087890312156162335761623261549f565b5b5f61624089828a016154bd565b965050602061625189828a016154bd565b955050604087013567ffffffffffffffff811115616272576162716154a3565b5b61627e89828a0161572f565b9450945050606087013567ffffffffffffffff8111156162a1576162a06154a3565b5b6162ad89828a016161c4565b92509250509295509295509295565b5f82825260208201905092915050565b5f6162d682615c29565b6162e081856162bc565b93506162f0818560208601615406565b6162f98161542e565b840191505092915050565b5f6040820190508181035f83015261631c818561605b565b9050818103602083015261633081846162cc565b90509392505050565b5f6020828403121561634e5761634d61549f565b5b5f61635b8482850161569e565b91505092915050565b5f6020820190508181035f83015261637c8184615eed565b905092915050565b5f81905092915050565b5f616398826153ec565b6163a28185616384565b93506163b2818560208601615406565b80840191505092915050565b7f20760000000000000000000000000000000000000000000000000000000000005f82015250565b5f6163f2600283616384565b91506163fd826163be565b600282019050919050565b7f2e000000000000000000000000000000000000000000000000000000000000005f82015250565b5f61643c600183616384565b915061644782616408565b600182019050919050565b5f61645d828761638e565b9150616468826163e6565b9150616474828661638e565b915061647f82616430565b915061648b828561638e565b915061649682616430565b91506164a2828461638e565b915081905095945050505050565b5f815190506164be8161586a565b92915050565b5f602082840312156164d9576164d861549f565b5b5f6164e6848285016164b0565b91505092915050565b5f6020820190506165025f830184615e92565b92915050565b5f67ffffffffffffffff82169050919050565b61652481616508565b82525050565b5f60208201905061653d5f83018461651b565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f61657a826153bb565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82036165ac576165ab616543565b5b600182019050919050565b5f815190506165c5816154a7565b92915050565b5f80604083850312156165e1576165e061549f565b5b5f6165ee858286016165b7565b92505060206165ff858286016165b7565b9150509250929050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602260045260245ffd5b5f600282049050600182168061664d57607f821691505b6020821081036166605761665f616609565b5b50919050565b5f819050815f5260205f209050919050565b5f6020601f8301049050919050565b5f82821b905092915050565b5f600883026166c27fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff82616687565b6166cc8683616687565b95508019841693508086168417925050509392505050565b5f819050919050565b5f6167076167026166fd846153bb565b6166e4565b6153bb565b9050919050565b5f819050919050565b616720836166ed565b61673461672c8261670e565b848454616693565b825550505050565b5f90565b61674861673c565b616753818484616717565b505050565b5b818110156167765761676b5f82616740565b600181019050616759565b5050565b601f8211156167bb5761678c81616666565b61679584616678565b810160208510156167a4578190505b6167b86167b085616678565b830182616758565b50505b505050565b5f82821c905092915050565b5f6167db5f19846008026167c0565b1980831691505092915050565b5f6167f383836167cc565b9150826002028217905092915050565b61680c82615c29565b67ffffffffffffffff81111561682557616824615898565b5b61682f8254616636565b61683a82828561677a565b5f60209050601f83116001811461686b575f8415616859578287015190505b61686385826167e8565b8655506168ca565b601f19841661687986616666565b5f5b828110156168a05784890151825560018201915060208501945060208101905061687b565b868310156168bd57848901516168b9601f8916826167cc565b8355505b6001600288020188555050505b505050505050565b5f6080820190506168e55f8301876153c4565b6168f260208301866153c4565b6168ff6040830185615667565b818103606083015261691181846162cc565b905095945050505050565b5f60408201905061692f5f8301856153c4565b61693c6020830184615e92565b9392505050565b5f819050919050565b60028110616958575f80fd5b50565b5f813590506169698161694c565b92915050565b5f61697d602084018461695b565b905092915050565b5f80fd5b5f80fd5b5f80fd5b5f80833560016020038436030381126169ad576169ac61698d565b5b83810192508235915060208301925067ffffffffffffffff8211156169d5576169d4616985565b5b6001820236038313156169eb576169ea616989565b5b509250929050565b5f6169fe8385615c33565b9350616a0b838584615940565b616a148361542e565b840190509392505050565b5f60408301616a305f84018461696f565b616a3c5f860182615c1a565b50616a4a6020840184616991565b8583036020870152616a5d8382846169f3565b925050508091505092915050565b5f616a768383616a1f565b905092915050565b5f82356001604003833603038112616a9957616a9861698d565b5b82810191505092915050565b5f602082019050919050565b5f616abc83856160ce565b935083602084028501616ace84616943565b805f5b87811015616b11578484038952616ae88284616a7e565b616af28582616a6b565b9450616afd83616aa5565b925060208a01995050600181019050616ad1565b50829750879450505050509392505050565b5f616b2e83856162bc565b9350616b3b838584615940565b616b448361542e565b840190509392505050565b5f608082019050616b625f8301896153c4565b8181036020830152616b75818789616ab1565b90508181036040830152616b8a818587616b23565b9050616b996060830184615e92565b979650505050505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f80fd5b5f80fd5b5f80fd5b5f82356001604003833603038112616bf857616bf7616bd1565b5b80830191505092915050565b5f8135616c108161694c565b80915050919050565b5f815f1b9050919050565b5f60ff616c3084616c19565b9350801983169250808416831791505092915050565b5f616c5082615bf7565b9050919050565b5f819050919050565b616c6982616c46565b616c7c616c7582616c57565b8354616c24565b8255505050565b5f8083356001602003843603038112616c9f57616c9e616bd1565b5b80840192508235915067ffffffffffffffff821115616cc157616cc0616bd5565b5b602083019250600182023603831315616cdd57616cdc616bd9565b5b509250929050565b5f82905092915050565b616cf98383616ce5565b67ffffffffffffffff811115616d1257616d11615898565b5b616d1c8254616636565b616d2782828561677a565b5f601f831160018114616d54575f8415616d42578287013590505b616d4c85826167e8565b865550616db3565b601f198416616d6286616666565b5f5b82811015616d8957848901358255600182019150602085019450602081019050616d64565b86831015616da65784890135616da2601f8916826167cc565b8355505b6001600288020188555050505b50505050505050565b616dc7838383616cef565b505050565b5f81015f830180616ddc81616c04565b9050616de88184616c60565b5050506001810160208301616dfd8185616c83565b616e08818386616dbc565b505050505050565b616e1a8282616dcc565b5050565b5f606082019050616e315f8301876153c4565b8181036020830152616e43818661605b565b90508181036040830152616e58818486616ab1565b905095945050505050565b5f80fd5b82818337505050565b5f616e7b8385615eab565b93507f07ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff831115616eae57616ead616e63565b5b602083029250616ebf838584616e67565b82840190509392505050565b5f60a082019050616ede5f83018a6153c4565b8181036020830152616ef181888a616e70565b90508181036040830152616f06818688616e70565b9050616f1560608301856153c4565b616f2260808301846153c4565b98975050505050505050565b5f606082019050616f415f8301876153c4565b8181036020830152616f54818587616b23565b9050616f636040830184615e92565b95945050505050565b5f60a082019050616f7f5f8301886153c4565b616f8c60208301876153c4565b616f9960408301866153c4565b616fa660608301856156fb565b8181036080830152616fb881846162cc565b90509695505050505050565b5f606082019050616fd75f8301866153c4565b616fe460208301856153c4565b8181036040830152616ff681846162cc565b9050949350505050565b5f6080820190506170135f8301896153c4565b8181036020830152617026818789616b23565b9050818103604083015261703b818587616b23565b905061704a6060830184615e92565b979650505050505050565b5f6060820190506170685f8301876153c4565b818103602083015261707a818661605b565b9050818103604083015261708f818486616b23565b905095945050505050565b7f4549503731323a20556e696e697469616c697a656400000000000000000000005f82015250565b5f6170ce6015836153f6565b91506170d98261709a565b602082019050919050565b5f6020820190508181035f8301526170fb816170c2565b9050919050565b5f6040820190506171155f8301856153c4565b61712260208301846153c4565b9392505050565b5f819050919050565b5f61713d8385615ff4565b935061714a838584615940565b6171538361542e565b840190509392505050565b5f61716a848484617132565b90509392505050565b5f808335600160200384360303811261718f5761718e61698d565b5b83810192508235915060208301925067ffffffffffffffff8211156171b7576171b6616985565b5b6001820236038313156171cd576171cc616989565b5b509250929050565b5f602082019050919050565b5f6171ec8385615fd5565b9350836020840285016171fe84617129565b805f5b878110156172435784840389526172188284617173565b61722386828461715e565b955061722e846171d5565b935060208b019a505050600181019050617201565b50829750879450505050509392505050565b5f6080820190506172685f8301896153c4565b818103602083015261727b8187896171e1565b90508181036040830152617290818587616ab1565b905061729f60608301846153c4565b979650505050505050565b5f6172b4826153bb565b91506172bf836153bb565b92508282019050808211156172d7576172d6616543565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f60ff82169050919050565b5f8160f81b9050919050565b5f61732c82617316565b9050919050565b61734461733f8261730a565b617322565b82525050565b5f819050919050565b61736461735f826153bb565b61734a565b82525050565b5f6173758286617333565b6001820191506173858285617353565b6020820191506173958284617353565b602082019150819050949350505050565b6173af816156f0565b81146173b9575f80fd5b50565b5f815190506173ca816173a6565b92915050565b5f602082840312156173e5576173e461549f565b5b5f6173f2848285016173bc565b91505092915050565b5f602082840312156174105761740f61549f565b5b5f61741d8482850161695b565b91505092915050565b5f81905092915050565b5f61743b8385617426565b9350617448838584615940565b82840190509392505050565b5f617460828486617430565b91508190509392505050565b61747581615c09565b82525050565b5f60608201905061748e5f830186615a1f565b61749b602083018561746c565b6174a86040830184615a1f565b949350505050565b5f81519050919050565b5f81905092915050565b5f819050602082019050919050565b6174dc81615a16565b82525050565b5f6174ed83836174d3565b60208301905092915050565b5f602082019050919050565b5f61750f826174b0565b61751981856174ba565b9350617524836174c4565b805f5b8381101561755457815161753b88826174e2565b9750617546836174f9565b925050600181019050617527565b5085935050505092915050565b5f61756c8284617505565b915081905092915050565b5f60a08201905061758a5f830188615a1f565b61759760208301876153c4565b6175a460408301866153c4565b6175b16060830185615a1f565b6175be6080830184615a1f565b9695505050505050565b5f602082840312156175dd576175dc61549f565b5b5f6175ea848285016165b7565b91505092915050565b5f80fd5b5f80fd5b5f67ffffffffffffffff82111561761557617614615898565b5b61761e8261542e565b9050602081019050919050565b5f61763d617638846175fb565b6158f6565b90508281526020810184848401111561765957617658615894565b5b617664848285615406565b509392505050565b5f82601f8301126176805761767f615723565b5b815161769084826020860161762b565b91505092915050565b5f608082840312156176ae576176ad6175f3565b5b6176b860806158f6565b90505f6176c7848285016164b0565b5f8301525060206176da848285016164b0565b602083015250604082015167ffffffffffffffff8111156176fe576176fd6175f7565b5b61770a8482850161766c565b604083015250606082015167ffffffffffffffff81111561772e5761772d6175f7565b5b61773a8482850161766c565b60608301525092915050565b5f6020828403121561775b5761775a61549f565b5b5f82015167ffffffffffffffff811115617778576177776154a3565b5b61778484828501617699565b91505092915050565b61779681615a16565b81146177a0575f80fd5b50565b5f815190506177b18161778d565b92915050565b5f602082840312156177cc576177cb61549f565b5b5f6177d9848285016177a3565b91505092915050565b5f6060820190506177f55f830186615a1f565b61780260208301856153c4565b61780f6040830184615a1f565b949350505050565b6178208161730a565b82525050565b5f6020820190506178395f830184617817565b92915050565b5f6060820190506178525f8301866153c4565b61785f6020830185615667565b818103604083015261787181846162cc565b9050949350505050565b5f819050815f5260205f209050919050565b601f8211156178ce5761789f8161787b565b6178a884616678565b810160208510156178b7578190505b6178cb6178c385616678565b830182616758565b50505b505050565b6178dc826153ec565b67ffffffffffffffff8111156178f5576178f4615898565b5b6178ff8254616636565b61790a82828561788d565b5f60209050601f83116001811461793b575f8415617929578287015190505b61793385826167e8565b86555061799a565b601f1984166179498661787b565b5f5b828110156179705784890151825560018201915060208501945060208101905061794b565b8683101561798d5784890151617989601f8916826167cc565b8355505b6001600288020188555050505b505050505050565b5f6040820190506179b55f830185615e92565b6179c26020830184615e92565b9392505050565b5f6179d382615c29565b6179dd8185617426565b93506179ed818560208601615406565b80840191505092915050565b5f617a0482846179c9565b915081905092915050565b5f60a082019050617a225f830188615a1f565b617a2f6020830187615a1f565b617a3c6040830186615a1f565b617a4960608301856153c4565b617a566080830184615e92565b9695505050505050565b5f608082019050617a735f830187615a1f565b617a806020830186617817565b617a8d6040830185615a1f565b617a9a6060830184615a1f565b9594505050505056fe507265704b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c6279746573206578747261446174612943727367656e566572696669636174696f6e2875696e743235362063727349642c75696e74323536206d61784269744c656e6774682c6279746573206372734469676573742c627974657320657874726144617461294b657967656e566572696669636174696f6e2875696e7432353620707265704b657967656e49642c75696e74323536206b657949642c4b65794469676573745b5d206b6579446967657374732c627974657320657874726144617461294b65794469676573742875696e7438206b6579547970652c627974657320646967657374294b65794469676573742875696e7438206b6579547970652c62797465732064696765737429
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xD7W_5`\xE0\x1C\x80cb\x94\xF4b\x11a\x01\x01W\x80c\xC2\xC1\xFA\xEE\x11a\0\x94W\x80c\xD5/\x10\xEB\x11a\0cW\x80c\xD5/\x10\xEB\x14a\x06\x85W\x80c\xDA\xBDs/\x14a\x06\xAFW\x80c\xE4\x10\x11~\x14a\x06\xD9W\x80c\xF0\xF8\xCB\xC6\x14a\x07\x03Wa\x01\xD7V[\x80c\xC2\xC1\xFA\xEE\x14a\x05\xE2W\x80c\xC4\x11Xt\x14a\x06\nW\x80c\xC5[\x87$\x14a\x06 W\x80c\xCA\xA3g\xDB\x14a\x06]Wa\x01\xD7V[\x80c\xAA\xA4p\x16\x11a\0\xD0W\x80c\xAA\xA4p\x16\x14a\x05>W\x80c\xAD<\xB1\xCC\x14a\x05fW\x80c\xB5;<\xCC\x14a\x05\x90W\x80c\xBA\xFF!\x1E\x14a\x05\xB8Wa\x01\xD7V[\x80cb\x94\xF4b\x14a\x04mW\x80cb\x97\x87\x87\x14a\x04\xA9W\x80c\x84\xB0\x19n\x14a\x04\xD1W\x80c\x93f\x08\xAE\x14a\x05\x01Wa\x01\xD7V[\x80c<\x02\xF84\x11a\x01yW\x80cO\x1E\xF2\x86\x11a\x01HW\x80cO\x1E\xF2\x86\x14a\x03\xD7W\x80cR\xD1\x90-\x14a\x03\xF3W\x80cV\xA6\x10\xB4\x14a\x04\x1DW\x80cX\x9A\xDB\x0E\x14a\x04EWa\x01\xD7V[\x80c<\x02\xF84\x14a\x03\x0FW\x80c=^\xC7\xE3\x14a\x037W\x80cE\xAF&\x1B\x14a\x03sW\x80cF\x10\xFF\xE8\x14a\x03\xAFWa\x01\xD7V[\x80c\x17\x03\xC6\x1A\x11a\x01\xB5W\x80c\x17\x03\xC6\x1A\x14a\x02kW\x80c\x19\xF4\xF62\x14a\x02\x93W\x80c9\xF78\x10\x14a\x02\xCFW\x80c:\xC5\0r\x14a\x02\xE5Wa\x01\xD7V[\x80c\x0Bh\x073\x14a\x01\xDBW\x80c\r\x8En,\x14a\x02\x05W\x80c\x16\xC7\x13\xD9\x14a\x02/W[_\x80\xFD[4\x80\x15a\x01\xE6W_\x80\xFD[Pa\x01\xEFa\x07?V[`@Qa\x01\xFC\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x10W_\x80\xFD[Pa\x02\x19a\x07VV[`@Qa\x02&\x91\x90aTvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02:W_\x80\xFD[Pa\x02U`\x04\x806\x03\x81\x01\x90a\x02P\x91\x90aT\xD1V[a\x07\xD1V[`@Qa\x02b\x91\x90aU\xE3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02vW_\x80\xFD[Pa\x02\x91`\x04\x806\x03\x81\x01\x90a\x02\x8C\x91\x90aT\xD1V[a\x08\xA2V[\0[4\x80\x15a\x02\x9EW_\x80\xFD[Pa\x02\xB9`\x04\x806\x03\x81\x01\x90a\x02\xB4\x91\x90aT\xD1V[a\n\xD0V[`@Qa\x02\xC6\x91\x90aVvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\xDAW_\x80\xFD[Pa\x02\xE3a\x0B\xD6V[\0[4\x80\x15a\x02\xF0W_\x80\xFD[Pa\x02\xF9a\x0E&V[`@Qa\x03\x06\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\x1AW_\x80\xFD[Pa\x035`\x04\x806\x03\x81\x01\x90a\x030\x91\x90aV\xB2V[a\x0E=V[\0[4\x80\x15a\x03BW_\x80\xFD[Pa\x03]`\x04\x806\x03\x81\x01\x90a\x03X\x91\x90aT\xD1V[a\x114V[`@Qa\x03j\x91\x90aW\nV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03~W_\x80\xFD[Pa\x03\x99`\x04\x806\x03\x81\x01\x90a\x03\x94\x91\x90aT\xD1V[a\x11hV[`@Qa\x03\xA6\x91\x90aVvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03\xBAW_\x80\xFD[Pa\x03\xD5`\x04\x806\x03\x81\x01\x90a\x03\xD0\x91\x90aW\xD9V[a\x12VV[\0[a\x03\xF1`\x04\x806\x03\x81\x01\x90a\x03\xEC\x91\x90aY\xBCV[a\x17\xA6V[\0[4\x80\x15a\x03\xFEW_\x80\xFD[Pa\x04\x07a\x17\xC5V[`@Qa\x04\x14\x91\x90aZ.V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04(W_\x80\xFD[Pa\x04C`\x04\x806\x03\x81\x01\x90a\x04>\x91\x90aZ\x9CV[a\x17\xF6V[\0[4\x80\x15a\x04PW_\x80\xFD[Pa\x04k`\x04\x806\x03\x81\x01\x90a\x04f\x91\x90a[?V[a\x19\xD4V[\0[4\x80\x15a\x04xW_\x80\xFD[Pa\x04\x93`\x04\x806\x03\x81\x01\x90a\x04\x8E\x91\x90aT\xD1V[a\x1D\x90V[`@Qa\x04\xA0\x91\x90a]\xA7V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x04\xB4W_\x80\xFD[Pa\x04\xCF`\x04\x806\x03\x81\x01\x90a\x04\xCA\x91\x90a]\xC7V[a\x1F\xFDV[\0[4\x80\x15a\x04\xDCW_\x80\xFD[Pa\x04\xE5a$*V[`@Qa\x04\xF8\x97\x96\x95\x94\x93\x92\x91\x90a_IV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x0CW_\x80\xFD[Pa\x05'`\x04\x806\x03\x81\x01\x90a\x05\"\x91\x90aT\xD1V[a%3V[`@Qa\x055\x92\x91\x90aaQV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05IW_\x80\xFD[Pa\x05d`\x04\x806\x03\x81\x01\x90a\x05_\x91\x90aa\x86V[a(\x99V[\0[4\x80\x15a\x05qW_\x80\xFD[Pa\x05za*]V[`@Qa\x05\x87\x91\x90aTvV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\x9BW_\x80\xFD[Pa\x05\xB6`\x04\x806\x03\x81\x01\x90a\x05\xB1\x91\x90ab\x19V[a*\x96V[\0[4\x80\x15a\x05\xC3W_\x80\xFD[Pa\x05\xCCa,\xECV[`@Qa\x05\xD9\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x05\xEDW_\x80\xFD[Pa\x06\x08`\x04\x806\x03\x81\x01\x90a\x06\x03\x91\x90aT\xD1V[a-\x03V[\0[4\x80\x15a\x06\x15W_\x80\xFD[Pa\x06\x1Ea/|V[\0[4\x80\x15a\x06+W_\x80\xFD[Pa\x06F`\x04\x806\x03\x81\x01\x90a\x06A\x91\x90aT\xD1V[a1\xC5V[`@Qa\x06T\x92\x91\x90ac\x04V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06hW_\x80\xFD[Pa\x06\x83`\x04\x806\x03\x81\x01\x90a\x06~\x91\x90ac9V[a4\x96V[\0[4\x80\x15a\x06\x90W_\x80\xFD[Pa\x06\x99a5\x94V[`@Qa\x06\xA6\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xBAW_\x80\xFD[Pa\x06\xC3a5\xABV[`@Qa\x06\xD0\x91\x90acdV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x06\xE4W_\x80\xFD[Pa\x06\xEDa6\x0FV[`@Qa\x06\xFA\x91\x90acdV[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x07\x0EW_\x80\xFD[Pa\x07)`\x04\x806\x03\x81\x01\x90a\x07$\x91\x90aT\xD1V[a6sV[`@Qa\x076\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xF3[_\x80a\x07Ia6\xA5V[\x90P\x80`\x05\x01T\x91PP\x90V[```@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x07\x97_a6\xCCV[a\x07\xA1`\x02a6\xCCV[a\x07\xAA_a6\xCCV[`@Q` \x01a\x07\xBD\x94\x93\x92\x91\x90adRV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[``_a\x07\xDCa6\xA5V[\x90P_\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x02\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x82\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08\x94W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08KW[PPPPP\x92PPP\x91\x90PV[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xFFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t#\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\t\x92W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x89\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a\t\x9Ba6\xA5V[\x90P\x80`\t\x01T\x82\x11\x80a\t\xC6WP`\xF8`\x05`\x08\x81\x11\x15a\t\xC0Wa\t\xBFaV\x03V[[\x90\x1B\x82\x11\x15[\x15a\n\x08W\x81`@Q\x7F\xCB\xE9&V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\xFF\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\njW\x81`@Q\x7F\xDF\r\xB5\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\na\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F8O\x90\xFE\xFB\xCF\xAAh\xF2.\0\tJ\xEA\xA5++\xC6\x93\x93m,\xE1\xAF\xED\x12\x12R\x0BY\xB5\x8E\x82`@Qa\n\xC4\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\n\xDAa6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x0B=W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B4\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x0B\x96W\x82`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0B\x8D\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\r\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x92PPP\x91\x90PV[`\x01a\x0B\xE0a7\x96V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0C!W`@Q\x7FoOs\x1F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03_a\x0C,a7\xBAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a\x0CtWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x0C\xABW`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa\rd`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FKMSGeneration\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa7\xE1V[_a\rma6\xA5V[\x90P`\xF8`\x03`\x08\x81\x11\x15a\r\x85Wa\r\x84aV\x03V[[\x90\x1B\x81`\x04\x01\x81\x90UP`\xF8`\x04`\x08\x81\x11\x15a\r\xA5Wa\r\xA4aV\x03V[[\x90\x1B\x81`\x05\x01\x81\x90UP`\xF8`\x05`\x08\x81\x11\x15a\r\xC5Wa\r\xC4aV\x03V[[\x90\x1B\x81`\t\x01\x81\x90UPP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa\x0E\x1A\x91\x90ae*V[`@Q\x80\x91\x03\x90\xA1PPV[_\x80a\x0E0a6\xA5V[\x90P\x80`\t\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x9AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xBE\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x0F-W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F$\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a\x0F6a6\xA5V[\x90P_\x81`\t\x01T\x90P`\xF8`\x05`\x08\x81\x11\x15a\x0FVWa\x0FUaV\x03V[[\x90\x1B\x81\x14\x15\x80\x15a\x0F\x84WP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15a\x0F\xC6W\x80`@Q\x7F\x06\x1A\xC6\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xBD\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x81`\t\x01_\x81T\x80\x92\x91\x90a\x0F\xDA\x90aepV[\x91\x90PUP_\x82`\t\x01T\x90P\x84\x83`\n\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x83`\r\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a\x104Wa\x103aV\x03V[[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\x97W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xBB\x91\x90ae\xCBV[\x91P\x91P_a\x10\xCA\x83\x83a7\xF7V[\x90P\x80\x86`\x0E\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x90\x81a\x10\xEC\x91\x90ah\x03V[P\x7F\x8C\xF0\x15\x13\x93\xF8O\xD6\x94\xC5\xE3\x15\xCBt\xCC\x05\xB2G\xDE\nEO\xD9\xE9\x12\x9Cf\x1E\xFD\xF9@\x1D\x84\x89\x89\x84`@Qa\x11\"\x94\x93\x92\x91\x90ah\xD2V[`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[_\x80a\x11>a6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_\x80a\x11ra6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x11\xD5W\x82`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xCC\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x12.W\x82`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12%\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x80`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x91PP\x91\x90PV[_a\x12_a6\xA5V[\x90P\x80`\x05\x01T\x86\x11\x80a\x12\x8AWP`\xF8`\x04`\x08\x81\x11\x15a\x12\x84Wa\x12\x83aV\x03V[[\x90\x1B\x86\x11\x15[\x15a\x12\xCCW\x85`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x12\xC3\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a\x13\x13W\x85`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x13\n\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x13\x1E\x88a8&V[\x91P\x91P_\x83`\x06\x01_\x8A\x81R` \x01\x90\x81R` \x01_ T\x90P\x83`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x13\x8FW`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x9D\x82\x8B\x8B\x8B\x88a9\xB0V[\x90P_a\x13\xAC\x84\x83\x8A\x8Aa;\x91V[\x90P\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x14LW\x8A\x81`@Q\x7F\x98\xFB\x95}\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14C\x92\x91\x90ai\x1CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x86_\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x86`\x02\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F*\xFEd\xFB:\xFD\xE8\xE2g\x8A\xEA\x84\xCF6\"?3\x0E/\xB1(m7\xAE\xD5s\xAB\x9C\xD1\xDBG\xC7\x8C\x8C\x8C\x8C\x8C3`@Qa\x15n\x96\x95\x94\x93\x92\x91\x90akOV[`@Q\x80\x91\x03\x90\xA1\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x15\xADWPa\x15\xAC\x85\x82\x80T\x90Pa;\xF9V[[\x15a\x17\x98W`\x01\x87`\x01\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_[\x8B\x8B\x90P\x81\x10\x15a\x16cW\x87`\x07\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x8C\x8C\x83\x81\x81\x10a\x16\x10Wa\x16\x0Fak\xA4V[[\x90P` \x02\x81\x01\x90a\x16\"\x91\x90ak\xDDV[\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x90`\x02\x02\x01_\x90\x91\x90\x91\x90\x91P\x81\x81a\x16T\x91\x90an\x10V[PP\x80\x80`\x01\x01\x91PPa\x15\xDFV[P\x82\x87`\x03\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x86`\x0F\x01\x8C\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU_\x87`\x11\x01_\x8E\x81R` \x01\x90\x81R` \x01_ _\x01T\x03a\x17\x97W\x8B\x87`\x08\x01\x81\x90UP_a\x17V\x86\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x17LW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x17\x03W[PPPPPa<\x96V[\x90P\x7F\xEB\x85\xC2m\xBC\xADF\xB8\nh\xA0\xF2L\xCE|,\x90\xF0\xA1\xFA\xDE\xD8A\x84\x13\x889\xFC\x9E\x80\xA2[\x8D\x82\x8E\x8E`@Qa\x17\x8D\x94\x93\x92\x91\x90an\x1EV[`@Q\x80\x91\x03\x90\xA1P[[PPPPPPPPPPPPV[a\x17\xAEa=\xDEV[a\x17\xB7\x82a>\xC4V[a\x17\xC1\x82\x82a?\xB7V[PPV[_a\x17\xCEa@\xD5V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x90P\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18SW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18w\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x18\xE6W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x18\xDD\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a\x18\xEFa6\xA5V[\x90P\x83\x83\x90P\x86\x86\x90P\x14a\x190W`@Q\x7F\x89K*\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`\x12\x01_\x89\x81R` \x01\x90\x81R` \x01_ T\x03a\x19\x87W\x86`@Q\x7F\x05\xB0\x83\xF2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x19~\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x7F\x8B\xFA}\x0E\xD6\xF8~Rkb4)\x18\xEE{\xFAS\x95+\xAD\xD4c\xDC\x93@T\xD7\xDD\x94\x0E\xAF\xDC\x87\x87\x87\x87\x87\x87`\x01`@Qa\x19\xC3\x97\x96\x95\x94\x93\x92\x91\x90an\xCBV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_a\x19\xDDa6\xA5V[\x90P\x80`\x04\x01T\x84\x11\x80a\x1A\x08WP`\xF8`\x03`\x08\x81\x11\x15a\x1A\x02Wa\x1A\x01aV\x03V[[\x90\x1B\x84\x11\x15[\x15a\x1AJW\x83`@Q\x7F\n\xB7\xF6\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1AA\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80a\x1AU\x86a8&V[\x91P\x91P_a\x1Ad\x87\x84aA\\V[\x90P_a\x1As\x83\x83\x89\x89a;\x91V[\x90P\x84_\x01_\x89\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\x1B\x13W\x87\x81`@Q\x7F3\xCA\x1F\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1B\n\x92\x91\x90ai\x1CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7FLq\\W4\xCE\\\x18\xC9\xC1.\x84\x96\xE5=*e\xF1\xEC8\x1DGiW\xF0\xF5\x96\xB3d\xA5\x9B\x0C\x89\x89\x893`@Qa\x1C1\x94\x93\x92\x91\x90ao.V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8A\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\x1CpWPa\x1Co\x84\x82\x80T\x90Pa;\xF9V[[\x15a\x1D\x85W`\x01\x86`\x01\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x82\x86`\x03\x01_\x8B\x81R` \x01\x90\x81R` \x01_ \x81\x90UP_\x86`\x06\x01_\x8B\x81R` \x01\x90\x81R` \x01_ T\x90P_\x87`\x11\x01_\x83\x81R` \x01\x90\x81R` \x01_ \x90P_\x81_\x01T\x14a\x1DFW\x7F\xE4S\xC2\x9CF\xCC\xC7fL\x03\x98\xE8FM[\xB4!\xE9\x95C-\xAFU\x06\xA3\xFD\xBCj\xA0\x96j\x93\x8B\x83\x83_\x01T\x84`\x01\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x8B`@Qa\x1D9\x95\x94\x93\x92\x91\x90aolV[`@Q\x80\x91\x03\x90\xA1a\x1D\x82V[\x7F:\x11a \xCC\xA5\xD4\xF0s\xCC\x1F\xC3\x1F\xF2a3\xAB{\x04\x99\xF2q/\xA0\x10\x02;\x87\xD5\xA1\xF9\xEE\x8B\x83\x89`@Qa\x1Dy\x93\x92\x91\x90ao\xC4V[`@Q\x80\x91\x03\x90\xA1[PP[PPPPPPPPPV[a\x1D\x98aS\x84V[_a\x1D\xA1a6\xA5V[\x90P\x80`\x01\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a\x1E\x04W\x82`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xFB\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80\x1B\x81`\x03\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x03a\x1E]W\x82`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1ET\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ T\x90P`@Q\x80`\x80\x01`@R\x80\x82\x81R` \x01\x85\x81R` \x01\x83`\r\x01_\x84\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1E\xBDWa\x1E\xBCaV\x03V[[\x81R` \x01\x83`\x07\x01_\x87\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x1F\xEDW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a\x1F8Wa\x1F7aV\x03V[[`\x01\x81\x11\x15a\x1FJWa\x1FIaV\x03V[[\x81R` \x01`\x01\x82\x01\x80Ta\x1F^\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1F\x8A\x90af6V[\x80\x15a\x1F\xD5W\x80`\x1F\x10a\x1F\xACWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F\xD5V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1E\xF4V[PPPP\x81RP\x92PPP\x91\x90PV[_a \x06a6\xA5V[\x90P\x80`\t\x01T\x86\x11\x80a 1WP`\xF8`\x05`\x08\x81\x11\x15a +Wa *aV\x03V[[\x90\x1B\x86\x11\x15[\x15a sW\x85`@Q\x7F\x8D\x8C\x94\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a j\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x80a ~\x88a8&V[\x91P\x91P_a \xA3\x89\x85`\n\x01_\x8C\x81R` \x01\x90\x81R` \x01_ T\x8A\x8A\x87aA\xBEV[\x90P_a \xB2\x83\x83\x89\x89a;\x91V[\x90P\x84_\x01_\x8B\x81R` \x01\x90\x81R` \x01_ _\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a!RW\x89\x81`@Q\x7F\xFC\xF5\xA6\xE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!I\x92\x91\x90ai\x1CV[`@Q\x80\x91\x03\x90\xFD[`\x01\x85_\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x85`\x02\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x84\x81R` \x01\x90\x81R` \x01_ \x90P\x803\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F{\xF1\xB4,\x10\xE9I|\x87\x96 \xC5\xB7\xAF\xCE\xD1\x0B\xDA\x17\xD8\xC9\x0B\"\xF0\xE3\xBCk/\xD6\xCE\xD0\xBD\x8B\x8B\x8B\x8B\x8B3`@Qa\"t\x96\x95\x94\x93\x92\x91\x90ap\0V[`@Q\x80\x91\x03\x90\xA1\x85`\x01\x01_\x8C\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15\x80\x15a\"\xB3WPa\"\xB2\x84\x82\x80T\x90Pa;\xF9V[[\x15a$\x1DW`\x01\x86`\x01\x01_\x8D\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x89\x89\x87`\x0B\x01_\x8E\x81R` \x01\x90\x81R` \x01_ \x91\x82a#\x05\x92\x91\x90al\xEFV[P\x82\x86`\x03\x01_\x8D\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x8A\x86`\x0C\x01\x81\x90UP\x85`\x10\x01\x8B\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU_a#\xDC\x85\x83\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a#\xD2W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a#\x89W[PPPPPa<\x96V[\x90P\x7F\"X\xB7?\xAE\xD3?\xB2\xE2\xEAED\x03\xBE\xF9t\x92\x0C\xAFh*\xB3\xA7#HO\xCFgU;\x16\xA2\x8C\x82\x8D\x8D`@Qa$\x13\x94\x93\x92\x91\x90apUV[`@Q\x80\x91\x03\x90\xA1P[PPPPPPPPPPPV[_``\x80_\x80_``_a$<aBOV[\x90P_\x80\x1B\x81_\x01T\x14\x80\x15a$WWP_\x80\x1B\x81`\x01\x01T\x14[a$\x96W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\x8D\x90ap\xE4V[`@Q\x80\x91\x03\x90\xFD[a$\x9EaBvV[a$\xA6aC\x14V[F0_\x80\x1B_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a$\xC5Wa$\xC4aX\x98V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$\xF3W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94\x93\x92\x91\x90\x97P\x97P\x97P\x97P\x97P\x97P\x97PP\x90\x91\x92\x93\x94\x95\x96V[``\x80_a%?a6\xA5V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a%\xA2W\x83`@Q\x7F\x84\xDE\x131\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\x99\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a%\xFFW\x84`@Q\x7F\x83\xF1\x835\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a%\xF6\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a&\x9FW` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a&VW[PPPPP\x90P_a'I\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta&\xC8\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xF4\x90af6V[\x80\x15a'?W\x80`\x1F\x10a'\x16Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a'?V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a'\"W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaC\xB2V[\x90P_a'V\x82\x84a<\x96V[\x90P\x80\x85`\x07\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a(\x85W\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a'\xD0Wa'\xCFaV\x03V[[`\x01\x81\x11\x15a'\xE2Wa'\xE1aV\x03V[[\x81R` \x01`\x01\x82\x01\x80Ta'\xF6\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta(\"\x90af6V[\x80\x15a(mW\x80`\x1F\x10a(DWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a(mV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a(PW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a'\x8CV[PPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(\xF6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x1A\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)\x89W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\x80\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a)\x92a6\xA5V[\x90P\x80`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a)\xF5W\x81`@Q\x7F\xAD\xFA\xB9\x04\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a)\xEC\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_a)\xFF\x84aE\xA1V[\x91PP`@Q\x80`@\x01`@R\x80\x84\x81R` \x01`\x01\x15\x15\x81RP\x82`\x11\x01_\x83\x81R` \x01\x90\x81R` \x01_ _\x82\x01Q\x81_\x01U` \x82\x01Q\x81`\x01\x01_a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x90PPPPPPV[`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01\x7F5.0.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x81V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a*\xF3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x17\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a+\x86W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+}\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a+\x8Fa6\xA5V[\x90P\x86\x81`\x11\x01_\x88\x81R` \x01\x90\x81R` \x01_ _\x01T\x14a+\xECW\x85\x87`@Q\x7F\x941\xF3N\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a+\xE3\x92\x91\x90aq\x02V[`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a,BW`@Q\x7Fo\xBC\xDD+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x85\x85\x90P\x03a,\x89W\x86`@Q\x7F\xE6\xF9\x08;\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a,\x80\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x85\x81`\x12\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x7F\xA4vd\x86\x1A\xB5\x8C[\xD5\x04\x0E\x9C\xC4^h\xD0\xE4\x8E\xC0Cq\x03_\xD7P\x99\xE2\x17\xE0\xA6\xAA\x81\x87\x84\x84\x88\x88`\x01`@Qa,\xDB\x96\x95\x94\x93\x92\x91\x90arUV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[_\x80a,\xF6a6\xA5V[\x90P\x80`\x0C\x01T\x91PP\x90V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x84\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a-\xF3W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xEA\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[_a-\xFCa6\xA5V[\x90P\x80`\x04\x01T\x82\x11\x80a.'WP`\xF8`\x03`\x08\x81\x11\x15a.!Wa. aV\x03V[[\x90\x1B\x82\x11\x15[\x15a.iW\x81`@Q\x7F\xFC\xF2\xDBz\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.`\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x06\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x90P\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a.\xE2W\x82`@Q\x7F\x92x\x9Bg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xD9\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_\x81\x14a/@W`\x01\x82`\x01\x01_\x83\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP[\x7F+\x08{\x88K5\xA8\x1Dv\x9D\x1A\x1E\t(\x80\xF1\xDAV\xDE\x96NK3\x9E\xAB\xCB\x1FE\xF5\xFE2d\x83`@Qa/o\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xA1PPPV[`\x03_a/\x87a7\xBAV[\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x80a/\xCFWP\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a0\x06W`@Q\x7F\xF9.\xE8\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81_\x01_a\x01\0\n\x81T\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP_a0Ta6\xA5V[\x90P_`\x01`\xF8`\x04`\x08\x81\x11\x15a0oWa0naV\x03V[[\x90\x1Ba0{\x91\x90ar\xAAV[\x90P[\x81`\x05\x01T\x81\x11a0\xE1W_\x80\x1B\x82`\x03\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x14a0\xCEW\x81`\x0F\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU[\x80\x80a0\xD9\x90aepV[\x91PPa0~V[P_`\x01`\xF8`\x05`\x08\x81\x11\x15a0\xFBWa0\xFAaV\x03V[[\x90\x1Ba1\x07\x91\x90ar\xAAV[\x90P[\x81`\t\x01T\x81\x11a1mW_\x80\x1B\x82`\x03\x01_\x83\x81R` \x01\x90\x81R` \x01_ T\x14a1ZW\x81`\x10\x01\x81\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91PU[\x80\x80a1e\x90aepV[\x91PPa1\nV[PP_\x81_\x01`\x08a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x82`@Qa1\xB9\x91\x90ae*V[`@Q\x80\x91\x03\x90\xA1PPV[``\x80_a1\xD1a6\xA5V[\x90P\x80`\x01\x01_\x85\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16a24W\x83`@Q\x7F\xDA2\xD0\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2+\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x81`\x03\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P_\x80\x1B\x81\x03a2\x91W\x84`@Q\x7F\xD5\xFD<\xD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a2\x88\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[_\x82`\x02\x01_\x87\x81R` \x01\x90\x81R` \x01_ _\x83\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a31W` \x02\x82\x01\x91\x90_R` _ \x90[\x81_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a2\xE8W[PPPPP\x90P_a3\xDB\x84`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x80Ta3Z\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta3\x86\x90af6V[\x80\x15a3\xD1W\x80`\x1F\x10a3\xA8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a3\xD1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a3\xB4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPaC\xB2V[\x90P_a3\xE8\x82\x84a<\x96V[\x90P\x80\x85`\x0B\x01_\x8A\x81R` \x01\x90\x81R` \x01_ \x80\x80Ta4\n\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta46\x90af6V[\x80\x15a4\x81W\x80`\x1F\x10a4XWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a4\x81V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a4dW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x96P\x96PPPPPP\x91P\x91V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xF3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\x17\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a5\x86W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5}\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[a5\x8F\x81aE\xA1V[PPPV[_\x80a5\x9Ea6\xA5V[\x90P\x80`\x08\x01T\x91PP\x90V[``_a5\xB6a6\xA5V[\x90P\x80`\x10\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a6\x04W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a5\xF0W[PPPPP\x91PP\x90V[``_a6\x1Aa6\xA5V[\x90P\x80`\x0F\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a6hW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a6TW[PPPPP\x91PP\x90V[_\x80a6}a6\xA5V[`\x12\x01_\x84\x81R` \x01\x90\x81R` \x01_ T\x03a6\x9BW_a6\x9EV[`\x01[\x90P\x91\x90PV[_\x7F&\xFD\xAF\x8A,\xB2\r \xB5^6!\x89\x86\x90^SN\xE7\xA9p\xDD/\xA8'\x94nKt\x96\xDB\0\x90P\x90V[``_`\x01a6\xDA\x84aH\0V[\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xF8Wa6\xF7aX\x98V[[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a7*W\x81` \x01`\x01\x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x82` \x01\x82\x01\x90P[`\x01\x15a7\x8BW\x80\x80`\x01\x90\x03\x91PP\x7F0123456789abcdef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\n\x86\x06\x1A\x81S`\n\x85\x81a7\x80Wa7\x7Far\xDDV[[\x04\x94P_\x85\x03a77W[\x81\x93PPPP\x91\x90PV[_a7\x9Fa7\xBAV[_\x01_\x90T\x90a\x01\0\n\x90\x04g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[_\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x90P\x90V[a7\xE9aIQV[a7\xF3\x82\x82aI\x91V[PPV[```\x02\x83\x83`@Q` \x01a8\x0F\x93\x92\x91\x90asjV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[``_\x80a82a6\xA5V[\x90P\x80`\x0E\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x80Ta8R\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta8~\x90af6V[\x80\x15a8\xC9W\x80`\x1F\x10a8\xA0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a8\xC9V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a8\xACW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x92Pa8\xD9\x83aC\xB2V[\x91PsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cF\xC5\xBB\xBD\x833`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9*\x92\x91\x90ai\x1CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9i\x91\x90as\xD0V[a9\xAAW3`@Q\x7F\xAE\xE8c#\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a9\xA1\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[P\x91P\x91V[_\x80\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xCFWa9\xCEaX\x98V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a9\xFDW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_[\x85\x85\x90P\x81\x10\x15a;\x01W`@Q\x80``\x01`@R\x80`%\x81R` \x01a{\xB8`%\x919\x80Q\x90` \x01 \x86\x86\x83\x81\x81\x10a:@Wa:?ak\xA4V[[\x90P` \x02\x81\x01\x90a:R\x91\x90ak\xDDV[_\x01` \x81\x01\x90a:c\x91\x90as\xFBV[\x87\x87\x84\x81\x81\x10a:vWa:uak\xA4V[[\x90P` \x02\x81\x01\x90a:\x88\x91\x90ak\xDDV[\x80` \x01\x90a:\x97\x91\x90al\x83V[`@Qa:\xA5\x92\x91\x90atTV[`@Q\x80\x91\x03\x90 `@Q` \x01a:\xBF\x93\x92\x91\x90at{V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82\x82\x81Q\x81\x10a:\xE8Wa:\xE7ak\xA4V[[` \x02` \x01\x01\x81\x81RPP\x80\x80`\x01\x01\x91PPa:\x02V[Pa;\x85`@Q\x80`\xC0\x01`@R\x80`\x82\x81R` \x01a{6`\x82\x919\x80Q\x90` \x01 \x88\x88\x84`@Q` \x01a;8\x91\x90auaV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x87\x80Q\x90` \x01 `@Q` \x01a;j\x95\x94\x93\x92\x91\x90auwV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\xE2V[\x91PP\x95\x94PPPPPV[_\x80a;\xE0\x85\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPPaI\xFBV[\x90Pa;\xED\x86\x823aJ%V[\x80\x91PP\x94\x93PPPPV[_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cA\xAD\x06\x9C\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a<H\x91\x90aS\xD3V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a<cW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a<\x87\x91\x90au\xC8V[\x90P\x80\x83\x10\x15\x91PP\x92\x91PPV[``_\x82Q\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xB8Wa<\xB7aX\x98V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\xEBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a<\xD6W\x90P[P\x90P_[\x82\x81\x10\x15a=\xD2WsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x87\x87\x84\x81Q\x81\x10a=<Wa=;ak\xA4V[[` \x02` \x01\x01Q`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a=a\x92\x91\x90ai\x1CV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a={W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xA3\x91\x90awFV[``\x01Q\x82\x82\x81Q\x81\x10a=\xBAWa=\xB9ak\xA4V[[` \x02` \x01\x01\x81\x90RP\x80\x80`\x01\x01\x91PPa<\xF0V[P\x80\x92PPP\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x80a>\x8BWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a>raL\x06V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a>\xC2W`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[sP\x15|\xFF\xD6\xBB\xFA-\xEC\xE2\x04\xA8\x9E\xC4\x19\xC2>\xF5u]s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a?!W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?E\x91\x90ad\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a?\xB4W3`@Q\x7F!\xBF\xDA\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a?\xAB\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[PV[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a@\x1FWP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@\x1C\x91\x90aw\xB7V[`\x01[a@`W\x81`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@W\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1B\x81\x14a@\xC6W\x80`@Q\x7F\xAA\x1DI\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a@\xBD\x91\x90aZ.V[`@Q\x80\x91\x03\x90\xFD[a@\xD0\x83\x83aLYV[PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x160s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aAZW`@Q\x7F\xE0|\x8D\xBA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_aA\xB6`@Q\x80``\x01`@R\x80`<\x81R` \x01az\xA4`<\x919\x80Q\x90` \x01 \x84\x84\x80Q\x90` \x01 `@Q` \x01aA\x9B\x93\x92\x91\x90aw\xE2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\xE2V[\x90P\x92\x91PPV[_aBD`@Q\x80`\x80\x01`@R\x80`V\x81R` \x01az\xE0`V\x919\x80Q\x90` \x01 \x87\x87\x87\x87`@Q` \x01aA\xF7\x92\x91\x90atTV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x86\x80Q\x90` \x01 `@Q` \x01aB)\x95\x94\x93\x92\x91\x90auwV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 aI\xE2V[\x90P\x95\x94PPPPPV[_\x7F\xA1jF\xD9Ba\xC7Q|\xC8\xFF\x89\xF6\x1C\x0C\xE95\x98\xE3\xC8I\x80\x10\x11\xDE\xE6I\xA6\xA5W\xD1\0\x90P\x90V[``_aB\x81aBOV[\x90P\x80`\x02\x01\x80TaB\x92\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaB\xBE\x90af6V[\x80\x15aC\tW\x80`\x1F\x10aB\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aC\tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aB\xECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[``_aC\x1FaBOV[\x90P\x80`\x03\x01\x80TaC0\x90af6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80TaC\\\x90af6V[\x80\x15aC\xA7W\x80`\x1F\x10aC~Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91aC\xA7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11aC\x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x91PP\x90V[_\x80\x82Q\x14\x80aC\xE4WP_\x82_\x81Q\x81\x10aC\xD1WaC\xD0ak\xA4V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C`\xFF\x16\x14[\x15aDqWsD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x97o>\xB9`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aDFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aDj\x91\x90au\xC8V[\x90PaE\x9CV[_\x82_\x81Q\x81\x10aD\x85WaD\x84ak\xA4V[[` \x01\x01Q`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90P`\x01`\xFF\x16\x81`\xFF\x16\x14\x15\x80\x15aD\xB3WP`\x02`\xFF\x16\x81`\xFF\x16\x14\x15[\x15aD\xF5W\x80`@Q\x7F!9\xCC,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aD\xEC\x91\x90ax&V[`@Q\x80\x91\x03\x90\xFD[`\x01`\xFF\x16\x81`\xFF\x16\x14\x80\x15aE\rWP`!\x83Q\x14\x15[\x15aEDW`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\xFF\x16\x81`\xFF\x16\x14\x80\x15aE\\WP`A\x83Q\x14\x15[\x15aE\x93W`@Q\x7F\x8B$\x8B`\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`!\x83\x01Q\x91PP[\x91\x90PV[_\x80_aE\xACa6\xA5V[\x90P_\x81`\x05\x01T\x90P`\xF8`\x04`\x08\x81\x11\x15aE\xCCWaE\xCBaV\x03V[[\x90\x1B\x81\x14\x15\x80\x15aE\xFAWP\x81`\x01\x01_\x82\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15[\x15aF<W\x80`@Q\x7F;\x85=\xA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aF3\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[\x81`\x04\x01_\x81T\x80\x92\x91\x90aFP\x90aepV[\x91\x90PUP\x81`\x04\x01T\x93P\x81`\x05\x01_\x81T\x80\x92\x91\x90aFp\x90aepV[\x91\x90PUP\x81`\x05\x01T\x92P\x82\x82`\x06\x01_\x86\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x83\x82`\x06\x01_\x85\x81R` \x01\x90\x81R` \x01_ \x81\x90UP\x84\x82`\r\x01_\x86\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15aF\xE1WaF\xE0aV\x03V[[\x02\x17\x90UP_\x80sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ce\xB3\x94\xAF`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`@\x80Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aGDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aGh\x91\x90ae\xCBV[\x91P\x91P_aGw\x83\x83a7\xF7V[\x90P\x80\x85`\x0E\x01_\x89\x81R` \x01\x90\x81R` \x01_ \x90\x81aG\x99\x91\x90ah\x03V[P\x80\x85`\x0E\x01_\x88\x81R` \x01\x90\x81R` \x01_ \x90\x81aG\xBA\x91\x90ah\x03V[P\x7F\xFB\xF5'H\x10\xB9O\x86\x97\x0C\x11G\xE8\xFF\xAE\xBE\xD2F\xEE\x97w\xD6\x95\xA6\x90\x04\xDCbV\xD1\xFE\x91\x87\x89\x83`@QaG\xEE\x93\x92\x91\x90ax?V[`@Q\x80\x91\x03\x90\xA1PPPPP\x91P\x91V[_\x80_\x90Pz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x10aH\\Wz\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x83\x81aHRWaHQar\xDDV[[\x04\x92P`@\x81\x01\x90P[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10aH\x99Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x81aH\x8FWaH\x8Ear\xDDV[[\x04\x92P` \x81\x01\x90P[f#\x86\xF2o\xC1\0\0\x83\x10aH\xC8Wf#\x86\xF2o\xC1\0\0\x83\x81aH\xBEWaH\xBDar\xDDV[[\x04\x92P`\x10\x81\x01\x90P[c\x05\xF5\xE1\0\x83\x10aH\xF1Wc\x05\xF5\xE1\0\x83\x81aH\xE7WaH\xE6ar\xDDV[[\x04\x92P`\x08\x81\x01\x90P[a'\x10\x83\x10aI\x16Wa'\x10\x83\x81aI\x0CWaI\x0Bar\xDDV[[\x04\x92P`\x04\x81\x01\x90P[`d\x83\x10aI9W`d\x83\x81aI/WaI.ar\xDDV[[\x04\x92P`\x02\x81\x01\x90P[`\n\x83\x10aIHW`\x01\x81\x01\x90P[\x80\x91PP\x91\x90PV[aIYaL\xCBV[aI\x8FW`@Q\x7F\xD7\xE6\xBC\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[aI\x99aIQV[_aI\xA2aBOV[\x90P\x82\x81`\x02\x01\x90\x81aI\xB5\x91\x90ax\xD3V[P\x81\x81`\x03\x01\x90\x81aI\xC7\x91\x90ax\xD3V[P_\x80\x1B\x81_\x01\x81\x90UP_\x80\x1B\x81`\x01\x01\x81\x90UPPPPV[_aI\xF4aI\xEEaL\xE9V[\x83aL\xF7V[\x90P\x91\x90PV[_\x80_\x80aJ\t\x86\x86aM7V[\x92P\x92P\x92PaJ\x19\x82\x82aM\x8CV[\x82\x93PPPP\x92\x91PPV[sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x94G\xCF\xD4\x84\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aJt\x92\x91\x90ai\x1CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aJ\x8FW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aJ\xB3\x91\x90as\xD0V[aJ\xF6W\x81\x81`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aJ\xED\x92\x91\x90ay\xA2V[`@Q\x80\x91\x03\x90\xFD[_sD\xAA\x02\x8F\xD2d\xC7k\xF4\xA8\xF8\xB4\xD8\xA5'/j\xE2\\\xACs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c1\xFFA\xC8\x85\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01aKF\x92\x91\x90ai\x1CV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aK`W=_\x80>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aK\x88\x91\x90awFV[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14aL\0W\x82\x82`@Q\x7F\r\x86\xF5!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aK\xF7\x92\x91\x90ay\xA2V[`@Q\x80\x91\x03\x90\xFD[PPPPV[_aL2\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\xEEV[_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x90V[aLb\x82aN\xF7V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;`@Q`@Q\x80\x91\x03\x90\xA2_\x81Q\x11\x15aL\xBEWaL\xB8\x82\x82aO\xC0V[PaL\xC7V[aL\xC6aP@V[[PPV[_aL\xD4a7\xBAV[_\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x90P\x90V[_aL\xF2aP|V[\x90P\x90V[_`@Q\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x02\x82\x01R\x82`\"\x82\x01R`B\x81 \x91PP\x92\x91PPV[_\x80_`A\x84Q\x03aMwW_\x80_` \x87\x01Q\x92P`@\x87\x01Q\x91P``\x87\x01Q_\x1A\x90PaMi\x88\x82\x85\x85aP\xDFV[\x95P\x95P\x95PPPPaM\x85V[_`\x02\x85Q_\x1B\x92P\x92P\x92P[\x92P\x92P\x92V[_`\x03\x81\x11\x15aM\x9FWaM\x9EaV\x03V[[\x82`\x03\x81\x11\x15aM\xB2WaM\xB1aV\x03V[[\x03\x15aN\xEAW`\x01`\x03\x81\x11\x15aM\xCCWaM\xCBaV\x03V[[\x82`\x03\x81\x11\x15aM\xDFWaM\xDEaV\x03V[[\x03aN\x16W`@Q\x7F\xF6E\xEE\xDF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x03\x81\x11\x15aN*WaN)aV\x03V[[\x82`\x03\x81\x11\x15aN=WaN<aV\x03V[[\x03aN\x81W\x80_\x1C`@Q\x7F\xFC\xE6\x98\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aNx\x91\x90aS\xD3V[`@Q\x80\x91\x03\x90\xFD[`\x03\x80\x81\x11\x15aN\x94WaN\x93aV\x03V[[\x82`\x03\x81\x11\x15aN\xA7WaN\xA6aV\x03V[[\x03aN\xE9W\x80`@Q\x7F\xD7\x8B\xCE\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aN\xE0\x91\x90aZ.V[`@Q\x80\x91\x03\x90\xFD[[PPV[_\x81\x90P\x91\x90PV[_\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x03aORW\x80`@Q\x7FL\x9C\x8C\xE3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aOI\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[\x80aO~\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC_\x1BaN\xEEV[_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPV[``_\x80\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84`@QaO\xE9\x91\x90ay\xF9V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14aP!W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>aP&V[``\x91P[P\x91P\x91PaP6\x85\x83\x83aQ\xC6V[\x92PPP\x92\x91PPV[_4\x11\x15aPzW`@Q\x7F\xB3\x98\x97\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0FaP\xA6aRSV[aP\xAEaR\xC9V[F0`@Q` \x01aP\xC4\x95\x94\x93\x92\x91\x90az\x0FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x80_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84_\x1C\x11\x15aQ\x1BW_`\x03\x85\x92P\x92P\x92PaQ\xBCV[_`\x01\x88\x88\x88\x88`@Q_\x81R` \x01`@R`@QaQ>\x94\x93\x92\x91\x90az`V[` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15aQ^W=_\x80>=_\xFD[PPP` `@Q\x03Q\x90P_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aQ\xAFW_`\x01_\x80\x1B\x93P\x93P\x93PPaQ\xBCV[\x80_\x80_\x1B\x93P\x93P\x93PP[\x94P\x94P\x94\x91PPV[``\x82aQ\xDBWaQ\xD6\x82aS@V[aRKV[_\x82Q\x14\x80\x15aR\x01WP_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16;\x14[\x15aRCW\x83`@Q\x7F\x99\x96\xB3\x15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01aR:\x91\x90ad\xEFV[`@Q\x80\x91\x03\x90\xFD[\x81\x90PaRLV[[\x93\x92PPPV[_\x80aR]aBOV[\x90P_aRhaBvV[\x90P_\x81Q\x11\x15aR\x84W\x80\x80Q\x90` \x01 \x92PPPaR\xC6V[_\x82_\x01T\x90P_\x80\x1B\x81\x14aR\x9FW\x80\x93PPPPaR\xC6V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x80aR\xD3aBOV[\x90P_aR\xDEaC\x14V[\x90P_\x81Q\x11\x15aR\xFAW\x80\x80Q\x90` \x01 \x92PPPaS=V[_\x82`\x01\x01T\x90P_\x80\x1B\x81\x14aS\x16W\x80\x93PPPPaS=V[\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x93PPPP[\x90V[_\x81Q\x11\x15aSRW\x80Q\x80\x82` \x01\xFD[`@Q\x7F\xD6\xBD\xA2u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_`\x01\x81\x11\x15aS\xAEWaS\xADaV\x03V[[\x81R` \x01``\x81RP\x90V[_\x81\x90P\x91\x90PV[aS\xCD\x81aS\xBBV[\x82RPPV[_` \x82\x01\x90PaS\xE6_\x83\x01\x84aS\xC4V[\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_[\x83\x81\x10\x15aT#W\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90PaT\x08V[_\x84\x84\x01RPPPPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[_aTH\x82aS\xECV[aTR\x81\x85aS\xF6V[\x93PaTb\x81\x85` \x86\x01aT\x06V[aTk\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT\x8E\x81\x84aT>V[\x90P\x92\x91PPV[_`@Q\x90P\x90V[_\x80\xFD[_\x80\xFD[aT\xB0\x81aS\xBBV[\x81\x14aT\xBAW_\x80\xFD[PV[_\x815\x90PaT\xCB\x81aT\xA7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aT\xE6WaT\xE5aT\x9FV[[_aT\xF3\x84\x82\x85\x01aT\xBDV[\x91PP\x92\x91PPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aUN\x82aU%V[\x90P\x91\x90PV[aU^\x81aUDV[\x82RPPV[_aUo\x83\x83aUUV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aU\x91\x82aT\xFCV[aU\x9B\x81\x85aU\x06V[\x93PaU\xA6\x83aU\x16V[\x80_[\x83\x81\x10\x15aU\xD6W\x81QaU\xBD\x88\x82aUdV[\x97PaU\xC8\x83aU{V[\x92PP`\x01\x81\x01\x90PaU\xA9V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU\xFB\x81\x84aU\x87V[\x90P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aVAWaV@aV\x03V[[PV[_\x81\x90PaVQ\x82aV0V[\x91\x90PV[_aV`\x82aVDV[\x90P\x91\x90PV[aVp\x81aVVV[\x82RPPV[_` \x82\x01\x90PaV\x89_\x83\x01\x84aVgV[\x92\x91PPV[`\x02\x81\x10aV\x9BW_\x80\xFD[PV[_\x815\x90PaV\xAC\x81aV\x8FV[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aV\xC8WaV\xC7aT\x9FV[[_aV\xD5\x85\x82\x86\x01aT\xBDV[\x92PP` aV\xE6\x85\x82\x86\x01aV\x9EV[\x91PP\x92P\x92\x90PV[_\x81\x15\x15\x90P\x91\x90PV[aW\x04\x81aV\xF0V[\x82RPPV[_` \x82\x01\x90PaW\x1D_\x83\x01\x84aV\xFBV[\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12aWDWaWCaW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aWaWaW`aW'V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aW}WaW|aW+V[[\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aW\x99WaW\x98aW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aW\xB6WaW\xB5aW'V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15aW\xD2WaW\xD1aW+V[[\x92P\x92\x90PV[_\x80_\x80_``\x86\x88\x03\x12\x15aW\xF2WaW\xF1aT\x9FV[[_aW\xFF\x88\x82\x89\x01aT\xBDV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aX WaX\x1FaT\xA3V[[aX,\x88\x82\x89\x01aW/V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aXOWaXNaT\xA3V[[aX[\x88\x82\x89\x01aW\x84V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[aXs\x81aUDV[\x81\x14aX}W_\x80\xFD[PV[_\x815\x90PaX\x8E\x81aXjV[\x92\x91PPV[_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aX\xCE\x82aT.V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aX\xEDWaX\xECaX\x98V[[\x80`@RPPPV[_aX\xFFaT\x96V[\x90PaY\x0B\x82\x82aX\xC5V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aY*WaY)aX\x98V[[aY3\x82aT.V[\x90P` \x81\x01\x90P\x91\x90PV[\x82\x81\x837_\x83\x83\x01RPPPV[_aY`aY[\x84aY\x10V[aX\xF6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15aY|WaY{aX\x94V[[aY\x87\x84\x82\x85aY@V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aY\xA3WaY\xA2aW#V[[\x815aY\xB3\x84\x82` \x86\x01aYNV[\x91PP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15aY\xD2WaY\xD1aT\x9FV[[_aY\xDF\x85\x82\x86\x01aX\x80V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\0WaY\xFFaT\xA3V[[aZ\x0C\x85\x82\x86\x01aY\x8FV[\x91PP\x92P\x92\x90PV[_\x81\x90P\x91\x90PV[aZ(\x81aZ\x16V[\x82RPPV[_` \x82\x01\x90PaZA_\x83\x01\x84aZ\x1FV[\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aZ\\WaZ[aW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZyWaZxaW'V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15aZ\x95WaZ\x94aW+V[[\x92P\x92\x90PV[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15aZ\xB6WaZ\xB5aT\x9FV[[_aZ\xC3\x89\x82\x8A\x01aT\xBDV[\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xE4WaZ\xE3aT\xA3V[[aZ\xF0\x89\x82\x8A\x01aZGV[\x95P\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x13Wa[\x12aT\xA3V[[a[\x1F\x89\x82\x8A\x01aZGV[\x93P\x93PP``a[2\x89\x82\x8A\x01aT\xBDV[\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x80_`@\x84\x86\x03\x12\x15a[VWa[UaT\x9FV[[_a[c\x86\x82\x87\x01aT\xBDV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x84Wa[\x83aT\xA3V[[a[\x90\x86\x82\x87\x01aW\x84V[\x92P\x92PP\x92P\x92P\x92V[a[\xA5\x81aS\xBBV[\x82RPPV[a[\xB4\x81aVVV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[`\x02\x81\x10a[\xF4Wa[\xF3aV\x03V[[PV[_\x81\x90Pa\\\x04\x82a[\xE3V[\x91\x90PV[_a\\\x13\x82a[\xF7V[\x90P\x91\x90PV[a\\#\x81a\\\tV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a\\M\x82a\\)V[a\\W\x81\x85a\\3V[\x93Pa\\g\x81\x85` \x86\x01aT\x06V[a\\p\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_`@\x83\x01_\x83\x01Qa\\\x90_\x86\x01\x82a\\\x1AV[P` \x83\x01Q\x84\x82\x03` \x86\x01Ra\\\xA8\x82\x82a\\CV[\x91PP\x80\x91PP\x92\x91PPV[_a\\\xC0\x83\x83a\\{V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a\\\xDE\x82a[\xBAV[a\\\xE8\x81\x85a[\xC4V[\x93P\x83` \x82\x02\x85\x01a\\\xFA\x85a[\xD4V[\x80_[\x85\x81\x10\x15a]5W\x84\x84\x03\x89R\x81Qa]\x16\x85\x82a\\\xB5V[\x94Pa]!\x83a\\\xC8V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa\\\xFDV[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`\x80\x83\x01_\x83\x01Qa]\\_\x86\x01\x82a[\x9CV[P` \x83\x01Qa]o` \x86\x01\x82a[\x9CV[P`@\x83\x01Qa]\x82`@\x86\x01\x82a[\xABV[P``\x83\x01Q\x84\x82\x03``\x86\x01Ra]\x9A\x82\x82a\\\xD4V[\x91PP\x80\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xBF\x81\x84a]GV[\x90P\x92\x91PPV[_\x80_\x80_``\x86\x88\x03\x12\x15a]\xE0Wa]\xDFaT\x9FV[[_a]\xED\x88\x82\x89\x01aT\xBDV[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x0EWa^\raT\xA3V[[a^\x1A\x88\x82\x89\x01aW\x84V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^=Wa^<aT\xA3V[[a^I\x88\x82\x89\x01aW\x84V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x16\x90P\x91\x90PV[a^\x8C\x81a^XV[\x82RPPV[a^\x9B\x81aUDV[\x82RPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a^\xD5\x83\x83a[\x9CV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^\xF7\x82a^\xA1V[a_\x01\x81\x85a^\xABV[\x93Pa_\x0C\x83a^\xBBV[\x80_[\x83\x81\x10\x15a_<W\x81Qa_#\x88\x82a^\xCAV[\x97Pa_.\x83a^\xE1V[\x92PP`\x01\x81\x01\x90Pa_\x0FV[P\x85\x93PPPP\x92\x91PPV[_`\xE0\x82\x01\x90Pa_\\_\x83\x01\x8Aa^\x83V[\x81\x81\x03` \x83\x01Ra_n\x81\x89aT>V[\x90P\x81\x81\x03`@\x83\x01Ra_\x82\x81\x88aT>V[\x90Pa_\x91``\x83\x01\x87aS\xC4V[a_\x9E`\x80\x83\x01\x86a^\x92V[a_\xAB`\xA0\x83\x01\x85aZ\x1FV[\x81\x81\x03`\xC0\x83\x01Ra_\xBD\x81\x84a^\xEDV[\x90P\x98\x97PPPPPPPPV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a`\x0E\x82aS\xECV[a`\x18\x81\x85a_\xF4V[\x93Pa`(\x81\x85` \x86\x01aT\x06V[a`1\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_a`G\x83\x83a`\x04V[\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a`e\x82a_\xCBV[a`o\x81\x85a_\xD5V[\x93P\x83` \x82\x02\x85\x01a`\x81\x85a_\xE5V[\x80_[\x85\x81\x10\x15a`\xBCW\x84\x84\x03\x89R\x81Qa`\x9D\x85\x82a`<V[\x94Pa`\xA8\x83a`OV[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Pa`\x84V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_a`\xE8\x82a[\xBAV[a`\xF2\x81\x85a`\xCEV[\x93P\x83` \x82\x02\x85\x01aa\x04\x85a[\xD4V[\x80_[\x85\x81\x10\x15aa?W\x84\x84\x03\x89R\x81Qaa \x85\x82a\\\xB5V[\x94Paa+\x83a\\\xC8V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Paa\x07V[P\x82\x97P\x87\x95PPPPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Raai\x81\x85a`[V[\x90P\x81\x81\x03` \x83\x01Raa}\x81\x84a`\xDEV[\x90P\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aa\x9CWaa\x9BaT\x9FV[[_aa\xA9\x85\x82\x86\x01aV\x9EV[\x92PP` aa\xBA\x85\x82\x86\x01aT\xBDV[\x91PP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12aa\xD9Waa\xD8aW#V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aa\xF6Waa\xF5aW'V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15ab\x12Wab\x11aW+V[[\x92P\x92\x90PV[_\x80_\x80_\x80`\x80\x87\x89\x03\x12\x15ab3Wab2aT\x9FV[[_ab@\x89\x82\x8A\x01aT\xBDV[\x96PP` abQ\x89\x82\x8A\x01aT\xBDV[\x95PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15abrWabqaT\xA3V[[ab~\x89\x82\x8A\x01aW/V[\x94P\x94PP``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab\xA1Wab\xA0aT\xA3V[[ab\xAD\x89\x82\x8A\x01aa\xC4V[\x92P\x92PP\x92\x95P\x92\x95P\x92\x95V[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_ab\xD6\x82a\\)V[ab\xE0\x81\x85ab\xBCV[\x93Pab\xF0\x81\x85` \x86\x01aT\x06V[ab\xF9\x81aT.V[\x84\x01\x91PP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Rac\x1C\x81\x85a`[V[\x90P\x81\x81\x03` \x83\x01Rac0\x81\x84ab\xCCV[\x90P\x93\x92PPPV[_` \x82\x84\x03\x12\x15acNWacMaT\x9FV[[_ac[\x84\x82\x85\x01aV\x9EV[\x91PP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rac|\x81\x84a^\xEDV[\x90P\x92\x91PPV[_\x81\x90P\x92\x91PPV[_ac\x98\x82aS\xECV[ac\xA2\x81\x85ac\x84V[\x93Pac\xB2\x81\x85` \x86\x01aT\x06V[\x80\x84\x01\x91PP\x92\x91PPV[\x7F v\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ac\xF2`\x02\x83ac\x84V[\x91Pac\xFD\x82ac\xBEV[`\x02\x82\x01\x90P\x91\x90PV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ad<`\x01\x83ac\x84V[\x91PadG\x82ad\x08V[`\x01\x82\x01\x90P\x91\x90PV[_ad]\x82\x87ac\x8EV[\x91Padh\x82ac\xE6V[\x91Padt\x82\x86ac\x8EV[\x91Pad\x7F\x82ad0V[\x91Pad\x8B\x82\x85ac\x8EV[\x91Pad\x96\x82ad0V[\x91Pad\xA2\x82\x84ac\x8EV[\x91P\x81\x90P\x95\x94PPPPPV[_\x81Q\x90Pad\xBE\x81aXjV[\x92\x91PPV[_` \x82\x84\x03\x12\x15ad\xD9Wad\xD8aT\x9FV[[_ad\xE6\x84\x82\x85\x01ad\xB0V[\x91PP\x92\x91PPV[_` \x82\x01\x90Pae\x02_\x83\x01\x84a^\x92V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[ae$\x81ae\x08V[\x82RPPV[_` \x82\x01\x90Pae=_\x83\x01\x84ae\x1BV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aez\x82aS\xBBV[\x91P\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x03ae\xACWae\xABaeCV[[`\x01\x82\x01\x90P\x91\x90PV[_\x81Q\x90Pae\xC5\x81aT\xA7V[\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15ae\xE1Wae\xE0aT\x9FV[[_ae\xEE\x85\x82\x86\x01ae\xB7V[\x92PP` ae\xFF\x85\x82\x86\x01ae\xB7V[\x91PP\x92P\x92\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\"`\x04R`$_\xFD[_`\x02\x82\x04\x90P`\x01\x82\x16\x80afMW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03af`Waf_af\tV[[P\x91\x90PV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_` `\x1F\x83\x01\x04\x90P\x91\x90PV[_\x82\x82\x1B\x90P\x92\x91PPV[_`\x08\x83\x02af\xC2\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82af\x87V[af\xCC\x86\x83af\x87V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[_\x81\x90P\x91\x90PV[_ag\x07ag\x02af\xFD\x84aS\xBBV[af\xE4V[aS\xBBV[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ag \x83af\xEDV[ag4ag,\x82ag\x0EV[\x84\x84Taf\x93V[\x82UPPPPV[_\x90V[agHag<V[agS\x81\x84\x84ag\x17V[PPPV[[\x81\x81\x10\x15agvWagk_\x82ag@V[`\x01\x81\x01\x90PagYV[PPV[`\x1F\x82\x11\x15ag\xBBWag\x8C\x81affV[ag\x95\x84afxV[\x81\x01` \x85\x10\x15ag\xA4W\x81\x90P[ag\xB8ag\xB0\x85afxV[\x83\x01\x82agXV[PP[PPPV[_\x82\x82\x1C\x90P\x92\x91PPV[_ag\xDB_\x19\x84`\x08\x02ag\xC0V[\x19\x80\x83\x16\x91PP\x92\x91PPV[_ag\xF3\x83\x83ag\xCCV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[ah\x0C\x82a\\)V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ah%Wah$aX\x98V[[ah/\x82Taf6V[ah:\x82\x82\x85agzV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ahkW_\x84\x15ahYW\x82\x87\x01Q\x90P[ahc\x85\x82ag\xE8V[\x86UPah\xCAV[`\x1F\x19\x84\x16ahy\x86affV[_[\x82\x81\x10\x15ah\xA0W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pah{V[\x86\x83\x10\x15ah\xBDW\x84\x89\x01Qah\xB9`\x1F\x89\x16\x82ag\xCCV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`\x80\x82\x01\x90Pah\xE5_\x83\x01\x87aS\xC4V[ah\xF2` \x83\x01\x86aS\xC4V[ah\xFF`@\x83\x01\x85aVgV[\x81\x81\x03``\x83\x01Rai\x11\x81\x84ab\xCCV[\x90P\x95\x94PPPPPV[_`@\x82\x01\x90Pai/_\x83\x01\x85aS\xC4V[ai<` \x83\x01\x84a^\x92V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[`\x02\x81\x10aiXW_\x80\xFD[PV[_\x815\x90Paii\x81aiLV[\x92\x91PPV[_ai}` \x84\x01\x84ai[V[\x90P\x92\x91PPV[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12ai\xADWai\xACai\x8DV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15ai\xD5Wai\xD4ai\x85V[[`\x01\x82\x026\x03\x83\x13\x15ai\xEBWai\xEAai\x89V[[P\x92P\x92\x90PV[_ai\xFE\x83\x85a\\3V[\x93Paj\x0B\x83\x85\x84aY@V[aj\x14\x83aT.V[\x84\x01\x90P\x93\x92PPPV[_`@\x83\x01aj0_\x84\x01\x84aioV[aj<_\x86\x01\x82a\\\x1AV[PajJ` \x84\x01\x84ai\x91V[\x85\x83\x03` \x87\x01Raj]\x83\x82\x84ai\xF3V[\x92PPP\x80\x91PP\x92\x91PPV[_ajv\x83\x83aj\x1FV[\x90P\x92\x91PPV[_\x825`\x01`@\x03\x836\x03\x03\x81\x12aj\x99Waj\x98ai\x8DV[[\x82\x81\x01\x91PP\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aj\xBC\x83\x85a`\xCEV[\x93P\x83` \x84\x02\x85\x01aj\xCE\x84aiCV[\x80_[\x87\x81\x10\x15ak\x11W\x84\x84\x03\x89Raj\xE8\x82\x84aj~V[aj\xF2\x85\x82ajkV[\x94Paj\xFD\x83aj\xA5V[\x92P` \x8A\x01\x99PP`\x01\x81\x01\x90Paj\xD1V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_ak.\x83\x85ab\xBCV[\x93Pak;\x83\x85\x84aY@V[akD\x83aT.V[\x84\x01\x90P\x93\x92PPPV[_`\x80\x82\x01\x90Pakb_\x83\x01\x89aS\xC4V[\x81\x81\x03` \x83\x01Raku\x81\x87\x89aj\xB1V[\x90P\x81\x81\x03`@\x83\x01Rak\x8A\x81\x85\x87ak#V[\x90Pak\x99``\x83\x01\x84a^\x92V[\x97\x96PPPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_\x80\xFD[_\x80\xFD[_\x80\xFD[_\x825`\x01`@\x03\x836\x03\x03\x81\x12ak\xF8Wak\xF7ak\xD1V[[\x80\x83\x01\x91PP\x92\x91PPV[_\x815al\x10\x81aiLV[\x80\x91PP\x91\x90PV[_\x81_\x1B\x90P\x91\x90PV[_`\xFFal0\x84al\x19V[\x93P\x80\x19\x83\x16\x92P\x80\x84\x16\x83\x17\x91PP\x92\x91PPV[_alP\x82a[\xF7V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[ali\x82alFV[al|alu\x82alWV[\x83Tal$V[\x82UPPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12al\x9FWal\x9Eak\xD1V[[\x80\x84\x01\x92P\x825\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15al\xC1Wal\xC0ak\xD5V[[` \x83\x01\x92P`\x01\x82\x026\x03\x83\x13\x15al\xDDWal\xDCak\xD9V[[P\x92P\x92\x90PV[_\x82\x90P\x92\x91PPV[al\xF9\x83\x83al\xE5V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15am\x12Wam\x11aX\x98V[[am\x1C\x82Taf6V[am'\x82\x82\x85agzV[_`\x1F\x83\x11`\x01\x81\x14amTW_\x84\x15amBW\x82\x87\x015\x90P[amL\x85\x82ag\xE8V[\x86UPam\xB3V[`\x1F\x19\x84\x16amb\x86affV[_[\x82\x81\x10\x15am\x89W\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PamdV[\x86\x83\x10\x15am\xA6W\x84\x89\x015am\xA2`\x1F\x89\x16\x82ag\xCCV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[am\xC7\x83\x83\x83al\xEFV[PPPV[_\x81\x01_\x83\x01\x80am\xDC\x81al\x04V[\x90Pam\xE8\x81\x84al`V[PPP`\x01\x81\x01` \x83\x01am\xFD\x81\x85al\x83V[an\x08\x81\x83\x86am\xBCV[PPPPPPV[an\x1A\x82\x82am\xCCV[PPV[_``\x82\x01\x90Pan1_\x83\x01\x87aS\xC4V[\x81\x81\x03` \x83\x01RanC\x81\x86a`[V[\x90P\x81\x81\x03`@\x83\x01RanX\x81\x84\x86aj\xB1V[\x90P\x95\x94PPPPPV[_\x80\xFD[\x82\x81\x837PPPV[_an{\x83\x85a^\xABV[\x93P\x7F\x07\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x15an\xAEWan\xADancV[[` \x83\x02\x92Pan\xBF\x83\x85\x84angV[\x82\x84\x01\x90P\x93\x92PPPV[_`\xA0\x82\x01\x90Pan\xDE_\x83\x01\x8AaS\xC4V[\x81\x81\x03` \x83\x01Ran\xF1\x81\x88\x8AanpV[\x90P\x81\x81\x03`@\x83\x01Rao\x06\x81\x86\x88anpV[\x90Pao\x15``\x83\x01\x85aS\xC4V[ao\"`\x80\x83\x01\x84aS\xC4V[\x98\x97PPPPPPPPV[_``\x82\x01\x90PaoA_\x83\x01\x87aS\xC4V[\x81\x81\x03` \x83\x01RaoT\x81\x85\x87ak#V[\x90Paoc`@\x83\x01\x84a^\x92V[\x95\x94PPPPPV[_`\xA0\x82\x01\x90Pao\x7F_\x83\x01\x88aS\xC4V[ao\x8C` \x83\x01\x87aS\xC4V[ao\x99`@\x83\x01\x86aS\xC4V[ao\xA6``\x83\x01\x85aV\xFBV[\x81\x81\x03`\x80\x83\x01Rao\xB8\x81\x84ab\xCCV[\x90P\x96\x95PPPPPPV[_``\x82\x01\x90Pao\xD7_\x83\x01\x86aS\xC4V[ao\xE4` \x83\x01\x85aS\xC4V[\x81\x81\x03`@\x83\x01Rao\xF6\x81\x84ab\xCCV[\x90P\x94\x93PPPPV[_`\x80\x82\x01\x90Pap\x13_\x83\x01\x89aS\xC4V[\x81\x81\x03` \x83\x01Rap&\x81\x87\x89ak#V[\x90P\x81\x81\x03`@\x83\x01Rap;\x81\x85\x87ak#V[\x90PapJ``\x83\x01\x84a^\x92V[\x97\x96PPPPPPPV[_``\x82\x01\x90Paph_\x83\x01\x87aS\xC4V[\x81\x81\x03` \x83\x01Rapz\x81\x86a`[V[\x90P\x81\x81\x03`@\x83\x01Rap\x8F\x81\x84\x86ak#V[\x90P\x95\x94PPPPPV[\x7FEIP712: Uninitialized\0\0\0\0\0\0\0\0\0\0\0_\x82\x01RPV[_ap\xCE`\x15\x83aS\xF6V[\x91Pap\xD9\x82ap\x9AV[` \x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Rap\xFB\x81ap\xC2V[\x90P\x91\x90PV[_`@\x82\x01\x90Paq\x15_\x83\x01\x85aS\xC4V[aq\"` \x83\x01\x84aS\xC4V[\x93\x92PPPV[_\x81\x90P\x91\x90PV[_aq=\x83\x85a_\xF4V[\x93PaqJ\x83\x85\x84aY@V[aqS\x83aT.V[\x84\x01\x90P\x93\x92PPPV[_aqj\x84\x84\x84aq2V[\x90P\x93\x92PPPV[_\x80\x835`\x01` \x03\x846\x03\x03\x81\x12aq\x8FWaq\x8Eai\x8DV[[\x83\x81\x01\x92P\x825\x91P` \x83\x01\x92Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aq\xB7Waq\xB6ai\x85V[[`\x01\x82\x026\x03\x83\x13\x15aq\xCDWaq\xCCai\x89V[[P\x92P\x92\x90PV[_` \x82\x01\x90P\x91\x90PV[_aq\xEC\x83\x85a_\xD5V[\x93P\x83` \x84\x02\x85\x01aq\xFE\x84aq)V[\x80_[\x87\x81\x10\x15arCW\x84\x84\x03\x89Rar\x18\x82\x84aqsV[ar#\x86\x82\x84aq^V[\x95Par.\x84aq\xD5V[\x93P` \x8B\x01\x9APPP`\x01\x81\x01\x90Par\x01V[P\x82\x97P\x87\x94PPPPP\x93\x92PPPV[_`\x80\x82\x01\x90Parh_\x83\x01\x89aS\xC4V[\x81\x81\x03` \x83\x01Rar{\x81\x87\x89aq\xE1V[\x90P\x81\x81\x03`@\x83\x01Rar\x90\x81\x85\x87aj\xB1V[\x90Par\x9F``\x83\x01\x84aS\xC4V[\x97\x96PPPPPPPV[_ar\xB4\x82aS\xBBV[\x91Par\xBF\x83aS\xBBV[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15ar\xD7War\xD6aeCV[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[_\x81`\xF8\x1B\x90P\x91\x90PV[_as,\x82as\x16V[\x90P\x91\x90PV[asDas?\x82as\nV[as\"V[\x82RPPV[_\x81\x90P\x91\x90PV[asdas_\x82aS\xBBV[asJV[\x82RPPV[_asu\x82\x86as3V[`\x01\x82\x01\x91Pas\x85\x82\x85asSV[` \x82\x01\x91Pas\x95\x82\x84asSV[` \x82\x01\x91P\x81\x90P\x94\x93PPPPV[as\xAF\x81aV\xF0V[\x81\x14as\xB9W_\x80\xFD[PV[_\x81Q\x90Pas\xCA\x81as\xA6V[\x92\x91PPV[_` \x82\x84\x03\x12\x15as\xE5Was\xE4aT\x9FV[[_as\xF2\x84\x82\x85\x01as\xBCV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15at\x10Wat\x0FaT\x9FV[[_at\x1D\x84\x82\x85\x01ai[V[\x91PP\x92\x91PPV[_\x81\x90P\x92\x91PPV[_at;\x83\x85at&V[\x93PatH\x83\x85\x84aY@V[\x82\x84\x01\x90P\x93\x92PPPV[_at`\x82\x84\x86at0V[\x91P\x81\x90P\x93\x92PPPV[atu\x81a\\\tV[\x82RPPV[_``\x82\x01\x90Pat\x8E_\x83\x01\x86aZ\x1FV[at\x9B` \x83\x01\x85atlV[at\xA8`@\x83\x01\x84aZ\x1FV[\x94\x93PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[at\xDC\x81aZ\x16V[\x82RPPV[_at\xED\x83\x83at\xD3V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_au\x0F\x82at\xB0V[au\x19\x81\x85at\xBAV[\x93Pau$\x83at\xC4V[\x80_[\x83\x81\x10\x15auTW\x81Qau;\x88\x82at\xE2V[\x97PauF\x83at\xF9V[\x92PP`\x01\x81\x01\x90Pau'V[P\x85\x93PPPP\x92\x91PPV[_aul\x82\x84au\x05V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Pau\x8A_\x83\x01\x88aZ\x1FV[au\x97` \x83\x01\x87aS\xC4V[au\xA4`@\x83\x01\x86aS\xC4V[au\xB1``\x83\x01\x85aZ\x1FV[au\xBE`\x80\x83\x01\x84aZ\x1FV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15au\xDDWau\xDCaT\x9FV[[_au\xEA\x84\x82\x85\x01ae\xB7V[\x91PP\x92\x91PPV[_\x80\xFD[_\x80\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15av\x15Wav\x14aX\x98V[[av\x1E\x82aT.V[\x90P` \x81\x01\x90P\x91\x90PV[_av=av8\x84au\xFBV[aX\xF6V[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15avYWavXaX\x94V[[avd\x84\x82\x85aT\x06V[P\x93\x92PPPV[_\x82`\x1F\x83\x01\x12av\x80Wav\x7FaW#V[[\x81Qav\x90\x84\x82` \x86\x01av+V[\x91PP\x92\x91PPV[_`\x80\x82\x84\x03\x12\x15av\xAEWav\xADau\xF3V[[av\xB8`\x80aX\xF6V[\x90P_av\xC7\x84\x82\x85\x01ad\xB0V[_\x83\x01RP` av\xDA\x84\x82\x85\x01ad\xB0V[` \x83\x01RP`@\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15av\xFEWav\xFDau\xF7V[[aw\n\x84\x82\x85\x01avlV[`@\x83\x01RP``\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aw.Waw-au\xF7V[[aw:\x84\x82\x85\x01avlV[``\x83\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15aw[WawZaT\x9FV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15awxWawwaT\xA3V[[aw\x84\x84\x82\x85\x01av\x99V[\x91PP\x92\x91PPV[aw\x96\x81aZ\x16V[\x81\x14aw\xA0W_\x80\xFD[PV[_\x81Q\x90Paw\xB1\x81aw\x8DV[\x92\x91PPV[_` \x82\x84\x03\x12\x15aw\xCCWaw\xCBaT\x9FV[[_aw\xD9\x84\x82\x85\x01aw\xA3V[\x91PP\x92\x91PPV[_``\x82\x01\x90Paw\xF5_\x83\x01\x86aZ\x1FV[ax\x02` \x83\x01\x85aS\xC4V[ax\x0F`@\x83\x01\x84aZ\x1FV[\x94\x93PPPPV[ax \x81as\nV[\x82RPPV[_` \x82\x01\x90Pax9_\x83\x01\x84ax\x17V[\x92\x91PPV[_``\x82\x01\x90PaxR_\x83\x01\x86aS\xC4V[ax_` \x83\x01\x85aVgV[\x81\x81\x03`@\x83\x01Raxq\x81\x84ab\xCCV[\x90P\x94\x93PPPPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[`\x1F\x82\x11\x15ax\xCEWax\x9F\x81ax{V[ax\xA8\x84afxV[\x81\x01` \x85\x10\x15ax\xB7W\x81\x90P[ax\xCBax\xC3\x85afxV[\x83\x01\x82agXV[PP[PPPV[ax\xDC\x82aS\xECV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ax\xF5Wax\xF4aX\x98V[[ax\xFF\x82Taf6V[ay\n\x82\x82\x85ax\x8DV[_` \x90P`\x1F\x83\x11`\x01\x81\x14ay;W_\x84\x15ay)W\x82\x87\x01Q\x90P[ay3\x85\x82ag\xE8V[\x86UPay\x9AV[`\x1F\x19\x84\x16ayI\x86ax{V[_[\x82\x81\x10\x15aypW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90PayKV[\x86\x83\x10\x15ay\x8DW\x84\x89\x01Qay\x89`\x1F\x89\x16\x82ag\xCCV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[_`@\x82\x01\x90Pay\xB5_\x83\x01\x85a^\x92V[ay\xC2` \x83\x01\x84a^\x92V[\x93\x92PPPV[_ay\xD3\x82a\\)V[ay\xDD\x81\x85at&V[\x93Pay\xED\x81\x85` \x86\x01aT\x06V[\x80\x84\x01\x91PP\x92\x91PPV[_az\x04\x82\x84ay\xC9V[\x91P\x81\x90P\x92\x91PPV[_`\xA0\x82\x01\x90Paz\"_\x83\x01\x88aZ\x1FV[az/` \x83\x01\x87aZ\x1FV[az<`@\x83\x01\x86aZ\x1FV[azI``\x83\x01\x85aS\xC4V[azV`\x80\x83\x01\x84a^\x92V[\x96\x95PPPPPPV[_`\x80\x82\x01\x90Pazs_\x83\x01\x87aZ\x1FV[az\x80` \x83\x01\x86ax\x17V[az\x8D`@\x83\x01\x85aZ\x1FV[az\x9A``\x83\x01\x84aZ\x1FV[\x95\x94PPPPPV\xFEPrepKeygenVerification(uint256 prepKeygenId,bytes extraData)CrsgenVerification(uint256 crsId,uint256 maxBitLength,bytes crsDigest,bytes extraData)KeygenVerification(uint256 prepKeygenId,uint256 keyId,KeyDigest[] keyDigests,bytes extraData)KeyDigest(uint8 keyType,bytes digest)KeyDigest(uint8 keyType,bytes digest)",
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
    /**Custom error with signature `KeyMaterialNotPublished(uint256)` and selector `0x05b083f2`.
```solidity
error KeyMaterialNotPublished(uint256 keyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyMaterialNotPublished {
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
        impl ::core::convert::From<KeyMaterialNotPublished> for UnderlyingRustTuple<'_> {
            fn from(value: KeyMaterialNotPublished) -> Self {
                (value.keyId,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyMaterialNotPublished {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { keyId: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyMaterialNotPublished {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyMaterialNotPublished(uint256)";
            const SELECTOR: [u8; 4] = [5u8, 176u8, 131u8, 242u8];
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
    /**Custom error with signature `MigrationKeyNotForExistingKey(uint256,uint256)` and selector `0x9431f34e`.
```solidity
error MigrationKeyNotForExistingKey(uint256 migrationKeyId, uint256 existingKeyId);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MigrationKeyNotForExistingKey {
        #[allow(missing_docs)]
        pub migrationKeyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub existingKeyId: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<MigrationKeyNotForExistingKey>
        for UnderlyingRustTuple<'_> {
            fn from(value: MigrationKeyNotForExistingKey) -> Self {
                (value.migrationKeyId, value.existingKeyId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MigrationKeyNotForExistingKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    migrationKeyId: tuple.0,
                    existingKeyId: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MigrationKeyNotForExistingKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MigrationKeyNotForExistingKey(uint256,uint256)";
            const SELECTOR: [u8; 4] = [148u8, 49u8, 243u8, 78u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.migrationKeyId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.existingKeyId),
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
    /**Custom error with signature `MismatchedMigrationArrays()` and selector `0x894b2ab3`.
```solidity
error MismatchedMigrationArrays();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MismatchedMigrationArrays;
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
        impl ::core::convert::From<MismatchedMigrationArrays>
        for UnderlyingRustTuple<'_> {
            fn from(value: MismatchedMigrationArrays) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MismatchedMigrationArrays {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MismatchedMigrationArrays {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MismatchedMigrationArrays()";
            const SELECTOR: [u8; 4] = [137u8, 75u8, 42u8, 179u8];
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
    #[derive()]
    /**Event with signature `KeyMaterialAdded(uint256,string[],(uint8,bytes)[],uint256)` and selector `0xa47664861ab58c5bd5040e9cc45e68d0e48ec04371035fd75099e217e0a6aa81`.
```solidity
event KeyMaterialAdded(uint256 keyId, string[] kmsNodeStorageUrls, IKMSGeneration.KeyDigest[] keyDigests, uint256 materialVersion);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyMaterialAdded {
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
        #[allow(missing_docs)]
        pub materialVersion: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KeyMaterialAdded {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeyMaterialAdded(uint256,string[],(uint8,bytes)[],uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8, 118u8, 100u8, 134u8, 26u8, 181u8, 140u8, 91u8, 213u8, 4u8, 14u8,
                156u8, 196u8, 94u8, 104u8, 208u8, 228u8, 142u8, 192u8, 67u8, 113u8, 3u8,
                95u8, 215u8, 80u8, 153u8, 226u8, 23u8, 224u8, 166u8, 170u8, 129u8,
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
                    materialVersion: data.3,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.materialVersion),
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
        impl alloy_sol_types::private::IntoLogData for KeyMaterialAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyMaterialAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeyMaterialAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KeyMaterialMigrationScheduled(uint256,uint256[],uint256[],uint256,uint256)` and selector `0x8bfa7d0ed6f87e526b62342918ee7bfa53952badd463dc934054d7dd940eafdc`.
```solidity
event KeyMaterialMigrationScheduled(uint256 keyId, uint256[] hostChainIds, uint256[] hostMigrationBlocks, uint256 gatewayMigrationBlock, uint256 materialVersion);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyMaterialMigrationScheduled {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hostChainIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub hostMigrationBlocks: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub gatewayMigrationBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub materialVersion: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for KeyMaterialMigrationScheduled {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "KeyMaterialMigrationScheduled(uint256,uint256[],uint256[],uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8, 250u8, 125u8, 14u8, 214u8, 248u8, 126u8, 82u8, 107u8, 98u8, 52u8,
                41u8, 24u8, 238u8, 123u8, 250u8, 83u8, 149u8, 43u8, 173u8, 212u8, 99u8,
                220u8, 147u8, 64u8, 84u8, 215u8, 221u8, 148u8, 14u8, 175u8, 220u8,
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
                    hostChainIds: data.1,
                    hostMigrationBlocks: data.2,
                    gatewayMigrationBlock: data.3,
                    materialVersion: data.4,
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostChainIds),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostMigrationBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.gatewayMigrationBlock,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.materialVersion),
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
        impl alloy_sol_types::private::IntoLogData for KeyMaterialMigrationScheduled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyMaterialMigrationScheduled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &KeyMaterialMigrationScheduled,
            ) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `MigrationKeygenRequest(uint256,uint256,uint256,bool,bytes)` and selector `0xe453c29c46ccc7664c0398e8464d5bb421e995432daf5506a3fdbc6aa0966a93`.
```solidity
event MigrationKeygenRequest(uint256 prepKeygenId, uint256 keyId, uint256 existingKeyId, bool copyToOriginal, bytes extraData);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MigrationKeygenRequest {
        #[allow(missing_docs)]
        pub prepKeygenId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub existingKeyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub copyToOriginal: bool,
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
        impl alloy_sol_types::SolEvent for MigrationKeygenRequest {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MigrationKeygenRequest(uint256,uint256,uint256,bool,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                228u8, 83u8, 194u8, 156u8, 70u8, 204u8, 199u8, 102u8, 76u8, 3u8, 152u8,
                232u8, 70u8, 77u8, 91u8, 180u8, 33u8, 233u8, 149u8, 67u8, 45u8, 175u8,
                85u8, 6u8, 163u8, 253u8, 188u8, 106u8, 160u8, 150u8, 106u8, 147u8,
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
                    existingKeyId: data.2,
                    copyToOriginal: data.3,
                    extraData: data.4,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.existingKeyId),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.copyToOriginal,
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
        impl alloy_sol_types::private::IntoLogData for MigrationKeygenRequest {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MigrationKeygenRequest> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MigrationKeygenRequest) -> alloy_sol_types::private::LogData {
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
    /**Function with signature `addKeyMaterials(uint256,uint256,(uint8,bytes)[],string[])` and selector `0xb53b3ccc`.
```solidity
function addKeyMaterials(uint256 existingKeyId, uint256 migrationKeyId, IKMSGeneration.KeyDigest[] memory keyDigests, string[] memory kmsNodeStorageUrls) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addKeyMaterialsCall {
        #[allow(missing_docs)]
        pub existingKeyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub migrationKeyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub keyDigests: alloy::sol_types::private::Vec<
            <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub kmsNodeStorageUrls: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
        >,
    }
    ///Container type for the return parameters of the [`addKeyMaterials(uint256,uint256,(uint8,bytes)[],string[])`](addKeyMaterialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addKeyMaterialsReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addKeyMaterialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: addKeyMaterialsCall) -> Self {
                    (
                        value.existingKeyId,
                        value.migrationKeyId,
                        value.keyDigests,
                        value.kmsNodeStorageUrls,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addKeyMaterialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        existingKeyId: tuple.0,
                        migrationKeyId: tuple.1,
                        keyDigests: tuple.2,
                        kmsNodeStorageUrls: tuple.3,
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
            impl ::core::convert::From<addKeyMaterialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addKeyMaterialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addKeyMaterialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addKeyMaterialsReturn {
            fn _tokenize(
                &self,
            ) -> <addKeyMaterialsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addKeyMaterialsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<IKMSGeneration::KeyDigest>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addKeyMaterialsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addKeyMaterials(uint256,uint256,(uint8,bytes)[],string[])";
            const SELECTOR: [u8; 4] = [181u8, 59u8, 60u8, 204u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.existingKeyId),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.migrationKeyId),
                    <alloy::sol_types::sol_data::Array<
                        IKMSGeneration::KeyDigest,
                    > as alloy_sol_types::SolType>::tokenize(&self.keyDigests),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.kmsNodeStorageUrls),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addKeyMaterialsReturn::_tokenize(ret)
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
    /**Function with signature `getKeyMaterialVersion(uint256)` and selector `0xf0f8cbc6`.
```solidity
function getKeyMaterialVersion(uint256 keyId) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialVersionCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyMaterialVersion(uint256)`](getKeyMaterialVersionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyMaterialVersionReturn {
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
            impl ::core::convert::From<getKeyMaterialVersionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialVersionCall) -> Self {
                    (value.keyId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyMaterialVersionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyId: tuple.0 }
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
            impl ::core::convert::From<getKeyMaterialVersionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getKeyMaterialVersionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getKeyMaterialVersionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyMaterialVersionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyMaterialVersion(uint256)";
            const SELECTOR: [u8; 4] = [240u8, 248u8, 203u8, 198u8];
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
                        let r: getKeyMaterialVersionReturn = r.into();
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
                        let r: getKeyMaterialVersionReturn = r.into();
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
    /**Function with signature `migrationKeygen(uint8,uint256)` and selector `0xaaa47016`.
```solidity
function migrationKeygen(IKMSGeneration.ParamsType paramsType, uint256 existingKeyId) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationKeygenCall {
        #[allow(missing_docs)]
        pub paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub existingKeyId: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`migrationKeygen(uint8,uint256)`](migrationKeygenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrationKeygenReturn {}
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
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<migrationKeygenCall> for UnderlyingRustTuple<'_> {
                fn from(value: migrationKeygenCall) -> Self {
                    (value.paramsType, value.existingKeyId)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for migrationKeygenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        paramsType: tuple.0,
                        existingKeyId: tuple.1,
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
            impl ::core::convert::From<migrationKeygenReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrationKeygenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrationKeygenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl migrationKeygenReturn {
            fn _tokenize(
                &self,
            ) -> <migrationKeygenCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrationKeygenCall {
            type Parameters<'a> = (
                IKMSGeneration::ParamsType,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrationKeygenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrationKeygen(uint8,uint256)";
            const SELECTOR: [u8; 4] = [170u8, 164u8, 112u8, 22u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.existingKeyId),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                migrationKeygenReturn::_tokenize(ret)
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
    /**Function with signature `reinitializeV2()` and selector `0xc4115874`.
```solidity
function reinitializeV2() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct reinitializeV2Call;
    ///Container type for the return parameters of the [`reinitializeV2()`](reinitializeV2Call) function.
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for reinitializeV2Call {
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
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = reinitializeV2Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "reinitializeV2()";
            const SELECTOR: [u8; 4] = [196u8, 17u8, 88u8, 116u8];
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
    /**Function with signature `scheduleKeyMaterialMigration(uint256,uint256[],uint256[],uint256)` and selector `0x56a610b4`.
```solidity
function scheduleKeyMaterialMigration(uint256 keyId, uint256[] memory hostChainIds, uint256[] memory hostMigrationBlocks, uint256 gatewayMigrationBlock) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleKeyMaterialMigrationCall {
        #[allow(missing_docs)]
        pub keyId: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub hostChainIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub hostMigrationBlocks: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        #[allow(missing_docs)]
        pub gatewayMigrationBlock: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`scheduleKeyMaterialMigration(uint256,uint256[],uint256[],uint256)`](scheduleKeyMaterialMigrationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct scheduleKeyMaterialMigrationReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<scheduleKeyMaterialMigrationCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleKeyMaterialMigrationCall) -> Self {
                    (
                        value.keyId,
                        value.hostChainIds,
                        value.hostMigrationBlocks,
                        value.gatewayMigrationBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleKeyMaterialMigrationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        keyId: tuple.0,
                        hostChainIds: tuple.1,
                        hostMigrationBlocks: tuple.2,
                        gatewayMigrationBlock: tuple.3,
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
            impl ::core::convert::From<scheduleKeyMaterialMigrationReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: scheduleKeyMaterialMigrationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for scheduleKeyMaterialMigrationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl scheduleKeyMaterialMigrationReturn {
            fn _tokenize(
                &self,
            ) -> <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::ReturnToken<
                '_,
            > {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for scheduleKeyMaterialMigrationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = scheduleKeyMaterialMigrationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "scheduleKeyMaterialMigration(uint256,uint256[],uint256[],uint256)";
            const SELECTOR: [u8; 4] = [86u8, 166u8, 16u8, 180u8];
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostChainIds),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.hostMigrationBlocks),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.gatewayMigrationBlock),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                scheduleKeyMaterialMigrationReturn::_tokenize(ret)
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
        addKeyMaterials(addKeyMaterialsCall),
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
        getKeyMaterialVersion(getKeyMaterialVersionCall),
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
        migrationKeygen(migrationKeygenCall),
        #[allow(missing_docs)]
        prepKeygenResponse(prepKeygenResponseCall),
        #[allow(missing_docs)]
        proxiableUUID(proxiableUUIDCall),
        #[allow(missing_docs)]
        reinitializeV2(reinitializeV2Call),
        #[allow(missing_docs)]
        scheduleKeyMaterialMigration(scheduleKeyMaterialMigrationCall),
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
            [86u8, 166u8, 16u8, 180u8],
            [88u8, 154u8, 219u8, 14u8],
            [98u8, 148u8, 244u8, 98u8],
            [98u8, 151u8, 135u8, 135u8],
            [132u8, 176u8, 25u8, 110u8],
            [147u8, 102u8, 8u8, 174u8],
            [170u8, 164u8, 112u8, 22u8],
            [173u8, 60u8, 177u8, 204u8],
            [181u8, 59u8, 60u8, 204u8],
            [186u8, 255u8, 33u8, 30u8],
            [194u8, 193u8, 250u8, 238u8],
            [196u8, 17u8, 88u8, 116u8],
            [197u8, 91u8, 135u8, 36u8],
            [202u8, 163u8, 103u8, 219u8],
            [213u8, 47u8, 16u8, 235u8],
            [218u8, 189u8, 115u8, 47u8],
            [228u8, 16u8, 17u8, 126u8],
            [240u8, 248u8, 203u8, 198u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KMSGenerationCalls {
        const NAME: &'static str = "KMSGenerationCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 31usize;
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
                Self::addKeyMaterials(_) => {
                    <addKeyMaterialsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getKeyMaterialVersion(_) => {
                    <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::migrationKeygen(_) => {
                    <migrationKeygenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::prepKeygenResponse(_) => {
                    <prepKeygenResponseCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::proxiableUUID(_) => {
                    <proxiableUUIDCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::reinitializeV2(_) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::scheduleKeyMaterialMigration(_) => {
                    <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn scheduleKeyMaterialMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::scheduleKeyMaterialMigration)
                    }
                    scheduleKeyMaterialMigration
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
                    fn migrationKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <migrationKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::migrationKeygen)
                    }
                    migrationKeygen
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
                    fn addKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::addKeyMaterials)
                    }
                    addKeyMaterials
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV2)
                    }
                    reinitializeV2
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
                    fn getKeyMaterialVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterialVersion)
                    }
                    getKeyMaterialVersion
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
                    fn scheduleKeyMaterialMigration(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::scheduleKeyMaterialMigration)
                    }
                    scheduleKeyMaterialMigration
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
                    fn migrationKeygen(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <migrationKeygenCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::migrationKeygen)
                    }
                    migrationKeygen
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
                    fn addKeyMaterials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::addKeyMaterials)
                    }
                    addKeyMaterials
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
                    fn reinitializeV2(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <reinitializeV2Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::reinitializeV2)
                    }
                    reinitializeV2
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
                    fn getKeyMaterialVersion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationCalls> {
                        <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationCalls::getKeyMaterialVersion)
                    }
                    getKeyMaterialVersion
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
                Self::addKeyMaterials(inner) => {
                    <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getKeyMaterialVersion(inner) => {
                    <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::migrationKeygen(inner) => {
                    <migrationKeygenCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::scheduleKeyMaterialMigration(inner) => {
                    <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::addKeyMaterials(inner) => {
                    <addKeyMaterialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getKeyMaterialVersion(inner) => {
                    <getKeyMaterialVersionCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::migrationKeygen(inner) => {
                    <migrationKeygenCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::reinitializeV2(inner) => {
                    <reinitializeV2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::scheduleKeyMaterialMigration(inner) => {
                    <scheduleKeyMaterialMigrationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        EmptyKeyDigests(EmptyKeyDigests),
        #[allow(missing_docs)]
        FailedCall(FailedCall),
        #[allow(missing_docs)]
        InvalidInitialization(InvalidInitialization),
        #[allow(missing_docs)]
        KeyAborted(KeyAborted),
        #[allow(missing_docs)]
        KeyManagementRequestPending(KeyManagementRequestPending),
        #[allow(missing_docs)]
        KeyMaterialNotPublished(KeyMaterialNotPublished),
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
        MigrationKeyNotForExistingKey(MigrationKeyNotForExistingKey),
        #[allow(missing_docs)]
        MismatchedMigrationArrays(MismatchedMigrationArrays),
        #[allow(missing_docs)]
        NotHostOwner(NotHostOwner),
        #[allow(missing_docs)]
        NotInitializing(NotInitializing),
        #[allow(missing_docs)]
        NotInitializingFromEmptyProxy(NotInitializingFromEmptyProxy),
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
            [5u8, 176u8, 131u8, 242u8],
            [6u8, 26u8, 198u8, 29u8],
            [10u8, 183u8, 246u8, 135u8],
            [13u8, 134u8, 245u8, 33u8],
            [33u8, 57u8, 204u8, 44u8],
            [33u8, 191u8, 218u8, 16u8],
            [51u8, 202u8, 31u8, 227u8],
            [59u8, 133u8, 61u8, 168u8],
            [76u8, 156u8, 140u8, 227u8],
            [111u8, 79u8, 115u8, 31u8],
            [111u8, 188u8, 221u8, 43u8],
            [131u8, 241u8, 131u8, 53u8],
            [132u8, 222u8, 19u8, 49u8],
            [137u8, 75u8, 42u8, 179u8],
            [139u8, 36u8, 139u8, 96u8],
            [141u8, 140u8, 148u8, 10u8],
            [146u8, 120u8, 155u8, 103u8],
            [148u8, 49u8, 243u8, 78u8],
            [152u8, 251u8, 149u8, 125u8],
            [153u8, 150u8, 179u8, 21u8],
            [170u8, 29u8, 73u8, 164u8],
            [173u8, 250u8, 185u8, 4u8],
            [174u8, 232u8, 99u8, 35u8],
            [179u8, 152u8, 151u8, 159u8],
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
        const COUNT: usize = 38usize;
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
                Self::EmptyKeyDigests(_) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedCall(_) => {
                    <FailedCall as alloy_sol_types::SolError>::SELECTOR
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
                Self::KeyMaterialNotPublished(_) => {
                    <KeyMaterialNotPublished as alloy_sol_types::SolError>::SELECTOR
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
                Self::MigrationKeyNotForExistingKey(_) => {
                    <MigrationKeyNotForExistingKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MismatchedMigrationArrays(_) => {
                    <MismatchedMigrationArrays as alloy_sol_types::SolError>::SELECTOR
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
                    fn KeyMaterialNotPublished(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyMaterialNotPublished)
                    }
                    KeyMaterialNotPublished
                },
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
                    fn MismatchedMigrationArrays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::MismatchedMigrationArrays)
                    }
                    MismatchedMigrationArrays
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
                    fn MigrationKeyNotForExistingKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <MigrationKeyNotForExistingKey as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(KMSGenerationErrors::MigrationKeyNotForExistingKey)
                    }
                    MigrationKeyNotForExistingKey
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
                    fn KeyMaterialNotPublished(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::KeyMaterialNotPublished)
                    }
                    KeyMaterialNotPublished
                },
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
                    fn MismatchedMigrationArrays(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::MismatchedMigrationArrays)
                    }
                    MismatchedMigrationArrays
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
                    fn MigrationKeyNotForExistingKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KMSGenerationErrors> {
                        <MigrationKeyNotForExistingKey as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KMSGenerationErrors::MigrationKeyNotForExistingKey)
                    }
                    MigrationKeyNotForExistingKey
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
                Self::EmptyKeyDigests(inner) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::KeyMaterialNotPublished(inner) => {
                    <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::MigrationKeyNotForExistingKey(inner) => {
                    <MigrationKeyNotForExistingKey as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MismatchedMigrationArrays(inner) => {
                    <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::EmptyKeyDigests(inner) => {
                    <EmptyKeyDigests as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedCall(inner) => {
                    <FailedCall as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::KeyMaterialNotPublished(inner) => {
                    <KeyMaterialNotPublished as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::MigrationKeyNotForExistingKey(inner) => {
                    <MigrationKeyNotForExistingKey as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MismatchedMigrationArrays(inner) => {
                    <MismatchedMigrationArrays as alloy_sol_types::SolError>::abi_encode_raw(
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
        CrsgenRequest(CrsgenRequest),
        #[allow(missing_docs)]
        CrsgenResponse(CrsgenResponse),
        #[allow(missing_docs)]
        EIP712DomainChanged(EIP712DomainChanged),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        KeyMaterialAdded(KeyMaterialAdded),
        #[allow(missing_docs)]
        KeyMaterialMigrationScheduled(KeyMaterialMigrationScheduled),
        #[allow(missing_docs)]
        KeygenRequest(KeygenRequest),
        #[allow(missing_docs)]
        KeygenResponse(KeygenResponse),
        #[allow(missing_docs)]
        MigrationKeygenRequest(MigrationKeygenRequest),
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
                123u8, 241u8, 180u8, 44u8, 16u8, 233u8, 73u8, 124u8, 135u8, 150u8, 32u8,
                197u8, 183u8, 175u8, 206u8, 209u8, 11u8, 218u8, 23u8, 216u8, 201u8, 11u8,
                34u8, 240u8, 227u8, 188u8, 107u8, 47u8, 214u8, 206u8, 208u8, 189u8,
            ],
            [
                139u8, 250u8, 125u8, 14u8, 214u8, 248u8, 126u8, 82u8, 107u8, 98u8, 52u8,
                41u8, 24u8, 238u8, 123u8, 250u8, 83u8, 149u8, 43u8, 173u8, 212u8, 99u8,
                220u8, 147u8, 64u8, 84u8, 215u8, 221u8, 148u8, 14u8, 175u8, 220u8,
            ],
            [
                140u8, 240u8, 21u8, 19u8, 147u8, 248u8, 79u8, 214u8, 148u8, 197u8, 227u8,
                21u8, 203u8, 116u8, 204u8, 5u8, 178u8, 71u8, 222u8, 10u8, 69u8, 79u8,
                217u8, 233u8, 18u8, 156u8, 102u8, 30u8, 253u8, 249u8, 64u8, 29u8,
            ],
            [
                164u8, 118u8, 100u8, 134u8, 26u8, 181u8, 140u8, 91u8, 213u8, 4u8, 14u8,
                156u8, 196u8, 94u8, 104u8, 208u8, 228u8, 142u8, 192u8, 67u8, 113u8, 3u8,
                95u8, 215u8, 80u8, 153u8, 226u8, 23u8, 224u8, 166u8, 170u8, 129u8,
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
                228u8, 83u8, 194u8, 156u8, 70u8, 204u8, 199u8, 102u8, 76u8, 3u8, 152u8,
                232u8, 70u8, 77u8, 91u8, 180u8, 33u8, 233u8, 149u8, 67u8, 45u8, 175u8,
                85u8, 6u8, 163u8, 253u8, 188u8, 106u8, 160u8, 150u8, 106u8, 147u8,
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
        const COUNT: usize = 16usize;
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
                Some(<KeyMaterialAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeyMaterialAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeyMaterialAdded)
                }
                Some(
                    <KeyMaterialMigrationScheduled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <KeyMaterialMigrationScheduled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::KeyMaterialMigrationScheduled)
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
                    <MigrationKeygenRequest as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MigrationKeygenRequest as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::MigrationKeygenRequest)
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
                Self::KeyMaterialAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeyMaterialMigrationScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MigrationKeygenRequest(inner) => {
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
                Self::KeyMaterialAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeyMaterialMigrationScheduled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenRequest(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeygenResponse(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MigrationKeygenRequest(inner) => {
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
        ///Creates a new call builder for the [`addKeyMaterials`] function.
        pub fn addKeyMaterials(
            &self,
            existingKeyId: alloy::sol_types::private::primitives::aliases::U256,
            migrationKeyId: alloy::sol_types::private::primitives::aliases::U256,
            keyDigests: alloy::sol_types::private::Vec<
                <IKMSGeneration::KeyDigest as alloy::sol_types::SolType>::RustType,
            >,
            kmsNodeStorageUrls: alloy::sol_types::private::Vec<
                alloy::sol_types::private::String,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, addKeyMaterialsCall, N> {
            self.call_builder(
                &addKeyMaterialsCall {
                    existingKeyId,
                    migrationKeyId,
                    keyDigests,
                    kmsNodeStorageUrls,
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
        ///Creates a new call builder for the [`getKeyMaterialVersion`] function.
        pub fn getKeyMaterialVersion(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyMaterialVersionCall, N> {
            self.call_builder(&getKeyMaterialVersionCall { keyId })
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
        ///Creates a new call builder for the [`migrationKeygen`] function.
        pub fn migrationKeygen(
            &self,
            paramsType: <IKMSGeneration::ParamsType as alloy::sol_types::SolType>::RustType,
            existingKeyId: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, migrationKeygenCall, N> {
            self.call_builder(
                &migrationKeygenCall {
                    paramsType,
                    existingKeyId,
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
        ///Creates a new call builder for the [`reinitializeV2`] function.
        pub fn reinitializeV2(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, reinitializeV2Call, N> {
            self.call_builder(&reinitializeV2Call)
        }
        ///Creates a new call builder for the [`scheduleKeyMaterialMigration`] function.
        pub fn scheduleKeyMaterialMigration(
            &self,
            keyId: alloy::sol_types::private::primitives::aliases::U256,
            hostChainIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            hostMigrationBlocks: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            gatewayMigrationBlock: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, scheduleKeyMaterialMigrationCall, N> {
            self.call_builder(
                &scheduleKeyMaterialMigrationCall {
                    keyId,
                    hostChainIds,
                    hostMigrationBlocks,
                    gatewayMigrationBlock,
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
        ///Creates a new event filter for the [`KeyMaterialAdded`] event.
        pub fn KeyMaterialAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeyMaterialAdded, N> {
            self.event_filter::<KeyMaterialAdded>()
        }
        ///Creates a new event filter for the [`KeyMaterialMigrationScheduled`] event.
        pub fn KeyMaterialMigrationScheduled_filter(
            &self,
        ) -> alloy_contract::Event<&P, KeyMaterialMigrationScheduled, N> {
            self.event_filter::<KeyMaterialMigrationScheduled>()
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
        ///Creates a new event filter for the [`MigrationKeygenRequest`] event.
        pub fn MigrationKeygenRequest_filter(
            &self,
        ) -> alloy_contract::Event<&P, MigrationKeygenRequest, N> {
            self.event_filter::<MigrationKeygenRequest>()
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
