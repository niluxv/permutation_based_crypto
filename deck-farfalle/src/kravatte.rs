//! Kravatte: a keccak-p instantiation of Farfalle.

use super::{Farfalle, FarfalleConfig, RollFunction};
use crypto_permutation::PermutationState;
use permutation_keccak::{KeccakP1600, KeccakState1600};

#[derive(Copy, Clone, Default, Debug)]
pub struct RollC;

impl RollFunction for RollC {
    type State = KeccakState1600;

    fn apply(self, state: &mut Self::State) {
        // The y = 4 plane is given by `5 * 4 + x` indexing into the state
        let y4_plane = &mut state.get_state_mut()[20..];
        let x0 = y4_plane[0];
        let x1 = y4_plane[1];
        let x5 = x0.rotate_left(7) ^ x1 ^ (x1 >> 3);
        for i in 0..4 {
            y4_plane[i] = y4_plane[i + 1];
        }
        y4_plane[4] = x5;
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct RollE;

impl RollFunction for RollE {
    type State = KeccakState1600;

    fn apply(self, state: &mut Self::State) {
        // The y plane is given by `5 * y + x` indexing into the state
        let y34_plane = &mut state.get_state_mut()[15..];
        let x0 = y34_plane[0];
        let x1 = y34_plane[1];
        let x2 = y34_plane[2];
        let x10 = x0.rotate_left(7) ^ x1.rotate_left(18) ^ (x2 & (x1 >> 1));
        for i in 0..9 {
            y34_plane[i] = y34_plane[i + 1];
        }
        y34_plane[9] = x10;
    }
}

/// Kravatte (Achouffe) configuration for Farfalle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct KravatteConfig;

impl FarfalleConfig for KravatteConfig {
    type PermutationB = KeccakP1600<6>;
    type PermutationC = KeccakP1600<6>;
    type PermutationD = KeccakP1600<6>;
    type PermutationE = KeccakP1600<6>;
    type RollC = RollC;
    type RollE = RollE;
    type State = KeccakState1600;

    fn perm_b(&self) -> Self::PermutationB {
        Default::default()
    }

    fn perm_c(&self) -> Self::PermutationC {
        Default::default()
    }

    fn perm_d(&self) -> Self::PermutationD {
        Default::default()
    }

    fn perm_e(&self) -> Self::PermutationE {
        Default::default()
    }

    fn roll_c(&self) -> Self::RollC {
        Default::default()
    }

    fn roll_e(&self) -> Self::RollE {
        Default::default()
    }
}

/// The Kravatte (Achouffe) deck function.
pub type Kravatte = Farfalle<KravatteConfig>;

#[cfg(test)]
mod tests {
    use super::Kravatte;
    use crypto_permutation::{BufMut, DeckFunction, Reader, Writer};

    // Test cases generated using python `kravatte` package

    /// Test with single input and 32 bytes of output. Expected output computed
    /// using the python `kravatte` package.
    #[test]
    fn single_input() {
        let key = b"kravatte test key";
        let msg = b"hello world";
        let expected = [
            0x4, 0x54, 0x69, 0x85, 0xc4, 0xc7, 0x41, 0x5e, 0xe3, 0x56, 0x76, 0x24, 0xbf, 0x5, 0xa1,
            0x53, 0x35, 0x1a, 0x57, 0x1b, 0xe2, 0x9e, 0x23, 0x26, 0xd3, 0xa0, 0x85, 0x75, 0x1,
            0x42, 0xba, 0xb0,
        ];
        let mut kravatte = Kravatte::init_default(key.as_ref());
        {
            let mut writer = kravatte.input_writer();
            writer
                .write_bytes(msg.as_ref())
                .expect("writing message failed");
            writer.finish();
        }
        let output = {
            let mut output = [0_u8; 32];
            let buf: BufMut<'_> = output.as_mut().into();
            let mut reader = kravatte.output_reader();
            reader.write_to_buf(buf).expect("writing output failed");
            output
        };
        assert_eq!(expected, output);
    }

    /// Single input, but split over two slices. Should do the same as
    /// `single_input`.
    #[test]
    fn split_input() {
        let key = b"kravatte test key";
        let expected = [
            0x4, 0x54, 0x69, 0x85, 0xc4, 0xc7, 0x41, 0x5e, 0xe3, 0x56, 0x76, 0x24, 0xbf, 0x5, 0xa1,
            0x53, 0x35, 0x1a, 0x57, 0x1b, 0xe2, 0x9e, 0x23, 0x26, 0xd3, 0xa0, 0x85, 0x75, 0x1,
            0x42, 0xba, 0xb0,
        ];
        let mut kravatte = Kravatte::init_default(key.as_ref());
        {
            let mut writer = kravatte.input_writer();
            writer
                .write_bytes(b"hello ")
                .expect("writing message failed");
            writer
                .write_bytes(b"world")
                .expect("writing message failed");
            writer.finish();
        }
        let output = {
            let mut output = [0_u8; 32];
            let buf: BufMut<'_> = output.as_mut().into();
            let mut reader = kravatte.output_reader();
            reader.write_to_buf(buf).expect("writing output failed");
            output
        };
        assert_eq!(expected, output);
    }

    /// Test with two separate inputs and 32 bytes of output. Expected output
    /// computed using the python `kravatte` package.
    #[test]
    fn multi_input() {
        let key = b"kravatte test key";
        let msg1 = b"hello";
        let msg2 = b"world";
        let expected = [
            0x36, 0x3e, 0x3, 0x73, 0xff, 0x47, 0x22, 0x1b, 0x63, 0x47, 0xe6, 0x87, 0x9b, 0x9a,
            0x5d, 0x24, 0x2e, 0xcd, 0x6c, 0xde, 0xcb, 0xa, 0x43, 0x12, 0x45, 0xa2, 0xe3, 0x56,
            0x5f, 0x1a, 0xf7, 0xb9,
        ];
        let mut kravatte = Kravatte::init_default(key.as_ref());
        {
            let mut writer = kravatte.input_writer();
            writer
                .write_bytes(msg1.as_ref())
                .expect("writing message failed");
            writer.finish();
        }
        {
            let mut writer = kravatte.input_writer();
            writer
                .write_bytes(msg2.as_ref())
                .expect("writing message failed");
            writer.finish();
        }
        let output = {
            let mut output = [0_u8; 32];
            let buf: BufMut<'_> = output.as_mut().into();
            let mut reader = kravatte.output_reader();
            reader.write_to_buf(buf).expect("writing output failed");
            output
        };
        assert_eq!(expected, output);
    }

    /// Test with two separate inputs and 32 bytes of output. Expected output
    /// computed using the python `kravatte` package.
    #[test]
    fn multi_output() {
        let key = b"kravatte test key";
        let msg = b"hello world";
        let expected = [
            0x4, 0x54, 0x69, 0x85, 0xc4, 0xc7, 0x41, 0x5e, 0xe3, 0x56, 0x76, 0x24, 0xbf, 0x5, 0xa1,
            0x53, 0x35, 0x1a, 0x57, 0x1b, 0xe2, 0x9e, 0x23, 0x26, 0xd3, 0xa0, 0x85, 0x75, 0x1,
            0x42, 0xba, 0xb0, 0x2a, 0xe7, 0x5a, 0x93, 0x35, 0x91, 0x60, 0x95, 0x19, 0x0, 0xd, 0xea,
            0xc1, 0x45, 0x78, 0x13, 0x8d, 0x9a, 0xee, 0xd0, 0xf5, 0x5c, 0x56, 0x23, 0xe7, 0xb9,
            0x64, 0x45, 0x6e, 0x53, 0xf9, 0x9, 0xf, 0xe3, 0x85, 0xe8, 0x28, 0x90, 0x55, 0x21, 0x5b,
            0xf8, 0xfc, 0x9a, 0xe, 0x42, 0x71, 0xa8, 0x26, 0x5e, 0xe0, 0xd6, 0xde, 0xf1, 0x17,
            0xb1, 0x2d, 0xa4, 0x68, 0xb9, 0xba, 0x6, 0x83, 0xcb, 0x78, 0x69, 0xeb, 0x1c, 0xf4, 0xb,
            0x71, 0xd0, 0x81, 0xb9, 0x8f, 0xa1, 0x14, 0xe9, 0x27, 0xfd, 0xfa, 0x31, 0x9b, 0xa0,
            0x46, 0x90, 0x58, 0xac, 0xa8, 0xaa, 0x11, 0x34, 0xf4, 0x30, 0x4c, 0xe1,
        ];
        let mut kravatte = Kravatte::init_default(key.as_ref());
        {
            let mut writer = kravatte.input_writer();
            writer
                .write_bytes(msg.as_ref())
                .expect("writing message failed");
            writer.finish();
        }

        let mut reader = kravatte.output_reader();
        let output = {
            let mut output = [0_u8; 4 * 32];
            for chunk in output.as_mut().chunks_mut(32) {
                let buf: BufMut<'_> = chunk.into();
                reader.write_to_buf(buf).expect("writing output failed");
            }
            output
        };
        assert_eq!(expected, output);
    }

    /// Generic test to check that split inputs give identical internal states
    /// after `finish`ing the writer.
    #[test]
    fn split_input_equal_states() {
        let key = b"kravatte test key";
        let mut kra_full = Kravatte::init_default(key.as_ref());
        let mut kra_split = Kravatte::init_default(key.as_ref());
        {
            let mut writer = kra_full.input_writer();
            writer
                .write_bytes(b"hello world")
                .expect("writing message failed");
            writer.finish();
        }
        {
            let mut writer = kra_split.input_writer();
            writer
                .write_bytes(b"hello ")
                .expect("writing message failed");
            writer
                .write_bytes(b"world")
                .expect("writing message failed");
            writer.finish();
        }

        assert_eq!(kra_full, kra_split);
    }
}
