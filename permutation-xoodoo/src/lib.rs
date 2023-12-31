//! [Xoodoo] permutation in the [`crypto-permutation`] framework.
//!
//! `Xoodoo: Permutation`
//!
//! Uses the [`xoodoo-p` crate] internally for the actual permutation
//! invocation.
//!
//! [`crypto-permutation`]: https://crates.io/crates/crypto-permutation
//! [`xoodoo-p` crate]: https://crates.io/crates/xoodoo-p
//! [Xoodoo]: https://keccak.team/xoodoo.html

#![no_std]
#![allow(clippy::needless_lifetimes)]

use crypto_permutation::{Permutation, PermutationState};
use xoodoo_p::{xoodoo, MAX_ROUNDS};

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
