use crate::core::cursor::Cursor;
use crate::core::monitor::Monitors;
use crate::core::window::{ConfigFlag, Window};
use crate::structs::Size;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicBool, Ordering};

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

    pub fn build(&self) -> Result<System, SystemBuildError> {
        if IS_SYSTEM_INITIALIZED.swap(true, Ordering::SeqCst) {
            return Err(SystemBuildError::AlreadyInitialized);
        }
        if unsafe { raylib4_sys::IsWindowReady() } {
            return Err(SystemBuildError::AlreadyInitialized);
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

        Ok(System {
            window: Window(()),
            monitors: Monitors(()),
            cursor: Cursor(()),
        })
    }
}

impl Default for SystemBuilder {
    fn default() -> Self {
        Self {
            window_size: Self::DEFAULT_WINDOW_SISE,
            window_title: Self::DEFAULT_WINDOW_TITLE.to_owned(),
            config_flags: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct System {
    window: Window,
    monitors: Monitors,
    cursor: Cursor,
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
}
