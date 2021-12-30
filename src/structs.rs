#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Position {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl From<raylib4_sys::Vector2> for Position {
    fn from(v: raylib4_sys::Vector2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Position> for raylib4_sys::Vector2 {
    fn from(v: Position) -> Self {
        Self { x: v.x, y: v.y }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl From<(f32, f32)> for Size {
    fn from((width, height): (f32, f32)) -> Self {
        Self { width, height }
    }
}

/// Vector2 type.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2(raylib4_sys::Vector2);

impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self(raylib4_sys::Vector2 { x, y })
    }

    pub const fn x(self) -> f32 {
        self.0.x
    }

    pub const fn y(self) -> f32 {
        self.0.y
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.0.x
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.0.y
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        Self::new(x, y)
    }
}

impl From<raylib4_sys::Vector2> for Vector2 {
    fn from(x: raylib4_sys::Vector2) -> Self {
        Self(x)
    }
}

impl From<Vector2> for raylib4_sys::Vector2 {
    fn from(x: Vector2) -> Self {
        x.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl From<Rectangle> for raylib4_sys::Rectangle {
    fn from(
        Rectangle {
            x,
            y,
            width,
            height,
        }: Rectangle,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
