use ark_bn254::G1Affine;
use ark_ec::{AffineRepr, CurveGroup};
use ark_std::rand::RngCore;
use ark_std::UniformRand;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jolt_optimizations::{batch_g1_additions_multi, msm_rows_mixed_bn254, SmallRow};
use rayon::prelude::*;

fn bench_msm_rows_mixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm_rows_mixed");
    let mut rng = ark_std::test_rng();
    group.sample_size(10);
    // Test different matrix sizes
    for &(n, k) in &[(1024, 512), (4096, 2048)] {
        let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

        let rows: Vec<SmallRow> = (0..n)
            .map(|_| {
                let num_indices = (rng.next_u64() as usize) % k + 1;
                let indices: Vec<u32> = (0..num_indices)
                    .map(|_| (rng.next_u64() as u32) % (n as u32))
                    .collect();
                SmallRow::from_indices(&indices)
            })
            .collect();

        let name = format!("n={}_k={}", n, k);

        group.bench_with_input(
            BenchmarkId::new("row_centric", &name),
            &(&key, &rows),
            |b, (key, rows)| {
                b.iter(|| black_box(msm_rows_mixed_bn254(key, rows)));
            },
        );
    }

    group.finish();
}

fn bench_msm_rows_vs_batch_additions(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm_comparison");
    let mut rng = ark_std::test_rng();
    group.sample_size(10);
    let n = 2048;
    let k = 1024;

    let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

    let rows: Vec<SmallRow> = (0..n)
        .map(|_| {
            let num_indices = (rng.next_u64() as usize) % k + 1;
            let indices: Vec<u32> = (0..num_indices)
                .map(|_| (rng.next_u64() as u32) % (n as u32))
                .collect();
            SmallRow::from_indices(&indices)
        })
        .collect();

    group.bench_function("msm_rows_mixed", |b| {
        b.iter(|| black_box(msm_rows_mixed_bn254(&key, &rows)));
    });

    // Convert to old format for comparison
    let indices_sets: Vec<Vec<usize>> = rows
        .iter()
        .map(|row| row.iter().map(|&idx| idx as usize).collect())
        .collect();

    group.bench_function("batch_g1_additions_multi", |b| {
        b.iter(|| black_box(batch_g1_additions_multi(&key, &indices_sets)));
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_msm_rows_mixed,
    bench_msm_rows_vs_batch_additions
);
criterion_main!(benches);
