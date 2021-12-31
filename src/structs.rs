use std::os::raw::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f32, // TODO: change type
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

impl Position {
    /// Check if point is inside rectangle.
    pub fn check_collision_point_rec(self, rec: Rectangle) -> bool {
        unsafe { raylib4_sys::CheckCollisionPointRec(self.into(), rec.into()) }
    }

    /// Check if point is inside circle.
    pub fn check_collision_point_circle(self, circle: Circle) -> bool {
        unsafe {
            raylib4_sys::CheckCollisionPointCircle(self.into(), circle.center.into(), circle.radius)
        }
    }

    /// Check if point is inside a triangle.
    pub fn check_collision_point_triangle(self, triangle: Triangle) -> bool {
        unsafe {
            raylib4_sys::CheckCollisionPointTriangle(
                self.into(),
                triangle.0.into(),
                triangle.1.into(),
                triangle.2.into(),
            )
        }
    }

    /// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold].
    pub fn check_collision_point_line(self, line: Line, threshold: usize) -> bool {
        unsafe {
            raylib4_sys::CheckCollisionPointLine(
                self.into(),
                line.start.into(),
                line.end.into(),
                threshold as c_int,
            )
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub start: Position,
    pub end: Position,
}

impl Line {
    /// Check the collision between two lines defined by two points each, returns collision point by reference.
    pub fn check_collision_lines(self, other: Line) -> Option<Position> {
        unsafe {
            let mut collision_point = raylib4_sys::Vector2 { x: 0.0, y: 0.0 };
            let b = raylib4_sys::CheckCollisionLines(
                self.start.into(),
                self.end.into(),
                other.start.into(),
                other.end.into(),
                &mut collision_point,
            );
            if b {
                Some(collision_point.into())
            } else {
                None
            }
        }
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

impl From<Size> for raylib4_sys::Vector2 {
    fn from(v: Size) -> Self {
        Self {
            x: v.width,
            y: v.height,
        }
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

impl Rectangle {
    /// Check collision between two rectangles.
    pub fn check_collision_recs(&self, other: &Self) -> bool {
        unsafe { raylib4_sys::CheckCollisionRecs((*self).into(), (*other).into()) }
    }

    /// Get collision rectangle for two rectangles collision.
    pub fn get_collision_rec(self, other: Self) -> Self {
        let v = unsafe { raylib4_sys::GetCollisionRec(self.into(), other.into()) };
        Self {
            x: v.x,
            y: v.y,
            width: v.width,
            height: v.height,
        }
    }
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

impl From<raylib4_sys::Rectangle> for Rectangle {
    fn from(
        raylib4_sys::Rectangle {
            x,
            y,
            width,
            height,
        }: raylib4_sys::Rectangle,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle {
    pub center: Position,
    pub radius: f32,
}

impl Circle {
    /// Check collision between two circles.
    pub fn check_collision_circles(&self, other: &Self) -> bool {
        unsafe {
            raylib4_sys::CheckCollisionCircles(
                self.center.into(),
                self.radius,
                other.center.into(),
                other.radius,
            )
        }
    }

    /// Check collision between circle and rectangle.
    pub fn check_collision_circle_rec(&self, rec: Rectangle) -> bool {
        unsafe { raylib4_sys::CheckCollisionCircleRec(self.center.into(), self.radius, rec.into()) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Triangle(pub Position, pub Position, pub Position);
