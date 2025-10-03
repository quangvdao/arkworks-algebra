use ark_ff::{AdditiveGroup, Field, Fp12, Fp12Config, Fp6Config, MontFp};

use crate::bn254::{Fq, Fq2, Fq6, Fq6Config};

pub type Fq12 = Fp12<Fq12Config>;
pub type CompressibleFq12 = Fp12<CompressibleFq12Config>;

pub fn fq12_to_compressible_fq12(value: Fq12) -> CompressibleFq12 {
    // Divide by the generator of Fq6
    let new_c1 = Fq6 {
        c0: value.c1.c1,
        c1: value.c1.c2,
        c2: -value.c1.c0 * Fq6Config::NONRESIDUE.inverse().unwrap(),
    };

    CompressibleFq12 {
        c0: value.c0,
        c1: new_c1,
    }
}

pub fn compressible_fq12_to_fq12(value: CompressibleFq12) -> Fq12 {
    // Multiply by the generator of Fq6
    let new_c1 = Fq6 {
        c0: -value.c1.c2 * Fq6Config::NONRESIDUE,
        c1: value.c1.c0,
        c2: value.c1.c1,
    };

    Fq12 {
        c0: value.c0,
        c1: new_c1,
    }
}

static COMPRESSIBLE_FROBENIUS_COEFFS: [Fq2; 4] = [
    Fq2::new(Fq::ONE, Fq::ZERO),
    Fq2::new(
        MontFp!("2821565182194536844548159561693502659359617185244120367078079554186484126554"),
        MontFp!("3505843767911556378687030309984248845540243509899259641013678093033130930403"),
    ),
    Fq2::new(
        MontFp!("21888242871839275222246405745257275088696311157297823662689037894645226208582"),
        MontFp!("0"),
    ),
    Fq2::new(
        MontFp!("19066677689644738377698246183563772429336693972053703295610958340458742082029"),
        MontFp!("18382399103927718843559375435273026243156067647398564021675359801612095278180"),
    ),
];

#[derive(Clone, Copy)]
pub struct Fq12Config;

// Implement the compression method in Proposition 1 of https://eprint.iacr.org/2007/429.pdf.

#[derive(Clone, Copy)]
pub struct CompressibleFq12Config;

impl Fp12Config for CompressibleFq12Config {
    type Fp6Config = Fq6Config;

    // The 12th degree extension is generated as a quadratic extension over the 6th degree extension. Another way to think about this is that the field as a 12th deg extension over the base field is really the composite field of a quadratic extension and a cubic extension, with generators sqrt(\gamma) and cbrt(\gamma), respectively, where \gamma is a sextic non-residue in the base field (itself a second deg extension over the base field on which the bn254 curve is defined). Therefore, the quadratic non-residue that generates the 12th degree extension over the 6th deg base field is \gamma = Fq6::non_residue.
    const NONRESIDUE: Fq6 = Fq6::new(<Fq6Config as Fp6Config>::NONRESIDUE, Fq2::ZERO, Fq2::ZERO);

    // TODO: need to implement this
    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        COMPRESSIBLE_FROBENIUS_COEFFS[0],
        COMPRESSIBLE_FROBENIUS_COEFFS[1],
        COMPRESSIBLE_FROBENIUS_COEFFS[2],
        COMPRESSIBLE_FROBENIUS_COEFFS[3],
        COMPRESSIBLE_FROBENIUS_COEFFS[0],
        COMPRESSIBLE_FROBENIUS_COEFFS[1],
        COMPRESSIBLE_FROBENIUS_COEFFS[2],
        COMPRESSIBLE_FROBENIUS_COEFFS[3],
        COMPRESSIBLE_FROBENIUS_COEFFS[0],
        COMPRESSIBLE_FROBENIUS_COEFFS[1],
        COMPRESSIBLE_FROBENIUS_COEFFS[2],
        COMPRESSIBLE_FROBENIUS_COEFFS[3],
    ];

    fn mul_fp6_by_nonresidue_in_place(fe: &mut Fq6) -> &mut Fq6 {
        Fq6Config::mul_fp2_by_nonresidue_in_place(&mut fe.c0);
        Fq6Config::mul_fp2_by_nonresidue_in_place(&mut fe.c1);
        Fq6Config::mul_fp2_by_nonresidue_in_place(&mut fe.c2);
        fe
    }
}

impl Fp12Config for Fq12Config {
    type Fp6Config = Fq6Config;

    const NONRESIDUE: Fq6 = Fq6::new(Fq2::ZERO, Fq2::ONE, Fq2::ZERO);

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fq6::NONRESIDUE^(((q^0) - 1) / 6)
        Fq2::new(Fq::ONE, Fq::ZERO),
        // Fq6::NONRESIDUE^(((q^1) - 1) / 6)
        Fq2::new(
            MontFp!("8376118865763821496583973867626364092589906065868298776909617916018768340080"),
            MontFp!(
                "16469823323077808223889137241176536799009286646108169935659301613961712198316"
            ),
        ),
        // Fq6::NONRESIDUE^(((q^2) - 1) / 6)
        Fq2::new(
            MontFp!(
                "21888242871839275220042445260109153167277707414472061641714758635765020556617"
            ),
            Fq::ZERO,
        ),
        // Fq6::NONRESIDUE^(((q^3) - 1) / 6)
        Fq2::new(
            MontFp!(
                "11697423496358154304825782922584725312912383441159505038794027105778954184319"
            ),
            MontFp!("303847389135065887422783454877609941456349188919719272345083954437860409601"),
        ),
        // Fq6::NONRESIDUE^(((q^4) - 1) / 6)
        Fq2::new(
            MontFp!(
                "21888242871839275220042445260109153167277707414472061641714758635765020556616"
            ),
            Fq::ZERO,
        ),
        // Fq6::NONRESIDUE^(((q^5) - 1) / 6)
        Fq2::new(
            MontFp!("3321304630594332808241809054958361220322477375291206261884409189760185844239"),
            MontFp!("5722266937896532885780051958958348231143373700109372999374820235121374419868"),
        ),
        // Fq6::NONRESIDUE^(((q^6) - 1) / 6)
        Fq2::new(MontFp!("-1"), Fq::ZERO),
        // Fq6::NONRESIDUE^(((q^7) - 1) / 6)
        Fq2::new(
            MontFp!(
                "13512124006075453725662431877630910996106405091429524885779419978626457868503"
            ),
            MontFp!("5418419548761466998357268504080738289687024511189653727029736280683514010267"),
        ),
        // Fq6::NONRESIDUE^(((q^8) - 1) / 6)
        Fq2::new(
            MontFp!("2203960485148121921418603742825762020974279258880205651966"),
            Fq::ZERO,
        ),
        // Fq6::NONRESIDUE^(((q^9) - 1) / 6)
        Fq2::new(
            MontFp!(
                "10190819375481120917420622822672549775783927716138318623895010788866272024264"
            ),
            MontFp!(
                "21584395482704209334823622290379665147239961968378104390343953940207365798982"
            ),
        ),
        // Fq6::NONRESIDUE^(((q^10) - 1) / 6)
        Fq2::new(
            MontFp!("2203960485148121921418603742825762020974279258880205651967"),
            Fq::ZERO,
        ),
        // Fq6::NONRESIDUE^(((q^11) - 1) / 6)
        Fq2::new(
            MontFp!(
                "18566938241244942414004596690298913868373833782006617400804628704885040364344"
            ),
            MontFp!(
                "16165975933942742336466353786298926857552937457188450663314217659523851788715"
            ),
        ),
    ];
}
