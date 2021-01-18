use k4a_sys_temp as k4a_sys;

/// Copied from k4a-sys
pub struct DeviceConfiguration (pub k4a_sys::k4a_device_configuration_t);

/// Copied from k4a-sys
impl DeviceConfiguration {
    pub fn new() -> Self {
        Self (k4a_sys::k4a_device_configuration_t {
            color_format: k4a_sys::k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            color_resolution: k4a_sys::k4a_color_resolution_t_K4A_COLOR_RESOLUTION_720P,
            depth_mode: k4a_sys::k4a_depth_mode_t_K4A_DEPTH_MODE_WFOV_2X2BINNED,
            camera_fps: 0,
            synchronized_images_only: false,
            depth_delay_off_color_usec: 0,
            wired_sync_mode: 0,
            subordinate_delay_off_master_usec: 0,
            disable_streaming_indicator: false,
        })
    }
}

