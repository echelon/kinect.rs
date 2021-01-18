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

    /// Returns a default configuration with everything disabled, mimicking the constant
    /// K4A_DEVICE_CONFIG_INIT_DISABLE_ALL.
    ///
    /// K4A docs: "Initial configuration setting for disabling all sensors."
    /// "    Use this setting to initialize a k4a_device_configuration_t to a disabled state."
    pub fn init_disable_all() -> Self {
        // This is the same as K4A_DEVICE_CONFIG_INIT_DISABLE_ALL, which I can't seem to reference.
        Self (k4a_sys::k4a_device_configuration_t {
            color_format: k4a_sys::k4a_image_format_t_K4A_IMAGE_FORMAT_COLOR_MJPG,
            color_resolution: k4a_sys::k4a_color_resolution_t_K4A_COLOR_RESOLUTION_OFF,
            depth_mode: k4a_sys::k4a_depth_mode_t_K4A_DEPTH_MODE_OFF,
            camera_fps: k4a_sys::k4a_fps_t_K4A_FRAMES_PER_SECOND_30,
            synchronized_images_only: false,
            depth_delay_off_color_usec: 0,
            wired_sync_mode: k4a_sys::k4a_wired_sync_mode_t_K4A_WIRED_SYNC_MODE_STANDALONE,
            subordinate_delay_off_master_usec: 0,
            disable_streaming_indicator: false,
        })
    }
}

