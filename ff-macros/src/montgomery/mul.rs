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
                macro_rules! subarray {
                ($t:expr, $b: literal, $l: literal) => {
                {
                 use seq_macro::seq;
                 let t = $t;
                 let mut s = [0;$l];

                // The compiler does not detect out-of-bounds when using `for` therefore `seq!` is used here
                seq!(i in 0..$l {
                s[i] = t[$b+i];
                 });
                s
                }
            };}
        });
        //
        let double_limbs = num_limbs * 2;
        body.extend(quote! {
            let mut t = [0u64; #double_limbs];
            let mut carry = 0;

            (t[0], carry) = fa::carrying_mul_add(a.0.0[0], b.0.0[0], t[0], carry);
            (t[1], carry) = fa::carrying_mul_add(a.0.0[0], b.0.0[1], t[1], carry);
            (t[2], carry) = fa::carrying_mul_add(a.0.0[0], b.0.0[2], t[2], carry);
            (t[3], carry) = fa::carrying_mul_add(a.0.0[0], b.0.0[3], t[3], carry);
            t[4] = carry;
            carry = 0;
            (t[1], carry) = fa::carrying_mul_add(a.0.0[1], b.0.0[0], t[1], carry);
            (t[2], carry) = fa::carrying_mul_add(a.0.0[1], b.0.0[1], t[2], carry);
            (t[3], carry) = fa::carrying_mul_add(a.0.0[1], b.0.0[2], t[3], carry);
            (t[4], carry) = fa::carrying_mul_add(a.0.0[1], b.0.0[3], t[4], carry);
            t[5] = carry;
            carry = 0;
            (t[2], carry) = fa::carrying_mul_add(a.0.0[2], b.0.0[0], t[2], carry);
            (t[3], carry) = fa::carrying_mul_add(a.0.0[2], b.0.0[1], t[3], carry);
            (t[4], carry) = fa::carrying_mul_add(a.0.0[2], b.0.0[2], t[4], carry);
            (t[5], carry) = fa::carrying_mul_add(a.0.0[2], b.0.0[3], t[5], carry);
            t[6] = carry;
            carry = 0;
            (t[3], carry) = fa::carrying_mul_add(a.0.0[3], b.0.0[0], t[3], carry);
            (t[4], carry) = fa::carrying_mul_add(a.0.0[3], b.0.0[1], t[4], carry);
            (t[5], carry) = fa::carrying_mul_add(a.0.0[3], b.0.0[2], t[5], carry);
            (t[6], carry) = fa::carrying_mul_add(a.0.0[3], b.0.0[3], t[6], carry);
            t[7] = carry;
        });

        //for i in 0..num_limbs {
        //    body.extend(quote! { let mut carry = 0u64; });
        //    for j in 0..num_limbs {
        //        let k = i + j;
        //        body.extend(quote!{t[#k] = fa::mac_with_carry(t[#k], (a.0).0[#i], (b.0).0[#j], &mut carry);});
        //    }
        //    body.extend(quote! { t[#i + #num_limbs] = carry; });
        //}

        // The precomputed multiplications!
        body.extend(quote! {
            let mut sub_arr = subarray!(t, 3, 5);
            let mut s_r1 = [0_u64; 5]; // TODO: Make this general later should be num_limbs//2 + 1
            let mut s_r2 = [0_u64; 5]; // TODO
            let mut s_r3 = [0_u64; 5]; // TODO

            (s_r1[0], s_r1[1]) = fa::carrying_mul_add(t[0], constants::U64_I3[0], 0, 0);
            (s_r1[1], s_r1[2]) = fa::carrying_mul_add(t[0], constants::U64_I3[1], s_r1[1], 0);
            (s_r1[2], s_r1[3]) = fa::carrying_mul_add(t[0], constants::U64_I3[2], s_r1[2], 0);
            (s_r1[3], s_r1[4]) = fa::carrying_mul_add(t[0], constants::U64_I3[3], s_r1[3], 0);

            (s_r2[0], s_r2[1]) = fa::carrying_mul_add(t[1], constants::U64_I2[0], 0, 0);
            (s_r2[1], s_r2[2]) = fa::carrying_mul_add(t[1], constants::U64_I2[1], s_r2[1], 0);
            (s_r2[2], s_r2[3]) = fa::carrying_mul_add(t[1], constants::U64_I2[2], s_r2[2], 0);
            (s_r2[3], s_r2[4]) = fa::carrying_mul_add(t[1], constants::U64_I2[3], s_r2[3], 0);

            (s_r3[0], s_r3[1]) = fa::carrying_mul_add(t[2], constants::U64_I1[0], 0, 0);
            (s_r3[1], s_r3[2]) = fa::carrying_mul_add(t[2], constants::U64_I1[1], s_r3[1], 0);
            (s_r3[2], s_r3[3]) = fa::carrying_mul_add(t[2], constants::U64_I1[2], s_r3[2], 0);
            (s_r3[3], s_r3[4]) = fa::carrying_mul_add(t[2], constants::U64_I1[3], s_r3[3], 0);

        });

        // mac_with_carry and carrying_mul_add do the same thing -- but using already existing
        // arkworks helpers to be consistent.
        // TODO: These constants here are the only things that make this code base not generic, but can
        // be fixed later.
        //for i in 0..num_limbs {
        //    body.extend(quote!{
        //        s_r1[#i] = fa::mac_with_carry(s_r1[#i], t[0], constants::U64_I3[#i], &mut s_r1[#i+1]);
        //        s_r2[#i] = fa::mac_with_carry(s_r2[#i], t[1], constants::U64_I2[#i], &mut s_r2[#i+1]);
        //        s_r3[#i] = fa::mac_with_carry(s_r3[#i], t[2], constants::U64_I1[#i], &mut s_r3[#i+1]);
        //    });
        //}
        //
        body.extend(quote! {
        let s = fa::addv(fa::addv(subarray!(t, 3, 5), s_r1), fa::addv(s_r2, s_r3));
        });

        body.extend(quote! {
            let m = s[0].wrapping_mul(Self::INV);
            let mut mp = [0_u64; 5]; // TODO: Make this general later; change this to limbs
            (mp[0], mp[1]) = fa::carrying_mul_add(m, #modulus_0, mp[0], 0);
            (mp[1], mp[2]) = fa::carrying_mul_add(m, #modulus_1, mp[1], 0);
            (mp[2], mp[3]) = fa::carrying_mul_add(m, #modulus_2, mp[2], 0);
            (mp[3], mp[4]) = fa::carrying_mul_add(m, #modulus_3, mp[3], 0);

        });

        //for i in 0..num_limbs {
        //    let mod_limb_i = modulus_limbs[i];
        //    body.extend(quote! {
        //        mp[#i] = fa::mac_with_carry(mp[#i], m, #mod_limb_i, &mut mp[#i+1]);
        //    });
        //}

        // TODO: handle the constants better
        body.extend(quote! {
            let r = fa::reduce_ct(subarray!(fa::addv(s, mp), 1, 4), constants::U64_2P);
            (a.0).0 = r;
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
                    #[allow(unsafe_code, unused_mut)]
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
            for j in 1..num_limbs {
                let modulus_j = modulus_limbs[j];
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
