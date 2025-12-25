use std::ops::{BitAnd, Mul, ShrAssign};

/// Generic binary exponentiation for integral bases.
/// Works with signed/unsigned integer base types.
///
/// - `base`: any Copy type supporting `*`
/// - `exp`: typically `u32/u64/usize` etc.
/// - returns: base^exp (no modulo)
#[inline]
pub fn binpow<T, E>(mut base: T, mut exp: E) -> T
where
    T: Copy + Mul<Output = T> + From<u8>,
    E: Copy + PartialEq + From<u8> + BitAnd<Output = E> + ShrAssign<u8>,
{
    let mut res = T::from(1u8);
    let one = E::from(1u8);
    let zero = E::from(0u8);

    while exp != zero {
        if (exp & one) != zero {
            res = res * base;
        }
        base = base * base;
        exp >>= 1u8;
    }
    res
}

