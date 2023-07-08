//! Abstractions for permutation based cryptography in Rust.
//!
//! This crate provides abstractions for generic permutation based cryptography.
//! This allows other crates to build constructions generic over the concrete
//! cryptographic permutation or a deck-function. The API can be considered to
//! consist of three main parts:
//!
//! 1. Cryptographic IO abstractions
//! 2. Cryptographic permutation abstraction
//! 3. Deck function abstraction
//!
//! The cryptographic IO abstractions are foundational for this entire crate.
//! The other abstractions build on top of it.
//!
//! # IO
//! The cryptographic IO abstractions give generic ways to input data into
//! cryptographic functions (like hash or dec/deck functions) or get output from
//! cryptographic functions (like stream ciphers, extendable output functions or
//! dec/deck functions). The same traits can also be used to abstract over
//! (fixed or variable sized) buffers, which is for example useful for
//! abstracting over low-level primitives like permutations.
//!
//! The API consists of two core traits:
//! * [`Writer`]: A buffer or construction data can be written to. This is used
//!   for example for inputting data into a deck function.
//! * [`Reader`]: A buffer that can be read from or a construction that can
//!   generate an output stream. This is used for example for generating an
//!   output stream from a deck function.
//!
//! # Permutations
//! Cryptographic permutations are abstracted over using two traits:
//! * [`PermutationState`]: A fixed size buffer cryptographic permutations can
//!   act on. It can have specific data layout (e.g. byteorder) requirements, as
//!   long as it is possible to clone states, xor states together and xor and
//!   write bytes into (using the [`Writer`] trait) and read bytes from (using
//!   the [`Reader`] trait).
//! * [`Permutation`]: A cryptographic permutation. It acts on a specific
//!   [`PermutationState`].
//!
//! # Deck functions
//! A deck function is a Doubly Extendable Cryptographic Keyed function. It is
//! abstracted over by the [`DeckFunction`] trait. It allows repeatedly
//! inputting and outputting variable length streams of data. For inputting
//! data, the [`Writer`] trait is used, and for outputting the [`Reader`] trait
//! is used.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod buffer;
pub use buffer::BufMut;

pub mod io;
pub use io::{CryptoReader, Reader, WriteTooLargeError, Writer};

/// A state where a cryptographic permutation acts upon.
///
/// The API of this trait consists of two parts: the generic and the specific
/// part. The generic part aims at users of permutation, that want to be generic
/// over the specific permutation used in the construction (e.g. in a Sponge or
/// Farfalle construction). The specific part aims at [`Permutation`]
/// implementers. It allows to directly access the state representation.
///
/// # Generic API
/// The generic API gives abstract ways to read, write and xor bytes from/to the
/// state. It consists of
/// * [`Self::SIZE`] (constant)
/// * [`Self::StateReader`] (type)
/// * [`Self::CopyWriter`] (type)
/// * [`Self::XorWriter`] (type)
/// * [`Self::reader`] (method)
/// * [`Self::copy_writer`] (method)
/// * [`Self::xor_writer`] (method)
///
/// Besides these trait items, there are also the [`Default`], [`Clone`] and
/// [`BitXorAssign`] trait bounds.
///
/// # Specific API
/// The specific API gives direct read and write access to the state
/// representation underlying the [`PermutationState`]. It consists of
/// * [`Self::Representation`] (type)
/// * [`Self::from_state`] (constructor function)
/// * [`Self::get_state`] (method)
/// * [`Self::get_state_mut`] (method)
///
/// [`BitXorAssign`]: core::ops::BitXorAssign
pub trait PermutationState: Default + Clone + for<'a> core::ops::BitXorAssign<&'a Self> {
    // # Generic API

    /// Number of bytes of the state.
    const SIZE: usize;
    /// [`Reader`] to read bytes from the state.
    type StateReader<'a>: Reader
    where
        Self: 'a;
    /// [`Writer`] to write into the state.
    type CopyWriter<'a>: Writer
    where
        Self: 'a;
    /// [`Writer`] to xor into the state.
    type XorWriter<'a>: Writer
    where
        Self: 'a;

