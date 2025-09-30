use ark_ff::{fields::*, MontFp};

use crate::bn254::*;

pub type Fq12 = Fp12<Fq12Config>;

#[derive(Clone, Copy)]
pub struct Fq12Config;

// q^2 + q + 1 where q = p^2. Using the notation currently in use in the module, it is q^4 + q^2 + 1 where q = p.
static Q4_PLUS_Q2_PLUS_1: [u64; 16] = [
    0x3e6d64f00b9a1613,
    0xeca692dee2d53c2e,
    0x236c9768ba60d0a8,
    0xea49ac953ebcd257,
    0x5588ca24314827a1,
    0x75f41c5c0ee9597a,
    0x75fc6a062a899806,
    0x2b3dd32423ab1f23,
    0x7e1b009439ceba33,
    0xca425189b6172413,
    0x4f97cc2276924233,
    0xea401bebaf1b1332,
    0x06f6feb7b4e30336,
    0x562e001117c18136,
    0x94d5ab7ebe19457b,
    0x53ad676ccd6cff,
];

static Q: [u64; 8] = [
    0x3b5458a2275d69b1,
    0xa602072d09eac101,
    0x4a50189c6d96cadc,
    0x04689e957a1242c8,
    0x26edfa5c34c6b38d,
    0xb00b855116375606,
    0x599a6f7c0348d21c,
    0x925c4b8763cbf9c,
];

// Implement the compression method in Proposition 1 of https://eprint.iacr.org/2007/429.pdf.
#[derive(Clone, Copy)]
pub struct TorusCompressedFq12(pub (Fq2, Fq2));

impl TorusCompressedFq12 {
    /// Compute the torus compressed form of element^(\psi_6(q^2)), where \psi_6 is the x^6 - 1 divided by the sixth cyclotomic polynomial.
    /// p.10 Proposition 1 https://eprint.iacr.org/2007/429.pdf.
    pub fn compress_psi_six_pow(element: Fq12) -> TorusCompressedFq12 {
        let ele_pow = element.pow(Q4_PLUS_Q2_PLUS_1);
        let a_tilde = ele_pow.torus_compress_q_minus_one_pow();
        let a_tilde_q_pow = a_tilde.pow(Q);
        let compressed: Fq6 = Fq12::mul_torus_compressed_elements(a_tilde_q_pow, a_tilde);
        TorusCompressedFq12((compressed.c0, compressed.c1))
    }

    pub fn decompress(compressed: TorusCompressedFq12) -> Fq12 {
        let b0 = Fq6::new(compressed.0 .0, Fq2::ZERO, Fq2::ZERO);
        let b1 = Fq6::new(Fq2::ZERO, compressed.0 .1, Fq2::ZERO);
        let b2 = (Fq6::from(3) * b0.pow([2 as u64]) + Fq12Config::NONRESIDUE)
            / (Fq6::from(3) * b1 * Fq12Config::NONRESIDUE);

        #[cfg(test)]
        {
            assert_eq!(b2.c0, Fq2::ZERO);
            assert_eq!(b2.c2, Fq2::ZERO);
        }

        let beta = Fp6::new(b0.c0, b1.c1, b2.c2);
        Fq12::torus_decompress(beta)
    }
}

impl Fp12Config for Fq12Config {
    type Fp6Config = Fq6Config;

    const NONRESIDUE: Fq6 = Fq6::new(Fq2::ZERO, Fq2::ONE, Fq2::ZERO);

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        Fq2::new(Fq::ONE, Fq::ZERO),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        Fq2::new(
            MontFp!("8376118865763821496583973867626364092589906065868298776909617916018768340080"),
            MontFp!(
                "16469823323077808223889137241176536799009286646108169935659301613961712198316"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        Fq2::new(
            MontFp!(
                "21888242871839275220042445260109153167277707414472061641714758635765020556617"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        Fq2::new(
            MontFp!(
                "11697423496358154304825782922584725312912383441159505038794027105778954184319"
            ),
            MontFp!("303847389135065887422783454877609941456349188919719272345083954437860409601"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        Fq2::new(
            MontFp!(
                "21888242871839275220042445260109153167277707414472061641714758635765020556616"
            ),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        Fq2::new(
            MontFp!("3321304630594332808241809054958361220322477375291206261884409189760185844239"),
            MontFp!("5722266937896532885780051958958348231143373700109372999374820235121374419868"),
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        Fq2::new(MontFp!("-1"), Fq::ZERO),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        Fq2::new(
            MontFp!(
                "13512124006075453725662431877630910996106405091429524885779419978626457868503"
            ),
            MontFp!("5418419548761466998357268504080738289687024511189653727029736280683514010267"),
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        Fq2::new(
            MontFp!("2203960485148121921418603742825762020974279258880205651966"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        Fq2::new(
            MontFp!(
                "10190819375481120917420622822672549775783927716138318623895010788866272024264"
            ),
            MontFp!(
                "21584395482704209334823622290379665147239961968378104390343953940207365798982"
            ),
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        Fq2::new(
            MontFp!("2203960485148121921418603742825762020974279258880205651967"),
            Fq::ZERO,
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
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
