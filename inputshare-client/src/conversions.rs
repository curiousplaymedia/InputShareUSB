use inputshare_common::{HidKeyCode, HidModifierKey, HidMouseButton};
use yawi::{WindowsScanCode, VirtualKey};

pub fn f32_to_i8(v: f32) -> i8 {
     v.round().clamp(i8::MIN as f32, i8::MAX as f32) as i8
}

pub fn vk_to_mod(key: VirtualKey) -> Option<HidModifierKey> {
    match key {
        VirtualKey::LShift   => Some(HidModifierKey::LShift),
        VirtualKey::LControl => Some(HidModifierKey::LCtrl),
        VirtualKey::LWin     => Some(HidModifierKey::LMeta),
        VirtualKey::LMenu    => Some(HidModifierKey::LAlt),
        VirtualKey::RShift   => Some(HidModifierKey::RShift),
        VirtualKey::RControl => Some(HidModifierKey::RCtrl),
        VirtualKey::RWin     => Some(HidModifierKey::RMeta),
        VirtualKey::RMenu    => Some(HidModifierKey::RAlt),
        _                    => None
    }
}

pub fn vk_to_mb(key: VirtualKey) -> Option<HidMouseButton> {
    match key {
        VirtualKey::LButton  => Some(HidMouseButton::LButton),
        VirtualKey::RButton  => Some(HidMouseButton::RButton),
        VirtualKey::MButton  => Some(HidMouseButton::MButton),
        VirtualKey::XButton1 => Some(HidMouseButton::Button4),
        VirtualKey::XButton2 => Some(HidMouseButton::Button5),
        _                    => None
    }
}

