use ct_uint::Uint;
use subtle::ConditionallySelectable;

#[inline(always)]
pub fn conditional_sub_p<const N: usize>(v: Uint<N>, p: &Uint<N>) -> Uint<N> {
    let (diff, underflow) = v.checked_sub(p);
    // underflow == 1 means v < p, keep v; otherwise take diff.
    Uint::conditional_select(&diff, &v, underflow)
}
