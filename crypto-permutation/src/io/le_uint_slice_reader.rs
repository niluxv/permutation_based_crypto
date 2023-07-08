//! Readers for arrays of little endian unsigned integers.

use super::util::{check_write_size, cold};
use super::{Reader, WriteTooLargeError, Writer};

// Requires separetely provided methods `write` and `reset_partial_block` for
// `$name`.
macro_rules! impl_le_uint_slice_reader {
    ($name:ident, $uint:ty) => {
        /// Reader that reads from a buffer `self.buffer` of `$uint`s , and outputs
        /// their bytes in little endian order.
        ///
        /// Does nothing fancy except for native-endian to little-endian conversion.
        pub struct $name<'a> {
            /// A slice of the part of the buffer that can still be read.
            buffer: &'a [$uint],
            /// Number of bytes of the first element of `buffer` that have already been
            /// read.
            partial_read: u8,
        }

        impl<'a> $name<'a> {
            /// Number of bytes that `$uint` is long.
            const UINT_SIZE: usize = core::mem::size_of::<$uint>();
            /// Constant for compile time assertion that `UINT_SIZE` fits a `u8`.
            const _CHECK: () = {
                let size = Self::UINT_SIZE;
                assert!(size as u8 as usize == size)
            };

            /// `self.partial_read as usize`
            fn partial_read_usize(&self) -> usize {
                usize::from(self.partial_read)
            }

            /// Step `n` `$uint`s forward in the buffer view.
            fn increment_view(&mut self, n: usize) {
                // We temporarily take ownership of `self.buffer` by swapping in an empty slice
                // instead. We can then mutate `buffer` without changing the lifetime and swap
                // it back in `self`.
                let mut buffer: &'a [$uint] = core::mem::take(&mut self.buffer);
                buffer = &buffer[n..];
                let _ = core::mem::replace(&mut self.buffer, buffer);
            }

            /// Create a new reader that reads bytes `buffer`, and outputs it's bytes
            /// little endian order.
            pub fn new(buffer: &'a [$uint]) -> Self {
                Self {
                    buffer,
                    partial_read: 0,
                }
            }
        }

        impl<'a> Reader for $name<'a> {
            fn capacity(&self) -> usize {
                self.buffer.len() * Self::UINT_SIZE - self.partial_read_usize()
            }

            fn skip(&mut self, mut n: usize) -> Result<(), WriteTooLargeError> {
                check_write_size(n, self.capacity())?;

                if self.partial_read != 0 {
                    cold();
                    let partial_read =
                        core::cmp::min(n, Self::UINT_SIZE - self.partial_read_usize());
                    self.partial_read += partial_read as u8;
                    n -= partial_read;
                    if self.partial_read == Self::UINT_SIZE as u8 {
                        self.increment_view(1);
                        self.partial_read = 0;
                    }
                }

                let remainder = n % Self::UINT_SIZE;
                n -= remainder;
                n /= Self::UINT_SIZE;
                self.increment_view(n);

                if remainder != 0 {
                    cold();
                    self.partial_read = remainder as u8;
                }

                Ok(())
            }

            fn write_to<W: Writer>(
                &mut self,
                writer: &mut W,
                mut n: usize,
            ) -> Result<(), WriteTooLargeError> {
                check_write_size(n, self.capacity())?;

                if self.partial_read != 0 {
                    cold();
                    let partial_read =
                        core::cmp::min(n, Self::UINT_SIZE - self.partial_read_usize());
                    {
                        let old_partial_read = self.partial_read_usize();
                        self.partial_read += partial_read as u8;
                        let bytes = self.buffer[0].to_le_bytes();
                        writer.write_bytes(&bytes[old_partial_read..self.partial_read_usize()])?;
                    }
                    n -= partial_read;
                    if self.partial_read == Self::UINT_SIZE as u8 {
                        self.increment_view(1);
                        self.partial_read = 0;
                    }
                }

                let remainder = n % Self::UINT_SIZE;
                n -= remainder;
                n /= Self::UINT_SIZE;
                for _ in 0..n {
                    let bytes = self.buffer[0].to_le_bytes();
                    writer.write_bytes(bytes.as_ref())?;
                    self.increment_view(1);
                }

                if remainder != 0 {
                    cold();
                    let bytes = self.buffer[0].to_le_bytes();
                    writer.write_bytes(&bytes[..remainder])?;
                    self.partial_read = remainder as u8;
                }

                Ok(())
            }
        }
    };
}

#[cfg(feature = "io_uint_u128")]
impl_le_uint_slice_reader!(LeU128SliceReader, u128);
#[cfg(feature = "io_uint_u64")]
impl_le_uint_slice_reader!(LeU64SliceReader, u64);
#[cfg(feature = "io_uint_u32")]
impl_le_uint_slice_reader!(LeU32SliceReader, u32);
#[cfg(feature = "io_uint_u16")]
impl_le_uint_slice_reader!(LeU16SliceReader, u16);
