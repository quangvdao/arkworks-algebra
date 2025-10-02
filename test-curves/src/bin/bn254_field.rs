use ark_ff::{AdditiveGroup, Field, Fp2Config, Fp6Config, MontFp, UniformRand};
use ark_test_curves::bn254::{Fq, Fq2, Fq6Config};

static Q_MINUS_ONE_DIV_SIX: [u64; 8] = [
    0x348e0ec5b13a3c48,
    0xc655abdcd6fc7580,
    0x0c62aec4bcee7724,
    0x2b66c518e9adb5cc,
    0x5bd25464b3767342,
    0x72ac96382e5e8e56,
    0x0eef1294ab36cdaf,
    0x1864b7413b4ca9a,
];

static P2_MINUS_ONE_DIV_THREE: [u64; 8] = [
    0x691c1d8b62747890,
    0x8cab57b9adf8eb00,
    0x18c55d8979dcee49,
    0x56cd8a31d35b6b98,
    0xb7a4a8c966ece684,
    0xe5592c705cbd1cac,
    0x1dde2529566d9b5e,
    0x30c96e827699534,
];

// static Q2_MINUS_ONE_DIV_TWO: [u64; 16] = [
//     0x818c8626f21e5630,
//     0x235245d8ec753d96,
//     0x6c8e3f66266502e6,
//     0x72f086ffe25547c7,
//     0x174d67e3fe40ba0a,
//     0xe2f44b857c5901ba,
//     0x8e30fd4513a062f4,
//     0x910c0735d6b72fc3,
//     0xbf0d804a1ce75d19,
//     0xe52128c4db0b9209,
//     0x27cbe6113b492119,
//     0x75200df5d78d8999,
//     0x037b7f5bda71819b,
//     0xab1700088be0c09b,
//     0xca6ad5bf5f0ca2bd,
//     0x29d6b3b666b67f,
// ];

