// -- portable implementation

#[macro_export]
macro_rules! clear_carry {
    ($c0:ident, $c1:ident, $c2:ident) => {
        $c0 = 0;
        $c1 = 0;
        $c2 = 0;
    };
}

#[macro_export]
macro_rules! carry_forward {
    ($c0:ident, $c1:ident, $c2:ident) => {
        $c0 = $c1;
        $c1 = $c2;
        $c2 = 0;
    };
}

/// Multiplies point i and j, updates carry `c1` and digit `c2`
#[macro_export]
macro_rules! sqradd {
    ($c0:ident, $c1:ident, $c2:ident, $i:expr, $j:expr) => {{
        // don't need to use j, it is always the same as i in thes invocations
        let mut t = ($c0 as FpWord).wrapping_add(($i as FpWord).pow(2));
        $c0 = t as FpDigit;

        t = ($c1 as FpWord).wrapping_add(t >> DIGIT_BIT as FpWord);
        $c1 = t as FpDigit;

        $c2 = $c2.wrapping_add((t >> DIGIT_BIT as FpWord) as FpDigit);
    }};
}

// #[cfg(not(target_arch = "x86_64"))]
// #[macro_export]
// macro_rules! sqradd {
//     ($c0:ident, $c1:ident, $c2:ident, $i:expr, $j:expr) => {
//         // note: this does not actually use `j`, as i == j for all invocations.
//         println!("{} {} {} {} {}", $c0, $c1, $c2, $i, $j);
//         unsafe {
//             asm!(
//                                         "movq $6, %rax;  // rax     <- i         \n\t\
//                                          mulq %rax    ;  // rdx:rax <- rax * rax \n\t\
//                                          addq %rax, $0;  // c0      <- c0  + rax \n\t\
//                                          adcq %rdx, $1;  // c1      <- c1  + rdx \n\t\
//                                          adcq $0, $2  ;  // c2      <- c0  + c2  \n\t\
//                                          "
//                                             : "=r"($c0), "=r"($c1), "=r"($c2)       // outputs
//                                             : "0"($c0), "1"($c1), "2"($c2), "r"($i) // inputs
//                                             : "rax", "rdx", "cc"                    // clobbers
//                                     );
//         }
//         println!("{} {} {} {} {}", $c0, $c1, $c2, $i, $j);
//     };
// }

// For squaring some of the terms are doubled
#[macro_export]
macro_rules! sqradd2 {
    ($c0:ident, $c1:ident, $c2:ident, $i:expr, $j:expr) => {{
        let t: FpWord = ($i as FpWord).wrapping_mul($j as FpWord);
        let mut tt: FpWord = ($c0 as FpWord).wrapping_add(t);
        $c0 = tt as FpDigit;

        tt = ($c1 as FpWord).wrapping_add(tt >> DIGIT_BIT as FpWord);
        $c1 = tt as FpDigit;
        $c2 = $c2.wrapping_add((tt >> DIGIT_BIT as FpWord) as FpDigit);

        tt = ($c0 as FpWord).wrapping_add(t);
        $c0 = tt as FpDigit;

        tt = ($c1 as FpWord).wrapping_add(tt >> DIGIT_BIT as FpWord);
        $c1 = tt as FpDigit;
        $c2 = $c2.wrapping_add((tt >> DIGIT_BIT as FpWord) as FpDigit);
    }};
}

// #[cfg(not(target_arch = "x86_64"))]
// #[macro_export]
// macro_rules! sqradd2 {
//     ($c0:ident, $c1:ident, $c2:ident, $i:expr, $j:expr) => {
//         println!("{} {} {} {} {}", $c0, $c1, $c2, $i, $j);
//         unsafe {
//             asm!(
//                                         "movq $6, %rax;  // rax     <- i         \n\t\
//                                          mulq $7      ;  // rdx:rax <- rax * j   \n\t\
//                                          addq %rax, $0;  // c0      <- c0  + rax \n\t\
//                                          adcq %rdx, $1;  // c1      <- c1  + rdx \n\t\
//                                          adcq $0, $2  ;  // c2      <- c0  + c2  \n\t\
//                                          addq %rax, $0; //  c0      <- c0 + rax  \n\t\
//                                          adcq %rdx, $1; //  c1      <- c1 + rdx  \n\t\
//                                          adcq $0, $2  ; //  c2      <- c0 + c2   \n\t\
//                                          "
//                                             : "=r"($c0), "=r"($c1), "=r"($c2)
//                                             : "0"($c0), "1"($c1), "2"($c2), "r"($i), "r"($j)
//                                             : "rax", "rdx", "cc"
//                                     );
//         }
//         println!("{} {} {} {} {}", $c0, $c1, $c2, $i, $j);
//     };
// }

#[macro_export]
macro_rules! sqraddsc {
    ($sc0:ident, $sc1:ident, $sc2:ident, $i:expr, $j:expr) => {{
        let mut t = ($i as FpWord) * ($j as FpWord);
        $sc0 = t as FpDigit;
        $sc1 = (t >> DIGIT_BIT as FpWord) as FpDigit;
        $sc2 = 0;
    }};
}

#[macro_export]
macro_rules! sqraddac {
    ($sc0:ident, $sc1:ident, $sc2:ident, $i:expr, $j:expr) => {{
        let mut t = ($sc0 as FpWord).wrapping_add(($i as FpWord) * ($j as FpWord));
        $sc0 = t as FpDigit;

        t = ($sc1 as FpWord).wrapping_add(t >> DIGIT_BIT as FpWord);
        $sc1 = t as FpDigit;
        $sc2 = ($sc2 as FpWord).wrapping_add(t >> DIGIT_BIT as FpWord) as FpDigit;
    }};
}

#[macro_export]
macro_rules! sqradddb {
    ($sc0:ident, $sc1:ident, $sc2:ident, $c0:ident, $c1:ident, $c2:ident) => {{
        let mut t = ($sc0 as FpWord)
            .wrapping_add($sc0 as FpWord)
            .wrapping_add($c0 as FpWord);
        $c0 = t as FpDigit;

        t = ($sc1 as FpWord)
            .wrapping_add($sc1 as FpWord)
            .wrapping_add($c1 as FpWord)
            .wrapping_add(t >> DIGIT_BIT as FpWord);
        $c1 = t as FpDigit;

        $c2 = ($c2 as FpWord)
            .wrapping_add($sc2 as FpWord)
            .wrapping_add($sc2 as FpWord)
            .wrapping_add(t >> DIGIT_BIT as FpWord) as FpDigit;
    }};
}
