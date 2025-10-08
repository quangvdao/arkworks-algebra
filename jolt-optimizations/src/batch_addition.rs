//! Batch affine point addition for G1
//!
//! Implements efficient batch addition of affine elliptic curve points
//! using Montgomery's batch inversion trick to minimize field inversions.
//!
//! Also provides high-performance row-wise binary MSM using projective
//! accumulator with mixed addition and batched normalization.
use ark_bn254::{G1Affine, G1Projective};
use ark_ec::{AffineRepr, CurveGroup};
use arrayvec::ArrayVec;
use rayon::prelude::*;

/// Default maximum capacity for sparse rows
pub const DEFAULT_MAX_ROW_CAPACITY: usize = 2048;

/// Sparse row representation for binary MSM
#[derive(Clone, Debug)]
pub struct SmallRow<const K: usize = DEFAULT_MAX_ROW_CAPACITY> {
    /// Number of non-zero entries in this row
    pub len: u16,
    /// Indices of non-zero entries (column positions)
    pub indices: ArrayVec<u32, K>,
}

impl<const K: usize> SmallRow<K> {
    #[inline]
    pub fn new() -> Self {
        Self {
            len: 0,
            indices: ArrayVec::new(),
        }
    }
    #[inline]
    pub fn from_indices(indices: &[u32]) -> Self {
        let mut row = Self::new();
        for &idx in indices {
            row.push(idx);
        }
        row
    }

