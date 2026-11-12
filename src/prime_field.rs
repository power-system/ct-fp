use ct_uint::Uint;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

use crate::helpers::conditional_sub_p;

pub trait PrimeField<const LIMBS: usize>:
    Copy + Clone + ConstantTimeEq + ConditionallySelectable + Sized
{
    const MODULUS: Uint<LIMBS>;

    fn mul(&self, rhs: &Self) -> Self;

    fn from_uint(v: Uint<LIMBS>) -> Self;

    fn to_uint(&self) -> Uint<LIMBS>;

    fn zero() -> Self {
        Self::from_uint(Uint::ZERO)
    }

    fn one() -> Self {
        Self::from_uint(Uint::ONE)
    }

    fn add(&self, rhs: &Self) -> Self {
        let (sum, _carry) = self.to_uint().checked_add(&rhs.to_uint());
        Self::from_uint(conditional_sub_p(sum, &Self::MODULUS))
    }

    fn sub(&self, rhs: &Self) -> Self {
        let (diff, underflow) = self.to_uint().checked_sub(&rhs.to_uint());
        let corrected =
            Uint::conditional_select(&diff, &diff.wrapping_add(&Self::MODULUS), underflow);
        Self::from_uint(corrected)
    }

    fn neg(&self) -> Self {
        Self::zero().sub(self)
    }

    fn square(&self) -> Self {
        self.mul(self)
    }

    fn pow(&self, exponent_le_bytes: &[u8]) -> Self {
        crate::pow::pow(self, exponent_le_bytes)
    }

    fn invert(&self) -> Self {
        assert!(LIMBS <= 8, "invert supports up to 8 limbs (512 bits)");
        let p_minus_2 = Self::MODULUS.wrapping_sub(&Uint::from_u64(2));
        let n = LIMBS * 8;
        let mut bytes = [0u8; 64];
        p_minus_2.write_le_bytes(&mut bytes[..n]);
        self.pow(&bytes[..n])
    }

    fn ct_is_zero(&self) -> Choice {
        self.to_uint().ct_is_zero()
    }
}
