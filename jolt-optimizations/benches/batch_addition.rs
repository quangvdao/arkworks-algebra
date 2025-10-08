use ark_bn254::G1Affine;
use ark_std::rand::RngCore;
use ark_std::UniformRand;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jolt_optimizations::{
    batch_g1_additions_multi, msm_rows_bucket_affine, msm_rows_bucket_projective, SmallRow,
};

fn bench_msm_bucket_vs_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("msm_comparison");
    let mut rng = ark_std::test_rng();
    group.sample_size(10);

    for &(n, k) in &[(4096, 128), (1 << 16, 512)] {
        let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

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

        let name = format!("n={}_k={}", n, k);

        group.bench_with_input(
            BenchmarkId::new("bucket_projective", &name),
            &(&key, &rows_new, k),
            |b, (key, rows, k)| {
                b.iter(|| black_box(msm_rows_bucket_projective(key, rows, *k)));
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bucket_affine", &name),
            &(&key, &rows_new, k),
            |b, (key, rows, k)| {
                b.iter(|| black_box(msm_rows_bucket_affine(key, rows, *k)));
            },
        );

        let indices_sets: Vec<Vec<usize>> = rows_new
            .iter()
            .map(|row| row.iter_usize().collect())
            .collect();

        group.bench_with_input(
            BenchmarkId::new("batch_additions_multi", &name),
            &(&key, &indices_sets),
            |b, (key, indices_sets)| {
                b.iter(|| black_box(batch_g1_additions_multi(key, indices_sets)));
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_msm_bucket_vs_batch);
criterion_main!(benches);
