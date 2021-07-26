use core::str::FromStr;

// Based on
// https://www.w3.org/TR/uievents-code/
// with additional values and mappings from
// https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code/code_values

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum KeyCode {
    // Writing System Keys
    Backquote,
    Backslash,
    Backspace,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,

    // Functional Keys
    AltLeft,
    AltRight,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    MetaLeft,  // Known as OSLeft in Firefox and Chrome sometimes
    MetaRight, // Known as OSRight in Firefox and Chrome sometimes
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,

    // Functional Keys found on Japanese and Korean keyboards
    Convert,
    KanaMode,
    Lang1, // AKA HangulMode in Chrome
    Lang2, // AKA Hanja in Chrome
    Lang3,
    Lang4,
    Lang5,
    NonConvert,

    // Control Pad Section
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,

    // Arrow Pad Section
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,

    // Numpad Section
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,  // No browser seems to use these?
    NumpadClear,      // No browser seems to use these?
    NumpadClearEntry, // No browser seems to use these?
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,           // No browser seems to use these?
    NumpadMemoryAdd,      // No browser seems to use these?
    NumpadMemoryClear,    // No browser seems to use these?
    NumpadMemoryRecall,   // No browser seems to use these?
    NumpadMemoryStore,    // No browser seems to use these?
    NumpadMemorySubtract, // No browser seems to use these?
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar, // No browser seems to use these?
    NumpadSubtract,

    // Function Section
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Fn,
    FnLock, // No browser seems to use these?
    PrintScreen,
    ScrollLock,
    Pause,

    // Media Keys
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop, // AKA Cancel in Chrome
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect, // Unused since Firefox 49
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,

    // Legacy, Non-Standard and Special Keys
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,

    // Gamepad Keys
    Gamepad0,
    Gamepad1,
    Gamepad2,
    Gamepad3,
    Gamepad4,
    Gamepad5,
    Gamepad6,
    Gamepad7,
    Gamepad8,
    Gamepad9,
    Gamepad10,
    Gamepad11,
    Gamepad12,
    Gamepad13,
    Gamepad14,
    Gamepad15,
    Gamepad16,
    Gamepad17,
    Gamepad18,
    Gamepad19,

    // Browser specific Keys
    LaunchMediaPlayer, // Firefox only
    NumpadChangeSign,  // Chrome only
}

