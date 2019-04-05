// extern "C" {
//     fn mul1024_avx512(dest: &mut [u64; 16], a: &[u64; 8], b: &[u64; 8]);
// }

pub mod u64 {
    use *;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqUnreduced(pub(crate) [u64; 12]);
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqReduced(pub(crate) [u64; 6]);

    #[cfg(target_feature = "avx2")]
    impl From<backend::avx2::FqReduced> for FqReduced {
        fn from(other: backend::avx2::FqReduced) -> FqReduced {
            let mut imm = [0u32; 16];
            (other.0)[0].write_to_slice_unaligned(&mut imm[..8]);
            (other.0)[1].write_to_slice_unaligned(&mut imm[8..]);

            let mut res = [0u64; 6];

            #[inline(always)]
            fn c(a: u32, b: u32) -> u64 {
                ((b as u64) << 32) | (a as u64)
            }

            res[0] = c(imm[0], imm[1]);
            res[1] = c(imm[2], imm[3]);
            res[2] = c(imm[4], imm[5]);
            res[3] = c(imm[6], imm[7]);

            res[4] = c(imm[8], imm[10]);
            res[5] = c(imm[12], imm[14]);

            backend::u64::FqReduced(res)
        }
    }

    impl ::rand::Rand for FqReduced {
        #[inline(always)]
        fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
            FqReduced(rng.gen())
        }
    }

    pub fn mul_alt(a: &FqReduced, b: &FqReduced) -> FqUnreduced {
        let mut a_l = [0; 8];
        a_l[..6].copy_from_slice(&a.0);

        let mut b_l = [0; 8];
        b_l[..6].copy_from_slice(&b.0);

        let mut res = [0; 16];
        unsafe { super::avx512::mul1024(&mut res, &a_l, &b_l) };

        let mut end_res = [0; 12];
        end_res.copy_from_slice(&res[..12]);

        FqUnreduced(end_res)
    }

    pub fn mul(a: &FqReduced, b: &FqReduced) -> FqUnreduced {
        let mut carry = 0;
        let r0 = mac_with_carry(0, a.0[0], b.0[0], &mut carry);
        let r1 = mac_with_carry(0, a.0[0], b.0[1], &mut carry);
        let r2 = mac_with_carry(0, a.0[0], b.0[2], &mut carry);
        let r3 = mac_with_carry(0, a.0[0], b.0[3], &mut carry);
        let r4 = mac_with_carry(0, a.0[0], b.0[4], &mut carry);
        let r5 = mac_with_carry(0, a.0[0], b.0[5], &mut carry);
        let r6 = carry;
        let mut carry = 0;
        let r1 = mac_with_carry(r1, a.0[1], b.0[0], &mut carry);
        let r2 = mac_with_carry(r2, a.0[1], b.0[1], &mut carry);
        let r3 = mac_with_carry(r3, a.0[1], b.0[2], &mut carry);
        let r4 = mac_with_carry(r4, a.0[1], b.0[3], &mut carry);
        let r5 = mac_with_carry(r5, a.0[1], b.0[4], &mut carry);
        let r6 = mac_with_carry(r6, a.0[1], b.0[5], &mut carry);
        let r7 = carry;
        let mut carry = 0;
        let r2 = mac_with_carry(r2, a.0[2], b.0[0], &mut carry);
        let r3 = mac_with_carry(r3, a.0[2], b.0[1], &mut carry);
        let r4 = mac_with_carry(r4, a.0[2], b.0[2], &mut carry);
        let r5 = mac_with_carry(r5, a.0[2], b.0[3], &mut carry);
        let r6 = mac_with_carry(r6, a.0[2], b.0[4], &mut carry);
        let r7 = mac_with_carry(r7, a.0[2], b.0[5], &mut carry);
        let r8 = carry;
        let mut carry = 0;
        let r3 = mac_with_carry(r3, a.0[3], b.0[0], &mut carry);
        let r4 = mac_with_carry(r4, a.0[3], b.0[1], &mut carry);
        let r5 = mac_with_carry(r5, a.0[3], b.0[2], &mut carry);
        let r6 = mac_with_carry(r6, a.0[3], b.0[3], &mut carry);
        let r7 = mac_with_carry(r7, a.0[3], b.0[4], &mut carry);
        let r8 = mac_with_carry(r8, a.0[3], b.0[5], &mut carry);
        let r9 = carry;
        let mut carry = 0;
        let r4 = mac_with_carry(r4, a.0[4], b.0[0], &mut carry);
        let r5 = mac_with_carry(r5, a.0[4], b.0[1], &mut carry);
        let r6 = mac_with_carry(r6, a.0[4], b.0[2], &mut carry);
        let r7 = mac_with_carry(r7, a.0[4], b.0[3], &mut carry);
        let r8 = mac_with_carry(r8, a.0[4], b.0[4], &mut carry);
        let r9 = mac_with_carry(r9, a.0[4], b.0[5], &mut carry);
        let r10 = carry;
        let mut carry = 0;
        let r5 = mac_with_carry(r5, a.0[5], b.0[0], &mut carry);
        let r6 = mac_with_carry(r6, a.0[5], b.0[1], &mut carry);
        let r7 = mac_with_carry(r7, a.0[5], b.0[2], &mut carry);
        let r8 = mac_with_carry(r8, a.0[5], b.0[3], &mut carry);
        let r9 = mac_with_carry(r9, a.0[5], b.0[4], &mut carry);
        let r10 = mac_with_carry(r10, a.0[5], b.0[5], &mut carry);
        let r11 = carry;

        FqUnreduced([r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11])
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use rand::{Rng, SeedableRng, XorShiftRng};

        #[test]
        fn test_mul_extended() {
            let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

            for i in 0..1000_0000 {
                let a: FqReduced = rng.gen();
                let b: FqReduced = rng.gen();

                let res = mul(&a, &b);
                let expected = mul_alt(&a, &b);
                assert_eq!(res, expected, "case {}", i);
            }
        }

    }
}

