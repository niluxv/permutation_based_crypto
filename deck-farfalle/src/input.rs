//! Compression layer of the Farfalle construction.

use super::{FarfalleConfig, RollFunction};
use crypto_permutation::{Permutation, PermutationState, WriteTooLargeError, Writer};

/// Generic Farfalle construction.
///
/// The intended way to interact with it is through the
/// [`crypto_permutation::DeckFunction`] trait. The [`Self::init_default`]
/// method provides a way to create an instance using a custom length key (but
/// it has to fit in a permutation block minus one byte).
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug, PartialEq))]
pub struct Farfalle<C: FarfalleConfig> {
    pub(super) key: C::State,
    pub(super) state: C::State,
    pub(super) config: C,
}

const PAD_BYTE: u8 = 1;

impl<C: FarfalleConfig> Farfalle<C> {
    fn key_expand(key: &[u8], p_b: C::PermutationB) -> C::State {
        assert!(key.len() < C::State::SIZE);
        let mut key_state = C::State::default();
        let mut state_writer = key_state.copy_writer();
        state_writer.write_bytes(key).unwrap();
        state_writer.write_bytes(&[PAD_BYTE]).unwrap();
        state_writer.finish();
        p_b.apply(&mut key_state);
        key_state
    }

    /// Create an instance using a key of custom length and non-default
    /// [`FarfalleConfig`] `config`. The key plus padding (1 byte) must fit in a
    /// single permutation block.
    ///
    /// # Panics
    /// Panics when the key plus padding (1 byte) don't fit a single permutation
    /// block.
    pub fn init_custom(key: &[u8], config: C) -> Self {
        Self {
            key: Self::key_expand(key, config.perm_b()),
            state: Default::default(),
            config,
        }
    }

    /// Create an instance using a key of custom length. The key plus padding (1
    /// byte) must fit in a single permutation block.
    ///
    /// # Panics
    /// Panics when the key plus padding (1 byte) don't fit a single permutation
    /// block.
    pub fn init_default(key: &[u8]) -> Self
    where
        C: Default,
    {
        Self::init_custom(key, C::default())
    }

    /// Apply rolling function C to the key.
    fn roll_c_key(&mut self) {
        self.config.roll_c().apply(&mut self.key);
    }

    /// Process one block of data, given as a permutation state.
    ///
    /// Note: this modifies `block`. The user should wipe or reuse it.
    fn process_block(&mut self, block: &mut C::State) {
        *block ^= &self.key;
        self.roll_c_key();
        self.config.perm_c().apply(block);
        self.state ^= block;
    }
}

/// A [`Writer`] structure that inputs all data that is written to it into the
/// Farfalle construction.
pub struct InputWriter<'a, C: FarfalleConfig> {
    /// A permutation state to accumulate data into before processing.
    block: C::State,
    /// Number of bytes of `block` that are initialised.
    filled: usize,
    /// The Farfalle construction to write data to.
    farfalle: &'a mut Farfalle<C>,
}

impl<'a, C: FarfalleConfig> InputWriter<'a, C> {
    /// Create a new writer inputting data into `farfalle`.
    pub(super) fn new(farfalle: &'a mut Farfalle<C>) -> Self {
        Self {
            block: Default::default(),
            filled: 0,
            farfalle,
        }
    }

    fn process_block(&mut self) {
        self.farfalle.process_block(&mut self.block);
        self.filled = 0;
    }
}

impl<'a, C: FarfalleConfig> Writer for InputWriter<'a, C> {
    type Return = ();

    fn capacity(&self) -> usize {
        usize::MAX
    }

    /// No-op.
    fn skip(&mut self, _n: usize) -> Result<(), WriteTooLargeError> {
        Ok(())
    }

    fn write_bytes(&mut self, mut data: &[u8]) -> Result<(), WriteTooLargeError> {
        if self.filled != 0 {
            let add_partial = core::cmp::min(data.len(), C::State::SIZE - self.filled);
            let old_filled = self.filled;
            self.filled += add_partial;
            let mut block_writer = self.block.copy_writer();
            block_writer.skip(old_filled).unwrap();
            block_writer.write_bytes(&data[..add_partial]).unwrap();
            block_writer.finish();
            data = &data[add_partial..];
            if self.filled == C::State::SIZE {
                self.process_block();
            }
        }

        let mut chunks = data.chunks_exact(C::State::SIZE);
        for chunk in &mut chunks {
            let mut block_writer = self.block.copy_writer();
            block_writer.write_bytes(&chunk).unwrap();
            block_writer.finish();
            self.process_block();
        }

        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            self.filled = remainder.len();
            let mut block_writer = self.block.copy_writer();
            block_writer.write_bytes(&remainder).unwrap();
            block_writer.finish();
        }

        Ok(())
    }

    /// Applies padding to the final block and processes it.
    fn finish(mut self) {
        self.write_bytes(&[PAD_BYTE]).unwrap();
        self.process_block();
        self.farfalle.roll_c_key();
    }
}
