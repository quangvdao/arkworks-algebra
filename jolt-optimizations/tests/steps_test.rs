use ark_bn254::{Fq, Fq12};
use ark_ff::{Field, One, PrimeField, UniformRand};
use ark_std::test_rng;
use jolt_optimizations::expression::{Expression, Term};
use jolt_optimizations::steps::pow_with_steps_le;
use jolt_optimizations::sz_check::batch_verify;

#[test]
fn test_pow_with_steps_correctness() {
    let mut rng = test_rng();

    // Test with random base and exponent
    let base = Fq12::rand(&mut rng);
    let exponent = Fq::rand(&mut rng);

    // Compute with steps
    let steps = pow_with_steps_le(base, exponent);

    // Verify the result matches standard pow
    let expected = base.pow(exponent.into_bigint());
    assert_eq!(steps.result, expected, "Result mismatch");

    // Verify the steps are internally consistent
    assert!(steps.sanity_verify(), "Steps verification failed");

    // Verify that products can be verified using batch_verify
    let products = steps.to_products();
    let r = Fq::rand(&mut rng);
    assert!(batch_verify(&products, &r), "Batch verification failed");
}

#[test]
fn test_pow_with_steps_edge_cases() {
    let mut rng = test_rng();
    let base = Fq12::rand(&mut rng);

    // Test exponent = 0
    let steps = pow_with_steps_le(base, Fq::from(0u64));
    assert_eq!(steps.result, Fq12::one());
    assert_eq!(steps.steps.len(), 0);

    // Test exponent = 1
    let steps = pow_with_steps_le(base, Fq::from(1u64));
    assert_eq!(steps.result, base);
    assert_eq!(steps.steps.len(), 0);

    // Test exponent = 2
    let steps = pow_with_steps_le(base, Fq::from(2u64));
    assert_eq!(steps.result, base * base);
    assert_eq!(steps.steps.len(), 1);
    assert!(steps.sanity_verify());
}

#[test]
fn test_expression_with_steps() {
    let mut rng = test_rng();

    // Create an expression with multiple terms
    let terms = vec![
        Term {
            base: Fq12::rand(&mut rng),
            exponent: Fq::from(5u64),
        },
        Term {
            base: Fq12::rand(&mut rng),
            exponent: Fq::from(3u64),
        },
    ];

    let expr = Expression::new(terms);

    // Evaluate with steps
    let (result, steps) = expr.evaluate_with_steps();

    // Verify result matches expected
    let expected = expr.terms[0].base.pow(expr.terms[0].exponent.into_bigint())
        * expr.terms[1].base.pow(expr.terms[1].exponent.into_bigint());
    assert_eq!(result, expected);

    // Verify all steps
    for term_step in &steps.term_steps {
        assert!(term_step.sanity_verify());
    }

    // Convert to products and verify
    let products = Expression::steps_to_products(&steps);
    let r = Fq::rand(&mut rng);
    assert!(
        batch_verify(&products, &r),
        "Batch verification of expression steps failed"
    );
}

#[test]
fn test_step_continuity() {
    let mut rng = test_rng();
    let base = Fq12::rand(&mut rng);
    let exponent = Fq::from(255u64); // Use a reasonable sized exponent

    let steps = pow_with_steps_le(base, exponent);

    // Check continuity between steps
    for i in 0..steps.steps.len() - 1 {
        assert_eq!(
            steps.steps[i].rho_after(),
            steps.steps[i + 1].rho_before(),
            "Step continuity broken at step {}",
            i
        );
    }

    // Check final step leads to result
    if let Some(last_step) = steps.steps.last() {
        assert_eq!(last_step.rho_after(), steps.result);
    }
}

#[test]
fn test_squaring_correctness() {
    let mut rng = test_rng();
    let base = Fq12::rand(&mut rng);
    let exponent = Fq::from(100u64);

    let steps = pow_with_steps_le(base, exponent);

    // Verify each squaring operation: a_i = a_{i-1}^2
    for step in &steps.steps {
        let expected_square = step.a_prev() * step.a_prev();
        assert_eq!(
            step.a_curr(), expected_square,
            "Squaring incorrect at step {}",
            step.step_index
        );
    }
}
