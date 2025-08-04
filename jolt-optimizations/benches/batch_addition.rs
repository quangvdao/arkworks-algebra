use ark_bn254::G1Affine;
use ark_ec::{AffineRepr, CurveGroup};
use ark_std::rand::RngCore;
use ark_std::UniformRand;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jolt_optimizations::{batch_g1_additions, batch_g1_additions_multi};
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
    for size in [1 << 15].iter() {
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

fn bench_batch_addition_multi(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_g1_addition_multi");
    let mut rng = ark_std::test_rng();

    let base_size = 1 << 19;
    let bases: Vec<G1Affine> = (0..base_size).map(|_| G1Affine::rand(&mut rng)).collect();

    for num_batches in [10].iter() {
        let batch_size = 1 << 16;

        let indices_sets: Vec<Vec<usize>> = (0..*num_batches)
            .map(|_| {
                (0..batch_size)
                    .map(|_| (rng.next_u64() as usize) % base_size)
                    .collect()
            })
            .collect();

        group.bench_with_input(
            BenchmarkId::new("multi_batch_shared", num_batches),
            num_batches,
            |b, _| {
                b.iter(|| black_box(batch_g1_additions_multi(&bases, &indices_sets)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel_naive_sum", num_batches),
            num_batches,
            |b, _| {
                b.iter(|| {
                    black_box(
                        indices_sets
                            .par_iter()
                            .map(|indices| {
                                // Naive parallel sum for each batch
                                indices.par_iter().map(|&idx| bases[idx]).reduce(
                                    || G1Affine::zero(),
                                    |acc, point| (acc + point).into_affine(),
                                )
                            })
                            .collect::<Vec<_>>(),
                    )
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_batch_addition, bench_batch_addition_multi);
criterion_main!(benches);
