//! High level interface for Azure Kinect.

use std::error::Error;
use std::fmt;

// Re-export patched crate
// Normally we'd follow k4a-sys upstream, but it doesn't properly build on Linux.
pub use k4a_sys_temp as k4a_sys;

mod calibration;
mod capture;
mod device;
mod device_configuration;
mod image;
mod image_format;
mod transformation;

pub use {
    calibration::Calibration,
    capture::Capture,
    device::Device,
    device_configuration::DeviceConfiguration,
    image::Image,
    image_format::ImageFormat,
    transformation::Transformation,
};

pub mod error;

/// A library error
#[derive(Debug)]
pub enum KinectError {
    UnableToOpen { error_code: u32 },
    UnableToGetSerialNumber,
    UnableToStartCameras { error_code: u32 },
    UnableToCreateImage { error_code: u32 },
    UnableToGetSyncJackStatus { error_code: u32 },
}

/// Synchronization jack status.
#[derive(Debug,Copy,Clone)]
pub struct SynchronizationJackStatus {
    pub sync_in_jack_connected: bool,
    pub sync_out_jack_connected: bool,
}

/// Errors for GetCapture
#[derive(Debug)]
pub enum GetCaptureError {
    TimeoutError,
    FailedError,
    UnknownError(u32),
}

#[derive(Debug)]
pub enum CaptureError {
    NullCapture,
}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CaptureError")
    }
}

impl Error for CaptureError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Errors for GetCalibration
#[derive(Debug)]
pub enum GetCalibrationError {
    FailedError,
    UnknownError(u32),
}

#[derive(Clone,Debug)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

