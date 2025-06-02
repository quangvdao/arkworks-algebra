use quote::quote;

pub(super) fn square_in_place_impl(
    can_use_no_carry_mul_opt: bool,
    num_limbs: usize,
    modulus_limbs: &[u64],
    modulus_has_spare_bit: bool,
    yd_opt: bool,
) -> proc_macro2::TokenStream {
    let mut body = proc_macro2::TokenStream::new();
    let mut default = proc_macro2::TokenStream::new();
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
    if yd_opt
        && num_limbs == 4
        && modulus_0 == U64_P[0]
        && modulus_1 == U64_P[1]
        && modulus_2 == U64_P[2]
        && modulus_3 == U64_P[3]
    {
        body.extend(quote! {
            let (c00hi, c00lo) = fa::mult((a.0).0[0], (a.0).0[0]);
            let (c01hi, c01lo) = fa::mult((a.0).0[0], (a.0).0[1]);
            let (c02hi, c02lo) = fa::mult((a.0).0[0], (a.0).0[2]);
            let (c03hi, c03lo) = fa::mult((a.0).0[0], (a.0).0[3]);
            let (c10hi, c10lo) = (c01hi, c01lo);
            let (c11hi, c11lo) = fa::mult((a.0).0[1], (a.0).0[1]);
            let (c12hi, c12lo) = fa::mult((a.0).0[1], (a.0).0[2]);
            let (c13hi, c13lo) = fa::mult((a.0).0[1], (a.0).0[3]);
            let (c20hi, c20lo) = (c02hi, c02lo);
            let (c21hi, c21lo) = (c12hi, c12lo);
            let (c22hi, c22lo) = fa::mult((a.0).0[2], (a.0).0[2]);
            let (c23hi, c23lo) = fa::mult((a.0).0[2], (a.0).0[3]);
            let (c30hi, c30lo) = (c03hi, c03lo);
            let (c31hi, c31lo) = (c13hi, c13lo);
            let (c32hi, c32lo) = (c23hi, c23lo);
            let (c33hi, c33lo) = fa::mult((a.0).0[3], (a.0).0[3]);

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

            const U64_I2: [u64; 4] = [
                0x18ee753c76f9dc6f,
                0x54ad7e14a329e70f,
                0x2b16366f4f7684df,
                0x133100d71fdf3579,
            ];
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

            const U64_I1: [u64; 4] = [
                0x2d3e8053e396ee4d,
                0xca478dbeab3c92cd,
                0xb2d8f06f77f52a93,
                0x24d6ba07f7aa8f04,
            ];
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
            a.0.0 = [r2 as u64, (r2 >> 64) as u64, r3 as u64, (r3 >> 64) as u64];
            //__subtract_modulus(a);
            //__subtract_modulus(a);
            fa::reduce_twice_if_needed(&mut a.0.0);
        });
        body
    } else {
        let modulus_0 = modulus_limbs[0];
        let double_num_limbs = 2 * num_limbs;

        default.extend(quote! {
            let mut r = [0u64; #double_num_limbs];
            let mut carry = 0;
        });
        for i in 0..(num_limbs - 1) {
            for j in (i + 1)..num_limbs {
                let idx = i + j;
                default.extend(quote! {
                    r[#idx] = fa::mac_with_carry(r[#idx], (a.0).0[#i], (a.0).0[#j], &mut carry);
                })
            }
            default.extend(quote! {
                r[#num_limbs + #i] = carry;
                carry = 0;
            });
        }
        default.extend(quote! { r[#double_num_limbs - 1] = r[#double_num_limbs - 2] >> 63; });
        for i in 2..(double_num_limbs - 1) {
            let idx = double_num_limbs - i;
            default.extend(quote! { r[#idx] = (r[#idx] << 1) | (r[#idx - 1] >> 63); });
        }
        default.extend(quote! { r[1] <<= 1; });

        for i in 0..num_limbs {
            let idx = 2 * i;
            default.extend(quote! {
                r[#idx] = fa::mac_with_carry(r[#idx], (a.0).0[#i], (a.0).0[#i], &mut carry);
                carry = fa::adc(&mut r[#idx + 1], 0, carry);
            });
        }
        // Montgomery reduction
        default.extend(quote! { let mut carry2 = 0; });
        for i in 0..num_limbs {
            default.extend(quote! {
                let k = r[#i].wrapping_mul(Self::INV);
                let mut carry = 0;
                fa::mac_discard(r[#i], k, #modulus_0, &mut carry);
            });
            for (j, modulus_j) in modulus_limbs.iter().enumerate().take(num_limbs).skip(1) {
                let idx = j + i;
                default.extend(quote! {
                    r[#idx] = fa::mac_with_carry(r[#idx], k, #modulus_j, &mut carry);
                });
            }
            default.extend(quote! { carry2 = fa::adc(&mut r[#num_limbs + #i], carry, carry2); });
        }
        default.extend(quote! { (a.0).0 = r[#num_limbs..].try_into().unwrap(); });

        if num_limbs == 1 {
            // We default to multiplying with `a` using the `Mul` impl
            // for the N == 1 case
            quote!({
                *a *= *a;
            })
        } else if (2..=6).contains(&num_limbs) && can_use_no_carry_mul_opt {
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
                    #[allow(unsafe_code, unused_mut)]
                    {
                        ark_ff::x86_64_asm_square!(#num_limbs, (a.0).0);
                    }
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
            }));
            body.extend(quote!(__subtract_modulus(a);));
            body
        } else {
            body.extend(quote!( #default ));
            if modulus_has_spare_bit {
                body.extend(quote!(__subtract_modulus(a);));
            } else {
                body.extend(quote!(__subtract_modulus_with_carry(a, carry2 != 0);));
            }
            body
        }
    }
}
