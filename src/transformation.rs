#![allow(unused)]

use crate::Resolution;
use crate::Calibration;

use k4a_sys_temp as k4a_sys;

#[derive(Clone,Debug)]
pub struct Transformation {
    transformation: k4a_sys::k4a_transformation_t,
    pub color_resolution: Resolution,
    pub depth_resolution: Resolution,
}

impl Transformation {
    /// Creates a transformation associated with a calibration
    pub fn from_calibration(calibration: &Calibration) -> Self {
        let transformation = unsafe {
            k4a_sys::k4a_transformation_create(&calibration.0)
        };
        Self {
            transformation,
            color_resolution: Resolution {
                width: calibration.0.color_camera_calibration.resolution_width,
                height: calibration.0.color_camera_calibration.resolution_height,
            },
            depth_resolution: Resolution {
                width: calibration.0.depth_camera_calibration.resolution_width,
                height: calibration.0.depth_camera_calibration.resolution_height,
            },
        }
    }

    /// Returns the underlying opaque handle *without* an additional refcount.
    /// Do not deallocate it.
    pub fn get_handle(&self) -> k4a_sys::k4a_transformation_t {
        self.transformation
    }
}

impl Drop for Transformation {
    fn drop(&mut self) {
        unsafe {
            k4a_sys::k4a_transformation_destroy(self.transformation);
        }
    }
}
