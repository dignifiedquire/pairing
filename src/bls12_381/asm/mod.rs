#[macro_use]
mod mul_asm;

#[macro_use]
mod square_asm;

pub mod mul;
pub mod square;

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
