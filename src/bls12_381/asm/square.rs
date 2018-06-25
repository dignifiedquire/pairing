use mac_with_carry;
use std::cmp;

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
type FpWord = u32;
// else
//   type fp_digit = u16;
//   const SIZEOF_FP_DIGIT = 4
//   type fp_word = u64;
// end

struct FpInt {
    dp: [FpDigit; FP_SIZE],
    used: u16, // unused for now
    sign: u16,
}

///  a *= a
/// This work is based on https://github.com/libtom/tomsfastmath/blob/master/src/sqr/fp_sqr.c
pub fn sqr(a: &mut [FpDigit; FP_SIZE]) {
    // for now only the generic version is available
    // TODO: investigate which unrolled versions make sense
    let mut fp = FpInt {
        dp: *a,
        used: 0,
        sign: 0,
    };
    sqr_comba(&mut fp);
}

/// Generic comba squarer.
fn sqr_comba(a: &mut FpInt) {
    let mut c0: FpDigit = 0;
    let mut c1: FpDigit = 0;
    let mut c2: FpDigit = 0;
    let mut dst = FpInt {
        dp: [0; FP_SIZE],
        used: 0,
        sign: 0,
    };
    let mut tt: FpWord = 0;

    // output size
    // TODO: use used field
    let pa = FP_SIZE - 1;

    comba_start();
    clear_carry(&mut c0, &mut c1, &mut c2);

    for ix in 0..pa {
        // get offsets into the two bignums
        // TODO: used field
        let ty = cmp::min(FP_SIZE - 1, ix);
        let tx = ix - ty;

        // setup temp aliases;
        let mut tmpx = tx;
        let mut tmpy = ty;

        // loop number
        // TODO: used field
        let mut iy = cmp::min(FP_SIZE - tx, ty + 1);

        // now for squaring tx can never equal ty
        // we halve the distance since they approach
        // at a rate of 2x and we have to round because
        // odd cases need to be executed
        iy = cmp::min(iy, (ty - tx + 1) >> 1);

        // forward carries
        carry_forward(&mut c0, &mut c1, &mut c2);

        // execute loop

        for _iz in 0..iy {
            tmpx += 1;
            tmpy -= 1;
            sqradd2(&mut c0, &mut c1, &mut c2, a.dp[tmpx], a.dp[tmpy]);
        }

        // even columns have the square term in them
        if (ix & 1) == 0 {
            sqradd(&mut c0, &mut c1, &mut c2, a.dp[ix >> 1], a.dp[ix >> 1]);
        }

        // store it
        println!("c1: {}", c1);
        comba_store(&mut c0, &mut dst.dp[ix]);
        println!("dst[ix]: {}", dst.dp[ix]);
    }

    comba_fini();

    // setup dest
    // TODO: improve perf
    *a = dst;
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
    let mut t = c0.wrapping_add(i.wrapping_mul(j));
    *c0 = t;
    t = c1.wrapping_add(t.wrapping_shr(8));
    *c1 = t;
    *c2 = c2.wrapping_add(t.wrapping_shr(8));
}

// For squaring some of the terms are doubled
#[inline(always)]
fn sqradd2(c0: &mut FpDigit, c1: &mut FpDigit, c2: &mut FpDigit, i: FpDigit, j: FpDigit) {
    let t = i.wrapping_mul(j);
    let mut tt = c0.wrapping_add(t);
    *c0 = tt;
    tt = c1.wrapping_add(tt.wrapping_shr(8));
    *c1 = tt;
    *c2 = c2.wrapping_add(tt.wrapping_shr(8));
    tt = c0.wrapping_add(t);
    *c0 = tt;
    tt = c1.wrapping_add(tt.wrapping_shr(8));
    *c1 = tt;
    *c2 = c2.wrapping_add(tt.wrapping_shr(8));
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
    assert_eq!(c1, 50);
    assert_eq!(c2, 50);

    sqradd(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 100);
    assert_eq!(c1, 150);
    assert_eq!(c2, 200);

    sqradd(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 150);
    assert_eq!(c1, 44);
    assert_eq!(c2, 244);
}

#[test]
fn test_sqradd2() {
    let mut c0 = 0;
    let mut c1 = 0;
    let mut c2 = 0;
    let i = 5;
    let j = 10;

    sqradd2(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 100);
    assert_eq!(c1, 150);
    assert_eq!(c2, 200);

    sqradd2(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 200);
    assert_eq!(c1, 244);
    assert_eq!(c2, 232);

    sqradd2(&mut c0, &mut c1, &mut c2, i, j);

    assert_eq!(c0, 44);
    assert_eq!(c1, 26);
    assert_eq!(c2, 240);
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

#[test]
fn test_sqr() {
    for i in 0..256 {
        let mut a = [0, 0, 0, 0, 0, i as u64];
        sqr(&mut a);
        assert_eq!(
            a.to_vec(),
            vec![0, 0, 0, 0, 0, (i * i) as u64],
            "{}^2 = {}",
            i,
            i * i
        );
    }
}
