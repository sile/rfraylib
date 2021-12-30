use crate::core::cursor::Cursor;
use crate::core::drawing::{RenderTexture, TextureCanvas, WindowCanvas};
use crate::core::input::gamepad::{Gamepad, GamepadButton};
use crate::core::input::Keyboard;
use crate::core::monitor::Monitors;
use crate::core::window::{ConfigFlag, Window};
use crate::structs::Size;
use std::collections::BTreeSet;
use std::os::raw::{c_char, c_int};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

static IS_SYSTEM_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SystemBuildError {
    #[error("there is an already initialized raylib system.")]
    AlreadyInitialized,

    #[error("malformed title")]
    MalformedTitle {
        #[from]
        source: std::ffi::NulError,
    },
}

#[derive(Debug)]
pub struct SystemBuilder {
    window_size: Size,
    window_title: String,
    config_flags: BTreeSet<ConfigFlag>,
    target_fps: Option<usize>,
}

impl SystemBuilder {
    pub const DEFAULT_WINDOW_SISE: Size = Size {
        width: 800.0,
        height: 600.0,
    };
    pub const DEFAULT_WINDOW_TITLE: &'static str = "";

    pub fn new() -> Self {
        Self::default()
    }

    pub fn window_size(&mut self, size: Size) -> &mut Self {
        self.window_size = size;
        self
    }

    pub fn window_title(&mut self, title: &str) -> &mut Self {
        self.window_title = title.to_owned();
        self
    }

    pub fn config_flags(&mut self, flags: impl Iterator<Item = ConfigFlag>) -> &mut Self {
        self.config_flags = flags.collect();
        self
    }

    pub fn target_fps(&mut self, fps: usize) -> &mut Self {
        self.target_fps = Some(fps);
        self
    }

    pub fn build(&self) -> Result<System, SystemBuildError> {
        if IS_SYSTEM_INITIALIZED.swap(true, Ordering::SeqCst) {
            return Err(SystemBuildError::AlreadyInitialized);
        }
        if unsafe { raylib4_sys::IsWindowReady() } {
            return Err(SystemBuildError::AlreadyInitialized);
        }

        unsafe {
            raylib4_sys::SetTraceLogLevel(raylib4_sys::TraceLogLevel_LOG_ALL as c_int);
            raylib4_sys::SetTraceLogCallback(Some(trace_log_callback));
        }

        // Initialize window.
        unsafe {
            let flags = ConfigFlag::flags_to_bits(self.config_flags.iter().copied());
            raylib4_sys::SetConfigFlags(flags);
        }
        unsafe {
            let title = std::ffi::CString::new(self.window_title.as_bytes())?;
            raylib4_sys::InitWindow(
                self.window_size.width as std::os::raw::c_int,
                self.window_size.height as std::os::raw::c_int,
                title.as_ptr(),
            );
        }

        const MAX_GAMEPADS: u32 = 8; // TODO: Make configurable.
        let mut system = System {
            window: Window(()),
            monitors: Monitors(()),
            cursor: Cursor(()),
            keyboard: Keyboard(()),
            gamepads: (0..MAX_GAMEPADS).map(|index| Gamepad { index }).collect(),
        };

        if let Some(x) = self.target_fps {
            system.set_target_fps(x);
        }

        Ok(system)
    }
}

impl Default for SystemBuilder {
    fn default() -> Self {
        Self {
            window_size: Self::DEFAULT_WINDOW_SISE,
            window_title: Self::DEFAULT_WINDOW_TITLE.to_owned(),
            config_flags: Default::default(),
            target_fps: None,
        }
    }
}

#[derive(Debug)]
pub struct System {
    window: Window,
    monitors: Monitors,
    cursor: Cursor,
    keyboard: Keyboard,
    gamepads: Vec<Gamepad>,
}

