use quote::quote;
pub(super) fn mul_assign_impl(
    can_use_no_carry_mul_opt: bool,
    yd_opt: bool,
    num_limbs: usize,
    modulus_limbs: &[u64],
    modulus_has_spare_bit: bool,
) -> proc_macro2::TokenStream {
    let mut body = proc_macro2::TokenStream::new();
    let modulus_0 = modulus_limbs[0];
    let modulus_1 = modulus_limbs[1];
    let modulus_2 = modulus_limbs[2];
    let modulus_3 = modulus_limbs[3];

    const U64_P: [u64; 4] = [
        0x43e1f593f0000001,
        0x2833e84879b97091,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ];
    // RIGHT now this is just for BN_255
    if yd_opt
        && num_limbs == 4
        && modulus_0 == U64_P[0]
        && modulus_1 == U64_P[1]
        && modulus_2 == U64_P[2]
        && modulus_3 == U64_P[3]
    {
        body.extend(quote! {
        //let (c00hi, c00lo) = fa::mult(a.0.0[0], b.0.0[0]);
        //let (c01hi, c01lo) = fa::mult(a.0.0[0], b.0.0[1]);
        //let (c02hi, c02lo) = fa::mult(a.0.0[0], b.0.0[2]);
        //let (c03hi, c03lo) = fa::mult(a.0.0[0], b.0.0[3]);
        //let (c10hi, c10lo) = fa::mult(a.0.0[1], b.0.0[0]);
        //let (c11hi, c11lo) = fa::mult(a.0.0[1], b.0.0[1]);
        //let (c12hi, c12lo) = fa::mult(a.0.0[1], b.0.0[2]);
        //let (c13hi, c13lo) = fa::mult(a.0.0[1], b.0.0[3]);
        //let (c20hi, c20lo) = fa::mult(a.0.0[2], b.0.0[0]);
        //let (c21hi, c21lo) = fa::mult(a.0.0[2], b.0.0[1]);
        //let (c22hi, c22lo) = fa::mult(a.0.0[2], b.0.0[2]);
        //let (c23hi, c23lo) = fa::mult(a.0.0[2], b.0.0[3]);
        //let (c30hi, c30lo) = fa::mult(a.0.0[3], b.0.0[0]);
        //let (c31hi, c31lo) = fa::mult(a.0.0[3], b.0.0[1]);
        //let (c32hi, c32lo) = fa::mult(a.0.0[3], b.0.0[2]);
        //let (c33hi, c33lo) = fa::mult(a.0.0[3], b.0.0[3]);
        //
        //let mut c: bool;
        //let mut r00 = c00lo;
        //let mut r01 = 0u64;
        //let mut r10 = 0u64;
        //let mut r11 = 0u64;
        //let mut r20 = 0u64;
        //let mut r21 = 0u64;
        //let mut r30 = 0u64;
        //let mut r31 = 0u64;
        //
        //let m = (Self::INV).wrapping_mul(r00);
        //let (m00hi, m00lo) = fa::mult(m, #modulus_0);
        //let (m01hi, m01lo) = fa::mult(m, #modulus_1);
        //let (m02hi, m02lo) = fa::mult(m, #modulus_2);
        //let (m03hi, m03lo) = fa::mult(m, #modulus_3);
        //
        //let (_, c) = r00.carrying_add(m00lo, false);
        //let (r01, _) = r01.carrying_add(c00hi, c); // carry is rare
        //
        //let (r01, c) = r01.carrying_add(c01lo, false);
        //let (r10, _) = r10.carrying_add(c11lo, c); // carry is rare
        //
        //let (r01, c) = r01.carrying_add(c10lo, false);
        //let (r10, c) = r10.carrying_add(c01hi, c);
        //let (r11, _) = r11.carrying_add(c11hi, c); // carry is rare
        //
        //let (r01, c) = r01.carrying_add(m00hi, false);
        //let (r10, c) = r10.carrying_add(c10hi, c);
        //let (r11, c) = r11.carrying_add(c12lo, c);
        //let (r20, _) = r20.carrying_add(c12hi, c); // carry is rare
        //
        //let (r01, c) = r01.carrying_add(m01lo, false);
        //let (r10, c) = r10.carrying_add(c02lo, c);
        //let (r11, c) = r11.carrying_add(c21lo, c);
        //let (r20, c) = r20.carrying_add(c21hi, c);
        //let (r21, _) = r21.carrying_add(c13hi, c); // carry is rare
        //
        //let m = (Self::INV).wrapping_mul(r01);
        //let (m10hi, m10lo) = fa::mult(m, #modulus_0);
        //let (m11hi, m11lo) = fa::mult(m, #modulus_1);
        //let (m12hi, m12lo) = fa::mult(m, #modulus_2);
        //let (m13hi, m13lo) = fa::mult(m, #modulus_3);
        //
        //let (_, c) = r01.carrying_add(m10lo, false);
        //let (r10, c) = r10.carrying_add(c20lo, c);
        //let (r11, c) = r11.carrying_add(c02hi, c);
        //let (r20, c) = r20.carrying_add(c13lo, c);
        //let (r21, c) = r21.carrying_add(c31hi, c);
        //let (r30, _) = r30.carrying_add(c23hi, c); // carry is rare
        //
        //let (r10, c) = r10.carrying_add(m02lo, false);
        //let (r11, c) = r11.carrying_add(c20hi, c);
        //let (r20, c) = r20.carrying_add(c31lo, c);
        //let (r21, c) = r21.carrying_add(c23lo, c);
        //let (r30, c) = r30.carrying_add(c32hi, c);
        //let (r31, _) = r31.carrying_add(c33hi, c);
        //
        //let (r10, c) = r10.carrying_add(m01hi, false);
        //let (r11, c) = r11.carrying_add(c03lo, c);
        //let (r20, c) = r20.carrying_add(c03hi, c);
        //let (r21, c) = r21.carrying_add(c32lo, c);
        //let (r30, c) = r30.carrying_add(c33lo, c);
        //let (r31, _) = r31.carrying_add(0u64 , c);
        //
        //let (r10, c) = r10.carrying_add(m10hi, false);
        //let (r11, c) = r11.carrying_add(c30lo, c);
        //let (r20, c) = r20.carrying_add(c30hi, c);
        //let (r21, c) = r21.carrying_add(c22hi, c);
        //let (r30, _) = r30.carrying_add(0u64 , c);
        //
        //let (r10, c) = r10.carrying_add(m11lo, false);
        //let (r11, c) = r11.carrying_add(m02hi, c);
        //let (r20, c) = r20.carrying_add(c22lo, c);
        //let (r21, c) = r21.carrying_add(m13hi, c);
        //let (r30, _) = r30.carrying_add(0u64 , c);
        //
        //let m = (Self::INV).wrapping_mul(r10);
        //let (m20hi, m20lo) = fa::mult(m, #modulus_0);
        //let (m21hi, m21lo) = fa::mult(m, #modulus_1);
        //let (m22hi, m22lo) = fa::mult(m, #modulus_2);
        //let (m23hi, m23lo) = fa::mult(m, #modulus_3);
        //
        //let (_, c) = r10.carrying_add(m20lo, false);
        //let (r11, c) = r11.carrying_add(m03lo, c);
        //let (r20, c) = r20.carrying_add(m03hi, c);
        //let (r21, c) = r21.carrying_add(m22hi, c);
        //let (r30, c) = r30.carrying_add(m23hi, c);
        //let (r31, _) = r31.carrying_add(0u64 , c);
        //
        //let (r11, c) = r11.carrying_add(m12lo, false);
        //let (r20, c) = r20.carrying_add(m12hi, c);
        //let (r21, c) = r21.carrying_add(m23lo, c);
        //let (r30, _) = r30.carrying_add(0u64 , c);
        //
        //let (r11, c) = r11.carrying_add(m11hi, false);
        //let (r20, c) = r20.carrying_add(m13lo, c);
        //let (r21, _) = r21.carrying_add(0u64 , c);
        //
        //let (r11, c) = r11.carrying_add(m20hi, false);
        //let (r20, c) = r20.carrying_add(m22lo, c);
        //let (r21, _) = r21.carrying_add(0u64 , c);
        //
        //let (r11, c) = r11.carrying_add(m21lo, false);
        //let (r20, c) = r20.carrying_add(m21hi, c);
        //let (r21, _) = r21.carrying_add(0u64 , c);
        //
        //let m = (Self::INV).wrapping_mul(r11);
        //let (m30hi, m30lo) = fa::mult(m, #modulus_0);
        //let (m31hi, m31lo) = fa::mult(m, #modulus_1);
        //let (m32hi, m32lo) = fa::mult(m, #modulus_2);
        //let (m33hi, m33lo) = fa::mult(m, #modulus_3);
        //
        //let (_, c) = r11.carrying_add(m30lo, false);
        //let (r20, c) = r20.carrying_add(m30hi, c);
        //let (r21, c) = r21.carrying_add(m32lo, c);
        //let (r30, c) = r30.carrying_add(m32hi, c);
        //let (r31, c) = r31.carrying_add(m33hi, c);
        //let (r20, c) = r20.carrying_add(m31lo, false);
        //let (r21, c) = r21.carrying_add(m31hi, c);
        //let (r30, c) = r30.carrying_add(m33lo, c);
        //let (r31, _) = r31.carrying_add(0u64 , c);
        //a.0.0 = [r20, r21, r30, r31];
        //

        //--------------------------------------------YPT-----------------------------
        //
        //============================================================================
        let (c00hi, c00lo) = fa::mult(a.0.0[0], b.0.0[0]);
        let (c01hi, c01lo) = fa::mult(a.0.0[0], b.0.0[1]);
        let (c02hi, c02lo) = fa::mult(a.0.0[0], b.0.0[2]);
        let (c03hi, c03lo) = fa::mult(a.0.0[0], b.0.0[3]);
        let (c10hi, c10lo) = fa::mult(a.0.0[1], b.0.0[0]);
        let (c11hi, c11lo) = fa::mult(a.0.0[1], b.0.0[1]);
        let (c12hi, c12lo) = fa::mult(a.0.0[1], b.0.0[2]);
        let (c13hi, c13lo) = fa::mult(a.0.0[1], b.0.0[3]);
        let (c20hi, c20lo) = fa::mult(a.0.0[2], b.0.0[0]);
        let (c21hi, c21lo) = fa::mult(a.0.0[2], b.0.0[1]);
        let (c22hi, c22lo) = fa::mult(a.0.0[2], b.0.0[2]);
        let (c23hi, c23lo) = fa::mult(a.0.0[2], b.0.0[3]);
        let (c30hi, c30lo) = fa::mult(a.0.0[3], b.0.0[0]);
        let (c31hi, c31lo) = fa::mult(a.0.0[3], b.0.0[1]);
        let (c32hi, c32lo) = fa::mult(a.0.0[3], b.0.0[2]);
        let (c33hi, c33lo) = fa::mult(a.0.0[3], b.0.0[3]);

        let mut c: bool;
        let mut r0 = 0u128;
        let mut r1 = 0u128;
        let mut r2 = 0u128;
        let mut r3 = 0u128;

        (r0, _) = fa::wadd(c00hi, c00lo, r0, false);

        (r0, c) = fa::wadd(c01lo, 0u64, r0, false);
        (r1, _) = fa::wadd(c11hi, c11lo, r1, c);

        (r0, c) = fa::wadd(c10lo, 0u64, r0, false);
        (r1, c) = fa::wadd(c12lo, c01hi, r1, c);
        (r2, _) = fa::wadd(0u64, c12hi, r2, c);

        (r1, c) = fa::wadd(c21lo, c10hi, r1, false);
        (r2, _) = fa::wadd(0u64, c21hi, r2, c);

        (r1, c) = fa::wadd(c02hi, c02lo, r1, false);
        (r2, c) = fa::wadd(c13hi, c13lo, r2, c); // ignore c - limited to input < p

        (r1, c) = fa::wadd(c20hi, c20lo, r1, false);
        (r2, c) = fa::wadd(c31hi, c31lo, r2, c); // ignore c - limited to input < p

        (r1, c) = fa::wadd(c03lo, 0u64, r1, false);
        (r2, c) = fa::wadd(c23lo, c03hi, r2, c);
        (r3, _) = fa::wadd(0u64, c23hi, r3, c);

        (r1, c) = fa::wadd(c30lo, 0u64, r1, false);
        (r2, c) = fa::wadd(c32lo, c30hi, r2, c);
        (r3, _) = fa::wadd(0u64, c32hi, r3, c);

        const U64_I2: [u64; 4] = [0x18ee753c76f9dc6f, 0x54ad7e14a329e70f, 0x2b16366f4f7684df, 0x133100d71fdf3579];
        let (r0hi, r0lo) = ((r0 >> 64) as u64, r0 as u64);
        let (ir000hi, ir000lo) = fa::mult(r0lo, U64_I2[0]);
        let (ir001hi, ir001lo) = fa::mult(r0lo, U64_I2[1]);
        let (ir002hi, ir002lo) = fa::mult(r0lo, U64_I2[2]);
        let (ir003hi, ir003lo) = fa::mult(r0lo, U64_I2[3]);
        let (ir010hi, ir010lo) = fa::mult(r0hi, U64_I2[0]);
        let (ir011hi, ir011lo) = fa::mult(r0hi, U64_I2[1]);
        let (ir012hi, ir012lo) = fa::mult(r0hi, U64_I2[2]);
        let (ir013hi, ir013lo) = fa::mult(r0hi, U64_I2[3]);

        (r1, c) = fa::wadd(ir000hi, ir000lo, r1, false);
        (r2, c) = fa::wadd(c22hi, c22lo, r2, c);
        (r3, _) = fa::wadd(c33hi, c33lo, r3, c);

        (r1, c) = fa::wadd(ir001lo, 0u64, r1, false);
        (r2, c) = fa::wadd(ir002hi, ir002lo, r2, c);
        (r3, _) = fa::wadd(0u64, ir003hi, r3, c);

        (r1, c) = fa::wadd(ir010lo, 0u64, r1, false);
        (r2, c) = fa::wadd(ir003lo, ir001hi, r2, c);
        (r3, _) = fa::wadd(0u64, ir012hi, r3, c);

        const U64_I1: [u64; 4] = [0x2d3e8053e396ee4d, 0xca478dbeab3c92cd, 0xb2d8f06f77f52a93, 0x24d6ba07f7aa8f04];
        let r1lo = r1 as u64;
        let (ir100hi, ir100lo) = fa::mult(r1lo, U64_I1[0]);
        let (ir101hi, ir101lo) = fa::mult(r1lo, U64_I1[1]);
        let (ir102hi, ir102lo) = fa::mult(r1lo, U64_I1[2]);
        let (ir103hi, ir103lo) = fa::mult(r1lo, U64_I1[3]);

        (r1, c) = fa::wadd(ir100lo, 0u64, r1, false);
        (r2, c) = fa::wadd(ir012lo, ir010hi, r2, c);
        (r3, _) = fa::wadd(ir013hi, ir013lo, r3, c);

        let m = (Self::INV).wrapping_mul((r1 >> 64) as u64);
        let (m0hi, m0lo) = fa::mult(m, #modulus_0);
        let (m1hi, m1lo) = fa::mult(m, #modulus_1);
        let (m2hi, m2lo) = fa::mult(m, #modulus_2);
        let (m3hi, m3lo) = fa::mult(m, #modulus_3);

        (_, c) = fa::wadd(m0lo, 0u64, r1, false);
        (r2, c) = fa::wadd(ir011hi, ir011lo, r2, c);
        (r3, _) = fa::wadd(0u64, ir102hi, r3, c);

        (r2, c) = fa::wadd(ir102lo, ir100hi, r2, false);
        (r3, _) = fa::wadd(ir103hi, ir103lo, r3, c);

        (r2, c) = fa::wadd(ir101hi, ir101lo, r2, false);
        (r3, _) = fa::wadd(0u64, m2hi, r3, c);

        (r2, c) = fa::wadd(m2lo, m0hi, r2, false);
        (r3, _) = fa::wadd(m3hi, m3lo, r3, c);

        (r2, c) = fa::wadd(m1hi, m1lo, r2, false);
        (r3, _) = fa::wadd(0u64, 0u64, r3, c);

        // return
        (a.0).0 = [r2 as u64, (r2 >> 64) as u64, r3 as u64, (r3 >> 64) as u64];
        //---------------------------------Y-opt----------------------------------
        
        __subtract_modulus(a);
        __subtract_modulus(a);
        });
    } else if can_use_no_carry_mul_opt {
        // This modular multiplication algorithm uses Montgomery
        // reduction for efficient implementation. It also additionally
        // uses the "no-carry optimization" outlined
        // [here](https://hackmd.io/@gnark/modular_multiplication) if
        // `MODULUS` has (a) a non-zero MSB, and (b) at least one
        // zero bit in the rest of the modulus.

        let mut default = proc_macro2::TokenStream::new();
        default.extend(quote! { let mut r = [0u64; #num_limbs]; });
        for i in 0..num_limbs {
            default.extend(quote! {
                let mut carry1 = 0u64;
                r[0] = fa::mac(r[0], (a.0).0[0], (b.0).0[#i], &mut carry1);
                let k = r[0].wrapping_mul(Self::INV);
                let mut carry2 = 0u64;
                fa::mac_discard(r[0], k, #modulus_0, &mut carry2);
            });
            for (j, modulus_j) in modulus_limbs.iter().enumerate().take(num_limbs).skip(1) {
                let idx = j - 1;
                default.extend(quote! {
                    r[#j] = fa::mac_with_carry(r[#j], (a.0).0[#j], (b.0).0[#i], &mut carry1);
                    r[#idx] = fa::mac_with_carry(r[#j], k, #modulus_j, &mut carry2);
                });
            }
            default.extend(quote!(r[#num_limbs - 1] = carry1 + carry2;));
        }
        default.extend(quote!((a.0).0 = r;));
        // Avoid using assembly for `N == 1`.
        if (2..=6).contains(&num_limbs) {
            body.extend(quote!({
                if cfg!(all(
                    feature = "asm",
                    target_feature = "bmi2",
                    target_feature = "adx",
                    target_arch = "x86_64"
                )) {
                    #[cfg(
                        all(
                            feature = "asm",
                            target_feature = "bmi2",
                            target_feature = "adx",
                            target_arch = "x86_64"
                        )
                    )]
                    ark_ff::x86_64_asm_mul!(#num_limbs, (a.0).0, (b.0).0);
                } else {
                    #[cfg(
                        not(all(
                            feature = "asm",
                            target_feature = "bmi2",
                            target_feature = "adx",
                            target_arch = "x86_64"
                        ))
                    )]
                    {
                        #default
                    }
                }
            }))
        } else {
            body.extend(quote!({ #default }))
        }
        body.extend(quote!(__subtract_modulus(a);));
    } else {
        // We use standard CIOS
        let double_limbs = num_limbs * 2;
        body.extend(quote! {
            let mut scratch = [0u64; #double_limbs];
        });
        for i in 0..num_limbs {
            body.extend(quote! { let mut carry = 0u64; });
            for j in 0..num_limbs {
                let k = i + j;
                body.extend(quote!{scratch[#k] = fa::mac_with_carry(scratch[#k], (a.0).0[#i], (b.0).0[#j], &mut carry);});
            }
            body.extend(quote! { scratch[#i + #num_limbs] = carry; });
        }
        body.extend(quote!( let mut carry2 = 0u64; ));
        for i in 0..num_limbs {
            body.extend(quote! {
                let tmp = scratch[#i].wrapping_mul(Self::INV);
                let mut carry = 0u64;
                fa::mac(scratch[#i], tmp, #modulus_0, &mut carry);
            });
            for (j, modulus_j) in modulus_limbs.iter().enumerate().take(num_limbs).skip(1) {
                let k = i + j;
                body.extend(quote!(scratch[#k] = fa::mac_with_carry(scratch[#k], tmp, #modulus_j, &mut carry);));
            }
            body.extend(quote!(carry2 = fa::adc(&mut scratch[#i + #num_limbs], carry, carry2);));
        }
        body.extend(quote! {
            (a.0).0 = scratch[#num_limbs..].try_into().unwrap();
        });
        if modulus_has_spare_bit {
            body.extend(quote!(__subtract_modulus(a);));
        } else {
            body.extend(quote!(__subtract_modulus_with_carry(a, carry2 != 0);));
        }
    }
    body
}
