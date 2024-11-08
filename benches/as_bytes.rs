#![feature(test)]
extern crate test;
use test::{black_box, Bencher};
use umbra_slice::UmbraString;

// On my machine:
//
// test boxed_str_as_bytes ... bench:           0.45 ns/iter (+/- 0.00)
// test umbra_str_as_bytes ... bench:           0.67 ns/iter (+/- 0.01)
//
// We need an extra comparison operation for Umbra strings. Regular strings, boxed strs and
// str references become bytes just by transmuting.

#[bench]
fn umbra_str_as_bytes(b: &mut Bencher) {
    let s: UmbraString = "a".repeat(50).try_into().unwrap();
    b.iter(|| black_box(&s).as_bytes());
}

#[bench]
fn boxed_str_as_bytes(b: &mut Bencher) {
    let s: Box<str> = "a".repeat(50).into();
    b.iter(|| black_box(&s).as_bytes());
}
