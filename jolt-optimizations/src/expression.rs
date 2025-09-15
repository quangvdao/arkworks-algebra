use crate::steps::{pow_with_steps_le, ExponentiationSteps};
use crate::sz_check::Product;
use ark_bn254::{Fq, Fq12};
use ark_ff::{Field, One, PrimeField};

#[derive(Clone)]
pub struct Term {
    pub base: Fq12,
    pub exponent: Fq,
}

pub struct Expression {
    pub terms: Vec<Term>,
}

pub struct ExpressionSteps {
    pub term_steps: Vec<ExponentiationSteps>,
    pub multiplication_products: Vec<Product>,
}

impl Expression {
    pub fn new(terms: Vec<Term>) -> Self {
        Self { terms }
    }

    pub fn to_products(&self) -> Vec<Product> {
        let mut products = Vec::new();
        let mut current_result = Fq12::one();

        for term in &self.terms {
            let term_value = term.base.pow(term.exponent.into_bigint());
            let term_products = exponentiate_to_products(term.base, term.exponent);

            products.extend(term_products);

            if current_result != Fq12::one() {
                // Multiply this term's result with the accumulated result
                let new_result = current_result * term_value;
                products.push(Product::new(current_result, term_value, new_result));
                current_result = new_result;
            } else {
                current_result = term_value;
            }
        }

        products
    }

    /// Evaluate the expression and return both the result and all computation steps
    pub fn evaluate_with_steps(&self) -> (Fq12, ExpressionSteps) {
        let mut term_steps = Vec::new();
        let mut multiplication_products = Vec::new();
        let mut current_result = Fq12::one();

        for term in &self.terms {
            // Compute this term with steps
            let steps = pow_with_steps_le(term.base, term.exponent);
            let term_value = steps.result;
            term_steps.push(steps);

            if current_result != Fq12::one() {
                // Multiply this term's result with the accumulated result
                let new_result = current_result * term_value;
                multiplication_products.push(Product::new(current_result, term_value, new_result));
                current_result = new_result;
            } else {
                current_result = term_value;
            }
        }

        let expression_steps = ExpressionSteps {
            term_steps,
            multiplication_products,
        };

        (current_result, expression_steps)
    }

    /// Convert expression steps to a flat list of products for verification
    pub fn steps_to_products(steps: &ExpressionSteps) -> Vec<Product> {
        let mut products = Vec::new();

        // Add all products from individual term exponentiations
        for term_step in &steps.term_steps {
            products.extend(term_step.to_products());
        }

        // Add products from multiplying terms together
        for product in &steps.multiplication_products {
            products.push(product.clone());
        }

        products
    }
}

fn exponentiate_to_products(base: Fq12, exponent: Fq) -> Vec<Product> {
    // Use the new stepped implementation to get products
    let steps = pow_with_steps_le(base, exponent);
    steps.to_products()
}
