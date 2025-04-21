//! Potentially uninitialised buffers that guarantee that they are not
//! deinitialised again after init.

use super::io::{check_write_size, WriteTooLargeError, Writer};
use core::mem::MaybeUninit;
use core::slice::SliceIndex;

/// Potentially uninitialised buffer that can never be deinitialised after it
/// has been initialised.
pub struct BufMut<'a> {
    buf: &'a mut [MaybeUninit<u8>],
}

impl<'a> From<&'a mut [MaybeUninit<u8>]> for BufMut<'a> {
    fn from(buf: &'a mut [MaybeUninit<u8>]) -> Self {
        Self { buf }
    }
}

// SAFETY: this conversion is safe since a [`BufMut`] cannot be used to
// deinitialise the underlying memory.
impl<'a> From<&'a mut [u8]> for BufMut<'a> {
    fn from(slice: &'a mut [u8]) -> Self {
        let ptr: *mut MaybeUninit<u8> = slice.as_mut_ptr().cast();
        let len: usize = slice.len();
        // SAFETY: `ptr` and `len` formed a slice to `u8`s, so definitely valid
        // `MaybeUninit<u8>`s. SAFETY: we just have to make sure that the
        // pointed-to bytes remain initialised but that is what the `Self`
        // wrapper struct is for.
        let buf = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
        Self { buf }
    }
}

impl<'a> BufMut<'a> {
    /// Length of the buffer.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Copy non-overlapping memory from `buf` to `self`.
    ///
    /// Requires that `self.len() >= buf.len()`. Doesn't change where the buffer
    /// `self` points to.
    ///
    /// # Errors
    /// Errors when `buf.len() > self.buf.len()`, without doing any copying.
    pub fn copy(&mut self, buf: &[u8]) -> Result<(), WriteTooLargeError> {
        // SAFETY: `self` has unique mutable access to the buffer referenced by
        // `self.buf`, so this cannot overlap with `buf`.
        let _: &mut [MaybeUninit<u8>] = self.buf;

        let len = buf.len();
        check_write_size(len, self.len())?;

        let src: *const u8 = buf.as_ptr();
        let dst: *mut u8 = self.buf.as_mut_ptr().cast();
        // SAFETY: `src` and `dst` don't overlap by the comment above; both slices have
        // length at least `len`
        unsafe {
            core::ptr::copy_nonoverlapping(src, dst, len);
        }

        Ok(())
    }

    pub fn reborrow<'b>(&'b mut self) -> BufMut<'b>
    where
        'a: 'b,
    {
        let buf = &mut self.buf;
        BufMut { buf }
    }

    pub fn restrict<'b, I>(&'b mut self, range: I) -> BufMut<'b>
    where
        'a: 'b,
        I: SliceIndex<[MaybeUninit<u8>], Output = [MaybeUninit<u8>]>,
    {
        let reborrowed: &'b mut [MaybeUninit<u8>] = self.buf;
        let buf = &mut reborrowed[range];
        BufMut { buf }
    }

    pub fn restrict_inplace<'b, I>(&'b mut self, range: I)
    where
        'a: 'b,
        I: SliceIndex<[MaybeUninit<u8>], Output = [MaybeUninit<u8>]>,
    {
        let mut buf = core::mem::take(&mut self.buf);
        buf = &mut buf[range];
        let _ = core::mem::replace(&mut self.buf, buf);
    }
}

impl<'a> Writer for BufMut<'a> {
    type Return = ();

    fn capacity(&self) -> usize {
        self.len()
    }

    fn skip(&mut self, n: usize) -> Result<(), WriteTooLargeError> {
        check_write_size(n, self.capacity())?;
        self.restrict(n..);
        Ok(())
    }

    fn write_bytes(&mut self, data: &[u8]) -> Result<(), WriteTooLargeError> {
        self.copy(data)?;
        self.restrict_inplace(data.len()..);
        Ok(())
    }

    /// No-op.
    fn finish(self) -> Self::Return {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writer_write() {
        let mut buf = [0; 3];
        let mut bufmut = BufMut::from(&mut buf[..]);
        bufmut.write_bytes(&[1, 2, 3]).unwrap();
        assert_eq!(buf, [1, 2, 3]);
    }

    #[test]
    fn writer_write_out_of_bounds() {
        let mut buf = [0; 3];
        let mut bufmut = BufMut::from(&mut buf[..]);
        let res = bufmut.write_bytes(&[1, 2, 3, 4]);
        assert!(res.is_err());
        assert_eq!(buf, [0; 3]);
    }

    #[test]
    fn writer_write_write() {
        let mut buf = [0; 5];
        let mut bufmut = BufMut::from(&mut buf[..]);
        bufmut.write_bytes(&[1, 2]).unwrap();
        bufmut.write_bytes(&[3]).unwrap();
        assert_eq!(buf, [1, 2, 3, 0, 0]);
    }

    #[test]
    fn writer_skip() {
        let mut buf = [0; 3];
        let mut bufmut = BufMut::from(&mut buf[..]);
        bufmut.skip(3).unwrap();
        assert_eq!(buf, [0; 3]);
    }

    #[test]
    fn writer_skip_out_of_bounds() {
        let mut buf = [0; 3];
        let mut bufmut = BufMut::from(&mut buf[..]);
        let res = bufmut.skip(4);
        assert!(res.is_err());
    }

    #[test]
    fn writer_skip_capacity() {
        let mut buf = [0; 5];
        let mut bufmut = BufMut::from(&mut buf[..]);
        assert_eq!(bufmut.capacity(), 5);
        bufmut.skip(2).unwrap();
        assert_eq!(bufmut.capacity(), 3);
    }

    #[test]
    fn writer_skip_write() {
        let mut buf = [0; 5];
        let mut bufmut = BufMut::from(&mut buf[..]);
        bufmut.skip(2).unwrap();
        bufmut.write_bytes(&[1, 1]).unwrap();
        assert_eq!(buf, [0, 0, 1, 1, 0]);
    }
}
