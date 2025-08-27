use std::panic;

use crate::fq12_poly::{fq12_to_poly12_coeffs, g_coeffs, poly_div_rem_monic, poly_mul};
use ark_bn254::{Fq, Fq12};
use ark_ff::{Field, Zero};

pub struct Product {
    pub a: Fq12,
    pub b: Fq12,
    pub c: Fq12,
    pub quotient: Vec<Fq>,
}

impl Product {
    pub fn new(a: Fq12, b: Fq12, c: Fq12) -> Self {
        let a_poly = fq12_to_poly12_coeffs(&a);
        let b_poly = fq12_to_poly12_coeffs(&b);
        let c_poly = fq12_to_poly12_coeffs(&c);

        let mut ab = poly_mul(&a_poly, &b_poly);
        for i in 0..c_poly.len().min(ab.len()) {
            ab[i] -= c_poly[i];
        }

        let (quotient, remainder) = poly_div_rem_monic(ab, &g_coeffs());

        if !remainder.is_empty() && remainder.iter().any(|r| !r.is_zero()) {
            panic!("invalid product: remainder is non-zero")
        }

        Self { a, b, c, quotient }
    }
}

fn compute_r_powers(r: &Fq) -> [Fq; 12] {
    let mut powers = [Fq::zero(); 12];
    powers[0] = Fq::from(1u64);
    for i in 1..12 {
        powers[i] = powers[i - 1] * r;
    }
    powers
}

fn eval_with_powers(coeffs: &[Fq; 12], r_powers: &[Fq; 12]) -> Fq {
    let mut result = Fq::zero();
    for i in 0..12 {
        result += coeffs[i] * r_powers[i];
    }
    result
}

pub fn g_eval_optimized(r: &Fq) -> Fq {
    let r2 = r.square();
    let r3 = r2 * r;
    let r6 = r3.square();
    let r12 = r6.square();
    r12 - Fq::from(18u64) * r6 + Fq::from(82u64)
}

pub fn batch_verify(products: &[Product], r: &Fq) -> bool {
    let r_powers = compute_r_powers(r);
    let g_r = g_eval_optimized(r);

    for product in products {
        let a_coeffs = fq12_to_poly12_coeffs(&product.a);
        let b_coeffs = fq12_to_poly12_coeffs(&product.b);
        let c_coeffs = fq12_to_poly12_coeffs(&product.c);

        let a_r = eval_with_powers(&a_coeffs, &r_powers);
        let b_r = eval_with_powers(&b_coeffs, &r_powers);
        let c_r = eval_with_powers(&c_coeffs, &r_powers);

        let lhs = a_r * b_r - c_r;

        let mut q_r = Fq::zero();
        for (i, coeff) in product.quotient.iter().enumerate() {
            if i < 12 {
                q_r += *coeff * r_powers[i];
            } else {
                panic!("this can't happen")
            }
        }
        let rhs = q_r * g_r;

        if lhs != rhs {
            return false;
        }
    }

    true
}
