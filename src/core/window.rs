//! Window-related functions.
use crate::structs::{Position, Size};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub struct ScaleDpiFactor {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum ConfigFlag {
    FullscreenMode,
    WindowResizable,
    WindowUndecorated,
    WindowTransparent,
    Msaa4xHint,
    VsyncHint,
    WindowHidden,
    WindowAlwaysRun,
    WindowMinimized,
    WindowMaximized,
    WindowUnfocused,
    WindowTopmost,
    WindowHighdpi,
    InterlacedHint,
}

impl ConfigFlag {
    fn to_int(self) -> raylib4_sys::ConfigFlags {
        match self {
            Self::FullscreenMode => raylib4_sys::ConfigFlags_FLAG_FULLSCREEN_MODE,
            Self::WindowResizable => raylib4_sys::ConfigFlags_FLAG_WINDOW_RESIZABLE,
            Self::WindowUndecorated => raylib4_sys::ConfigFlags_FLAG_WINDOW_UNDECORATED,
            Self::WindowTransparent => raylib4_sys::ConfigFlags_FLAG_WINDOW_TRANSPARENT,
            Self::Msaa4xHint => raylib4_sys::ConfigFlags_FLAG_MSAA_4X_HINT,
            Self::VsyncHint => raylib4_sys::ConfigFlags_FLAG_VSYNC_HINT,
            Self::WindowHidden => raylib4_sys::ConfigFlags_FLAG_WINDOW_HIDDEN,
            Self::WindowAlwaysRun => raylib4_sys::ConfigFlags_FLAG_WINDOW_ALWAYS_RUN,
            Self::WindowMinimized => raylib4_sys::ConfigFlags_FLAG_WINDOW_MINIMIZED,
            Self::WindowMaximized => raylib4_sys::ConfigFlags_FLAG_WINDOW_MAXIMIZED,
            Self::WindowUnfocused => raylib4_sys::ConfigFlags_FLAG_WINDOW_UNFOCUSED,
            Self::WindowTopmost => raylib4_sys::ConfigFlags_FLAG_WINDOW_TOPMOST,
            Self::WindowHighdpi => raylib4_sys::ConfigFlags_FLAG_WINDOW_HIGHDPI,
            Self::InterlacedHint => raylib4_sys::ConfigFlags_FLAG_INTERLACED_HINT,
        }
    }

    pub(crate) fn flags_to_bits(flags: impl Iterator<Item = Self>) -> raylib4_sys::ConfigFlags {
        let mut bits = 0;
        for f in flags {
            bits |= f.to_int();
        }
        bits
    }
}

#[derive(Debug)]
pub struct Window(pub(crate) ()); // TODO: remove pub

impl Window {
    /// Check if KEY_ESCAPE pressed or Close icon pressed.
    pub fn should_close(&self) -> bool {
        unsafe { raylib4_sys::WindowShouldClose() }
    }

    /// Check if window is currently fullscreen.
    pub fn is_fullscreen(&self) -> bool {
        unsafe { raylib4_sys::IsWindowFullscreen() }
    }

    /// Check if window is currently hidden (only PLATFORM_DESKTOP).
    pub fn is_hidden(&self) -> bool {
        unsafe { raylib4_sys::IsWindowHidden() }
    }

    /// Check if window is currently minimized (only PLATFORM_DESKTOP).
    pub fn is_minimized(&self) -> bool {
        unsafe { raylib4_sys::IsWindowMinimized() }
    }

    /// Check if window is currently maximized (only PLATFORM_DESKTOP).
    pub fn is_maximized(&self) -> bool {
        unsafe { raylib4_sys::IsWindowMaximized() }
    }

    /// Check if window is currently focused (only PLATFORM_DESKTOP).
    pub fn is_focused(&self) -> bool {
        unsafe { raylib4_sys::IsWindowFocused() }
    }

    /// Check if window has been resized last frame.
    pub fn is_resized(&self) -> bool {
        unsafe { raylib4_sys::IsWindowResized() }
    }

    /// Check if one specific window flag is enabled.
    pub fn is_state(&self, flag: ConfigFlag) -> bool {
        unsafe { raylib4_sys::IsWindowState(flag.to_int()) }
    }

    /// Set window configuration state using flags.
    pub fn set_state(&mut self, flags: impl Iterator<Item = ConfigFlag>) {
        unsafe { raylib4_sys::SetWindowState(ConfigFlag::flags_to_bits(flags)) };
    }

    /// Clear window configuration state flags.
    pub fn clear_state(&mut self, flags: impl Iterator<Item = ConfigFlag>) {
        unsafe { raylib4_sys::ClearWindowState(ConfigFlag::flags_to_bits(flags)) };
    }

    /// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP).
    pub fn toggle_fullscreen(&mut self) {
        unsafe { raylib4_sys::ToggleFullscreen() };
    }

    /// Set window state: maximized, if resizable (only PLATFORM_DESKTOP).
    pub fn maximize(&mut self) {
        unsafe { raylib4_sys::MaximizeWindow() };
    }

    /// Set window state: minimized, if resizable (only PLATFORM_DESKTOP).
    pub fn minimize(&mut self) {
        unsafe { raylib4_sys::MinimizeWindow() }
    }

    /// Set window state: not minimized/maximized (only PLATFORM_DESKTOP).
    pub fn restore(&mut self) {
        unsafe { raylib4_sys::RestoreWindow() };
    }

    // TODO
    //   void SetWindowIcon(Image image);                                        // Set icon for window (only PLATFORM_DESKTOP)

    /// Set title for window (only PLATFORM_DESKTOP).
    pub fn set_title(&mut self, title: &str) -> Result<(), std::ffi::NulError> {
        let title = std::ffi::CString::new(title)?;
        unsafe { raylib4_sys::SetWindowTitle(title.as_ptr()) };
        Ok(())
    }

    /// Set window position on screen (only PLATFORM_DESKTOP).
    pub fn set_position(&mut self, x: u16, y: u16) {
        unsafe { raylib4_sys::SetWindowPosition(x.into(), y.into()) };
    }

    /// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE).
    pub fn set_min_size(&mut self, size: Size) {
        unsafe { raylib4_sys::SetWindowMinSize(size.width as c_int, size.height as c_int) };
    }

    /// Set window dimensions.
    pub fn set_size(&mut self, size: Size) {
        unsafe { raylib4_sys::SetWindowSize(size.width as c_int, size.height as c_int) };
    }

    /// Get current screen size.
    pub fn get_screen_size(&self) -> Size {
        let width = unsafe { raylib4_sys::GetScreenWidth() } as i32;
        let height = unsafe { raylib4_sys::GetScreenHeight() } as i32;
        Size { width, height }
    }

    /// Get window position XY on monitor.
    pub fn get_position(&self) -> Position {
        unsafe { raylib4_sys::GetWindowPosition() }.into()
    }

    /// Get window scale DPI factor.
    pub fn get_scale_dpi(&self) -> ScaleDpiFactor {
        let v = unsafe { raylib4_sys::GetWindowScaleDPI() };
        ScaleDpiFactor { x: v.x, y: v.y }
    }

    /// Set clipboard text content.
    pub fn set_clipboard_text(&mut self, text: &str) -> Result<(), std::ffi::NulError> {
        let text = std::ffi::CString::new(text)?;
        unsafe { raylib4_sys::SetClipboardText(text.as_ptr()) };
        Ok(())
    }

    /// Get clipboard text content.
    pub fn get_clipboard_text(&self) -> Result<&str, std::str::Utf8Error> {
        unsafe { CStr::from_ptr(raylib4_sys::GetClipboardText()) }.to_str()
    }

    /// Check if a file has been dropped into window.
    pub fn is_file_dropped(&self) -> bool {
        unsafe { raylib4_sys::IsFileDropped() }
    }

    /// Get dropped files names (memory should be freed).
    pub fn get_dropped_files(&self) -> DroppedFiles {
        DroppedFiles::new(self)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            raylib4_sys::CloseWindow();
        }
    }
}

#[derive(Debug)]
pub struct DroppedFiles<'a> {
    i: usize,
    count: usize,
    files: *mut *mut c_char,

    #[allow(dead_code)]
    window: &'a Window,
}

impl<'a> DroppedFiles<'a> {
    fn new(window: &'a Window) -> Self {
        let mut count = 0;
        let files = unsafe { raylib4_sys::GetDroppedFiles(&mut count) };
        Self {
            i: 0,
            count: count as usize,
            files,
            window,
        }
    }
}

impl<'a> Iterator for DroppedFiles<'a> {
    type Item = Result<PathBuf, std::str::Utf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.count {
            return None;
        };
        let file = unsafe { &*std::ptr::slice_from_raw_parts(self.files, self.count) }[self.i];
        self.i += 1;
        Some(unsafe { CStr::from_ptr(file) }.to_str().map(PathBuf::from))
    }
}

impl<'a> Drop for DroppedFiles<'a> {
    fn drop(&mut self) {
        unsafe { raylib4_sys::ClearDroppedFiles() };
    }
}
