use aligned_vec::ABox;
use anyhow::anyhow;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use tfhe::named::Named;
use tfhe::Versionize;

use std::panic::Location;

use tfhe::boolean::prelude::PolynomialSize;
use tfhe::core_crypto::prelude::{
    allocate_and_trivially_encrypt_new_glwe_ciphertext, keyswitch_lwe_ciphertext,
    lwe_ciphertext_cleartext_mul_assign, programmable_bootstrap_f128_lwe_ciphertext,
    CiphertextModulus, Cleartext, Container, ContainerMut, Fourier128LweBootstrapKey,
    GlweCiphertextOwned, GlweSize, LweCiphertext, LweCiphertextOwned, LweKeyswitchKey,
    LweSecretKeyOwned, LweSize, PlaintextList, UnsignedInteger, UnsignedTorus,
};

use tfhe::{
    core_crypto::commons::traits::{CastFrom, CastInto},
    integer::IntegerCiphertext,
    shortint::PBSOrder,
};

use tfhe::integer::ciphertext::BaseRadixCiphertext;
use tfhe::shortint::ClassicPBSParameters;
use tfhe_versionable::VersionsDispatch;

#[cfg(feature = "test_decrypt_128")]
use {
    num_traits::{AsPrimitive, ConstZero},
    std::num::Wrapping,
    tfhe::core_crypto::prelude::decrypt_lwe_ciphertext,
    tfhe::integer::block_decomposition::BlockRecomposer,
};

#[cfg(feature = "test_decrypt_128")]
pub type Z128 = Wrapping<u128>;

pub type Ciphertext64 = BaseRadixCiphertext<tfhe::shortint::Ciphertext>;
pub type Ciphertext64Block = tfhe::shortint::Ciphertext;

#[derive(VersionsDispatch)]
pub enum Ciphertext128Versioned {
    V0(Ciphertext128),
}

// Observe that tfhe-rs is hard-coded to use u64, hence we require custom types for the 128 bit versions for now.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Versionize)]
#[versionize(Ciphertext128Versioned)]
pub struct Ciphertext128 {
    pub inner: Vec<Ciphertext128Block>,
}

impl Named for Ciphertext128 {
    const NAME: &'static str = "Ciphertext128";
}

