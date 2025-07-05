use crate::{
    tfhe_ops::*,
    types::{SupportedFheCiphertexts, SupportedFheOperations},
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
            SupportedFheCiphertexts::Scalar(_) => {} // TODO - need to move scalars?
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

// TODO: boolean ops & missing (Mul, Div, Rem, Eq, Ne, ...)
pub fn get_op_size_on_gpu(
    fhe_operation_int: i16,
    input_operands: &[SupportedFheCiphertexts],
    // for deterministc randomness functions
) -> u64 {
    let fhe_operation: SupportedFheOperations =
        fhe_operation_int.try_into().expect("Invalid operation");
    match fhe_operation {
        SupportedFheOperations::FheAdd => {
            assert_eq!(input_operands.len(), 2);

            // fhe add
            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_add_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_add_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_add_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_add_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_add_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_add_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_add_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_add_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_add_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }

        SupportedFheOperations::FheSub => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_sub_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_sub_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_sub_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_sub_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_sub_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_sub_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_sub_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_sub_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_sub_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }

        SupportedFheOperations::FheMul => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_mul_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_mul_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_mul_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_mul_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_mul_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_mul_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_mul_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_mul_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_mul_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheDiv => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_div_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_div_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_div_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_div_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_div_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_div_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_div_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_div_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_div_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheRem => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_rem_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_rem_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_rem_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_rem_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_rem_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_rem_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_rem_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_rem_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rem_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheBitAnd => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    a.get_bitand_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_bitand_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_bitand_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_bitand_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_bitand_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_bitand_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_bitand_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_bitand_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_bitand_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_bitand_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_bitand_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_bitand_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u4_bit(b) > 0)
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitand_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheBitOr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    a.get_bitor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_bitor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_bitor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_bitor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_bitor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_bitor_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_bitor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_bitor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_bitor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_bitor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_bitor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_bitor_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_bitor_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitor_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheBitXor => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    a.get_bitxor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_bitxor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_bitxor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_bitxor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_bitxor_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_bitxor_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_bitxor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_bitxor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_bitxor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_bitxor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_bitxor_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_bitxor_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_bitxor_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_bitxor_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheShl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_left_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_left_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_left_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_left_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_left_shift_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_left_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_left_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_left_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_left_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_left_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_left_shift_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_left_shift_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheShr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_right_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_right_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_right_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_right_shift_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_right_shift_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_right_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_right_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_right_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_right_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_right_shift_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_right_shift_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_right_shift_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheRotl => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_rotate_left_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_rotate_left_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_rotate_left_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_rotate_left_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_rotate_left_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_rotate_left_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_rotate_left_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_rotate_left_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_rotate_left_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_rotate_left_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_rotate_left_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_left_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheRotr => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_rotate_right_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_rotate_right_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_rotate_right_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_rotate_right_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_rotate_right_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_rotate_right_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_rotate_right_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_rotate_right_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_rotate_right_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_rotate_right_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_rotate_right_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_rotate_right_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheMin => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_min_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_min_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_min_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_min_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_min_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_min_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_min_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_min_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_min_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_min_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_min_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_min_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheMax => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_max_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_max_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_max_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_max_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_max_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_max_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_max_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_max_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_max_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_max_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_max_size_on_gpu(b),
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_max_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheEq => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    a.get_eq_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_eq_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_eq_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_eq_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_eq_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_eq_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_eq_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_eq_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_eq_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_eq_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_eq_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_eq_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_eq_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_eq_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheNe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    a.get_ne_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_ne_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_ne_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_ne_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_ne_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_ne_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_ne_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_ne_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_ne_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_ne_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_ne_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_ne_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_ne_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u256_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u512_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u1024_bit(b))
                }
                (SupportedFheCiphertexts::FheBytes256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ne_size_on_gpu(to_be_u2048_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheGe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_ge_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_ge_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_ge_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_ge_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_ge_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_ge_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_ge_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_ge_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_ge_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_ge_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_ge_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_ge_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_ge_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheGt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_gt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_gt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_gt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_gt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_gt_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_gt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_gt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_gt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_gt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_gt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_gt_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_gt_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_gt_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheLe => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_le_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_le_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_le_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_le_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_le_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_le_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_le_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_le_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_le_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_le_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_le_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_le_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_le_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheLt => {
            assert_eq!(input_operands.len(), 2);

            match (&input_operands[0], &input_operands[1]) {
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    a.get_lt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    a.get_lt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    a.get_lt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    a.get_lt_size_on_gpu(b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    a.get_lt_size_on_gpu(b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => a.get_lt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => a.get_lt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => a.get_lt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => a.get_lt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => a.get_lt_size_on_gpu(b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => a.get_lt_size_on_gpu(b),
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::Scalar(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    a.get_lt_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u4_bit(b))
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u8_bit(b))
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u16_bit(b))
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u32_bit(b))
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u64_bit(b))
                }
                (SupportedFheCiphertexts::FheUint128(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u128_bit(b))
                }
                (SupportedFheCiphertexts::FheUint160(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u160_bit(b))
                }
                (SupportedFheCiphertexts::FheUint256(a), SupportedFheCiphertexts::Scalar(b)) => {
                    a.get_lt_size_on_gpu(to_be_u256_bit(b))
                }
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheNot => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheBool(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint4(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint8(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint16(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint32(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint64(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint128(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint160(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheUint256(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheBytes64(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheBytes128(a) => a.get_bitnot_size_on_gpu(),
                SupportedFheCiphertexts::FheBytes256(a) => a.get_bitnot_size_on_gpu(),
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheNeg => {
            assert_eq!(input_operands.len(), 1);

            match &input_operands[0] {
                SupportedFheCiphertexts::FheUint4(a) => a.get_neg_size_on_gpu(),
                SupportedFheCiphertexts::FheUint8(a) => a.get_neg_size_on_gpu(),
                SupportedFheCiphertexts::FheUint16(a) => a.get_neg_size_on_gpu(),
                SupportedFheCiphertexts::FheUint32(a) => a.get_neg_size_on_gpu(),
                SupportedFheCiphertexts::FheUint64(a) => a.get_neg_size_on_gpu(),
                SupportedFheCiphertexts::FheUint128(a) => a.get_neg_size_on_gpu(),
                SupportedFheCiphertexts::FheUint160(a) => a.get_neg_size_on_gpu(),
                SupportedFheCiphertexts::FheUint256(a) => a.get_neg_size_on_gpu(),
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheIfThenElse => {
            assert_eq!(input_operands.len(), 3);

            let SupportedFheCiphertexts::FheBool(flag) = &input_operands[0] else {
                return 0;
            };

            match (&input_operands[1], &input_operands[2]) {
                (SupportedFheCiphertexts::FheBool(a), SupportedFheCiphertexts::FheBool(b)) => {
                    let a: FheUint2 = a.to_owned().cast_into();
                    let b: FheUint2 = b.to_owned().cast_into();
                    flag.get_if_then_else_size_on_gpu(&a, &b)
                }
                (SupportedFheCiphertexts::FheUint4(a), SupportedFheCiphertexts::FheUint4(b)) => {
                    flag.get_if_then_else_size_on_gpu(a, b)
                }
                (SupportedFheCiphertexts::FheUint8(a), SupportedFheCiphertexts::FheUint8(b)) => {
                    flag.get_if_then_else_size_on_gpu(a, b)
                }
                (SupportedFheCiphertexts::FheUint16(a), SupportedFheCiphertexts::FheUint16(b)) => {
                    flag.get_if_then_else_size_on_gpu(a, b)
                }
                (SupportedFheCiphertexts::FheUint32(a), SupportedFheCiphertexts::FheUint32(b)) => {
                    flag.get_if_then_else_size_on_gpu(a, b)
                }
                (SupportedFheCiphertexts::FheUint64(a), SupportedFheCiphertexts::FheUint64(b)) => {
                    flag.get_if_then_else_size_on_gpu(a, b)
                }
                (
                    SupportedFheCiphertexts::FheUint128(a),
                    SupportedFheCiphertexts::FheUint128(b),
                ) => flag.get_if_then_else_size_on_gpu(a, b),
                (
                    SupportedFheCiphertexts::FheUint160(a),
                    SupportedFheCiphertexts::FheUint160(b),
                ) => flag.get_if_then_else_size_on_gpu(a, b),
                (
                    SupportedFheCiphertexts::FheUint256(a),
                    SupportedFheCiphertexts::FheUint256(b),
                ) => flag.get_if_then_else_size_on_gpu(a, b),
                (
                    SupportedFheCiphertexts::FheBytes64(a),
                    SupportedFheCiphertexts::FheBytes64(b),
                ) => flag.get_if_then_else_size_on_gpu(a, b),
                (
                    SupportedFheCiphertexts::FheBytes128(a),
                    SupportedFheCiphertexts::FheBytes128(b),
                ) => flag.get_if_then_else_size_on_gpu(a, b),
                (
                    SupportedFheCiphertexts::FheBytes256(a),
                    SupportedFheCiphertexts::FheBytes256(b),
                ) => flag.get_if_then_else_size_on_gpu(a, b),
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheTrivialEncrypt | SupportedFheOperations::FheCast => {
            match (&input_operands[0], &input_operands[1]) {
                (_, SupportedFheCiphertexts::Scalar(op)) => {
                    trivial_encrypt_be_bytes(to_be_u16_bit(op) as i16, &[1u8]).get_size_on_gpu()
                }
                (_, _) => panic!(),
            }
        }
        SupportedFheOperations::FheRand => {
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[1] else {
                return 0;
            };
            let to_type = to_be_u16_bit(to_type) as i16;
            match to_type {
                0 => tfhe::FheUint2::get_generate_oblivious_pseudo_random_size_on_gpu(),
                1 => tfhe::FheUint4::get_generate_oblivious_pseudo_random_size_on_gpu(),
                2 => tfhe::FheUint8::get_generate_oblivious_pseudo_random_size_on_gpu(),
                3 => tfhe::FheUint16::get_generate_oblivious_pseudo_random_size_on_gpu(),
                4 => tfhe::FheUint32::get_generate_oblivious_pseudo_random_size_on_gpu(),
                5 => tfhe::FheUint64::get_generate_oblivious_pseudo_random_size_on_gpu(),
                6 => tfhe::FheUint128::get_generate_oblivious_pseudo_random_size_on_gpu(),
                7 => tfhe::FheUint160::get_generate_oblivious_pseudo_random_size_on_gpu(),
                8 => tfhe::FheUint256::get_generate_oblivious_pseudo_random_size_on_gpu(),
                9 => tfhe::FheUint512::get_generate_oblivious_pseudo_random_size_on_gpu(),
                10 => tfhe::FheUint1024::get_generate_oblivious_pseudo_random_size_on_gpu(),
                11 => tfhe::FheUint2048::get_generate_oblivious_pseudo_random_size_on_gpu(),
                _ => panic!(),
            }
        }
        SupportedFheOperations::FheRandBounded => {
            let SupportedFheCiphertexts::Scalar(to_type) = &input_operands[2] else {
                return 0;
            };
            let to_type = to_be_u16_bit(to_type) as i16;
            match to_type {
                0 => tfhe::FheUint2::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                1 => tfhe::FheUint4::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                2 => tfhe::FheUint8::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                3 => tfhe::FheUint16::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                4 => tfhe::FheUint32::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                5 => tfhe::FheUint64::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                6 => tfhe::FheUint128::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                7 => tfhe::FheUint160::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                8 => tfhe::FheUint256::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                9 => tfhe::FheUint512::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                10 => tfhe::FheUint1024::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                11 => tfhe::FheUint2048::get_generate_oblivious_pseudo_random_bounded_size_on_gpu(),
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}
