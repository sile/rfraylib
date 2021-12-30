use std::os::raw::c_int;

#[derive(Debug)]
pub struct Keyboard(pub(crate) ());

impl Keyboard {
    /// Check if a key has been pressed once.
    pub fn is_key_pressed(&self, key: Key) -> bool {
        unsafe { raylib4_sys::IsKeyPressed(key as c_int) }
    }

    /// Check if a key is being pressed.
    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { raylib4_sys::IsKeyDown(key as c_int) }
    }

    /// Check if a key has been released once.
    pub fn is_key_released(&self, key: Key) -> bool {
        unsafe { raylib4_sys::IsKeyReleased(key as c_int) }
    }

    /// Check if a key is NOT being pressed.
    pub fn is_key_up(&self, key: Key) -> bool {
        unsafe { raylib4_sys::IsKeyUp(key as c_int) }
    }

    /// Set a custom key to exit program (default is ESC).
    pub fn set_exit_key(&mut self, key: Key) {
        unsafe { raylib4_sys::SetExitKey(key as c_int) };
    }

    /// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty.
    pub fn take_pressed_chars(&mut self) -> PressedChars {
        PressedChars { keyboard: self }
    }

    /// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty.
    pub fn take_pressed_keys(&mut self) -> PressedKeys {
        PressedKeys { keyboard: self }
    }
}

#[derive(Debug)]
pub struct PressedChars<'a> {
    #[allow(dead_code)]
    keyboard: &'a Keyboard,
}

impl<'a> Iterator for PressedChars<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match unsafe { raylib4_sys::GetCharPressed() } {
                0 => return None,
                c => {
                    if let Some(c) = char::from_u32(c as u32) {
                        return Some(c);
                    } else {
                        log::warn!("unknown unicode char (ignored): {}", c);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PressedKeys<'a> {
    #[allow(dead_code)]
    keyboard: &'a Keyboard,
}

impl<'a> Iterator for PressedKeys<'a> {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match unsafe { raylib4_sys::GetKeyPressed() } {
                0 => return None,
                c => {
                    if let Some(c) = Key::from_u32(c as u32) {
                        return Some(c);
                    } else {
                        log::warn!("unknown unicode key (ignored): {}", c);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Key {
    Back = 4,
    VolumeUp = 24,
    VolumeDown = 25,
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    Zero = 48,
    One = 49,
    Two = 50,
    Three = 51,
    Four = 52,
    Five = 53,
    Six = 54,
    Seven = 55,
    Eight = 56,
    Nine = 57,
    Semicolon = 59,
    Equal = 60,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    Grave = 96,
    Escape = 256,
    Enter = 257,
    Tab = 258,
    Backspace = 259,
    Insert = 260,
    Delete = 261,
    Right = 262,
    Left = 263,
    Down = 264,
    Up = 265,
    PageUp = 266,
    PageDown = 267,
    Home = 268,
    End = 269,
    CapsLock = 280,
    ScrollLock = 281,
    NumLock = 282,
    PrintScreen = 283,
    Pause = 284,
    F1 = 290,
    F2 = 291,
    F3 = 292,
    F4 = 293,
    F5 = 294,
    F6 = 295,
    F7 = 296,
    F8 = 297,
    F9 = 298,
    F10 = 299,
    F11 = 300,
    F12 = 301,
    Kp0 = 320,
    Kp1 = 321,
    Kp2 = 322,
    Kp3 = 323,
    Kp4 = 324,
    Kp5 = 325,
    Kp6 = 326,
    Kp7 = 327,
    Kp8 = 328,
    Kp9 = 329,
    KpDecimal = 330,
    KpDivide = 331,
    KpMultiply = 332,
    KpSubtract = 333,
    KpAdd = 334,
    KpEnter = 335,
    KpEqual = 336,
    LeftShift = 340,
    LeftControl = 341,
    LeftAlt = 342,
    LeftSuper = 343,
    RightShift = 344,
    RightControl = 345,
    RightAlt = 346,
    RightSuper = 347,
    KbMenu = 348,
}

impl Key {
    fn from_u32(v: u32) -> Option<Self> {
        use Key::*;
        let k = match v {
            4 => Back,
            24 => VolumeUp,
            25 => VolumeDown,
            32 => Space,
            39 => Apostrophe,
            44 => Comma,
            45 => Minus,
            46 => Period,
            47 => Slash,
            48 => Zero,
            49 => One,
            50 => Two,
            51 => Three,
            52 => Four,
            53 => Five,
            54 => Six,
            55 => Seven,
            56 => Eight,
            57 => Nine,
            59 => Semicolon,
            60 => Equal,
            65 => A,
            66 => B,
            67 => C,
            68 => D,
            69 => E,
            70 => F,
            71 => G,
            72 => H,
            73 => I,
            74 => J,
            75 => K,
            76 => L,
            77 => M,
            78 => N,
            79 => O,
            80 => P,
            81 => Q,
            82 => R,
            83 => S,
            84 => T,
            85 => U,
            86 => V,
            87 => W,
            88 => X,
            89 => Y,
            90 => Z,
            91 => LeftBracket,
            92 => Backslash,
            93 => RightBracket,
            96 => Grave,
            256 => Escape,
            257 => Enter,
            258 => Tab,
            259 => Backspace,
            260 => Insert,
            261 => Delete,
            262 => Right,
            263 => Left,
            264 => Down,
            265 => Up,
            266 => PageUp,
            267 => PageDown,
            268 => Home,
            269 => End,
            280 => CapsLock,
            281 => ScrollLock,
            282 => NumLock,
            283 => PrintScreen,
            284 => Pause,
            290 => F1,
            291 => F2,
            292 => F3,
            293 => F4,
            294 => F5,
            295 => F6,
            296 => F7,
            297 => F8,
            298 => F9,
            299 => F10,
            300 => F11,
            301 => F12,
            320 => Kp0,
            321 => Kp1,
            322 => Kp2,
            323 => Kp3,
            324 => Kp4,
            325 => Kp5,
            326 => Kp6,
            327 => Kp7,
            328 => Kp8,
            329 => Kp9,
            330 => KpDecimal,
            331 => KpDivide,
            332 => KpMultiply,
            333 => KpSubtract,
            334 => KpAdd,
            335 => KpEnter,
            336 => KpEqual,
            340 => LeftShift,
            341 => LeftControl,
            342 => LeftAlt,
            343 => LeftSuper,
            344 => RightShift,
            345 => RightControl,
            346 => RightAlt,
            347 => RightSuper,
            348 => KbMenu,
            _ => return None,
        };
        Some(k)
    }
}