impl Ciphertext128 {
    pub fn new(inner: Vec<Ciphertext128Block>) -> Self {
        Self { inner }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

pub type Ciphertext128Block = LweCiphertextOwned<u128>;

// NOTE: the below is copied from core/threshold
// since the calling tracing from another crate
// does not generate correct logs in tracing_test::traced_test
#[track_caller]
pub(crate) fn anyhow_error_and_log<S: AsRef<str> + fmt::Display>(msg: S) -> anyhow::Error {
    println!("Error in {}: {}", Location::caller(), msg);
    anyhow!("Error in {}: {}", Location::caller(), msg)
}

/// Key used for switch-and-squash to convert a ciphertext over u64 to one over
/// u128
#[derive(Serialize, Deserialize, Clone, PartialEq, VersionsDispatch)]
pub enum SwitchAndSquashKeyVersioned {
    V0(SwitchAndSquashKey),
}

/// Key used for switch-and-squash to convert a ciphertext over u64 to one over
/// u128
#[derive(Serialize, Deserialize, Clone, PartialEq, Versionize)]
#[versionize(SwitchAndSquashKeyVersioned)]
pub struct SwitchAndSquashKey {
    pub fbsk_out: Fourier128LweBootstrapKey<ABox<[f64]>>,
    //ksk is needed if PBSOrder is KS-PBS
    pub ksk: LweKeyswitchKey<Vec<u64>>,
}

impl Named for SwitchAndSquashKey {
    const NAME: &'static str = "SwitchAndSquashKey";
}

pub trait AugmentedCiphertextParameters {
    // Return the minimum amount of bits that can be used for a message in each
    // block.
    fn message_modulus_log(&self) -> u32;

    // Return the minimum amount of bits that can be used for a carry in each block.
    fn carry_modulus_log(&self) -> u32;
    // Return the minimum total amounts of availble bits in each block. I.e.
    // including both message and carry bits
    fn total_block_bits(&self) -> u32;
}

impl AugmentedCiphertextParameters for tfhe::shortint::Ciphertext {
    // Return the minimum amount of bits that can be used for a message in each
    // block.
    fn message_modulus_log(&self) -> u32 {
        self.message_modulus.0.ilog2()
    }

    // Return the minimum amount of bits that can be used for a carry in each block.
    fn carry_modulus_log(&self) -> u32 {
        self.carry_modulus.0.ilog2()
    }

    // Return the minimum total amounts of availble bits in each block. I.e.
    // including both message and carry bits
    fn total_block_bits(&self) -> u32 {
        self.carry_modulus_log() + self.message_modulus_log()
    }
}

impl SwitchAndSquashKey {
    pub fn new(
        fbsk_out: Fourier128LweBootstrapKey<ABox<[f64]>>,
        ksk: LweKeyswitchKey<Vec<u64>>,
    ) -> Self {
        SwitchAndSquashKey { fbsk_out, ksk }
    }

    /// Converts a ciphertext over a 64 bit domain to a ciphertext over a 128
    /// bit domain (which is needed for secure threshold decryption).
    /// Conversion is done using a precreated conversion key [conversion_key].
    /// Observe that the decryption key will be different after conversion,
    /// since [conversion_key] is actually a key-switching key. This version
    /// computes SnS on all blocks in parallel.
    pub fn to_large_ciphertext(
        &self,
        raw_small_ct: &Ciphertext64,
    ) -> anyhow::Result<Ciphertext128> {
        let blocks = raw_small_ct.blocks();
        // do switch and squash on all blocks in parallel
        let inner = blocks
            .par_iter()
            .map(|current_block| self.to_large_ciphertext_block(current_block))
            .collect::<anyhow::Result<Vec<Ciphertext128Block>>>()?;
        Ok(Ciphertext128 { inner })
    }

    /// Converts a single ciphertext block over a 64 bit domain to a ciphertext
    /// block over a 128 bit domain (which is needed for secure threshold
    /// decryption). Conversion is done using a precreated conversion key,
    /// [conversion_key]. Observe that the decryption key will be different
    /// after conversion, since [conversion_key] is actually a key-switching
    /// key.
    pub fn to_large_ciphertext_block(
        &self,
        small_ct_block: &Ciphertext64Block,
    ) -> anyhow::Result<Ciphertext128Block> {
        let total_bits = small_ct_block.total_block_bits();

        // Accumulator definition
        let delta = 1_u64 << (u64::BITS - 1 - total_bits);
        let msg_modulus = 1_u64 << total_bits;

        let f_out = |x: u128| x;
        let delta_u128 = (delta as u128) << 64;
        let accumulator_out: GlweCiphertextOwned<u128> = Self::generate_accumulator(
            self.fbsk_out.polynomial_size(),
            self.fbsk_out.glwe_size(),
            msg_modulus.cast_into(),
            CiphertextModulus::<u128>::new_native(),
            delta_u128,
            f_out,
        );

        //MSUP
        let mut ms_output_lwe = LweCiphertext::new(
            0_u128,
            self.fbsk_out.input_lwe_dimension().to_lwe_size(),
            CiphertextModulus::new_native(),
        );
        //If ctype = F-GLWE we need to KS before doing the Bootstrap
        if small_ct_block.pbs_order == PBSOrder::KeyswitchBootstrap {
            let mut output_raw_ctxt =
                LweCiphertext::new(0, self.ksk.output_lwe_size(), self.ksk.ciphertext_modulus());
            keyswitch_lwe_ciphertext(&self.ksk, &small_ct_block.ct, &mut output_raw_ctxt);
            Self::lwe_ciphertext_modulus_switch_up(&mut ms_output_lwe, &output_raw_ctxt)?;
        } else {
            Self::lwe_ciphertext_modulus_switch_up(&mut ms_output_lwe, &small_ct_block.ct)?;
        };

        let pbs_cipher_size = LweSize(
            1 + self.fbsk_out.glwe_size().to_glwe_dimension().0 * self.fbsk_out.polynomial_size().0,
        );
        let mut out_pbs_ct = LweCiphertext::new(
            0_u128,
            pbs_cipher_size,
            CiphertextModulus::<u128>::new_native(),
        );
        programmable_bootstrap_f128_lwe_ciphertext(
            &ms_output_lwe,
            &mut out_pbs_ct,
            &accumulator_out,
            &self.fbsk_out,
        );
        Ok(out_pbs_ct)
    }

    // Here we will define a helper function to generate an accumulator for a PBS
    fn generate_accumulator<F, Scalar: UnsignedTorus + CastFrom<usize>>(
        polynomial_size: PolynomialSize,
        glwe_size: GlweSize,
        message_modulus: usize,
        ciphertext_modulus: CiphertextModulus<Scalar>,
        delta: Scalar,
        f: F,
    ) -> GlweCiphertextOwned<Scalar>
    where
        F: Fn(Scalar) -> Scalar,
    {
        // N/(p/2) = size of each block, to correct noise from the input we introduce
        // the notion of box, which manages redundancy to yield a denoised value
        // for several noisy values around a true input value.
        let box_size = polynomial_size.0 / message_modulus;

        // Create the accumulator
        let mut accumulator_scalar = vec![Scalar::ZERO; polynomial_size.0];

        // Fill each box with the encoded denoised value
        for i in 0..message_modulus {
            let index = i * box_size;
            accumulator_scalar[index..index + box_size]
                .iter_mut()
                .for_each(|a| *a = f(Scalar::cast_from(i)) * delta);
        }

        let half_box_size = box_size / 2;

        // Negate the first half_box_size coefficients to manage negacyclicity and
        // rotate
        for a_i in accumulator_scalar[0..half_box_size].iter_mut() {
            *a_i = (*a_i).wrapping_neg();
        }

        // Rotate the accumulator
        accumulator_scalar.rotate_left(half_box_size);

        let accumulator_plaintext = PlaintextList::from_container(accumulator_scalar);

        allocate_and_trivially_encrypt_new_glwe_ciphertext(
            glwe_size,
            &accumulator_plaintext,
            ciphertext_modulus,
        )
    }

    /// The method below is copied from the `noise-gap-exp` branch in
    /// tfhe-rs-internal (and added error handling) since this branch will
    /// likely not be merged in main.
    ///
    /// Takes a ciphertext, `input`, of a certain domain, [InputScalar] and
    /// overwrites the content of `output` with the ciphertext converted to
    /// the [OutputScaler] domain.
    fn lwe_ciphertext_modulus_switch_up<InputScalar, OutputScalar, InputCont, OutputCont>(
        output: &mut LweCiphertext<OutputCont>,
        input: &LweCiphertext<InputCont>,
    ) -> anyhow::Result<()>
    where
        InputScalar: UnsignedInteger + CastInto<OutputScalar>,
        OutputScalar: UnsignedInteger,
        InputCont: Container<Element = InputScalar>,
        OutputCont: ContainerMut<Element = OutputScalar>,
    {
        if !input.ciphertext_modulus().is_native_modulus() {
            return Err(anyhow_error_and_log(
                "Ciphertext modulus is not native, which is the only kind supported",
            ));
        }

        output
            .as_mut()
            .iter_mut()
            .zip(input.as_ref().iter())
            .for_each(|(dst, &src)| *dst = src.cast_into());
        let modulus_up: CiphertextModulus<OutputScalar> = input
            .ciphertext_modulus()
            .try_to()
            .map_err(|_| anyhow_error_and_log("Could not parse ciphertext modulus"))?;

        lwe_ciphertext_cleartext_mul_assign(
            output,
            Cleartext(modulus_up.get_power_of_two_scaling_to_native_torus()),
        );
        Ok(())
    }
}

impl AugmentedCiphertextParameters for ClassicPBSParameters {
    // Return the minimum amount of bits that can be used for a message in each
    // block.
    fn message_modulus_log(&self) -> u32 {
        self.message_modulus.0.ilog2()
    }

    // Return the minimum amount of bits that can be used for a carry in each block.
    fn carry_modulus_log(&self) -> u32 {
        self.carry_modulus.0.ilog2()
    }

    // Return the minimum total amounts of availble bits in each block. I.e.
    // including both message and carry bits
    fn total_block_bits(&self) -> u32 {
        self.carry_modulus_log() + self.message_modulus_log()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SnsClientKey {
    pub key: LweSecretKeyOwned<u128>,
    pub params: ClassicPBSParameters,
}

impl SnsClientKey {
    pub fn new(params: ClassicPBSParameters, sns_secret_key: LweSecretKeyOwned<u128>) -> Self {
        SnsClientKey {
            key: sns_secret_key,
            params,
        }
    }

    #[cfg(feature = "test_decrypt_128")]
    pub fn decrypt_128(&self, ct: &Ciphertext128) -> u128 {
        if ct.is_empty() {
            return 0;
        }

        let bits_in_block = self.params.message_modulus_log();
        let mut recomposer = BlockRecomposer::<u128>::new(bits_in_block);

        for encrypted_block in ct.inner.iter() {
            let decrypted_block = self.decrypt_block_128(encrypted_block);
            if !recomposer.add_unmasked(decrypted_block.0) {
                // End of T::BITS reached no need to try more
                // recomposition
                break;
            };
        }

        recomposer.value()
    }

    #[cfg(feature = "test_decrypt_128")]
    pub(crate) fn decrypt_block_128(&self, ct: &Ciphertext128Block) -> Z128 {
        let total_bits = self.params.total_block_bits() as usize;
        let raw_plaintext = decrypt_lwe_ciphertext(&self.key, ct);
        from_expanded_msg(raw_plaintext.0, total_bits)
    }
}

#[cfg(feature = "test_decrypt_128")]
// Map a raw, decrypted message to its real value by dividing by the appropriate
// shift, delta, assuming padding
pub(crate) fn from_expanded_msg<Scalar: UnsignedInteger + AsPrimitive<u128>>(
    raw_plaintext: Scalar,
    message_and_carry_mod_bits: usize,
) -> Z128 {
    // delta = q/t where t is the amount of plain text bits
    // Observe that t includes the message and carry bits as well as the padding bit
    // (hence the + 1)
    let delta_pad_bits = (Scalar::BITS as u128) - (message_and_carry_mod_bits as u128 + 1_u128);

    // Observe that in certain situations the computation of b-<a,s> may be negative
    // Concretely this happens when the message encrypted is 0 and randomness ends
    // up being negative. We cannot simply do the standard modulo operation
    // then, as this would mean the message becomes 2^message_mod_bits instead
    // of 0 as it should be. However the maximal negative value it can have
    // (without a general decryption error) is delta/2 which we can compute as 1
    // << delta_pad_bits, since the padding already halves the true delta
    if raw_plaintext.as_() > Scalar::MAX.as_() - (1 << delta_pad_bits) {
        Z128::ZERO
    } else {
        // compute delta / 2
        let delta_pad_half = 1 << (delta_pad_bits - 1);

        // add delta/2 to kill the negative noise, note this does not affect the
        // message. and then divide by delta
        let raw_msg = raw_plaintext.as_().wrapping_add(delta_pad_half) >> delta_pad_bits;
        std::num::Wrapping(raw_msg % (1 << message_and_carry_mod_bits))
    }
}
