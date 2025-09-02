use ark_bn254::{Fq, Fq12};
use ark_ff::{Field, PrimeField, UniformRand, Zero};
use ark_std::test_rng;
use jolt_optimizations::expression::{Expression, Term};
use jolt_optimizations::fq12_poly::{fq12_to_multilinear_evals, fq12_to_poly12_coeffs};
use jolt_optimizations::sz_check::{batch_verify, Product};

#[test]
fn test_large_batch() {
    let mut rng = test_rng();
    let k = 100000;

    let mut products = Vec::new();
    for _ in 0..k {
        let a = Fq12::rand(&mut rng);
        let b = Fq12::rand(&mut rng);
        let c = a * b;
        products.push(Product::new(a, b, c));
    }

    let r = Fq::rand(&mut rng);

    assert!(batch_verify(&products, &r));
}

#[test]
fn test_expression_to_sz_check() {
    let mut rng = test_rng();
    let a1 = Fq12::rand(&mut rng);
    let c1 = Fq::rand(&mut rng);

    let a2 = Fq12::rand(&mut rng);
    let c2 = Fq::rand(&mut rng);

    let a3 = Fq12::rand(&mut rng);
    let c3 = Fq::rand(&mut rng);

    let expected = a1.pow(c1.into_bigint()) * a2.pow(c2.into_bigint()) * a3.pow(c3.into_bigint());

    let expr = Expression::new(vec![
        Term {
            base: a1,
            exponent: c1,
        },
        Term {
            base: a2,
            exponent: c2,
        },
        Term {
            base: a3,
            exponent: c3,
        },
    ]);

    let products = expr.to_products();

    let r = Fq::rand(&mut rng);
    assert!(batch_verify(&products, &r));

    if !products.is_empty() {
        let final_result = products.last().unwrap().c;
        assert_eq!(final_result, expected);
    }
}
