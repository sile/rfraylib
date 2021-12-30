use crate::{Position, Size};
use std::os::raw::c_int;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const LIGHTGRAY: Color = Color::rgb(200, 200, 200);
    pub const GRAY: Color = Color::rgb(130, 130, 130);
    pub const DARKGRAY: Color = Color::rgb(80, 80, 80);
    pub const YELLOW: Color = Color::rgb(253, 249, 0);
    pub const GOLD: Color = Color::rgb(255, 203, 0);
    pub const ORANGE: Color = Color::rgb(255, 161, 0);
    pub const PINK: Color = Color::rgb(255, 109, 194);
    pub const RED: Color = Color::rgb(230, 41, 55);
    pub const MAROON: Color = Color::rgb(190, 33, 55);
    pub const GREEN: Color = Color::rgb(0, 228, 48);
    pub const LIME: Color = Color::rgb(0, 158, 47);
    pub const DARKGREEN: Color = Color::rgb(0, 117, 44);
    pub const SKYBLUE: Color = Color::rgb(102, 191, 255);
    pub const BLUE: Color = Color::rgb(0, 121, 241);
    pub const DARKBLUE: Color = Color::rgb(0, 82, 172);
    pub const PURPLE: Color = Color::rgb(200, 122, 255);
    pub const VIOLET: Color = Color::rgb(135, 60, 190);
    pub const DARKPURPLE: Color = Color::rgb(112, 31, 126);
    pub const BEIGE: Color = Color::rgb(211, 176, 131);
    pub const BROWN: Color = Color::rgb(127, 106, 79);
    pub const DARKBROWN: Color = Color::rgb(76, 63, 47);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const BLANK: Color = Color::rgba(0, 0, 0, 0);
    pub const MAGENTA: Color = Color::rgb(255, 0, 255);
    pub const RAYWHITE: Color = Color::rgb(245, 245, 245);

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for raylib4_sys::Color {
    fn from(v: Color) -> Self {
        Self {
            r: v.r,
            g: v.g,
            b: v.b,
            a: v.a,
        }
    }
}

pub trait Draw {
    /// Set background color (framebuffer clear color).
    fn clear_background(&mut self, color: Color) {
        unsafe { raylib4_sys::ClearBackground(color.into()) };
    }

    /// Begin 2D mode with custom camera (2D).
    fn with_camera(&mut self, camera: Camera) -> WithCamera<Self>
    where
        Self: Sized,
    {
        WithCamera::new(self, camera)
    }

    /// Begin blending mode (alpha, additive, multiplied, subtract, custom)
    ///
    /// On drop: End blending mode (reset to default: alpha blending)
    fn begin_blend_mode(&mut self, mode: BlendMode) -> BlendModeCanvas<Self>
    where
        Self: Sized,
    {
        BlendModeCanvas::new(self, mode)
    }

    /// Begin scissor mode (define screen area for following drawing).
    fn begin_scissor_mode(&mut self, position: Position, size: Size) -> ScissorModeCanvas<Self>
    where
        Self: Sized,
    {
        ScissorModeCanvas::new(self, position, size)
    }

    fn create_texture_canvas<'a, 'b>(
        &'a mut self,
        target: &'b mut RenderTexture,
    ) -> TextureCanvas<'a, 'b, Self>
    where
        Self: Sized,
    {
        TextureCanvas::new(self, target)
    }
}

// CustomCameraCanvas
#[derive(Debug)]
pub struct WithCamera<'a, T> {
    #[allow(dead_code)]
    canvas: &'a T,
}

impl<'a, T> WithCamera<'a, T> {
    fn new(canvas: &'a T, camera: Camera) -> Self {
        unsafe { raylib4_sys::BeginMode2D(camera.into()) };
        Self { canvas }
    }
}

impl<'a, T: Draw> Draw for WithCamera<'a, T> {}

impl<'a, T> Drop for WithCamera<'a, T> {
    fn drop(&mut self) {
        unsafe { raylib4_sys::EndMode2D() };
    }
}

#[derive(Debug)]
pub struct WindowCanvas<'a> {
    #[allow(dead_code)]
    system: &'a crate::System,
}

