use allocative::Allocative;
use ark_std::cmp::Ordering;
use ark_std::vec::Vec;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Compact signed big-integer parameterized by limb count `N` (total width = `N*64 + 32` bits).
///
/// Representation (sign-magnitude):
/// - `magnitude_lo: [u64; N]` holds the low limbs in little-endian order (index 0 is least significant).
/// - `magnitude_hi: u32` holds the high 32-bit tail of the magnitude.
/// - `is_positive: bool` is the sign flag. The magnitude stores the absolute value.
///
/// Arithmetic semantics:
/// - Addition, subtraction, and multiplication operate on magnitudes modulo `2^(64*N + 32)`
///   and then set the sign via standard sign rules.
/// - Zero is not normalized: a zero magnitude can be paired with either sign. Equality is structural,
///   so `+0 != -0`. Callers that require canonical zero should normalize externally.
///
/// Notes:
/// - Specialized fast paths exist for `N ∈ {0,1,2}`; larger `N` uses a generic path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Allocative)]
pub struct SignedBigIntHi32<const N: usize> {
    /// Little-endian low limbs: limb 0 = low 64 bits, limb 1 = next 64 bits, and so on
    magnitude_lo: [u64; N],
    /// Top 32 bits
    magnitude_hi: u32,
    /// Whether the value is non-negative
    is_positive: bool,
}

pub type S96 = SignedBigIntHi32<1>;
pub type S160 = SignedBigIntHi32<2>;
pub type S224 = SignedBigIntHi32<3>;

// ------------------------------------------------------------------------------------------------
// Implementation
// ------------------------------------------------------------------------------------------------

impl<const N: usize> SignedBigIntHi32<N> {
    /// Creates a new `SignedBigIntHi32`.
    ///
    /// The sign is not normalized: a zero magnitude can be positive or negative.
    pub const fn new(magnitude_lo: [u64; N], magnitude_hi: u32, is_positive: bool) -> Self {
        Self {
            magnitude_lo,
            magnitude_hi,
            is_positive,
        }
    }

    /// Returns the value `0`.
    pub const fn zero() -> Self {
        Self {
            magnitude_lo: [0; N],
            magnitude_hi: 0,
            is_positive: true,
        }
    }

    /// Returns the value `1`.
    pub fn one() -> Self {
        let mut magnitude_lo = [0; N];
        let magnitude_hi;

        if N == 0 {
            magnitude_hi = 1;
        } else {
            magnitude_lo[0] = 1;
            magnitude_hi = 0;
        }

        Self {
            magnitude_lo,
            magnitude_hi,
            is_positive: true,
        }
    }

    // ------------------------------------------------------------------------------------------------
    // Accessors
    // ------------------------------------------------------------------------------------------------

    /// Returns the low limbs of the magnitude.
    pub const fn magnitude_lo(&self) -> &[u64; N] {
        &self.magnitude_lo
    }

    /// Returns the high 32 bits of the magnitude.
    pub const fn magnitude_hi(&self) -> u32 {
        self.magnitude_hi
    }

    /// Returns the sign flag (`true` for a positive sign).
    /// Note: zero is not canonicalized; a zero magnitude can have either sign.
    pub const fn is_positive(&self) -> bool {
        self.is_positive
    }

    /// Returns `true` if the number is zero.
    pub const fn is_zero(&self) -> bool {
        let mut lo_is_zero = true;
        let mut i = 0;
        while i < N {
            if self.magnitude_lo[i] != 0 {
                lo_is_zero = false;
                break;
            }
            i += 1;
        }
        self.magnitude_hi == 0 && lo_is_zero
    }

    // ------------------------------------------------------------------------------------------------
    // Private arithmetic helpers
    // ------------------------------------------------------------------------------------------------

    fn compare_magnitudes(&self, other: &Self) -> Ordering {
        if self.magnitude_hi != other.magnitude_hi {
            return self.magnitude_hi.cmp(&other.magnitude_hi);
        }
        for i in (0..N).rev() {
            if self.magnitude_lo[i] != other.magnitude_lo[i] {
                return self.magnitude_lo[i].cmp(&other.magnitude_lo[i]);
            }
        }
        Ordering::Equal
    }