impl System {
    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }

    pub fn monitors(&self) -> &Monitors {
        &self.monitors
    }

    pub fn monitors_mut(&mut self) -> &mut Monitors {
        &mut self.monitors
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    pub fn cursor_mut(&mut self) -> &mut Cursor {
        &mut self.cursor
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn keyboard_mut(&mut self) -> &mut Keyboard {
        &mut self.keyboard
    }

    pub fn gamepads(&self) -> impl Iterator<Item = &Gamepad> {
        self.gamepads.iter().filter(|x| x.is_available())
    }

    pub fn gamepads_mut(&mut self) -> impl Iterator<Item = &mut Gamepad> {
        self.gamepads.iter_mut().filter(|x| x.is_available())
    }

    /// Get the last gamepad button pressed.
    pub fn get_gamepad_button_pressed(&self) -> Option<GamepadButton> {
        Gamepad::get_button_pressed()
    }

    /// Set internal gamepad mappings (SDL_GameControllerDB).
    pub fn set_mappings(&mut self, mappings: &str) -> Result<(), std::ffi::NulError> {
        let mappings = std::ffi::CString::new(mappings)?;
        unsafe { raylib4_sys::SetGamepadMappings(mappings.as_ptr()) };
        Ok(())
    }

    /// Setup canvas (framebuffer) to start drawing.
    ///
    /// On drop: End canvas drawing and swap buffers (double buffering).
    pub fn next_frame(&mut self) -> WindowCanvas {
        WindowCanvas::new(self)
    }

    pub fn create_texture_canvas<'a, 'b>(
        &'a mut self,
        target: &'b mut RenderTexture,
    ) -> TextureCanvas<'a, 'b, Self> {
        TextureCanvas::new(self, target)
    }

    /// Set target FPS (maximum).
    pub fn set_target_fps(&mut self, fps: usize) {
        unsafe { raylib4_sys::SetTargetFPS(fps as c_int) };
    }

    /// Get current FPS.
    pub fn get_fps(&self) -> usize {
        unsafe { raylib4_sys::GetFPS() as usize }
    }

    /// Get time in seconds for last frame drawn (delta time).
    pub fn get_frame_time(&self) -> Duration {
        let seconds = unsafe { raylib4_sys::GetFrameTime() };
        Duration::from_secs_f32(seconds)
    }

    /// Get elapsed time in seconds since InitWindow().
    pub fn get_time(&self) -> Duration {
        let seconds = unsafe { raylib4_sys::GetTime() };
        Duration::from_secs_f64(seconds)
    }

    /// Takes a screenshot of current screen (filename extension defines format).
    pub fn take_screenshot(&self, path: &str) -> Result<(), std::ffi::NulError> {
        let path = std::ffi::CString::new(path)?;
        unsafe { raylib4_sys::TakeScreenshot(path.as_ptr()) };
        Ok(())
    }

    /// Open URL with default system browser (if available).
    pub fn open_url(&self, url: &str) -> Result<(), std::ffi::NulError> {
        let url = std::ffi::CString::new(url)?;
        unsafe { raylib4_sys::OpenURL(url.as_ptr()) };
        Ok(())
    }
}

extern "C" fn trace_log_callback(
    log_level: c_int,
    text: *const c_char,
    args: *mut raylib4_sys::__va_list_tag,
) {
    let filter = match log::max_level() {
        log::LevelFilter::Off => raylib4_sys::TraceLogLevel_LOG_NONE,
        log::LevelFilter::Error => raylib4_sys::TraceLogLevel_LOG_ERROR,
        log::LevelFilter::Warn => raylib4_sys::TraceLogLevel_LOG_WARNING,
        log::LevelFilter::Info => raylib4_sys::TraceLogLevel_LOG_INFO,
        log::LevelFilter::Debug => raylib4_sys::TraceLogLevel_LOG_DEBUG,
        log::LevelFilter::Trace => raylib4_sys::TraceLogLevel_LOG_TRACE,
    };
    if log_level < filter as c_int {
        return;
    }

    let result = unsafe { vsprintf::vsprintf(text, args) };
    match result {
        Err(e) => {
            log::warn!("`vsprintf()` failed: {}", e);
        }
        Ok(s) => {
            if log_level >= raylib4_sys::TraceLogLevel_LOG_ERROR as c_int {
                log::error!("{}", s);
            } else if log_level >= raylib4_sys::TraceLogLevel_LOG_WARNING as c_int {
                log::warn!("{}", s);
            } else if log_level >= raylib4_sys::TraceLogLevel_LOG_INFO as c_int {
                log::info!("{}", s);
            } else if log_level >= raylib4_sys::TraceLogLevel_LOG_DEBUG as c_int {
                log::debug!("{}", s);
            } else {
                log::trace!("{}", s);
            }
        }
    }
}
