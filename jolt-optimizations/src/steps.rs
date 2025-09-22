use crate::sz_check::Product;
use ark_bn254::{Fq, Fq12};
use ark_ff::{BigInteger, Field, One, PrimeField};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use std::fmt;

/// Error types for exponentiation verification
#[derive(Debug, Clone, PartialEq)]
pub enum VerificationError {
    IncorrectResult { expected: Fq12, actual: Fq12 },
    InvalidSquaring { step: usize, expected: Fq12, actual: Fq12 },
    InvalidMultiplication { step: usize, expected: Fq12, actual: Fq12 },
    InconsistentChain { step: usize },
}

impl fmt::Display for VerificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncorrectResult { .. } => write!(f, "Final result doesn't match expected"),
            Self::InvalidSquaring { step, .. } => write!(f, "Invalid squaring at step {}", step),
            Self::InvalidMultiplication { step, .. } => write!(f, "Invalid multiplication at step {}", step),
            Self::InconsistentChain { step } => write!(f, "Inconsistent state chain at step {}", step),
        }
    }
}

impl std::error::Error for VerificationError {}

/// State transition in exponentiation
#[derive(Clone, Debug, Default, CanonicalSerialize, CanonicalDeserialize)]
pub struct StepTransition {
    /// Previous and current accumulator values
    pub accumulator: (Fq12, Fq12),
    /// Running product before and after this step
    pub product: (Fq12, Fq12),
}

/// Single step in square-and-multiply algorithm
#[derive(Clone, Debug, Default, CanonicalSerialize, CanonicalDeserialize)]
pub struct ExponentiationStep {
    pub step_index: usize,
    pub bit_value: bool,
    pub transition: StepTransition,
}

impl ExponentiationStep {
    fn new(step_index: usize, bit_value: bool, a_prev: Fq12, a_curr: Fq12, rho_before: Fq12, rho_after: Fq12) -> Self {
        Self {
            step_index,
            bit_value,
            transition: StepTransition {
                accumulator: (a_prev, a_curr),
                product: (rho_before, rho_after),
            },
        }
    }

    /// Get the previous accumulator value
    pub fn a_prev(&self) -> Fq12 {
        self.transition.accumulator.0
    }

    /// Get the current accumulator value
    pub fn a_curr(&self) -> Fq12 {
        self.transition.accumulator.1
    }

    /// Get the product before this step
    pub fn rho_before(&self) -> Fq12 {
        self.transition.product.0
    }

    /// Get the product after this step
    pub fn rho_after(&self) -> Fq12 {
        self.transition.product.1
    }
}

#[derive(Clone, Debug, Default, CanonicalSerialize, CanonicalDeserialize)]
pub struct ExponentiationSteps {
    pub base: Fq12,
    pub exponent: Fq,
    pub steps: Vec<ExponentiationStep>,
    pub result: Fq12,
}

/// Builder for ExponentiationSteps
pub struct StepsBuilder {
    base: Fq12,
    exponent: Fq,
    steps: Vec<ExponentiationStep>,
}

impl StepsBuilder {
    fn new(base: Fq12, exponent: Fq) -> Self {
        Self {
            base,
            exponent,
            steps: Vec::new(),
        }
    }

    fn add_step(&mut self, step: ExponentiationStep) {
        self.steps.push(step);
    }

    fn build(self, result: Fq12) -> ExponentiationSteps {
        ExponentiationSteps {
            base: self.base,
            exponent: self.exponent,
            steps: self.steps,
            result,
        }
    }
}

impl ExponentiationSteps {
    /// Convert steps to Products for sz_check verification
    pub fn to_products(&self) -> Vec<Product> {
        self.steps
            .iter()
            .flat_map(|step| {
                let mut products = vec![
                    // Squaring: a_i = a_{i-1} * a_{i-1}
                    Product::new(step.a_prev(), step.a_prev(), step.a_curr()),
                ];

                // Multiplication if bit is set
                if step.bit_value && step.rho_before() != step.rho_after() {
                    products.push(Product::new(
                        step.rho_before(),
                        step.a_curr(),
                        step.rho_after(),
                    ));
                }

                products
            })
            .collect()
    }

