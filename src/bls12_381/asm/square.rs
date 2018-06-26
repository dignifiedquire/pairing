use super::{FpDigit, FpWord, DIGIT_BIT, FP_SIZE};
use std::cmp;

///  a *= a
/// This work is based on https://github.com/libtom/tomsfastmath/blob/master/src/sqr/fp_sqr.c
// a is assumed to be in Little Endian order.
#[inline]
pub fn sqr(a: &[FpDigit; FP_SIZE]) -> [FpDigit; 12] {
    // for now only the generic version is available
    // TODO: investigate unrolled versions
    // sqr_comba(a)
    sqr_comba6(a)
}

/// Generic comba squarer.
/// Calculates a <- a^2
#[inline(always)]
// needed because of https://github.com/rust-lang/rust/issues/24580
#[allow(unused_assignments)]
fn sqr_comba(a: &[FpDigit; FP_SIZE]) -> [FpDigit; 12] {
    let mut c0: FpDigit = 0;
    let mut c1: FpDigit = 0;
    let mut c2: FpDigit = 0;

    let mut used = 6;
    for (i, el) in a.iter().enumerate() {
        if *el == 0 {
            used = i;
            break;
        }
    }

    // output size
    // TODO: absolute limit
    let pa = 2 * used;

    let mut dst = [0u64; 12];

    comba_start!();
    clear_carry!(c0, c1, c2);

    for ix in 0..pa {
        // get offsets into the two bignums
        // TODO: used field
        let ty = cmp::min(used.wrapping_sub(1), ix);
        let tx = ix - ty;

        // setup temp aliases;
        let mut tmpx = tx;
        let mut tmpy = ty;

        // this is the number of times the loop will iterrate,
        // while (tx++ < a->used && ty-- >= 0) { ... }
        let mut iy = cmp::min(used.wrapping_sub(tx), ty + 1);

        // now for squaring tx can never equal ty
        // we halve the distance since they approach
        // at a rate of 2x and we have to round because
        // odd cases need to be executed
        iy = cmp::min(iy, (ty.wrapping_sub(tx).wrapping_add(1)) >> 1);

        // forward carries
        carry_forward!(c0, c1, c2);

        // execute loop
        for _iz in 0..iy {
            sqradd2!(c0, c1, c2, *unsafe { a.get_unchecked(tmpx) }, *unsafe {
                a.get_unchecked(tmpy)
            });
            tmpx += 1;
            tmpy -= 1;
        }

        // even columns have the square term in them
        if (ix & 1) == 0 {
            sqradd!(c0, c1, c2, *unsafe { a.get_unchecked(ix >> 1) }, *unsafe {
                a.get_unchecked(ix >> 1)
            });
        }

        // store it
        comba_store!(c0, dst[ix]);
    }

    comba_fini!();

    dst
}

#[inline(always)]
fn sqr_comba6(a: &[FpDigit; FP_SIZE]) -> [FpDigit; 12] {
    let mut c0: FpDigit = 0;
    let mut c1: FpDigit = 0;
    let mut c2: FpDigit = 0;
    let mut sc0: FpDigit = 0;
    let mut sc1: FpDigit = 0;
    let mut sc2: FpDigit = 0;

    let mut b: [FpDigit; 12] = [0; 12];

    comba_start!();

    // clear carries
    clear_carry!(c0, c1, c2);

    // output 0
    sqradd!(c0, c1, c2, a[0], a[0]);
    comba_store!(c0, b[0]);

    // output 1
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[0], a[1]);
    comba_store!(c0, b[1]);

    // output 2
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[0], a[2]);
    sqradd!(c0, c1, c2, a[1], a[1]);
    comba_store!(c0, b[2]);

    // output 3
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[0], a[3]);
    sqradd2!(c0, c1, c2, a[1], a[2]);
    comba_store!(c0, b[3]);

    // output 4
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[0], a[4]);
    sqradd2!(c0, c1, c2, a[1], a[3]);
    sqradd!(c0, c1, c2, a[2], a[2]);
    comba_store!(c0, b[4]);

    // output 5
    carry_forward!(c0, c1, c2);
    sqraddsc!(sc0, sc1, sc2, a[0], a[5]);
    sqraddac!(sc0, sc1, sc2, a[1], a[4]);
    sqraddac!(sc0, sc1, sc2, a[2], a[3]);
    sqradddb!(sc0, sc1, sc2, c0, c1, c2);
    comba_store!(c0, b[5]);

    // output 6
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[1], a[5]);
    sqradd2!(c0, c1, c2, a[2], a[4]);
    sqradd!(c0, c1, c2, a[3], a[3]);
    comba_store!(c0, b[6]);

    // output 7
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[2], a[5]);
    sqradd2!(c0, c1, c2, a[3], a[4]);
    comba_store!(c0, b[7]);

    // output 8
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[3], a[5]);
    sqradd!(c0, c1, c2, a[4], a[4]);
    comba_store!(c0, b[8]);

    // output 9
    carry_forward!(c0, c1, c2);
    sqradd2!(c0, c1, c2, a[4], a[5]);
    comba_store!(c0, b[9]);

    // output 10
    carry_forward!(c0, c1, c2);
    sqradd!(c0, c1, c2, a[5], a[5]);
    comba_store!(c0, b[10]);
    comba_store2!(c1, b[11]);
    comba_fini!();

    b
}

#[cfg(test)]
mod test {
    use super::sqr;
    use num_bigint::BigUint;

    fn split_u64(i: u64) -> (u32, u32) {
        ((i & 0xFFFFFFFF) as u32, (i >> 32) as u32)
    }

    #[test]
    fn sqr_range() {
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
                        println!("\n----\n[{}, {}, {}, {}, {}, {}]^2\n", l, k, j, i, 0, 0);

                        let a_u64 = [*l, *k, *j, *i, 0, 0];

                        let a_u64_slice: Vec<u32> = a_u64
                            .iter()
                            .flat_map(|v| vec![split_u64(*v).0, split_u64(*v).1])
                            .collect();
                        let a_big = BigUint::from_slice(&a_u64_slice);

                        let a_res_u64 = sqr(&a_u64);

                        let a_res_big = a_big.clone() * a_big.clone();

                        println!(
                            "{:?} {:?} {:?} {:?}",
                            a_big.to_bytes_le(),
                            a_res_big.to_bytes_le(),
                            a_u64,
                            a_res_u64
                        );
                        let mut a_res_slice: Vec<u32> = a_res_u64
                            .iter()
                            .flat_map(|v| vec![split_u64(*v).0, split_u64(*v).1])
                            .collect();

                        let a_res_u64_big = BigUint::from_slice(&a_res_slice);
                        assert_eq!(a_res_big, a_res_u64_big);
                    }
                }
            }
        }
    }
}
