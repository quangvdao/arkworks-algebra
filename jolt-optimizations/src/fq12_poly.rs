//! Fq12 polynomial operations and conversions for BN254
//!
//! This module provides:
//! - Conversion between Fq12 field elements and polynomial representations
//! - Polynomial arithmetic operations over Fq[X]
//! - Evaluation and manipulation of the minimal polynomial g(X) = X^12 - 18X^6 + 82

use ark_bn254::{Fq, Fq12};
use ark_ff::{Field, One, Zero};

/// Flatten Fq12 to 12 base-field coefficients for a(X)=Σ c_i X^i, X=w,
/// with the relation g(X) = X^12 - 18 X^6 + 82.
///
/// The BN254 Fq12 field is constructed as a tower extension:
/// - Fq2 = Fq[u]/(u^2 + 1)
/// - Fq6 = Fq2[v]/(v^3 - (9 + u))
/// - Fq12 = Fq6[w]/(w^2 - v)
///
/// This function maps an Fq12 element to its polynomial representation
/// in Fq[X] where X = w, using the mapping:
/// (x + y·u)·w^k = (x - 9y)·w^k + y·w^{k+6}, for k∈{0..5}.
/// @TODO(markosg04) provide proof?
pub fn fq12_to_poly12_coeffs(a: &Fq12) -> [Fq; 12] {
    let nine = Fq::from(9u64);
    let mut c = [Fq::zero(); 12];

    // (term, k) pairs mapping Fq12 basis elements to powers of w:
    // 1, v, v^2, w, v·w, v^2·w  ↔  w^0, w^2, w^4, w^1, w^3, w^5
    let terms = [
        (&a.c0.c0, 0usize), // 1 → w^0
        (&a.c0.c1, 2usize), // v → w^2
        (&a.c0.c2, 4usize), // v^2 → w^4
        (&a.c1.c0, 1usize), // w → w^1
        (&a.c1.c1, 3usize), // v·w → w^3
        (&a.c1.c2, 5usize), // v^2·w → w^5
    ];

    for (fp2, k) in terms {
        let x = fp2.c0; // coefficient of 1 in Fp2
        let y = fp2.c1; // coefficient of u in Fp2 (with u^2 = -1)
                        // Apply the mapping: (x + y·u)·w^k = (x - 9y)·w^k + y·w^{k+6}
        c[k] += x - nine * y;
        c[k + 6] += y;
    }
    c
}

/// Evaluate g(X) = X^12 - 18 X^6 + 82 at a given point r.
pub fn g_eval(r: &Fq) -> Fq {
    let r2 = r.square(); // r^2
    let r3 = r2 * r; // r^3
    let r6 = r3.square(); // r^6
    let r12 = r6.square(); // r^12
    r12 - (Fq::from(18u64) * r6) + Fq::from(82u64)
}

/// Horner evaluation for arbitrary-degree polynomial.
pub fn eval_poly_vec(coeffs: &[Fq], r: &Fq) -> Fq {
    let mut acc = Fq::zero();
    for &c in coeffs.iter().rev() {
        acc *= r;
        acc += c;
    }
    acc
}

/// Add polynomial b to polynomial a in place.
pub fn poly_add_in_place(a: &mut Vec<Fq>, b: &[Fq]) {
    if b.len() > a.len() {
        a.resize(b.len(), Fq::zero());
    }
    for i in 0..b.len() {
        a[i] += b[i];
    }
}

/// Subtract polynomial b from polynomial a in place.
pub fn poly_sub_in_place(a: &mut Vec<Fq>, b: &[Fq]) {
    if b.len() > a.len() {
        a.resize(b.len(), Fq::zero());
    }
    for i in 0..b.len() {
        a[i] -= b[i];
    }
}

/// Multiply two polynomials using convolution.
pub fn poly_mul(a: &[Fq], b: &[Fq]) -> Vec<Fq> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![Fq::zero(); a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            out[i + j] += a[i] * b[j];
        }
    }
    out
}

/// Polynomial long division by a monic divisor.
pub fn poly_div_rem_monic(mut dividend: Vec<Fq>, g: &[Fq]) -> (Vec<Fq>, Vec<Fq>) {
    assert!(!g.is_empty(), "divisor g must be non-empty");
    assert!(
        g.last().unwrap().is_one(),
        "divisor g must be monic (leading coefficient = 1)"
    );

    if dividend.is_empty() || dividend.len() < g.len() {
        return (vec![], dividend);
    }

    let n = dividend.len() - 1;
    let m = g.len() - 1; // deg g
    let mut q = vec![Fq::zero(); n - m + 1];

    for k in (m..=n).rev() {
        let lead = dividend[k]; // since g is monic, this is the quotient coefficient
        q[k - m] = lead;
        if lead.is_zero() {
            continue;
        }
        // subtract lead * x^{k-m} * g from dividend
        for j in 0..=m {
            dividend[k - m + j] -= lead * g[j];
        }
    }

    // trim trailing zeros from remainder
    while let Some(true) = dividend.last().map(|c| c.is_zero()) {
        dividend.pop();
    }

    (q, dividend)
}

/// Build the coefficients for g(X) = X^12 - 18 X^6 + 82.
pub fn g_coeffs() -> Vec<Fq> {
    let mut g = vec![Fq::zero(); 13];
    g[0] = Fq::from(82u64);
    g[6] = -Fq::from(18u64);
    g[12] = Fq::one();
    g
}

/// Convert Fq12 polynomial coefficients to multilinear evaluations by padding to 16 elements.
/// The 12 coefficients are padded with 4 zeros to make a power-of-2 size suitable for
/// multilinear polynomial commitment schemes.
pub fn to_multilinear_evals(coeffs: &[Fq; 12]) -> Vec<Fq> {
    let mut evals = coeffs.to_vec();
    evals.resize(16, Fq::zero());
    evals
}

/// Convert an Fq12 element to multilinear evaluations.
/// First converts to polynomial coefficients, then pads to 16 elements.
pub fn fq12_to_multilinear_evals(a: &Fq12) -> Vec<Fq> {
    let coeffs = fq12_to_poly12_coeffs(a);
    to_multilinear_evals(&coeffs)
}
