use std::collections::BTreeSet;
use std::os::raw::c_int;

#[derive(Debug)]
pub struct Gamepad {
    pub(crate) index: u32,
}

impl Gamepad {
    pub(crate) fn is_available(&self) -> bool {
        unsafe { raylib4_sys::IsGamepadAvailable(self.index as c_int) }
    }

    /// Get gamepad internal name id.
    pub fn get_name(&self) -> &str {
        let n =
            unsafe { std::ffi::CStr::from_ptr(raylib4_sys::GetGamepadName(self.index as c_int)) };
        n.to_str().expect("unreachable")
    }

    /// Check if a gamepad button has been pressed once.
    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        unsafe { raylib4_sys::IsGamepadButtonPressed(self.index as c_int, button as c_int) }
    }

    /// Check if a gamepad button is being pressed.
    pub fn is_button_down(&self, button: GamepadButton) -> bool {
        unsafe { raylib4_sys::IsGamepadButtonDown(self.index as c_int, button as c_int) }
    }

    /// Check if a gamepad button has been released once.
    pub fn is_button_released(&self, button: GamepadButton) -> bool {
        unsafe { raylib4_sys::IsGamepadButtonReleased(self.index as c_int, button as c_int) }
    }

    /// Check if a gamepad button is NOT being pressed.
    pub fn is_button_up(&self, button: GamepadButton) -> bool {
        unsafe { raylib4_sys::IsGamepadButtonUp(self.index as c_int, button as c_int) }
    }

    /// Get gamepad axis count for a gamepad.
    pub fn get_axises(&self) -> BTreeSet<GamepadAxis> {
        let n = unsafe { raylib4_sys::GetGamepadAxisCount(self.index as c_int) as u32 };
        (0..n).filter_map(GamepadAxis::from_u32).collect()
    }

    /// Get axis movement value for a gamepad axis.
    pub fn get_axis_movement(&self, axis: GamepadAxis) -> f32 {
        unsafe { raylib4_sys::GetGamepadAxisMovement(self.index as c_int, axis as c_int) }
    }

    pub(crate) fn get_button_pressed() -> Option<GamepadButton> {
        let n = unsafe { raylib4_sys::GetGamepadButtonPressed() };
        if n == -1 {
            return None;
        }
        let b = match n {
            1 => GamepadButton::LeftFaceUp,
            2 => GamepadButton::LeftFaceRight,
            3 => GamepadButton::LeftFaceDown,
            4 => GamepadButton::LeftFaceLeft,
            5 => GamepadButton::RightFaceUp,
            6 => GamepadButton::RightFaceRight,
            7 => GamepadButton::RightFaceDown,
            8 => GamepadButton::RightFaceLeft,
            9 => GamepadButton::LeftTrigger1,
            10 => GamepadButton::LeftTrigger2,
            11 => GamepadButton::RightTrigger1,
            12 => GamepadButton::RightTrigger2,
            13 => GamepadButton::MiddleLeft,
            14 => GamepadButton::Middle,
            15 => GamepadButton::MiddleRight,
            16 => GamepadButton::LeftThumb,
            17 => GamepadButton::RightThumb,
            _ => GamepadButton::Unknown,
        };
        Some(b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum GamepadButton {
    Unknown = 0,
    LeftFaceUp = 1,
    LeftFaceRight = 2,
    LeftFaceDown = 3,
    LeftFaceLeft = 4,
    RightFaceUp = 5,
    RightFaceRight = 6,
    RightFaceDown = 7,
    RightFaceLeft = 8,
    LeftTrigger1 = 9,
    LeftTrigger2 = 10,
    RightTrigger1 = 11,
    RightTrigger2 = 12,
    MiddleLeft = 13,
    Middle = 14,
    MiddleRight = 15,
    LeftThumb = 16,
    RightThumb = 17,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum GamepadAxis {
    /// Gamepad left stick X axis.
    LeftX = 0,

    /// Gamepad left stick Y axis.
    LeftY = 1,

    /// Gamepad right stick X axis.
    RightX = 2,

    /// Gamepad right stick Y axis.
    RightY = 3,

    /// Gamepad back trigger left, pressure level: [1..-1].
    LeftTrigger = 4,

    /// Gamepad back trigger right, pressure level: [1..-1].
    RightTrigger = 5,
}

impl GamepadAxis {
    fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Self::LeftX),
            1 => Some(Self::LeftY),
            2 => Some(Self::RightX),
            3 => Some(Self::RightY),
            4 => Some(Self::LeftTrigger),
            5 => Some(Self::RightTrigger),
            _ => None,
        }
    }
}
