#![allow(unused)]

use crate::ImageFormat;
use crate::KinectError;
use k4a_sys_temp as k4a_sys;
use std::ptr::null_mut;
use crate::error::CreateImageError;

/// Adapted from k4a-sys. Represents an image within a capture.
#[derive(Debug)]
pub struct Image(pub k4a_sys::k4a_image_t);

impl Image {

    /// Create a blank image.
    ///
    /// This function is used to create images of formats that have consistent stride. The function
    /// is not suitable for compressed formats that may not be represented by the same number of bytes
    /// per line.
    ///
    /// For most image formats, the function will allocate an image buffer of size
    /// height_pixels * stride_bytes. Buffers K4A_IMAGE_FORMAT_COLOR_NV12 format will allocate an
    /// additional height_pixels / 2 set of lines (each of stride_bytes). This function cannot be used
    /// to allocate K4A_IMAGE_FORMAT_COLOR_MJPG buffers.
    ///
    /// To create an image object without the API allocating memory, or to represent an image that has
    /// a non-deterministic stride, use k4a_image_create_from_buffer().
    ///
    /// The k4a_image_t is created with a reference count of 1.
    ///
    /// When finished using the created image, release it with k4a_image_release.
    ///
    ///   stride_bytes - The number of bytes per horizontal line of the image. If set to 0, the stride
    ///                  will be set to the minimum size given the format and width_pixels.
    ///
    pub fn create(format: ImageFormat,
                  width: u32,
                  height: u32,
                  stride_bytes: u32)
                  -> Result<Self, CreateImageError>
    {
        let mut handle = null_mut();

        let result = unsafe {
            k4a_sys::k4a_image_create(
                format as k4a_sys::k4a_image_format_t,
                width as i32,
                height as i32,
                stride_bytes as i32,
                &mut handle
            )
        };

        if result != k4a_sys::k4a_result_t_K4A_RESULT_SUCCEEDED {
            // NB: Linux and Windows platforms differ in integer types used here, so we cast this.
            // Linux uses u32 and Windows uses i32.
            // This should be fixed in the `k4a-sys` build script.
            return Err(CreateImageError { error_code: result as i32 });
        }

        Ok(Image(handle))
    }

    pub fn get_height_pixels(&self) -> usize {
        unsafe {
            k4a_sys::k4a_image_get_height_pixels(self.0) as usize
        }
    }

    pub fn get_width_pixels(&self) -> usize {
        unsafe {
            k4a_sys::k4a_image_get_width_pixels(self.0) as usize
        }
    }

    pub fn get_stride_bytes(&self) -> usize {
        unsafe {
            k4a_sys::k4a_image_get_stride_bytes(self.0) as usize
        }
    }

    pub fn get_size(&self) -> usize {
        unsafe {
            k4a_sys::k4a_image_get_size(self.0) // returns size_t
        }
    }

    pub fn get_buffer(&self) -> *mut u8 {
        unsafe {
            k4a_sys::k4a_image_get_buffer(self.0)
        }
    }

    /// Use this function to determine the format of the image buffer.
    /// This function is not expected to fail, all k4a_image_t's are created with a
    /// known format. If the image_handle is invalid, the function will return
    /// K4A_IMAGE_FORMAT_CUSTOM.
    pub fn get_format(&self) -> ImageFormat {
        let format = unsafe {
            k4a_sys::k4a_image_get_format(self.0)
        };
        format.into()
    }

    /// Returns the underlying opaque handle *without* an additional refcount.
    /// Do not deallocate it.
    pub fn get_handle(&self) -> k4a_sys::k4a_image_t {
        self.0
    }
}

/// Remove a libk4a image refcount on every drop.
/// When the refcount drops to zero, the image goes away.
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            k4a_sys::k4a_image_release(self.0);
        }
    }
}

/// Handles are refcounted by libk4a. The final reference is destroyed
impl Clone for Image {
    fn clone(&self) -> Self {
        // We must increment the refcount.
        let handle = self.get_handle();
        unsafe {
            k4a_sys::k4a_image_reference(handle);
        }
        Self {
            0: handle,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let handle = source.get_handle();
        unsafe {
            k4a_sys::k4a_image_reference(handle);
        }
        unsafe {
            // drop refcount, potentially releasing if reached zero
            k4a_sys::k4a_image_release(self.0);
        }
        self.0 = handle;
    }
}

