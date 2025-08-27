use ark_bn254::{Fq, Fq12};
use ark_ff::{Field, PrimeField, UniformRand};
use ark_std::test_rng;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jolt_optimizations::expression::{Expression, Term};
use jolt_optimizations::sz_check::batch_verify;

fn benchmark_expression_verification(c: &mut Criterion) {
    let mut rng = test_rng();

    let configs = vec![(15, 6)];

    for (n, m) in configs {
        // Generate n expressions, each with m terms
        let mut all_expressions = Vec::new();
        let mut all_expected_results = Vec::new();

        for _ in 0..n {
            let mut terms = Vec::new();
            let mut expected = Fq12::from(1u64);

            for _ in 0..m {
                let base = Fq12::rand(&mut rng);
                let exponent = Fq::rand(&mut rng);
                terms.push(Term { base, exponent });
                expected *= base.pow(exponent.into_bigint());
            }

            all_expressions.push(Expression::new(terms));
            all_expected_results.push(expected);
        }

        let mut all_products = Vec::new();
        for expr in &all_expressions {
            all_products.extend(expr.to_products());
        }

        let r = Fq::rand(&mut rng);

        // naive computation
        c.bench_function(&format!("naive_expr_{}x{}", n, m), |bench| {
            bench.iter(|| {
                for i in 0..n {
                    let mut result = Fq12::from(1u64);
                    for term in &all_expressions[i].terms {
                        result *= black_box(term.base.pow(term.exponent.into_bigint()));
                    }
                    black_box(result);
                }
            });
        });

        //  SZ check verification
        c.bench_function(&format!("sz_check_expr_{}x{}", n, m), |bench| {
            bench.iter(|| black_box(batch_verify(&all_products, &r)));
        });
    }
}

criterion_group!(benches, benchmark_expression_verification,);
criterion_main!(benches);
