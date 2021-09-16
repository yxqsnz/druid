// Copyright 2020 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Keyboard types.

// This is a reasonable lint, but we keep signatures in sync with the
// bitflags implementation of the inner Modifiers type.
#![allow(clippy::trivially_copy_pass_by_ref)]

use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

pub use keyboard_types::{Code, KeyState, Location};

/// The meaning (mapped value) of a keypress.
pub type KbKey = keyboard_types::Key;

/// Information about a keyboard event.
///
/// Note that this type is similar to [`KeyboardEvent`] in keyboard-types,
/// but has a few small differences for convenience. It is missing the `state`
/// field because that is already implicit in the event.
///
/// [`KeyboardEvent`]: keyboard_types::KeyboardEvent
#[non_exhaustive]
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct KeyEvent {
    /// Whether the key is pressed or released.
    pub state: KeyState,
    /// Logical key value.
    pub key: KbKey,
    /// Physical key position.
    pub code: Code,
    /// Location for keys with multiple instances on common keyboards.
    pub location: Location,
    /// Flags for pressed modifier keys.
    pub mods: Modifiers,
    /// True if the key is currently auto-repeated.
    pub repeat: bool,
    /// Events with this flag should be ignored in a text editor
    /// and instead composition events should be used.
    pub is_composing: bool,
}

/// The modifiers.
///
/// This type is a thin wrappers around [`keyboard_types::Modifiers`],
/// mostly for the convenience methods. If those get upstreamed, it
/// will simply become that type.
///
/// [`keyboard_types::Modifiers`]: keyboard_types::Modifiers
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Modifiers(keyboard_types::Modifiers);

/// A convenience trait for creating Key objects.
///
/// This trait is implemented by [`KbKey`] itself and also strings, which are
/// converted into the `Character` variant. It is defined this way and not
/// using the standard `Into` mechanism because `KbKey` is a type in an external
/// crate.
///
/// [`KbKey`]: KbKey
pub trait IntoKey {
    fn into_key(self) -> KbKey;
}

impl KeyEvent {
    #[doc(hidden)]
    /// Create a key event for testing purposes.
    pub fn for_test(mods: impl Into<Modifiers>, key: impl IntoKey) -> KeyEvent {
        let mods = mods.into();
        let key = key.into_key();
        KeyEvent {
            key,
            code: Code::Unidentified,
            location: Location::Standard,
            state: KeyState::Down,
            mods,
            is_composing: false,
            repeat: false,
        }
    }
}

impl Modifiers {
    pub const ALT: Modifiers = Modifiers(keyboard_types::Modifiers::ALT);
    pub const ALT_GRAPH: Modifiers = Modifiers(keyboard_types::Modifiers::ALT_GRAPH);
    pub const CAPS_LOCK: Modifiers = Modifiers(keyboard_types::Modifiers::CAPS_LOCK);
    pub const CONTROL: Modifiers = Modifiers(keyboard_types::Modifiers::CONTROL);
    pub const FN: Modifiers = Modifiers(keyboard_types::Modifiers::FN);
    pub const FN_LOCK: Modifiers = Modifiers(keyboard_types::Modifiers::FN_LOCK);
    pub const META: Modifiers = Modifiers(keyboard_types::Modifiers::META);
    pub const NUM_LOCK: Modifiers = Modifiers(keyboard_types::Modifiers::NUM_LOCK);
    pub const SCROLL_LOCK: Modifiers = Modifiers(keyboard_types::Modifiers::SCROLL_LOCK);
    pub const SHIFT: Modifiers = Modifiers(keyboard_types::Modifiers::SHIFT);
    pub const SYMBOL: Modifiers = Modifiers(keyboard_types::Modifiers::SYMBOL);
    pub const SYMBOL_LOCK: Modifiers = Modifiers(keyboard_types::Modifiers::SYMBOL_LOCK);
    pub const HYPER: Modifiers = Modifiers(keyboard_types::Modifiers::HYPER);
    pub const SUPER: Modifiers = Modifiers(keyboard_types::Modifiers::SUPER);

    /// Get the inner value.
    ///
    /// Note that this function might go away if our changes are upstreamed.
    pub fn raw(&self) -> keyboard_types::Modifiers {
        self.0
    }

    /// Determine whether Shift is set.
    pub fn shift(&self) -> bool {
        self.contains(Modifiers::SHIFT)
    }

    /// Determine whether Ctrl is set.
    pub fn ctrl(&self) -> bool {
        self.contains(Modifiers::CONTROL)
    }

