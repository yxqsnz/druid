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
use winit::keyboard::{KeyCode, NativeKeyCode};

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
    pub code: KeyCode,
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
            code: KeyCode::Unidentified(NativeKeyCode::Unidentified),
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

pub fn winit_key(input: winit::keyboard::Key<'static>) -> KbKey {
    match input {
        winit::keyboard::Key::Character(c) => KbKey::Character(c.to_string()),
        winit::keyboard::Key::Unidentified(_) => KbKey::Unidentified,
        winit::keyboard::Key::Dead(_) => KbKey::Dead,
        winit::keyboard::Key::Alt => KbKey::Alt,
        winit::keyboard::Key::AltGraph => KbKey::AltGraph,
        winit::keyboard::Key::CapsLock => KbKey::CapsLock,
        winit::keyboard::Key::Control => KbKey::Control,
        winit::keyboard::Key::Fn => KbKey::Fn,
        winit::keyboard::Key::FnLock => KbKey::FnLock,
        winit::keyboard::Key::NumLock => KbKey::NumLock,
        winit::keyboard::Key::ScrollLock => KbKey::ScrollLock,
        winit::keyboard::Key::Shift => KbKey::Shift,
        winit::keyboard::Key::Symbol => KbKey::Symbol,
        winit::keyboard::Key::SymbolLock => KbKey::SymbolLock,
        winit::keyboard::Key::Hyper => KbKey::Hyper,
        winit::keyboard::Key::Super => KbKey::Super,
        winit::keyboard::Key::Enter => KbKey::Enter,
        winit::keyboard::Key::Tab => KbKey::Tab,
        winit::keyboard::Key::Space => KbKey::Character(" ".to_string()),
        winit::keyboard::Key::ArrowDown => KbKey::ArrowDown,
        winit::keyboard::Key::ArrowLeft => KbKey::ArrowLeft,
        winit::keyboard::Key::ArrowRight => KbKey::ArrowRight,
        winit::keyboard::Key::ArrowUp => KbKey::ArrowUp,
        winit::keyboard::Key::End => KbKey::End,
        winit::keyboard::Key::Home => KbKey::Home,
        winit::keyboard::Key::PageDown => KbKey::PageDown,
        winit::keyboard::Key::PageUp => KbKey::PageUp,
        winit::keyboard::Key::Backspace => KbKey::Backspace,
        winit::keyboard::Key::Clear => KbKey::Clear,
        winit::keyboard::Key::Copy => KbKey::Copy,
        winit::keyboard::Key::CrSel => KbKey::CrSel,
        winit::keyboard::Key::Cut => KbKey::Cut,
        winit::keyboard::Key::Delete => KbKey::Delete,
        winit::keyboard::Key::EraseEof => KbKey::EraseEof,
        winit::keyboard::Key::ExSel => KbKey::ExSel,
        winit::keyboard::Key::Insert => KbKey::Insert,
        winit::keyboard::Key::Paste => KbKey::Paste,
        winit::keyboard::Key::Redo => KbKey::Redo,
        winit::keyboard::Key::Undo => KbKey::Undo,
        winit::keyboard::Key::Accept => KbKey::Accept,
        winit::keyboard::Key::Again => KbKey::Again,
        winit::keyboard::Key::Attn => KbKey::Attn,
        winit::keyboard::Key::Cancel => KbKey::Cancel,
        winit::keyboard::Key::ContextMenu => KbKey::ContextMenu,
        winit::keyboard::Key::Escape => KbKey::Escape,
        winit::keyboard::Key::Execute => KbKey::Execute,
        winit::keyboard::Key::Find => KbKey::Find,
        winit::keyboard::Key::Help => KbKey::Help,
        winit::keyboard::Key::Pause => KbKey::Pause,
        winit::keyboard::Key::Play => KbKey::Play,
        winit::keyboard::Key::Props => KbKey::Props,
        winit::keyboard::Key::Select => KbKey::Select,
        winit::keyboard::Key::ZoomIn => KbKey::ZoomIn,
        winit::keyboard::Key::ZoomOut => KbKey::ZoomOut,
        winit::keyboard::Key::BrightnessDown => KbKey::BrightnessDown,
        winit::keyboard::Key::BrightnessUp => KbKey::BrightnessUp,
        winit::keyboard::Key::Eject => KbKey::Eject,
        winit::keyboard::Key::LogOff => KbKey::LogOff,
        winit::keyboard::Key::Power => KbKey::Power,
        winit::keyboard::Key::PowerOff => KbKey::PowerOff,
        winit::keyboard::Key::PrintScreen => KbKey::PrintScreen,
        winit::keyboard::Key::Hibernate => KbKey::Hibernate,
        winit::keyboard::Key::Standby => KbKey::Standby,
        winit::keyboard::Key::WakeUp => KbKey::WakeUp,
        winit::keyboard::Key::AllCandidates => KbKey::AllCandidates,
        winit::keyboard::Key::Alphanumeric => KbKey::Alphanumeric,
        winit::keyboard::Key::CodeInput => KbKey::CodeInput,
        winit::keyboard::Key::Compose => KbKey::Compose,
        winit::keyboard::Key::Convert => KbKey::Convert,
        winit::keyboard::Key::FinalMode => KbKey::FinalMode,
        winit::keyboard::Key::GroupFirst => KbKey::GroupFirst,
        winit::keyboard::Key::GroupLast => KbKey::GroupLast,
        winit::keyboard::Key::GroupNext => KbKey::GroupNext,
        winit::keyboard::Key::GroupPrevious => KbKey::GroupPrevious,
        winit::keyboard::Key::ModeChange => KbKey::ModeChange,
        winit::keyboard::Key::NextCandidate => KbKey::NextCandidate,
        winit::keyboard::Key::NonConvert => KbKey::NonConvert,
        winit::keyboard::Key::PreviousCandidate => KbKey::PreviousCandidate,
        winit::keyboard::Key::Process => KbKey::Process,
        winit::keyboard::Key::SingleCandidate => KbKey::SingleCandidate,
        winit::keyboard::Key::HangulMode => KbKey::HangulMode,
        winit::keyboard::Key::HanjaMode => KbKey::HanjaMode,
        winit::keyboard::Key::JunjaMode => KbKey::JunjaMode,
        winit::keyboard::Key::Eisu => KbKey::Eisu,
        winit::keyboard::Key::Hankaku => KbKey::Hankaku,
        winit::keyboard::Key::Hiragana => KbKey::Hiragana,
        winit::keyboard::Key::HiraganaKatakana => KbKey::HiraganaKatakana,
        winit::keyboard::Key::KanaMode => KbKey::KanaMode,
        winit::keyboard::Key::KanjiMode => KbKey::KanjiMode,
        winit::keyboard::Key::Katakana => KbKey::Katakana,
        winit::keyboard::Key::Romaji => KbKey::Romaji,
        winit::keyboard::Key::Zenkaku => KbKey::Zenkaku,
        winit::keyboard::Key::ZenkakuHankaku => KbKey::ZenkakuHankaku,
        winit::keyboard::Key::Soft1 => KbKey::Soft1,
        winit::keyboard::Key::Soft2 => KbKey::Soft2,
        winit::keyboard::Key::Soft3 => KbKey::Soft3,
        winit::keyboard::Key::Soft4 => KbKey::Soft4,
        winit::keyboard::Key::ChannelDown => KbKey::ChannelDown,
        winit::keyboard::Key::ChannelUp => KbKey::ChannelUp,
        winit::keyboard::Key::Close => KbKey::Close,
        winit::keyboard::Key::MailForward => KbKey::MailForward,
        winit::keyboard::Key::MailReply => KbKey::MailReply,
        winit::keyboard::Key::MailSend => KbKey::MailSend,
        winit::keyboard::Key::MediaClose => KbKey::MediaClose,
        winit::keyboard::Key::MediaFastForward => KbKey::MediaFastForward,
        winit::keyboard::Key::MediaPause => KbKey::MediaPause,
        winit::keyboard::Key::MediaPlay => KbKey::MediaPlay,
        winit::keyboard::Key::MediaPlayPause => KbKey::MediaPlayPause,
        winit::keyboard::Key::MediaRecord => KbKey::MediaRecord,
        winit::keyboard::Key::MediaRewind => KbKey::MediaRewind,
        winit::keyboard::Key::MediaStop => KbKey::MediaStop,
        winit::keyboard::Key::MediaTrackNext => KbKey::MediaTrackNext,
        winit::keyboard::Key::MediaTrackPrevious => KbKey::MediaTrackPrevious,
        winit::keyboard::Key::New => KbKey::New,
        winit::keyboard::Key::Open => KbKey::Open,
        winit::keyboard::Key::Print => KbKey::Print,
        winit::keyboard::Key::Save => KbKey::Save,
        winit::keyboard::Key::SpellCheck => KbKey::SpellCheck,
        winit::keyboard::Key::Key11 => KbKey::Key11,
        winit::keyboard::Key::Key12 => KbKey::Key12,
        winit::keyboard::Key::AudioBalanceLeft => KbKey::AudioBalanceLeft,
        winit::keyboard::Key::AudioBalanceRight => KbKey::AudioBalanceRight,
        winit::keyboard::Key::AudioBassBoostDown => KbKey::AudioBassBoostDown,
        winit::keyboard::Key::AudioBassBoostToggle => KbKey::AudioBassBoostToggle,
        winit::keyboard::Key::AudioBassBoostUp => KbKey::AudioBassBoostUp,
        winit::keyboard::Key::AudioFaderFront => KbKey::AudioFaderFront,
        winit::keyboard::Key::AudioFaderRear => KbKey::AudioFaderRear,
        winit::keyboard::Key::AudioSurroundModeNext => KbKey::AudioSurroundModeNext,
        winit::keyboard::Key::AudioTrebleDown => KbKey::AudioTrebleDown,
        winit::keyboard::Key::AudioTrebleUp => KbKey::AudioTrebleUp,
        winit::keyboard::Key::AudioVolumeDown => KbKey::AudioVolumeDown,
        winit::keyboard::Key::AudioVolumeUp => KbKey::AudioVolumeUp,
        winit::keyboard::Key::AudioVolumeMute => KbKey::AudioVolumeMute,
        winit::keyboard::Key::MicrophoneToggle => KbKey::MicrophoneToggle,
        winit::keyboard::Key::MicrophoneVolumeDown => KbKey::MicrophoneVolumeDown,
        winit::keyboard::Key::MicrophoneVolumeUp => KbKey::MicrophoneVolumeUp,
        winit::keyboard::Key::MicrophoneVolumeMute => KbKey::MicrophoneVolumeMute,
        winit::keyboard::Key::SpeechCorrectionList => KbKey::SpeechCorrectionList,
        winit::keyboard::Key::SpeechInputToggle => KbKey::SpeechInputToggle,
        winit::keyboard::Key::LaunchApplication1 => KbKey::LaunchApplication1,
        winit::keyboard::Key::LaunchApplication2 => KbKey::LaunchApplication2,
        winit::keyboard::Key::LaunchCalendar => KbKey::LaunchCalendar,
        winit::keyboard::Key::LaunchContacts => KbKey::LaunchContacts,
        winit::keyboard::Key::LaunchMail => KbKey::LaunchMail,
        winit::keyboard::Key::LaunchMediaPlayer => KbKey::LaunchMediaPlayer,
        winit::keyboard::Key::LaunchMusicPlayer => KbKey::LaunchMusicPlayer,
        winit::keyboard::Key::LaunchPhone => KbKey::LaunchPhone,
        winit::keyboard::Key::LaunchScreenSaver => KbKey::LaunchScreenSaver,
        winit::keyboard::Key::LaunchSpreadsheet => KbKey::LaunchSpreadsheet,
        winit::keyboard::Key::LaunchWebBrowser => KbKey::LaunchWebBrowser,
        winit::keyboard::Key::LaunchWebCam => KbKey::LaunchWebCam,
        winit::keyboard::Key::LaunchWordProcessor => KbKey::LaunchWordProcessor,
        winit::keyboard::Key::BrowserBack => KbKey::BrowserBack,
        winit::keyboard::Key::BrowserFavorites => KbKey::BrowserFavorites,
        winit::keyboard::Key::BrowserForward => KbKey::BrowserForward,
        winit::keyboard::Key::BrowserHome => KbKey::BrowserHome,
        winit::keyboard::Key::BrowserRefresh => KbKey::BrowserRefresh,
        winit::keyboard::Key::BrowserSearch => KbKey::BrowserSearch,
        winit::keyboard::Key::BrowserStop => KbKey::BrowserStop,
        winit::keyboard::Key::AppSwitch => KbKey::AppSwitch,
        winit::keyboard::Key::Call => KbKey::Call,
        winit::keyboard::Key::Camera => KbKey::Camera,
        winit::keyboard::Key::CameraFocus => KbKey::CameraFocus,
        winit::keyboard::Key::EndCall => KbKey::EndCall,
        winit::keyboard::Key::GoBack => KbKey::GoBack,
        winit::keyboard::Key::GoHome => KbKey::GoHome,
        winit::keyboard::Key::HeadsetHook => KbKey::HeadsetHook,
        winit::keyboard::Key::LastNumberRedial => KbKey::LastNumberRedial,
        winit::keyboard::Key::Notification => KbKey::Notification,
        winit::keyboard::Key::MannerMode => KbKey::MannerMode,
        winit::keyboard::Key::VoiceDial => KbKey::VoiceDial,
        winit::keyboard::Key::TV => KbKey::TV,
        winit::keyboard::Key::TV3DMode => KbKey::TV3DMode,
        winit::keyboard::Key::TVAntennaCable => KbKey::TVAntennaCable,
        winit::keyboard::Key::TVAudioDescription => KbKey::TVAudioDescription,
        winit::keyboard::Key::TVAudioDescriptionMixDown => KbKey::TVAudioDescriptionMixDown,
        winit::keyboard::Key::TVAudioDescriptionMixUp => KbKey::TVAudioDescriptionMixUp,
        winit::keyboard::Key::TVContentsMenu => KbKey::TVContentsMenu,
        winit::keyboard::Key::TVDataService => KbKey::TVDataService,
        winit::keyboard::Key::TVInput => KbKey::TVInput,
        winit::keyboard::Key::TVInputComponent1 => KbKey::TVInputComponent1,
        winit::keyboard::Key::TVInputComponent2 => KbKey::TVInputComponent2,
        winit::keyboard::Key::TVInputComposite1 => KbKey::TVInputComposite1,
        winit::keyboard::Key::TVInputComposite2 => KbKey::TVInputComposite2,
        winit::keyboard::Key::TVInputHDMI1 => KbKey::TVInputHDMI1,
        winit::keyboard::Key::TVInputHDMI2 => KbKey::TVInputHDMI2,
        winit::keyboard::Key::TVInputHDMI3 => KbKey::TVInputHDMI3,
        winit::keyboard::Key::TVInputHDMI4 => KbKey::TVInputHDMI4,
        winit::keyboard::Key::TVInputVGA1 => KbKey::TVInputVGA1,
        winit::keyboard::Key::TVMediaContext => KbKey::TVMediaContext,
        winit::keyboard::Key::TVNetwork => KbKey::TVNetwork,
        winit::keyboard::Key::TVNumberEntry => KbKey::TVNumberEntry,
        winit::keyboard::Key::TVPower => KbKey::TVPower,
        winit::keyboard::Key::TVRadioService => KbKey::TVRadioService,
        winit::keyboard::Key::TVSatellite => KbKey::TVSatellite,
        winit::keyboard::Key::TVSatelliteBS => KbKey::TVSatelliteBS,
        winit::keyboard::Key::TVSatelliteCS => KbKey::TVSatelliteCS,
        winit::keyboard::Key::TVSatelliteToggle => KbKey::TVSatelliteToggle,
        winit::keyboard::Key::TVTerrestrialAnalog => KbKey::TVTerrestrialAnalog,
        winit::keyboard::Key::TVTerrestrialDigital => KbKey::TVTerrestrialDigital,
        winit::keyboard::Key::TVTimer => KbKey::TVTimer,
        winit::keyboard::Key::AVRInput => KbKey::AVRInput,
        winit::keyboard::Key::AVRPower => KbKey::AVRPower,
        winit::keyboard::Key::ColorF0Red => KbKey::ColorF0Red,
        winit::keyboard::Key::ColorF1Green => KbKey::ColorF1Green,
        winit::keyboard::Key::ColorF2Yellow => KbKey::ColorF2Yellow,
        winit::keyboard::Key::ColorF3Blue => KbKey::ColorF3Blue,
        winit::keyboard::Key::ColorF4Grey => KbKey::ColorF4Grey,
        winit::keyboard::Key::ColorF5Brown => KbKey::ColorF5Brown,
        winit::keyboard::Key::ClosedCaptionToggle => KbKey::ClosedCaptionToggle,
        winit::keyboard::Key::Dimmer => KbKey::Dimmer,
        winit::keyboard::Key::DisplaySwap => KbKey::DisplaySwap,
        winit::keyboard::Key::DVR => KbKey::DVR,
        winit::keyboard::Key::Exit => KbKey::Exit,
        winit::keyboard::Key::FavoriteClear0 => KbKey::FavoriteClear0,
        winit::keyboard::Key::FavoriteClear1 => KbKey::FavoriteClear1,
        winit::keyboard::Key::FavoriteClear2 => KbKey::FavoriteClear2,
        winit::keyboard::Key::FavoriteClear3 => KbKey::FavoriteClear3,
        winit::keyboard::Key::FavoriteRecall0 => KbKey::FavoriteRecall0,
        winit::keyboard::Key::FavoriteRecall1 => KbKey::FavoriteRecall1,
        winit::keyboard::Key::FavoriteRecall2 => KbKey::FavoriteRecall2,
        winit::keyboard::Key::FavoriteRecall3 => KbKey::FavoriteRecall3,
        winit::keyboard::Key::FavoriteStore0 => KbKey::FavoriteStore0,
        winit::keyboard::Key::FavoriteStore1 => KbKey::FavoriteStore1,
        winit::keyboard::Key::FavoriteStore2 => KbKey::FavoriteStore2,
        winit::keyboard::Key::FavoriteStore3 => KbKey::FavoriteStore3,
        winit::keyboard::Key::Guide => KbKey::Guide,
        winit::keyboard::Key::GuideNextDay => KbKey::GuideNextDay,
        winit::keyboard::Key::GuidePreviousDay => KbKey::GuidePreviousDay,
        winit::keyboard::Key::Info => KbKey::Info,
        winit::keyboard::Key::InstantReplay => KbKey::InstantReplay,
        winit::keyboard::Key::Link => KbKey::Link,
        winit::keyboard::Key::ListProgram => KbKey::ListProgram,
        winit::keyboard::Key::LiveContent => KbKey::LiveContent,
        winit::keyboard::Key::Lock => KbKey::Lock,
        winit::keyboard::Key::MediaApps => KbKey::MediaApps,
        winit::keyboard::Key::MediaAudioTrack => KbKey::MediaAudioTrack,
        winit::keyboard::Key::MediaLast => KbKey::MediaLast,
        winit::keyboard::Key::MediaSkipBackward => KbKey::MediaSkipBackward,
        winit::keyboard::Key::MediaSkipForward => KbKey::MediaSkipForward,
        winit::keyboard::Key::MediaStepBackward => KbKey::MediaStepBackward,
        winit::keyboard::Key::MediaStepForward => KbKey::MediaStepForward,
        winit::keyboard::Key::MediaTopMenu => KbKey::MediaTopMenu,
        winit::keyboard::Key::NavigateIn => KbKey::NavigateIn,
        winit::keyboard::Key::NavigateNext => KbKey::NavigateNext,
        winit::keyboard::Key::NavigateOut => KbKey::NavigateOut,
        winit::keyboard::Key::NavigatePrevious => KbKey::NavigatePrevious,
        winit::keyboard::Key::NextFavoriteChannel => KbKey::NextFavoriteChannel,
        winit::keyboard::Key::NextUserProfile => KbKey::NextUserProfile,
        winit::keyboard::Key::OnDemand => KbKey::OnDemand,
        winit::keyboard::Key::Pairing => KbKey::Pairing,
        winit::keyboard::Key::PinPDown => KbKey::PinPDown,
        winit::keyboard::Key::PinPMove => KbKey::PinPMove,
        winit::keyboard::Key::PinPToggle => KbKey::PinPToggle,
        winit::keyboard::Key::PinPUp => KbKey::PinPUp,
        winit::keyboard::Key::PlaySpeedDown => KbKey::PlaySpeedDown,
        winit::keyboard::Key::PlaySpeedReset => KbKey::PlaySpeedReset,
        winit::keyboard::Key::PlaySpeedUp => KbKey::PlaySpeedUp,
        winit::keyboard::Key::RandomToggle => KbKey::RandomToggle,
        winit::keyboard::Key::RcLowBattery => KbKey::RcLowBattery,
        winit::keyboard::Key::RecordSpeedNext => KbKey::RecordSpeedNext,
        winit::keyboard::Key::RfBypass => KbKey::RfBypass,
        winit::keyboard::Key::ScanChannelsToggle => KbKey::ScanChannelsToggle,
        winit::keyboard::Key::ScreenModeNext => KbKey::ScreenModeNext,
        winit::keyboard::Key::Settings => KbKey::Settings,
        winit::keyboard::Key::SplitScreenToggle => KbKey::SplitScreenToggle,
        winit::keyboard::Key::STBInput => KbKey::STBInput,
        winit::keyboard::Key::STBPower => KbKey::STBPower,
        winit::keyboard::Key::Subtitle => KbKey::Subtitle,
        winit::keyboard::Key::Teletext => KbKey::Teletext,
        winit::keyboard::Key::VideoModeNext => KbKey::VideoModeNext,
        winit::keyboard::Key::Wink => KbKey::Wink,
        winit::keyboard::Key::ZoomToggle => KbKey::ZoomToggle,
        winit::keyboard::Key::F1 => KbKey::F1,
        winit::keyboard::Key::F2 => KbKey::F2,
        winit::keyboard::Key::F3 => KbKey::F3,
        winit::keyboard::Key::F4 => KbKey::F4,
        winit::keyboard::Key::F5 => KbKey::F5,
        winit::keyboard::Key::F6 => KbKey::F6,
        winit::keyboard::Key::F7 => KbKey::F7,
        winit::keyboard::Key::F8 => KbKey::F8,
        winit::keyboard::Key::F9 => KbKey::F9,
        winit::keyboard::Key::F10 => KbKey::F10,
        winit::keyboard::Key::F11 => KbKey::F11,
        winit::keyboard::Key::F12 => KbKey::F12,
        winit::keyboard::Key::F13 => KbKey::Unidentified,
        winit::keyboard::Key::F14 => KbKey::Unidentified,
        winit::keyboard::Key::F15 => KbKey::Unidentified,
        winit::keyboard::Key::F16 => KbKey::Unidentified,
        winit::keyboard::Key::F17 => KbKey::Unidentified,
        winit::keyboard::Key::F18 => KbKey::Unidentified,
        winit::keyboard::Key::F19 => KbKey::Unidentified,
        winit::keyboard::Key::F20 => KbKey::Unidentified,
        winit::keyboard::Key::F21 => KbKey::Unidentified,
        winit::keyboard::Key::F22 => KbKey::Unidentified,
        winit::keyboard::Key::F23 => KbKey::Unidentified,
        winit::keyboard::Key::F24 => KbKey::Unidentified,
        winit::keyboard::Key::F25 => KbKey::Unidentified,
        winit::keyboard::Key::F26 => KbKey::Unidentified,
        winit::keyboard::Key::F27 => KbKey::Unidentified,
        winit::keyboard::Key::F28 => KbKey::Unidentified,
        winit::keyboard::Key::F29 => KbKey::Unidentified,
        winit::keyboard::Key::F30 => KbKey::Unidentified,
        winit::keyboard::Key::F31 => KbKey::Unidentified,
        winit::keyboard::Key::F32 => KbKey::Unidentified,
        winit::keyboard::Key::F33 => KbKey::Unidentified,
        winit::keyboard::Key::F34 => KbKey::Unidentified,
        winit::keyboard::Key::F35 => KbKey::Unidentified,
        _ => KbKey::Unidentified,
    }
}
