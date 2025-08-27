use crate::sz_check::Product;
use ark_bn254::{Fq, Fq12};
use ark_ff::{BigInteger, Field, One, PrimeField};

pub struct Term {
    pub base: Fq12,
    pub exponent: Fq,
}

pub struct Expression {
    pub terms: Vec<Term>,
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
}

fn exponentiate_to_products(base: Fq12, exponent: Fq) -> Vec<Product> {
    let mut products = Vec::new();

    let bigint = exponent.into_bigint();
    let exp_bits = bigint.to_bits_le();

    let last_one = exp_bits.iter().rposition(|&b| b);

    if last_one.is_none() {
        return vec![];
    }

    let last_one = last_one.unwrap();

    if last_one == 0 {
        return vec![];
    }

    let mut current_power = base;
    let mut result = if exp_bits[0] { base } else { Fq12::one() };

    // square and multiply
    for i in 1..=last_one {
        let squared = current_power * current_power;
        products.push(Product::new(current_power, current_power, squared));
        current_power = squared;

        if exp_bits[i] {
            let new_result = result * current_power;
            products.push(Product::new(result, current_power, new_result));
            result = new_result;
        }
    }

    products
}
