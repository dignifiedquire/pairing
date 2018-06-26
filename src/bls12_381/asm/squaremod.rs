use super::{FpDigit, FpWord, DIGIT_BIT, FP_SIZE};

/// Calculate `a^2 mod p`
pub fn sqr_mod(a: &[u64; 6], p: &[u64; 6], mp: u64) -> [u64; 6] {
    let M: [[FpDigit; FP_SIZE]; 64];
    // TODO: size = 12
    let mut res: [FpDigit; FP_SIZE] = [0; 6];

    // window size is always 1, as we square
    let winsize = 1;

    // setup is already done as p and mp are assumed to be fixed

    // create M table
    // The M table contains powers of the input base, e.g. M[x] = G^x mod P
    // The first half is not computed, except for M[0] and M[1].

    // TODO: can be static
    montgomery_calc_normalization(&mut res, p);

    // return the last digits
    // [res[6], res[7], res[8], res[9], res[10], res[11]]
    res
}

/// Computes a = B**n mod b without division or multiplication.
#[inline]
fn montgomery_calc_normalization(a: &mut [FpDigit; FP_SIZE], b: &[FpDigit; FP_SIZE]) {
    // how many bits of last digit does b ues
    let mut bits = count_bits(b) % DIGIT_BIT;
    if bits == 0 {
        bits = DIGIT_BIT;
    }

    let used_b = FP_SIZE;

    // compute A = B^(n-1) * 2^(bits-1)
    epxt2(a, (used_b - 1) * DIGIT_BIT + bits - 1);

    // now compute C = A * B mod b
    for x in (bits - 1)..DIGIT_BIT {
        mul2(a);
        if cmp_mag(a, b) != -1 {
            u_sub(a, b);
        }
    }
}

/// Computes a = 2^b.
#[inline]
fn epxt2(a: &mut [FpDigit; FP_SIZE], b: usize) {
    unimplemented!("");
}

#[inline]
fn cmp_mag(a: &[FpDigit; FP_SIZE], b: &[FpDigit; FP_SIZE]) -> isize {
    unimplemented!("");
}

/// Calculates a = 2 * a
#[inline]
fn mul2(a: &mut [FpDigit; FP_SIZE]) {
    unimplemented!("");
}

/// Unsigned subtraction `||a|| >= ||b`.
/// Calculates a = a - b
#[inline]
fn u_sub(a: &mut [FpDigit; FP_SIZE], b: &[FpDigit; FP_SIZE]) {
    // TODO: used..
    let used = FP_SIZE;
    let mut t = 0;

    for x in 0..used {
        t = (a[x] as FpWord) - ((b[x] as FpWord) + t);
        a[x] = t as FpDigit;
        t = (t >> DIGIT_BIT) & 1;
    }
}

#[inline]
fn count_bits(a: &[FpDigit; FP_SIZE]) -> usize {
    let mut used = FP_SIZE;
    for i in 0..FP_SIZE {
        if a[i] == 0 {
            used = i;
            break;
        }
    }

    if used == 0 {
        return 0;
    }

    let mut r = (used - 1) * DIGIT_BIT;
    let mut q = a[used - 1];

    while q > 0 {
        r += 1;
        q >>= 1;
    }

    r
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_bits() {
        assert_eq!(count_bits(&[0u64; 6]), 0);
        assert_eq!(count_bits(&[1u64; 6]), 5 * DIGIT_BIT + 1);

        let mut other = [0u64; 6];
        other[0] = 1;
        assert_eq!(count_bits(&other), 1);
    }

    #[test]
    fn test_u_sub() {
        let mut a = [1; 6];
        let b = [1; 6];
        u_sub(&mut a, &b);
        assert_eq!(a.to_vec(), vec![0u64; 6]);
    }
}
