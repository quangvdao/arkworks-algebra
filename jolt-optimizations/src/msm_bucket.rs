//! Row-wise binary MSM using Bucket (XYZZ coordinates)

use ark_bn254::g1::Config as G1Config;
use ark_bn254::{G1Affine, G1Projective};
use ark_ec::{models::short_weierstrass::Bucket, CurveGroup};
use rayon::prelude::*;

use crate::small_row::SmallRow;

/// Computes row-wise binary MSM using Bucket (XYZZ) coordinates, returning projective points.
///
/// # Arguments
/// * `key` - Fixed G1Affine key of length n (column basis points)
/// * `rows` - Slice of SmallRow; each entry holds indices into `key`
/// * `k_hint` - Runtime hint for max row size (used to tune ILP)
///
/// # Returns
/// Vector of G1Projective of length n (row sums in projective form)
pub fn msm_rows_bucket_projective(
    key: &[G1Affine],
    rows: &[SmallRow],
    k_hint: usize,
) -> Vec<G1Projective> {
    #[inline(always)]
    fn ilp_from_k(k: usize) -> usize {
        match k {
            0..=64 => 2,
            65..=256 => 4,
            257..=1024 => 6,
            _ => 8,
        }
    }

    let ilp = ilp_from_k(k_hint);

    rows.par_iter()
        .map(|row| {
            let mut acc = Bucket::<G1Config>::ZERO;

            if row.is_u16() {
                let s = row.as_u16_slice();
                let mut chunks = s.chunks_exact(ilp);

                for ch in &mut chunks {
                    acc += key[ch[0] as usize];
                    if ilp > 1 {
                        acc += key[ch[1] as usize];
                    }
                    if ilp > 2 {
                        acc += key[ch[2] as usize];
                    }
                    if ilp > 3 {
                        acc += key[ch[3] as usize];
                    }
                    if ilp > 4 {
                        acc += key[ch[4] as usize];
                    }
                    if ilp > 5 {
                        acc += key[ch[5] as usize];
                    }
                    if ilp > 6 {
                        acc += key[ch[6] as usize];
                    }
                    if ilp > 7 {
                        acc += key[ch[7] as usize];
                    }
                }

                for &j in chunks.remainder() {
                    acc += key[j as usize];
                }
            } else {
                let s = row.as_u32_slice();
                let mut chunks = s.chunks_exact(ilp);

                for ch in &mut chunks {
                    acc += key[ch[0] as usize];
                    if ilp > 1 {
                        acc += key[ch[1] as usize];
                    }
                    if ilp > 2 {
                        acc += key[ch[2] as usize];
                    }
                    if ilp > 3 {
                        acc += key[ch[3] as usize];
                    }
                    if ilp > 4 {
                        acc += key[ch[4] as usize];
                    }
                    if ilp > 5 {
                        acc += key[ch[5] as usize];
                    }
                    if ilp > 6 {
                        acc += key[ch[6] as usize];
                    }
                    if ilp > 7 {
                        acc += key[ch[7] as usize];
                    }
                }

                for &j in chunks.remainder() {
                    acc += key[j as usize];
                }
            }

            acc.into()
        })
        .collect()
}

/// Computes row-wise binary MSM using Bucket (XYZZ), returning affine points.
///
/// # Arguments
/// * `key` - Fixed G1Affine key of length n (column basis points)
/// * `rows` - Slice of SmallRow; each entry holds indices into `key`
/// * `k_hint` - Runtime hint for max row size (used to tune ILP)
///
/// # Returns
/// Vector of G1Affine of length n (row sums in affine form)
pub fn msm_rows_bucket_affine(key: &[G1Affine], rows: &[SmallRow], k_hint: usize) -> Vec<G1Affine> {
    let proj = msm_rows_bucket_projective(key, rows, k_hint);
    G1Projective::normalize_batch(&proj)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::{rand::RngCore, UniformRand};

    #[test]
    fn test_msm_bucket_correctness() {
        let mut rng = ark_std::test_rng();

        let n = 100;
        let k = 50;

        let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

        let rows: Vec<SmallRow> = (0..n)
            .map(|_| {
                let num_indices = (rng.next_u64() as usize) % k + 1;
                let indices: Vec<u16> = (0..num_indices)
                    .map(|_| (rng.next_u64() as u16) % (n as u16))
                    .collect();
                SmallRow::from_u16(indices)
            })
            .collect();

        let result = msm_rows_bucket_affine(&key, &rows, k);

        for (row_idx, row) in rows.iter().enumerate() {
            let mut expected = G1Affine::identity();
            for idx in row.iter_usize() {
                expected = (expected + key[idx]).into();
            }
            assert_eq!(
                result[row_idx], expected,
                "Bucket MSM mismatch at row {}",
                row_idx
            );
        }
    }

    #[test]
    fn test_msm_bucket_projective() {
        let mut rng = ark_std::test_rng();

        let n = 50;
        let k = 30;

        let key: Vec<G1Affine> = (0..n).map(|_| G1Affine::rand(&mut rng)).collect();

        let rows: Vec<SmallRow> = (0..n)
            .map(|_| {
                let num_indices = (rng.next_u64() as usize) % k + 1;
                let indices: Vec<u16> = (0..num_indices)
                    .map(|_| (rng.next_u64() as u16) % (n as u16))
                    .collect();
                SmallRow::from_u16(indices)
            })
            .collect();

        let proj = msm_rows_bucket_projective(&key, &rows, k);
        let result = G1Projective::normalize_batch(&proj);

        for (row_idx, row) in rows.iter().enumerate() {
            let mut expected = G1Affine::identity();
            for idx in row.iter_usize() {
                expected = (expected + key[idx]).into();
            }
            assert_eq!(
                result[row_idx], expected,
                "Bucket projective MSM mismatch at row {}",
                row_idx
            );
        }
    }
}
