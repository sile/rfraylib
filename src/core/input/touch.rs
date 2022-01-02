use crate::Position;
use std::os::raw::c_int;
use std::time::Duration;

#[derive(Debug)]
pub struct Touch(pub(crate) ());

impl Touch {
    /// Get touch position for touch point 0 (relative to screen size).
    pub fn get_position(&self) -> Position {
        let x = unsafe { raylib4_sys::GetTouchX() };
        let y = unsafe { raylib4_sys::GetTouchY() };
        Position { x, y }
    }

    pub fn get_touch_points(&self) -> impl Iterator<Item = TouchPoint> {
        let n = unsafe { raylib4_sys::GetTouchPointCount() };
        (0..n).map(|i| unsafe {
            TouchPoint {
                id: raylib4_sys::GetTouchPointId(i) as u32,
                position: raylib4_sys::GetTouchPosition(i).into(),
            }
        })
    }

    /// Enable a set of gestures using flags.
    pub fn set_gestures_enabled(&mut self, flags: impl Iterator<Item = Gesture>) {
        unsafe { raylib4_sys::SetGesturesEnabled(gestures_to_flags(flags)) };
    }

    /// Check if a gesture have been detected.
    pub fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        unsafe { raylib4_sys::IsGestureDetected(gesture as c_int) }
    }

    /// Get latest detected gesture.
    pub fn get_gesture_detected(&self) -> Gesture {
        let n = unsafe { raylib4_sys::GetGestureDetected() };
        match n {
            0 => Gesture::None,
            1 => Gesture::Tap,
            2 => Gesture::Doubletap,
            4 => Gesture::Hold,
            8 => Gesture::Drag,
            16 => Gesture::SwipeRight,
            32 => Gesture::SwipeLeft,
            64 => Gesture::SwipeUp,
            128 => Gesture::SwipeDown,
            256 => Gesture::PinchIn,
            512 => Gesture::PinchOut,
            _ => {
                log::warn!("unknown gesture (ignored): {}", n);
                Gesture::None
            }
        }
    }

    /// Get gesture hold time in milliseconds.
    pub fn get_gesture_hold_duration(&self) -> Duration {
        let n = unsafe { raylib4_sys::GetGestureHoldDuration() };
        Duration::from_secs_f32(n * 1000.0)
    }

    /// Get drag vector and angle (between initial touch point to current).
    ///
    /// NOTE: Angle in degrees, horizontal-right is 0, counterclock-wise.
    pub fn get_gesture_drag_distance(&self) -> Distance {
        let angle = unsafe { raylib4_sys::GetGestureDragAngle() };
        let vector = unsafe { raylib4_sys::GetGestureDragVector() };
        Distance {
            angle,
            x: vector.x,
            y: vector.y,
        }
    }

    /// Get vector and angle beween two pinch points.
    ///
    /// NOTE: Angle in degrees, horizontal-right is 0, counterclock-wise
    pub fn get_gesture_pinch_distance(&self) -> Distance {
        let angle = unsafe { raylib4_sys::GetGesturePinchAngle() };
        let vector = unsafe { raylib4_sys::GetGesturePinchVector() };
        Distance {
            angle,
            x: vector.x,
            y: vector.y,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Distance {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub id: u32,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gesture {
    None = 0,
    Tap = 1,
    Doubletap = 2,
    Hold = 4,
    Drag = 8,
    SwipeRight = 16,
    SwipeLeft = 32,
    SwipeUp = 64,
    SwipeDown = 128,
    PinchIn = 256,
    PinchOut = 512,
}

fn gestures_to_flags(xs: impl Iterator<Item = Gesture>) -> u32 {
    let mut flags = 0;
    for x in xs {
        flags |= x as u32;
    }
    flags
}