static Q_MINUS_ONE_DIV_TWO: [u64; 8] = [
    0x9daa2c5113aeb4d8,
    0x5301039684f56080,
    0x25280c4e36cb656e,
    0x82344f4abd092164,
    0x1376fd2e1a6359c6,
    0x5805c2a88b1bab03,
    0x2ccd37be01a4690e,
    0x492e25c3b1e5fce,
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

static P3_MINUS_ONE_DIV_TWO: [u64; 12] = [
    0xdcd94cc1630c1e8b,
    0x04b908c1c4d95181,
    0xa19b3bf1e2b399e1,
    0xdfcfc974c8ecc167,
    0x4fbe1d3b45e55ff0,
    0x37884f86bed5233a,
    0x0290c87382ac9873,
    0x7335c9e6c15d7689,
    0xe1210e1b96f716ee,
    0xb40a47e972f243f8,
    0x998f60a8c18bbfd7,
    0xdd55388583acd6,
];

// static Q3_MINUS_ONE_DIV_TWO: [u64; 24] = [
//     0x7a7485138b71fc08,
//     0x5ed45f36f6486710,
//     0x392e94749c72c00b,
//     0xeb2a0c5a678982c4,
//     0xee873527325c0a47,
//     0x2a040943436c1ae5,
//     0x50481b32ca96b5c9,
//     0xb663d7e841364d22,
//     0xd700f70fbe37732b,
//     0x4002efd4aadfcb23,
//     0x94a689b2b7b758b0,
//     0x0f09a17d93141b97,
//     0x94762abdaa1ff285,
//     0x3f0e0e9b86f66f90,
//     0xa330d579aeefefae,
//     0x575f33f82f8a45f0,
//     0x32769e9cbd216181,
//     0x1774561c9f0fcb84,
//     0x752f7b0bb16e83d5,
//     0xecef9ee20e2c1876,
//     0x29a807d66f281119,
//     0x78f8c030e1e980ca,
//     0x1512ad753853761d,
//     0x17eb87fea34f9,
// ];

static Q6_MINUS_ONE_DIV_TWO: [u64; 48] = [
    0xe0ea716965437890,
    0xa0df8735834557ed,
    0x11e2d8acfc719a7b,
    0xed9fa0abea7a60d3,
    0x2e89c86856a266f1,
    0x5ef18761d60cf29d,
    0xf4b3dbb9b2d65fdb,
    0xeeee1e6abbc558ba,
    0x8a4eed3a1633f9f5,
    0x807fdb96958dea84,
    0x9049783e5571ac86,
    0x2caa5b432e494f8a,
    0xdf81d3f5622c32e5,
    0xb56642ff98286c22,
    0x827b9a20163f4e43,
    0xfd7fee69f21057af,
    0x1cdcf3e19a3375e2,
    0x4d35ef85f395cddb,
    0x5271da1cbc14c023,
    0x93d40726dea36bc6,
    0x7dd9ec8df3dbffa4,
    0x55d6f338a6b45508,
    0x5ac8a6b06c58a809,
    0x6eb005d2ab613b0e,
    0x649d42e65b2917dd,
    0x85a04ee843756313,
    0x1c1774e472981197,
    0x8e70e4a80552adc8,
    0x661389dfc2613cec,
    0xf553d250b68d02ee,
    0x8d3d512bb3e0e762,
    0x10cdc5086edf0891,
    0xd291bacf130a0456,
    0x72ffb3a3af8f9d6e,
    0xe4877dca1ce5d302,
    0x08e5dc806d6aae46,
    0x5c7638d4152fd444,
    0xe96678a0a0eb7cd0,
    0xa6d67c874709ce34,
    0x1c07ed9495b4b546,
    0x882355934258075b,
    0xa6da0d9152dcdd4f,
    0xe0f3625e7f551c69,
    0x25fa1fbfac99ec86,
    0x3fa2389474020ed2,
    0x16614bc9fd4e3a9d,
    0x2d5b6f8c1b78bb86,
    0x47856456e,
];

fn find_sextic_non_residue_over_fq2() -> Fq2 {
    let mut rng = ark_std::test_rng();
    for i in (1..100000).chain(std::iter::once(-1)) {
        let candidate = Fq2::new(Fq::from(i), Fq::from(i));
        let candidate = Fq2::rand(&mut rng);

        let val = candidate.pow(Q_MINUS_ONE_DIV_TWO);
        if val != Fq2::ONE {
            println!("right candidate = {:?}", candidate);
            break;
        }
        println!("val = {:?}", val);
        println!("candidate = {:?}", candidate);
        panic!("--------------------------------");
    }
    let mut rng = ark_std::test_rng();
    let fq2_non_quad_residue = (0..100000)
        .into_iter()
        .find_map(|candidate| {
            // let candidate = Fq2::new(Fq::ONE, Fq::from(candidate));
            let candidate = Fq2::rand(&mut rng);
            if candidate.pow(Q_MINUS_ONE_DIV_TWO) != Fq2::ONE {
                println!("candidate = {:?}", candidate);
                Some(candidate)
            } else {
                None
            }
        })
        .unwrap();
    let fq2_non_cubic_residue = <Fq6Config as Fp6Config>::NONRESIDUE;
    println!("fq2_non_cubic_residue = {:?}", fq2_non_cubic_residue);
    println!("fq2_non_quad_residue = {:?}", fq2_non_quad_residue);

    let pow3 = fq2_non_cubic_residue.pow(&P2_MINUS_ONE_DIV_THREE);
    println!("pow3 = {:?}", pow3);
    let pow2 = fq2_non_quad_residue.pow(&Q_MINUS_ONE_DIV_TWO);
    println!("pow2 = {:?}", pow2);

    (fq2_non_cubic_residue.square() * fq2_non_quad_residue.square() * fq2_non_quad_residue)
        .inverse()
        .unwrap()
}

fn main() {
    let fq6_modulus = <Fq6Config as Fp6Config>::NONRESIDUE;
    let sextic_non_residue_over_fq2 = find_sextic_non_residue_over_fq2();
    println!(
        "sextic_non_residue_over_fq2 = {:?}",
        sextic_non_residue_over_fq2
    );
    let pow6 = sextic_non_residue_over_fq2
        // .pow([6u64])
        .pow(&Q_MINUS_ONE_DIV_SIX);
    println!("pow6 = {:?}", pow6);
    let val = Fq2::new(MontFp!("9"), Fq::ONE);
    println!(
        "val pow is one? {:?}",
        val.pow(&Q_MINUS_ONE_DIV_SIX) == Fq2::ONE
    );
    println!("val = {:?}", val);
    println!(
        "sextic_non_residue_over_fq2 = {:?}",
        sextic_non_residue_over_fq2
    );
    let quad_non_residue = val.pow(&Q_MINUS_ONE_DIV_SIX).sqrt().unwrap();
    println!("quad_non_residue = {:?}", quad_non_residue);
}
