#![allow(unused)]

use k4a_sys_temp as k4a_sys;

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
