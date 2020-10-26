use bevy::input::{keyboard::KeyCode, mouse::MouseButton};

pub fn convert_mouse_button(mouse_button: usize) -> MouseButton {
    match mouse_button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        btn => MouseButton::Other(btn as u8),
    }
}

pub fn convert_key_str(key: &str) -> Option<KeyCode> {
    match key {
        "Digit1" => Some(KeyCode::Key1),
        "Digit2" => Some(KeyCode::Key2),
        "Digit3" => Some(KeyCode::Key3),
        "Digit4" => Some(KeyCode::Key4),
        "Digit5" => Some(KeyCode::Key5),
        "Digit6" => Some(KeyCode::Key6),
        "Digit7" => Some(KeyCode::Key7),
        "Digit8" => Some(KeyCode::Key8),
        "Digit9" => Some(KeyCode::Key9),
        "Digit0" => Some(KeyCode::Key0),
        "KeyA" => Some(KeyCode::A),
        "KeyB" => Some(KeyCode::B),
        "KeyC" => Some(KeyCode::C),
        "KeyD" => Some(KeyCode::D),
        "KeyE" => Some(KeyCode::E),
        "KeyF" => Some(KeyCode::F),
        "KeyG" => Some(KeyCode::G),
        "KeyH" => Some(KeyCode::H),
        "KeyI" => Some(KeyCode::I),
        "KeyJ" => Some(KeyCode::J),
        "KeyK" => Some(KeyCode::K),
        "KeyL" => Some(KeyCode::L),
        "KeyM" => Some(KeyCode::M),
        "KeyN" => Some(KeyCode::N),
        "KeyO" => Some(KeyCode::O),
        "KeyP" => Some(KeyCode::P),
        "KeyQ" => Some(KeyCode::Q),
        "KeyR" => Some(KeyCode::R),
        "KeyS" => Some(KeyCode::S),
        "KeyT" => Some(KeyCode::T),
        "KeyU" => Some(KeyCode::U),
        "KeyV" => Some(KeyCode::V),
        "KeyW" => Some(KeyCode::W),
        "KeyX" => Some(KeyCode::X),
        "KeyY" => Some(KeyCode::Y),
        "KeyZ" => Some(KeyCode::Z),
        "Escape" => Some(KeyCode::Escape),
        "F1" => Some(KeyCode::F1),
        "F2" => Some(KeyCode::F2),
        "F3" => Some(KeyCode::F3),
        "F4" => Some(KeyCode::F4),
        "F5" => Some(KeyCode::F5),
        "F6" => Some(KeyCode::F6),
        "F7" => Some(KeyCode::F7),
        "F8" => Some(KeyCode::F8),
        "F9" => Some(KeyCode::F9),
        "F10" => Some(KeyCode::F10),
        "F11" => Some(KeyCode::F11),
        "F12" => Some(KeyCode::F12),
        "F13" => Some(KeyCode::F13),
        "F14" => Some(KeyCode::F14),
        "F15" => Some(KeyCode::F15),
        "F16" => Some(KeyCode::F16),
        "F17" => Some(KeyCode::F17),
        "F18" => Some(KeyCode::F18),
        "F19" => Some(KeyCode::F19),
        "F20" => Some(KeyCode::F20),
        "F21" => Some(KeyCode::F21),
        "F22" => Some(KeyCode::F22),
        "F23" => Some(KeyCode::F23),
        "F24" => Some(KeyCode::F24),
        "Snapshot" => Some(KeyCode::Snapshot),
        "ScrollLock" => Some(KeyCode::Scroll),
        "Pause" => Some(KeyCode::Pause),
        "Insert" => Some(KeyCode::Insert),
        "Home" => Some(KeyCode::Home),
        "Delete" => Some(KeyCode::Delete),
        "End" => Some(KeyCode::End),
        "PageDown" => Some(KeyCode::PageDown),
        "PageUp" => Some(KeyCode::PageUp),
        "ArrowLeft" => Some(KeyCode::Left),
        "ArrowUp" => Some(KeyCode::Up),
        "ArrowRight" => Some(KeyCode::Right),
        "ArrowDown" => Some(KeyCode::Down),
        "Backspace" => Some(KeyCode::Back),
        "Enter" => Some(KeyCode::Return),
        "Space" => Some(KeyCode::Space),
        "Compose" => Some(KeyCode::Compose),
        "NumLock" => Some(KeyCode::Numlock),
        "Numpad0" => Some(KeyCode::Numpad0),
        "Numpad1" => Some(KeyCode::Numpad1),
        "Numpad2" => Some(KeyCode::Numpad2),
        "Numpad3" => Some(KeyCode::Numpad3),
        "Numpad4" => Some(KeyCode::Numpad4),
        "Numpad5" => Some(KeyCode::Numpad5),
        "Numpad6" => Some(KeyCode::Numpad6),
        "Numpad7" => Some(KeyCode::Numpad7),
        "Numpad8" => Some(KeyCode::Numpad8),
        "Numpad9" => Some(KeyCode::Numpad9),
        "AbntC1" => Some(KeyCode::AbntC1),
        "AbntC2" => Some(KeyCode::AbntC2),
        "NumpadAdd" => Some(KeyCode::NumpadAdd),
        "Apostrophe" => Some(KeyCode::Apostrophe),
        "Backslash" => Some(KeyCode::Backslash),
        "Colon" => Some(KeyCode::Colon),
        "Comma" => Some(KeyCode::Comma),
        "NumpadDecimal" => Some(KeyCode::NumpadDecimal),
        "NumpadDivide" => Some(KeyCode::NumpadDivide),
        "Equal" => Some(KeyCode::Equals),
        "Backquote" => Some(KeyCode::Grave),
        "AltLeft" => Some(KeyCode::LAlt),
        "BracketLeft" => Some(KeyCode::LBracket),
        "ControlLeft" => Some(KeyCode::LControl),
        "ShiftLeft" => Some(KeyCode::LShift),
        "LeftWin" => Some(KeyCode::LWin),
        "Minus" => Some(KeyCode::Minus),
        "NumpadMultiply" => Some(KeyCode::NumpadMultiply),
        "NumpadEnter" => Some(KeyCode::NumpadEnter),
        "NumpadEqual" => Some(KeyCode::NumpadEquals),
        "Period" => Some(KeyCode::Period),
        "AltRight" => Some(KeyCode::RAlt),
        "BracketRight" => Some(KeyCode::RBracket),
        "ControlRight" => Some(KeyCode::RControl),
        "ShiftRight" => Some(KeyCode::RShift),
        "RightWin" => Some(KeyCode::RWin),
        "Semicolon" => Some(KeyCode::Semicolon),
        "Slash" => Some(KeyCode::Slash),
        "NumpadSubtract" => Some(KeyCode::NumpadSubtract),
        "Tab" => Some(KeyCode::Tab),
        "Caret" => Some(KeyCode::Caret),
        "Apps" => Some(KeyCode::Apps),
        "Asterix" => Some(KeyCode::Asterix),
        "Plus" => Some(KeyCode::Plus),
        "At" => Some(KeyCode::At),
        "Ax" => Some(KeyCode::Ax),
        "Calculator" => Some(KeyCode::Calculator),
        "Capital" => Some(KeyCode::Capital),
        "Convert" => Some(KeyCode::Convert),
        "Kana" => Some(KeyCode::Kana),
        "Kanji" => Some(KeyCode::Kanji),
        "Mail" => Some(KeyCode::Mail),
        "MediaSelect" => Some(KeyCode::MediaSelect),
        "MediaStop" => Some(KeyCode::MediaStop),
        "Mute" => Some(KeyCode::Mute),
        "MyComputer" => Some(KeyCode::MyComputer),
        "NavigateForward" => Some(KeyCode::NavigateForward),
        "NavigateBackward" => Some(KeyCode::NavigateBackward),
        "NextTrack" => Some(KeyCode::NextTrack),
        "NoConvert" => Some(KeyCode::NoConvert),
        "NumpadComma" => Some(KeyCode::NumpadComma),
        "OEM102" => Some(KeyCode::OEM102),
        "PlayPause" => Some(KeyCode::PlayPause),
        "Power" => Some(KeyCode::Power),
        "PrevTrack" => Some(KeyCode::PrevTrack),
        "Sleep" => Some(KeyCode::Sleep),
        "Stop" => Some(KeyCode::Stop),
        "Sysrq" => Some(KeyCode::Sysrq),
        "Underline" => Some(KeyCode::Underline),
        "Unlabeled" => Some(KeyCode::Unlabeled),
        "VolumeDown" => Some(KeyCode::VolumeDown),
        "VolumeUp" => Some(KeyCode::VolumeUp),
        "Wake" => Some(KeyCode::Wake),
        "WebBack" => Some(KeyCode::WebBack),
        "WebFavorites" => Some(KeyCode::WebFavorites),
        "WebForward" => Some(KeyCode::WebForward),
        "WebHome" => Some(KeyCode::WebHome),
        "WebRefresh" => Some(KeyCode::WebRefresh),
        "WebSearch" => Some(KeyCode::WebSearch),
        "WebStop" => Some(KeyCode::WebStop),
        "Yen" => Some(KeyCode::Yen),
        "Copy" => Some(KeyCode::Copy),
        "Paste" => Some(KeyCode::Paste),
        "Cut" => Some(KeyCode::Cut),
        _ => None,
    }
}