    /// Verify consistency of recorded steps
    pub fn verify_consistency(&self) -> Result<(), VerificationError> {
        // Check final result
        let expected = self.base.pow(self.exponent.into_bigint());
        if self.result != expected {
            return Err(VerificationError::IncorrectResult {
                expected,
                actual: self.result,
            });
        }

        // Verify each step
        for (i, step) in self.steps.iter().enumerate() {
            // Verify squaring
            let expected_a = step.a_prev() * step.a_prev();
            if step.a_curr() != expected_a {
                return Err(VerificationError::InvalidSquaring {
                    step: i,
                    expected: expected_a,
                    actual: step.a_curr(),
                });
            }

            // Verify multiplication
            let expected_rho = if step.bit_value {
                step.rho_before() * step.a_curr()
            } else {
                step.rho_before()
            };
            if step.rho_after() != expected_rho {
                return Err(VerificationError::InvalidMultiplication {
                    step: i,
                    expected: expected_rho,
                    actual: step.rho_after(),
                });
            }

            // Verify chain consistency
            if let Some(next) = self.steps.get(i + 1) {
                if step.a_curr() != next.a_prev() || step.rho_after() != next.rho_before() {
                    return Err(VerificationError::InconsistentChain { step: i + 1 });
                }
            }
        }

        // Verify final step matches result
        if let Some(last) = self.steps.last() {
            if last.rho_after() != self.result {
                return Err(VerificationError::IncorrectResult {
                    expected: self.result,
                    actual: last.rho_after(),
                });
            }
        }

        Ok(())
    }

    /// Legacy verification method for compatibility
    pub fn sanity_verify(&self) -> bool {
        self.verify_consistency().is_ok()
    }
}

/// Helper to iterate over significant bits
struct BitIterator {
    bits: Vec<bool>,
    last_one_pos: Option<usize>,
}

impl BitIterator {
    fn new(exponent: Fq) -> Self {
        let bits = exponent.into_bigint().to_bits_le();
        let last_one_pos = bits.iter().rposition(|&b| b);
        Self { bits, last_one_pos }
    }

    fn is_trivial(&self) -> Option<Fq12> {
        match self.last_one_pos {
            None => Some(Fq12::one()),        // exp = 0
            Some(0) => None,                  // exp = 1, handled separately
            _ => None,
        }
    }

    fn initial_bit(&self) -> bool {
        self.bits.get(0).copied().unwrap_or(false)
    }

    fn significant_bits(&self) -> impl Iterator<Item = (usize, bool)> + '_ {
        let end = self.last_one_pos.unwrap_or(0);
        (1..=end).map(move |i| (i - 1, self.bits[i]))
    }
}

/// Compute base^exponent with step-by-step recording (LSB-first)
pub fn pow_with_steps_le(base: Fq12, exponent: Fq) -> ExponentiationSteps {
    let bits = BitIterator::new(exponent);

    // Handle trivial cases
    if let Some(result) = bits.is_trivial() {
        return ExponentiationSteps {
            base,
            exponent,
            steps: vec![],
            result: if bits.last_one_pos.is_none() { result } else { base },
        };
    }

    let mut builder = StepsBuilder::new(base, exponent);
    let mut accumulator = base;
    let mut product = if bits.initial_bit() { base } else { Fq12::one() };

    for (step_idx, bit) in bits.significant_bits() {
        let prev_acc = accumulator;
        let prev_prod = product;

        accumulator = prev_acc.square();
        product = if bit { prev_prod * accumulator } else { prev_prod };

        builder.add_step(ExponentiationStep::new(
            step_idx,
            bit,
            prev_acc,
            accumulator,
            prev_prod,
            product,
        ));
    }

    builder.build(product)
}
