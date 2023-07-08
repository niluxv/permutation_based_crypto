//! Xoofff: a xoodoo instantiation of Farfalle.

use super::{Farfalle, FarfalleConfig, RollFunction};
use crypto_permutation::PermutationState;
use permutation_xoodoo::{XoodooP, XoodooState};

#[derive(Copy, Clone, Default, Debug)]
pub struct RollC;

impl RollFunction for RollC {
    type State = XoodooState;

    fn apply(self, state: &mut Self::State) {
        // The y plane is given by `4 * y + x` indexing into the state
        let a = &mut state.get_state_mut();
        a[4 * 0 + 0] ^= (a[4 * 0 + 0] << 13) ^ a[4 * 1 + 0].rotate_left(3);
        let b: [u32; 4] = {
            let mut b = [0; 4];
            b[3] = a[0];
            b[0] = a[1];
            b[1] = a[2];
            b[2] = a[3];
            b
        };
        for i in 0..8 {
            a[i] = a[i + 4];
        }
        a[8..].copy_from_slice(&b[..]);
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct RollE;

impl RollFunction for RollE {
    type State = XoodooState;

    fn apply(self, state: &mut Self::State) {
        // The y plane is given by `4 * y + x` indexing into the state
        let a = &mut state.get_state_mut();
        a[4 * 0 + 0] = (a[4 * 1 + 0] & a[4 * 2 + 0])
            ^ (a[4 * 0 + 0].rotate_left(5))
            ^ (a[4 * 1 + 0].rotate_left(13))
            ^ 0x00000007;
        let b: [u32; 4] = {
            let mut b = [0; 4];
            b[3] = a[0];
            b[0] = a[1];
            b[1] = a[2];
            b[2] = a[3];
            b
        };
        for i in 0..8 {
            a[i] = a[i + 4];
        }
        a[8..].copy_from_slice(&b[..]);
    }
}

/// Xoofff configuration for Farfalle.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct XoofffConfig;

impl FarfalleConfig for XoofffConfig {
    type PermutationB = XoodooP<6>;
    type PermutationC = XoodooP<6>;
    type PermutationD = XoodooP<6>;
    type PermutationE = XoodooP<6>;
    type RollC = RollC;
    type RollE = RollE;
    type State = XoodooState;

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

/// The Xoofff deck function.
pub type Xoofff = Farfalle<XoofffConfig>;

#[cfg(test)]
mod tests {
    use super::Xoofff;
    use crypto_permutation::{BufMut, DeckFunction, Reader, Writer};

    struct XoofffTester {
        farfalle: Xoofff,
        farfalle_output_reader: Option<<Xoofff as DeckFunction>::OutputGenerator>,
        xoofff_crate: xoofff::Xoofff,
        state: XoofffState,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum XoofffState {
        Absorb,
        Squeeze,
    }

    impl XoofffTester {
        fn new(key: &[u8]) -> Self {
            Self {
                farfalle: Xoofff::init_default(key),
                farfalle_output_reader: None,
                xoofff_crate: xoofff::Xoofff::new(key),
                state: XoofffState::Absorb,
            }
        }

        fn input_str(&mut self, slices: &[&[u8]]) {
            assert!(self.state == XoofffState::Absorb);
            let mut writer = self.farfalle.input_writer();
            for data in slices.iter() {
                writer.write_bytes(data);
                self.xoofff_crate.absorb(data);
            }
            writer.finish();
            self.state = XoofffState::Squeeze;
            self.farfalle_output_reader = Some(self.farfalle.output_reader());
            self.xoofff_crate.finalize(0, 0, 0);
        }

        fn squeeze_compare(&mut self, n: usize) {
            assert!(self.state == XoofffState::Squeeze);
            let mut buf1 = vec![0; n];
            let mut buf2 = vec![0; n];
            {
                let mut reader = self.farfalle_output_reader.as_mut().unwrap();
                reader.write_to_buf((&mut buf1[..]).into());
                self.xoofff_crate.squeeze(buf2.as_mut());
            }
            assert_eq!(buf1, buf2);
        }

        fn finish_squeeze(&mut self) {
            assert!(self.state == XoofffState::Squeeze);
            self.state = XoofffState::Absorb;
            self.farfalle_output_reader = None;
            self.xoofff_crate.restart();
        }
    }

    /// Test with single input and 32 bytes of output.
    #[test]
    fn single_input() {
        let key = b"xoofff test key";
        let msg = b"hello world";
        let mut tester = XoofffTester::new(key);
        tester.input_str(&[msg]);
        tester.squeeze_compare(32);
    }

    /// Single input, but split over two slices. Should do the same as
    /// `single_input`.
    #[test]
    fn split_input() {
        let key = b"xoofff test key";
        let mut tester = XoofffTester::new(key);
        tester.input_str(&[b"hello ", b"world"]);
        tester.squeeze_compare(32);
    }

    /// Test with two separate inputs and 32 bytes of output.
    #[test]
    fn multi_input() {
        let key = b"xoofff test key";
        let mut tester = XoofffTester::new(key);
        tester.input_str(&[b"hello"]);
        tester.finish_squeeze();
        tester.input_str(&[b"world"]);
        tester.squeeze_compare(32);
    }

    /// Test with five 32 bytes of output.
    #[test]
    fn multi_output() {
        let key = b"xoofff test key";
        let mut tester = XoofffTester::new(key);
        tester.input_str(&[b"hello world"]);
        for i in 0..4 {
            tester.squeeze_compare(32);
        }
    }

    /// Generic test to check that split inputs give identical internal states
    /// after `finish`ing the writer.
    #[test]
    fn split_input_equal_states() {
        let key = b"xoofff test key";
        let mut xoofff_full = Xoofff::init_default(key.as_ref());
        let mut xoofff_split = Xoofff::init_default(key.as_ref());
        {
            let mut writer = xoofff_full.input_writer();
            writer
                .write_bytes(b"hello world")
                .expect("writing message failed");
            writer.finish();
        }
        {
            let mut writer = xoofff_split.input_writer();
            writer
                .write_bytes(b"hello ")
                .expect("writing message failed");
            writer
                .write_bytes(b"world")
                .expect("writing message failed");
            writer.finish();
        }

        assert_eq!(xoofff_full, xoofff_split);
    }
}
