use crate::structs::Rectangle;
use crate::text::Font;
use crate::texture::{PixelFormat, RenderTexture};
use crate::{Position, Size};
use std::ffi::CString;
use std::os::raw::{c_int, c_void};

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

    /// Returns color with alpha applied, alpha goes from 0.0f to 1.0f.
    pub fn fade(self, alpha: f32) -> Self {
        unsafe { raylib4_sys::Fade(self.into(), alpha) }.into()
    }

    /// Returns hexadecimal value for a Color.
    pub fn to_int(self) -> u32 {
        unsafe { raylib4_sys::ColorToInt(self.into()) as u32 }
    }

    /// Returns Color normalized as float [0..1].
    pub fn normalize(self) -> (f32, f32, f32, f32) {
        let v = unsafe { raylib4_sys::ColorNormalize(self.into()) };
        (v.x, v.y, v.z, v.w)
    }

    /// Returns Color from normalized values [0..1].
    pub fn from_normalized((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        unsafe { raylib4_sys::ColorFromNormalized(raylib4_sys::Vector4 { x, y, z, w }).into() }
    }

    /// Returns HSV values for a Color, hue [0..360], saturation/value [0..1].
    pub fn to_hsv(self) -> (f32, f32, f32) {
        let v = unsafe { raylib4_sys::ColorToHSV(self.into()) };
        (v.x, v.y, v.z)
    }

    /// Returns a Color from HSV values, hue [0..360], saturation/value [0..1].
    pub fn from_hsv(hue: f32, saturation: f32, value: f32) -> Self {
        unsafe { raylib4_sys::ColorFromHSV(hue, saturation, value).into() }
    }

    /// Returns color with alpha applied, alpha goes from 0.0f to 1.0f.
    pub fn alpha(self, alpha: f32) -> Self {
        unsafe { raylib4_sys::ColorAlpha(self.into(), alpha).into() }
    }

    /// Returns src alpha-blended into dst color with tint.
    pub fn alpha_blend(self, src: Self, tint: Self) -> Self {
        unsafe { raylib4_sys::ColorAlphaBlend(self.into(), src.into(), tint.into()).into() }
    }

    /// Get Color structure from hexadecimal value.
    pub fn from_hexadecimal(hex_value: u32) -> Self {
        unsafe { raylib4_sys::GetColor(hex_value).into() }
    }

    /// Get Color from a source pixel pointer of certain format.
    pub fn get_pixel_color(src: &[u8], format: PixelFormat) -> Self {
        // TODO: check size
        unsafe { raylib4_sys::GetPixelColor(src.as_ptr() as *mut c_void, format as c_int).into() }
    }

    /// Set color formatted into destination pixel pointer.
    pub fn set_pixel_color(self, dst: &mut [u8], format: PixelFormat) {
        // TODO: check size
        unsafe {
            raylib4_sys::SetPixelColor(
                dst.as_mut_ptr() as *mut c_void,
                self.into(),
                format as c_int,
            )
        };
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

impl From<raylib4_sys::Color> for Color {
    fn from(v: raylib4_sys::Color) -> Self {
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

    /// Draw a pixel.
    fn draw_pixel(&mut self, position: Position, color: Color) {
        unsafe { raylib4_sys::DrawPixelV(position.into(), color.into()) };
    }

    /// Draw a line.
    fn draw_line(&mut self, start: Position, end: Position, color: Color) {
        unsafe { raylib4_sys::DrawLineV(start.into(), end.into(), color.into()) };
    }

    /// Draw a line defining thickness.
    fn draw_line_ex(&mut self, start: Position, end: Position, thick: f32, color: Color) {
        unsafe { raylib4_sys::DrawLineEx(start.into(), end.into(), thick, color.into()) };
    }

    /// Draw a line using cubic-bezier curves in-out.
    fn draw_line_bezier(&mut self, start: Position, end: Position, thick: f32, color: Color) {
        unsafe { raylib4_sys::DrawLineBezier(start.into(), end.into(), thick, color.into()) };
    }

    /// Draw line using quadratic bezier curves with a control point.
    fn draw_line_bezier_quad(
        &mut self,
        start: Position,
        end: Position,
        control: Position,
        thick: f32,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawLineBezierQuad(
                start.into(),
                end.into(),
                control.into(),
                thick,
                color.into(),
            )
        };
    }

    /// Draw line using cubic bezier curves with 2 control points.
    fn draw_line_bezier_cubic(
        &mut self,
        start: Position,
        end: Position,
        start_control: Position,
        end_control: Position,
        thick: f32,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawLineBezierCubic(
                start.into(),
                end.into(),
                start_control.into(),
                end_control.into(),
                thick,
                color.into(),
            )
        };
    }

    /// Draw lines sequence.
    fn draw_line_strip(&mut self, points: impl Iterator<Item = Position>, color: Color) {
        let mut points = points.map(raylib4_sys::Vector2::from).collect::<Vec<_>>();
        unsafe {
            raylib4_sys::DrawLineStrip(points.as_mut_ptr(), points.len() as c_int, color.into())
        };
    }

    /// Draw a color-filled circle.
    fn draw_circle(&mut self, center: Position, radius: f32, color: Color) {
        unsafe { raylib4_sys::DrawCircleV(center.into(), radius, color.into()) };
    }

    /// Draw a piece of a circle.
    fn draw_circle_sector(
        &mut self,
        center: Position,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: usize,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawCircleSector(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments as c_int,
                color.into(),
            );
        }
    }

    /// Draw circle sector outline.
    fn draw_circle_sector_lines(
        &mut self,
        center: Position,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: usize,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawCircleSectorLines(
                center.into(),
                radius,
                start_angle,
                end_angle,
                segments as c_int,
                color.into(),
            );
        }
    }

    /// Draw a gradient-filled circle.
    fn draw_circle_gradient(
        &mut self,
        center: Position,
        radius: f32,
        color1: Color,
        color2: Color,
    ) {
        unsafe {
            raylib4_sys::DrawCircleGradient(
                center.x as c_int,
                center.y as c_int,
                radius,
                color1.into(),
                color2.into(),
            );
        }
    }

    /// Draw circle outline.
    fn draw_circle_lines(&mut self, center: Position, radius: f32, color: Color) {
        unsafe {
            raylib4_sys::DrawCircleLines(center.x as c_int, center.y as c_int, radius, color.into())
        };
    }

    /// Draw ellipse.
    fn draw_ellipse(&mut self, center: Position, radius_h: f32, radius_v: f32, color: Color) {
        unsafe {
            raylib4_sys::DrawEllipse(
                center.x as c_int,
                center.y as c_int,
                radius_h,
                radius_v,
                color.into(),
            )
        };
    }

    /// Draw ellipse outline.
    fn draw_ellipse_lines(&mut self, center: Position, radius_h: f32, radius_v: f32, color: Color) {
        unsafe {
            raylib4_sys::DrawEllipseLines(
                center.x as c_int,
                center.y as c_int,
                radius_h,
                radius_v,
                color.into(),
            )
        };
    }

    /// Draw ring.
    fn draw_ring(
        &mut self,
        center: Position,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: usize,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawRing(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments as c_int,
                color.into(),
            );
        }
    }

    /// Draw ring outline.
    fn draw_ring_lines(
        &mut self,
        center: Position,
        inner_radius: f32,
        outer_radius: f32,
        start_angle: f32,
        end_angle: f32,
        segments: usize,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawRingLines(
                center.into(),
                inner_radius,
                outer_radius,
                start_angle,
                end_angle,
                segments as c_int,
                color.into(),
            );
        }
    }

    /// Draw a color-filled rectangle.
    fn draw_rectangle(&mut self, rectangle: Rectangle, color: Color) {
        unsafe { raylib4_sys::DrawRectangleRec(rectangle.into(), color.into()) };
    }

    /// Draw a color-filled rectangle with pro parameters.
    fn draw_rectangle_pro(
        &mut self,
        rectangle: Rectangle,
        origin: Position,
        rotation: f32,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawRectanglePro(rectangle.into(), origin.into(), rotation, color.into());
        }
    }

    /// Draw a vertical-gradient-filled rectangle.
    fn draw_rectangle_gradient_v(&mut self, rectangle: Rectangle, color1: Color, color2: Color) {
        unsafe {
            raylib4_sys::DrawRectangleGradientV(
                rectangle.position.x,
                rectangle.position.y,
                rectangle.size.width as c_int,
                rectangle.size.height as c_int,
                color1.into(),
                color2.into(),
            );
        }
    }

    /// Draw a horizontal-gradient-filled rectangle.
    fn draw_rectangle_gradient_h(&mut self, rectangle: Rectangle, color1: Color, color2: Color) {
        unsafe {
            raylib4_sys::DrawRectangleGradientH(
                rectangle.position.x,
                rectangle.position.y,
                rectangle.size.width as c_int,
                rectangle.size.height as c_int,
                color1.into(),
                color2.into(),
            );
        }
    }

    /// Draw a gradient-filled rectangle with custom vertex colors.
    fn draw_rectangle_gradient_ex(
        &mut self,
        rectangle: Rectangle,
        color1: Color,
        color2: Color,
        color3: Color,
        color4: Color,
    ) {
        unsafe {
            raylib4_sys::DrawRectangleGradientEx(
                rectangle.into(),
                color1.into(),
                color2.into(),
                color3.into(),
                color4.into(),
            );
        }
    }

    /// Draw rectangle outline.
    fn draw_rectangle_lines(&mut self, rectangle: Rectangle, color: Color) {
        unsafe {
            raylib4_sys::DrawRectangleLines(
                rectangle.position.x,
                rectangle.position.y,
                rectangle.size.width as c_int,
                rectangle.size.height as c_int,
                color.into(),
            );
        }
    }

    /// Draw rectangle outline with extended parameters.
    fn draw_rectangle_lines_ex(&mut self, rectangle: Rectangle, line_thick: f32, color: Color) {
        unsafe {
            raylib4_sys::DrawRectangleLinesEx(rectangle.into(), line_thick, color.into());
        }
    }

    /// Draw rectangle with rounded edges.
    fn draw_rectangle_rounded(
        &mut self,
        rectangle: Rectangle,
        roundness: f32,
        segments: usize,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawRectangleRounded(
                rectangle.into(),
                roundness,
                segments as c_int,
                color.into(),
            );
        }
    }

    /// Draw rectangle with rounded edges outline.
    fn draw_rectangle_rounded_lines(
        &mut self,
        rectangle: Rectangle,
        roundness: f32,
        segments: usize,
        line_thick: f32,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawRectangleRoundedLines(
                rectangle.into(),
                roundness,
                segments as c_int,
                line_thick,
                color.into(),
            );
        }
    }

    /// Draw a color-filled triangle (vertex in counter-clockwise order!).
    fn draw_triangle(&mut self, v1: Position, v2: Position, v3: Position, color: Color) {
        unsafe {
            raylib4_sys::DrawTriangle(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draw triangle outline (vertex in counter-clockwise order!).
    fn draw_triangle_lines(&mut self, v1: Position, v2: Position, v3: Position, color: Color) {
        unsafe {
            raylib4_sys::DrawTriangleLines(v1.into(), v2.into(), v3.into(), color.into());
        }
    }

    /// Draw a triangle fan defined by points (first vertex is the center).
    fn draw_triangle_fan(&mut self, points: impl Iterator<Item = Position>, color: Color) {
        let mut points = points.map(raylib4_sys::Vector2::from).collect::<Vec<_>>();
        unsafe {
            raylib4_sys::DrawTriangleFan(points.as_mut_ptr(), points.len() as c_int, color.into());
        }
    }

    /// Draw a triangle strip defined by points.
    fn draw_triangle_strip(&mut self, points: impl Iterator<Item = Position>, color: Color) {
        let mut points = points.map(raylib4_sys::Vector2::from).collect::<Vec<_>>();
        unsafe {
            raylib4_sys::DrawTriangleStrip(
                points.as_mut_ptr(),
                points.len() as c_int,
                color.into(),
            );
        }
    }

    /// Draw a regular polygon (Vector version).
    fn draw_poly(
        &mut self,
        center: Position,
        sides: usize,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawPoly(
                center.into(),
                sides as c_int,
                radius,
                rotation,
                color.into(),
            );
        }
    }

    /// Draw a polygon outline of n sides.
    fn draw_poly_lines(
        &mut self,
        center: Position,
        sides: usize,
        radius: f32,
        rotation: f32,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawPolyLines(
                center.into(),
                sides as c_int,
                radius,
                rotation,
                color.into(),
            );
        }
    }

    /// Draw a polygon outline of n sides with extended parameters.
    fn draw_poly_lines_ex(
        &mut self,
        center: Position,
        sides: usize,
        radius: f32,
        rotation: f32,
        line_thick: f32,
        color: Color,
    ) {
        unsafe {
            raylib4_sys::DrawPolyLinesEx(
                center.into(),
                sides as c_int,
                radius,
                rotation,
                line_thick,
                color.into(),
            );
        }
    }

    /// Draw current FPS.
    fn draw_fps(&mut self, position: Position) {
        unsafe { raylib4_sys::DrawFPS(position.x as c_int, position.y as c_int) };
    }

    /// Draw text (using default font).
    fn draw_text(
        &mut self,
        text: &str,
        position: Position,
        font_size: usize,
        color: Color,
    ) -> Result<(), std::ffi::NulError> {
        let text = CString::new(text)?;
        unsafe {
            raylib4_sys::DrawText(
                text.as_ptr(),
                position.x as c_int,
                position.y as c_int,
                font_size as c_int,
                color.into(),
            );
        }
        Ok(())
    }

    /// Draw text using font and additional parameters.
    fn draw_text_ex(
        &mut self,
        font: &Font,
        text: &str,
        position: Position,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Result<(), std::ffi::NulError> {
        let text = CString::new(text)?;
        unsafe {
            raylib4_sys::DrawTextEx(
                font.0,
                text.as_ptr(),
                position.into(),
                font_size,
                spacing,
                tint.into(),
            );
        }
        Ok(())
    }

    /// Draw text using Font and pro parameters (rotation).
    fn draw_text_pro(
        &mut self,
        font: &Font,
        text: &str,
        position: Position,
        origin: Position,
        rotation: f32,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Result<(), std::ffi::NulError> {
        let text = CString::new(text)?;
        unsafe {
            raylib4_sys::DrawTextPro(
                font.0,
                text.as_ptr(),
                position.into(),
                origin.into(),
                rotation,
                font_size,
                spacing,
                tint.into(),
            );
        }
        Ok(())
    }

    /// Draw one character (codepoint).
    fn draw_char(&mut self, font: &Font, c: char, position: Position, font_size: f32, tint: Color) {
        unsafe {
            raylib4_sys::DrawTextCodepoint(
                font.0,
                u32::from(c) as c_int,
                position.into(),
                font_size,
                tint.into(),
            );
        }
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

#[derive(Debug, Copy, Clone)]
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
