use ark_bn254::{G1Affine, G1Projective};
use ark_ec::CurveGroup;
use ark_std::rand::RngCore;
use ark_std::UniformRand;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jolt_optimizations::{
    batch_g1_additions_multi, msm_rows_bucket_affine, msm_rows_bucket_projective,
    msm_rows_mixed_bn254, msm_rows_mixed_bn254_projective, SmallRow, SmallRowOld,
};

fn bench_msm_rows_mixed(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm_rows_mixed");
    let mut rng = ark_std::test_rng();
    group.sample_size(10);
    // Test different matrix sizes
    for &(n, k) in &[(1 << 16, 512)] {
        let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

        let rows: Vec<SmallRowOld> = (0..n)
            .map(|_| {
                let num_indices = (rng.next_u64() as usize) % k + 1;
                let indices: Vec<u32> = (0..num_indices)
                    .map(|_| (rng.next_u64() as u32) % (n as u32))
                    .collect();
                SmallRowOld::from_indices(&indices)
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

        group.bench_with_input(
            BenchmarkId::new("projective_only", &name),
            &(&key, &rows, k),
            |b, (key, rows, k)| {
                b.iter(|| black_box(msm_rows_mixed_bn254_projective(key, rows, *k)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("projective_plus_normalize", &name),
            &(&key, &rows, k),
            |b, (key, rows, k)| {
                b.iter(|| {
                    let proj = msm_rows_mixed_bn254_projective(key, rows, *k);
                    black_box(G1Projective::normalize_batch(&proj))
                });
            },
        );
    }

    group.finish();
}

fn bench_msm_rows_vs_batch_additions(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm_comparison");
    let mut rng = ark_std::test_rng();
    group.sample_size(10);
    let n = 1 << 16;
    let k = 512;

    let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

    let rows: Vec<SmallRowOld> = (0..n)
        .map(|_| {
            let num_indices = (rng.next_u64() as usize) % k + 1;
            let indices: Vec<u32> = (0..num_indices)
                .map(|_| (rng.next_u64() as u32) % (n as u32))
                .collect();
            SmallRowOld::from_indices(&indices)
        })
        .collect();

    group.bench_function("msm_rows_mixed", |b| {
        b.iter(|| black_box(msm_rows_mixed_bn254(&key, &rows)));
    });

    group.bench_function("msm_rows_projective", |b| {
        b.iter(|| black_box(msm_rows_mixed_bn254_projective(&key, &rows, k)));
    });

    group.bench_function("msm_rows_proj_plus_norm", |b| {
        b.iter(|| {
            let proj = msm_rows_mixed_bn254_projective(&key, &rows, k);
            black_box(G1Projective::normalize_batch(&proj))
        });
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

fn bench_msm_bucket_vs_projective(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm_bucket_comparison");
    let mut rng = ark_std::test_rng();
    group.sample_size(10);

    for &(n, k) in &[(4096, 128), (1 << 16, 512)] {
        let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

        // New runtime-sized SmallRow - use u16 for n < 65536, u32 otherwise
        let rows_new: Vec<SmallRow> = if n < 65536 {
            (0..n)
                .map(|_| {
                    let num_indices = (rng.next_u64() as usize) % k + 1;
                    let indices: Vec<u16> = (0..num_indices)
                        .map(|_| (rng.next_u64() as usize % n) as u16)
                        .collect();
                    SmallRow::from_u16(indices)
                })
                .collect()
        } else {
            (0..n)
                .map(|_| {
                    let num_indices = (rng.next_u64() as usize) % k + 1;
                    let indices: Vec<u32> = (0..num_indices)
                        .map(|_| (rng.next_u64() as usize % n) as u32)
                        .collect();
                    SmallRow::from_u32(indices)
                })
                .collect()
        };

        // Old const-generic SmallRow
        let rows_old: Vec<SmallRowOld> = (0..n)
            .map(|_| {
                let num_indices = (rng.next_u64() as usize) % k + 1;
                let indices: Vec<u32> = (0..num_indices)
                    .map(|_| (rng.next_u64() as u32) % (n as u32))
                    .collect();
                SmallRowOld::from_indices(&indices)
            })
            .collect();

        let name = format!("n={}_k={}", n, k);

        // Bucket (XYZZ) - projective only
        group.bench_with_input(
            BenchmarkId::new("bucket_projective", &name),
            &(&key, &rows_new, k),
            |b, (key, rows, k)| {
                b.iter(|| black_box(msm_rows_bucket_projective(key, rows, *k)));
            },
        );

        // Bucket (XYZZ) - with normalization
        group.bench_with_input(
            BenchmarkId::new("bucket_affine", &name),
            &(&key, &rows_new, k),
            |b, (key, rows, k)| {
                b.iter(|| black_box(msm_rows_bucket_affine(key, rows, *k)));
            },
        );

        // Old projective - for comparison
        group.bench_with_input(
            BenchmarkId::new("old_projective", &name),
            &(&key, &rows_old, k),
            |b, (key, rows, k)| {
                b.iter(|| black_box(msm_rows_mixed_bn254_projective(key, rows, *k)));
            },
        );

        // Old with normalization - for comparison
        group.bench_with_input(
            BenchmarkId::new("old_affine", &name),
            &(&key, &rows_old),
            |b, (key, rows)| {
                b.iter(|| black_box(msm_rows_mixed_bn254(key, rows)));
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_msm_rows_mixed,
    bench_msm_rows_vs_batch_additions,
    bench_msm_bucket_vs_projective
);
criterion_main!(benches);