impl<'a> WindowCanvas<'a> {
    pub(crate) fn new(system: &'a crate::System) -> Self {
        unsafe { raylib4_sys::BeginDrawing() };
        Self { system }
    }
}

impl<'a> Draw for WindowCanvas<'a> {}

impl<'a> Drop for WindowCanvas<'a> {
    fn drop(&mut self) {
        unsafe { raylib4_sys::EndDrawing() };
    }
}

#[derive(Debug, Clone)]
pub struct Camera {
    pub offset: Position,
    pub target: Position,
    pub rotation: f32,
    pub zoom: f32,
}

impl Camera {
    /// Get the screen space position for a 2d camera world space position.
    pub fn world_to_screen(&self, position: Position) -> Position {
        unsafe { raylib4_sys::GetWorldToScreen2D(position.into(), self.clone().into()) }.into()
    }

    /// Get the world space position for a 2d camera screen space position.
    pub fn screen_to_world(&self, position: Position) -> Position {
        unsafe { raylib4_sys::GetScreenToWorld2D(position.into(), self.clone().into()) }.into()
    }
}

impl From<Camera> for raylib4_sys::Camera2D {
    fn from(x: Camera) -> Self {
        Self {
            offset: x.offset.into(),
            target: x.target.into(),
            rotation: x.rotation,
            zoom: x.zoom,
        }
    }
}

#[derive(Debug)]
pub struct TextureCanvas<'a, 'b, T> {
    #[allow(dead_code)]
    parent: &'a T,
    #[allow(dead_code)]
    target: &'b RenderTexture,
}

impl<'a, 'b, T> TextureCanvas<'a, 'b, T> {
    pub(crate) fn new(parent: &'a T, target: &'b RenderTexture) -> Self {
        unsafe { raylib4_sys::BeginTextureMode(target.0) };
        Self { parent, target }
    }
}

impl<'a, 'b, T> Draw for TextureCanvas<'a, 'b, T> {}

impl<'a, 'b, T> Drop for TextureCanvas<'a, 'b, T> {
    fn drop(&mut self) {
        unsafe { raylib4_sys::EndTextureMode() };
    }
}

#[derive(Debug)]
pub struct RenderTexture(raylib4_sys::RenderTexture);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BlendMode {
    Alpha,
    Additive,
    Multiplied,
    AddColors,
    SubtractColors,
    Custom,
}

impl Default for BlendMode {
    fn default() -> Self {
        Self::Alpha
    }
}

impl BlendMode {
    fn to_raw_value(self) -> raylib4_sys::BlendMode {
        match self {
            Self::Alpha => 0,
            Self::Additive => 1,
            Self::Multiplied => 2,
            Self::AddColors => 3,
            Self::SubtractColors => 4,
            Self::Custom => 5,
        }
    }
}

#[derive(Debug)]
pub struct BlendModeCanvas<'a, T> {
    #[allow(dead_code)]
    parent: &'a T,
}

impl<'a, T> BlendModeCanvas<'a, T> {
    pub(crate) fn new(parent: &'a T, mode: BlendMode) -> Self {
        unsafe { raylib4_sys::BeginBlendMode(mode.to_raw_value() as c_int) };
        Self { parent }
    }
}

impl<'a, T: Draw> Draw for BlendModeCanvas<'a, T> {}

impl<'a, T> Drop for BlendModeCanvas<'a, T> {
    fn drop(&mut self) {
        unsafe { raylib4_sys::EndBlendMode() };
    }
}

#[derive(Debug)]
pub struct ScissorModeCanvas<'a, T> {
    #[allow(dead_code)]
    parent: &'a T,
}

impl<'a, T> ScissorModeCanvas<'a, T> {
    pub(crate) fn new(parent: &'a T, position: Position, size: Size) -> Self {
        unsafe {
            raylib4_sys::BeginScissorMode(
                position.x as c_int,
                position.y as c_int,
                size.width as c_int,
                size.height as c_int,
            )
        };
        Self { parent }
    }
}

impl<'a, T: Draw> Draw for ScissorModeCanvas<'a, T> {}

impl<'a, T> Drop for ScissorModeCanvas<'a, T> {
    fn drop(&mut self) {
        unsafe { raylib4_sys::EndScissorMode() };
    }
}
