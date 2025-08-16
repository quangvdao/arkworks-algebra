#![allow(clippy::bool_assert_comparison)]
use crate::{biginteger::BigInteger, UniformRand};
use num_bigint::BigUint;

// Test elementary math operations for BigInteger.
#[allow(clippy::eq_op)]
fn biginteger_arithmetic_test<B: BigInteger>(a: B, b: B, zero: B, max: B) {
    // zero == zero
    assert_eq!(zero, zero);

    // zero.is_zero() == true
    assert!(zero.is_zero());

    // a == a
    assert_eq!(a, a);

    // a + 0 = a
    let mut a0_add = a;
    let carry = a0_add.add_with_carry(&zero);
    assert_eq!(a0_add, a);
    assert_eq!(carry, false);

    // a - 0 = a
    let mut a0_sub = a;
    let borrow = a0_sub.sub_with_borrow(&zero);
    assert_eq!(a0_sub, a);
    assert_eq!(borrow, false);

    // a - a = 0
    let mut aa_sub = a;
    let borrow = aa_sub.sub_with_borrow(&a);
    assert_eq!(aa_sub, zero);
    assert_eq!(borrow, false);

    // a + b = b + a
    let mut ab_add = a;
    let ab_carry = ab_add.add_with_carry(&b);
    let mut ba_add = b;
    let ba_carry = ba_add.add_with_carry(&a);
    assert_eq!(ab_add, ba_add);
    assert_eq!(ab_carry, ba_carry);

    // a * 1 = a
    let mut a_mul1 = a;
    a_mul1 <<= 0;
    assert_eq!(a_mul1, a);

    // a * 2 = a + a
    let mut a_mul2 = a;
    a_mul2.mul2();
    let mut a_plus_a = a;
    let carry_a_plus_a = a_plus_a.add_with_carry(&a); // Won't assert anything about carry bit.
    assert_eq!(a_mul2, a_plus_a);

    // a * 1 = a
    assert_eq!(a.mul_low(&B::from(1u64)), a);

    // a * 2 = a
    assert_eq!(a.mul_low(&B::from(2u64)), a_plus_a);

    // a * b = b * a
    assert_eq!(a.mul_low(&b), b.mul_low(&a));

    // a * 2 * b * 0 = 0
    assert!(a.mul_low(&zero).is_zero());

    // a * 2 * ... * 2  = a * 2^n
    let mut a_mul_n = a;
    for _ in 0..20 {
        a_mul_n = a_mul_n.mul_low(&B::from(2u64));
    }
    assert_eq!(a_mul_n, a << 20);

    // a * 0 = (0, 0)
    assert_eq!(a.mul(&zero), (zero, zero));

    // a * 1 = (a, 0)
    assert_eq!(a.mul(&B::from(1u64)), (a, zero));

    // a * 1 = 0 (high part of the result)
    assert_eq!(a.mul_high(&B::from(1u64)), (zero));

    // a * 0 = 0 (high part of the result)
    assert!(a.mul_high(&zero).is_zero());

    // If a + a has a carry
    if carry_a_plus_a {
        // a + a has a carry: high part of a * 2 is not zero
        assert_ne!(a.mul_high(&B::from(2u64)), zero);
    } else {
        // a + a has no carry: high part of a * 2 is zero
        assert_eq!(a.mul_high(&B::from(2u64)), zero);
    }

    // max + max = max * 2
    let mut max_plus_max = max;
    max_plus_max.add_with_carry(&max);
    assert_eq!(max.mul(&B::from(2u64)), (max_plus_max, B::from(1u64)));
    assert_eq!(max.mul_high(&B::from(2u64)), B::from(1u64));
}

