use mac_with_carry;
use std::cmp;
// warning, lots of wrapping ahead

// fq = [u64; 6] = 384bits;
// this means we need at least 384 * 2 = 768bits range.
const CHAR_BIT: usize = 8;
const FP_MAX_SIZE: usize = 384; // (4096 + (8 * DIGIT_BIT));
const DIGIT_BIT: usize = ((CHAR_BIT) * SIZEOF_FP_DIGIT);
const FP_SIZE: usize = (FP_MAX_SIZE / DIGIT_BIT);

// x86-64 => FP_64BIT = true
// x86-32, sse2, arm, ppc32 => FP_64BIT = false

// ECC384 => FP_64BIT ? MUL6, SQR6 : MUL12, SQR12

// if FP_64BIT
type FpDigit = u64;
const SIZEOF_FP_DIGIT: usize = 8;
type FpWord = u128;
const FP_WORD_SIZE: usize = 128;

// else
//   type fp_digit = u16;
//   const SIZEOF_FP_DIGIT = 4
//   type fp_word = u64;
//  const FP_WORD_SIZE = 64;
// end

struct FpInt {
    dp: [FpDigit; FP_SIZE],
    used: u16, // unused for now
    sign: u16,
}

///  a *= a
/// This work is based on https://github.com/libtom/tomsfastmath/blob/master/src/sqr/fp_sqr.c
// a is assumed to be in Little Endian order.
pub fn sqr(a: &mut [FpDigit; FP_SIZE]) {
    // for now only the generic version is available
    // TODO: investigate which unrolled versions make sense
    sqr_comba(a);
}

/// Generic comba squarer.
/// Calculates a <- a^2
fn sqr_comba(a: &mut [FpDigit; FP_SIZE]) {
    let mut c0: FpDigit = 0;
    let mut c1: FpDigit = 0;
    let mut c2: FpDigit = 0;

    // todo: correct used number
    let used = a.iter().position(|&x| x == 0).unwrap_or_else(|| 6);

    println!("sqr_comba {:?}^2", a);

    // output size
    // TODO: absolute limit
    let pa = 2 * used;

    let mut dst = vec![0; pa];

    comba_start();
    clear_carry(&mut c0, &mut c1, &mut c2);

    for ix in 0..pa {
        println!("outer: {}/{}", ix, pa);
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
        carry_forward(&mut c0, &mut c1, &mut c2);

        println!("iy {} ix: {} a {:?} ({}, {})", iy, ix, a, tmpx, tmpy);
        // execute loop
        for _iz in 0..iy {
            println!("tmpx, tmpy {} {}", tmpx, tmpy);
            sqradd2(&mut c0, &mut c1, &mut c2, a[tmpx], a[tmpy]);
            tmpx += 1;
            tmpy -= 1;
        }

        // even columns have the square term in them
        if (ix & 1) == 0 {
            sqradd(&mut c0, &mut c1, &mut c2, a[ix >> 1], a[ix >> 1]);
        }

        // store it
        println!("c0: {}", c0);
        comba_store(&mut c0, &mut dst[ix]);
        println!("dst[ix]: {}, {}", dst[ix], ix);
    }
    println!("fin {} {} {} ({:?}) ({:?})", c0, c1, c2, a, dst);
    comba_fini();

    // write back to a
    let len = cmp::min(pa, a.len());
    if len > 0 {
        a[0..len].copy_from_slice(&dst[0..len]);
    }
}

// -- portable implementation

#[inline(always)]
fn comba_start() {}

#[inline(always)]
fn clear_carry(c0: &mut FpDigit, c1: &mut FpDigit, c2: &mut FpDigit) {
    *c0 = 0;
    *c1 = 0;
    *c2 = 0;
}

#[inline(always)]
fn comba_store(c0: &mut FpDigit, x: &mut FpDigit) {
    *x = *c0;
}

#[inline(always)]
fn comba_store2(c1: &mut FpDigit, x: &mut FpDigit) {
    *x = *c1;
}

#[inline(always)]
fn carry_forward(c0: &mut FpDigit, c1: &mut FpDigit, c2: &mut FpDigit) {
    *c0 = *c1;
    *c1 = *c2;
    *c2 = 0;
}

