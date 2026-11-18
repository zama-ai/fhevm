//! Bounds dynamic ABI fields before Alloy decodes them.
//!
//! Dynamic tails must remain in order and disjoint. This prevents multiple
//! array elements from aliasing the same encoded tail and multiplying the
//! allocations Alloy makes while materializing nested vectors.
use alloy::sol_types::{
    abi::token::{DynSeqToken, FixedSeqToken, PackedSeqToken, WordToken},
    SolEvent,
};

use crate::contracts::{
    AclContract, KMSGeneration, ProtocolConfig, TfheContract,
};

const ABI_WORD_BYTES: usize = 32;

trait AbiShape {
    const DYNAMIC: bool;
    const STATIC_SIZE: Option<usize>;

    fn validate_at(data: &[u8], offset: usize) -> Option<usize>;
}

fn read_usize_word(data: &[u8], offset: usize) -> Option<usize> {
    let end = offset.checked_add(ABI_WORD_BYTES)?;
    let word = data.get(offset..end)?;
    let value_bytes = std::mem::size_of::<usize>();
    if word[..ABI_WORD_BYTES - value_bytes]
        .iter()
        .any(|byte| *byte != 0)
    {
        return None;
    }

    word[ABI_WORD_BYTES - value_bytes..]
        .iter()
        .try_fold(0usize, |value, byte| {
            value.checked_mul(256)?.checked_add(usize::from(*byte))
        })
}

fn dynamic_target(
    data: &[u8],
    head_offset: usize,
    sequence_offset: usize,
    head_size: usize,
) -> Option<usize> {
    let relative_offset = read_usize_word(data, head_offset)?;
    if relative_offset < head_size || relative_offset % ABI_WORD_BYTES != 0 {
        return None;
    }
    let target = sequence_offset.checked_add(relative_offset)?;
    target.checked_add(ABI_WORD_BYTES)?;
    Some(target)
}

fn validate_repeated<T: AbiShape>(
    data: &[u8],
    sequence_offset: usize,
    count: usize,
) -> Option<usize> {
    let head_word_size = if T::DYNAMIC {
        ABI_WORD_BYTES
    } else {
        T::STATIC_SIZE?
    };
    let head_size = count.checked_mul(head_word_size)?;
    let head_end = sequence_offset.checked_add(head_size)?;
    data.get(sequence_offset..head_end)?;
    if !T::DYNAMIC {
        return Some(head_end);
    }

    let mut tail_end = head_end;
    for index in 0..count {
        let head_offset = index
            .checked_mul(ABI_WORD_BYTES)
            .and_then(|relative| sequence_offset.checked_add(relative))?;
        let target =
            dynamic_target(data, head_offset, sequence_offset, head_size)?;
        if target < tail_end {
            return None;
        }
        tail_end = T::validate_at(data, target)?;
    }
    Some(tail_end)
}

impl AbiShape for WordToken {
    const DYNAMIC: bool = false;
    const STATIC_SIZE: Option<usize> = Some(ABI_WORD_BYTES);

    fn validate_at(data: &[u8], offset: usize) -> Option<usize> {
        let end = offset.checked_add(ABI_WORD_BYTES)?;
        data.get(offset..end)?;
        Some(end)
    }
}

impl AbiShape for () {
    const DYNAMIC: bool = false;
    const STATIC_SIZE: Option<usize> = Some(0);

    fn validate_at(data: &[u8], offset: usize) -> Option<usize> {
        data.get(offset..offset)?;
        Some(offset)
    }
}

impl AbiShape for PackedSeqToken<'_> {
    const DYNAMIC: bool = true;
    const STATIC_SIZE: Option<usize> = None;

    fn validate_at(data: &[u8], offset: usize) -> Option<usize> {
        let length = read_usize_word(data, offset)?;
        let payload_start = offset.checked_add(ABI_WORD_BYTES)?;
        let padded_length = length
            .checked_add(ABI_WORD_BYTES - 1)?
            .checked_div(ABI_WORD_BYTES)?
            .checked_mul(ABI_WORD_BYTES)?;
        let padded_end = payload_start.checked_add(padded_length)?;
        data.get(payload_start..padded_end)?;
        Some(padded_end)
    }
}

impl<T: AbiShape> AbiShape for DynSeqToken<T> {
    const DYNAMIC: bool = true;
    const STATIC_SIZE: Option<usize> = None;

    fn validate_at(data: &[u8], offset: usize) -> Option<usize> {
        let count = read_usize_word(data, offset)?;
        let sequence_offset = offset.checked_add(ABI_WORD_BYTES)?;
        validate_repeated::<T>(data, sequence_offset, count)
    }
}

