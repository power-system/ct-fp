// tests/toy_field.rs
use ct_fp::PrimeField;
use ct_uint::Uint;
use subtle::{Choice, ConditionallySelectable, ConstantTimeEq};

/// Toy field: p = 2^64 - 59 (prime), single limb.
#[derive(Clone, Copy, Debug)]
struct Toy(u64);

impl ConstantTimeEq for Toy {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl ConditionallySelectable for Toy {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Toy(u64::conditional_select(&a.0, &b.0, choice))
    }
}

const TOY_P: u64 = 0xFFFF_FFFF_FFFF_FFC5; // 2^64 - 59

impl PrimeField<1> for Toy {
    const MODULUS: Uint<1> = Uint::from_u64(TOY_P);

    fn mul(&self, rhs: &Self) -> Self {
        // Test-only: `%` is not constant-time, but correctness is what we check.
        let r = ((self.0 as u128) * (rhs.0 as u128)) % (TOY_P as u128);
        Toy(r as u64)
    }

    fn from_uint(v: Uint<1>) -> Self {
        Toy(v.to_u64().unwrap())
    }

    fn to_uint(&self) -> Uint<1> {
        Uint::from_u64(self.0)
    }
}

#[test]
fn add_sub_neg() {
    let a = Toy::from_uint(Uint::from_u64(12345));
    let b = Toy::from_uint(Uint::from_u64(67890));
    let sum = a.add(&b);
    assert_eq!(sum.to_uint().to_u64().unwrap(), (12345 + 67890) % TOY_P);

    let diff = sum.sub(&b);
    assert!(bool::from(diff.ct_eq(&a)));

    let neg_a = a.neg();
    assert!(bool::from(a.add(&neg_a).ct_is_zero()));
}

#[test]
fn mul_square() {
    let a = Toy::from_uint(Uint::from_u64(1234567));
    let b = Toy::from_uint(Uint::from_u64(7654321));
    let expected = ((1234567u128 * 7654321u128) % (TOY_P as u128)) as u64;
    assert_eq!(a.mul(&b).to_uint().to_u64().unwrap(), expected);
    assert_eq!(
        a.square().to_uint().to_u64().unwrap(),
        ((1234567u128 * 1234567u128) % (TOY_P as u128)) as u64
    );
}

#[test]
fn invert_roundtrip() {
    for v in [1u64, 2, 3, 7, 42, 12345, TOY_P - 1] {
        let a = Toy::from_uint(Uint::from_u64(v));
        let inv = a.invert();
        assert!(bool::from(a.mul(&inv).ct_eq(&Toy::one())), "v = {v}");
    }
}

#[test]
fn pow_matches_naive() {
    let a = Toy::from_uint(Uint::from_u64(3));
    let e: u64 = 20;
    let mut expected: u128 = 1;
    for _ in 0..e {
        expected = (expected * 3) % (TOY_P as u128);
    }
    let got = a.pow(&e.to_le_bytes());
    assert_eq!(got.to_uint().to_u64().unwrap(), expected as u64);
}
