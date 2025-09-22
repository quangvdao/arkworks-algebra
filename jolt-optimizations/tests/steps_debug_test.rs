use ark_bn254::{Fq, Fq12};
use ark_ff::BigInteger;
use ark_ff::{Field, PrimeField, UniformRand};
use ark_std::test_rng;
use jolt_optimizations::steps::pow_with_steps_le;

#[test]
#[ignore] // Run with: cargo test --test steps_debug_test test_debug_trace -- --nocapture --ignored
fn test_debug_trace() {
    let mut rng = test_rng();

    // Use a small exponent for readable output
    let base = Fq12::rand(&mut rng);
    let exponent = Fq::from(13u64); // Binary: 1101

    println!("=== Square-and-Multiply Debug Trace ===");
    println!("Base: {:?}", base);
    println!("Exponent: {} (binary: 1101)", 13u64);
    println!();

    let steps = pow_with_steps_le(base, exponent);

    // Print bit representation
    let bigint = exponent.into_bigint();
    let exp_bits = bigint.to_bits_le();
    println!("Bit representation (LSB first):");
    for (i, bit) in exp_bits.iter().take(8).enumerate() {
        println!("  Bit {}: {}", i, if *bit { "1" } else { "0" });
    }
    println!();

    // Print initial state
    println!("Initial state:");
    println!("  a_0 = base");
    println!(
        "  rho_0 = {} (since bit 0 = {})",
        if exp_bits[0] { "base" } else { "1" },
        if exp_bits[0] { "1" } else { "0" }
    );
    println!();

    // Print each step
    println!("Steps:");
    for (i, step) in steps.steps.iter().enumerate() {
        println!(
            "Step {} (processing bit {} = {}):",
            i + 1,
            i + 1,
            if step.bit_value { "1" } else { "0" }
        );

        println!("  Squaring: a_{} = a_{}^2", i + 1, i);
        println!("    a_{} = {:?}", i, step.a_prev());
        println!("    a_{} = {:?}", i + 1, step.a_curr());

        // Verify squaring
        let expected_square = step.a_prev() * step.a_prev();
        println!(
            "    Verification: a_curr == a_prev^2? {}",
            if step.a_curr() == expected_square {
                "✓"
            } else {
                "✗"
            }
        );

        println!("  Accumulator update:");
        println!("    rho_before = {:?}", step.rho_before());

        if step.bit_value {
            println!("    Bit is 1, so: rho_after = rho_before * a_curr");
        } else {
            println!("    Bit is 0, so: rho_after = rho_before (unchanged)");
        }

        println!("    rho_after = {:?}", step.rho_after());

        // Verify accumulator update
        let expected_rho = if step.bit_value {
            step.rho_before() * step.a_curr()
        } else {
            step.rho_before()
        };
        println!(
            "    Verification: rho_after correct? {}",
            if step.rho_after() == expected_rho {
                "✓"
            } else {
                "✗"
            }
        );

        println!();
    }

    // Print final result
    println!("Final result: {:?}", steps.result);

    // Verify against standard pow
    let expected = base.pow(exponent.into_bigint());
    println!("Expected (base^13): {:?}", expected);
    println!(
        "Results match: {}",
        if steps.result == expected {
            "✓"
        } else {
            "✗"
        }
    );

    // Print summary of operations
    println!();
    println!("=== Summary ===");
    let num_squarings = steps.steps.len();
    let num_multiplications = steps.steps.iter().filter(|s| s.bit_value).count();
    println!("Total squarings: {}", num_squarings);
    println!("Total multiplications by base: {}", num_multiplications);
    println!("Total operations: {}", num_squarings + num_multiplications);

    // Verify the steps
    assert!(steps.sanity_verify(), "Steps verification failed");
    assert_eq!(steps.result, expected, "Result doesn't match expected");
}

#[test]
#[ignore] // Run with: cargo test --test steps_debug_test test_trace_products -- --nocapture --ignored
fn test_trace_products() {
    let mut rng = test_rng();

    let base = Fq12::rand(&mut rng);
    let exponent = Fq::from(5u64); // Binary: 101

    println!("=== Products Generated from Steps ===");
    println!("Exponent: 5 (binary: 101)");
    println!();

    let steps = pow_with_steps_le(base, exponent);
    let products = steps.to_products();

    println!("Products generated:");
    for (i, product) in products.iter().enumerate() {
        println!("Product {}:", i);
        println!("  a * b = c");
        println!("  a: {:?}", product.a);
        println!("  b: {:?}", product.b);
        println!("  c: {:?}", product.c);

        // Verify the product
        let expected_c = product.a * product.b;
        println!(
            "  Verification: c == a * b? {}",
            if product.c == expected_c {
                "✓"
            } else {
                "✗"
            }
        );
        println!();
    }

    println!("Total products: {}", products.len());

    // Test batch verification
    use jolt_optimizations::sz_check::batch_verify;
    let r = Fq::rand(&mut rng);
    let batch_result = batch_verify(&products, &r);
    println!(
        "Batch verification with random r: {}",
        if batch_result {
            "✓ PASSED"
        } else {
            "✗ FAILED"
        }
    );
}