#[inline(always)]
fn comba_fini() {}

/// Multiplies point i and j, updates carry `c1` and digit `c2`
#[inline(always)]
fn sqradd(c0: &mut FpDigit, c1: &mut FpDigit, c2: &mut FpDigit, i: FpDigit, j: FpDigit) {
    println!("sqradd {} {} {} {} {}", c0, c1, c2, i, j);
    let mut t = (*c0 as FpWord).wrapping_add((i as FpWord).wrapping_mul(j as FpWord));
    *c0 = t as FpDigit;

    t = (*c1 as FpWord).wrapping_add(t >> DIGIT_BIT as FpWord);
    *c1 = t as FpDigit;
    *c2 = c2.wrapping_add((t >> DIGIT_BIT as FpWord) as FpDigit);

    println!("sqradd-done {} {} {} {} {}", c0, c1, c2, i, j);
}

// For squaring some of the terms are doubled
#[inline(always)]
fn sqradd2(c0: &mut FpDigit, c1: &mut FpDigit, c2: &mut FpDigit, i: FpDigit, j: FpDigit) {
    println!("sqradd2 {} {} {} {} {}", c0, c1, c2, i, j);
    let t: FpWord = (i as FpWord).wrapping_mul(j as FpWord);
    let mut tt: FpWord = (*c0 as FpWord).wrapping_add(t);
    *c0 = tt as FpDigit;

    tt = (*c1 as FpWord).wrapping_add(tt >> DIGIT_BIT as FpWord);
    *c1 = tt as FpDigit;
    *c2 = c2.wrapping_add((tt >> DIGIT_BIT as FpWord) as FpDigit);

    tt = (*c0 as FpWord).wrapping_add(t);
    *c0 = tt as FpDigit;

    tt = (*c1 as FpWord).wrapping_add(tt >> DIGIT_BIT as FpWord);
    *c1 = tt as FpDigit;
    *c2 = c2.wrapping_add((tt >> DIGIT_BIT as FpWord) as FpDigit);
}

#[cfg(test)]
use rand::{Rand, Rng, SeedableRng, XorShiftRng};

#[test]
fn test_sqradd() {
    let mut c0 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    let i = 5;
    let j = 10;

    sqradd(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 50);
    assert_eq!(c1, 0);
    assert_eq!(c2, 0);

    sqradd(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 100);
    assert_eq!(c1, 150);
    assert_eq!(c2, 200);

    sqradd(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 150);
    assert_eq!(c1, 44);
    assert_eq!(c2, 244);
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

#[test]
fn test_mul() {
    // no overflow
    let a = 100;
    let b = 100;
    let c = 5;
    let mut carry = 1;
    let mut carry2 = 1;

    assert_eq!(
        mac(a, b, c, &mut carry),
        mac_with_carry(a, b, c, &mut carry2)
    );

    assert_eq!(carry, carry2);

    // with overflow

    let a = 18446744073709551615;
    let b = 10;
    let c = 5;
    let carry = &mut 0;
    let carry2 = &mut 0;

    assert_eq!(mac(a, b, c, carry), mac_with_carry(a, b, c, carry2));
    assert_eq!(carry, carry2);

    // range
    let mut rng = XorShiftRng::from_seed([0x5dbe6259, 0x8d313d76, 0x3237db17, 0xe5bc0654]);

    for _ in 0..1000 {
        let a: u64 = rng.gen();
        let b: u64 = rng.gen();
        let c: u64 = rng.gen();
        let mut carry: u64 = rng.gen();
        let mut carry2 = carry.clone();

        assert_eq!(
            mac(a, b, c, &mut carry),
            mac_with_carry(a, b, c, &mut carry2)
        );
        assert_eq!(carry, carry2);
    }
}

#[cfg(test)]
use num_bigint::BigUint;

fn split_u64(i: u64) -> (u32, u32) {
    ((i & 0xFFFFFFFF) as u32, (i >> 32) as u32)
}

#[test]
fn test_sqr_range() {
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

                    let mut a_res_u64 = a_u64.clone();
                    sqr(&mut a_res_u64);

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