    /// Determine whether Alt is set.
    pub fn alt(&self) -> bool {
        self.contains(Modifiers::ALT)
    }

    /// Determine whether Meta is set.
    pub fn meta(&self) -> bool {
        self.contains(Modifiers::META)
    }

    /// Returns an empty set of modifiers.
    pub fn empty() -> Modifiers {
        Default::default()
    }

    /// Returns `true` if no modifiers are set.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if all the modifiers in `other` are set.
    pub fn contains(&self, other: Modifiers) -> bool {
        self.0.contains(other.0)
    }

    /// Inserts or removes the specified modifiers depending on the passed value.
    pub fn set(&mut self, other: Modifiers, value: bool) {
        self.0.set(other.0, value)
    }
}

impl BitAnd for Modifiers {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Modifiers(self.0 & rhs.0)
    }
}

impl BitAndAssign for Modifiers {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Modifiers(self.0 & rhs.0)
    }
}

impl BitOr for Modifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Modifiers(self.0 | rhs.0)
    }
}

impl BitOrAssign for Modifiers {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Modifiers(self.0 | rhs.0)
    }
}

impl BitXor for Modifiers {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self {
        Modifiers(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Modifiers {
    // rhs is the "right-hand side" of the expression `a &= b`
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = Modifiers(self.0 ^ rhs.0)
    }
}

impl Not for Modifiers {
    type Output = Self;

