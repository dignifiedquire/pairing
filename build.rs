extern crate cc;

fn main() {
    cc::Build::new().file("mul.s").compile("my-asm-lib");
}
