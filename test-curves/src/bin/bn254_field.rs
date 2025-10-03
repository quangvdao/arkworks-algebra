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
static Q_MINUS_1_DIV_6: [u64; 8] = [
    0x348e0ec5b13a3c48,
    0xc655abdcd6fc7580,
    0x0c62aec4bcee7724,
    0x2b66c518e9adb5cc,
    0x5bd25464b3767342,
    0x72ac96382e5e8e56,
    0x0eef1294ab36cdaf,
    0x1864b7413b4ca9a,
];
static P_MINUS_1_DIV_6: [u64; 4] = [
    0x34b017592414d4e1,
    0xee9591c2e6bda1c2,
    0xf40d60f3c0403964,
    0x810b7bdd032f006,
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

fn compute_frobenius_coeffs_for_compressible_fq12() {
    let pows = [
        vec![0x0],
        vec![
            0x9e10460b6c3e7ea3,
            0xcbc0b548b438e546,
            0xdc2822db40c0ac2e,
            0x183227397098d014,
        ],
        vec![
            0x9daa2c5113aeb4d8,
            0x5301039684f56080,
            0x25280c4e36cb656e,
            0x82344f4abd092164,
            0x1376fd2e1a6359c6,
            0x5805c2a88b1bab03,
            0x2ccd37be01a4690e,
            0x492e25c3b1e5fce,
        ],
        vec![
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
        ],
        vec![
            0x818c8626f21e5630,
            0x235245d8ec753d96,
            0x6c8e3f66266502e6,
            0x72f086ffe25547c7,
            0x174d67e3fe40ba0a,
            0xe2f44b857c5901ba,
            0x8e30fd4513a062f4,
            0x910c0735d6b72fc3,
            0xbf0d804a1ce75d19,
            0xe52128c4db0b9209,
            0x27cbe6113b492119,
            0x75200df5d78d8999,
            0x037b7f5bda71819b,
            0xab1700088be0c09b,
            0xca6ad5bf5f0ca2bd,
            0x29d6b3b666b67f,
        ],
        vec![
            0xaf827faacd15d5f3,
            0xe2fa27c3afc5957d,
            0x3c71f021205be243,
            0x070bd0cabf406e54,
            0x8b83f3f47794e6d1,
            0x52c30a9bfebc9d9e,
            0x119e2679efc3f9e9,
            0x8591a8bbef6382f3,
            0xe05a29436a1a839d,
            0x7da893a31993115e,
            0xf7eb9c382f8e30ca,
            0xac7b25407b86b593,
            0x0fa8face9763d0f9,
            0xcb4a9dbe7467402f,
            0xac2c57a8b4fd20e5,
            0xf5e1dda611ad7e74,
            0xfd6a1c2ce1b9b30a,
            0x0a74fb035d2af1bf,
            0xfdef52ab611ccc72,
            0x7e8a66297adc1,
        ],
        vec![
            0x7a7485138b71fc08,
            0x5ed45f36f6486710,
            0x392e94749c72c00b,
            0xeb2a0c5a678982c4,
            0xee873527325c0a47,
            0x2a040943436c1ae5,
            0x50481b32ca96b5c9,
            0xb663d7e841364d22,
            0xd700f70fbe37732b,
            0x4002efd4aadfcb23,
            0x94a689b2b7b758b0,
            0x0f09a17d93141b97,
            0x94762abdaa1ff285,
            0x3f0e0e9b86f66f90,
            0xa330d579aeefefae,
            0x575f33f82f8a45f0,
            0x32769e9cbd216181,
            0x1774561c9f0fcb84,
            0x752f7b0bb16e83d5,
            0xecef9ee20e2c1876,
            0x29a807d66f281119,
            0x78f8c030e1e980ca,
            0x1512ad753853761d,
            0x17eb87fea34f9,
        ],
        vec![
            0x61a53fd392cf4cdb,
            0xf1edf11f178eca40,
            0x07192cf8c44db8e1,
            0xf962928aebbda20c,
            0xb9cdd3790b9997a8,
            0xed7abe6426015fa4,
            0xf2be4beb9c3502af,
            0x34173334eef4e3a7,
            0x4b18c6b35e85cfdb,
            0x161ccf83c5629892,
            0x3629422a052a9370,
            0x0a06cf736c91fbc2,
            0xc7aa6bcec037ad59,
            0xc2f37818afeedbad,
            0x3fe01e514129a2e1,
            0x1edbe94b4ec880c9,
            0x127361e9f9d38259,
            0xe31caf1013fc5744,
            0xbc5ef425712b18d5,
            0x2d320d239504fbdc,
            0x27a5f901817e0595,
            0x7380db53058d8888,
            0xc8c92713e19554ff,
            0x702d26af95900ade,
            0xd9d1e5902e84ea77,
            0x70ff466e8f3f68bc,
            0x10de3a975a261ab8,
            0x48588d55d738,
        ],
        vec![
            0x5c8b75e404c53e60,
            0xbfcb0db14bba9a8b,
            0xf9ce3d677bf3f8cd,
            0xae08a0c6a86aa2c2,
            0x0f9196699c7bf1c3,
            0x1eef40d40a548af0,
            0x144f0d45d5b32ede,
            0x73135f55b02009f0,
            0x33f904f44cb2e9d1,
            0x4dd0387dd89f5444,
            0x42d8b51547f0464f,
            0x2656913b060e2360,
            0xe483ab567cc81811,
            0xdc72b7b13468df97,
            0x6e322eb109bfe9b9,
            0x38e4efd971ee54e7,
            0x6cfa52c90d58b38d,
            0x86b0e84948f021d5,
            0x6196c335c4e91bc4,
            0x4962611c0dc16b36,
            0xf23e7bd3dcb0ca1a,
            0x7c0f978ca1365a9e,
            0x10f04aa307fc3c65,
            0x1514f2d7479c4b89,
            0xc655d68984be1e3b,
            0xa69512ff832f5c81,
            0xd1efab7fd7f82d41,
            0x96ad8433e8ca0450,
            0x911f556fcd9c0e38,
            0xf4404e162880004b,
            0x874bd017939cee79,
            0xdacf342c005,
        ],
        vec![
            0x2dc8e96ae417ab43,
            0x8a61c4dcc39abdc3,
            0x7f25d37dd776c350,
            0x7f8c07b6aafb9c68,
            0xc749081f53f366d0,
            0xbed98424d4d352a8,
            0x43af0b7ac851291f,
            0x21668b5f36c38785,
            0xd8e37761b67fd23d,
            0x1a05fc8f28f36111,
            0x8a5cbf6d43ddb8ff,
            0x542b8f4b3d799736,
            0x67776f727cdcb767,
            0xc36fe0038dc296ed,
            0x93830012e1352eaa,
            0xad311b5c9f868a0f,
            0x939b7385987ae72a,
            0xa06e61f3f9e6ce5e,
            0x7c222454cb11b12b,
            0x64ed581a80c86f61,
            0x69bf098a42c1c02b,
            0xfa4917e47c1fd291,
            0x98340a278810507a,
            0xa3f220f7d67ddfc5,
            0x89889303cb69c244,
            0x5da82c2965fc4460,
            0x5bc5f44542036706,
            0xa37ee6a48fa4925b,
            0x4fbdc6300b14abe7,
            0x60abe721710b5c63,
            0x3c3b401645a533bc,
            0x8aad29d7f1e7d749,
            0x347fe70bb11489ae,
            0x261082c0f5df9e20,
            0x5e38cf71f15708a8,
            0x295c95c5f3e,
        ],
        vec![
            0xc7c375d58f843538,
            0x575d9958a354e1d7,
            0xd58eb2d664c94be9,
            0x01834d2c486a1a93,
            0xf12f3fd550003c38,
            0xf01d4f5ecc80623c,
            0x84b399239f76ea38,
            0x3aed76bc78693f86,
            0x72d1d7414d1fa1dc,
            0x7c1f9340a7833bf1,
            0xf533bab553a86f74,
            0x301a60252a42574c,
            0x8b45b2101ff51737,
            0x11f0f8d3ae569685,
            0x2498b359f68fc2ca,
            0x7fb70cab9623bef2,
            0x2aeb911381e059f9,
            0xce1fe3afd01d6f92,
            0x80eb380b50a387e2,
            0x093163f033afb1e9,
            0xec8499fd1267db2e,
            0xac91cde0e5418ecb,
            0x05a325670146c7a8,
            0xc08faf3fff678144,
            0x4317faad127a201b,
            0x08bb61e9b2d98604,
            0x13c6dcae19edb22c,
            0x2bc7c15544f8196f,
            0x3f57c556505efe70,
            0x553255aadb0210f3,
            0x084fe2dff70b195f,
            0x953dadfdc9a6105f,
            0xa7742db73393097b,
            0x0c153d0dcdaec848,
            0x0ce8107d3a883e5d,
            0x92ce55d755693e24,
            0xda65e1467d974b68,
            0xc6486d16bb8f0551,
            0xdc72a3a929936b5c,
            0x7d190ec644,
        ],
        vec![
            0xdeba46dce9a1992b,
            0xc2f1915bcc20ce52,
            0x44488df5c345e0ce,
            0x3a4ea34285fa1382,
            0x552ff18c1b73d78f,
            0x490925bd6717f163,
            0x7423888bf366226e,
            0x1dd49de0f7260962,
            0x2e539b01bc6fe7d0,
            0x3e77508f19d218f7,
            0xe129df615c3dee24,
            0x3e3f3bc5209c3479,
            0x3e4165724310d8b0,
            0x49a98cb29e6a30c6,
            0x5a856dc94341904f,
            0x5962a118ebb7aaf8,
            0xa2d12fc74e0eb423,
            0x777077503c748279,
            0x1f34b8f138c44392,
            0xa25d0e80c183777a,
            0xa9ee97c4dbddc413,
            0x42ed7ee14eb08f84,
            0x671679a92c365c77,
            0xbf18b5ac4eb113a1,
            0xa178f35a26e11e16,
            0xb115a68db96358a5,
            0xfde7393cace88905,
            0x326f05f36cf2a3ca,
            0x8a85ec49af7ec213,
            0xd7f382175a4092a2,
            0x97495fb10bdaacb9,
            0x33ace91963e1a82c,
            0xcf9ee7112265eb78,
            0x631091c5a4ed9d12,
            0xf610502f87ad136a,
            0xa8ae819051656b47,
            0x3295b914c66ea3a4,
            0x24499e33731d7d81,
            0x721abd4f2c94869f,
            0xc0d92f879870a742,
            0x0289c1f55ba61759,
            0x4216a1a94df73d39,
            0x107eed6f883e905e,
            0x17a5b6e4b8,
        ],
    ];

    for i in 0..12 {
        let pow = pows[i].clone();
        let val = Fq6Config::NONRESIDUE.pow(pow);
        println!("val[{}] = {:?}", i, val);
    }
}

fn main() {
    // let fq6_modulus = <Fq6Config as Fp6Config>::NONRESIDUE;
    // let sextic_non_residue_over_fq2 = find_sextic_non_residue_over_fq2();
    // println!(
    //     "sextic_non_residue_over_fq2 = {:?}",
    //     sextic_non_residue_over_fq2
    // );
    // let pow6 = sextic_non_residue_over_fq2
    //     // .pow([6u64])
    //     .pow(&Q_MINUS_ONE_DIV_SIX);
    // println!("pow6 = {:?}", pow6);
    // let val = Fq2::new(MontFp!("9"), Fq::ONE);
    // println!(
    //     "val pow is one? {:?}",
    //     val.pow(&Q_MINUS_ONE_DIV_SIX) == Fq2::ONE
    // );
    // println!("val = {:?}", val);
    // println!(
    //     "sextic_non_residue_over_fq2 = {:?}",
    //     sextic_non_residue_over_fq2
    // );
    // let quad_non_residue = val.pow(&Q_MINUS_ONE_DIV_SIX).sqrt().unwrap();
    // println!("quad_non_residue = {:?}", quad_non_residue);

    // let fp6_non_residue_pow = Fq6Config::NONRESIDUE.pow(&P_MINUS_1_DIV_6);
    // println!("fp6_non_residue_pow = {:?}", fp6_non_residue_pow);

    compute_frobenius_coeffs_for_compressible_fq12();
}
