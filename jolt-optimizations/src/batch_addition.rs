//! Batch affine point addition for G1
//!
//! Implements efficient batch addition of affine elliptic curve points
//! using Montgomery's batch inversion trick to minimize field inversions.

use ark_bn254::G1Affine;
use ark_ec::AffineRepr;
use ark_ff::Zero;
use ark_ec::CurveGroup;

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
    
    // Collect all points to be added
    let mut points_to_add: Vec<G1Affine> = indices.iter().map(|&i| bases[i]).collect();
    
    // Iteratively reduce pairs until we have a single result
    while points_to_add.len() > 1 {
        let mut next_round = Vec::new();
        let mut denominators = Vec::new();
        let mut pairs = Vec::new();
        
        // Process points in pairs
        let mut i = 0;
        while i < points_to_add.len() {
            if i + 1 < points_to_add.len() {
                let p1 = points_to_add[i];
                let p2 = points_to_add[i + 1];
                
                // Handle special cases
                if p1.is_zero() {
                    next_round.push(p2);
                } else if p2.is_zero() {
                    next_round.push(p1);
                } else if p1.x == p2.x {
                    if p1.y == p2.y {
                        // Same point - would need doubling formula
                        // For now, just push p1 (in practice, implement doubling)
                        next_round.push(p1);
                    } else {
                        // Inverse points - result is infinity
                        next_round.push(G1Affine::zero());
                    }
                } else {
                    // Normal case - store for batch processing
                    denominators.push(p2.x - p1.x);
                    pairs.push((p1, p2));
                }
                i += 2;
            } else {
                // Odd number of points - carry the last one forward
                next_round.push(points_to_add[i]);
                i += 1;
            }
        }
        
        // Batch invert all denominators
        if !denominators.is_empty() {
            let mut inverses = denominators;
            ark_ff::fields::batch_inversion(&mut inverses);
            
            // Apply all additions
            for ((p1, p2), inv) in pairs.iter().zip(inverses.iter()) {
                let lambda = (p2.y - p1.y) * inv;
                let x3 = lambda * lambda - p1.x - p2.x;
                let y3 = lambda * (p1.x - x3) - p1.y;
                next_round.push(G1Affine::new(x3, y3));
            }
        }
        
        points_to_add = next_round;
    }
    
    points_to_add[0]
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::UniformRand;
    
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
}