    fn add_assign_in_place(&mut self, rhs: &Self) {
        if self.is_positive == rhs.is_positive {
            let (lo, hi, _carry) = self.add_magnitudes_with_carry(rhs);
            self.magnitude_lo = lo;
            self.magnitude_hi = hi;
        } else {
            match self.compare_magnitudes(rhs) {
                Ordering::Greater | Ordering::Equal => {
                    let (lo, hi, _borrow) = self.sub_magnitudes_with_borrow(rhs);
                    self.magnitude_lo = lo;
                    self.magnitude_hi = hi;
                },
                Ordering::Less => {
                    let (lo, hi, _borrow) = rhs.sub_magnitudes_with_borrow(self);
                    self.magnitude_lo = lo;
                    self.magnitude_hi = hi;
                    self.is_positive = rhs.is_positive;
                },
            }
        }
    }

    fn sub_assign_in_place(&mut self, rhs: &Self) {
        let neg_rhs = -*rhs;
        self.add_assign_in_place(&neg_rhs);
    }

    fn mul_magnitudes(&self, other: &Self) -> ([u64; N], u32) {
        // Fast paths for small N to avoid heap allocation and loops
        if N == 0 {
            let a2 = self.magnitude_hi as u64;
            let b2 = other.magnitude_hi as u64;
            let prod = a2.wrapping_mul(b2);
            let hi = (prod & 0xFFFF_FFFF) as u32;
            let lo: [u64; N] = [0u64; N];
            return (lo, hi);
        }

        if N == 1 {
            let a0 = self.magnitude_lo[0];
            let a1 = self.magnitude_hi as u64; // 32-bit value widened
            let b0 = other.magnitude_lo[0];
            let b1 = other.magnitude_hi as u64; // 32-bit value widened

            let t0 = (a0 as u128) * (b0 as u128);
            let lo0 = t0 as u64;

            let cross = (t0 >> 64) + (a0 as u128) * (b1 as u128) + (a1 as u128) * (b0 as u128);

            let hi = (cross as u64 & 0xFFFF_FFFF) as u32;
            let mut lo = [0u64; N];
            lo[0] = lo0;
            return (lo, hi);
        }

        if N == 2 {
            let a0 = self.magnitude_lo[0];
            let a1 = self.magnitude_lo[1];
            let a2 = self.magnitude_hi as u64; // 32-bit value widened
            let b0 = other.magnitude_lo[0];
            let b1 = other.magnitude_lo[1];
            let b2 = other.magnitude_hi as u64; // 32-bit value widened

            // word 0
            let t0 = (a0 as u128) * (b0 as u128);
            let r0 = t0 as u64;
            let carry0 = t0 >> 64;

            // word 1
            let sum1 = carry0 + (a0 as u128) * (b1 as u128) + (a1 as u128) * (b0 as u128);
            let r1 = sum1 as u64;
            let carry1 = sum1 >> 64;

            // word 2 (only need low 32 bits)
            let sum2 = carry1
                + (a0 as u128) * (b2 as u128)
                + (a1 as u128) * (b1 as u128)
                + (a2 as u128) * (b0 as u128);
            let r2 = sum2 as u64;
            let hi = (r2 & 0xFFFF_FFFF) as u32;
            let mut lo = [0u64; N];
            lo[0] = r0;
            lo[1] = r1;
            return (lo, hi);
        }

        // General path
        // Product of (N*64 + 32)-bit numbers fits in (2*N*64 + 64) bits.
        // Allocate 2*N + 2 u64 limbs to safely propagate carries; we'll truncate to N u64 + 32 bits.
        let mut prod = vec![0u64; 2 * N + 2];

        let self_limbs: Vec<u64> = self
            .magnitude_lo
            .iter()
            .cloned()
            .chain(core::iter::once(self.magnitude_hi as u64))
            .collect();

        let other_limbs: Vec<u64> = other
            .magnitude_lo
            .iter()
            .cloned()
            .chain(core::iter::once(other.magnitude_hi as u64))
            .collect();

        for i in 0..self_limbs.len() {
            let mut carry: u128 = 0;
            for j in 0..other_limbs.len() {
                let idx = i + j;
                let p = (self_limbs[i] as u128) * (other_limbs[j] as u128)
                    + (prod[idx] as u128)
                    + carry;
                prod[idx] = p as u64;
                carry = p >> 64;
            }
            if carry > 0 {
                let spill = i + other_limbs.len();
                if spill < prod.len() {
                    prod[spill] = prod[spill].wrapping_add(carry as u64);
                }
                // else: spill is beyond the truncated width; ignore (mod 2^(64*N+32)).
            }
        }

        // Truncate and split into lo and hi (keep only the low N u64 limbs and the low 32 bits of limb N)
        let mut magnitude_lo = [0u64; N];
        if N > 0 {
            magnitude_lo.copy_from_slice(&prod[0..N]);
        }
        let magnitude_hi = (prod[N] & 0xFFFF_FFFF) as u32;

        (magnitude_lo, magnitude_hi)
    }

