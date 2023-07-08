//! Utilities for implementing [`Reader`] and [`Writer`].

use super::WriteTooLargeError;

#[cold]
pub(crate) fn cold() {}

/// Helper function checking that `requested <= capacity` and creating an
/// appropriate [`WriteTooLargeError`] if this is not the case.
pub fn check_write_size(requested: usize, capacity: usize) -> Result<(), WriteTooLargeError> {
    if requested <= capacity {
        Ok(())
    } else {
        Err(WriteTooLargeError {
            requested,
            capacity,
        })
    }
}
