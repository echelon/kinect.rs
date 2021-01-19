//! Crate error types

use std::fmt;
use std::error::Error;

/// Represents errors creating images with `k4a_image_create`.
#[derive(Copy, Clone, Debug)]
pub struct CreateImageError {
    /// The error code returned by libk4a.
    pub error_code: i32,
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

/// Represents errors opening devices with `k4a_device_get_calibration`.
#[derive(Copy, Clone, Debug)]
pub enum DeviceGetCalibrationError {
    /// Failed to get device calibration.
    FailedError,
    /// Unexpected error code returned by libk4a.
    UnexpectedError(i32),
}

impl fmt::Display for DeviceGetCalibrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceGetCalibrationError::FailedError =>
                write!(f, "DeviceGetCalibrationError::FailedError"),
            DeviceGetCalibrationError::UnexpectedError(code) =>
                write!(f, "DeviceGetCalibrationError::UnexpectedError (code: {})", code),
        }
    }
}

impl Error for DeviceGetCalibrationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Represents errors opening devices with `k4a_device_get_capture`.
#[derive(Copy, Clone, Debug)]
pub enum DeviceGetCaptureError {
    /// It took too long to get the capture, and our timeout elapsed.
    TimeoutError,
    /// There was a failure in getting the capture
    FailedError,
    /// Unexpected error code returned by libk4a
    UnexpectedError(i32),
}

impl fmt::Display for DeviceGetCaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceGetCaptureError::TimeoutError=>
                write!(f, "DeviceGetCaptureError::TimeoutError"),
            DeviceGetCaptureError::FailedError =>
                write!(f, "DeviceGetCaptureError::FailedError"),
            DeviceGetCaptureError::UnexpectedError(code) =>
                write!(f, "DeviceGetCaptureError::UnexpectedError (code: {})", code),
        }
    }
}

impl Error for DeviceGetCaptureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Represents errors opening devices with `k4a_device_open`.
#[derive(Copy, Clone, Debug)]
pub struct DeviceOpenError {
    pub error_code: i32,
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
    pub error_code: i32,
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
