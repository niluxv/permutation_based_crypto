//! Reader and writer traits to generalise over writing to and reading from
//! buffers in memory and cryptographic constructions which take variable length
//! input or generate variable length output.

mod util;
pub use util::check_write_size;

// `Reader` and `Writer` implementations:
#[cfg(feature = "io_le_uint_slice")]
pub mod le_uint_slice_reader;
#[cfg(feature = "io_le_uint_slice")]
pub mod le_uint_slice_writer;

use crate::buffer::BufMut;

/// Requested a write larger than `self.capacity()`.
#[derive(Debug, Clone)]
pub struct WriteTooLargeError {
    /// Length of write request in bytes.
    pub requested: usize,
    /// Capacity in bytes that is left.
    pub capacity: usize,
}

impl core::fmt::Display for WriteTooLargeError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            fmt,
            "Requested a write of size {} but writer has only {} bytes capacity left",
            self.requested, self.capacity
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WriteTooLargeError {}

/// An object to which bytes can be written.
///
/// Writes may be buffered, so it is required to call [`Self::finish`] to flush
/// pending writes. Using a [`Writer`] and dropping it afterwards instead of
/// calling [`Self::finish`] on it is a logic error.
pub trait Writer {
    /// Optional return type for the [`Self::finish`] method.
    type Return;

    /// Return the number of bytes that can still be written to `self`.
    ///
    /// When the writer has infinite capacity then `usize::MAX` is returned.
    fn capacity(&self) -> usize;

    /// Skip over `len` bytes. If skipping over bytes is not meaningful for the
    /// buffer then this is a no-op.
    ///
    /// # Errors
    /// Errors when `len > self.capacity()`.
    fn skip(&mut self, len: usize) -> Result<(), WriteTooLargeError>;

    /// Write `data.len()` bytes to the buffer.
    ///
    /// # Errors
    /// Errors when `data.len() > self.capacity()`.
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), WriteTooLargeError>;

    /// Flush any pending/buffered writes and optionally return something.
    ///
    /// If the buffer must initialise leftover bytes it will set them to zero.
    fn finish(self) -> Self::Return;
}

/// An object from which bytes can be read.
pub trait Reader {
    /// Return the number of bytes that can still be read from `self`.
    ///
    /// When the reader can generate arbitrary long output streams, `usize::MAX`
    /// is returned.
    fn capacity(&self) -> usize;

    /// Skip over `len` bytes.
    ///
    /// # Errors
    /// Errors when `len > self.capacity()`.
    fn skip(&mut self, len: usize) -> Result<(), WriteTooLargeError>;

    /// Write `n` bytes to `writer`.
    ///
    /// # Errors
    /// Errors when `n` exceeds reader or writer capacity.
    fn write_to<W: Writer>(&mut self, writer: &mut W, n: usize) -> Result<(), WriteTooLargeError>;

    /// Write `buf.len()` bytes of data into `buf`.
    ///
    /// # Errors
    /// Errors when `buf.len()` exceeds reader capacity.
    fn write_to_buf(&mut self, mut buf: BufMut<'_>) -> Result<(), WriteTooLargeError> {
        let len = buf.len();
        self.write_to(&mut buf, len)
    }

    /// Write `buf.len()` bytes of data into `buf`.
    ///
    /// # Errors
    /// Errors when `buf.len()` exceeds reader capacity.
    fn write_to_slice(&mut self, buf: &mut [u8]) -> Result<(), WriteTooLargeError> {
        self.write_to_buf(buf.into())
    }
}

/// Marker trait to indicate that the output of a [`Reader`] can be considered
/// to be pseudo random.
pub trait CryptoReader: Reader {}
