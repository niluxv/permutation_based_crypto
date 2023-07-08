//! Keccak-f and [keccak-p] permutations in the [`crypto-permutation`]
//! framework.
//!
//! `Keccak-p: Permutation`
//!
//! Uses the RustCrypto [`keccak` crate] internally for the actual permutation
//! invocation.
//!
//! [`crypto-permutation`]: https://crates.io/crates/crypto-permutation
//! [`keccak` crate]: https://crates.io/crates/keccak
//! [keccak-p]: https://keccak.team/keccakp.html

#![no_std]

use crypto_permutation::{Permutation, PermutationState};
use keccak::{f1600, keccak_p};

mod state;
pub use state::KeccakState1600;

/// Keccak-f\[1600\] permutation (i.e. full 24 rounds Keccak-p).
#[derive(Clone, Copy, Debug, Default)]
pub struct KeccakF1600;

impl Permutation for KeccakF1600 {
    type State = KeccakState1600;

    fn apply(self, state: &mut Self::State) {
        f1600(state.get_state_mut());
    }
}

/// Keccak-\[1600, ROUNDS\] permutation (i.e. `ROUNDS` rounds Keccak-p).
/// `ROUNDS` can be at most 24.
#[derive(Clone, Copy, Debug, Default)]
pub struct KeccakP1600<const ROUNDS: usize>;

impl<const ROUNDS: usize> KeccakP1600<ROUNDS> {
    const _ROUNDS_CHECK: () = {
        assert!(ROUNDS > 0);
        assert!(ROUNDS <= 24);
    };
}

impl<const ROUNDS: usize> Permutation for KeccakP1600<ROUNDS> {
    type State = KeccakState1600;

    fn apply(self, state: &mut Self::State) {
        keccak_p(state.get_state_mut(), ROUNDS);
    }
}