fn biginteger_shr<B: BigInteger>() {
    let mut rng = ark_std::test_rng();
    let a = B::rand(&mut rng);
    assert_eq!(a >> 0, a);

    // Binary simple test
    let a = B::from(256u64);
    assert_eq!(a >> 2, B::from(64u64));

    // Test saturated underflow
    let a = B::from(1u64);
    assert_eq!(a >> 5, B::from(0u64));

    // Test null bits
    let a = B::rand(&mut rng);
    let b = a >> 3;
    assert_eq!(b.get_bit(B::NUM_LIMBS * 64 - 1), false);
    assert_eq!(b.get_bit(B::NUM_LIMBS * 64 - 2), false);
    assert_eq!(b.get_bit(B::NUM_LIMBS * 64 - 3), false);
}

fn biginteger_shl<B: BigInteger>() {
    let mut rng = ark_std::test_rng();
    let a = B::rand(&mut rng);
    assert_eq!(a << 0, a);

    // Binary simple test
    let a = B::from(64u64);
    assert_eq!(a << 2, B::from(256u64));

    // Testing saturated overflow
    let a = B::rand(&mut rng);
    assert_eq!(a << ((B::NUM_LIMBS as u32) * 64), B::from(0u64));

    // Test null bits
    let a = B::rand(&mut rng);
    let b = a << 3;
    assert_eq!(b.get_bit(0), false);
    assert_eq!(b.get_bit(1), false);
    assert_eq!(b.get_bit(2), false);
}

// Test for BigInt's bitwise operations
fn biginteger_bitwise_ops_test<B: BigInteger>() {
    let mut rng = ark_std::test_rng();

    // Test XOR
    // a xor a = 0
    let a = B::rand(&mut rng);
    assert_eq!(a ^ &a, B::from(0_u64));

    // Testing a xor b xor b
    let a = B::rand(&mut rng);
    let b = B::rand(&mut rng);
    let xor_ab = a ^ b;
    assert_eq!(xor_ab ^ b, a);

    // Test OR
    // a or a = a
    let a = B::rand(&mut rng);
    assert_eq!(a | &a, a);

    // Testing a or b or b
    let a = B::rand(&mut rng);
    let b = B::rand(&mut rng);
    let or_ab = a | b;
    assert_eq!(or_ab | &b, a | b);

    // Test AND
    // a and a = a
    let a = B::rand(&mut rng);
    assert_eq!(a & (&a), a);

    // Testing a and a and b.
    let a = B::rand(&mut rng);
    let b = B::rand(&mut rng);
    let b_clone = b;
    let and_ab = a & b;
    assert_eq!(and_ab & b_clone, a & b);

    // Testing De Morgan's law
    let a = 0x1234567890abcdef_u64;
    let b = 0xfedcba0987654321_u64;
    let de_morgan_lhs = B::from(!(a | b));
    let de_morgan_rhs = B::from(!a) & B::from(!b);
    assert_eq!(de_morgan_lhs, de_morgan_rhs);
}

// Test correctness of BigInteger's bit values
fn biginteger_bits_test<B: BigInteger>() {
    let mut one = B::from(1u64);
    // 0th bit of BigInteger representing 1 is 1
    assert!(one.get_bit(0));
    // 1st bit of BigInteger representing 1 is not 1
    assert!(!one.get_bit(1));
    one <<= 5;
    let thirty_two = one;
    // 0th bit of BigInteger representing 32 is not 1
    assert!(!thirty_two.get_bit(0));
    // 1st bit of BigInteger representing 32 is not 1
    assert!(!thirty_two.get_bit(1));
    // 2nd bit of BigInteger representing 32 is not 1
    assert!(!thirty_two.get_bit(2));
    // 3rd bit of BigInteger representing 32 is not 1
    assert!(!thirty_two.get_bit(3));
    // 4th bit of BigInteger representing 32 is not 1
    assert!(!thirty_two.get_bit(4));
    // 5th bit of BigInteger representing 32 is 1
    assert!(thirty_two.get_bit(5), "{:?}", thirty_two);

    // Generates a random BigInteger and tests bit construction methods.
    let mut rng = ark_std::test_rng();
    let a: B = UniformRand::rand(&mut rng);
    assert_eq!(B::from_bits_be(&a.to_bits_be()), a);
    assert_eq!(B::from_bits_le(&a.to_bits_le()), a);
}

