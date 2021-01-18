//! High level interface for Azure Kinect.

use std::mem::MaybeUninit;
use std::{ptr, fmt};
use std::ptr::null_mut;

pub use k4a_sys_temp as k4a_sys;

use std::error::Error;

/// A library error
#[derive(Debug)]
pub enum KinectError {
  UnableToOpen { error_code: u32 },
  UnableToGetSerialNumber,
  UnableToStartCameras { error_code: u32 },
  UnableToCreateImage { error_code: u32 },
  UnableToGetSyncJackStatus { error_code: u32 },
}

/// A Kinect Device Handle
#[derive(Debug)]
pub struct Device {
  device_pointer: k4a_sys::k4a_device_t,
}

impl Device {
  /// Get the number of installed devices
  pub fn get_installed_count() -> u32 {
    unsafe {
      k4a_sys::k4a_device_get_installed_count()
    }
  }

  /// Open a device with the given index
  pub fn open(device_index: u32) -> Result<Self, KinectError> {
    let mut device_pointer: k4a_sys::k4a_device_t = ptr::null_mut();
    unsafe {
      let result = k4a_sys::k4a_device_open(device_index, &mut device_pointer);
      if result != k4a_sys::k4a_buffer_result_t_K4A_BUFFER_RESULT_SUCCEEDED {
        return Err(KinectError::UnableToOpen { error_code: result })
      }
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

#[derive(Clone)]
pub struct Calibration(pub k4a_sys::k4a_calibration_t);

impl Calibration {
  pub fn default() -> Self {
    let extrinsics = k4a_sys::_k4a_calibration_extrinsics_t {
      rotation: [0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32],
      translation: [0.0f32, 0.0f32, 0.0f32],
    };

    let extrinsics_4 = [
      extrinsics.clone(),
      extrinsics.clone(),
      extrinsics.clone(),
      extrinsics.clone(),
    ];

    let extrinsics_4_4 = [
      extrinsics_4.clone(),
      extrinsics_4.clone(),
      extrinsics_4.clone(),
      extrinsics_4.clone(),
    ];

    let camera_calibration = k4a_sys::_k4a_calibration_camera_t {
      resolution_width: 0,
      resolution_height: 0,
      metric_radius: 0.0f32,
      extrinsics: extrinsics.clone(),
      intrinsics: k4a_sys::_k4a_calibration_intrinsics_t {
        type_: 0,
        parameter_count: 0,
        parameters: k4a_sys::k4a_calibration_intrinsic_parameters_t {
          param: k4a_sys::k4a_calibration_intrinsic_parameters_t__param {
            cx: 0.0f32,
            cy: 0.0f32,
            fx: 0.0f32,
            fy: 0.0f32,
            k1: 0.0f32,
            k2: 0.0f32,
            k3: 0.0f32,
            k4: 0.0f32,
            k5: 0.0f32,
            k6: 0.0f32,
            codx: 0.0f32,
            cody: 0.0f32,
            p2: 0.0f32,
            p1: 0.0f32,
            metric_radius: 0.0f32,
          },
        },
      },
    };

    Self(k4a_sys::k4a_calibration_t {
      color_camera_calibration: camera_calibration.clone(),
      depth_camera_calibration: camera_calibration.clone(),
      color_resolution: 0,
      depth_mode: 0,
      extrinsics: extrinsics_4_4,
    })
  }

  // TODO: Make this the `Debug` trait output instead.
  pub fn debug_print(&self) {
    println!("===== CALIBRATION =====");
    println!("\t Color resolution: {}", self.0.color_resolution);
    println!("\t Depth mode: {}", self.0.depth_mode);
    println!("\t Extrinsics: {:?}", self.0.extrinsics);

    println!("\t depth.resolution_width: {}", self.0.depth_camera_calibration.resolution_width);
    println!("\t depth.resolution_height: {}", self.0.depth_camera_calibration.resolution_height);
    println!("\t depth.metric_radius: {}", self.0.depth_camera_calibration.metric_radius);
    println!("\t depth.extrinsics: {:?}", self.0.depth_camera_calibration.extrinsics);
    println!("\t depth.intrinsics.type: {}", self.0.depth_camera_calibration.intrinsics.type_);
    println!("\t depth.intrinsics.parameter_count: {}", self.0.depth_camera_calibration.intrinsics.parameter_count);
    unsafe {
      // NB: This is a union field, so we have to use unsafe access
      println!("\t depth.intrinsics.parameters.param.cx: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.cx);
      println!("\t depth.intrinsics.parameters.param.cy: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.cy);
      println!("\t depth.intrinsics.parameters.param.fx: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.fx);
      println!("\t depth.intrinsics.parameters.param.fy: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.fy);
      println!("\t depth.intrinsics.parameters.param.k1: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k1);
      println!("\t depth.intrinsics.parameters.param.k2: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k2);
      println!("\t depth.intrinsics.parameters.param.k3: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k3);
      println!("\t depth.intrinsics.parameters.param.k4: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k4);
      println!("\t depth.intrinsics.parameters.param.k5: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k5);
      println!("\t depth.intrinsics.parameters.param.k6: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.k6);
      println!("\t depth.intrinsics.parameters.param.codx: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.codx);
      println!("\t depth.intrinsics.parameters.param.cody: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.cody);
      println!("\t depth.intrinsics.parameters.param.p2: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.p2);
      println!("\t depth.intrinsics.parameters.param.p1: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.p1);
      println!("\t depth.intrinsics.parameters.param.metric_radius: {}", self.0.depth_camera_calibration.intrinsics.parameters.param.metric_radius);
    }
    println!("\t color.resolution_width: {}", self.0.color_camera_calibration.resolution_width);
    println!("\t color.resolution_height: {}", self.0.color_camera_calibration.resolution_height);
    println!("\t color.metric_radius: {}", self.0.color_camera_calibration.metric_radius);
    println!("\t color.extrinsics: {:?}", self.0.color_camera_calibration.extrinsics);
    println!("\t color.intrinsics.type: {}", self.0.color_camera_calibration.intrinsics.type_);
    println!("\t color.intrinsics.parameter_count: {}", self.0.color_camera_calibration.intrinsics.parameter_count);
    unsafe {
      // NB: This is a union field, so we have to use unsafe access
      println!("\t color.intrinsics.parameters.param.cx: {}", self.0.color_camera_calibration.intrinsics.parameters.param.cx);
      println!("\t color.intrinsics.parameters.param.cy: {}", self.0.color_camera_calibration.intrinsics.parameters.param.cy);
      println!("\t color.intrinsics.parameters.param.fx: {}", self.0.color_camera_calibration.intrinsics.parameters.param.fx);
      println!("\t color.intrinsics.parameters.param.fy: {}", self.0.color_camera_calibration.intrinsics.parameters.param.fy);
      println!("\t color.intrinsics.parameters.param.k1: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k1);
      println!("\t color.intrinsics.parameters.param.k2: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k2);
      println!("\t color.intrinsics.parameters.param.k3: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k3);
      println!("\t color.intrinsics.parameters.param.k4: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k4);
      println!("\t color.intrinsics.parameters.param.k5: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k5);
      println!("\t color.intrinsics.parameters.param.k6: {}", self.0.color_camera_calibration.intrinsics.parameters.param.k6);
      println!("\t color.intrinsics.parameters.param.codx: {}", self.0.color_camera_calibration.intrinsics.parameters.param.codx);
      println!("\t color.intrinsics.parameters.param.cody: {}", self.0.color_camera_calibration.intrinsics.parameters.param.cody);
      println!("\t color.intrinsics.parameters.param.p2: {}", self.0.color_camera_calibration.intrinsics.parameters.param.p2);
      println!("\t color.intrinsics.parameters.param.p1: {}", self.0.color_camera_calibration.intrinsics.parameters.param.p1);
      println!("\t color.intrinsics.parameters.param.metric_radius: {}", self.0.color_camera_calibration.intrinsics.parameters.param.metric_radius);
    }
    println!("==========");
  }
}

/// Adapted from k4a-sys. Represents a capture.
#[derive(Debug)]
pub struct Capture(pub k4a_sys::k4a_capture_t);

impl Capture {
  pub fn get_depth_image(&self) -> Result<Image, CaptureError> {
    let image = unsafe {
      k4a_sys::k4a_capture_get_depth_image(self.0)
    };
    if image.is_null() {
      return Err(CaptureError::NullCapture);
    }
    Ok(Image(image))
  }

  pub fn get_color_image(&self) -> Result<Image, CaptureError> {
    let image = unsafe {
      k4a_sys::k4a_capture_get_color_image(self.0)
    };
    if image.is_null() {
      return Err(CaptureError::NullCapture);
    }
    Ok(Image(image))
  }

  pub fn get_ir_image(&self) -> Result<Image, CaptureError> {
    let image = unsafe {
      k4a_sys::k4a_capture_get_ir_image(self.0)
    };
    if image.is_null() {
      return Err(CaptureError::NullCapture);
    }
    Ok(Image(image))
  }

  /// Returns the underlying opaque handle *without* an additional refcount.
  /// Do not deallocate it.
  pub fn get_handle(&self) -> k4a_sys::k4a_capture_t {
    self.0
  }
}

/// Remove a libk4a capture refcount on every drop.
/// When the refcount drops to zero, the capture goes away.
impl Drop for Capture {
  fn drop(&mut self) {
    unsafe {
      k4a_sys::k4a_capture_release(self.0);
      self.0 = null_mut();
    }
  }
}

/// Handles are refcounted by libk4a. The final reference is destroyed
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

/// FIXME FIXME FIXME: WHY IS THIS NOT SEND WITH ARC<MUTEX<T>>!?
/// We are making k4a_sys::k4a_capture_t Send only when wrapped with Arc<Mutex<>>,
/// but the compiler can't figure that out. Freaky gross.
///
/// "Rust automatically determines whether a type is Send and/or Sync. Anything that has a
/// raw ptr inside, which is what you have here, is considered !Send and !Sync. This isn’t because
/// it automatically means the type is unsafe in Send/Sync terms, but rather more like a lint for
/// the code author: they need to determine the safety themselves, and then if they’re safe,
/// manually impl Send and/or Sync, as appropriate, for the type."
///
/// Actually Rust is doing a good job here.
///
/// https://users.rust-lang.org/t/solved-how-to-move-non-send-between-threads-or-an-alternative/19928/11
unsafe impl Send for Capture{}

/// TODO: We also need to send device
unsafe impl Send for Device{}

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
    -> Result<Self, KinectError>
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
      return Err(KinectError::UnableToCreateImage { error_code: result });
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

#[derive(Debug,Clone,Copy)]
pub enum ImageFormat {
  ColorMjpg,
  ColorNv12,
  ColorYuy2,
  ColorBgra32,
  /// Depth image type DEPTH16.
  /// Each pixel of DEPTH16 data is two bytes of little endian unsigned depth data.
  /// The unit of the data is in millimeters from the origin of the camera.
  /// Stride indicates the length of each line in bytes and should be used to determine
  /// the start location of each line of the image in memory.
  Depth16,
  Ir16,
  Custom8,
  Custom16,
  Custom,
  UnknownFormatError, // FIXME: Just return Result<T>?
}

impl ImageFormat {
  pub fn to_k4a(&self) -> k4a_sys::k4a_image_format_t {
    match self {
      ImageFormat::ColorMjpg => 0,
      ImageFormat::ColorNv12 => 1,
      ImageFormat::ColorYuy2 => 2,
      ImageFormat::ColorBgra32 => 3,
      ImageFormat::Depth16 => 4,
      ImageFormat::Ir16 => 5,
      ImageFormat::Custom8 => 6,
      ImageFormat::Custom16 => 7,
      ImageFormat::Custom => 8,
      ImageFormat::UnknownFormatError => 255, // TODO: Better handling?
    }
  }
}

impl From<k4a_sys::k4a_image_format_t> for ImageFormat {
  fn from(format: k4a_sys::k4a_image_format_t) -> Self {
    match format {
      0 => ImageFormat::ColorMjpg,
      1 => ImageFormat::ColorNv12,
      2 => ImageFormat::ColorYuy2,
      3 => ImageFormat::ColorBgra32,
      4 => ImageFormat::Depth16,
      5 => ImageFormat::Ir16,
      6 => ImageFormat::Custom8,
      7 => ImageFormat::Custom16,
      8 => ImageFormat::Custom,
      _ => ImageFormat::UnknownFormatError,
    }
  }
}

/// Deallocate open device handles
impl Drop for Device {
  fn drop(&mut self) {
    unsafe {
      k4a_sys::k4a_device_close(self.device_pointer);
    }
  }
}

/// Copied from k4a-sys
pub struct DeviceConfiguration (k4a_sys::k4a_device_configuration_t);

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

#[derive(Clone,Debug)]
pub struct Resolution {
  pub width: i32,
  pub height: i32,
}

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
