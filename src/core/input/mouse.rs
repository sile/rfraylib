use crate::Position;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct Mouse(pub(crate) ());

impl Mouse {
    /// Check if a mouse button has been pressed once.
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { raylib4_sys::IsMouseButtonPressed(button as c_int) }
    }

    /// Check if a mouse button is being pressed.
    pub fn is_button_down(&self, button: MouseButton) -> bool {
        unsafe { raylib4_sys::IsMouseButtonDown(button as c_int) }
    }

    /// Check if a mouse button has been released once.
    pub fn is_button_released(&self, button: MouseButton) -> bool {
        unsafe { raylib4_sys::IsMouseButtonReleased(button as c_int) }
    }

    /// Check if a mouse button is NOT being pressed.
    pub fn is_button_up(&self, button: MouseButton) -> bool {
        unsafe { raylib4_sys::IsMouseButtonUp(button as c_int) }
    }

    /// Get mouse position XY.
    pub fn get_position(&self) -> Position {
        unsafe { raylib4_sys::GetMousePosition() }.into()
    }

    /// Get mouse delta between frames.
    pub fn get_delta(&self) -> Position {
        unsafe { raylib4_sys::GetMouseDelta() }.into()
    }

    /// Set mouse position XY.
    pub fn set_position(&mut self, position: Position) {
        unsafe { raylib4_sys::SetMousePosition(position.x as c_int, position.y as c_int) };
    }

    /// Set mouse offset.
    pub fn set_offset(&mut self, offset: Position) {
        unsafe { raylib4_sys::SetMouseOffset(offset.x as c_int, offset.y as c_int) };
    }

    /// Set mouse scaling.
    pub fn set_scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe { raylib4_sys::SetMouseScale(scale_x, scale_y) };
    }

    /// Get mouse wheel movement Y.
    pub fn get_wheel_move(&self) -> f32 {
        unsafe { raylib4_sys::GetMouseWheelMove() }
    }

    // TODO: move to `Cursor`
    /// Set mouse cursor.
    pub fn set_cursor(&mut self, cursor: MouseCursor) {
        unsafe { raylib4_sys::SetMouseCursor(cursor as c_int) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Side = 3,
    Extra = 4,
    Forward = 5,
    Back = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseCursor {
    Default = 0,
    Arrow = 1,
    Ibeam = 2,
    Crosshair = 3,
    PointingHand = 4,
    ResizeEw = 5,
    ResizeNs = 6,
    ResizeNwse = 7,
    ResizeNesw = 8,
    ResizeAll = 9,
    NotAllowed = 10,
}