// Test conversion from BigInteger to BigUint
fn biginteger_conversion_test<B: BigInteger>() {
    let mut rng = ark_std::test_rng();

    let x: B = UniformRand::rand(&mut rng);
    let x_bigint: BigUint = x.into();
    let x_recovered = B::try_from(x_bigint).ok().unwrap();

    assert_eq!(x, x_recovered);
}

// Wrapper test function for BigInteger
fn test_biginteger<B: BigInteger>(max: B, zero: B) {
    let mut rng = ark_std::test_rng();
    let a: B = UniformRand::rand(&mut rng);
    let b: B = UniformRand::rand(&mut rng);
    biginteger_arithmetic_test(a, b, zero, max);
    biginteger_bits_test::<B>();
    biginteger_conversion_test::<B>();
    biginteger_bitwise_ops_test::<B>();
    biginteger_shr::<B>();
    biginteger_shl::<B>();
}

#[test]
fn test_biginteger64() {
    use crate::biginteger::BigInteger64 as B;
    test_biginteger(B::new([u64::MAX; 1]), B::new([0u64; 1]));
}

#[test]
fn test_biginteger128() {
    use crate::biginteger::BigInteger128 as B;
    test_biginteger(B::new([u64::MAX; 2]), B::new([0u64; 2]));
}

#[test]
fn test_biginteger256() {
    use crate::biginteger::BigInteger256 as B;
    test_biginteger(B::new([u64::MAX; 4]), B::new([0u64; 4]));
}

#[test]
fn test_biginteger384() {
    use crate::biginteger::BigInteger384 as B;
    test_biginteger(B::new([u64::MAX; 6]), B::new([0u64; 6]));
}

#[test]
fn test_biginteger448() {
    use crate::biginteger::BigInteger448 as B;
    test_biginteger(B::new([u64::MAX; 7]), B::new([0u64; 7]));
}

#[test]
fn test_biginteger768() {
    use crate::biginteger::BigInteger768 as B;
    test_biginteger(B::new([u64::MAX; 12]), B::new([0u64; 12]));
}

#[test]
fn test_biginteger832() {
    use crate::biginteger::BigInteger832 as B;
    test_biginteger(B::new([u64::MAX; 13]), B::new([0u64; 13]));
}

// Tests for NEW functions
use crate::biginteger::BigInteger256;

#[test]
fn test_mul_u64_in_place() {
    let mut a = BigInteger256::from(0x123456789ABCDEFu64);
    let b = 0x987654321u64;
    
    // Test against reference implementation
    let expected = BigUint::from(0x123456789ABCDEFu64) * BigUint::from(b);
    a.mul_u64_in_place(b);
    assert_eq!(BigUint::from(a), expected);
    
    // Test zero multiplication
    let mut zero = BigInteger256::zero();
    zero.mul_u64_in_place(12345);
    assert!(zero.is_zero());
    
    // Test multiplication by zero
    let mut a = BigInteger256::from(12345u64);
    a.mul_u64_in_place(0);
    assert!(a.is_zero());
    
    // Test multiplication by one
    let orig = BigInteger256::from(0xDEADBEEFu64);
    let mut a = orig;
    a.mul_u64_in_place(1);
    assert_eq!(a, orig);
}

#[test]
fn test_mul_u64_w_carry() {
    let a = BigInteger256::from(u64::MAX);
    let b = u64::MAX;
    
    // Test against reference implementation
    let expected = BigUint::from(u64::MAX) * BigUint::from(u64::MAX);
    let result = a.mul_u64_w_carry::<5>(b);
    assert_eq!(BigUint::from(result), expected);
    
    // Test with small numbers
    let a = BigInteger256::from(12345u64);
    let b = 67890u64;
    let expected = BigUint::from(12345u64) * BigUint::from(67890u64);
    let result = a.mul_u64_w_carry::<5>(b);
    assert_eq!(BigUint::from(result), expected);
    
    // Test zero cases
    let zero = BigInteger256::zero();
    let result = zero.mul_u64_w_carry::<5>(12345);
    assert!(result.is_zero());
    
    let a = BigInteger256::from(12345u64);
    let result = a.mul_u64_w_carry::<5>(0);
    assert!(result.is_zero());
    
    // Test multiplication by one
    let a = BigInteger256::from(0xDEADBEEFu64);
    let result = a.mul_u64_w_carry::<5>(1);
    let expected_bytes = a.to_bytes_le();
    let result_bytes = result.to_bytes_le();
    assert_eq!(&result_bytes[..expected_bytes.len()], &expected_bytes[..]);
}

