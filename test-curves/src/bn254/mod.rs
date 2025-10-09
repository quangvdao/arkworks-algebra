use crate::*;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub mod g1;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub use g1::*;

#[cfg(feature = "bn254")] // Use the main bn254 feature
pub mod g2;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub use g2::*;

#[cfg(feature = "bn254")] // Use the main bn254 feature
pub mod fr;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub use fr::*;

#[cfg(feature = "bn254")] // Use the main bn254 feature
pub mod fq;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub use fq::*;

#[cfg(feature = "bn254")] // Use the main bn254 feature
pub mod fq2;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub use fq2::*;

#[cfg(feature = "bn254")] // Use the main bn254 feature
pub mod fq6;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub use fq6::*;

#[cfg(feature = "bn254")] // Use the main bn254 feature
pub mod fq12;
#[cfg(feature = "bn254")] // Use the main bn254 feature
pub use fq12::*;

use ark_ec::{
    bn::{
        self, pow_sixth_cyclotomic_polynomial_over_r, raise_to_psi_six_pow, Bn, BnConfig,
        G1Prepared, G2Prepared, TwistType,
    },
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use ark_ff::MontFp;

#[cfg(test)]
mod test;

pub struct Config;
pub struct CompressibleConfig;

impl BnConfig for Config {
    const X: &'static [u64] = &[4965661367192848881];
    /// `x` is positive.
    const X_IS_NEGATIVE: bool = false;
    const ATE_LOOP_COUNT: &'static [i8] = &[
        0, 0, 0, 1, 0, 1, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, -1, 0, -1, 0, 0, 0, 1, 0, -1, 0, 0, 0,
        0, -1, 0, 0, 1, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, -1, 0,
        -1, 0, 0, 0, 1, 0, 1, 1,
    ];

    const TWIST_MUL_BY_Q_X: Fq2 = Fq2::new(
        MontFp!("21575463638280843010398324269430826099269044274347216827212613867836435027261"),
        MontFp!("10307601595873709700152284273816112264069230130616436755625194854815875713954"),
    );
    const TWIST_MUL_BY_Q_Y: Fq2 = Fq2::new(
        MontFp!("2821565182194536844548159561693502659359617185244120367078079554186484126554"),
        MontFp!("3505843767911556378687030309984248845540243509899259641013678093033130930403"),
    );
    const TWIST_TYPE: TwistType = TwistType::D;
    type Fp = Fq;
    type Fp2Config = Fq2Config;
    type Fp6Config = Fq6Config;
    type Fp12Config = Fq12Config;
    type G1Config = g1::Config;
    type G2Config = g2::Config;
}

const D_PRIME: [u64; 15] = [
    0xcaa4152366144ab4,
    0x114dc0ec2cab7ffd,
    0x0cf0888c7a0ff6cf,
    0x65c5644e949b6a90,
    0x1be2458885117085,
    0x5b35eb719e58db4b,
    0x2566c550aeb7e0e2,
    0x0c974024a316619f,
    0xd147cb7d3a5203dc,
    0x621d9bfed77c2ad0,
    0x26473fbcd1c3ec1e,
    0xe86518527b5e4036,
    0x29259e9712ca7b71,
    0x1891045f68d15763,
    0x679dd974c68787,
];

impl BnConfig for CompressibleConfig {
    const X: &'static [u64] = &[4965661367192848881];
    /// `x` is positive.
    const X_IS_NEGATIVE: bool = false;
    const ATE_LOOP_COUNT: &'static [i8] = &[
        0, 0, 0, 1, 0, 1, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, -1, 0, -1, 0, 0, 0, 1, 0, -1, 0, 0, 0,
        0, -1, 0, 0, 1, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, -1, 0,
        -1, 0, 0, 0, 1, 0, 1, 1,
    ];

    const TWIST_MUL_BY_Q_X: Fq2 = Fq2::new(
        MontFp!("21575463638280843010398324269430826099269044274347216827212613867836435027261"),
        MontFp!("10307601595873709700152284273816112264069230130616436755625194854815875713954"),
    );
    const TWIST_MUL_BY_Q_Y: Fq2 = Fq2::new(
        MontFp!("2821565182194536844548159561693502659359617185244120367078079554186484126554"),
        MontFp!("3505843767911556378687030309984248845540243509899259641013678093033130930403"),
    );
    const TWIST_TYPE: TwistType = TwistType::D;
    type Fp = Fq;
    type Fp2Config = Fq2Config;
    type Fp6Config = Fq6Config;
    type Fp12Config = CompressibleFq12Config;
    type G1Config = g1::Config;
    type G2Config = g2::Config;

    // fn final_exponentiation(f: MillerLoopOutput<Bn<Self>>) -> Option<PairingOutput<Bn<Self>>> {
    //     // let f = raise_to_sixth_cyclotomic_polynomial::<Self>(f.0);
    //     raise_to_psi_six_pow::<Self>(f.0)
    //         .map(|f| f.pow(D_PRIME))
    //         .map(PairingOutput)
    // }
}

pub type Bn254 = Bn<Config>;
pub type CompressibleBn254 = Bn<CompressibleConfig>;

pub type G1Affine = bn::G1Affine<Config>;
pub type G1Projective = bn::G1Projective<Config>;
pub type G2Affine = bn::G2Affine<Config>;
pub type G2Projective = bn::G2Projective<Config>;

pub fn fq12_compressed_pairing(
    a: impl Into<G1Prepared<Config>>,
    b: impl Into<G2Prepared<Config>>,
) -> CompressedFq12 {
    fq12_compressed_multi_pairing([a], [b])
}

pub fn fq12_compressed_multi_pairing(
    a: impl IntoIterator<Item = impl Into<G1Prepared<Config>>>,
    b: impl IntoIterator<Item = impl Into<G2Prepared<Config>>>,
) -> CompressedFq12 {
    let miller_loop_output = Bn254::multi_miller_loop(a, b);
    let pow = pow_sixth_cyclotomic_polynomial_over_r::<Config>(miller_loop_output.0);
    torus_compress_psi_6_pow_to_two_fq2(fq12_to_compressible_fq12(pow))
}