#[cfg(target_feature = "avx2")]
pub mod avx2 {
    // 381 -> 12 * 32bit = 384

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use packed_simd::*;

    use backend;

    // Layout [[a_0, .., a_7]], [a_8, .., a_15], [a16, .. a24]]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqUnreduced(pub(crate) [u32x8; 3]);

    // Layout: [[a_0, .., a_7], [a_8, 0, a_9, 0, a_10, 0, a_11, 0]]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqReduced(pub(crate) [u32x8; 2]);

    impl From<FqUnreduced> for [u64; 12] {
        fn from(other: FqUnreduced) -> [u64; 12] {
            let mut imm = [0u32; 32];
            (other.0)[0].write_to_slice_unaligned(&mut imm[..8]);
            (other.0)[1].write_to_slice_unaligned(&mut imm[8..16]);
            (other.0)[2].write_to_slice_unaligned(&mut imm[16..24]);

            let mut res = [0u64; 12];

            #[inline(always)]
            fn c(a: u32, b: u32) -> u64 {
                ((b as u64) << 32) | (a as u64)
            }

            res[0] = c(imm[0], imm[1]);
            res[1] = c(imm[2], imm[3]);
            res[2] = c(imm[4], imm[5]);
            res[3] = c(imm[6], imm[7]);
            res[4] = c(imm[8], imm[9]);
            res[5] = c(imm[10], imm[11]);
            res[6] = c(imm[12], imm[13]);
            res[7] = c(imm[14], imm[15]);

            res[8] = c(imm[16], imm[17]);
            res[9] = c(imm[18], imm[19]);
            res[10] = c(imm[20], imm[21]);
            res[11] = c(imm[22], imm[23]);

            res
        }
    }