pub fn wsc_to_hkc(scancode: WindowsScanCode) -> Option<HidKeyCode> {
     match scancode {
          0x1 => Some(HidKeyCode::Escape),
          0x2 => Some(HidKeyCode::Key1),
          0x3 => Some(HidKeyCode::Key2),
          0x4 => Some(HidKeyCode::Key3),
          0x5 => Some(HidKeyCode::Key4),
          0x6 => Some(HidKeyCode::Key5),
          0x7 => Some(HidKeyCode::Key6),
          0x8 => Some(HidKeyCode::Key7),
          0x9 => Some(HidKeyCode::Key8),
          0xa => Some(HidKeyCode::Key9),
          0xb => Some(HidKeyCode::Key0),
          0xc => Some(HidKeyCode::Minus),
          0xd => Some(HidKeyCode::Equal),
          0xe => Some(HidKeyCode::Backspace),
          0xf => Some(HidKeyCode::Tab),
          0x10 => Some(HidKeyCode::KeyQ),
          0x11 => Some(HidKeyCode::KeyW),
          0x12 => Some(HidKeyCode::KeyE),
          0x13 => Some(HidKeyCode::KeyR),
          0x14 => Some(HidKeyCode::KeyT),
          0x15 => Some(HidKeyCode::KeyY),
          0x16 => Some(HidKeyCode::KeyU),
          0x17 => Some(HidKeyCode::KeyI),
          0x18 => Some(HidKeyCode::KeyO),
          0x19 => Some(HidKeyCode::KeyP),
          0x1a => Some(HidKeyCode::LeftBrace),
          0x1b => Some(HidKeyCode::RightBrace),
          0x1c => Some(HidKeyCode::Enter),
          0x1d => Some(HidKeyCode::LeftCtrl),
          0x1e => Some(HidKeyCode::KeyA),
          0x1f => Some(HidKeyCode::KeyS),
          0x20 => Some(HidKeyCode::KeyD),
          0x21 => Some(HidKeyCode::KeyF),
          0x22 => Some(HidKeyCode::KeyG),
          0x23 => Some(HidKeyCode::KeyH),
          0x24 => Some(HidKeyCode::KeyJ),
          0x25 => Some(HidKeyCode::KeyK),
          0x26 => Some(HidKeyCode::KeyL),
          0x27 => Some(HidKeyCode::Semicolon),
          0x28 => Some(HidKeyCode::Apostrophe),
          0x29 => Some(HidKeyCode::Grave),
          0x2a => Some(HidKeyCode::LeftShift),
          0x2b => Some(HidKeyCode::Backslash),
          0x2c => Some(HidKeyCode::KeyZ),
          0x2d => Some(HidKeyCode::KeyX),
          0x2e => Some(HidKeyCode::KeyC),
          0x2f => Some(HidKeyCode::KeyV),
          0x30 => Some(HidKeyCode::KeyB),
          0x31 => Some(HidKeyCode::KeyN),
          0x32 => Some(HidKeyCode::KeyM),
          0x33 => Some(HidKeyCode::Comma),
          0x34 => Some(HidKeyCode::Dot),
          0x35 => Some(HidKeyCode::Slash),
          0x36 => Some(HidKeyCode::RightShift),
          0x37 => Some(HidKeyCode::KpAsterisk),
          0x38 => Some(HidKeyCode::LeftAlt),
          0x39 => Some(HidKeyCode::Space),
          0x3a => Some(HidKeyCode::Capslock),
          0x3b => Some(HidKeyCode::F1),
          0x3c => Some(HidKeyCode::F2),
          0x3d => Some(HidKeyCode::F3),
          0x3e => Some(HidKeyCode::F4),
          0x3f => Some(HidKeyCode::F5),
          0x40 => Some(HidKeyCode::F6),
          0x41 => Some(HidKeyCode::F7),
          0x42 => Some(HidKeyCode::F8),
          0x43 => Some(HidKeyCode::F9),
          0x44 => Some(HidKeyCode::F10),
          0x45 => Some(HidKeyCode::NumLock),
          0x46 => Some(HidKeyCode::ScrollLock),
          0x47 => Some(HidKeyCode::Kp7),
          0x48 => Some(HidKeyCode::Kp8),
          0x49 => Some(HidKeyCode::Kp9),
          0x4a => Some(HidKeyCode::KpMinus),
          0x4b => Some(HidKeyCode::Kp4),
          0x4c => Some(HidKeyCode::Kp5),
          0x4d => Some(HidKeyCode::Kp6),
          0x4e => Some(HidKeyCode::KpPlus),
          0x4f => Some(HidKeyCode::Kp1),
          0x50 => Some(HidKeyCode::Kp2),
          0x51 => Some(HidKeyCode::Kp3),
          0x52 => Some(HidKeyCode::Kp0),
          0x53 => Some(HidKeyCode::KpDot),
          0x54 => Some(HidKeyCode::PrintScreen),
          0x56 => Some(HidKeyCode::Key102ND),
          0x57 => Some(HidKeyCode::F11),
          0x58 => Some(HidKeyCode::F12),
          0x59 => Some(HidKeyCode::KpEqual),
          0x5c => Some(HidKeyCode::International6),
          0x64 => Some(HidKeyCode::F13),
          0x65 => Some(HidKeyCode::F14),
          0x66 => Some(HidKeyCode::F15),
          0x67 => Some(HidKeyCode::F16),
          0x68 => Some(HidKeyCode::F17),
          0x69 => Some(HidKeyCode::F18),
          0x6a => Some(HidKeyCode::F19),
          0x6b => Some(HidKeyCode::F20),
          0x6c => Some(HidKeyCode::F21),
          0x6d => Some(HidKeyCode::F22),
          0x6e => Some(HidKeyCode::F23),
          0x70 => Some(HidKeyCode::International2),
          0x73 => Some(HidKeyCode::International1),
          0x76 => Some(HidKeyCode::F24),
          0x77 => Some(HidKeyCode::Language4),
          0x78 => Some(HidKeyCode::Language3),
          0x79 => Some(HidKeyCode::International4),
          0x7b => Some(HidKeyCode::International5),
          0x7d => Some(HidKeyCode::International3),
          0x7e => Some(HidKeyCode::KpComma),
          0xf1 => Some(HidKeyCode::Language2),
          0xf2 => Some(HidKeyCode::Language1),
          0xff => Some(HidKeyCode::ErrorRollOver),
          0xe010 => Some(HidKeyCode::MediaPreviousSong),
          0xe019 => Some(HidKeyCode::MediaNextSong),
          0xe01c => Some(HidKeyCode::KpEnter),
          0xe01d => Some(HidKeyCode::RightCtrl),
          0xe020 => Some(HidKeyCode::MediaMute),
          0xe021 => Some(HidKeyCode::MediaCalc),
          0xe022 => Some(HidKeyCode::MediaPlayPause),
          0xe024 => Some(HidKeyCode::MediaStopCD),
          0xe02e => Some(HidKeyCode::MediaVolumeDown),
          0xe030 => Some(HidKeyCode::MediaVolumeUp),
          0xe032 => Some(HidKeyCode::MediaWWW),
          0xe035 => Some(HidKeyCode::KpSlash),
          0xe038 => Some(HidKeyCode::RightAlt),
          0xe047 => Some(HidKeyCode::Home),
          0xe048 => Some(HidKeyCode::Up),
          0xe049 => Some(HidKeyCode::PageUp),
          0xe04b => Some(HidKeyCode::Left),
          0xe04d => Some(HidKeyCode::Right),
          0xe04f => Some(HidKeyCode::End),
          0xe050 => Some(HidKeyCode::Down),
          0xe051 => Some(HidKeyCode::PageDown),
          0xe052 => Some(HidKeyCode::Insert),
          0xe053 => Some(HidKeyCode::Delete),
          0xe05b => Some(HidKeyCode::LeftMeta),
          0xe05c => Some(HidKeyCode::RightMeta),
          0xe05d => Some(HidKeyCode::Compose),
          0xe05e => Some(HidKeyCode::Power),
          0xe05f => Some(HidKeyCode::LockingCapsLock),
          0xe063 => Some(HidKeyCode::LockingNumLock),
          0xe065 => Some(HidKeyCode::MediaFind),
          0xe067 => Some(HidKeyCode::MediaRefresh),
          0xe068 => Some(HidKeyCode::MediaStop),
          0xe069 => Some(HidKeyCode::MediaForward),
          0xe06a => Some(HidKeyCode::MediaBack),
          0xe06d => Some(HidKeyCode::MediaEjectCD),
          0xe11d => Some(HidKeyCode::Pause),
          _ => None
     }
}