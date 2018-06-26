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
        let mut t = ($c0 as FpWord).wrapping_add(($i as FpWord).wrapping_mul($j as FpWord));
        $c0 = t as FpDigit;

        t = ($c1 as FpWord).wrapping_add(t >> DIGIT_BIT as FpWord);
        $c1 = t as FpDigit;
        $c2 = $c2.wrapping_add((t >> DIGIT_BIT as FpWord) as FpDigit);
    }};
}

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
