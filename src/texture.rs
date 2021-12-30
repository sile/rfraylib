use crate::structs::Rectangle;
use crate::{Color, Size};
use std::ffi::CString;
use std::os::raw::c_int;
use std::path::Path;

#[derive(Debug)]
pub struct RenderTexture {
    pub(crate) inner: raylib4_sys::RenderTexture, // TODO
}

#[derive(Debug)]
pub struct Texture {
    pub(crate) inner: raylib4_sys::Texture, // TODO
}

impl Texture {
    pub(crate) fn to_raw(&self) -> raylib4_sys::Texture {
        self.inner.clone()
    }
}

#[derive(Debug)]
pub struct Image(raylib4_sys::Image);

impl Image {
    /// Load image from file into CPU memory (RAM).
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path_to_cstring(path)?;
        let image = unsafe { raylib4_sys::LoadImage(path.as_ptr()) };
        if image.data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(image))
        }
    }

    /// Load image from RAW file data.
    pub fn load_raw<P: AsRef<Path>>(
        path: P,
        size: Size,
        format: PixelFormat,
        header_size: usize,
    ) -> Option<Self> {
        let path = path_to_cstring(path)?;
        let image = unsafe {
            raylib4_sys::LoadImageRaw(
                path.as_ptr(),
                size.width as c_int,
                size.height as c_int,
                format as c_int,
                header_size as c_int,
            )
        };
        if image.data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(image))
        }
    }

    /// Load image sequence from file (frames appended to image.data).
    pub fn load_anim<P: AsRef<Path>>(path: P) -> Option<(Self, usize)> {
        let path = path_to_cstring(path)?;
        let mut frames = 0;
        let image = unsafe { raylib4_sys::LoadImageAnim(path.as_ptr(), &mut frames) };
        if image.data == std::ptr::null_mut() {
            None
        } else {
            Some((Self(image), frames as usize))
        }
    }

    /// Load image from memory buffer.
    pub fn load_from_memory(file_type: &str, file_data: &[u8]) -> Option<Self> {
        let file_type = CString::new(file_type).ok()?;
        let image = unsafe {
            raylib4_sys::LoadImageFromMemory(
                file_type.as_ptr(),
                file_data.as_ptr(),
                file_data.len() as c_int,
            )
        };
        if image.data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(image))
        }
    }

    /// Load image from GPU texture data.
    pub fn load_from_texture(texture: &Texture) -> Option<Self> {
        let image = unsafe { raylib4_sys::LoadImageFromTexture(texture.to_raw()) };
        if image.data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(image))
        }
    }

    /// Load image from screen buffer and (screenshot).
    pub fn load_from_screen(_window: &crate::Window) -> Option<Self> {
        let image = unsafe { raylib4_sys::LoadImageFromScreen() };
        if image.data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(image))
        }
    }

    /// Export image data to file, returns true on success.
    pub fn export<P: AsRef<Path>>(&self, path: P) -> bool {
        path_to_cstring(path)
            .map(|path| unsafe { raylib4_sys::ExportImage(self.0, path.as_ptr()) })
            .unwrap_or(false)
    }

    /// Export image as code file defining an array of bytes, returns true on success.
    pub fn export_as_code<P: AsRef<Path>>(&self, path: P) -> bool {
        path_to_cstring(path)
            .map(|path| unsafe { raylib4_sys::ExportImageAsCode(self.0, path.as_ptr()) })
            .unwrap_or(false)
    }

    /// Generate image: plain color.
    pub fn generate_color(size: Size, color: Color) -> Self {
        Self(unsafe {
            raylib4_sys::GenImageColor(size.width as c_int, size.height as c_int, color.into())
        })
    }

    /// Generate image: vertical gradient.
    pub fn generate_gradient_v(size: Size, top: Color, bottom: Color) -> Self {
        Self(unsafe {
            raylib4_sys::GenImageGradientV(
                size.width as c_int,
                size.height as c_int,
                top.into(),
                bottom.into(),
            )
        })
    }

    /// Generate image: horizontal gradient.
    pub fn generate_gradient_h(size: Size, left: Color, right: Color) -> Self {
        Self(unsafe {
            raylib4_sys::GenImageGradientH(
                size.width as c_int,
                size.height as c_int,
                left.into(),
                right.into(),
            )
        })
    }

    /// Generate image: radial gradient.
    pub fn generate_gradient_radial(size: Size, density: f32, inner: Color, outer: Color) -> Self {
        Self(unsafe {
            raylib4_sys::GenImageGradientRadial(
                size.width as c_int,
                size.height as c_int,
                density,
                inner.into(),
                outer.into(),
            )
        })
    }

    /// Generate image: checked.
    pub fn generate_checked(
        size: Size,
        checks_x: usize,
        checks_y: usize,
        color1: Color,
        color2: Color,
    ) -> Self {
        Self(unsafe {
            raylib4_sys::GenImageChecked(
                size.width as c_int,
                size.height as c_int,
                checks_x as c_int,
                checks_y as c_int,
                color1.into(),
                color2.into(),
            )
        })
    }

    /// Generate image: white noise.
    pub fn generate_white_noise(size: Size, factor: f32) -> Self {
        Self(unsafe {
            raylib4_sys::GenImageWhiteNoise(size.width as c_int, size.height as c_int, factor)
        })
    }

    /// Generate image: cellular algorithm. Bigger tileSize means bigger cells.
    pub fn generate_cellular(size: Size, tile_size: usize) -> Self {
        Self(unsafe {
            raylib4_sys::GenImageCellular(
                size.width as c_int,
                size.height as c_int,
                tile_size as c_int,
            )
        })
    }

    /// Create an image from another image piece.
    pub fn to_trimed_image(&self, rectangle: Rectangle) -> Self {
        Self(unsafe { raylib4_sys::ImageFromImage(self.0, rectangle.into()) })
    }

    /// Create an image from text (default font)
    pub fn from_text(text: &str, font_size: usize, color: Color) -> Option<Self> {
        let text = CString::new(text).ok()?;
        Some(Self(unsafe {
            raylib4_sys::ImageText(text.as_ptr(), font_size as c_int, color.into())
        }))
    }

    // TODO: ImageTextEx

    /// Convert image data to desired format.
    pub fn set_image_format(&mut self, format: PixelFormat) {
        unsafe { raylib4_sys::ImageFormat(&mut self.0, format as c_int) };
    }

    /// Convert image to POT (power-of-two).
    pub fn convert_to_pot(&mut self, fill: Color) {
        unsafe { raylib4_sys::ImageToPOT(&mut self.0, fill.into()) };
    }

    /// Crop an image to a defined rectangle.
    pub fn crop(&mut self, rectangle: Rectangle) {
        unsafe { raylib4_sys::ImageCrop(&mut self.0, rectangle.into()) };
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        Self(unsafe { raylib4_sys::ImageCopy(self.0) })
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { raylib4_sys::UnloadImage(self.0) };
    }
}

fn path_to_cstring<P: AsRef<Path>>(path: P) -> Option<CString> {
    path.as_ref().to_str().and_then(|p| CString::new(p).ok())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PixelFormat {
    UncompressedGrayscale = 1,
    UncompressedGrayAlpha = 2,
    UncompressedR5g6b5 = 3,
    UncompressedR8g8b8 = 4,
    UncompressedR5g5b5a1 = 5,
    UncompressedR4g4b4a4 = 6,
    UncompressedR8g8b8a8 = 7,
    UncompressedR32 = 8,
    UncompressedR32g32b32 = 9,
    UncompressedR32g32b32a32 = 10,
    CompressedDxt1Rgb = 11,
    CompressedDxt1Rgba = 12,
    CompressedDxt3Rgba = 13,
    CompressedDxt5Rgba = 14,
    CompressedEtc1Rgb = 15,
    CompressedEtc2Rgb = 16,
    CompressedEtc2EacRgba = 17,
    CompressedPvrtRgb = 18,
    CompressedPvrtRgba = 19,
    CompressedAstc4x4Rgba = 20,
    CompressedAstc8x8Rgba = 21,
}
