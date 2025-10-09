#[cfg(feature = "bn254")]
use ark_ec::pairing::Pairing;
#[cfg(feature = "bn254")]
use ark_ff::UniformRand;
#[cfg(feature = "bn254")]
use ark_std::test_rng;
#[cfg(feature = "bn254")]
use ark_test_curves::bn254::Bn254;
#[cfg(feature = "bn254")]
use ark_test_curves::bn254::Fq2;
#[cfg(feature = "bn254")]
use ark_test_curves::bn254::{fq12_compressed_multi_pairing, CompressedFq12};
#[cfg(feature = "bn254")]
use ark_test_curves::bn254::{fq12_compressed_pairing, G1Projective, G2Projective};
#[cfg(feature = "bn254")]
use criterion::{black_box, BatchSize, Criterion};

#[cfg(feature = "bn254")]
fn uncompressed_pairing_bench(c: &mut Criterion) {
    let mut rng = test_rng();

    c.bench_function("uncompressed_pairing (1 pair)", |b| {
        b.iter_batched(
            || {
                let g1 = G1Projective::rand(&mut rng);
                let g2 = G2Projective::rand(&mut rng);
                (g1, g2)
            },
            |(g1, g2)| black_box(Bn254::pairing(g1, g2)),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "bn254")]
fn uncompressed_multi_pairing_len_5_bench(c: &mut Criterion) {
    let mut rng = test_rng();

    c.bench_function("Bn254::multi_pairing (5 pairs)", |b| {
        b.iter_batched(
            || {
                let g1 = (0..5)
                    .map(|_| G1Projective::rand(&mut rng))
                    .collect::<Vec<_>>();
                let g2 = (0..5)
                    .map(|_| G2Projective::rand(&mut rng))
                    .collect::<Vec<_>>();
                (g1, g2)
            },
            |(g1, g2)| black_box(Bn254::multi_pairing(g1, g2)),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "bn254")]
fn compressed_pairing_bench(c: &mut Criterion) {
    let mut rng = test_rng();

    c.bench_function("fq12_compressed_pairing (1 pair)", |b| {
        b.iter_batched(
            || {
                let g1 = G1Projective::rand(&mut rng);
                let g2 = G2Projective::rand(&mut rng);
                (g1, g2)
            },
            |(g1, g2)| black_box(fq12_compressed_pairing(g1, g2)),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "bn254")]
fn compressed_multi_pairing_len_5_bench(c: &mut Criterion) {
    let mut rng = test_rng();

    c.bench_function("fq12_compressed_multi_pairing (5 pairs)", |b| {
        b.iter_batched(
            || {
                let g1 = (0..5)
                    .map(|_| G1Projective::rand(&mut rng))
                    .collect::<Vec<_>>();
                let g2 = (0..5)
                    .map(|_| G2Projective::rand(&mut rng))
                    .collect::<Vec<_>>();
                (g1, g2)
            },
            |(g1, g2)| black_box(fq12_compressed_multi_pairing(g1, g2)),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "bn254")]
fn decompression(c: &mut Criterion) {
    let mut rng = test_rng();

    c.bench_function("pairing_value_decompression (1 pair)", |b| {
        b.iter_batched(
            || CompressedFq12((Fq2::rand(&mut rng), Fq2::rand(&mut rng))),
            |compressed_value| black_box(compressed_value.decompress_to_fq12()),
            BatchSize::SmallInput,
        )
    });
}

#[cfg(feature = "bn254")]
criterion::criterion_group!(
    benches,
    compressed_pairing_bench,
    uncompressed_pairing_bench,
    compressed_multi_pairing_len_5_bench,
    uncompressed_multi_pairing_len_5_bench,
    decompression
);
#[cfg(feature = "bn254")]
criterion::criterion_main!(benches);

#[cfg(not(feature = "bn254"))]
fn main() {}
