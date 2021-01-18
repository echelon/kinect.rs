#![allow(unused)]

use crate::Image;
use k4a_sys_temp as k4a_sys;
use std::ptr::null_mut;

/// Represents a single capture.
#[derive(Debug)]
pub struct Capture(pub k4a_sys::k4a_capture_t);

// These are ref-counted handles and are safe to Send.
unsafe impl Send for Capture{}

// Remove a libk4a capture refcount on every drop.
// When the refcount drops to zero, the capture goes away.
impl Drop for Capture {
    fn drop(&mut self) {
        unsafe {
            k4a_sys::k4a_capture_release(self.0);
            self.0 = null_mut();
        }
    }
}

// Handles are refcounted by libk4a. The final reference is destroyed.
impl Clone for Capture {
    fn clone(&self) -> Self {
        // We must increment the refcount.
        let handle = self.get_handle();
        unsafe {
            k4a_sys::k4a_capture_reference(handle);
        }
        Self {
            0: handle,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        let handle = source.get_handle();
        unsafe {
            k4a_sys::k4a_capture_reference(handle);
        }
        unsafe {
            // drop refcount, potentially releasing if reached zero
            k4a_sys::k4a_capture_release(self.0);
        }
        self.0 = handle;
    }
}

impl Capture {
    /// Get the depth image from the capture (if any).
    pub fn get_depth_image(&self) -> Option<Image> {
        let image = unsafe {
            k4a_sys::k4a_capture_get_depth_image(self.0)
        };
        if image.is_null() {
            return None;
        }
        Some(Image(image))
    }

    /// Get the color image from the capture (if any).
    pub fn get_color_image(&self) -> Option<Image> {
        let image = unsafe {
            k4a_sys::k4a_capture_get_color_image(self.0)
        };
        if image.is_null() {
            return None;
        }
        Some(Image(image))
    }

    /// Get the IR image from the capture (if any).
    pub fn get_ir_image(&self) -> Option<Image> {
        let image = unsafe {
            k4a_sys::k4a_capture_get_ir_image(self.0)
        };
        if image.is_null() {
            return None;
        }
        Some(Image(image))
    }

    /// Returns the underlying opaque handle *without* an additional refcount.
    /// Do not deallocate it.
    pub fn get_handle(&self) -> k4a_sys::k4a_capture_t {
        self.0
    }
}