impl FromStr for KeyCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::KeyCode::*;
        Ok(match s {
            // Writing System Keys
            "Backquote" => Backquote,
            "Backslash" => Backslash,
            "Backspace" => Backspace,
            "BracketLeft" => BracketLeft,
            "BracketRight" => BracketRight,
            "Comma" => Comma,
            "Digit0" | "0" => Digit0,
            "Digit1" | "1" => Digit1,
            "Digit2" | "2" => Digit2,
            "Digit3" | "3" => Digit3,
            "Digit4" | "4" => Digit4,
            "Digit5" | "5" => Digit5,
            "Digit6" | "6" => Digit6,
            "Digit7" | "7" => Digit7,
            "Digit8" | "8" => Digit8,
            "Digit9" | "9" => Digit9,
            "Equal" => Equal,
            "IntlBackslash" => IntlBackslash,
            "IntlRo" => IntlRo,
            "IntlYen" => IntlYen,
            "KeyA" | "A" => KeyA,
            "KeyB" | "B" => KeyB,
            "KeyC" | "C" => KeyC,
            "KeyD" | "D" => KeyD,
            "KeyE" | "E" => KeyE,
            "KeyF" | "F" => KeyF,
            "KeyG" | "G" => KeyG,
            "KeyH" | "H" => KeyH,
            "KeyI" | "I" => KeyI,
            "KeyJ" | "J" => KeyJ,
            "KeyK" | "K" => KeyK,
            "KeyL" | "L" => KeyL,
            "KeyM" | "M" => KeyM,
            "KeyN" | "N" => KeyN,
            "KeyO" | "O" => KeyO,
            "KeyP" | "P" => KeyP,
            "KeyQ" | "Q" => KeyQ,
            "KeyR" | "R" => KeyR,
            "KeyS" | "S" => KeyS,
            "KeyT" | "T" => KeyT,
            "KeyU" | "U" => KeyU,
            "KeyV" | "V" => KeyV,
            "KeyW" | "W" => KeyW,
            "KeyX" | "X" => KeyX,
            "KeyY" | "Y" => KeyY,
            "KeyZ" | "Z" => KeyZ,
            "Minus" => Minus,
            "Period" => Period,
            "Quote" => Quote,
            "Semicolon" => Semicolon,
            "Slash" => Slash,

            // Functional Keys
            "AltLeft" => AltLeft,
            "AltRight" => AltRight,
            "CapsLock" => CapsLock,
            "ContextMenu" => ContextMenu,
            "ControlLeft" => ControlLeft,
            "ControlRight" => ControlRight,
            "Enter" => Enter,
            "MetaLeft" | "OSLeft" => MetaLeft,
            "MetaRight" | "OSRight" => MetaRight,
            "ShiftLeft" => ShiftLeft,
            "ShiftRight" => ShiftRight,
            "Space" => Space,
            "Tab" => Tab,

            // Functional Keys found on Japanese and Korean keyboards
            "Convert" => Convert,
            "KanaMode" => KanaMode,
            "Lang1" | "HangulMode" => Lang1,
            "Lang2" | "Hanja" => Lang2,
            "Lang3" => Lang3,
            "Lang4" => Lang4,
            "Lang5" => Lang5,
            "NonConvert" => NonConvert,

            // Control Pad Section
            "Delete" => Delete,
            "End" => End,
            "Help" => Help,
            "Home" => Home,
            "Insert" => Insert,
            "PageDown" => PageDown,
            "PageUp" => PageUp,

            // Arrow Pad Section
            "ArrowDown" => ArrowDown,
            "ArrowLeft" => ArrowLeft,
            "ArrowRight" => ArrowRight,
            "ArrowUp" => ArrowUp,

            // Numpad Section
            "NumLock" => NumLock,
            "Numpad0" => Numpad0,
            "Numpad1" => Numpad1,
            "Numpad2" => Numpad2,
            "Numpad3" => Numpad3,
            "Numpad4" => Numpad4,
            "Numpad5" => Numpad5,
            "Numpad6" => Numpad6,
            "Numpad7" => Numpad7,
            "Numpad8" => Numpad8,
            "Numpad9" => Numpad9,
            "NumpadAdd" => NumpadAdd,
            "NumpadBackspace" => NumpadBackspace, // No browser seems to use these?
            "NumpadClear" => NumpadClear,         // No browser seems to use these?
            "NumpadClearEntry" => NumpadClearEntry, // No browser seems to use these?
            "NumpadComma" => NumpadComma,
            "NumpadDecimal" => NumpadDecimal,
            "NumpadDivide" => NumpadDivide,
            "NumpadEnter" => NumpadEnter,
            "NumpadEqual" => NumpadEqual,
            "NumpadHash" => NumpadHash, // No browser seems to use these?
            "NumpadMemoryAdd" => NumpadMemoryAdd, // No browser seems to use these?
            "NumpadMemoryClear" => NumpadMemoryClear, // No browser seems to use these?
            "NumpadMemoryRecall" => NumpadMemoryRecall, // No browser seems to use these?
            "NumpadMemoryStore" => NumpadMemoryStore, // No browser seems to use these?
            "NumpadMemorySubtract" => NumpadMemorySubtract, // No browser seems to use these?
            "NumpadMultiply" => NumpadMultiply,
            "NumpadParenLeft" => NumpadParenLeft,
            "NumpadParenRight" => NumpadParenRight,
            "NumpadStar" => NumpadStar, // No browser seems to use these?
            "NumpadSubtract" => NumpadSubtract,

            // Function Section
            "Escape" => Escape,
            "F1" => F1,
            "F2" => F2,
            "F3" => F3,
            "F4" => F4,
            "F5" => F5,
            "F6" => F6,
            "F7" => F7,
            "F8" => F8,
            "F9" => F9,
            "F10" => F10,
            "F11" => F11,
            "F12" => F12,
            "F13" => F13,
            "F14" => F14,
            "F15" => F15,
            "F16" => F16,
            "F17" => F17,
            "F18" => F18,
            "F19" => F19,
            "F20" => F20,
            "F21" => F21,
            "F22" => F22,
            "F23" => F23,
            "F24" => F24,
            "Fn" => Fn,
            "FnLock" => FnLock, // No browser seems to use these?
            "PrintScreen" => PrintScreen,
            "ScrollLock" => ScrollLock,
            "Pause" => Pause,

            // Media Keys
            "BrowserBack" => BrowserBack,
            "BrowserFavorites" => BrowserFavorites,
            "BrowserForward" => BrowserForward,
            "BrowserHome" => BrowserHome,
            "BrowserRefresh" => BrowserRefresh,
            "BrowserSearch" => BrowserSearch,
            "BrowserStop" | "Cancel" => BrowserStop,
            "Eject" => Eject,
            "LaunchApp1" => LaunchApp1,
            "LaunchApp2" => LaunchApp2,
            "LaunchMail" => LaunchMail,
            "MediaPlayPause" => MediaPlayPause,
            "MediaSelect" => MediaSelect, // Unused since Firefox 49
            "MediaStop" => MediaStop,
            "MediaTrackNext" => MediaTrackNext,
            "MediaTrackPrevious" => MediaTrackPrevious,
            "Power" => Power,
            "Sleep" => Sleep,
            "AudioVolumeDown" => AudioVolumeDown,
            "AudioVolumeMute" => AudioVolumeMute,
            "AudioVolumeUp" => AudioVolumeUp,
            "WakeUp" => WakeUp,

            // Legacy, Non-Standard and Special Keys
            "Again" => Again,
            "Copy" => Copy,
            "Cut" => Cut,
            "Find" => Find,
            "Open" => Open,
            "Paste" => Paste,
            "Props" => Props,
            "Select" => Select,
            "Undo" => Undo,

            // Gamepad Keys
            "Gamepad0" => Gamepad0,
            "Gamepad1" => Gamepad1,
            "Gamepad2" => Gamepad2,
            "Gamepad3" => Gamepad3,
            "Gamepad4" => Gamepad4,
            "Gamepad5" => Gamepad5,
            "Gamepad6" => Gamepad6,
            "Gamepad7" => Gamepad7,
            "Gamepad8" => Gamepad8,
            "Gamepad9" => Gamepad9,
            "Gamepad10" => Gamepad10,
            "Gamepad11" => Gamepad11,
            "Gamepad12" => Gamepad12,
            "Gamepad13" => Gamepad13,
            "Gamepad14" => Gamepad14,
            "Gamepad15" => Gamepad15,
            "Gamepad16" => Gamepad16,
            "Gamepad17" => Gamepad17,
            "Gamepad18" => Gamepad18,
            "Gamepad19" => Gamepad19,

            // Browser specific Keys
            "LaunchMediaPlayer" => LaunchMediaPlayer, // Firefox only
            "NumpadChangeSign" => NumpadChangeSign,   // Chrome only
            _ => return Err(()),
        })
    }
}
