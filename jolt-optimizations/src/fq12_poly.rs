//! Fq12 polynomial operations and conversions for BN254
use ark_bn254::{Fq, Fq12};
use ark_ff::{Field, One, Zero};

/// Constant for the tower extension mapping
const NINE: u64 = 9;

/// Newtype wrapper for degree-12 polynomial coefficients
#[derive(Clone, Debug, Default)]
pub struct Poly12([Fq; 12]);

impl Poly12 {
    pub fn new(coeffs: [Fq; 12]) -> Self {
        Self(coeffs)
    }

    pub fn coeffs(&self) -> &[Fq; 12] {
        &self.0
    }

    pub fn coeffs_mut(&mut self) -> &mut [Fq; 12] {
        &mut self.0
    }

    pub fn to_vec(&self) -> Vec<Fq> {
        self.0.to_vec()
    }

    /// Evaluate at a point using Horner's method
    pub fn eval(&self, r: &Fq) -> Fq {
        self.0.iter().rev().fold(Fq::zero(), |acc, c| acc * r + c)
    }
}

/// Tower basis mapping for Fq12 -> polynomial conversion
struct TowerBasis {
    /// Maps basis elements to power indices: [(element, power_of_w)]
    mappings: [(usize, usize, usize); 6], // (c0/c1, inner_idx, w_power)
}

impl TowerBasis {
    const fn new() -> Self {
        Self {
            mappings: [
                (0, 0, 0), // a.c0.c0 → w^0
                (0, 1, 2), // a.c0.c1 → w^2
                (0, 2, 4), // a.c0.c2 → w^4
                (1, 0, 1), // a.c1.c0 → w^1
                (1, 1, 3), // a.c1.c1 → w^3
                (1, 2, 5), // a.c1.c2 → w^5
            ],
        }
    }

    fn apply(&self, a: &Fq12) -> Poly12 {
        let nine = Fq::from(NINE);
        let mut coeffs = [Fq::zero(); 12];

        for &(outer, inner, w_power) in &self.mappings {
            let fp2 = match (outer, inner) {
                (0, 0) => &a.c0.c0,
                (0, 1) => &a.c0.c1,
                (0, 2) => &a.c0.c2,
                (1, 0) => &a.c1.c0,
                (1, 1) => &a.c1.c1,
                (1, 2) => &a.c1.c2,
                _ => unreachable!(),
            };

            let (x, y) = (fp2.c0, fp2.c1);
            // Apply: (x + y·u)·w^k = (x - 9y)·w^k + y·w^{k+6}
            coeffs[w_power] += x - nine * y;
            coeffs[w_power + 6] += y;
        }

        Poly12::new(coeffs)
    }
}

static TOWER_BASIS: TowerBasis = TowerBasis::new();

/// Convert Fq12 to polynomial representation
pub fn fq12_to_poly12_coeffs(a: &Fq12) -> [Fq; 12] {
    TOWER_BASIS.apply(a).0
}

/// The minimal polynomial g(X) = X^12 - 18 X^6 + 82
struct MinimalPolynomial;

impl MinimalPolynomial {
    const COEFF_0: u64 = 82;
    const COEFF_6: i64 = -18;

    /// Evaluate g(X) at point r
    fn eval(r: &Fq) -> Fq {
        let r6 = (r.square() * r).square(); // r^6 = (r^2 * r)^2
        let r12 = r6.square();
        r12 - Fq::from(18u64) * r6 + Fq::from(Self::COEFF_0)
    }

    /// Get coefficients as a vector
    fn coeffs() -> Vec<Fq> {
        let mut g = vec![Fq::zero(); 13];
        g[0] = Fq::from(Self::COEFF_0);
        g[6] = -Fq::from(18u64);
        g[12] = Fq::one();
        g
    }
}

/// Evaluate g(X) = X^12 - 18 X^6 + 82 at a given point r
pub fn g_eval(r: &Fq) -> Fq {
    MinimalPolynomial::eval(r)
}

/// Horner evaluation for arbitrary-degree polynomial
pub fn eval_poly_vec(coeffs: &[Fq], r: &Fq) -> Fq {
    coeffs.iter().rev().fold(Fq::zero(), |acc, c| acc * r + c)
}

/// Generic polynomial operation in place
fn poly_op_in_place<F>(a: &mut Vec<Fq>, b: &[Fq], op: F)
where
    F: Fn(&mut Fq, Fq),
{
    if b.len() > a.len() {
        a.resize(b.len(), Fq::zero());
    }
    b.iter().enumerate().for_each(|(i, &coeff)| op(&mut a[i], coeff));
}

/// Add polynomial b to polynomial a in place
pub fn poly_add_in_place(a: &mut Vec<Fq>, b: &[Fq]) {
    poly_op_in_place(a, b, |a, b| *a += b);
}

/// Subtract polynomial b from polynomial a in place
pub fn poly_sub_in_place(a: &mut Vec<Fq>, b: &[Fq]) {
    poly_op_in_place(a, b, |a, b| *a -= b);
}

/// Multiply two polynomials using convolution
pub fn poly_mul(a: &[Fq], b: &[Fq]) -> Vec<Fq> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }

    let mut out = vec![Fq::zero(); a.len() + b.len() - 1];
    a.iter().enumerate().for_each(|(i, &ai)| {
        b.iter().enumerate().for_each(|(j, &bj)| {
            out[i + j] += ai * bj;
        })
    });
    out
}

/// Polynomial long division by a monic divisor
pub fn poly_div_rem_monic(mut dividend: Vec<Fq>, divisor: &[Fq]) -> (Vec<Fq>, Vec<Fq>) {
    assert!(!divisor.is_empty(), "divisor must be non-empty");
    assert!(
        divisor.last().unwrap().is_one(),
        "divisor must be monic (leading coefficient = 1)"
    );

    if dividend.is_empty() || dividend.len() < divisor.len() {
        return (vec![], dividend);
    }

    let deg_dividend = dividend.len() - 1;
    let deg_divisor = divisor.len() - 1;
    let mut quotient = vec![Fq::zero(); deg_dividend - deg_divisor + 1];

    for k in (deg_divisor..=deg_dividend).rev() {
        let coeff = dividend[k];
        quotient[k - deg_divisor] = coeff;

        if !coeff.is_zero() {
            // Subtract coeff * x^{k-deg_divisor} * divisor from dividend
            (0..=deg_divisor).for_each(|j| {
                dividend[k - deg_divisor + j] -= coeff * divisor[j];
            });
        }
    }

    // Trim trailing zeros from remainder
    while dividend.last() == Some(&Fq::zero()) {
        dividend.pop();
    }

    (quotient, dividend)
}

/// Build the coefficients for g(X) = X^12 - 18 X^6 + 82
pub fn g_coeffs() -> Vec<Fq> {
    MinimalPolynomial::coeffs()
}

/// Convert Fq12 polynomial coefficients to multilinear evaluations by padding to 16 elements
pub fn to_multilinear_evals(coeffs: &[Fq; 12]) -> Vec<Fq> {
    let mut evals = Vec::with_capacity(16);
    evals.extend_from_slice(coeffs);
    evals.resize(16, Fq::zero());
    evals
}

/// Convert Fq12 directly to multilinear evaluations
pub fn fq12_to_multilinear_evals(a: &Fq12) -> Vec<Fq> {
    to_multilinear_evals(&fq12_to_poly12_coeffs(a))
}