impl<T: AbiShape, const N: usize> AbiShape for FixedSeqToken<T, N> {
    const DYNAMIC: bool = T::DYNAMIC;
    const STATIC_SIZE: Option<usize> = if T::DYNAMIC {
        None
    } else {
        Some(T::STATIC_SIZE.unwrap() * N)
    };

    fn validate_at(data: &[u8], offset: usize) -> Option<usize> {
        validate_repeated::<T>(data, offset, N)
    }
}

macro_rules! impl_tuple_abi_shape {
    ($($ty:ident),+) => {
        impl<$($ty: AbiShape),+> AbiShape for ($($ty,)+) {
            const DYNAMIC: bool = false $(|| $ty::DYNAMIC)+;
            const STATIC_SIZE: Option<usize> = if Self::DYNAMIC {
                None
            } else {
                Some(0 $(+ $ty::STATIC_SIZE.unwrap())+)
            };

            #[allow(unused_assignments)]
            fn validate_at(data: &[u8], sequence_offset: usize) -> Option<usize> {
                let head_size = 0usize $(+ if $ty::DYNAMIC {
                    ABI_WORD_BYTES
                } else {
                    $ty::STATIC_SIZE.unwrap_or(0)
                })+;
                let head_end = sequence_offset.checked_add(head_size)?;
                data.get(sequence_offset..head_end)?;
                let mut head_offset = sequence_offset;
                let mut tail_end = head_end;
                $(
                    if $ty::DYNAMIC {
                        let target = dynamic_target(
                            data,
                            head_offset,
                            sequence_offset,
                            head_size,
                        )?;
                        if target < tail_end {
                            return None;
                        }
                        tail_end = $ty::validate_at(data, target)?;
                        let next = head_offset.checked_add(ABI_WORD_BYTES)?;
                        head_offset = next;
                    } else {
                        let size = $ty::STATIC_SIZE?;
                        let next = head_offset.checked_add(size)?;
                        data.get(head_offset..next)?;
                        head_offset = next;
                    }
                )+
                Some(tail_end)
            }
        }
    };
}

impl_tuple_abi_shape!(A);
impl_tuple_abi_shape!(A, B);
impl_tuple_abi_shape!(A, B, C);
impl_tuple_abi_shape!(A, B, C, D);
impl_tuple_abi_shape!(A, B, C, D, E);
impl_tuple_abi_shape!(A, B, C, D, E, F);
impl_tuple_abi_shape!(A, B, C, D, E, F, G);
impl_tuple_abi_shape!(A, B, C, D, E, F, G, H);
impl_tuple_abi_shape!(A, B, C, D, E, F, G, H, I);
impl_tuple_abi_shape!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_abi_shape!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple_abi_shape!(A, B, C, D, E, F, G, H, I, J, K, L);

fn validate_event_body<E: SolEvent>(data: &[u8]) -> bool
where
    for<'a> E::DataToken<'a>: AbiShape,
{
    <E::DataToken<'_> as AbiShape>::validate_at(data, 0).is_some()
}

fn dynamic_payload_fits(log: &alloy::primitives::Log) -> bool {
    let Some(selector) = log.data.topics().first().map(|topic| topic.0) else {
        return true;
    };
    let data = log.data.data.as_ref();

    macro_rules! check_event {
        ($event:ty) => {
            if selector == <$event as SolEvent>::SIGNATURE_HASH.0 {
                return validate_event_body::<$event>(data);
            }
        };
    }

    check_event!(AclContract::AllowedForDecryption);
    check_event!(TfheContract::FheSum);
    check_event!(TfheContract::FheIsIn);
    check_event!(TfheContract::VerifyInput);
    check_event!(KMSGeneration::PrepKeygenRequest);
    check_event!(KMSGeneration::PrepKeygenResponse);
    check_event!(KMSGeneration::KeygenRequest);
    check_event!(KMSGeneration::KeygenResponse);
    check_event!(KMSGeneration::ActivateKey);
    check_event!(KMSGeneration::CrsgenRequest);
    check_event!(KMSGeneration::CrsgenResponse);
    check_event!(KMSGeneration::ActivateCrs);
    check_event!(ProtocolConfig::NewKmsContext);
    check_event!(ProtocolConfig::ActivateEpoch);
    check_event!(ProtocolConfig::CoprocessorUpgradeProposed);

    true
}

pub(crate) fn dynamic_event_payload_fits(log: &alloy::primitives::Log) -> bool {
    dynamic_payload_fits(log)
}
