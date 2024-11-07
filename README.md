# umbra_slice

`umbra_slice` provides a generic slice type that can replace `Box<[T]>` and `Box<str>`. `UmbraSlice<T>` is optimized to store small slices inline in a layout based on ["German strings"](https://cedardb.com/blog/german_strings/).

`umbra_slice` was extracted from [`spellbook`](https://github.com/helix-editor/spellbook). See the post ["German string" optimizations in Spellbook](https://the-mikedavis.github.io/posts/german-string-optimizations-in-spellbook/) for more details.

## Quick facts

* Takes 16 bytes on the stack.
* Stores up to 14 bytes inline.
* Optimization for `Eq` to disqualify strings with different lengths and prefixes quickly.
* Can be used to store 16 or 32 bit integers as well as `u8`s.

(Assuming a 64-bit machine.)

## Use-cases

`UmbraSlice<T>` and `UmbraString` are designed for a specific use-case and should not be thought of as generic replacements for `Vec<T>`/`Box<[T]>` and `String`/`Box<str>`, respectively. If you need a generic replacement, consider one of many great existing crates like [`smallvec`](https://crates.io/crates/smallvec/1.13.2). `UmbraSlice<T>` differs from most existing crates because it focuses on emulating non-resizable, immutable slices rather than resizable and mutable `Vec<T>`s (which typically take 24 bytes on the stack).

Umbra slices are immutable and have a shorter maximum length than Vecs or boxed slices. Umbra slices are meant to be used when you have very many (thousands+) slices or strings which are typically smaller than 14 bytes.

## License

Licensed under either of:

* MPL version 2.0 ([LICENSE-MPL](LICENSE-MPL) or <https://www.mozilla.org/en-US/MPL/2.0/>)
* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)

at your option.
