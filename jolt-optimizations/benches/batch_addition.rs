use ark_bn254::G1Affine;
use ark_ec::{AffineRepr, CurveGroup};
use ark_std::rand::RngCore;
use ark_std::UniformRand;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use jolt_optimizations::{
    batch_g1_additions, batch_g1_additions_multi, msm_rows_mixed_bn254,
    msm_rows_mixed_bn254_column_centric, SmallRow,
};
use rayon::prelude::*;

fn naive_parallel_sum(bases: &[G1Affine], indices: &[usize]) -> G1Affine {
    indices.par_iter().map(|&idx| bases[idx]).reduce(
        || G1Affine::zero(),
        |acc, point| (acc + point).into_affine(),
    )
}

// fn bench_batch_addition(c: &mut Criterion) {
//     let mut group = c.benchmark_group("batch_g1_addition");
//     let mut rng = ark_std::test_rng();

//     // Test different sizes
//     for size in [1 << 15].iter() {
//         let bases: Vec<G1Affine> = (0..*size).map(|_| G1Affine::rand(&mut rng)).collect();

//         // Use half the points
//         let indices: Vec<usize> = (0..size / 2)
//             .map(|_| (rng.next_u64() as usize) % size)
//             .collect();

//         group.bench_with_input(BenchmarkId::new("batch_optimized", size), size, |b, _| {
//             b.iter(|| black_box(batch_g1_additions(&bases, &indices)));
//         });

//         group.bench_with_input(BenchmarkId::new("naive_parallel", size), size, |b, _| {
//             b.iter(|| black_box(naive_parallel_sum(&bases, &indices)));
//         });
//     }

//     group.finish();
// }

// fn bench_batch_addition_multi(c: &mut Criterion) {
//     let mut group = c.benchmark_group("batch_g1_addition_multi");
//     let mut rng = ark_std::test_rng();

//     let base_size = 1 << 19;
//     let bases: Vec<G1Affine> = (0..base_size).map(|_| G1Affine::rand(&mut rng)).collect();

//     for num_batches in [10].iter() {
//         let batch_size = 1 << 16;

//         let indices_sets: Vec<Vec<usize>> = (0..*num_batches)
//             .map(|_| {
//                 (0..batch_size)
//                     .map(|_| (rng.next_u64() as usize) % base_size)
//                     .collect()
//             })
//             .collect();

//         group.bench_with_input(
//             BenchmarkId::new("multi_batch_shared", num_batches),
//             num_batches,
//             |b, _| {
//                 b.iter(|| black_box(batch_g1_additions_multi(&bases, &indices_sets)));
//             },
//         );

//         group.bench_with_input(
//             BenchmarkId::new("parallel_naive_sum", num_batches),
//             num_batches,
//             |b, _| {
//                 b.iter(|| {
//                     black_box(
//                         indices_sets
//                             .par_iter()
//                             .map(|indices| {
//                                 // Naive parallel sum for each batch
//                                 indices.par_iter().map(|&idx| bases[idx]).reduce(
//                                     || G1Affine::zero(),
//                                     |acc, point| (acc + point).into_affine(),
//                                 )
//                             })
//                             .collect::<Vec<_>>(),
//                     )
//                 });
//             },
//         );
//     }

//     group.finish();
// }

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

        group.bench_with_input(
            BenchmarkId::new("column_centric", &name),
            &(&key, &rows),
            |b, (key, rows)| {
                b.iter(|| black_box(msm_rows_mixed_bn254_column_centric(key, rows)));
            },
        );

        // Compare with naive approach
        // group.bench_with_input(
        //     BenchmarkId::new("naive_per_row", &name),
        //     &(&key, &rows),
        //     |b, (key, rows)| {
        //         b.iter(|| {
        //             black_box(
        //                 rows.iter()
        //                     .map(|row| {
        //                         let mut sum = G1Affine::zero();
        //                         for &idx in row.iter() {
        //                             sum = (sum + key[idx as usize]).into_affine();
        //                         }
        //                         sum
        //                     })
        //                     .collect::<Vec<_>>(),
        //             )
        //         });
        //     },
        // );
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
