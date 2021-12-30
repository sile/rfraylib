#[derive(Debug)]
pub struct Cursor(pub(crate) ()); // TODO

impl Cursor {
    /// Shows cursor.
    pub fn show(&mut self) {
        unsafe { raylib4_sys::ShowCursor() };
    }

    /// Hides cursor.
    pub fn hide(&mut self) {
        unsafe { raylib4_sys::HideCursor() };
    }

    /// Check if cursor is not visible.
    pub fn is_hidden(&self) -> bool {
        unsafe { raylib4_sys::IsCursorHidden() }
    }

    /// Enables cursor (unlock cursor).
    pub fn enable(&mut self) {
        unsafe { raylib4_sys::EnableCursor() };
    }

    /// Disables cursor (lock cursor).
    pub fn disable(&mut self) {
        unsafe { raylib4_sys::DisableCursor() };
    }

    /// Check if cursor is on the screen.
    pub fn is_on_screen(&self) -> bool {
        unsafe { raylib4_sys::IsCursorOnScreen() }
    }
}