#[test]
fn test_fmu64a() {
    let a = BigInteger256::from(12345u64);
    let b = 67890u64;
    let mut acc = BigInteger256::from(11111u64).mul_u64_w_carry::<5>(1);
    
    // Perform fused multiply-accumulate
    a.fmu64a(b, &mut acc);
    
    // Compare against separate multiply and add
    let expected_mul = BigUint::from(12345u64) * BigUint::from(67890u64);
    let expected_total = expected_mul + BigUint::from(11111u64);
    assert_eq!(BigUint::from(acc), expected_total);
    
    // Test zero cases
    let zero = BigInteger256::zero();
    let mut acc = BigInteger256::from(12345u64).mul_u64_w_carry::<5>(1);
    let acc_copy = acc;
    zero.fmu64a(67890, &mut acc);
    assert_eq!(acc, acc_copy); // Should be unchanged
    
    // Test multiplication by zero
    let a = BigInteger256::from(12345u64);
    let mut acc = BigInteger256::from(11111u64).mul_u64_w_carry::<5>(1);
    let acc_copy = acc;
    a.fmu64a(0, &mut acc);
    assert_eq!(acc, acc_copy); // Should be unchanged
    
    // Test multiplication by one (should be just addition)
    let a = BigInteger256::from(12345u64);
    let mut acc = BigInteger256::from(11111u64).mul_u64_w_carry::<5>(1);
    a.fmu64a(1, &mut acc);
    let expected = BigUint::from(12345u64) + BigUint::from(11111u64);
    assert_eq!(BigUint::from(acc), expected);
}

#[test]
fn test_mul_u128_w_carry() {
    let a = BigInteger256::from(0x123456789ABCDEFu64);
    let b = 0x987654321DEADBEEFu128;
    
    // Test against reference implementation
    let expected = BigUint::from(0x123456789ABCDEFu64) * BigUint::from(0x987654321DEADBEEFu128);
    let result = a.mul_u128_w_carry::<5, 6>(b);
    assert_eq!(BigUint::from(result), expected);
    
    // Test with u64 value (should be same as mul_u64_w_carry)
    let b_u64 = 0x987654321u64;
    let result_u128 = a.mul_u128_w_carry::<5, 6>(b_u64 as u128);
    let result_u64 = a.mul_u64_w_carry::<5>(b_u64);
    
    // Compare first 5 limbs (u64 result size)
    for i in 0..5 {
        assert_eq!(result_u128.0[i], result_u64.0[i]);
    }
    assert_eq!(result_u128.0[5], 0); // Extra limb should be zero
    
    // Test zero cases
    let zero = BigInteger256::zero();
    let result = zero.mul_u128_w_carry::<5, 6>(12345);
    assert!(result.is_zero());
    
    let a = BigInteger256::from(12345u64);
    let result = a.mul_u128_w_carry::<5, 6>(0);
    assert!(result.is_zero());
    
    // Test multiplication by one
    let a = BigInteger256::from(0xDEADBEEFu64);
    let result = a.mul_u128_w_carry::<5, 6>(1);
    let expected_bytes = a.to_bytes_le();
    let result_bytes = result.to_bytes_le();
    assert_eq!(&result_bytes[..expected_bytes.len()], &expected_bytes[..]);
}

