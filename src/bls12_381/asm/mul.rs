use super::{FpDigit, FpWord, DIGIT_BIT, FP_SIZE};
use std::cmp;

#[inline]
pub fn mul(a: &[u64; 6], b: &[u64; 6]) -> [u64; 12] {
    // TODO: unrolled
    mul_comba(a, b)
}

#[inline(always)]
// needed because of https://github.com/rust-lang/rust/issues/24580
#[allow(unused_assignments)]
fn mul_comba(a: &[FpDigit; FP_SIZE], b: &[FpDigit; FP_SIZE]) -> [FpDigit; 12] {
    let mut c0: FpDigit = 0;
    let mut c1: FpDigit = 0;
    let mut c2: FpDigit = 0;

    comba_start!();
    comba_clear!(c0, c1, c2);

    let mut used_a = 6;
    let mut used_b = 6;
    for i in 0..FP_SIZE {
        if a[i] == 0 {
            used_a = i;
        }
        if b[i] == 0 {
            used_b = i;
        }
    }

    let pa = used_a + used_b;

    let mut dst = [0u64; 12];

    for ix in 0..pa {
        // get the offsets into the two bignums
        let ty = cmp::min(ix, used_b.wrapping_sub(1));
        let tx = ix - ty;

        // setup tmp aliases
        let mut tmpx = tx;
        let mut tmpy = ty;

        // this is the number of times the loop will iterrate, essentially its
        // while (tx++ < a->used && ty-- >= 0) { ... }
        let iy = cmp::min(used_a.wrapping_sub(tx), ty + 1);

        // execute loop
        comba_forward!(c0, c1, c2);
        for _iz in 0..iy {
            muladd!(c0, c1, c2, a[tmpx], b[tmpy]);
            tmpx += 1;
            tmpy = tmpy.wrapping_sub(1);
        }

        // store term
        comba_store!(c0, dst[ix]);
    }

    comba_fini!();

    dst
}

#[cfg(test)]
mod test {
    use super::mul;
    use num_bigint::BigUint;

    fn split_u64(i: u64) -> (u32, u32) {
        ((i & 0xFFFFFFFF) as u32, (i >> 32) as u32)
    }

    #[test]
    fn mul_range() {
        let range = vec![
            1,
            2,
            100000,
            u32::max_value() as u64,
            u64::max_value() - 1,
            u64::max_value(),
        ];

        for i in range.iter() {
            for j in range.iter() {
                for k in range.iter() {
                    for l in range.iter() {
                        let a_u64 = [*l, *k, *j, *i, 0, 0];
                        let b_u64 = [*j, *k, *i, *l, 1, 1];

                        let a_u64_slice: Vec<u32> = a_u64
                            .iter()
                            .flat_map(|v| vec![split_u64(*v).0, split_u64(*v).1])
                            .collect();
                        let b_u64_slice: Vec<u32> = b_u64
                            .iter()
                            .flat_map(|v| vec![split_u64(*v).0, split_u64(*v).1])
                            .collect();

                        let a_big = BigUint::from_slice(&a_u64_slice);
                        let b_big = BigUint::from_slice(&b_u64_slice);

                        let res_u64 = mul(&a_u64, &b_u64);

                        let res_big = a_big.clone() * b_big.clone();

                        let mut res_slice: Vec<u32> = res_u64
                            .iter()
                            .flat_map(|v| vec![split_u64(*v).0, split_u64(*v).1])
                            .collect();

                        let res_u64_big = BigUint::from_slice(&res_slice);
                        assert_eq!(res_big, res_u64_big);
                    }
                }
            }
        }
    }
}