    impl ::rand::Rand for FqReduced {
        #[inline(always)]
        fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
            FqReduced([
                u32x8::new(
                    rng.gen::<u32>(),
                    rng.gen::<u32>(),
                    rng.gen::<u32>(),
                    rng.gen::<u32>(),
                    rng.gen::<u32>(),
                    rng.gen::<u32>(),
                    rng.gen::<u32>(),
                    rng.gen::<u32>(),
                ),
                u32x8::new(
                    rng.gen::<u32>(),
                    0,
                    rng.gen::<u32>(),
                    0,
                    rng.gen::<u32>(),
                    0,
                    rng.gen::<u32>(),
                    0,
                ),
            ])
        }
    }

    #[inline]
    pub fn mul(a: &FqReduced, b: &FqReduced) -> FqUnreduced {
        println!("\n-- mul\n\n");
        let mut out_0 = u32x8::default();
        let mut out_1 = u32x8::default();

        let mut x_0: u32x8;
        let mut x_1: u32x8;
        let mut t: u32x8;

        macro_rules! round {
            ($i:expr, $a:expr, $b: expr, $t:expr, $out:expr, $x_0:expr, $x_1:expr, [
                $l0:expr, $l1:expr, $l2:expr, $l3:expr,
                $l4:expr, $l5:expr, $l6:expr, $l7:expr
            ]) => {
                println!("round {}", $i);

                let tp = $t;
                let a_i = $a.extract($i);
                $t = u32x8::splat(a_i);

                println!("tp: {:?}", tp);
                println!("t:  {:?}", $t);
                println!("b[0]: {:?}", $b[0]);
                println!("b[1]: {:?}", $b[1]);

                $x_0 = $x_0 + m_lo($b[0], $t) + m_hi_half($b[0], tp);
                $x_1 = $x_1 + m_lo($b[1], $t) + m_hi_half($b[1], tp);

                // store x_0[0] at x[i]
                $out = shuffle!($out, $x_0, [$l0, $l1, $l2, $l3, $l4, $l5, $l6, $l7]);
                println!("out_0: {:?}", $out);

                $x_0 = shr32($x_0);
                $x_1 = shr32($x_1);
            };
        }

        {
            // first round
            println!("round 0");
            let a_i = a.0[0].extract(0);
            t = u32x8::splat(a_i);

            x_0 = m_lo(b.0[0], t);
            x_1 = m_lo_half(b.0[1], t);
            println!("x_0: {:?} * {:?} = {:?}", b.0[0], t, x_0);
            println!("x_1: {:?} * {:?} = {:?}", b.0[1], t, x_1);

            // store x_0[0] at x[i]
            out_0 = shuffle!(out_0, x_0, [8, 1, 2, 3, 4, 5, 6, 7]);
            println!("out_0: {:?}", out_0);

            x_0 = shr32(x_0);
            x_1 = shr32(x_1);
            println!("x_0 (shr32): {:?}", x_0);
            println!("x_1 (shr32): {:?}", x_1);
        }

        round!(1, a.0[0], b.0, t, out_0, x_0, x_1, [0, 8, 2, 3, 4, 5, 6, 7]);
        round!(2, a.0[0], b.0, t, out_0, x_0, x_1, [0, 1, 8, 3, 4, 5, 6, 7]);
        round!(3, a.0[0], b.0, t, out_0, x_0, x_1, [0, 1, 2, 8, 4, 5, 6, 7]);
        round!(4, a.0[0], b.0, t, out_0, x_0, x_1, [0, 1, 2, 3, 8, 5, 6, 7]);
        round!(5, a.0[0], b.0, t, out_0, x_0, x_1, [0, 1, 2, 3, 4, 8, 6, 7]);
        round!(6, a.0[0], b.0, t, out_0, x_0, x_1, [0, 1, 2, 3, 4, 5, 8, 7]);
        round!(7, a.0[0], b.0, t, out_0, x_0, x_1, [0, 1, 2, 3, 4, 5, 6, 8]);

        round!(0, a.0[1], b.0, t, out_1, x_0, x_1, [8, 1, 2, 3, 4, 5, 6, 7]);
        round!(1, a.0[1], b.0, t, out_1, x_0, x_1, [0, 8, 2, 3, 4, 5, 6, 7]);
        round!(2, a.0[1], b.0, t, out_1, x_0, x_1, [0, 1, 8, 3, 4, 5, 6, 7]);
        round!(3, a.0[1], b.0, t, out_1, x_0, x_1, [0, 1, 2, 8, 4, 5, 6, 7]);
        round!(4, a.0[1], b.0, t, out_1, x_0, x_1, [0, 1, 2, 3, 8, 5, 6, 7]);
        round!(5, a.0[1], b.0, t, out_1, x_0, x_1, [0, 1, 2, 3, 4, 8, 6, 7]);
        round!(6, a.0[1], b.0, t, out_1, x_0, x_1, [0, 1, 2, 3, 4, 5, 8, 7]);
        round!(7, a.0[1], b.0, t, out_1, x_0, x_1, [0, 1, 2, 3, 4, 5, 6, 8]);

        {
            // last round
            println!("round 16");
            x_0 = x_0 + m_hi(b.0[0], t);
        }

        // store x_q-1..x_0 starting at x[m+1]
        // out[m..].copy_from_slice(x_0.into_bits());

        println!("res: {:?}, {:?}, {:?}", out_0, out_1, x_0);
        FqUnreduced([out_0, out_1, x_0])
    }

    #[inline(always)]
    fn shr32(x: u32x8) -> u32x8 {
        unsafe { _mm256_srli_epi32(x.into_bits(), 32) }.into_bits()
    }

    #[inline(always)]
    fn m_hi(x: u32x8, y: u32x8) -> u32x8 {
        // mul lo 32 bits into 64 bits
        let a: u32x8 = unsafe { _mm256_mul_epu32(x.into_bits(), y.into_bits()) }.into_bits();

        let x_s: u32x8 = shuffle!(x, [1, 0, 3, 2, 5, 4, 7, 6]);
        let y_s: u32x8 = shuffle!(y, [1, 0, 3, 2, 5, 4, 7, 6]);
        let b: u32x8 = unsafe { _mm256_mul_epu32(x_s.into_bits(), y_s.into_bits()) }.into_bits();

        shuffle!(a, b, [1, 9, 3, 11, 5, 13, 7, 15])
    }

    // Assumes that `x` is of the layout (a0, 0, a1, 0, ..).
    #[inline(always)]
    fn m_hi_half(x: u32x8, y: u32x8) -> u32x8 {
        // mul lo 32 bits into 64 bits
        let a: u32x8 = unsafe { _mm256_mul_epu32(x.into_bits(), y.into_bits()) }.into_bits();

        let zero = u32x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        unsafe { _mm256_unpackhi_epi32(a.into_bits(), zero.into_bits()) }.into_bits()
    }

    // Assumes that `x` is of the layout (a0, 0, a1, 0, ..).
    #[inline(always)]
    fn m_lo_half(x: u32x8, y: u32x8) -> u32x8 {
        let a: u32x8 = unsafe { _mm256_mullo_epi32(x.into_bits(), y.into_bits()) }.into_bits();
        let zero = u32x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        unsafe { _mm256_unpacklo_epi32(a.into_bits(), zero.into_bits()) }.into_bits()
    }

    #[inline(always)]
    fn m_lo(x: u32x8, y: u32x8) -> u32x8 {
        unsafe { _mm256_mullo_epi32(x.into_bits(), y.into_bits()).into_bits() }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use backend::u64;
        use rand::{Rng, SeedableRng, XorShiftRng};

        #[test]
        fn test_from_reduced() {
            let cases = vec![
                (
                    [
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [1, 0, 0, 0, 0, 0],
                ),
                (
                    [
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [1, 0, 0, 0, 1, 0],
                ),
                (
                    [
                        u32x8::new(
                            0b1111_1111_1111_1111_1111_1111_1111_1111,
                            0b1111_1111_1111_1111_1111_1111_1111_1111,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [std::u64::MAX, 0, 0, 0, 0, 0],
                ),
                (
                    [
                        u32x8::new(
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                        ),
                        u32x8::new(
                            std::u32::MAX,
                            0,
                            std::u32::MAX,
                            0,
                            std::u32::MAX,
                            0,
                            std::u32::MAX,
                            0,
                        ),
                    ],
                    [
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                    ],
                ),
            ];

            for (i, case) in cases.into_iter().enumerate() {
                let a = FqReduced(case.0);

                let a_u64: u64::FqReduced = a.into();
                assert_eq!(a_u64.0, case.1, "case {}", i);
            }
        }

        #[test]
        fn test_from_unreduced() {
            let cases = vec![
                (
                    [
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                (
                    [
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
                ),
                (
                    [
                        u32x8::new(std::u32::MAX, std::u32::MAX, 0, 0, 0, 0, 0, 0),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [std::u64::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                (
                    [
                        u32x8::new(
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                        ),
                        u32x8::new(
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                        ),
                        u32x8::new(
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                            std::u32::MAX,
                        ),
                    ],
                    [
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                        std::u64::MAX,
                    ],
                ),
            ];

            for (i, case) in cases.into_iter().enumerate() {
                let a = FqUnreduced(case.0);

                let a_u64: [u64; 12] = a.into();
                assert_eq!(a_u64, case.1, "case {}", i);
            }
        }

        #[test]
        fn test_mul_simple() {
            let cases = vec![
                // 0 * 0 = 0
                (
                    (
                        [
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                        [
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                    ),
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                // 1 * 0 = 0
                (
                    (
                        [
                            u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                        [
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                    ),
                    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                // 1 * 1 = 1
                (
                    (
                        [
                            u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                        [
                            u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                    ),
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                // 5 * 4 = 20
                (
                    (
                        [
                            u32x8::new(5, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                        [
                            u32x8::new(4, 0, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                    ),
                    [20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                (
                    (
                        [
                            u32x8::new(2, 2, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                        [
                            u32x8::new(2, 2, 0, 0, 0, 0, 0, 0),
                            u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        ],
                    ),
                    [20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
            ];

            for (i, case) in cases.into_iter().enumerate() {
                let a = FqReduced((case.0).0);
                let b = FqReduced((case.0).1);
                let a_u64: u64::FqReduced = a.clone().into();
                let b_u64: u64::FqReduced = b.clone().into();
                println!("{:?} {:?}", a_u64, b_u64);

                let res: [u64; 12] = mul(&a, &b).into();
                let expected: [u64; 12] = backend::u64::mul(&a_u64, &b_u64).0;

                assert_eq!(res, expected, "case {}", i);
            }
        }

        #[test]
        fn test_mul_extended() {
            let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

            for _ in 0..1
            /*000*/
            {
                let a: FqReduced = rng.gen();
                let b: FqReduced = rng.gen();
                let a_u64: u64::FqReduced = a.clone().into();
                let b_u64: u64::FqReduced = b.clone().into();

                let res: [u64; 12] = mul(&a, &b).into();
                let expected: [u64; 12] = backend::u64::mul(&a_u64, &b_u64).0;

                for (i, (r, e)) in res.iter().zip(expected.iter()).enumerate() {
                    assert_eq!(r, e, "digit {}:\n{:#b}\n{:#b}", i, r, e);
                }
            }
        }

        #[test]
        fn test_mul_lo() {
            let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

            let a: FqReduced = rng.gen();
            let b: FqReduced = rng.gen();

            let c = m_lo(a.0[0], b.0[0]);

            for i in 0..8 {
                let expected = ((a.0)[0].extract(i) as u64 * (b.0)[0].extract(i) as u64) as u32;
                assert_eq!(c.extract(i), expected);
            }
        }

        #[test]
        fn test_mul_hi() {
            let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

            let a: FqReduced = rng.gen();
            let b: FqReduced = rng.gen();

            let c = m_hi(a.0[0], b.0[0]);

            let x = u32x8::new(0, 1, 2, 3, 4, 5, 6, 7);
            let x_s: u32x8 = unsafe {
                _mm256_permutevar8x32_epi32(
                    x.into_bits(),
                    u32x8::new(1, 0, 3, 2, 5, 4, 7, 6).into_bits(),
                )
            }
            .into_bits();
            assert_eq!(x_s, u32x8::new(1, 0, 3, 2, 5, 4, 7, 6));

            for i in 0..8 {
                let a_i = (a.0)[0].extract(i);
                let b_i = (b.0)[0].extract(i);
                let expected = ((a_i as u64 * b_i as u64) >> 32) as u32;

                println!("{}\n{:#b}\n{:#b}", i, expected, c.extract(i));
                assert_eq!(c.extract(i), expected);
            }
        }
    }
}

#[cfg(target_feature = "avx512f")]
pub mod avx512 {
    // 381 -> 12 * 32bit = 384

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use packed_simd::*;

    use backend;
    #[repr(simd)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Wi32x16(
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
        i32,
    );
    #[repr(simd)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Wu32x16(
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
        u32,
    );
    #[allow(improper_ctypes)]
    extern "C" {
        #[link_name = "llvm.x86.avx512.pmulu.dq.512"]
        fn _mm512_mul_epu32(a: u32x16, b: u32x16) -> u32x16;

        #[link_name = "llvm.x86.avx512.psrl.dq.512"]
        fn _mm512_srli_epi32(a: u32x16, imm8: i32) -> u32x16;
    }

    extern "platform-intrinsic" {
        pub fn simd_mul<T>(x: T, y: T) -> T;
    }

    unsafe fn _mm512_mullo_epi32(a: u32x16, b: u32x16) -> u32x16 {
        let a_i: Wi32x16 = std::mem::transmute(a);
        let b_i: Wi32x16 = std::mem::transmute(b);

        std::mem::transmute(simd_mul(a_i, b_i))
    }

    // -- Masks for conversion from 2^64 -> 2^29

    const PERM_MASK: [u16; 32] = [
        0, 1, 0, 0, 1, 2, 3, 0, 3, 4, 5, 0, 5, 6, 7, 0, 7, 8, 9, 0, 9, 10, 0, 0, 10, 11, 12, 0, 12,
        13, 14, 0,
    ];

    const SHIFT_MASK: [u64; 8] = [0, 13, 10, 7, 4, 1, 14, 11];

    // --Masks for conversion from 2^29 -> 2^64

    const FIX_MASK_0: [u32; 16] = [0, 1, 4, 1, 8, 1, 12, 1, 16, 1, 22, 1, 26, 1, 30, 1];
    const FIX_MASK_1: [u16; 32] = [
        4, 5, 3, 3, 12, 13, 3, 3, 20, 21, 3, 3, 28, 29, 3, 3, 36, 37, 3, 44, 48, 49, 3, 3, 56, 57,
        3, 3, 34, 35, 3, 3,
    ];
    const FIX_MASK_2: [u32; 16] = [4, 1, 8, 1, 12, 1, 16, 1, 20, 1, 26, 1, 30, 1, 19, 1];

    const FIX_SHIFT_0: [u32; 16] = [4, 1, 8, 1, 12, 1, 16, 1, 20, 1, 26, 1, 30, 1, 19, 1];
    const FIX_SHIFT_1: [u64; 8] = [29, 23, 17, 11, 5, 28, 22, 16];
    const FIX_SHIFT_2: [u64; 8] = [58, 52, 46, 40, 34, 57, 51, 45];

    const FIX_MASK3: [u32; 16] = [2, 1, 6, 1, 12, 1, 16, 1, 20, 1, 24, 1, 28, 1, 19, 1];
    const FIX_MASK_4: [u16; 32] = [
        8, 9, 3, 3, 16, 17, 3, 24, 28, 29, 3, 3, 36, 37, 3, 3, 44, 45, 3, 3, 52, 53, 3, 3, 60, 61,
        3, 38, 42, 43, 3, 3,
    ];

    const FIX_MASK_5: [u32; 16] = [6, 1, 10, 1, 16, 1, 20, 1, 24, 1, 28, 1, 17, 1, 23, 1];
    const FIX_SHIFT_3: [u64; 8] = [19, 25, 2, 8, 14, 20, 26, 3];
    const FIX_SHIFT_4: [u64; 8] = [10, 4, 27, 21, 15, 9, 3, 26];
    const FIX_SHIFT_5: [u64; 8] = [39, 33, 56, 50, 44, 38, 32, 55];

    const FIX_MASK_6: [u32; 16] = [6, 1, 10, 1, 14, 1, 18, 1, 24, 1, 28, 1, 17, 1, 21, 1];
    const FIX_MASK_7: [u16; 32] = [
        16, 17, 3, 3, 24, 25, 3, 3, 32, 33, 3, 3, 40, 41, 3, 48, 52, 53, 3, 3, 60, 61, 3, 3, 38,
        39, 3, 3, 46, 47, 3, 3,
    ];
    const FIX_MASK_8: [u32; 16] = [10, 1, 14, 1, 18, 1, 22, 1, 28, 1, 17, 1, 21, 1, 25, 1];
    const FIX_SHIFT_6: [u64; 8] = [9, 15, 21, 27, 4, 10, 16, 22];
    const FIX_SHIFT_7: [u64; 8] = [20, 14, 8, 2, 25, 19, 13, 7];
    const FIX_SHIFT_8: [u64; 8] = [49, 43, 37, 31, 54, 48, 42, 36];

    const FIX_MASK_9: [u32; 16] = [8, 1, 14, 1, 18, 1, 22, 1, 26, 1, 30, 1, 21, 1, 25, 1];
    const FIX_MASK_10: [u16; 32] = [
        20, 21, 3, 28, 32, 33, 3, 3, 40, 41, 3, 3, 48, 49, 3, 3, 56, 57, 3, 3, 34, 35, 3, 42, 46,
        47, 3, 3, 54, 55, 3, 3,
    ];
    const FIX_MASK_11: [u32; 16] = [12, 1, 18, 1, 22, 1, 26, 1, 30, 1, 19, 1, 25, 1, 29, 1];
    const FIX_SHIFT_9: [u64; 8] = [28, 5, 11, 17, 23, 29, 6, 12];
    const FIX_SHIFT_10: [u64; 8] = [1, 24, 18, 12, 6, 0, 23, 17];
    const FIX_SHIFT_11: [u64; 8] = [30, 53, 47, 41, 35, 29, 52, 46];

    /// Mask for the bottom 29 bits
    const AND_MASK: u64 = 0x1FFFFFFF;

    unsafe fn mul1024_avx512(dest: &mut [u64; 16], a: &[u64; 8], b: &[u64; 8]) {
        let zero = __mm512_setzero();
        let mut idx = __mm512_setzero();

        // -- Conversion from 2^64 -> 2^29
        let mut shift_mask = _mm512_setzero();
        shift_mask = _mm512_set_epi64(
            SHIFT_MASK[0],
            SHIFT_MASK[1],
            SHIFT_MASK[2],
            SHIFT_MASK[3],
            SHIFT_MASK[4],
            SHIFT_MASK[5],
            SHIFT_MASK[6],
            SHIFT_MASK[7],
        );

        let mut perm_mask = _mm512_setzero();
        perm_mask = _mm512_set_epi64()
    }

    // Layout: [[a_0, .., a_15], [a_16, .., a_23, 0, 0, 0, 0, 0, 0, 0, 0]]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqUnreduced(pub(crate) [u32x16; 2]);

    // Layout: [a_0, .., a_11, 0, 0, 0, 0]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqReduced(pub(crate) u32x16);

    impl ::rand::Rand for FqReduced {
        #[inline(always)]
        fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
            FqReduced(u32x16::new(
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                rng.gen::<u32>(),
                0,
                0,
                0,
                0,
            ))
        }
    }

    #[inline(never)]
    pub fn mul(a: &FqReduced, b: &FqReduced) -> FqUnreduced {
        // let a = a.0;
        // let b = b.0;

        let mut out = u32x16::default();
        let mut x_0: u32x16;
        let mut t: u32x16;

        macro_rules! round {
            ($i:expr, $a:expr, $b: expr, $t:expr, $out:expr, $x_0:expr, [
                $l0:expr, $l1:expr, $l2:expr, $l3:expr,
                $l4:expr, $l5:expr, $l6:expr, $l7:expr,
                $l8:expr, $l9:expr, $l10:expr, $l11:expr,
                $l12:expr, $l13:expr, $l14:expr, $l15:expr
            ]) => {
                $x_0 = $x_0 + m_hi($b, $t);

                let a_i = $a.0.extract($i);
                $t = u32x16::splat(a_i);
                $x_0 = $x_0 + m_lo($b, $t);

                // store x_0[0] at x[i]
                // $out[$i] = $x_0.extract(0);
                $out = shuffle!(
                    $out,
                    $x_0,
                    [
                        $l0, $l1, $l2, $l3, $l4, $l5, $l6, $l7, $l8, $l9, $l10, $l11, $l12, $l13,
                        $l14, $l15
                    ]
                );

                $x_0 = shr32($x_0);
            };
        }

        // TODO: the last 4 rows are empty and see if we can skip some operations

        {
            // first  round
            let a_i = a.0.extract(0);
            t = u32x16::splat(a_i);

            x_0 = m_lo(b.0, t);

            // store x_0[0] at x[i]
            // out[0] = x_0.extract(0);
            out = shuffle!(
                out,
                x_0,
                [16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
            );

            x_0 = shr32(x_0);
        }

        round!(
            1,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 16, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            2,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 16, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            3,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 16, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            4,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 16, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            5,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            6,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            7,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 16, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            8,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 7, 16, 9, 10, 11, 12, 13, 14, 15]
        );
        round!(
            9,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 16, 10, 11, 12, 13, 14, 15]
        );
        round!(
            10,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 16, 11, 12, 13, 14, 15]
        );
        round!(
            11,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 16, 12, 13, 14, 15]
        );
        round!(
            12,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 16, 13, 14, 15]
        );
        round!(
            13,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 16, 14, 15]
        );
        round!(
            14,
            a,
            b.0,
            t,
            out,
            x_0,
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 16, 15]
        );

        {
            // last round
            x_0 = x_0 + m_hi(b.0, t);
            // store x_0[0] at x[i]
            out = shuffle!(
                out,
                x_0,
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16]
            );
            // out[15] = x_0.extract(0);

            x_0 = shr32(x_0);
        }

        // store x_q-1..x_0 starting at x[m+1]
        // out[m..].copy_from_slice(x_0.into_bits());

        FqUnreduced([out, x_0])
    }

    #[inline]
    fn shr32(x: u32x16) -> u32x16 {
        unsafe { _mm512_srli_epi32(x.into_bits(), 32) }.into_bits()
    }

    #[inline]
    fn m_hi(x: u32x16, y: u32x16) -> u32x16 {
        // mul lo 32 bits into 64 bits
        let a: u32x16 = unsafe { _mm512_mul_epu32(x.into_bits(), y.into_bits()) }.into_bits();

        let x_s: u32x16 = shuffle!(x, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]);
        let y_s: u32x16 = shuffle!(y, [1, 0, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14]);
        let b: u32x16 = unsafe { _mm512_mul_epu32(x_s.into_bits(), y_s.into_bits()) }.into_bits();

        shuffle!(
            a,
            b,
            [1, 17, 3, 19, 5, 21, 7, 23, 9, 25, 11, 27, 13, 29, 15, 31]
        )
    }

    #[inline]
    fn m_lo(x: u32x16, y: u32x16) -> u32x16 {
        unsafe { _mm512_mullo_epi32(x, y) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul1024() {
        let mut res = [0; 16];
        let a = [2, 2, 0, 0, 0, 0, 0, 0];
        let b = [2, 2, 0, 0, 0, 0, 0, 0];

        unsafe { mul1024_avx512(&mut res, &a, &b) };

        assert_eq!([4, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,], res,);
    }
}
