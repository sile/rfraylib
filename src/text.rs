use crate::structs::Rectangle;
use crate::texture::Image;
use crate::Color;
use std::ffi::CString;
use std::os::raw::c_int;
use std::path::Path;

#[derive(Debug)]
pub struct Font(raylib4_sys::Font);

impl Font {
    /// Load font from file into GPU memory (VRAM).
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        path_to_cstring(path)
            .map(|path| unsafe { raylib4_sys::LoadFont(path.as_ptr()) })
            .map(Self)
            .unwrap_or_else(Self::default)
    }

    /// Load font from file with extended parameters.
    pub fn load_ex<P: AsRef<Path>>(path: P, font_size: usize, font_chars: &[char]) -> Self {
        let mut chars = font_chars
            .iter()
            .copied()
            .map(u32::from)
            .collect::<Vec<_>>();
        path_to_cstring(path)
            .map(|path| unsafe {
                raylib4_sys::LoadFontEx(
                    path.as_ptr(),
                    font_size as c_int,
                    chars.as_mut_ptr() as *mut i32,
                    chars.len() as c_int,
                )
            })
            .map(Self)
            .unwrap_or_else(Self::default)
    }

    /// Load font from Image (XNA style).
    pub fn load_from_image(image: &Image, key: Color, first_char: usize) -> Self {
        Self(unsafe { raylib4_sys::LoadFontFromImage(image.0, key.into(), first_char as c_int) })
    }

    /// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
    pub fn load_from_memory(
        file_type: &str,
        file_data: &[u8],
        font_size: usize,
        font_chars: &[char],
    ) -> Self {
        (|| {
            let file_type = CString::new(file_type).ok()?;
            let chars = font_chars
                .iter()
                .copied()
                .map(u32::from)
                .collect::<Vec<_>>();
            let font = unsafe {
                raylib4_sys::LoadFontFromMemory(
                    file_type.as_ptr(),
                    file_data.as_ptr(),
                    file_data.len() as c_int,
                    font_size as c_int,
                    chars.as_ptr() as *mut c_int,
                    chars.len() as c_int,
                )
            };
            Some(Self(font))
        })()
        .unwrap_or_else(Self::default)
    }

    pub fn is_default(&self) -> bool {
        self.0.texture.id == Self::default().0.texture.id
    }
}

impl Default for Font {
    fn default() -> Self {
        Self(unsafe { raylib4_sys::GetFontDefault() })
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { raylib4_sys::UnloadFont(self.0) };
    }
}

#[derive(Debug)]
pub struct FondData(Vec<raylib4_sys::GlyphInfo>);

impl FondData {
    /// Load font data for further use.
    pub fn load(
        file_data: &[u8],
        font_size: usize,
        font_chars: &[char],
        font_type: FontType,
    ) -> Option<Self> {
        let chars = font_chars
            .iter()
            .copied()
            .map(u32::from)
            .collect::<Vec<_>>();
        let font_data = unsafe {
            raylib4_sys::LoadFontData(
                file_data.as_ptr(),
                file_data.len() as c_int,
                font_size as c_int,
                chars.as_ptr() as *mut i32,
                chars.len() as c_int,
                font_type as c_int,
            )
        };
        if font_data == std::ptr::null_mut() {
            None
        } else {
            Some(Self(
                (0..font_chars.len())
                    .map(|i| unsafe {
                        &*std::ptr::slice_from_raw_parts(font_data, font_chars.len())
                     }[i])
                    .collect(),
            ))
        }
    }

    /// Generate image font atlas using chars info.
    pub fn generate_font_atlas(
        &self,
        font_size: usize,
        padding: usize,
        pack_method: PackMethod,
    ) -> (Image, Vec<Rectangle>) {
        let mut recs: *mut raylib4_sys::Rectangle = std::ptr::null_mut();
        let image = unsafe {
            raylib4_sys::GenImageFontAtlas(
                self.0.as_ptr(),
                &mut recs,
                self.0.len() as c_int,
                font_size as c_int,
                padding as c_int,
                pack_method as c_int,
            )
        };
        let rectangles = (0..self.0.len())
            .map(|i| unsafe { &*std::ptr::slice_from_raw_parts(recs, self.0.len()) }[i])
            .map(Rectangle::from)
            .collect();
        (Image(image), rectangles)
    }
}

impl Drop for FondData {
    fn drop(&mut self) {
        unsafe { raylib4_sys::UnloadFontData(self.0.as_mut_ptr(), self.0.len() as c_int) };
    }
}

fn path_to_cstring<P: AsRef<Path>>(path: P) -> Option<CString> {
    path.as_ref().to_str().and_then(|p| CString::new(p).ok())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontType {
    Default = 0,
    Bitmap = 1,
    Sdf = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PackMethod {
    Default = 0,
    Skyline = 1,
}