#[test]
fn test_fm128a_basic_and_edges() {
    use crate::biginteger::BigInteger256 as B;
    // Basic reference check against BigUint
    let a = B::from(0x123456789ABCDEFu64);
    let b = 0x987654321DEADBEEFu128;
    let mut acc = B::zero().mul_u128_w_carry::<5, 6>(1); // zero-extended accumulator (6 limbs)
    a.fm128a::<6>(b, &mut acc);
    let expected = num_bigint::BigUint::from(0x123456789ABCDEFu64)
        * num_bigint::BigUint::from(0x987654321DEADBEEFu128);
    assert_eq!(num_bigint::BigUint::from(acc), expected);

    // Zero multiplier: no change
    let a = B::from(12345u64);
    let mut acc = B::from(11111u64).mul_u128_w_carry::<5, 6>(1);
    let acc_copy = acc;
    a.fm128a::<6>(0, &mut acc);
    assert_eq!(acc, acc_copy);

    // One multiplier: reduces to addition
    let a = B::from(12345u64);
    let mut acc = B::from(11111u64).mul_u128_w_carry::<5, 6>(1);
    a.fm128a::<6>(1, &mut acc);
    let expected = num_bigint::BigUint::from(12345u64) + num_bigint::BigUint::from(11111u64);
    assert_eq!(num_bigint::BigUint::from(acc), expected);

    // Overflow propagation from limb N into highest limb
    let a = B::new([u64::MAX; 4]);
    let mut acc = B::zero().mul_u128_w_carry::<5, 6>(1);
    // Pre-fill limb N to force overflow when adding the final carry from low pass
    acc.0[4] = u64::MAX; // limb N
    acc.0[5] = 0; // highest limb
    // cause carry=1 from low pass (a * 2)
    a.fm128a::<6>(2, &mut acc);
    // Expect highest limb incremented by 1 due to overflow from limb N
    assert_eq!(acc.0[5], 1);
}

#[test]
fn test_overflow_behavior_fmu64a() {
    // Test that overflow in the highest limb wraps around as documented
    let a = BigInteger256::new([u64::MAX; 4]);
    let mut acc = BigInteger256::new([0, 0, 0, 0]).mul_u64_w_carry::<5>(1);
    acc.0[4] = u64::MAX; // Set highest limb to max
    
    // This should cause overflow in the highest limb
    a.fmu64a(2, &mut acc);
    
    // The overflow should wrap around
    // u64::MAX * 2 = 2^65 - 2, which when added to u64::MAX = 2^65 + u64::MAX - 2
    // This wraps to u64::MAX - 2 with a carry of 1 that itself wraps
    assert_eq!(acc.0[4], u64::MAX.wrapping_add(1)); // Wrapped result
}

#[test]
fn test_edge_cases_large_numbers() {
    // Test with maximum values
    let max_bi = BigInteger256::new([u64::MAX; 4]);
    
    // mul_u64_w_carry with max values
    let result = max_bi.mul_u64_w_carry::<5>(u64::MAX);
    let expected = BigUint::from(max_bi) * BigUint::from(u64::MAX);
    assert_eq!(BigUint::from(result), expected);
    
    // mul_u128_w_carry with max values  
    let result = max_bi.mul_u128_w_carry::<5, 6>(u128::MAX);
    let expected = BigUint::from(max_bi) * BigUint::from(u128::MAX);
    assert_eq!(BigUint::from(result), expected);
}

