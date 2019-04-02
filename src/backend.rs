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

            // now extract the low 24 bits of each u32 and combine them to u64s
            let mut res = [0u64; 6];

            #[inline(always)]
            fn e24(a: u32) -> u64 {
                (a & 0b0000_0000_1111_1111_1111_1111_1111_1111) as u64
            }

            #[inline(always)]
            fn e16(a: u32) -> u64 {
                (a & 0b0000_0000_0000_0000_1111_1111_1111_1111) as u64
            }

            #[inline(always)]
            fn e8(a: u32) -> u64 {
                (a & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u64
            }

            // 16 + 24 + 24
            res[0] = e16(imm[2]) << 48 | e24(imm[1]) << 24 | e24(imm[0]);
            // 8 + 24 + 24 + 8
            res[1] = e8(imm[5]) << 56 | e24(imm[4]) << 32 | e24(imm[3]) << 8 | e24(imm[2]) >> 16;
            // 24 + 24 + 16
            res[2] = e24(imm[7]) << 40 | e24(imm[6]) << 16 | e24(imm[5]) >> 8;

            // 16 + 24 + 24
            res[3] = e16(imm[10]) << 48 | e24(imm[9]) << 24 | e24(imm[8]);
            // 8 + 24 + 24 + 8
            res[4] =
                e8(imm[13]) << 56 | e24(imm[12]) << 32 | e24(imm[11]) << 8 | e24(imm[10]) >> 16;
            // 24 + 24 + 16
            res[5] = e24(imm[15]) << 40 | e24(imm[14]) << 16 | e24(imm[13]) >> 8;

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
    // 381 -> 16 * 24bit = 384
    // this means a reduced element only uses the 21 bits in the last limb.

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    use packed_simd::*;

    use backend;

    const MASK24: u32 = 0b0000_0000_1111_1111_1111_1111_1111_1111;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqUnreduced(pub(crate) [u32x8; 4]);

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FqReduced(pub(crate) [u32x8; 2]);

    impl From<FqUnreduced> for [u64; 12] {
        fn from(other: FqUnreduced) -> [u64; 12] {
            let mut imm = [0u32; 32];
            (other.0)[0].write_to_slice_unaligned(&mut imm[..8]);
            (other.0)[1].write_to_slice_unaligned(&mut imm[8..16]);
            (other.0)[2].write_to_slice_unaligned(&mut imm[16..24]);
            (other.0)[3].write_to_slice_unaligned(&mut imm[24..]);

            // now extract the low 24 bits of each u32 and combine them to u64s
            let mut res = [0u64; 12];

            #[inline(always)]
            fn e24(a: u32) -> u64 {
                (a & 0b0000_0000_1111_1111_1111_1111_1111_1111) as u64
            }

            #[inline(always)]
            fn e16(a: u32) -> u64 {
                (a & 0b0000_0000_0000_0000_1111_1111_1111_1111) as u64
            }

            #[inline(always)]
            fn e8(a: u32) -> u64 {
                (a & 0b0000_0000_0000_0000_0000_0000_1111_1111) as u64
            }

            // 16 + 24 + 24
            res[0] = e16(imm[2]) << 48 | e24(imm[1]) << 24 | e24(imm[0]);
            // 8 + 24 + 24 + 8
            res[1] = e8(imm[5]) << 56 | e24(imm[4]) << 32 | e24(imm[3]) << 8 | e24(imm[2]) >> 16;
            // 24 + 24 + 16
            res[2] = e24(imm[7]) << 40 | e24(imm[6]) << 16 | e24(imm[5]) >> 8;

            // 16 + 24 + 24
            res[3] = e16(imm[10]) << 48 | e24(imm[9]) << 24 | e24(imm[8]);
            // 8 + 24 + 24 + 8
            res[4] =
                e8(imm[13]) << 56 | e24(imm[12]) << 32 | e24(imm[11]) << 8 | e24(imm[10]) >> 16;
            // 24 + 24 + 16
            res[5] = e24(imm[15]) << 40 | e24(imm[14]) << 16 | e24(imm[13]) >> 8;

            // 16 + 24 + 24
            res[6] = e16(imm[18]) << 48 | e24(imm[17]) << 24 | e24(imm[16]);
            // 8 + 24 + 24 + 8
            res[7] =
                e8(imm[21]) << 56 | e24(imm[20]) << 32 | e24(imm[19]) << 8 | e24(imm[18]) >> 16;
            // 24 + 24 + 16
            res[8] = e24(imm[23]) << 40 | e24(imm[22]) << 16 | e24(imm[21]) >> 8;

            // 16 + 24 + 24
            res[9] = e16(imm[26]) << 48 | e24(imm[25]) << 24 | e24(imm[24]);
            // 8 + 24 + 24 + 8
            res[10] =
                e8(imm[29]) << 56 | e24(imm[28]) << 32 | e24(imm[27]) << 8 | e24(imm[26]) >> 16;
            // 24 + 24 + 16
            res[11] = e24(imm[31]) << 40 | e24(imm[30]) << 16 | e24(imm[29]) >> 8;

            res
        }
    }

    impl ::rand::Rand for FqReduced {
        #[inline(always)]
        fn rand<R: ::rand::Rng>(rng: &mut R) -> Self {
            // Generates 24 bit random values
            FqReduced([
                u32x8::new(
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                ),
                u32x8::new(
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                    rng.gen::<u32>() & MASK24,
                ),
            ])
        }
    }

    /// Unpack 32-bit lanes into 64-bit lanes:
    /// ```ascii,no_run
    /// (a0, b0, a1, b1, c0, d0, c1, d1)
    /// ```
    /// into
    /// ```ascii,no_run
    /// (a0, 0, b0, 0, c0, 0, d0, 0)
    /// (a1, 0, b1, 0, c1, 0, d1, 0)
    /// ```
    #[inline(always)]
    fn unpack_pair(src: u32x8) -> (u32x8, u32x8) {
        let a: u32x8;
        let b: u32x8;
        let zero = i32x8::new(0, 0, 0, 0, 0, 0, 0, 0);
        unsafe {
            a = _mm256_unpacklo_epi32(src.into_bits(), zero.into_bits()).into_bits();
            b = _mm256_unpackhi_epi32(src.into_bits(), zero.into_bits()).into_bits();
        }
        (a, b)
    }

    pub fn mul(a: &FqReduced, b: &FqReduced) -> FqUnreduced {
        let m = 16;

        let a = a.0;
        let b = b.0;

        let mut out = [0u32; 16];
        let mut x_0 = u32x8::splat(0);
        let mut x_1 = u32x8::splat(0);
        let mut t = u32x8::splat(0);

        let b_0_s: u32x8 = shuffle!(b[0], [1, 0, 3, 2, 5, 4, 7, 6]);
        let b_1_s: u32x8 = shuffle!(b[1], [1, 0, 3, 2, 5, 4, 7, 6]);

        for i in 0..m {
            let tp = t;
            let tp_s = shuffle!(t, [1, 0, 3, 2, 5, 4, 7, 6]);

            if i == m {
                t = u32x8::splat(0);
            } else {
                let a_i = if i < 8 {
                    a[0].extract(i)
                } else {
                    a[1].extract(i - 8)
                };
                t = u32x8::splat(a_i);
            }

            x_0 = x_0 + m_lo(b[0], t);
            x_0 = x_0 + m_hi(b[0], tp, b_0_s, tp_s);

            x_1 = x_1 + m_lo(b[1], t);
            x_1 = x_1 + m_hi(b[1], tp, b_1_s, tp_s);

            // store x_0[0] at x[i]
            out[i] = x_0.extract(0);

            x_0 = shr32(x_0);
            x_1 = shr32(x_1);
        }

        // store x_q-1..x_0 starting at x[m+1]
        // out[m..].copy_from_slice(x_0.into_bits());

        FqUnreduced([
            u32x8::new(
                out[0], out[1], out[2], out[3], out[4], out[5], out[6], out[7],
            ),
            u32x8::new(
                out[8], out[9], out[10], out[11], out[12], out[13], out[14], out[15],
            ),
            x_0,
            x_1,
        ])
    }

    #[inline(always)]
    fn shr32(x: u32x8) -> u32x8 {
        let zero = u32x8::splat(0);
        shuffle!(x, zero, [8, 0, 1, 2, 3, 4, 5, 6])
    }

    #[inline(always)]
    fn m_hi(x: u32x8, y: u32x8, x_s: u32x8, y_s: u32x8) -> u32x8 {
        // mul lo 32 bits into 64 bits
        let a: u32x8 = unsafe { _mm256_mul_epu32(x.into_bits(), y.into_bits()) }.into_bits();
        let b: u32x8 = unsafe { _mm256_mul_epu32(x_s.into_bits(), y_s.into_bits()) }.into_bits();

        shuffle!(a, b, [1, 9, 3, 11, 5, 13, 7, 15])
    }

    #[inline(always)]
    fn m_lo(x: u32x8, y: u32x8) -> u32x8 {
        unsafe { _mm256_mullo_epi32(x.into_bits(), y.into_bits()).into_bits() }
    }

    // pub fn mul(a: &FqReduced, b: &FqReduced) -> FqUnreduced {
    //     #[inline(always)]
    //     fn m(x: u32x8, y: u32x8) -> u64x4 {
    //         unsafe { _mm256_mul_epu32(x.into_bits(), y.into_bits()).into_bits() }
    //     }

    //     let (x0, x1) = unpack_pair(a.0[0]);
    //     let (x2, x3) = unpack_pair(a.0[1]);

    //     let (y0, y1) = unpack_pair(b.0[0]);
    //     let (y2, y3) = unpack_pair(b.0[1]);

    //     let x1_2 = x1 + x1;
    //     let x3_2 = x3 + x3;

    //     // Long multiplication
    //     // (x0, x1, x2, x3) * (y0, y1, y2, y3)

    //     // c_ = 0
    //     //
    //     // r0 = (c_ + a0 * b0) mod D
    //     // c0 = (c_ + a0 * b0) / D
    //     //
    //     // r1 = (c0 + a1 * b0 + a0 * b1) mod D
    //     // c1 = (c0 + a1 * b0 + a0 * b1) / D
    //     //
    //     // r2 = (c1 + a0 * b2 + a2 * b0 + a1 * b1) mod D
    //     // c2 = (c1 + a0 * b2 + a2 * b0 + a1 * b1) / D
    //     //
    //     // r3 = (c2 + a0 * b3 + a3 * b0 + a1 * b2 + a2 * b1) mod D
    //     // c3 = (c2 + a0 * b3 + a3 * b0 + a1 * b2 + a2 * b1) / D
    //     //

    //     fn shr(x: u64x4) -> u64x4 {
    //         unsafe { _mm256_srli_epi32(x.into_bits(), 32).into_bits() }
    //     }
    //     // TODO: can we avoid shifting by using a mask?

    //     let z0 = m(x0, y0);
    //     let c0 = shr(z0);
    //     println!("z0: {:#b}", z0);
    //     println!("c0: {:#b}", c0);

    //     let z1 = c0 + m(x0, y1) + m(x1, y0);
    //     let c1 = shr(z1);

    //     let i2 = m(x0, y2) + m(x2, y0) + m(x1, y1);
    //     let z2 = c1 + i2;
    //     let c2 = shr(i2);

    //     let z3 = c2 + m(x0, y3) + m(x3, y0) + m(x1, y2) + m(x2, y1);

    //     // let z0 = m(x0, y0) + m(x1_2, y3) + m(x2, y2) + m(x3_2, y1);
    //     // let z1 = m(x0, y1) + m(x1, y0) + m(x2, y3) + m(x3, y2);
    //     // let z2 = m(x0, y2) + m(x1, y1) + m(x2, y0);
    //     // let z3 = m(x0, y3) + m(x1, y2) + m(x2, y1);

    //     FqUnreduced([
    //         z0.into_bits(),
    //         z1.into_bits(),
    //         z2.into_bits(),
    //         z3.into_bits(),
    //     ])
    // }

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
                    [1, 0, 0, 1, 0, 0],
                ),
                (
                    [
                        u32x8::new(
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_0000_0000_1111_1111_1111_1111,
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
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                        ),
                        u32x8::new(
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
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
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                (
                    [
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(1, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0],
                ),
                (
                    [
                        u32x8::new(
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_0000_0000_1111_1111_1111_1111,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                        u32x8::new(0, 0, 0, 0, 0, 0, 0, 0),
                    ],
                    [std::u64::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ),
                (
                    [
                        u32x8::new(
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                        ),
                        u32x8::new(
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                        ),
                        u32x8::new(
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                        ),
                        u32x8::new(
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
                            0b0000_0000_1111_1111_1111_1111_1111_1111,
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

            let a_s: u32x8 = shuffle!((a.0)[0], [1, 0, 3, 2, 5, 4, 7, 6]);
            let b_s: u32x8 = shuffle!((b.0)[0], [1, 0, 3, 2, 5, 4, 7, 6]);

            let c = m_hi(a.0[0], b.0[0], a_s, b_s);

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

        #[test]
        fn test_shr32() {
            let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

            let a: FqReduced = rng.gen();
            let v = (a.0)[0];

            let res = shr32(v);

            assert_eq!(res.extract(0), 0);
            for i in 1..8 {
                assert_eq!(res.extract(i), v.extract(i - 1));
            }
        }
    }
}
