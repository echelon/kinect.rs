#![allow(unused)]

use crate::Calibration;
use crate::Capture;
use crate::CaptureError;
use crate::DeviceConfiguration;
use crate::GetCalibrationError;
use crate::GetCaptureError;
use crate::KinectError;
use crate::SynchronizationJackStatus;

use k4a_sys_temp as k4a_sys;
use std::mem::MaybeUninit;
use std::{ptr, fmt};
use crate::error::DeviceOpenError;

/// A Kinect Device Handle
#[derive(Debug)]
pub struct Device {
    pub device_pointer: k4a_sys::k4a_device_t,
}

// These are ref-counted handles and are safe to Send.
unsafe impl Send for Device{}

// Deallocate open device handles
impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            k4a_sys::k4a_device_close(self.device_pointer);
        }
    }
}

impl Device {
    /// Get the number of installed devices
    pub fn get_installed_count() -> u32 {
        unsafe {
            k4a_sys::k4a_device_get_installed_count()
        }
    }

    /// Open a device with the given index
    pub fn open(device_index: u32) -> Result<Self, DeviceOpenError> {
        let mut device_pointer: k4a_sys::k4a_device_t = ptr::null_mut();
        let result = unsafe {
            k4a_sys::k4a_device_open(device_index, &mut device_pointer)
        };
        if result != k4a_sys::k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
            return Err(DeviceOpenError { error_code: result })
        }
        Ok(Device {
            device_pointer,
        })
    }

    /// Fetch the device serial number.
    pub fn get_serial_number(&self) -> Result<String, KinectError> {
        // First we interrogate the serial number size.
        let mut serial_number_length: usize = 0;

        let result = unsafe {
            k4a_sys::k4a_device_get_serialnum(self.device_pointer, ptr::null_mut(), &mut serial_number_length)
        };

        if result != k4a_sys::k4a_buffer_result_t_K4A_BUFFER_RESULT_TOO_SMALL {
            return Err(KinectError::UnableToGetSerialNumber);
        }

        // Now we request to fill a serial number buffer.
        let mut serial_number = vec![0i8; serial_number_length];
        let serial_number_ptr = (&mut serial_number).as_mut_ptr();

        let result = unsafe {
            k4a_sys::k4a_device_get_serialnum(self.device_pointer, serial_number_ptr, &mut serial_number_length)
        };

        if result != k4a_sys::k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
            return Err(KinectError::UnableToGetSerialNumber);
        }

        // NB: Library shouldn't be returning i8's
        let serial_number = serial_number.iter().map(|v| *v as u8).collect();

        String::from_utf8(serial_number)
            .map(|s| s.trim_matches(char::from(0)).into()) // Remove trailing null byte
            .map_err(|_| KinectError::UnableToGetSerialNumber)
    }

    /// Get the device synchronization jack statuses.
    /// Each device has an 'in' jack and an 'out' jack.
    pub fn get_synchronization_jack_status(&self) -> Result<SynchronizationJackStatus, KinectError> {
        let mut sync_in_jack_connected = false;
        let mut sync_out_jack_connected = false;

        let result = unsafe {
            k4a_sys::k4a_device_get_sync_jack(self.device_pointer,
                                              &mut sync_in_jack_connected, &mut sync_out_jack_connected)
        };

        if result != k4a_sys::k4a_result_t_K4A_RESULT_SUCCEEDED {
            return Err(KinectError::UnableToGetSyncJackStatus { error_code: result });
        }

        Ok(SynchronizationJackStatus {
            sync_in_jack_connected,
            sync_out_jack_connected,
        })
    }

    // TODO: having a 'DeviceConfigurations' struct that goes unused is kind of gross.
    /// Start the cameras
    pub fn start_cameras(&self,
                         device_config: k4a_sys::k4a_device_configuration_t)
                         -> Result<(), KinectError>
    {
        let result = unsafe {
            k4a_sys::k4a_device_start_cameras(self.device_pointer, &device_config)
        };

        if result != k4a_sys::k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
            return Err(KinectError::UnableToStartCameras { error_code: result });
        }

        return Ok(())
    }

    // TODO: More sensible defaults, or get rid of this entirely.
    /// Start the cameras.
    pub fn start_cameras_default_config(&self) -> Result<(), KinectError> {
        let mut device_config = DeviceConfiguration::new();
        // NB: Although the Kinect docs say this format isn't natively supported by the color camera
        // and that extra CPU is required, this is the only color mode supported by 'k4aviewer' 3D view.
        device_config.0.color_format = k4a_sys::k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_BGRA32;
        device_config.0.color_resolution = k4a_sys::k4a_color_resolution_t_K4A_COLOR_RESOLUTION_2160P;
        device_config.0.depth_mode = k4a_sys::k4a_depth_mode_t_K4A_DEPTH_MODE_NFOV_UNBINNED;
        device_config.0.camera_fps = k4a_sys::k4a_fps_t_K4A_FRAMES_PER_SECOND_30;

        self.start_cameras(device_config.0)
    }

    /// Stops the color and depth camera capture.
    ///
    /// The streaming of individual sensors stops as a result of this call. Once called,
    /// k4a_device_start_cameras() may be called again to resume sensor streaming.
    /// This function may be called while another thread is blocking in k4a_device_get_capture().
    /// Calling this function while another thread is in that function will result in that function
    /// returning a failure.
    pub fn stop_cameras(&self) {
        unsafe {
            k4a_sys::k4a_device_stop_cameras(self.device_pointer)
        }
    }

    /// Get capture and return a new buffer.
    pub fn get_capture(&self, timeout_ms: i32) -> Result<Capture, GetCaptureError> {
        let mut capture_buffer: k4a_sys::k4a_capture_t = ptr::null_mut();
        self.get_capture_buffered(&mut capture_buffer, timeout_ms)
            .map(|_| Capture(capture_buffer)) // TODO: Can capture be null?
    }

    /// Get capture and reuse an existing buffer.
    pub fn get_capture_buffered(&self, capture_buffer: &mut k4a_sys::k4a_capture_t, timeout_ms: i32)
                                -> Result<(), GetCaptureError>
    {
        let result = unsafe {
            k4a_sys::k4a_device_get_capture(self.device_pointer, capture_buffer, timeout_ms)
        };

        match result {
            k4a_sys::k4a_wait_result_t_K4A_WAIT_RESULT_SUCCEEDED => { /* ok, continue */ },
            k4a_sys::k4a_wait_result_t_K4A_WAIT_RESULT_TIMEOUT => {
                return Err(GetCaptureError::TimeoutError);
            },
            k4a_sys::k4a_wait_result_t_K4A_WAIT_RESULT_FAILED => {
                return Err(GetCaptureError::TimeoutError);
            }
            _ => {
                return Err(GetCaptureError::UnknownError(result));
            }
        }

        Ok(())
    }

    /// Get the camera calibration for the entire Azure Kinect device.
    ///
    /// The calibration represents the data needed to transform between the camera views and may be
    /// different for each operating depth_mode and color_resolution the device is configured to
    /// operate in.
    /// The calibration output is used as input to all calibration and transformation functions.
    pub fn get_calibration(&self,
                           depth_mode: k4a_sys::k4a_depth_mode_t,
                           color_resolution: k4a_sys::k4a_color_resolution_t)
                           -> Result<Calibration, GetCalibrationError>
    {
        // TODO: Why isn't the way I've been using to init structures before still working?
        //let mut calibration_buffer: k4a_sys::k4a_calibration_t = ptr::null_mut();
        /*let mut calibration_buffer: k4a_sys::k4a_calibration_t = k4a_sys::k4a_calibration_t {
          color_camera_calibration: 0,
          color_resolution: 0,
          depth_camera_calibration: 0,
          depth_mode: 0,
          extrinsics: [0,0,0,0],
        };*/

        unsafe {
            let mut calibration_buffer: MaybeUninit<k4a_sys::k4a_calibration_t> = MaybeUninit::uninit();

            let result =  k4a_sys::k4a_device_get_calibration(
                self.device_pointer,
                depth_mode,
                color_resolution,
                calibration_buffer.as_mut_ptr(),
            );

            match result {
                k4a_sys::k4a_result_t_K4A_RESULT_SUCCEEDED  => { /* ok, continue */ },
                k4a_sys::k4a_result_t_K4A_RESULT_FAILED => {
                    return Err(GetCalibrationError::FailedError);
                },
                _ => {
                    return Err(GetCalibrationError::UnknownError(result));
                },
            }

            let handle = calibration_buffer.assume_init();
            let calibration = Calibration(handle);

            Ok(calibration)
        }
    }
}

