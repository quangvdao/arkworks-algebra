// This bench prefers bn254; if not enabled, provide a no-op main
#[cfg(feature = "bn254")]
use ark_ff::UniformRand;
#[cfg(feature = "bn254")]
use ark_std::rand::{rngs::StdRng, Rng, SeedableRng};
#[cfg(feature = "bn254")]
use ark_test_curves::bn254::Fr;
#[cfg(feature = "bn254")]
use criterion::{criterion_group, criterion_main, Criterion};

// Hack: copy over the helper functions from the Montgomery backend to be benched

#[cfg(feature = "bn254")]
fn mul_small_bench(c: &mut Criterion) {
    const SAMPLES: usize = 1000;
    // Use a fixed seed for reproducibility
    let mut rng = StdRng::seed_from_u64(0u64);

    let a_s = (0..SAMPLES)
        .map(|_| Fr::rand(&mut rng))
        .collect::<Vec<_>>();
    // let a_limbs_s = a_s.iter().map(|a| a.0.0).collect::<Vec<_>>();

    let b_u64_s = (0..SAMPLES)
        .map(|_| rng.gen::<u64>())
        .collect::<Vec<_>>();
    // Convert u64 to Fr for standard multiplication benchmark
    let b_fr_s = b_u64_s.iter().map(|&b| Fr::from(b)).collect::<Vec<_>>();

    let b_u64_as_u128_s = b_u64_s.iter().map(|&b| b as u128).collect::<Vec<_>>();

    let b_i64_s = (0..SAMPLES)
        .map(|_| rng.gen::<i64>())
        .collect::<Vec<_>>();

    let b_u128_s = (0..SAMPLES)
        .map(|_| rng.gen::<u128>())
        .collect::<Vec<_>>();

    let b_i128_s = (0..SAMPLES)
        .map(|_| rng.gen::<i128>())
        .collect::<Vec<_>>();

    // Generate another set of random Fr elements for addition
    let c_s = (0..SAMPLES)
        .map(|_| Fr::rand(&mut rng))
        .collect::<Vec<_>>();

    let mut group = c.benchmark_group("Fr Arithmetic Comparison");

    group.bench_function("mul_u64", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            // bn254 Fr has N=4 limbs => N+1 = 5
            criterion::black_box(a_s[i].mul_u64::<5>(b_u64_s[i]))
        })
    });

    group.bench_function("mul_i64", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            criterion::black_box(a_s[i].mul_i64::<5>(b_i64_s[i]))
        })
    });

    // Note: results might be worse than in real applications due to branch prediction being wrong
    // 50% of the time
    group.bench_function("mul_u128", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            // bn254 Fr has N=4 limbs => N+1 = 5, N+2 = 6
            criterion::black_box(a_s[i].mul_u128::<5, 6>(b_u128_s[i]))
        })
    });

    group.bench_function("mul_i128", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            criterion::black_box(a_s[i].mul_i128::<5, 6>(b_i128_s[i]))
        })
    });

    group.bench_function("standard mul (Fr * Fr)", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            criterion::black_box(a_s[i] * b_fr_s[i])
        })
    });

    // Bench specialized trailing-zero RHS fastpaths (K = 1, 2)
    // Construct b' with K trailing zeros in limbs for K=1 and K=2
    let mut b_k1 = b_fr_s.clone();
    for b in &mut b_k1 { (b.0).0[0] = 0; }
    let mut b_k2 = b_fr_s.clone();
    for b in &mut b_k2 { (b.0).0[0] = 0; (b.0).0[1] = 0; }

    group.bench_function("mul_assign_rhs_trailing_zeros::<1>", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            let mut x = a_s[i];
            x.mul_assign_rhs_trailing_zeros::<1>(&b_k1[i]);
            criterion::black_box(x)
        })
    });

    group.bench_function("mul_assign_rhs_trailing_zeros::<2>", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            let mut x = a_s[i];
            x.mul_assign_rhs_trailing_zeros::<2>(&b_k2[i]);
            criterion::black_box(x)
        })
    });

    group.bench_function("mul_rhs_trailing_zeros::<1>", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            criterion::black_box(a_s[i].mul_rhs_trailing_zeros::<1>(&b_k1[i]))
        })
    });

    group.bench_function("mul_rhs_trailing_zeros::<2>", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            criterion::black_box(a_s[i].mul_rhs_trailing_zeros::<2>(&b_k2[i]))
        })
    });

    group.bench_function("mul_u128 (u64 inputs)", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            // Call mul_u128 but provide a u64 input cast to u128
            criterion::black_box(a_s[i].mul_u128::<5, 6>(b_u64_as_u128_s[i]))
        })
    });

    // Benchmark the auxiliary function directly (assuming it's made public)
    // Note: Requires mul_u128_aux to be pub in montgomery_backend.rs
    // Need to import it if not already done via wildcard/specific import
    // Let's assume it's accessible via a_s[i].mul_u128_aux(...) for now
    group.bench_function("mul_u128_aux (u128 inputs)", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            criterion::black_box(a_s[i].mul_u128_aux::<5, 6>(b_u128_s[i]))
        })
    });

    group.bench_function("Addition (Fr + Fr)", |bench| {
        let mut i = 0;
        bench.iter(|| {
            i = (i + 1) % SAMPLES;
            criterion::black_box(a_s[i] + c_s[i])
        })
    });

    group.finish();
}

#[cfg(feature = "bn254")]
criterion_group!(benches, mul_small_bench);
#[cfg(feature = "bn254")]
criterion_main!(benches);

#[cfg(not(feature = "bn254"))]
fn main() {}