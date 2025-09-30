#![allow(unused_imports)]
use ark_ec::{
    models::short_weierstrass::SWCurveConfig, // Keep this as G1 is SW
    pairing::Pairing,
    AffineRepr,
    CurveGroup,
    PrimeGroup,
};
use ark_ff::{Field, One, UniformRand, Zero};
use ark_std::{rand::Rng, test_rng};

// Add imports for the newly defined types
use crate::bn254::{Fq, FqConfig, Fr, FrConfig, G1Affine, G1Projective};

use ark_algebra_test_templates::*;
use ark_std::ops::{AddAssign, MulAssign, SubAssign};

// test_field!(fr; Fr; mont_prime_field);
// Uncomment Fq test
test_field!(fq; Fq; mont_prime_field);

// Uncomment G1 test for Short Weierstrass
test_group!(g1; G1Projective; sw);

// Add other tests for G2, Pairing etc. as needed
#[cfg(test)]
mod test {
    use ark_ff::{Field, UniformRand};
    use ark_std::test_rng;

    use crate::bn254::{Fq12, TorusCompressedFq12};

    #[test]
    fn test_compression() {
        let q3_minus_1: [u64; 24] = [
            0xf4e90a2716e3f810,
            0xbda8be6dec90ce20,
            0x725d28e938e58016,
            0xd65418b4cf130588,
            0xdd0e6a4e64b8148f,
            0x5408128686d835cb,
            0xa0903665952d6b92,
            0x6cc7afd0826c9a44,
            0xae01ee1f7c6ee657,
            0x8005dfa955bf9647,
            0x294d13656f6eb160,
            0x1e1342fb2628372f,
            0x28ec557b543fe50a,
            0x7e1c1d370decdf21,
            0x4661aaf35ddfdf5c,
            0xaebe67f05f148be1,
            0x64ed3d397a42c302,
            0x2ee8ac393e1f9708,
            0xea5ef61762dd07aa,
            0xd9df3dc41c5830ec,
            0x53500facde502233,
            0xf1f18061c3d30194,
            0x2a255aea70a6ec3a,
            0x2fd70ffd469f2,
        ];
        let mut rng = test_rng();

        let num_trials = 5;

        for _ in 0..num_trials {
            let fq12_ele = Fq12::rand(&mut rng);
            let compressed = fq12_ele.torus_compress_q_minus_one_pow();
            let decompressed = Fq12::torus_decompress(compressed);
            assert_eq!(fq12_ele.pow(q3_minus_1), decompressed);
        }
    }
}
