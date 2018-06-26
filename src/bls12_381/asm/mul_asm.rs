#[macro_export]
macro_rules! comba_start {
    () => {};
}

#[macro_export]
macro_rules! comba_clear {
    ($c0:ident, $c1:ident, $c2:ident) => {
        $c0 = 0;
        $c1 = 0;
        $c2 = 0;
    };
}

#[macro_export]
macro_rules! comba_store {
    ($c0:ident, $x:expr) => {
        $x = $c0;
    };
}
#[macro_export]
macro_rules! comba_store2 {
    ($c1:ident, $x:expr) => {
        $x = $c1;
    };
}

#[macro_export]
macro_rules! comba_forward {
    ($c0:ident, $c1:ident, $c2:ident) => {
        $c0 = $c1;
        $c1 = $c2;
        $c2 = 0;
    };
}

#[macro_export]
macro_rules! comba_fini {
    () => {};
}

/// Multiplies point i and j, updates carry `c1` and digit `c2`
#[macro_export]
macro_rules! muladd {
    ($c0:ident, $c1:ident, $c2:ident, $i:expr, $j:expr) => {{
        let mut t = ($c0 as FpWord).wrapping_add(($i as FpWord).wrapping_mul($j as FpWord));
        $c0 = t as FpDigit;

        t = ($c1 as FpWord).wrapping_add(t >> DIGIT_BIT as FpWord);
        $c1 = t as FpDigit;
        $c2 = $c2.wrapping_add((t >> DIGIT_BIT as FpWord) as FpDigit);
    }};
}