    /// Create a [`Reader`] to read bytes from the state.
    fn reader<'a>(&'a self) -> Self::StateReader<'a>;
    /// Create a [`Writer`] to write into the state.
    fn copy_writer<'a>(&'a mut self) -> Self::CopyWriter<'a>;
    /// Create a [`Writer`] to xor into the state.
    fn xor_writer<'a>(&'a mut self) -> Self::XorWriter<'a>;

    // # Specific API

    /// Representation of the state the permutation works on.
    ///
    /// This should generally be an array of integers, e.g. `[u64; 25]` for
    /// Keccak-f\[1600\].
    type Representation;

    /// Initialise the permutation from the given state.
    fn from_state(state: Self::Representation) -> Self;
    /// Read from the state buffer.
    fn get_state(&self) -> &Self::Representation;
    /// Write into the state buffer.
    fn get_state_mut(&mut self) -> &mut Self::Representation;
}

/// A cryptographic permutation.
pub trait Permutation: Copy + Default {
    /// The state this permutation acts upon.
    ///
    /// Splitting this type out allows for different permutations to act on the
    /// same state. For example, keccak-p[1600, n] can act on the same state
    /// regardless the number of rounds `n`.
    type State: PermutationState;

    /// Apply the permutation to the state.
    fn apply(self, state: &mut Self::State);
}

/// A doubly-ended cryptographic keyed function.
///
/// A deck function is a Doubly Extendable Cryptographic Keyed function. It
/// allows repeatedly inputting and outputting variable length streams of data.
/// To input a stream of data, create an input writer using
/// [`Self::input_writer`]. Data can then be written to it using the methods of
/// the [`Writer`] trait. Writes to separate input writers are domain separated;
/// writes to a single input writer are concatenated. To generate an output
/// stream, create an output generator using [`Self::output_reader`]. An output
/// stream can be generated from it using the methods of the [`Reader`] trait.
/// Creating an output reader and reading from it does not mutate the state of
/// the original deck function. Inputting new data into the deck function does
/// not change an already existing output generator.
///
/// # Crypto
/// When a secret uniformly chosen key is used to initialise the deck function
/// (using [`Self::init`]), the output the output generator generates should be
/// secure as a pseudo-random function from on the input data, i.e.
/// indistinguishable from a truly random function. Identical `(key, input)`
/// pairs give identical output (determinism).
///
/// # Warning
/// This is a relatively low-level API and very flexible so you can easily
/// create many modes on top of it, but it is not misuse resistant. In
/// particular calling the `output` method doesn't modify the state, and calling
/// it twice on the same state gives identical output streams.
pub trait DeckFunction {
    type OutputGenerator: CryptoReader;
    /// [`Writer`] that inputs data that is written to it to the deck function.
    type InputWriter<'a>: Writer
    where
        Self: 'a;

    /// Create a deck function from a 256 bit secret key.
    fn init(key: &[u8; 32]) -> Self;
    /// Create a writer that inputs data into the deck function.
    ///
    /// Every input writer starts writing a new stream, i.e. domain separation
    /// between input streams is applied. Separate writes to the same
    /// input writer are not domain separated, but just concatenated.
    /// In code:
    /// ```compile_fail
    /// # use crypto_permutation::{DeckFunction, Writer};
    /// # fn deck() -> impl DeckFunction + Clone + PartialEq + core::fmt::Debug {
    /// #     unimplemented!()
    /// # }
    /// # let mut deck1 = deck();
    /// // assume `deck1` is a `DeckFunction`
    /// let mut deck2 = deck1.clone();
    /// let mut deck3 = deck1.clone();
    ///
    /// let mut writer1 = deck1.input_writer();
    /// writer1.write_bytes(b"hello world");
    /// writer1.finish();
    ///
    /// let mut writer2 = deck2.input_writer();
    /// writer2.write_bytes(b"hello");
    /// writer2.write_bytes(b" world");
    /// writer2.finish();
    ///
    /// let mut writer3 = deck3.input_writer();
    /// writer3.write_bytes(b"hello");
    /// writer3.finish();
    /// let mut writer3 = deck3.input_writer();
    /// writer3.write_bytes(b" world");
    /// writer3.finish();
    ///
    /// assert_eq!(deck1, deck2);
    /// assert_ne!(deck2, deck3);
    /// ```
    fn input_writer<'a>(&'a mut self) -> Self::InputWriter<'a>;

    /// Create an output generator from the current state.
    ///
    /// # Warning
    /// Never create an output generator from the same state twice, without
    /// inputting new data in between. These would generate identical output
    /// streams.
    fn output_reader(&self) -> Self::OutputGenerator;
}