    // Returns final carry bit.
    fn add_magnitudes_with_carry(&self, other: &Self) -> ([u64; N], u32, bool) {
        let mut magnitude_lo = [0; N];
        let mut carry: u128 = 0;

        for i in 0..N {
            let sum = (self.magnitude_lo[i] as u128) + (other.magnitude_lo[i] as u128) + carry;
            magnitude_lo[i] = sum as u64;
            carry = sum >> 64;
        }

        let sum_hi = (self.magnitude_hi as u128) + (other.magnitude_hi as u128) + carry;
        let magnitude_hi = sum_hi as u32;

        let final_carry = (sum_hi >> 32) != 0;
        (magnitude_lo, magnitude_hi, final_carry)
    }

    // Returns final borrow bit.
    fn sub_magnitudes_with_borrow(&self, other: &Self) -> ([u64; N], u32, bool) {
        let mut magnitude_lo = [0u64; N];
        let mut borrow = false;

        for i in 0..N {
            let (d1, b1) = self.magnitude_lo[i].overflowing_sub(other.magnitude_lo[i]);
            let (d2, b2) = d1.overflowing_sub(borrow as u64);
            magnitude_lo[i] = d2;
            borrow = b1 || b2;
        }

        let (hi1, b1) = self.magnitude_hi.overflowing_sub(other.magnitude_hi);
        let (hi2, b2) = hi1.overflowing_sub(borrow as u32);
        let final_borrow = b1 || b2;

        (magnitude_lo, hi2, final_borrow)
    }
}

// ------------------------------------------------------------------------------------------------
// Operator traits
// ------------------------------------------------------------------------------------------------

impl<const N: usize> Neg for SignedBigIntHi32<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.magnitude_lo, self.magnitude_hi, !self.is_positive)
    }
}

impl<const N: usize> Add for SignedBigIntHi32<N> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign_in_place(&rhs);
        self
    }
}

impl<const N: usize> AddAssign for SignedBigIntHi32<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign_in_place(&rhs);
    }
}

impl<const N: usize> Sub for SignedBigIntHi32<N> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.sub_assign_in_place(&rhs);
        self
    }
}

impl<const N: usize> SubAssign for SignedBigIntHi32<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign_in_place(&rhs);
    }
}

impl<const N: usize> MulAssign for SignedBigIntHi32<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(&rhs);
    }
}

// Reference variants for efficiency
impl<const N: usize> Add<&SignedBigIntHi32<N>> for SignedBigIntHi32<N> {
    type Output = SignedBigIntHi32<N>;

    #[inline]
    fn add(mut self, rhs: &SignedBigIntHi32<N>) -> Self::Output {
        self.add_assign_in_place(rhs);
        self
    }
}

impl<const N: usize> Sub<&SignedBigIntHi32<N>> for SignedBigIntHi32<N> {
    type Output = SignedBigIntHi32<N>;

    #[inline]
    fn sub(mut self, rhs: &SignedBigIntHi32<N>) -> Self::Output {
        self.sub_assign_in_place(rhs);
        self
    }
}

impl<const N: usize> Mul<&SignedBigIntHi32<N>> for SignedBigIntHi32<N> {
    type Output = SignedBigIntHi32<N>;