    #[inline]
    pub fn push(&mut self, idx: u32) {
        if self.indices.len() < K {
            self.indices.push(idx);
            self.len = self.indices.len() as u16;
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &u32> {
        self.indices.iter().take(self.len as usize)
    }

    #[inline]
    pub fn as_slice(&self) -> &[u32] {
        &self.indices[..self.len as usize]
    }
}

impl<const K: usize> Default for SmallRow<K> {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs batch addition of G1 affine points.
///
/// Given a slice of base points and indices, computes the sum of all points
/// at the specified indices: bases[indices[0]] + bases[indices[1]] + ... + bases[indices[n-1]]
///
/// Uses batch inversion to compute all divisions efficiently:
/// - Standard addition: 1 inversion per addition
/// - Batch addition: 1 inversion + 3n multiplications for n additions
///
/// # Arguments
/// * `bases` - Slice of G1 affine points to select from
/// * `indices` - Slice of indices specifying which points to sum
///
/// # Returns
/// The sum of all selected points as a single G1Affine point
pub fn batch_g1_additions(bases: &[G1Affine], indices: &[usize]) -> G1Affine {
    if indices.is_empty() {
        return G1Affine::zero();
    }

    if indices.len() == 1 {
        return bases[indices[0]];
    }

    let mut points: Vec<G1Affine> = Vec::with_capacity(indices.len());
    points.extend(indices.iter().map(|&i| bases[i]));

    while points.len() > 1 {
        let current_len = points.len();
        let pairs_count = current_len / 2;
        let has_odd = current_len % 2 == 1;

        let denominators: Vec<_> = (0..pairs_count)
            .into_par_iter()
            .map(|i| {
                let p1 = points[i * 2];
                let p2 = points[i * 2 + 1];
                p2.x - p1.x
            })
            .collect();

        // Batch invert all denominators
        let mut inverses = denominators;
        ark_ff::fields::batch_inversion(&mut inverses);

        let mut new_points: Vec<G1Affine> = (0..pairs_count)
            .into_par_iter()
            .zip(inverses.par_iter())
            .map(|(i, inv)| {
                let p1 = points[i * 2];
                let p2 = points[i * 2 + 1];
                let lambda = (p2.y - p1.y) * inv;
                let x3 = lambda * lambda - p1.x - p2.x;
                let y3 = lambda * (p1.x - x3) - p1.y;
                G1Affine::new_unchecked(x3, y3)
            })
            .collect();

        // Handle odd element
        if has_odd {
            new_points.push(points[current_len - 1]);
        }

        points = new_points;
    }

    points[0]
}

/// Performs multiple batch additions of G1 affine points in parallel.
///
/// Given a slice of base points and multiple sets of indices, computes the sum
/// for each set of indices. All additions across all batches share the same
/// batch inversion.
///
/// # Arguments
/// * `bases` - Slice of G1 affine points to select from
/// * `indices_sets` - Vector of index vectors, each specifying which points to sum
///
/// # Returns
/// Vector of sums, one for each index set
pub fn batch_g1_additions_multi(bases: &[G1Affine], indices_sets: &[Vec<usize>]) -> Vec<G1Affine> {
    if indices_sets.is_empty() {
        return vec![];
    }

    // Initialize working sets for each batch
    let mut working_sets: Vec<Vec<G1Affine>> = indices_sets
        .par_iter()
        .map(|indices| {
            if indices.is_empty() {
                vec![G1Affine::zero()]
            } else if indices.len() == 1 {
                vec![bases[indices[0]]]
            } else {
                indices.iter().map(|&i| bases[i]).collect()
            }
        })
        .collect();

    // Continue until all sets have been reduced to a single point
    loop {
        // Count total number of pairs across all sets
        let total_pairs: usize = working_sets.iter().map(|set| set.len() / 2).sum();

        if total_pairs == 0 {
            break;
        }

        // Collect all denominators across all sets
        let mut all_denominators = Vec::with_capacity(total_pairs);
        let mut pair_info = Vec::with_capacity(total_pairs);

        for (set_idx, set) in working_sets.iter().enumerate() {
            let pairs_in_set = set.len() / 2;
            for pair_idx in 0..pairs_in_set {
                let p1 = set[pair_idx * 2];
                let p2 = set[pair_idx * 2 + 1];
                all_denominators.push(p2.x - p1.x);
                pair_info.push((set_idx, pair_idx));
            }
        }

        // Batch invert all denominators at once
        let mut inverses = all_denominators;
        ark_ff::fields::batch_inversion(&mut inverses);

        // Apply additions using the inverted denominators
        let mut new_working_sets: Vec<Vec<G1Affine>> = working_sets
            .iter()
            .map(|set| Vec::with_capacity((set.len() + 1) / 2))
            .collect();

        // Process additions and maintain order
        for ((set_idx, pair_idx), inv) in pair_info.iter().zip(inverses.iter()) {
            let set = &working_sets[*set_idx];
            let p1 = set[*pair_idx * 2];
            let p2 = set[*pair_idx * 2 + 1];
            let lambda = (p2.y - p1.y) * inv;
            let x3 = lambda * lambda - p1.x - p2.x;
            let y3 = lambda * (p1.x - x3) - p1.y;
            new_working_sets[*set_idx].push(G1Affine::new_unchecked(x3, y3));
        }

        // Handle odd elements
        for (set_idx, set) in working_sets.iter().enumerate() {
            if set.len() % 2 == 1 {
                new_working_sets[set_idx].push(set[set.len() - 1]);
            }
        }

        working_sets = new_working_sets;
    }

    // Extract final results
    working_sets.into_iter().map(|set| set[0]).collect()
}

/// Computes row-wise binary MSM on an n×n sparse binary matrix with ≤k ones per row.
///
/// # Arguments
/// * `key` - Fixed G1Affine key of length n (column basis points)
/// * `rows` - Slice of length n; each entry holds up to k sorted unique indices into `key`
///
/// # Returns
/// Vector of G1Affine of length n (row sums)
///
/// # Algorithm
/// 1. Parallel accumulation in projective coordinates (no inversions in hot loop)
/// 2. Each row accumulates using mixed addition: `projective += affine`
/// 3. Single batched normalization at end (amortizes inversion cost)
pub fn msm_rows_mixed_bn254<const K: usize>(
    key: &[G1Affine],
    rows: &[SmallRow<K>],
) -> Vec<G1Affine> {
    assert_eq!(
        key.len(),
        rows.len(),
        "Key length must equal number of rows"
    );
    let proj: Vec<G1Projective> = rows
        .par_iter()
        .map(|row| {
            let mut acc = G1Projective::default();

            for &idx in row.iter() {
                let idx = idx as usize;
                if idx < key.len() {
                    acc += key[idx];
                }
            }

            acc
        })
        .collect();
    G1Projective::normalize_batch(&proj)
}

/// Computes row-wise binary MSM returning projective points (no batch normalization).
///
/// # Arguments
/// * `key` - Fixed G1Affine key of length n (column basis points)
/// * `rows` - Slice of SmallRow; each entry holds indices into `key`
/// * `k_hint` - Runtime hint for max row size (used to tune ILP)
///
/// # Returns
/// Vector of G1Projective of length n (row sums in projective form)
pub fn msm_rows_mixed_bn254_projective<const K: usize>(
    key: &[G1Affine],
    rows: &[SmallRow<K>],
    k_hint: usize,
) -> Vec<G1Projective> {
    #[inline]
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
            let mut acc = G1Projective::default();
            let indices = row.as_slice();
            for chunk in indices.chunks(ilp) {
                if let Some(&j0) = chunk.get(0) {
                    acc += key[j0 as usize];
                }
                if let Some(&j1) = chunk.get(1) {
                    acc += key[j1 as usize];
                }
                if let Some(&j2) = chunk.get(2) {
                    acc += key[j2 as usize];
                }
                if let Some(&j3) = chunk.get(3) {
                    acc += key[j3 as usize];
                }
                if let Some(&j4) = chunk.get(4) {
                    acc += key[j4 as usize];
                }
                if let Some(&j5) = chunk.get(5) {
                    acc += key[j5 as usize];
                }
                if let Some(&j6) = chunk.get(6) {
                    acc += key[j6 as usize];
                }
                if let Some(&j7) = chunk.get(7) {
                    acc += key[j7 as usize];
                }
            }

            acc
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ec::CurveGroup;
    use ark_std::rand::RngCore;
    use ark_std::UniformRand;

    #[test]
    fn test_batch_addition_correctness() {
        let mut rng = ark_std::test_rng();

        let bases: Vec<G1Affine> = (0..10).map(|_| G1Affine::rand(&mut rng)).collect();

        let indices = vec![2, 3, 4, 5, 6, 7];

        let batch_result = batch_g1_additions(&bases, &indices);

        let mut expected = G1Affine::zero();
        for &idx in &indices {
            expected = (expected + bases[idx]).into_affine();
        }

        assert_eq!(batch_result, expected, "Batch addition mismatch");
    }

    #[test]
    fn test_empty_indices() {
        let bases: Vec<G1Affine> = vec![G1Affine::generator(); 5];
        let result = batch_g1_additions(&bases, &[]);
        assert_eq!(result, G1Affine::zero());
    }

    #[test]
    fn test_single_index() {
        let mut rng = ark_std::test_rng();
        let bases: Vec<G1Affine> = (0..5).map(|_| G1Affine::rand(&mut rng)).collect();

        let result = batch_g1_additions(&bases, &[2]);
        assert_eq!(result, bases[2]);
    }

    #[test]
    fn test_stress_test_correctness() {
        let mut rng = ark_std::test_rng();

        let base_size = 100000;
        let indices_size = 50000;

        let bases: Vec<G1Affine> = (0..base_size).map(|_| G1Affine::rand(&mut rng)).collect();

        let indices: Vec<usize> = (0..indices_size)
            .map(|_| (rng.next_u64() as usize) % base_size)
            .collect();

        let batch_result = batch_g1_additions(&bases, &indices);

        let mut expected = G1Affine::zero();
        for &idx in &indices {
            expected = (expected + bases[idx]).into_affine();
        }

        assert_eq!(
            batch_result, expected,
            "Stress test failed: batch result doesn't match expected sum"
        );
    }

    #[test]
    fn test_msm_rows_mixed_correctness() {
        let mut rng = ark_std::test_rng();

        let n = 100;
        let k = 50;

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

        let result = msm_rows_mixed_bn254(&key, &rows);

        for (row_idx, row) in rows.iter().enumerate() {
            let mut expected = G1Affine::zero();
            for &idx in row.iter() {
                expected = (expected + key[idx as usize]).into_affine();
            }
            assert_eq!(result[row_idx], expected, "Mismatch at row {}", row_idx);
        }
    }

    #[test]
    fn test_msm_rows_empty() {
        let key: Vec<G1Affine> = vec![];
        let rows: Vec<SmallRow> = vec![];

        let result = msm_rows_mixed_bn254(&key, &rows);
        assert!(result.is_empty());
    }

    #[test]
    fn test_msm_rows_single_element() {
        let mut rng = ark_std::test_rng();
        let key: Vec<G1Affine> = vec![G1Affine::rand(&mut rng)];
        let mut row: SmallRow<DEFAULT_MAX_ROW_CAPACITY> = SmallRow::new();
        row.push(0);
        let rows = vec![row];

        let result = msm_rows_mixed_bn254(&key, &rows);
        assert_eq!(result[0], key[0]);
    }

    #[test]
    fn test_msm_rows_stress_test() {
        let mut rng = ark_std::test_rng();

        let n = 1000;
        let k = 500;

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

        let result = msm_rows_mixed_bn254(&key, &rows);

        for row_idx in [0, n / 4, n / 2, 3 * n / 4, n - 1] {
            let mut expected = G1Affine::zero();
            for &idx in rows[row_idx].iter() {
                expected = (expected + key[idx as usize]).into_affine();
            }
            assert_eq!(
                result[row_idx], expected,
                "Stress test failed at row {}",
                row_idx
            );
        }
    }

    #[test]
    fn test_msm_rows_projective_correctness() {
        let mut rng = ark_std::test_rng();

        let n = 100;
        let k = 50;

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

        let proj_result = msm_rows_mixed_bn254_projective(&key, &rows, k);

        let result = G1Projective::normalize_batch(&proj_result);

        for (row_idx, row) in rows.iter().enumerate() {
            let mut expected = G1Affine::zero();
            for &idx in row.iter() {
                expected = (expected + key[idx as usize]).into_affine();
            }
            assert_eq!(
                result[row_idx], expected,
                "Projective MSM mismatch at row {}",
                row_idx
            );
        }
    }

    #[test]
    fn test_batch_additions_multi_large() {
        let mut rng = ark_std::test_rng();

        let base_size = 10000;
        let num_batches = 50;

        let bases: Vec<G1Affine> = (0..base_size).map(|_| G1Affine::rand(&mut rng)).collect();

        let indices_sets: Vec<Vec<usize>> = (0..num_batches)
            .map(|_| {
                let size = (rng.next_u64() as usize) % 100 + 1;
                (0..size)
                    .map(|_| (rng.next_u64() as usize) % base_size)
                    .collect()
            })
            .collect();

        let batch_results = batch_g1_additions_multi(&bases, &indices_sets);

        for (i, (result, indices)) in batch_results.iter().zip(indices_sets.iter()).enumerate() {
            let single_result = batch_g1_additions(&bases, indices);
            assert_eq!(
                *result, single_result,
                "Multi vs single mismatch at batch {}",
                i
            );
        }
    }
}