#[test]
fn test_fmu64a_into_nplus4_correctness_and_edges() {
    use crate::biginteger::{BigInt, BigInteger256 as B};
    let a = B::from(0xDEADBEEFCAFEBABEu64);
    let other = 0xFEDCBA9876543210u64;
    let mut acc = BigInt::<8>::zero(); // N+4 accumulator for N=4

    // Reference: (a * other + acc_before) mod 2^(64*(N+4))
    let before = BigUint::from(acc.clone());
    a.fmu64a_into_nplus4::<8>(other, &mut acc);
    let mut expected = BigUint::from(a);
    expected *= BigUint::from(other);
    expected += before;
    let modulus = BigUint::from(1u8) << (64 * 8);
    expected %= &modulus;
    assert_eq!(BigUint::from(acc.clone()), expected);

    // Zero multiplier is no-op
    let mut acc2 = acc.clone();
    a.fmu64a_into_nplus4::<8>(0, &mut acc2);
    assert_eq!(acc2, acc);

    // One multiplier reduces to addition
    let mut acc3 = BigInt::<8>::zero();
    acc3.0[0] = 11111;
    let before3 = BigUint::from(acc3.clone());
    a.fmu64a_into_nplus4::<8>(1, &mut acc3);
    let mut expected3 = BigUint::from(a);
    expected3 += before3;
    expected3 %= &modulus;
    assert_eq!(BigUint::from(acc3), expected3);

    // Force cascading carry across N..=N+3
    let a = B::new([u64::MAX; 4]);
    let mut acc4 = BigInt::<8>::zero();
    acc4.0[4] = u64::MAX; // limb N
    acc4.0[5] = u64::MAX; // limb N+1
    acc4.0[6] = u64::MAX; // limb N+2
    acc4.0[7] = 0;        // limb N+3 (top)
    // Use multiplier 2 so the low pass produces a carry=1
    a.fmu64a_into_nplus4::<8>(2, &mut acc4);
    assert_eq!(acc4.0[7], 1);
}

#[test]
fn test_fm2x64a_into_nplus4_correctness() {
    use crate::biginteger::{BigInt, BigInteger256 as B};
    let a = B::from(0x1234567890ABCDEFu64);
    let other = [0x0FEDCBA987654321u64, 0x0011223344556677u64];
    let mut acc = BigInt::<8>::zero();

    let before = BigUint::from(acc.clone());
    a.fm2x64a_into_nplus4::<8>(other, &mut acc);

    // Expected: a * (lo + (hi << 64)) + acc_before mod 2^(64*8)
    let hi = BigUint::from(other[1]);
    let lo = BigUint::from(other[0]);
    let factor = (hi << 64) + lo;
    let mut expected = BigUint::from(a);
    expected *= factor;
    expected += before;
    let modulus = BigUint::from(1u8) << (64 * 8);
    expected %= &modulus;
    assert_eq!(BigUint::from(acc.clone()), expected);

    // Zero limbs are no-op
    let mut acc2 = acc.clone();
    a.fm2x64a_into_nplus4::<8>([0, 0], &mut acc2);
    assert_eq!(acc2, acc);
}

#[test]
fn test_fm3x64a_into_nplus4_correctness() {
    use crate::biginteger::{BigInt, BigInteger256 as B};
    let a = B::from(0x0F0E0D0C0B0A0908u64);
    let other = [0x89ABCDEF01234567u64, 0x76543210FEDCBA98u64, 0x1122334455667788u64];
    let mut acc = BigInt::<8>::zero();

    let before = BigUint::from(acc.clone());
    a.fm3x64a_into_nplus4::<8>(other, &mut acc);

    // Expected: a * (o0 + (o1<<64) + (o2<<128)) + acc_before mod 2^(64*8)
    let term0 = BigUint::from(other[0]);
    let term1 = BigUint::from(other[1]) << 64;
    let term2 = BigUint::from(other[2]) << 128;
    let factor = term0 + term1 + term2;
    let mut expected = BigUint::from(a);
    expected *= factor;
    expected += before;
    let modulus = BigUint::from(1u8) << (64 * 8);
    expected %= &modulus;
    assert_eq!(BigUint::from(acc.clone()), expected);

    // Edge: ensure offset accumulation lands in correct limbs
    // Fill acc with a pattern, then accumulate using only the highest limb to ensure writes start at index 2
    let a = B::from(3u64);
    let mut acc2 = BigInt::<8>::zero();
    acc2.0[0] = 5;
    acc2.0[1] = 7;
    let other2 = [0, 0, 2]; // Only offset by 2 limbs
    let before2 = BigUint::from(acc2.clone());
    a.fm3x64a_into_nplus4::<8>(other2, &mut acc2);
    let mut expected2 = BigUint::from(a);
    expected2 *= BigUint::from(2u64) << 128;
    expected2 += before2;
    let modulus = BigUint::from(1u8) << (64 * 8);
    expected2 %= &modulus;
    assert_eq!(BigUint::from(acc2), expected2);
}
