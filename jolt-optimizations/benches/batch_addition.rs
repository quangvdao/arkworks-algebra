use ark_bn254::G1Affine;
use ark_ec::{AffineRepr, CurveGroup};
use ark_std::rand::RngCore;
use ark_std::UniformRand;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jolt_optimizations::batch_g1_additions;
use rayon::prelude::*;

fn naive_parallel_sum(bases: &[G1Affine], indices: &[usize]) -> G1Affine {
    indices.par_iter().map(|&idx| bases[idx]).reduce(
        || G1Affine::zero(),
        |acc, point| (acc + point).into_affine(),
    )
}

fn bench_batch_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_g1_addition");
    let mut rng = ark_std::test_rng();

    // Test different sizes
    for size in [1 << 20].iter() {
        let bases: Vec<G1Affine> = (0..*size).map(|_| G1Affine::rand(&mut rng)).collect();

        // Use half the points
        let indices: Vec<usize> = (0..size / 2)
            .map(|_| (rng.next_u64() as usize) % size)
            .collect();

        group.bench_with_input(BenchmarkId::new("batch_optimized", size), size, |b, _| {
            b.iter(|| black_box(batch_g1_additions(&bases, &indices)));
        });

        group.bench_with_input(BenchmarkId::new("naive_parallel", size), size, |b, _| {
            b.iter(|| black_box(naive_parallel_sum(&bases, &indices)));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_batch_addition);
criterion_main!(benches);
