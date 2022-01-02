use crate::structs::{Position, Size};
use std::ffi::CStr;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct Monitors(pub(crate) ());

impl Monitors {
    pub fn len(&self) -> usize {
        unsafe { raylib4_sys::GetMonitorCount() as usize }
    }

    pub fn iter(&self) -> impl Iterator<Item = Monitor> {
        (0..self.len() as c_int).map(|index| Monitor {
            index,
            _monitors: self,
        })
    }

    /// Get current connected monitor.
    pub fn get_current(&self) -> Monitor {
        let index = unsafe { raylib4_sys::GetCurrentMonitor() };
        Monitor {
            index,
            _monitors: self,
        }
    }

    /// Set monitor for the current window (fullscreen mode).
    pub fn set_as_current(&mut self, monitor_no: MonitorNo) -> bool {
        unsafe { raylib4_sys::SetWindowMonitor(monitor_no.get() as c_int) };
        self.get_current().index as usize == monitor_no.get()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonitorNo(usize);

impl MonitorNo {
    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct Monitor<'a> {
    index: c_int,
    _monitors: &'a Monitors,
}

impl<'a> Monitor<'a> {
    pub fn number(&self) -> MonitorNo {
        MonitorNo(self.index as usize)
    }

    /// Get monitor position.
    pub fn get_position(&self) -> Position {
        unsafe { raylib4_sys::GetMonitorPosition(self.index) }.into()
    }

    /// Get monitor size (max available by monitor).
    pub fn get_size(&self) -> Size {
        let width = unsafe { raylib4_sys::GetMonitorWidth(self.index) } as i32;
        let height = unsafe { raylib4_sys::GetMonitorHeight(self.index) } as i32;
        Size { width, height }
    }

    /// Get monitor physical size in millimetres.
    pub fn get_physical_size(&self) -> Size {
        let width = unsafe { raylib4_sys::GetMonitorPhysicalWidth(self.index) } as i32;
        let height = unsafe { raylib4_sys::GetMonitorPhysicalHeight(self.index) } as i32;
        Size { width, height }
    }

    /// Get monitor refresh rate.
    pub fn get_refresh_rate(&self) -> u16 {
        unsafe { raylib4_sys::GetMonitorRefreshRate(self.index) as u16 }
    }

    /// Get the human-readable, UTF-8 encoded name of the primary monitor.
    pub fn get_name(&self) -> &str {
        unsafe { CStr::from_ptr(raylib4_sys::GetMonitorName(self.index)) }
            .to_str()
            .expect("unreachable")
    }
}
