# crypto-permutation ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![crypto-permutation on crates.io](https://img.shields.io/crates/v/crypto-permutation)](https://crates.io/crates/crypto-permutation) [![crypto-permutation on docs.rs](https://docs.rs/crypto-permutation/badge.svg)](https://docs.rs/crypto-permutation) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/niluxv/permutation_based_crypto) ![Rust Version: ^1.65](https://img.shields.io/badge/rustc-%5E1.65-orange.svg)

Abstractions for permutation based cryptography in Rust.

This crate provides abstractions for generic permutation based cryptography. This allows other crates to build constructions generic over the concrete cryptographic permutation or a deck-function. The API can be considered to consist of three main parts:

 1. Cryptographic IO abstractions
 2. Cryptographic permutation abstraction
 3. Deck function abstraction

The cryptographic IO abstractions are foundational for this entire crate. The other abstractions build on top of it.


## IO

The cryptographic IO abstractions give generic ways to input data into cryptographic functions (like hash or dec/deck functions) or get output from cryptographic functions (like stream ciphers, extendable output functions or dec/deck functions). The same traits can also be used to abstract over (fixed or variable sized) buffers, which is for example useful for abstracting over low-level primitives like permutations.

The API consists of two core traits:

 - [`Writer`][__link0]: A buffer or construction data can be written to. This is used for example for inputting data into a deck function.
 - [`Reader`][__link1]: A buffer that can be read from or a construction that can generate an output stream. This is used for example for generating an output stream from a deck function.


## Permutations

Cryptographic permutations are abstracted over using two traits:

 - [`PermutationState`][__link2]: A fixed size buffer cryptographic permutations can act on. It can have specific data layout (e.g. byteorder) requirements, as long as it is possible to clone states, xor states together and xor and write bytes into (using the [`Writer`][__link3] trait) and read bytes from (using the [`Reader`][__link4] trait).
 - [`Permutation`][__link5]: A cryptographic permutation. It acts on a specific [`PermutationState`][__link6].


## Deck functions

A deck function is a Doubly Extendable Cryptographic Keyed function. It is abstracted over by the [`DeckFunction`][__link7] trait. It allows repeatedly inputting and outputting variable length streams of data. For inputting data, the [`Writer`][__link8] trait is used, and for outputting the [`Reader`][__link9] trait is used.



 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEG8lq_dIqgTVtG1jCMXLwLpFYGykuOgo4U562G4crLsejH8cFYXKEG4AG289CBZ3IG8Y4ZItrJ5mpG505p-J2w_EWG9RKcNczGtNHYWSBg3JjcnlwdG8tcGVybXV0YXRpb25lMC4xLjByY3J5cHRvX3Blcm11dGF0aW9u
 [__link0]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/?search=io::Writer
 [__link1]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/?search=io::Reader
 [__link2]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/trait.PermutationState.html
 [__link3]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/?search=io::Writer
 [__link4]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/?search=io::Reader
 [__link5]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/trait.Permutation.html
 [__link6]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/trait.PermutationState.html
 [__link7]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/trait.DeckFunction.html
 [__link8]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/?search=io::Writer
 [__link9]: https://docs.rs/crypto-permutation/0.1.0/crypto_permutation/?search=io::Reader
