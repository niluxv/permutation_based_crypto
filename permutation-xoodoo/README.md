# permutation-xoodoo ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![permutation-xoodoo on crates.io](https://img.shields.io/crates/v/permutation-xoodoo)](https://crates.io/crates/permutation-xoodoo) [![permutation-xoodoo on docs.rs](https://docs.rs/permutation-xoodoo/badge.svg)](https://docs.rs/permutation-xoodoo) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/niluxv/permutation_based_crypto) ![Rust Version: 1.65.0](https://img.shields.io/badge/rustc-1.65.0-orange.svg)

[Xoodoo][__link0] permutation in the [`crypto-permutation`][__link1] framework.

`Xoodoo: Permutation`

## Example

**Note**: The following example makes use of very low-level cryptographic APIs, which you
shouldn’t use unless you know very well what you are doing. The intended use of this crate
is just to pass the [`XoodooP`][__link2] permutation as a parameter to some generic more high-level
construction like Farfalle as implemented in [`deck-farfalle`][__link3].

Suppose we want to apply the full 12-round Xoodoo permutation to the message
`"hello world"` (and then padded with null-bytes to make it 42 bytes in length),
and then get the first 3 bytes of output.

```rust
use permutation_xoodoo::{XoodooState, XoodooP};
use crypto_permutation::{Reader, Writer, Permutation, PermutationState};

// create a state and a permutation to act on it
let mut state = XoodooState::default();
let xoodoo = XoodooP::<12>::default();

// write input to the state
state.copy_writer().write_bytes(b"hello world");

// apply the xoodoo permutation to the state
xoodoo.apply(&mut state);

// and finally you can read the first 3 bytes of output
let mut out = [0u8; 3];
state.reader().write_to_slice(&mut out);
assert_eq!(out, [241, 234, 156]);
```


 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEG_W_Gn_kaocAGwCcVPfenh7eGy6gYLEwyIe4G6-xw_FwcbpjYXKEG__MCjU_sZp4G-ejpvtmu_W3G8Vslfu_FSsfG6m3r0Em2yqIYWSBg3JwZXJtdXRhdGlvbi14b29kb29lMC4xLjFycGVybXV0YXRpb25feG9vZG9v
 [__link0]: https://keccak.team/xoodoo.html
 [__link1]: https://crates.io/crates/crypto-permutation
 [__link2]: https://docs.rs/permutation-xoodoo/0.1.1/permutation_xoodoo/struct.XoodooP.html
 [__link3]: https://crates.io/crates/deck-farfalle