    #[inline]
    fn mul(self, rhs: &SignedBigIntHi32<N>) -> Self::Output {
        let (lo, hi) = self.mul_magnitudes(rhs);
        let is_positive = !(self.is_positive ^ rhs.is_positive);
        Self::new(lo, hi, is_positive)
    }
}

impl<const N: usize> AddAssign<&SignedBigIntHi32<N>> for SignedBigIntHi32<N> {
    #[inline]
    fn add_assign(&mut self, rhs: &SignedBigIntHi32<N>) {
        self.add_assign_in_place(rhs);
    }
}

impl<const N: usize> SubAssign<&SignedBigIntHi32<N>> for SignedBigIntHi32<N> {
    #[inline]
    fn sub_assign(&mut self, rhs: &SignedBigIntHi32<N>) {
        self.sub_assign_in_place(rhs);
    }
}

impl<const N: usize> MulAssign<&SignedBigIntHi32<N>> for SignedBigIntHi32<N> {
    #[inline]
    fn mul_assign(&mut self, rhs: &SignedBigIntHi32<N>) {
        *self = self.mul(rhs);
    }
}

// By-ref binary operator variants to avoid copying both operands
impl<'a, const N: usize> Add for &'a SignedBigIntHi32<N> {
    type Output = SignedBigIntHi32<N>;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = *self;
        out.add_assign_in_place(rhs);
        out
    }
}

impl<'a, const N: usize> Sub for &'a SignedBigIntHi32<N> {
    type Output = SignedBigIntHi32<N>;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut out = *self;
        out.sub_assign_in_place(rhs);
        out
    }
}

impl<'a, const N: usize> Mul for &'a SignedBigIntHi32<N> {
    type Output = SignedBigIntHi32<N>;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let (lo, hi) = self.mul_magnitudes(rhs);
        let is_positive = !(self.is_positive ^ rhs.is_positive);
        SignedBigIntHi32::new(lo, hi, is_positive)
    }
}

// ------------------------------------------------------------------------------------------------
// Symmetric mul: S160 * I8OrI96 -> S224 (for ergonomics)
// ------------------------------------------------------------------------------------------------

impl core::ops::Mul<crate::biginteger::I8OrI96> for S160 {
    type Output = S224;
    #[inline]
    fn mul(self, rhs: crate::biginteger::I8OrI96) -> Self::Output {
        rhs * self
    }
}

impl core::ops::Mul<&crate::biginteger::I8OrI96> for S160 {
    type Output = S224;
    #[inline]
    fn mul(self, rhs: &crate::biginteger::I8OrI96) -> Self::Output {
        (*rhs) * self
    }
}

impl core::ops::Mul<crate::biginteger::I8OrI96> for &S160 {
    type Output = S224;
    #[inline]
    fn mul(self, rhs: crate::biginteger::I8OrI96) -> Self::Output {
        rhs * *self
    }
}

impl core::ops::Mul<&crate::biginteger::I8OrI96> for &S160 {
    type Output = S224;
    #[inline]
    fn mul(self, rhs: &crate::biginteger::I8OrI96) -> Self::Output {
        (*rhs) * *self
    }
}

// ------------------------------------------------------------------------------------------------
// From traits
// ------------------------------------------------------------------------------------------------

impl From<i64> for S96 {
    fn from(val: i64) -> Self {
        Self::new([val.unsigned_abs()], 0, val.is_positive())
    }
}

impl From<u64> for S96 {
    fn from(val: u64) -> Self {
        Self::new([val], 0, true)
    }
}

impl From<i128> for S160 {
    fn from(val: i128) -> Self {
        let is_positive = val.is_positive();
        let mag = val.unsigned_abs();
        let lo = mag as u64;
        let hi = (mag >> 64) as u64;
        Self::new([lo, hi], 0, is_positive)
    }
}

impl From<u128> for S160 {
    fn from(val: u128) -> Self {
        let lo = val as u64;
        let hi = (val >> 64) as u64;
        Self::new([lo, hi], 0, true)
    }
}

impl From<S224> for crate::biginteger::BigInt<4> {
    #[inline]
    fn from(val: S224) -> Self {
        let lo = val.magnitude_lo();
        let hi = val.magnitude_hi() as u64;
        crate::biginteger::BigInt::<4>([lo[0], lo[1], lo[2], hi])
    }
}
