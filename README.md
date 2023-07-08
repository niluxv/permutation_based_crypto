# Permutation based cryptography

Permutation based cryptography in Rust.

## Crates

* `crypto-permutation`: This crate contains the core traits for abstracting over cryptographic
  permutations and deck functions.
* `permutation-keccak`: Implementation of the `Permutation` trait for the [Keccak-p permutation].
* `permutation-xoodoo`: Implementation of the `Permutation` trait for the [Xoodoo permutation].
* `deck-farfalle`: Generic [Farfalle construction] and the [Kravatte] and [Xoofff] instantiations.

## License
All crates in this repository are dual licensed MIT or Apache 2.0 at your option.

[Keccak-p permutation]: https://keccak.team/keccakp.html
[Xoodoo permutation]: https://keccak.team/xoodoo.html
[Farfalle construction]: https://keccak.team/farfalle.html
[Kravatte]: https://keccak.team/kravatte.html
[Xoofff]: https://keccak.team/xoofff.html
