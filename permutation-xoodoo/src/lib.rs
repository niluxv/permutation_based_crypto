//! [Xoodoo] permutation in the [`crypto-permutation`] framework.
//!
//! `Xoodoo: Permutation`
//!
//! # Example
//! __Note__: The following example makes use of very low-level cryptographic APIs, which you
//! shouldn't use unless you know very well what you are doing. The intended use of this crate
//! is just to pass the [`XoodooP`] permutation as a parameter to some generic more high-level
//! construction like Farfalle as implemented in [`deck-farfalle`].
//!
//! Suppose we want to apply the full 12-round Xoodoo permutation to the message
//! `"hello world"` (and then padded with null-bytes to make it 42 bytes in length),
//! and then get the first 3 bytes of output.
//! ```rust
//! use permutation_xoodoo::{XoodooState, XoodooP};
//! use crypto_permutation::{Reader, Writer, Permutation, PermutationState};
//!
//! // create a state and a permutation to act on it
//! let mut state = XoodooState::default();
//! let xoodoo = XoodooP::<12>::default();
//!
//! // write input to the state
//! state.copy_writer().write_bytes(b"hello world");
//!
//! // apply the xoodoo permutation to the state
//! xoodoo.apply(&mut state);
//!
//! // and finally you can read the first 3 bytes of output
//! let mut out = [0u8; 3];
//! state.reader().write_to_slice(&mut out);
//! assert_eq!(out, [241, 234, 156]);
//! ```
//!
//! [`crypto-permutation`]: https://crates.io/crates/crypto-permutation
//! [`deck-farfalle`]: https://crates.io/crates/deck-farfalle
//! [Xoodoo]: https://keccak.team/xoodoo.html

#![no_std]
#![allow(clippy::needless_lifetimes)]

use crypto_permutation::{Permutation, PermutationState};

mod permutation;
use permutation::{xoodoo, MAX_ROUNDS};
mod state;
pub use state::XoodooState;

/// Xoodoo permutation with `ROUNDS` rounds. `ROUNDS` must be at most 12.
#[derive(Clone, Copy, Debug, Default)]
pub struct XoodooP<const ROUNDS: usize>;

impl<const ROUNDS: usize> XoodooP<ROUNDS> {
    const _ROUNDS_CHECK: () = {
        assert!(ROUNDS > 0);
        assert!(ROUNDS <= MAX_ROUNDS);
    };
}

impl<const ROUNDS: usize> Permutation for XoodooP<ROUNDS> {
    type State = XoodooState;

    fn apply(self, state: &mut Self::State) {
        xoodoo::<ROUNDS>(state.get_state_mut());
    }
}

#[cfg(test)]
mod tests;
