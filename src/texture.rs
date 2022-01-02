use crate::structs::Rectangle;
use crate::{Color, Position, Size};
use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::path::Path;

#[derive(Debug)]
pub struct RenderTexture(
    pub(crate) raylib4_sys::RenderTexture, // TODO
);

impl RenderTexture {
    /// Load texture for rendering (framebuffer).
    pub fn load(size: Size) -> Option<Self> {
        let texture =
            unsafe { raylib4_sys::LoadRenderTexture(size.width as c_int, size.height as c_int) };
        if texture.id == 0 {
            None
        } else {
            Some(Self(texture))
        }
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { raylib4_sys::UnloadRenderTexture(self.0) };
    }
}

#[derive(Debug)]
pub struct Texture(
    pub(crate) raylib4_sys::Texture, // TODO
);

impl Texture {
    pub(crate) fn to_raw(&self) -> raylib4_sys::Texture {
        self.0.clone()
    }

    /// Load texture from file into GPU memory (VRAM).
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path = path_to_cstring(path)?;
        let texture = unsafe { raylib4_sys::LoadTexture(path.as_ptr()) };
        if texture.id == 0 {
            None
        } else {
            Some(Self(texture))
        }
    }

    /// Load texture from image data.
    pub fn load_from_image(image: &Image) -> Option<Self> {
        let texture = unsafe { raylib4_sys::LoadTextureFromImage(image.0) };
        if texture.id == 0 {
            None
        } else {
            Some(Self(texture))
        }
    }

    /// Load cubemap from image, multiple image cubemap layouts supported.
    pub fn load_texture_cubemap(image: &Image, layout: CubemapLayout) -> Option<TextureCubemap> {
        let texture = unsafe { raylib4_sys::LoadTextureCubemap(image.0, layout as c_int) };
        if texture.id == 0 {
            None
        } else {
            Some(Self(texture))
        }
    }

    /// Update GPU texture rectangle with new data.
    pub fn update_rec(&mut self, rectangle: Rectangle, pixels: &[u8]) {
        unsafe {
            raylib4_sys::UpdateTextureRec(
                self.0,
                rectangle.into(),
                pixels.as_ptr() as *const c_void,
            )
        };
    }

    /// Generate GPU mipmaps for a texture.
    pub fn generate_mipmaps(&mut self) {
        unsafe { raylib4_sys::GenTextureMipmaps(&mut self.0) };
    }

    /// Set texture scaling filter mode.
    pub fn set_filter(&mut self, filter: TextureFilter) {
        unsafe { raylib4_sys::SetTextureFilter(self.0, filter as c_int) };
    }

    /// Set texture wrapping mode.
    pub fn set_wrap(&mut self, wrap: TextureWrap) {
        unsafe { raylib4_sys::SetTextureWrap(self.0, wrap as c_int) };
    }

    /// Draw a Texture2D.
    pub fn draw(&mut self, position: Position, tint: Color) {
        unsafe { raylib4_sys::DrawTextureV(self.0, position.into(), tint.into()) };
    }

    /// Draw a Texture2D with extended parameters.
    pub fn draw_ex(&mut self, position: Position, rotation: f32, scale: f32, tint: Color) {
        unsafe {
            raylib4_sys::DrawTextureEx(self.0, position.into(), rotation, scale, tint.into())
        };
    }

    /// Draw a part of a texture defined by a rectangle.
    pub fn draw_rec(&mut self, source: Rectangle, position: Position, tint: Color) {
        unsafe {
            raylib4_sys::DrawTextureRec(self.0, source.into(), position.into(), tint.into());
        }
    }

    /// Draw texture quad with tiling and offset parameters.
    pub fn draw_quad(&mut self, tiling: Size, offset: Position, quad: Rectangle, tint: Color) {
        unsafe {
            raylib4_sys::DrawTextureQuad(
                self.0,
                tiling.into(),
                offset.into(),
                quad.into(),
                tint.into(),
            );
        }
    }

    /// Draw part of a texture (defined by a rectangle) with rotation and scale tiled into dest.
    pub fn draw_tiled(
        &mut self,
        source: Rectangle,
        dest: Rectangle,
        origin: Position,
        rotation: f32,
        scale: f32,
        tint: Color,
    ) {
        unsafe {
            raylib4_sys::DrawTextureTiled(
                self.0,
                source.into(),
                dest.into(),
                origin.into(),
                rotation,
                scale,
                tint.into(),
            );
        }
    }

    /// Draw a part of a texture defined by a rectangle with 'pro' parameters.
    pub fn draw_pro(
        &mut self,
        source: Rectangle,
        dest: Rectangle,
        origin: Position,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            raylib4_sys::DrawTexturePro(
                self.0,
                source.into(),
                dest.into(),
                origin.into(),
                rotation,
                tint.into(),
            );
        }
    }

    /// Draws a texture (or part of it) that stretches or shrinks nicely.
    pub fn draw_n_patch(
        &mut self,
        info: NpatchInfo,
        dest: Rectangle,
        origin: Position,
        rotation: f32,
        tint: Color,
    ) {
        unsafe {
            raylib4_sys::DrawTextureNPatch(
                self.0,
                info.into(),
                dest.into(),
                origin.into(),
                rotation,
                tint.into(),
            );
        }
    }

    /// Draw a textured polygon.
    pub fn draw_poly(
        &mut self,
        centor: Position,
        points: &[Position],
        texcoord: &[Position],
        tint: Color,
    ) {
        let mut points = points
            .iter()
            .copied()
            .map(raylib4_sys::Vector2::from)
            .collect::<Vec<_>>();
        let mut texcoord = texcoord
            .iter()
            .copied()
            .map(raylib4_sys::Vector2::from)
            .collect::<Vec<_>>();
        unsafe {
            raylib4_sys::DrawTexturePoly(
                self.0,
                centor.into(),
                points.as_mut_ptr(),
                texcoord.as_mut_ptr(),
                points.len() as c_int,
                tint.into(),
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { raylib4_sys::UnloadTexture(self.0) };
    }
}

pub type TextureCubemap = Texture;

#[derive(Debug, Clone)]
pub struct NpatchInfo {
    pub source: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub layout: NpatchLayout,
}

impl From<NpatchInfo> for raylib4_sys::NPatchInfo {
    fn from(v: NpatchInfo) -> Self {
        Self {
            source: v.source.into(),
            left: v.left,
            top: v.top,
            right: v.right,
            bottom: v.bottom,
            layout: v.layout as c_int,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NpatchLayout {
    NinePatch = 0,
    ThreePatchVertical = 1,
    ThreePatchHorizontal = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CubemapLayout {
    AutoDetect = 0,
    LineVertial = 1,
    LineHorizontal = 2,
    CrossThreeByFour = 3,
    CrossFourByThree = 4,
    Panorama = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextureFilter {
    Point = 0,
    Bilinear = 1,
    Trilinear = 2,
    Anisotropic4x = 3,
    Anisotropic8x = 4,
    Anisotoropic16x = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TextureWrap {
    Repeat = 0,
    Clamp = 1,
    MirrorRepeat = 2,
    MirrorClamp = 3,
}

#[derive(Debug)]
pub struct Image(pub(crate) raylib4_sys::Image); // TODO

impl Image {
    pub fn size(&self) -> Size {
        (self.0.width as u32, self.0.height as u32).into()
    }

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

    /// Crop image depending on alpha value.
    pub fn alpha_crop(&mut self, threshold: f32) {
        unsafe { raylib4_sys::ImageAlphaCrop(&mut self.0, threshold) };
    }

    /// Clear alpha channel to desired color.
    pub fn alpha_clear(&mut self, color: Color, threshold: f32) {
        unsafe { raylib4_sys::ImageAlphaClear(&mut self.0, color.into(), threshold) };
    }

    /// Apply alpha mask to image.
    pub fn alpha_mask(&mut self, mask: &Self) {
        unsafe { raylib4_sys::ImageAlphaMask(&mut self.0, mask.0) };
    }

    /// Premultiply alpha channel.
    pub fn alpha_premultiply(&mut self) {
        unsafe { raylib4_sys::ImageAlphaPremultiply(&mut self.0) };
    }

    /// Resize image (Bicubic scaling algorithm).
    pub fn resize(&mut self, size: Size) {
        unsafe { raylib4_sys::ImageResize(&mut self.0, size.width as c_int, size.height as c_int) };
    }

    /// Resize image (Nearest-Neighbor scaling algorithm).
    pub fn resize_nn(&mut self, size: Size) {
        unsafe {
            raylib4_sys::ImageResizeNN(&mut self.0, size.width as c_int, size.height as c_int)
        };
    }

    /// Resize canvas and fill with color.
    pub fn resize_canvas(&mut self, size: Size, offset: Position, color: Color) {
        unsafe {
            raylib4_sys::ImageResizeCanvas(
                &mut self.0,
                size.width as c_int,
                size.height as c_int,
                offset.x as c_int,
                offset.y as c_int,
                color.into(),
            );
        }
    }

    /// Generate all mipmap levels for a provided image.
    pub fn mipmaps(&mut self) {
        unsafe { raylib4_sys::ImageMipmaps(&mut self.0) };
    }

    /// Dither image data to 16bpp or lower (Floyd-Steinberg dithering).
    // TODO: add Bpp struct
    pub fn dither(&mut self, r_bpp: u8, g_bpp: u8, b_bpp: u8, a_bpp: u8) {
        unsafe {
            raylib4_sys::ImageDither(
                &mut self.0,
                r_bpp as c_int,
                g_bpp as c_int,
                b_bpp as c_int,
                a_bpp as c_int,
            )
        };
    }

    /// Flip image vertically.
    pub fn flip_vertical(&mut self) {
        unsafe { raylib4_sys::ImageFlipVertical(&mut self.0) };
    }

    /// Flip image horizontally.
    pub fn flip_horizontal(&mut self) {
        unsafe { raylib4_sys::ImageFlipHorizontal(&mut self.0) };
    }

    /// Rotate image clockwise 90deg.
    pub fn rotate_cw(&mut self) {
        unsafe { raylib4_sys::ImageRotateCW(&mut self.0) };
    }

    /// Rotate image counter-clockwise 90deg.
    pub fn rotate_ccw(&mut self) {
        unsafe { raylib4_sys::ImageRotateCCW(&mut self.0) };
    }

    /// Modify image color: tint.
    pub fn color_tint(&mut self, color: Color) {
        unsafe { raylib4_sys::ImageColorTint(&mut self.0, color.into()) };
    }

    /// Modify image color: invert.
    pub fn color_invert(&mut self) {
        unsafe { raylib4_sys::ImageColorInvert(&mut self.0) };
    }

    /// Modify image color: grayscale.
    pub fn color_grayscale(&mut self) {
        unsafe { raylib4_sys::ImageColorGrayscale(&mut self.0) };
    }

    /// Modify image color: contrast (-100 to 100).
    pub fn color_contrast(&mut self, contrast: f32) {
        unsafe { raylib4_sys::ImageColorContrast(&mut self.0, contrast) };
    }

    /// Modify image color: brightness (-255 to 255).
    pub fn color_brightness(&mut self, brightness: i16) {
        unsafe { raylib4_sys::ImageColorBrightness(&mut self.0, brightness as c_int) };
    }

    /// Modify image color: replace color.
    pub fn color_replace(&mut self, old: Color, new: Color) {
        unsafe { raylib4_sys::ImageColorReplace(&mut self.0, old.into(), new.into()) };
    }

    /// Load color data from image as a Color array (RGBA - 32bit).
    pub fn load_colors(&self) -> Vec<Color> {
        let size = self.size();
        let n = size.width as usize * size.height as usize;
        let colors_ptr = unsafe { raylib4_sys::LoadImageColors(self.0) };
        let colors = (0..n)
            .map(|i| Color::from(unsafe { std::slice::from_raw_parts(colors_ptr, n) }[i]))
            .collect();
        unsafe { raylib4_sys::UnloadImageColors(colors_ptr) };
        colors
    }

    /// Load colors palette from image as a Color array (RGBA - 32bit).
    pub fn load_palette(&self, max_palette_size: usize) -> Vec<Color> {
        let mut colors_count = 0;
        let palette_ptr = unsafe {
            raylib4_sys::LoadImagePalette(self.0, max_palette_size as c_int, &mut colors_count)
        };
        let n = colors_count as usize;
        let palette = (0..n)
            .map(|i| Color::from(unsafe { std::slice::from_raw_parts(palette_ptr, n) }[i]))
            .collect();
        unsafe { raylib4_sys::UnloadImagePalette(palette_ptr) };
        palette
    }

    /// Get image alpha border rectangle.
    pub fn get_alpha_border(&self, threshold: f32) -> Rectangle {
        unsafe { raylib4_sys::GetImageAlphaBorder(self.0, threshold).into() }
    }

    /// Get image pixel color at (x, y) position.
    pub fn get_color(&self, position: Position) -> Color {
        unsafe { raylib4_sys::GetImageColor(self.0, position.x as c_int, position.y as c_int) }
            .into()
    }

    /// Clear image background with given color.
    pub fn clear_background(&mut self, color: Color) {
        unsafe { raylib4_sys::ImageClearBackground(&mut self.0, color.into()) };
    }

    /// Draw pixel within an image.
    pub fn draw_pixel(&mut self, position: Position, color: Color) {
        unsafe { raylib4_sys::ImageDrawPixelV(&mut self.0, position.into(), color.into()) };
    }

    /// Draw line within an image.
    pub fn draw_line(&mut self, start: Position, end: Position, color: Color) {
        unsafe { raylib4_sys::ImageDrawLineV(&mut self.0, start.into(), end.into(), color.into()) };
    }

    /// Draw circle within an image.
    pub fn draw_circle(&mut self, center: Position, radius: usize, color: Color) {
        unsafe {
            raylib4_sys::ImageDrawCircleV(&mut self.0, center.into(), radius as c_int, color.into())
        };
    }

    /// Draw rectangle within an image.
    pub fn draw_rectangle(&mut self, rectangle: Rectangle, color: Color) {
        unsafe { raylib4_sys::ImageDrawRectangleRec(&mut self.0, rectangle.into(), color.into()) };
    }

    /// Draw rectangle lines within an image.
    pub fn draw_rectangle_lines(&mut self, rectangle: Rectangle, thick: usize, color: Color) {
        unsafe {
            raylib4_sys::ImageDrawRectangleLines(
                &mut self.0,
                rectangle.into(),
                thick as c_int,
                color.into(),
            )
        };
    }

    /// Draw a source image within a destination image (tint applied to source).
    pub fn draw(&mut self, src: &Self, src_rec: Rectangle, dst_rec: Rectangle, tint: Color) {
        unsafe {
            raylib4_sys::ImageDraw(
                &mut self.0,
                src.0,
                src_rec.into(),
                dst_rec.into(),
                tint.into(),
            )
        };
    }

    /// Draw text (using default font) within an image (destination).
    pub fn draw_text(
        &mut self,
        text: &str,
        position: Position,
        font_size: usize,
        color: Color,
    ) -> Result<(), std::ffi::NulError> {
        let text = CString::new(text)?;
        unsafe {
            raylib4_sys::ImageDrawText(
                &mut self.0,
                text.as_ptr(),
                position.x as c_int,
                position.y as c_int,
                font_size as c_int,
                color.into(),
            );
        }
        Ok(())
    }

    // TODO
    // void ImageDrawTextEx(Image *dst, Font font, const char *text, Vector2 position, float fontSize, float spacing, Color tint); // Draw text (custom sprite font) within an image (destination)
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

impl PixelFormat {
    /// Get pixel data size in bytes for certain format.
    pub fn get_pixel_data_size(self, size: Size) -> usize {
        unsafe {
            raylib4_sys::GetPixelDataSize(size.width as c_int, size.height as c_int, self as c_int)
                as usize
        }
    }
}