    fn not(self) -> Self {
        Modifiers(!self.0)
    }
}

impl IntoKey for KbKey {
    fn into_key(self) -> KbKey {
        self
    }
}

impl IntoKey for &str {
    fn into_key(self) -> KbKey {
        KbKey::Character(self.into())
    }
}

pub fn winit_key(input: &winit::event::KeyboardInput, shift: bool) -> KbKey {
    use winit::event::VirtualKeyCode;
    if let Some(key) = input.virtual_keycode.as_ref() {
        match key {
            VirtualKeyCode::Key1 => {
                if !shift {
                    KbKey::Character("1".to_string())
                } else {
                    KbKey::Character("!".to_string())
                }
            }
            VirtualKeyCode::Key2 => {
                if !shift {
                    KbKey::Character("2".to_string())
                } else {
                    KbKey::Character("@".to_string())
                }
            }
            VirtualKeyCode::Key3 => {
                if !shift {
                    KbKey::Character("3".to_string())
                } else {
                    KbKey::Character("#".to_string())
                }
            }
            VirtualKeyCode::Key4 => {
                if !shift {
                    KbKey::Character("4".to_string())
                } else {
                    KbKey::Character("$".to_string())
                }
            }
            VirtualKeyCode::Key5 => {
                if !shift {
                    KbKey::Character("5".to_string())
                } else {
                    KbKey::Character("%".to_string())
                }
            }
            VirtualKeyCode::Key6 => {
                if !shift {
                    KbKey::Character("6".to_string())
                } else {
                    KbKey::Character("^".to_string())
                }
            }
            VirtualKeyCode::Key7 => {
                if !shift {
                    KbKey::Character("7".to_string())
                } else {
                    KbKey::Character("&".to_string())
                }
            }
            VirtualKeyCode::Key8 => {
                if !shift {
                    KbKey::Character("8".to_string())
                } else {
                    KbKey::Character("*".to_string())
                }
            }
            VirtualKeyCode::Key9 => {
                if !shift {
                    KbKey::Character("9".to_string())
                } else {
                    KbKey::Character("(".to_string())
                }
            }
            VirtualKeyCode::Key0 => {
                if !shift {
                    KbKey::Character("0".to_string())
                } else {
                    KbKey::Character(")".to_string())
                }
            }
            VirtualKeyCode::A => {
                if !shift {
                    KbKey::Character("a".to_string())
                } else {
                    KbKey::Character("A".to_string())
                }
            }
            VirtualKeyCode::B => {
                if !shift {
                    KbKey::Character("b".to_string())
                } else {
                    KbKey::Character("B".to_string())
                }
            }
            VirtualKeyCode::C => {
                if !shift {
                    KbKey::Character("c".to_string())
                } else {
                    KbKey::Character("C".to_string())
                }
            }
            VirtualKeyCode::D => {
                if !shift {
                    KbKey::Character("d".to_string())
                } else {
                    KbKey::Character("D".to_string())
                }
            }
            VirtualKeyCode::E => {
                if !shift {
                    KbKey::Character("e".to_string())
                } else {
                    KbKey::Character("E".to_string())
                }
            }
            VirtualKeyCode::F => {
                if !shift {
                    KbKey::Character("f".to_string())
                } else {
                    KbKey::Character("F".to_string())
                }
            }
            VirtualKeyCode::G => {
                if !shift {
                    KbKey::Character("g".to_string())
                } else {
                    KbKey::Character("G".to_string())
                }
            }
            VirtualKeyCode::H => {
                if !shift {
                    KbKey::Character("h".to_string())
                } else {
                    KbKey::Character("H".to_string())
                }
            }
            VirtualKeyCode::I => {
                if !shift {
                    KbKey::Character("i".to_string())
                } else {
                    KbKey::Character("I".to_string())
                }
            }
            VirtualKeyCode::J => {
                if !shift {
                    KbKey::Character("j".to_string())
                } else {
                    KbKey::Character("J".to_string())
                }
            }
            VirtualKeyCode::K => {
                if !shift {
                    KbKey::Character("k".to_string())
                } else {
                    KbKey::Character("K".to_string())
                }
            }
            VirtualKeyCode::L => {
                if !shift {
                    KbKey::Character("l".to_string())
                } else {
                    KbKey::Character("L".to_string())
                }
            }
            VirtualKeyCode::M => {
                if !shift {
                    KbKey::Character("m".to_string())
                } else {
                    KbKey::Character("M".to_string())
                }
            }
            VirtualKeyCode::N => {
                if !shift {
                    KbKey::Character("n".to_string())
                } else {
                    KbKey::Character("N".to_string())
                }
            }
            VirtualKeyCode::O => {
                if !shift {
                    KbKey::Character("o".to_string())
                } else {
                    KbKey::Character("O".to_string())
                }
            }
            VirtualKeyCode::P => {
                if !shift {
                    KbKey::Character("p".to_string())
                } else {
                    KbKey::Character("P".to_string())
                }
            }
            VirtualKeyCode::Q => {
                if !shift {
                    KbKey::Character("q".to_string())
                } else {
                    KbKey::Character("Q".to_string())
                }
            }
            VirtualKeyCode::R => {
                if !shift {
                    KbKey::Character("r".to_string())
                } else {
                    KbKey::Character("R".to_string())
                }
            }
            VirtualKeyCode::S => {
                if !shift {
                    KbKey::Character("s".to_string())
                } else {
                    KbKey::Character("S".to_string())
                }
            }
            VirtualKeyCode::T => {
                if !shift {
                    KbKey::Character("t".to_string())
                } else {
                    KbKey::Character("T".to_string())
                }
            }
            VirtualKeyCode::U => {
                if !shift {
                    KbKey::Character("u".to_string())
                } else {
                    KbKey::Character("U".to_string())
                }
            }
            VirtualKeyCode::V => {
                if !shift {
                    KbKey::Character("v".to_string())
                } else {
                    KbKey::Character("V".to_string())
                }
            }
            VirtualKeyCode::W => {
                if !shift {
                    KbKey::Character("w".to_string())
                } else {
                    KbKey::Character("W".to_string())
                }
            }
            VirtualKeyCode::X => {
                if !shift {
                    KbKey::Character("x".to_string())
                } else {
                    KbKey::Character("X".to_string())
                }
            }
            VirtualKeyCode::Y => {
                if !shift {
                    KbKey::Character("y".to_string())
                } else {
                    KbKey::Character("Y".to_string())
                }
            }
            VirtualKeyCode::Z => {
                if !shift {
                    KbKey::Character("z".to_string())
                } else {
                    KbKey::Character("Z".to_string())
                }
            }
            VirtualKeyCode::Escape => KbKey::Escape,
            VirtualKeyCode::F1 => KbKey::F1,
            VirtualKeyCode::F2 => KbKey::F2,
            VirtualKeyCode::F3 => KbKey::F3,
            VirtualKeyCode::F4 => KbKey::F4,
            VirtualKeyCode::F5 => KbKey::F5,
            VirtualKeyCode::F6 => KbKey::F6,
            VirtualKeyCode::F7 => KbKey::F7,
            VirtualKeyCode::F8 => KbKey::F8,
            VirtualKeyCode::F9 => KbKey::F9,
            VirtualKeyCode::F10 => KbKey::F10,
            VirtualKeyCode::F11 => KbKey::F11,
            VirtualKeyCode::F12 => KbKey::F12,
            VirtualKeyCode::F13 => KbKey::Unidentified,
            VirtualKeyCode::F14 => KbKey::Unidentified,
            VirtualKeyCode::F15 => KbKey::Unidentified,
            VirtualKeyCode::F16 => KbKey::Unidentified,
            VirtualKeyCode::F17 => KbKey::Unidentified,
            VirtualKeyCode::F18 => KbKey::Unidentified,
            VirtualKeyCode::F19 => KbKey::Unidentified,
            VirtualKeyCode::F20 => KbKey::Unidentified,
            VirtualKeyCode::F21 => KbKey::Unidentified,
            VirtualKeyCode::F22 => KbKey::Unidentified,
            VirtualKeyCode::F23 => KbKey::Unidentified,
            VirtualKeyCode::F24 => KbKey::Unidentified,
            VirtualKeyCode::Snapshot => keyboard_types::Key::Unidentified,
            VirtualKeyCode::Scroll => keyboard_types::Key::Unidentified,
            VirtualKeyCode::Pause => keyboard_types::Key::Pause,
            VirtualKeyCode::Insert => keyboard_types::Key::Insert,
            VirtualKeyCode::Home => keyboard_types::Key::Home,
            VirtualKeyCode::Delete => keyboard_types::Key::Delete,
            VirtualKeyCode::End => keyboard_types::Key::End,
            VirtualKeyCode::PageDown => keyboard_types::Key::PageDown,
            VirtualKeyCode::PageUp => keyboard_types::Key::PageUp,
            VirtualKeyCode::Left => keyboard_types::Key::ArrowLeft,
            VirtualKeyCode::Up => keyboard_types::Key::ArrowUp,
            VirtualKeyCode::Right => keyboard_types::Key::ArrowRight,
            VirtualKeyCode::Down => keyboard_types::Key::ArrowDown,
            VirtualKeyCode::Back => KbKey::Backspace,
            VirtualKeyCode::Return => keyboard_types::Key::Enter,
            VirtualKeyCode::Space => KbKey::Character(" ".to_string()),
            VirtualKeyCode::Compose => KbKey::Compose,
            VirtualKeyCode::Caret => KbKey::Unidentified,
            VirtualKeyCode::Numlock => KbKey::NumLock,
            VirtualKeyCode::Numpad0 => KbKey::Unidentified,
            VirtualKeyCode::Numpad1 => KbKey::Unidentified,
            VirtualKeyCode::Numpad2 => KbKey::Unidentified,
            VirtualKeyCode::Numpad3 => KbKey::Unidentified,
            VirtualKeyCode::Numpad4 => KbKey::Unidentified,
            VirtualKeyCode::Numpad5 => KbKey::Unidentified,
            VirtualKeyCode::Numpad6 => KbKey::Unidentified,
            VirtualKeyCode::Numpad7 => KbKey::Unidentified,
            VirtualKeyCode::Numpad8 => KbKey::Unidentified,
            VirtualKeyCode::Numpad9 => KbKey::Unidentified,
            VirtualKeyCode::NumpadAdd => KbKey::Unidentified,
            VirtualKeyCode::NumpadDivide => KbKey::Unidentified,
            VirtualKeyCode::NumpadDecimal => KbKey::Unidentified,
            VirtualKeyCode::NumpadComma => KbKey::Unidentified,
            VirtualKeyCode::NumpadEnter => KbKey::Enter,
            VirtualKeyCode::NumpadEquals => KbKey::Unidentified,
            VirtualKeyCode::NumpadMultiply => KbKey::Unidentified,
            VirtualKeyCode::NumpadSubtract => KbKey::Unidentified,
            VirtualKeyCode::AbntC1 => KbKey::Unidentified,
            VirtualKeyCode::AbntC2 => KbKey::Unidentified,
            VirtualKeyCode::Apostrophe => {
                if !shift {
                    KbKey::Character("'".to_string())
                } else {
                    KbKey::Character("\"".to_string())
                }
            }
            VirtualKeyCode::Apps => KbKey::MediaApps,
            VirtualKeyCode::Asterisk => KbKey::Unidentified,
            VirtualKeyCode::At => KbKey::Unidentified,
            VirtualKeyCode::Ax => KbKey::Unidentified,
            VirtualKeyCode::Backslash => {
                if !shift {
                    KbKey::Character("\\".to_string())
                } else {
                    KbKey::Character("|".to_string())
                }
            }
            VirtualKeyCode::Calculator => KbKey::Unidentified,
            VirtualKeyCode::Capital => KbKey::CapsLock,
            VirtualKeyCode::Colon => KbKey::Unidentified,
            VirtualKeyCode::Comma => {
                if !shift {
                    KbKey::Character(",".to_string())
                } else {
                    KbKey::Character("<".to_string())
                }
            }
            VirtualKeyCode::Convert => KbKey::Convert,
            VirtualKeyCode::Equals => {
                if !shift {
                    KbKey::Character("=".to_string())
                } else {
                    KbKey::Character("+".to_string())
                }
            }
            VirtualKeyCode::Grave => {
                if !shift {
                    KbKey::Character("`".to_string())
                } else {
                    KbKey::Character("~".to_string())
                }
            }
            VirtualKeyCode::Kana => KbKey::KanaMode,
            VirtualKeyCode::Kanji => KbKey::KanjiMode,
            VirtualKeyCode::LAlt => KbKey::Alt,
            VirtualKeyCode::LBracket => {
                if !shift {
                    KbKey::Character("[".to_string())
                } else {
                    KbKey::Character("{".to_string())
                }
            }
            VirtualKeyCode::LControl => KbKey::Control,
            VirtualKeyCode::LShift => KbKey::Shift,
            VirtualKeyCode::LWin => KbKey::Meta,
            VirtualKeyCode::Mail => KbKey::MailSend,
            VirtualKeyCode::MediaSelect => KbKey::MediaApps,
            VirtualKeyCode::MediaStop => KbKey::MediaStop,
            VirtualKeyCode::Minus => {
                if !shift {
                    KbKey::Character("-".to_string())
                } else {
                    KbKey::Character("_".to_string())
                }
            }
            VirtualKeyCode::Mute => KbKey::AudioVolumeMute,
            VirtualKeyCode::MyComputer => KbKey::Unidentified,
            VirtualKeyCode::NavigateForward => KbKey::BrowserForward,
            VirtualKeyCode::NavigateBackward => KbKey::BrowserBack,
            VirtualKeyCode::NextTrack => KbKey::MediaTrackNext,
            VirtualKeyCode::NoConvert => KbKey::NonConvert,
            VirtualKeyCode::OEM102 => KbKey::Unidentified,
            VirtualKeyCode::Period => {
                if !shift {
                    KbKey::Character(".".to_string())
                } else {
                    KbKey::Character(">".to_string())
                }
            }
            VirtualKeyCode::PlayPause => KbKey::MediaPlayPause,
            VirtualKeyCode::Plus => KbKey::Unidentified,
            VirtualKeyCode::Power => KbKey::Power,
            VirtualKeyCode::PrevTrack => KbKey::MediaTrackPrevious,
            VirtualKeyCode::RAlt => KbKey::Alt,
            VirtualKeyCode::RBracket => {
                if !shift {
                    KbKey::Character("]".to_string())
                } else {
                    KbKey::Character("}".to_string())
                }
            }
            VirtualKeyCode::RControl => KbKey::Control,
            VirtualKeyCode::RShift => KbKey::Shift,
            VirtualKeyCode::RWin => KbKey::Meta,
            VirtualKeyCode::Semicolon => {
                if !shift {
                    KbKey::Character(";".to_string())
                } else {
                    KbKey::Character(":".to_string())
                }
            }
            VirtualKeyCode::Slash => {
                if !shift {
                    KbKey::Character("/".to_string())
                } else {
                    KbKey::Character("?".to_string())
                }
            }
            VirtualKeyCode::Sleep => KbKey::Unidentified,
            VirtualKeyCode::Stop => KbKey::MediaStop,
            VirtualKeyCode::Sysrq => KbKey::Unidentified,
            VirtualKeyCode::Tab => KbKey::Tab,
            VirtualKeyCode::Underline => KbKey::Unidentified,
            VirtualKeyCode::Unlabeled => KbKey::Unidentified,
            VirtualKeyCode::VolumeDown => KbKey::AudioVolumeDown,
            VirtualKeyCode::VolumeUp => KbKey::AudioVolumeUp,
            VirtualKeyCode::Wake => KbKey::WakeUp,
            VirtualKeyCode::WebBack => KbKey::BrowserBack,
            VirtualKeyCode::WebFavorites => KbKey::BrowserFavorites,
            VirtualKeyCode::WebForward => KbKey::BrowserForward,
            VirtualKeyCode::WebHome => KbKey::BrowserHome,
            VirtualKeyCode::WebRefresh => KbKey::BrowserRefresh,
            VirtualKeyCode::WebSearch => KbKey::BrowserSearch,
            VirtualKeyCode::WebStop => KbKey::BrowserStop,
            VirtualKeyCode::Yen => KbKey::Unidentified,
            VirtualKeyCode::Copy => KbKey::Copy,
            VirtualKeyCode::Paste => KbKey::Paste,
            VirtualKeyCode::Cut => KbKey::Cut,
        }
    } else {
        keyboard_types::Key::Unidentified
    }
}
