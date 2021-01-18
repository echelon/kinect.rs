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

/// Represents errors opening devices with `k4a_device_open`.
#[derive(Copy, Clone, Debug)]
pub struct DeviceOpenError {
    /// The error code returned by libk4a.
    pub error_code: u32,
}

impl fmt::Display for DeviceOpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceOpenError (code: {})", self.error_code)
    }
}

impl Error for DeviceOpenError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Represents errors opening devices with `k4a_device_start_cameras`.
#[derive(Copy, Clone, Debug)]
pub struct DeviceStartCamerasError {
    /// The error code returned by libk4a.
    pub error_code: u32,
}

impl fmt::Display for DeviceStartCamerasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceStartCamerasError (code: {})", self.error_code)
    }
}

impl Error for DeviceStartCamerasError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
