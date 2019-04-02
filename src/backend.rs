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
                $x_0 = $x_0 + m_hi($b[0], $t);
                $x_1 = $x_1 + m_hi_half($b[1], $t);

                let a_i = $a.extract($i);
                $t = u32x8::splat(a_i);

                $x_0 = $x_0 + m_lo($b[0], $t);
                $x_1 = $x_1 + m_lo_half($b[1], $t);

                // store x_0[0] at x[i]
                $out = shuffle!($out, $x_0, [$l0, $l1, $l2, $l3, $l4, $l5, $l6, $l7]);

                $x_0 = shr32($x_0);
                $x_1 = shr32($x_1);
            };
        }

        {
            // first round
            let a_i = a.0[0].extract(0);
            t = u32x8::splat(a_i);

            x_0 = m_lo(b.0[0], t);
            x_1 = m_lo_half(b.0[1], t);

            // store x_0[0] at x[i]
            out_0 = shuffle!(out_0, x_0, [8, 1, 2, 3, 4, 5, 6, 7]);

            x_0 = shr32(x_0);
            x_1 = shr32(x_1);
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

        {
            // last round
            x_0 = x_0 + m_hi(b.0[0], t);

            // store x_0[0] at x[i]
            out_1 = shuffle!(out_1, x_0, [0, 1, 2, 3, 4, 5, 6, 8]);
            x_0 = shr32(x_0);
        }

        // store x_q-1..x_0 starting at x[m+1]
        // out[m..].copy_from_slice(x_0.into_bits());

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
            ];

            for (i, case) in cases.into_iter().enumerate() {
                let a = FqReduced((case.0).0);
                let b = FqReduced((case.0).1);

                let res: [u64; 12] = mul(&a, &b).into();

                assert_eq!(res, case.1, "digit {}", i);
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

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqUnreduced(pub(crate) [u32x16; 2]);

    // Layout: [a_0, .., a_7, a_8, 0, a_9, 0, a_10, 0, a_11, 0]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqReduced(pub(crate) u32x16);

    impl From<FqUnreduced> for [u64; 12] {
        fn from(other: FqUnreduced) -> [u64; 12] {
            let mut imm = [0u32; 32];
            (other.0)[0].write_to_slice_unaligned(&mut imm[..16]);
            (other.0)[1].write_to_slice_unaligned(&mut imm[16..]);

            // now extract the low 24 bits of each u32 and combine them to u64s
            let mut res = [0u64; 12];

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

            res[6] = c(imm[16], imm[17]);
            res[7] = c(imm[18], imm[19]);
            res[8] = c(imm[20], imm[21]);
            res[9] = c(imm[22], imm[23]);

            res[10] = c(imm[24], imm[26]);
            res[11] = c(imm[28], imm[30]);

            res
        }
    }

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
                0,
                rng.gen::<u32>(),
                0,
                rng.gen::<u32>(),
                0,
                rng.gen::<u32>(),
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

        // TODO: move values such that the last 4 rows are empty and see if we can skip some operations

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
