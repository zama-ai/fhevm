use crate::{
    tfhe_ops::*,
    types::{FhevmError, SupportedFheCiphertexts, SupportedFheOperations},
};
use lazy_static::lazy_static;
use tfhe::{core_crypto::gpu::get_number_of_gpus, prelude::*, FheUint2, GpuIndex};

lazy_static! {
    pub static ref gpu_mem_reservation: Vec<std::sync::atomic::AtomicU64> = (0
        ..get_number_of_gpus())
        .map(|_| std::sync::atomic::AtomicU64::new(0))
        .collect::<Vec<_>>();
}

impl SupportedFheCiphertexts {
    pub fn move_to_current_device(&mut self) {
        match self {
            SupportedFheCiphertexts::FheBool(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint4(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint8(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint16(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint32(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint64(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint128(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint160(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheUint256(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheBytes64(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheBytes128(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::FheBytes256(v) => v.move_to_current_device(),
            SupportedFheCiphertexts::Scalar(_) => {}
        };
    }

    pub fn get_size_on_gpu(&self) -> u64 {
        match self {
            SupportedFheCiphertexts::FheBool(v) => {
                let v: FheUint2 = v.to_owned().cast_into();
                v.get_size_on_gpu()
            } // TODO fix when available
            SupportedFheCiphertexts::FheUint4(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint8(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint16(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint32(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint64(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint128(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint160(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheUint256(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheBytes64(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheBytes128(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::FheBytes256(v) => v.get_size_on_gpu(),
            SupportedFheCiphertexts::Scalar(v) => v.len() as u64,
        }
    }
}

pub fn get_supported_ct_size_on_gpu(ct_type: i16) -> u64 {
    trivial_encrypt_be_bytes(ct_type, &[1u8]).get_size_on_gpu()
}

// Reserving GPU memory happens in two stages:
//  - we add the amount we need atomically to the GPU's reservation pool
//  - we check that the new pool fits on GPU
//    - if it does, we continue and allocate, then remove the reservation from the pool
//    - if it doesn't, we remove from the pool and for now simply retry after a short interval
// TODO: refine retrying, possibly targeting a different GPU where appropriate
pub fn reserve_memory_on_gpu(amount: u64, idx: usize) {
    loop {
        let old_pool_size =
            gpu_mem_reservation[idx].fetch_add(amount, std::sync::atomic::Ordering::SeqCst);
        if check_valid_cuda_malloc(old_pool_size + amount, GpuIndex::new(idx as u32)) {
            break;
        } else {
            // Remove reservation as failed
            let _ = gpu_mem_reservation[idx].fetch_sub(amount, std::sync::atomic::Ordering::SeqCst);
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }
}
pub fn release_memory_on_gpu(amount: u64, idx: usize) {
    let current_pool_size = gpu_mem_reservation[idx].load(std::sync::atomic::Ordering::SeqCst);
    assert!(current_pool_size >= amount);
    let _ = gpu_mem_reservation[idx].fetch_sub(amount, std::sync::atomic::Ordering::SeqCst);
}

pub fn get_op_size_on_gpu(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
    // for deterministc randomness functions
) -> Result<u64, FhevmError> {
    let fhe_operation: SupportedFheOperations =
        fhe_operation_int.try_into().expect("Invalid operation");
    match fhe_operation {
        SupportedFheOperations::FheAdd => {
            assert_eq!(input_operands.len(), 2);

            // fhe add
            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_add_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_add_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_add_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_add_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_add_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }

        SupportedFheOperations::FheSub => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_sub_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_sub_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_sub_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_sub_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_sub_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }

        SupportedFheOperations::FheMul => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_mul_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_mul_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_mul_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_mul_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_mul_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheDiv => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_div_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_div_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_div_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_div_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_div_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRem => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_rem_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_rem_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_rem_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_rem_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rem_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitAnd => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_bitand_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_bitand_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u4_bit(b) > 0))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitand_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitOr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_bitor_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_bitor_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_bitor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitor_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheBitXor => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_bitxor_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_bitxor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_bitxor_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheShl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_left_shift_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_left_shift_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheShr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_right_shift_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_right_shift_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRotl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_rotate_left_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_left_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRotr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_rotate_right_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_rotate_right_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMin => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_min_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_min_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_min_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheMax => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_max_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_max_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_max_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheEq => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_eq_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_eq_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_eq_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_eq_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_ne_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_ne_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_ne_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u256_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u512_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u1024_bit(b)))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ne_size_on_gpu(to_be_u2048_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheGe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_ge_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_ge_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_ge_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_ge_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheGt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_gt_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_gt_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_gt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_gt_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheLe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_le_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_le_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_le_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_le_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheLt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(a.get_lt_size_on_gpu(b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(a.get_lt_size_on_gpu(b)),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    Ok(a.get_lt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u4_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u8_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u16_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u32_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u64_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u128_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u160_bit(b)))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    Ok(a.get_lt_size_on_gpu(to_be_u256_bit(b)))
                }
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNot => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheBool(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint4(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint8(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint16(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint32(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint64(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint128(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint160(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheUint256(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheBytes64(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheBytes128(a) => Ok(a.get_bitnot_size_on_gpu()),
                SupportedFheCiphertexts::FheBytes256(a) => Ok(a.get_bitnot_size_on_gpu()),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheNeg => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint4(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint8(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint16(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint32(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint64(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint128(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint160(a) => Ok(a.get_neg_size_on_gpu()),
                SupportedFheCiphertexts::FheUint256(a) => Ok(a.get_neg_size_on_gpu()),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheIfThenElse => {
            assert_eq!(input_operands.len(), 3);

            let SupportedFheCiphertexts::FheBool(flag) = &input_operands[0] else {
                return Ok(0);
            };

            match (&input_operands[1], &input_operands[2]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    let b: FheUint2 = b.to_owned().cast_into();
                    Ok(flag.get_if_then_else_size_on_gpu(&a, &b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    Ok(flag.get_if_then_else_size_on_gpu(a, b))
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => Ok(flag.get_if_then_else_size_on_gpu(a, b)),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheTrivialEncrypt | SupportedFheOperations::FheCast => {
            match (&input_operands[0], &input_operands[1]) {
                (_, SupportedFheCiphertexts::Scalar(op)) => Ok(trivial_encrypt_be_bytes(
                    to_be_u16_bit(op) as i16,
                    &[1u8],
                )
                .get_size_on_gpu()),
                (_, _) => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRand => {
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[1] else {
                return Ok(0);
            };
            let to_type = to_be_u16_bit(to_type) as i16;
            match to_type {
                0 => Ok(tfhe::FheUint2::get_generate_oblivious_pseudo_random_size_on_gpu()),
                1 => Ok(tfhe::FheUint4::get_generate_oblivious_pseudo_random_size_on_gpu()),
                2 => Ok(tfhe::FheUint8::get_generate_oblivious_pseudo_random_size_on_gpu()),
                3 => Ok(tfhe::FheUint16::get_generate_oblivious_pseudo_random_size_on_gpu()),
                4 => Ok(tfhe::FheUint32::get_generate_oblivious_pseudo_random_size_on_gpu()),
                5 => Ok(tfhe::FheUint64::get_generate_oblivious_pseudo_random_size_on_gpu()),
                6 => Ok(tfhe::FheUint128::get_generate_oblivious_pseudo_random_size_on_gpu()),
                7 => Ok(tfhe::FheUint160::get_generate_oblivious_pseudo_random_size_on_gpu()),
                8 => Ok(tfhe::FheUint256::get_generate_oblivious_pseudo_random_size_on_gpu()),
                9 => Ok(tfhe::FheUint512::get_generate_oblivious_pseudo_random_size_on_gpu()),
                10 => Ok(tfhe::FheUint1024::get_generate_oblivious_pseudo_random_size_on_gpu()),
                11 => Ok(tfhe::FheUint2048::get_generate_oblivious_pseudo_random_size_on_gpu()),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        SupportedFheOperations::FheRandBounded => {
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[2] else {
                return Ok(0);
            };
            let to_type = to_be_u16_bit(to_type) as i16;
            match to_type {
                0 => Ok(tfhe::FheUint2::get_generate_oblivious_pseudo_random_bounded_size_on_gpu()),
                1 => Ok(tfhe::FheUint4::get_generate_oblivious_pseudo_random_bounded_size_on_gpu()),
                2 => Ok(tfhe::FheUint8::get_generate_oblivious_pseudo_random_bounded_size_on_gpu()),
                3 => {
                    Ok(tfhe::FheUint16::get_generate_oblivious_pseudo_random_bounded_size_on_gpu())
                }
                4 => {
                    Ok(tfhe::FheUint32::get_generate_oblivious_pseudo_random_bounded_size_on_gpu())
                }
                5 => {
                    Ok(tfhe::FheUint64::get_generate_oblivious_pseudo_random_bounded_size_on_gpu())
                }
                6 => Ok(
                    tfhe::FheUint128::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                7 => Ok(
                    tfhe::FheUint160::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                8 => Ok(
                    tfhe::FheUint256::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                9 => Ok(
                    tfhe::FheUint512::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                10 => Ok(
                    tfhe::FheUint1024::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                11 => Ok(
                    tfhe::FheUint2048::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                ),
                _ => Err(FhevmError::UnsupportedFheTypes {
                    fhe_operation: format!("{:?}", fhe_operation),
                    input_types: input_operands.iter().map(|i| i.type_name()).collect(),
                }),
            }
        }
        _ => Err(FhevmError::UnknownFheOperation(fhe_operation_int.into())),
    }
}
