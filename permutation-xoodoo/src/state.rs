//! Xoodoo permutation state struct.

use crypto_permutation::PermutationState;

const LEN: usize = 12;
type StateRepresentation = [u32; LEN];

/// State for the Xoodoo permutation. 48 bytes, internally represented by 12
/// `u32`s in little endian encoding.
#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug, PartialEq))]
pub struct XoodooState {
    state: StateRepresentation,
}

/// Writer into the keccak permutation state.
///
/// Does nothing fancy except for little-endian to native-endian conversion.
type CopyWriter<'a> = crypto_permutation::io::le_uint_slice_writer::LeU32SliceWriter<'a>;
/// Writer that xors into the keccak permutation state.
///
/// Does nothing fancy except for little-endian to native-endian conversion.
type XorWriter<'a> = crypto_permutation::io::le_uint_slice_writer::LeU32SliceXorWriter<'a>;
/// Reader that reads from the keccak permutation state and outputs it's bytes
/// in little endian order.
type StateReader<'a> = crypto_permutation::io::le_uint_slice_reader::LeU32SliceReader<'a>;

impl Default for XoodooState {
    fn default() -> Self {
        Self { state: [0; LEN] }
    }
}

impl core::ops::BitXorAssign<&Self> for XoodooState {
    fn bitxor_assign(&mut self, rhs: &Self) {
        for (self_chunk, other_chunk) in self.get_state_mut().iter_mut().zip(rhs.get_state().iter())
        {
            *self_chunk ^= *other_chunk;
        }
    }
}

impl PermutationState for XoodooState {
    type CopyWriter<'a> = CopyWriter<'a>;
    type Representation = StateRepresentation;
    type StateReader<'a> = StateReader<'a>;
    type XorWriter<'a> = XorWriter<'a>;

    const SIZE: usize = 48;

    fn from_state(state: Self::Representation) -> Self {
        Self { state }
    }

    fn get_state(&self) -> &Self::Representation {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut Self::Representation {
        &mut self.state
    }

    fn reader<'a>(&'a self) -> Self::StateReader<'a> {
        StateReader::new(self.get_state())
    }

    fn copy_writer<'a>(&'a mut self) -> Self::CopyWriter<'a> {
        CopyWriter::new(self.get_state_mut())
    }

    fn xor_writer<'a>(&'a mut self) -> Self::XorWriter<'a> {
        XorWriter::new(self.get_state_mut())
    }
}
