//! Crate error types

use std::fmt;
use std::error::Error;

/// Represents errors creating images with `k4a_image_create`.
#[derive(Copy, Clone, Debug)]
pub struct CreateImageError {
    /// The error code returned by libk4a.
    pub error_code: u32,
}

impl fmt::Display for CreateImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CreateImageError (code: {})", self.error_code)
    }
}

impl Error for CreateImageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
