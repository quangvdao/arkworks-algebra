use ark_bn254::{Fq, Fq12};
use ark_ff::UniformRand;
use ark_std::test_rng;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use jolt_optimizations::sz_check::{batch_verify, Product};

fn benchmark_sz_check(c: &mut Criterion) {
    let mut rng = test_rng();
    let sizes = vec![100000];

    for k in sizes {
        let mut products = Vec::new();
        let mut a_values = Vec::new();
        let mut b_values = Vec::new();

        for _ in 0..k {
            let a = Fq12::rand(&mut rng);
            let b = Fq12::rand(&mut rng);
            let c = a * b;
            a_values.push(a);
            b_values.push(b);
            products.push(Product::new(a, b, c));
        }

        let r = Fq::rand(&mut rng);

        c.bench_function(&format!("naive_verify_{}", k), |bench| {
            bench.iter(|| {
                for i in 0..k {
                    let _ = black_box(a_values[i] * b_values[i]);
                }
            });
        });

        c.bench_function(&format!("sz_check_{}", k), |bench| {
            bench.iter(|| black_box(batch_verify(&products, &r)));
        });
    }
}

criterion_group!(benches, benchmark_sz_check);
criterion_main!(benches);
