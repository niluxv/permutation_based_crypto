//! Generic Farfalle construction which is generic over the cryptographic
//! permutation and Kravatte and Xoofff instantiations.
//!
//! `Farfalle: Permutation -> DeckFunction`
//!
//! This crate contains an implementation of the [Farfalle construction],
//! [`Farfalle`]. It is generic over the permutations and rolling functions
//! used, through the [`FarfalleConfig`] trait. The [`Farfalle`] struct is
//! intended to be used through the [`crypto_permutation::DeckFunction`] trait
//! that it implements.
//!
//! __Note__: No security audits of this crate have ever been performed. Use at
//! your own risk!
//!
//! The `kravatte` and `xoofff` crate-features enable the Kravatte and Xoofff
//! instantiations of Farfalle, in the [`kravatte`] and [`xoofff`] modules
//! respectively. These also contain the rolling functions that are used by
//! these instantiations, so it is easy create your own custom instantiation of
//! Farfalle that differs from Kravatte or Xoofff in the round count for the
//! permutation (in case you think the advised parameters are not conservative
//! enough).
//!
//! # Features
//! * `kravatte`: Enables the [`kravatte`] module.
//! * `xoofff`: Enables the [`xoofff`] module.
//! * `debug`: Used for tests. Don't use!
//!
//! # Testing
//! The Kravatte instantiation has been tested against the [`kravatte` python
//! package]. The Xoofff instantiation has been tested against the [`xoofff`
//! crate].
//!
//! [Farfalle construction]: https://keccak.team/farfalle.html
//! [`kravatte` python package]: https://pypi.org/project/kravatte
//! [`xoofff` crate]: https://crates.io/crates/xoofff

#![cfg_attr(not(test), no_std)]
#![allow(clippy::needless_lifetimes)]

use crypto_permutation::{DeckFunction, Permutation, PermutationState};

mod input;
mod output;
pub use input::{Farfalle, InputWriter};
pub use output::FarfalleOutputGenerator;

/// A rolling function as used in the Farfalle construction.
pub trait RollFunction: Copy + Default {
    /// The state this rolling function acts upon.
    type State: PermutationState;

    /// Apply the rolling function to the state.
    fn apply(self, state: &mut Self::State);
}

/// Parameters for the Farfalle construction.
///
/// The permutation state is expected to be at least 33 bytes long, i.e. 262
/// bits.
pub trait FarfalleConfig: Default + Clone {
    type State: PermutationState;
    type PermutationB: Permutation<State = Self::State>;
    type PermutationC: Permutation<State = Self::State>;
    type PermutationD: Permutation<State = Self::State>;
    type PermutationE: Permutation<State = Self::State>;
    type RollC: RollFunction<State = Self::State>;
    type RollE: RollFunction<State = Self::State>;

    fn perm_b(&self) -> Self::PermutationB;
    fn perm_c(&self) -> Self::PermutationC;
    fn perm_d(&self) -> Self::PermutationD;
    fn perm_e(&self) -> Self::PermutationE;
    fn roll_c(&self) -> Self::RollC;
    fn roll_e(&self) -> Self::RollE;
}

impl<C: FarfalleConfig> DeckFunction for Farfalle<C> {
    type InputWriter<'a> = InputWriter<'a, C> where Self: 'a;
    type OutputGenerator = FarfalleOutputGenerator<C>;

    fn init(key: &[u8; 32]) -> Self {
        Self::init_default(key.as_ref())
    }

    /// Produce a writer to input an additional input string.
    fn input_writer<'a>(&'a mut self) -> Self::InputWriter<'a> {
        InputWriter::new(self)
    }

    fn output_reader(&self) -> Self::OutputGenerator {
        let mut state = self.state.clone();
        self.config.perm_d().apply(&mut state);
        FarfalleOutputGenerator::new(self.key.clone(), state, self.config.clone())
    }
}

#[cfg(feature = "kravatte")]
pub mod kravatte;
#[cfg(feature = "xoofff")]
pub mod xoofff;
