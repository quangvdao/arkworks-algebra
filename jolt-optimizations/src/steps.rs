use crate::sz_check::Product;
use ark_bn254::{Fq, Fq12};
use ark_ff::{BigInteger, Field, One, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};

/// Represents a single step in the square-and-multiply exponentiation algorithm.
#[derive(Clone, Debug, Default, CanonicalSerialize, CanonicalDeserialize)]
pub struct ExponentiationStep {
    pub step_index: usize,
    pub bit_value: bool,
    pub a_prev: Fq12,
    pub a_curr: Fq12,
    pub rho_before: Fq12,
    pub rho_after: Fq12,
}

#[derive(Clone, Debug, Default, CanonicalSerialize, CanonicalDeserialize)]
pub struct ExponentiationSteps {
    /// The base being exponentiated
    pub base: Fq12,
    /// The exponent
    pub exponent: Fq,
    /// All steps in the computation
    pub steps: Vec<ExponentiationStep>,
    /// The final result (should equal base^exponent)
    pub result: Fq12,
}

impl ExponentiationSteps {
    /// Convert the steps into Products for verification with sz_check
    pub fn to_products(&self) -> Vec<Product> {
        let mut products = Vec::new();

        for step in &self.steps {
            // Each squaring operation creates a product: a_i = a_{i-1} * a_{i-1}
            products.push(Product::new(step.a_prev, step.a_prev, step.a_curr));

            // If the bit is 1, we multiply rho by the current power
            if step.bit_value && step.rho_before != step.rho_after {
                products.push(Product::new(step.rho_before, step.a_curr, step.rho_after));
            }
        }

        products
    }

    pub fn sanity_verify(&self) -> bool {
        let expected = self.base.pow(self.exponent.into_bigint());
        if self.result != expected {
            return false;
        }

        for (i, step) in self.steps.iter().enumerate() {
            if step.a_curr != step.a_prev * step.a_prev {
                return false;
            }

            let expected_rho_after = if step.bit_value {
                step.rho_before * step.a_curr
            } else {
                step.rho_before
            };

            if step.rho_after != expected_rho_after {
                return false;
            }

            if i + 1 < self.steps.len() {
                if step.a_curr != self.steps[i + 1].a_prev {
                    return false;
                }
                if step.rho_after != self.steps[i + 1].rho_before {
                    return false;
                }
            }
        }

        if let Some(last_step) = self.steps.last() {
            if last_step.rho_after != self.result {
                return false;
            }
        }

        true
    }
}

pub fn pow_with_steps_le(base: Fq12, exponent: Fq) -> ExponentiationSteps {
    let mut steps = Vec::new();

    let bigint = exponent.into_bigint();
    let exp_bits = bigint.to_bits_le();

    // Find the position of the last 1 bit
    let last_one = exp_bits.iter().rposition(|&b| b);

    if last_one.is_none() {
        // Exponent is 0, return 1
        return ExponentiationSteps {
            base,
            exponent,
            steps: vec![],
            result: Fq12::one(),
        };
    }

    let last_one = last_one.unwrap();

    if last_one == 0 {
        // Exponent is 1, return base
        return ExponentiationSteps {
            base,
            exponent,
            steps: vec![],
            result: base,
        };
    }

    let mut a_curr = base; // Current power of base
    let mut rho = if exp_bits[0] { base } else { Fq12::one() };

    for (step_idx, bit_idx) in (1..=last_one).enumerate() {
        let bit_value = exp_bits[bit_idx];
        let a_prev = a_curr;
        let rho_before = rho;

        a_curr = a_prev * a_prev;

        let rho_after = if bit_value {
            rho_before * a_curr
        } else {
            rho_before
        };

        steps.push(ExponentiationStep {
            step_index: step_idx,
            bit_value,
            a_prev,
            a_curr,
            rho_before,
            rho_after,
        });

        rho = rho_after;
    }

    ExponentiationSteps {
        base,
        exponent,
        steps,
        result: rho,
    }
}
