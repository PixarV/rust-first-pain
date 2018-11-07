#![feature(no_panic_pow)]
#[macro_use] extern crate text_io;

mod prime;
mod scalar;

static ARR1: &'static [i32] = &[2, 3, 4, 5];
static ARR2: &'static [i32] = &[2, 3, 4, 5, 6];

fn main() {
    let num : i64 = read!();

    let result = if prime::is_prime(num as u64) { "yes" } else { "no" };
    println!("{}", result);

    println!("{}", scalar::compute_scalar(ARR1, ARR2));
}
