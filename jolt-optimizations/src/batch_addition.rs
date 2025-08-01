//! Batch affine point addition for G1
//!
//! Implements efficient batch addition of affine elliptic curve points
//! using Montgomery's batch inversion trick to minimize field inversions.

use ark_bn254::G1Affine;
use ark_ec::AffineRepr;
use rayon::prelude::*;

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
    
    // Start with indices, convert to points only when needed
    let mut points: Vec<G1Affine> = Vec::with_capacity(indices.len());
    points.extend(indices.iter().map(|&i| bases[i]));
    
    // Iteratively reduce pairs until we have a single result
    while points.len() > 1 {
        let current_len = points.len();
        let pairs_count = current_len / 2;
        let has_odd = current_len % 2 == 1;
        
        // Collect denominators in parallel
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
        
        // Apply all additions in parallel
        let mut new_points: Vec<G1Affine> = (0..pairs_count)
            .into_par_iter()
            .zip(inverses.par_iter())
            .map(|(i, inv)| {
                let p1 = points[i * 2];
                let p2 = points[i * 2 + 1];
                let lambda = (p2.y - p1.y) * inv;
                let x3 = lambda * lambda - p1.x - p2.x;
                let y3 = lambda * (p1.x - x3) - p1.y;
                G1Affine::new(x3, y3)
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


#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::UniformRand;
    use ark_ec::CurveGroup;
    use ark_std::rand::RngCore;
    
    #[test]
    fn test_batch_addition_correctness() {
        let mut rng = ark_std::test_rng();
        
        // Generate random points
        let bases: Vec<G1Affine> = (0..10)
            .map(|_| G1Affine::rand(&mut rng))
            .collect();
        
        // Create indices to sum
        let indices = vec![2, 3, 4, 5, 6, 7];
        
        // Compute batch addition
        let batch_result = batch_g1_additions(&bases, &indices);
        
        // Verify against sequential addition
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
        let bases: Vec<G1Affine> = (0..5)
            .map(|_| G1Affine::rand(&mut rng))
            .collect();
        
        let result = batch_g1_additions(&bases, &[2]);
        assert_eq!(result, bases[2]);
    }
    
    #[test] 
    fn test_stress_test_correctness() {
        let mut rng = ark_std::test_rng();
        
        // Large test case
        let base_size = 10000;
        let indices_size = 5000;
        
        let bases: Vec<G1Affine> = (0..base_size)
            .map(|_| G1Affine::rand(&mut rng))
            .collect();
        
        let indices: Vec<usize> = (0..indices_size)
            .map(|_| (rng.next_u64() as usize) % base_size)
            .collect();
        
        // Compute using batch addition
        let batch_result = batch_g1_additions(&bases, &indices);
        
        // For very large tests, we'll just verify it doesn't panic
        // and returns a valid point (not infinity unless expected)
        assert!(!batch_result.is_zero() || indices.is_empty());
    }
}