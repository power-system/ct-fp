use subtle::{Choice, ConditionallySelectable};

use crate::PrimeField;

#[inline(never)]
pub fn pow<F: PrimeField<N>, const N: usize>(base: &F, exponent_le_bytes: &[u8]) -> F {
    let mut acc = F::one();
    for &byte in exponent_le_bytes.iter().rev() {
        for bit_idx in (0..8).rev() {
            acc = acc.square();
            let bit = Choice::from((byte >> bit_idx) & 1);
            let prod = acc.mul(base);
            acc = F::conditional_select(&acc, &prod, bit);
        }
    }
    acc
}
