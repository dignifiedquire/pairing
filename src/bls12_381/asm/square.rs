use super::{FpDigit, FpWord, DIGIT_BIT, FP_SIZE};
use std::cmp;

///  a *= a
/// This work is based on https://github.com/libtom/tomsfastmath/blob/master/src/sqr/fp_sqr.c
// a is assumed to be in Little Endian order.
#[inline]
pub fn sqr(a: &[FpDigit; FP_SIZE]) -> [FpDigit; 12] {
    // for now only the generic version is available
    // TODO: investigate unrolled versions
    sqr_comba(a)
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
        // println!("outer: {}/{}", ix, pa);
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
            // println!("tmpx, tmpy {} {}", tmpx, tmpy);
            sqradd2!(c0, c1, c2, a[tmpx], a[tmpy]);
            tmpx += 1;
            tmpy -= 1;
        }

        // even columns have the square term in them
        if (ix & 1) == 0 {
            sqradd!(c0, c1, c2, a[ix >> 1], a[ix >> 1]);
        }

        // store it
        comba_store!(c0, dst[ix]);
    }

    comba_fini!();

    dst
}

/// Calculate `a + b * c`.
#[cfg(target_arch = "x86_64")]
#[inline(never)]
pub fn mac(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let mut res = a;

    // println!("before: {} + {} * {} = {:?} ({})", a, b, c, res, carry);

    unsafe {
        asm!(
            "// movq $4, %rax ; // rax = b               \n\t\
             mulq $5       ; // rdx:rax = rax * c     \n\t\
             addq %rdx, $1 ; // carry += rax            \n\t\
             adcq %rax, $0 ; // res += rdx          \n\t\
             "
                : "=&r"(res), "=&r"(*carry)
                : "0"(res), "1"(*carry), "{rax}"(b), "r"(c)
                : "%rax", "%rdx", "cc"
                : "volatile"
        );
    }
    println!("after: {} + {} * {} = {:?} ({})", a, b, c, res, carry);

    res
}

// #[cfg(not(target_arch = "x86_64"))]
pub fn mac_fallback(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let mut c0 = a;
    let mut c1 = 0u64;
    let mut c2 = 0u64;

    let mut t: u64 = c0.wrapping_add(b.wrapping_mul(c));
    println!("t: {}", t);
    c0 = t;
    println!("c0: {}", c0);
    t = c1.wrapping_add(t.wrapping_shr(8));
    println!("t: {}", t);
    c1 = t;
    c2 = c2.wrapping_add(t.wrapping_shr(8));
    println!("c2: {}", c2);

    println!("{} {} {}", c0, c1, c2);
    *carry = c2;
    c0
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
