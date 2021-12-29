//! Window-related functions.
use crate::structs::Vector2;
use std::ffi::CStr;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WindowInitError {
    #[error("there are an already initialized window")]
    AlreadyInitialized,
}

bitflags::bitflags! {
    pub struct ConfigFlags: raylib4_sys::ConfigFlags {
        const FULLSCREEN_MODE = 2;
        const WINDOW_RESIZABLE = 4;
        const WINDOW_UNDECORATED = 8;
        const WINDOW_TRANSPARENT = 16;
        const MSAA_4X_HINT = 32;
        const VSYNC_HINT = 64;
        const WINDOW_HIDDEN = 128;
        const WINDOW_ALWAYS_RUN = 256;
        const WINDOW_MINIMIZED = 512;
        const WINDOW_MAXIMIZED = 1024;
        const WINDOW_UNFOCUSED = 2048;
        const WINDOW_TOPMOST = 4096;
        const WINDOW_HIGHDPI = 8192;
        const INTERLACED_HINT = 65536;
    }
}

#[derive(Debug)]
pub struct Window(());

impl Window {
    /// Initialize window and OpenGL context.
    pub fn new(width: u16, height: u16, title: &CStr) -> Result<Self, WindowInitError> {
        if Self::is_ready() {
            return Err(WindowInitError::AlreadyInitialized);
        }

        unsafe {
            raylib4_sys::InitWindow(width.into(), height.into(), title.as_ptr());
        }
        Ok(Self(()))
    }

    /// Initialize window and OpenGL context with configuration flags.
    pub fn with_config_flags(
        width: u16,
        height: u16,
        title: &CStr,
        flags: ConfigFlags,
    ) -> Result<Self, WindowInitError> {
        unsafe {
            raylib4_sys::SetConfigFlags(flags.bits());
        }
        Self::new(width, height, title)
    }

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
    ///
    /// If `flag` contains flags other than one, this method returns `None`.
    pub fn is_state(&self, flag: ConfigFlags) -> Option<bool> {
        if flag.bits().count_ones() != 1 {
            None
        } else {
            Some(unsafe { raylib4_sys::IsWindowState(flag.bits()) })
        }
    }

    /// Set window configuration state using flags.
    pub fn set_state(&mut self, flags: ConfigFlags) {
        unsafe { raylib4_sys::SetWindowState(flags.bits()) };
    }

    /// Clear window configuration state flags.
    pub fn clear_state(&mut self, flags: ConfigFlags) {
        unsafe { raylib4_sys::ClearWindowState(flags.bits()) };
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
    pub fn minimized(&mut self) {
        unsafe { raylib4_sys::MinimizeWindow() }
    }

    /// Set window state: not minimized/maximized (only PLATFORM_DESKTOP).
    pub fn restore(&mut self) {
        unsafe { raylib4_sys::RestoreWindow() };
    }

    // TODO
    //   void SetWindowIcon(Image image);                                        // Set icon for window (only PLATFORM_DESKTOP)

    /// Set title for window (only PLATFORM_DESKTOP).
    pub fn set_title(&mut self, title: &CStr) {
        unsafe { raylib4_sys::SetWindowTitle(title.as_ptr()) };
    }

    /// Set window position on screen (only PLATFORM_DESKTOP).
    pub fn set_position(&mut self, x: u16, y: u16) {
        unsafe { raylib4_sys::SetWindowPosition(x.into(), y.into()) };
    }

    /// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE).
    pub fn set_min_size(&mut self, width: u16, height: u16) {
        unsafe { raylib4_sys::SetWindowMinSize(width.into(), height.into()) };
    }

    /// Set window dimensions.
    pub fn set_size(&mut self, width: u16, height: u16) {
        unsafe { raylib4_sys::SetWindowSize(width.into(), height.into()) };
    }

    /// Get current screen width.
    pub fn get_screen_width(&self) -> u16 {
        saturating_int_to_u16(unsafe { raylib4_sys::GetScreenWidth() })
    }

    /// Get current screen height.
    pub fn get_screen_height(&self) -> u16 {
        saturating_int_to_u16(unsafe { raylib4_sys::GetScreenHeight() })
    }

    /// Set monitor for the current window (fullscreen mode).
    pub fn set_monitor(&mut self, monitor_index: u16) {
        unsafe { raylib4_sys::SetWindowMonitor(monitor_index.into()) };
    }

    /// Get number of connected monitors.
    pub fn get_monitors(&self) -> Vec<Monitor> {
        let n = saturating_int_to_u16(unsafe { raylib4_sys::GetMonitorCount() });
        (0..n)
            .map(|index| Monitor {
                index,
                window: self,
            })
            .collect()
    }

    /// Get current connected monitor.
    pub fn get_current_monitor(&self) -> Monitor {
        let n = saturating_int_to_u16(unsafe { raylib4_sys::GetCurrentMonitor() });
        Monitor {
            index: n,
            window: self,
        }
    }

    /// Get window position XY on monitor.
    pub fn get_position(&self) -> Vector2 {
        unsafe { raylib4_sys::GetWindowPosition() }.into()
    }

    /// Get window scale DPI factor.
    pub fn get_scale_dpi(&self) -> Vector2 {
        unsafe { raylib4_sys::GetWindowScaleDPI() }.into()
    }

    /// Set clipboard text content.
    pub fn set_clipboard_text(&self, text: &CStr) {
        unsafe { raylib4_sys::SetClipboardText(text.as_ptr()) };
    }

    /// Get clipboard text content.
    pub fn get_clipboard_text(&self) -> &CStr {
        unsafe { CStr::from_ptr(raylib4_sys::GetClipboardText()) }
    }

    /// Check if window has been initialized successfully.
    pub fn is_ready() -> bool {
        unsafe { raylib4_sys::IsWindowReady() }
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
pub struct Monitor<'a> {
    index: u16,

    #[allow(dead_code)]
    window: &'a Window,
}

impl<'a> Monitor<'a> {
    pub const fn index(&self) -> u16 {
        self.index
    }

    /// Get monitor position.
    pub fn get_position(&self) -> Vector2 {
        unsafe { raylib4_sys::GetMonitorPosition(self.index.into()) }.into()
    }

    /// Get monitor width (max available by monitor).
    pub fn get_width(&self) -> u16 {
        saturating_int_to_u16(unsafe { raylib4_sys::GetMonitorWidth(self.index.into()) })
    }

    /// Get monitor height (max available by monitor).
    pub fn get_height(&self) -> u16 {
        saturating_int_to_u16(unsafe { raylib4_sys::GetMonitorHeight(self.index.into()) })
    }

    /// Get monitor physical width in millimetres.
    pub fn get_physical_width(&self) -> u16 {
        saturating_int_to_u16(unsafe { raylib4_sys::GetMonitorPhysicalWidth(self.index.into()) })
    }

    /// Get monitor physical height in millimetres.
    pub fn get_physical_height(&self) -> u16 {
        saturating_int_to_u16(unsafe { raylib4_sys::GetMonitorPhysicalHeight(self.index.into()) })
    }

    /// Get monitor refresh rate.
    pub fn get_refresh_rate(&self) -> u16 {
        saturating_int_to_u16(unsafe { raylib4_sys::GetMonitorRefreshRate(self.index.into()) })
    }

    /// Get the human-readable, UTF-8 encoded name of the primary monitor.
    pub fn get_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(raylib4_sys::GetMonitorName(self.index.into())) }
    }
}

fn saturating_int_to_u16(n: std::os::raw::c_int) -> u16 {
    std::cmp::min(std::cmp::max(0, n), u16::MAX.into()) as u16
}
