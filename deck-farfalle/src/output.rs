//! Expansion layer of the Farfalle construction.

use crate::RollFunction;

use super::FarfalleConfig;
use crypto_permutation::io::{check_write_size, CryptoReader, Reader, WriteTooLargeError, Writer};
use crypto_permutation::{Permutation, PermutationState};

/// Expansion part in the Farfalle construction.
pub struct FarfalleOutputGenerator<C: FarfalleConfig> {
    /// Farfalle parameters.
    config: C,
    /// Immutable expansion key k' from the Farfalle construction.
    key: C::State,
    /// The accumulated state, to which permutation D and a number of roll E
    /// operations have already been applied.
    state: C::State,
    /// Buffer to store output bytes that haven't been output yet.
    output_buffer: C::State,
    /// Number of output bytes still available in `output_buffer`.
    buffered: usize,
}

impl<C: FarfalleConfig> FarfalleOutputGenerator<C> {
    /// Create a new [`FarfalleOutputGenerator`] from an expansion key `key`,
    /// state `state` (to which permutation D has already been applied) and
    /// Farfalle parameters `config`.
    pub(super) fn new(key: C::State, state: C::State, config: C) -> Self {
        Self {
            config,
            key,
            state,
            output_buffer: Default::default(),
            buffered: 0,
        }
    }

    /// Apply rolling function E to the state `self.state`.
    fn roll_e_state(&mut self) {
        self.config.roll_e().apply(&mut self.state);
    }

    /// Write the next output block to `self.output_buffer` and updates
    /// `self.state`. Does not modify `self.buffered`.
    fn next_out_block(&mut self) {
        self.output_buffer = self.state.clone();
        self.roll_e_state();
        self.config.perm_e().apply(&mut self.output_buffer);
        self.output_buffer ^= &self.key;
    }
}

impl<C: FarfalleConfig> Reader for FarfalleOutputGenerator<C> {
    fn capacity(&self) -> usize {
        usize::MAX
    }

    fn skip(&mut self, mut n: usize) -> Result<(), WriteTooLargeError> {
        if self.buffered != 0 {
            let out_size = core::cmp::min(self.buffered, n);
            n -= out_size;
            self.buffered -= out_size;
        }
        let remainder = n % C::State::SIZE;
        let n_blocks = (n - remainder) / C::State::SIZE;
        for _ in 0..n_blocks {
            self.next_out_block();
        }
        if remainder != 0 {
            self.next_out_block();
            self.buffered = C::State::SIZE - remainder;
        }
        Ok(())
    }

    fn write_to<W: Writer>(
        &mut self,
        writer: &mut W,
        mut n: usize,
    ) -> Result<(), WriteTooLargeError> {
        check_write_size(n, writer.capacity())?;
        if self.buffered != 0 {
            let out_size = core::cmp::min(self.buffered, n);
            let mut reader = self.output_buffer.reader();
            reader.skip(C::State::SIZE - self.buffered)?;
            reader.write_to(writer, out_size)?;
            n -= out_size;
            self.buffered -= out_size;
        }
        let remainder = n % C::State::SIZE;
        let n_blocks = (n - remainder) / C::State::SIZE;
        for _ in 0..n_blocks {
            self.next_out_block();
            let mut reader = self.output_buffer.reader();
            reader.write_to(writer, C::State::SIZE)?;
        }
        if remainder != 0 {
            self.next_out_block();
            let mut reader = self.output_buffer.reader();
            reader.write_to(writer, remainder)?;
            self.buffered = C::State::SIZE - remainder;
        }
        Ok(())
    }
}

impl<C: FarfalleConfig> CryptoReader for FarfalleOutputGenerator<C> {}
