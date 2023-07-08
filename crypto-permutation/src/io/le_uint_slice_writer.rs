//! Writers for arrays of little endian unsigned integers.

use super::util::{check_write_size, cold};
use super::{WriteTooLargeError, Writer};

// Requires separetely provided methods `write` and `reset_partial_block` for
// `$name`.
macro_rules! impl_le_uint_slice_writer_core {
    ($name:ident, $uint:ty) => {
        /// Writer that writes/xors into the buffer `self.buffer`, interpreting bytes as
        /// little endian encoded `$uint`s.
        ///
        /// Does nothing fancy except for little-endian to native-endian conversion.
        pub struct $name<'a> {
            /// A slice of the part of the buffer that can still be written to.
            buffer: &'a mut [$uint],
            /// Small buffer to aggregate bytes until we have enough for a `$uint`.
            partial_block: [u8; core::mem::size_of::<$uint>()],
            /// Number of bytes currently cached in `partial_block`.
            partial_filled: u8,
        }

        impl<'a> $name<'a> {
            /// Number of bytes that `$uint` is long.
            const UINT_SIZE: usize = core::mem::size_of::<$uint>();
            /// Constant for compile time assertion that `UINT_SIZE` fits a `u8`.
            const _CHECK: () = {
                let size = Self::UINT_SIZE;
                assert!(size as u8 as usize == size)
            };

            /// `self.partial_filled as usize`
            fn partial_filled_usize(&self) -> usize {
                usize::from(self.partial_filled)
            }

            /// Step `n` `$uint`s forward in the buffer view.
            fn increment_view(&mut self, n: usize) {
                // We temporarily take ownership of `self.buffer` by swapping in an empty slice
                // instead. We can then mutate `buffer` without changing the lifetime and swap
                // it back in `self`.
                let mut buffer: &'a mut [$uint] = core::mem::replace(&mut self.buffer, &mut []);
                buffer = &mut buffer[n..];
                let _ = core::mem::replace(&mut self.buffer, buffer);
            }

            /// Write the partial block to the next `$uint` of the buffer.
            fn write_partial_block(&mut self) {
                let x = <$uint>::from_le_bytes(self.partial_block);
                self.write(x);
                self.increment_view(1);
                self.partial_filled = 0;
            }

            /// Create a new writer that writes/xors into `buffer`, interpreting bytes
            /// as little endian encoded `$uint`s.
            pub fn new(buffer: &'a mut [$uint]) -> Self {
                Self {
                    buffer,
                    partial_block: [0; core::mem::size_of::<$uint>()],
                    partial_filled: 0,
                }
            }
        }

        impl<'a> Writer for $name<'a> {
            type Return = ();

            fn capacity(&self) -> usize {
                self.buffer.len() * Self::UINT_SIZE - self.partial_filled_usize()
            }

            fn skip(&mut self, mut n: usize) -> Result<(), WriteTooLargeError> {
                check_write_size(n, self.capacity())?;

                if self.partial_filled != 0 {
                    cold();
                    let add_partial =
                        core::cmp::min(n, Self::UINT_SIZE - self.partial_filled_usize());
                    self.partial_filled += add_partial as u8;
                    n -= add_partial;
                    if self.partial_filled == Self::UINT_SIZE as u8 {
                        self.write_partial_block();
                    }
                }

                let remainder = n % Self::UINT_SIZE;
                n -= remainder;
                n /= Self::UINT_SIZE;
                self.increment_view(n);

                if remainder != 0 {
                    cold();
                    self.partial_filled = remainder as u8;
                    self.reset_partial_block();
                }

                Ok(())
            }

            fn write_bytes(&mut self, mut data: &[u8]) -> Result<(), WriteTooLargeError> {
                check_write_size(data.len(), self.capacity())?;

                if self.partial_filled != 0 {
                    cold();
                    let add_partial =
                        core::cmp::min(data.len(), Self::UINT_SIZE - self.partial_filled_usize());
                    let old_partial_filled = self.partial_filled_usize();
                    self.partial_filled += add_partial as u8;
                    let partial =
                        &mut self.partial_block[old_partial_filled..self.partial_filled.into()];
                    partial.copy_from_slice(&data[..add_partial]);
                    data = &data[add_partial..];
                    if self.partial_filled == Self::UINT_SIZE as u8 {
                        self.write_partial_block();
                    }
                }

                let mut chunks = data.chunks_exact(Self::UINT_SIZE);
                for chunk in &mut chunks {
                    let chunk: &[u8; core::mem::size_of::<$uint>()] = chunk.try_into().unwrap();
                    self.write(<$uint>::from_le_bytes(*chunk));
                    self.increment_view(1);
                }

                let remainder = chunks.remainder();
                if !remainder.is_empty() {
                    cold();
                    self.partial_filled = remainder.len() as u8;
                    self.reset_partial_block();
                    let n = remainder.len();
                    self.partial_block[..n].copy_from_slice(remainder);
                }

                Ok(())
            }

            fn finish(mut self) -> Self::Return {
                if self.partial_filled != 0 {
                    cold();
                    self.write_partial_block();
                }
                ()
            }
        }
    };
}

macro_rules! impl_le_uint_slice_writer {
    ($name:ident, $uint:ty) => {
        impl_le_uint_slice_writer_core!($name, $uint);

        impl<'a> $name<'a> {
            /// Write `val` to first element of the buffer.
            fn write(&mut self, val: $uint) {
                self.buffer[0] = val;
            }

            /// Reset the partial block to a new clean state before use.
            fn reset_partial_block(&mut self) {
                self.partial_block = self.buffer[0].to_le_bytes();
            }
        }
    };
}

#[cfg(feature = "io_uint_u128")]
impl_le_uint_slice_writer!(LeU128SliceWriter, u128);
#[cfg(feature = "io_uint_u64")]
impl_le_uint_slice_writer!(LeU64SliceWriter, u64);
#[cfg(feature = "io_uint_u32")]
impl_le_uint_slice_writer!(LeU32SliceWriter, u32);
#[cfg(feature = "io_uint_u16")]
impl_le_uint_slice_writer!(LeU16SliceWriter, u16);

macro_rules! impl_le_uint_slice_xor_writer {
    ($name:ident, $uint:ty) => {
        impl_le_uint_slice_writer_core!($name, $uint);

        impl<'a> $name<'a> {
            /// Write `val` to first element of the buffer.
            fn write(&mut self, val: $uint) {
                self.buffer[0] ^= val;
            }

            /// Reset the partial block to a new clean state before use.
            fn reset_partial_block(&mut self) {
                self.partial_block = [0; core::mem::size_of::<$uint>()];
            }
        }
    };
}

#[cfg(feature = "io_uint_u128")]
impl_le_uint_slice_xor_writer!(LeU128SliceXorWriter, u128);
#[cfg(feature = "io_uint_u64")]
impl_le_uint_slice_xor_writer!(LeU64SliceXorWriter, u64);
#[cfg(feature = "io_uint_u32")]
impl_le_uint_slice_xor_writer!(LeU32SliceXorWriter, u32);
#[cfg(feature = "io_uint_u16")]
impl_le_uint_slice_xor_writer!(LeU16SliceXorWriter, u16);
