use crate::script::api::LuaApiTable;
use codegen::Event;
use mlua::prelude::*;
use winit::event::VirtualKeyCode;

fn keycode_to_str(keycode: VirtualKeyCode) -> &'static str {
    match keycode {
        VirtualKeyCode::Key1 => "key1",
        VirtualKeyCode::Key2 => "key2",
        VirtualKeyCode::Key3 => "key3",
        VirtualKeyCode::Key4 => "key4",
        VirtualKeyCode::Key5 => "key5",
        VirtualKeyCode::Key6 => "key6",
        VirtualKeyCode::Key7 => "key7",
        VirtualKeyCode::Key8 => "key8",
        VirtualKeyCode::Key9 => "key9",
        VirtualKeyCode::Key0 => "key0",
        VirtualKeyCode::A => "a",
        VirtualKeyCode::B => "b",
        VirtualKeyCode::C => "c",
        VirtualKeyCode::D => "d",
        VirtualKeyCode::E => "e",
        VirtualKeyCode::F => "f",
        VirtualKeyCode::G => "g",
        VirtualKeyCode::H => "h",
        VirtualKeyCode::I => "i",
        VirtualKeyCode::J => "j",
        VirtualKeyCode::K => "k",
        VirtualKeyCode::L => "l",
        VirtualKeyCode::M => "m",
        VirtualKeyCode::N => "n",
        VirtualKeyCode::O => "o",
        VirtualKeyCode::P => "p",
        VirtualKeyCode::Q => "q",
        VirtualKeyCode::R => "r",
        VirtualKeyCode::S => "s",
        VirtualKeyCode::T => "t",
        VirtualKeyCode::U => "u",
        VirtualKeyCode::V => "v",
        VirtualKeyCode::W => "w",
        VirtualKeyCode::X => "x",
        VirtualKeyCode::Y => "y",
        VirtualKeyCode::Z => "z",
        VirtualKeyCode::Escape => "escape",
        VirtualKeyCode::F1 => "f1",
        VirtualKeyCode::F2 => "f2",
        VirtualKeyCode::F3 => "f3",
        VirtualKeyCode::F4 => "f4",
        VirtualKeyCode::F5 => "f5",
        VirtualKeyCode::F6 => "f6",
        VirtualKeyCode::F7 => "f7",
        VirtualKeyCode::F8 => "f8",
        VirtualKeyCode::F9 => "f9",
        VirtualKeyCode::F10 => "f10",
        VirtualKeyCode::F11 => "f11",
        VirtualKeyCode::F12 => "f12",
        VirtualKeyCode::F13 => "f13",
        VirtualKeyCode::F14 => "f14",
        VirtualKeyCode::F15 => "f15",
        VirtualKeyCode::F16 => "f16",
        VirtualKeyCode::F17 => "f17",
        VirtualKeyCode::F18 => "f18",
        VirtualKeyCode::F19 => "f19",
        VirtualKeyCode::F20 => "f20",
        VirtualKeyCode::F21 => "f21",
        VirtualKeyCode::F22 => "f22",
        VirtualKeyCode::F23 => "f23",
        VirtualKeyCode::F24 => "f24",
        VirtualKeyCode::Snapshot => "snapshot",
        VirtualKeyCode::Scroll => "scroll",
        VirtualKeyCode::Pause => "pause",
        VirtualKeyCode::Insert => "insert",
        VirtualKeyCode::Home => "home",
        VirtualKeyCode::Delete => "delete",
        VirtualKeyCode::End => "end",
        VirtualKeyCode::PageDown => "page_down",
        VirtualKeyCode::PageUp => "page_up",
        VirtualKeyCode::Left => "left",
        VirtualKeyCode::Up => "up",
        VirtualKeyCode::Right => "right",
        VirtualKeyCode::Down => "down",
        VirtualKeyCode::Back => "back",
        VirtualKeyCode::Return => "return",
        VirtualKeyCode::Space => "space",
        VirtualKeyCode::Compose => "compose",
        VirtualKeyCode::Caret => "caret",
        VirtualKeyCode::Numlock => "numlock",
        VirtualKeyCode::Numpad0 => "numpad0",
        VirtualKeyCode::Numpad1 => "numpad1",
        VirtualKeyCode::Numpad2 => "numpad2",
        VirtualKeyCode::Numpad3 => "numpad3",
        VirtualKeyCode::Numpad4 => "numpad4",
        VirtualKeyCode::Numpad5 => "numpad5",
        VirtualKeyCode::Numpad6 => "numpad6",
        VirtualKeyCode::Numpad7 => "numpad7",
        VirtualKeyCode::Numpad8 => "numpad8",
        VirtualKeyCode::Numpad9 => "numpad9",
        VirtualKeyCode::NumpadAdd => "numpad_add",
        VirtualKeyCode::NumpadDivide => "numpad_divide",
        VirtualKeyCode::NumpadDecimal => "numpad_decimal",
        VirtualKeyCode::NumpadComma => "numpad_comma",
        VirtualKeyCode::NumpadEnter => "numpad_enter",
        VirtualKeyCode::NumpadEquals => "numpad_equals",
        VirtualKeyCode::NumpadMultiply => "numpad_multiply",
        VirtualKeyCode::NumpadSubtract => "numpad_subtract",
        VirtualKeyCode::AbntC1 => "abnt_c1",
        VirtualKeyCode::AbntC2 => "abnt_c2",
        VirtualKeyCode::Apostrophe => "apostrophe",
        VirtualKeyCode::Apps => "apps",
        VirtualKeyCode::Asterisk => "asterisk",
        VirtualKeyCode::At => "at",
        VirtualKeyCode::Ax => "ax",
        VirtualKeyCode::Backslash => "backslash",
        VirtualKeyCode::Calculator => "calculator",
        VirtualKeyCode::Capital => "capital",
        VirtualKeyCode::Colon => "colon",
        VirtualKeyCode::Comma => "comma",
        VirtualKeyCode::Convert => "convert",
        VirtualKeyCode::Equals => "equals",
        VirtualKeyCode::Grave => "grave",
        VirtualKeyCode::Kana => "kana",
        VirtualKeyCode::Kanji => "kanji",
        VirtualKeyCode::LAlt => "l_alt",
        VirtualKeyCode::LBracket => "l_bracket",
        VirtualKeyCode::LControl => "l_control",
        VirtualKeyCode::LShift => "l_shift",
        VirtualKeyCode::LWin => "l_win",
        VirtualKeyCode::Mail => "mail",
        VirtualKeyCode::MediaSelect => "media_select",
        VirtualKeyCode::MediaStop => "media_stop",
        VirtualKeyCode::Minus => "minus",
        VirtualKeyCode::Mute => "mute",
        VirtualKeyCode::MyComputer => "my_computer",
        VirtualKeyCode::NavigateForward => "navigate_forward",
        VirtualKeyCode::NavigateBackward => "navigate_backward",
        VirtualKeyCode::NextTrack => "next_track",
        VirtualKeyCode::NoConvert => "no_convert",
        VirtualKeyCode::OEM102 => "oem102",
        VirtualKeyCode::Period => "period",
        VirtualKeyCode::PlayPause => "play_pause",
        VirtualKeyCode::Plus => "plus",
        VirtualKeyCode::Power => "power",
        VirtualKeyCode::PrevTrack => "prev_track",
        VirtualKeyCode::RAlt => "r_alt",
        VirtualKeyCode::RBracket => "r_bracket",
        VirtualKeyCode::RControl => "r_control",
        VirtualKeyCode::RShift => "r_shift",
        VirtualKeyCode::RWin => "r_win",
        VirtualKeyCode::Semicolon => "semicolon",
        VirtualKeyCode::Slash => "slash",
        VirtualKeyCode::Sleep => "sleep",
        VirtualKeyCode::Stop => "stop",
        VirtualKeyCode::Sysrq => "sysrq",
        VirtualKeyCode::Tab => "tab",
        VirtualKeyCode::Underline => "underline",
        VirtualKeyCode::Unlabeled => "unlabeled",
        VirtualKeyCode::VolumeDown => "volume_down",
        VirtualKeyCode::VolumeUp => "volume_up",
        VirtualKeyCode::Wake => "wake",
        VirtualKeyCode::WebBack => "web_back",
        VirtualKeyCode::WebFavorites => "web_favorites",
        VirtualKeyCode::WebForward => "web_forward",
        VirtualKeyCode::WebHome => "web_home",
        VirtualKeyCode::WebRefresh => "web_refresh",
        VirtualKeyCode::WebSearch => "web_search",
        VirtualKeyCode::WebStop => "web_stop",
        VirtualKeyCode::Yen => "yen",
        VirtualKeyCode::Copy => "copy",
        VirtualKeyCode::Paste => "paste",
        VirtualKeyCode::Cut => "cut",
    }
}

#[derive(Event, Debug, Clone, Copy)]
#[event_name("key_down")]
pub struct KeyDown {
    pub key: &'static str,
}

impl KeyDown {
    pub fn from_key(key: VirtualKeyCode) -> Self {
        Self {
            key: keycode_to_str(key),
        }
    }
}

impl LuaApiTable for KeyDown {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for KeyDown {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("key", |_lua, this| Ok(this.key));
    }
}

#[derive(Event, Debug, Clone, Copy)]
#[event_name("key_up")]
pub struct KeyUp {
    pub key: &'static str,
}

impl KeyUp {
    pub fn from_key(key: VirtualKeyCode) -> Self {
        Self {
            key: keycode_to_str(key),
        }
    }
}

impl LuaApiTable for KeyUp {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for KeyUp {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("key", |_lua, this| Ok(this.key));
    }
}

#[derive(Event, Debug, Clone, Copy)]
#[event_name("pointer_enter")]
pub struct PointerEnter;

impl LuaApiTable for PointerEnter {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PointerEnter {}

#[derive(Event, Debug, Clone, Copy)]
#[event_name("pointer_enter")]
pub struct PointerExit;

impl LuaApiTable for PointerExit {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PointerExit {}

#[derive(Event, Debug, Clone, Copy)]
#[event_name("pointer_move")]
pub struct PointerMove {
    pub pointer_x: f64,
    pub pointer_y: f64,
}

impl LuaApiTable for PointerMove {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PointerMove {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("pointer_x", |_lua, this| Ok(this.pointer_x));
        fields.add_field_method_get("pointer_y", |_lua, this| Ok(this.pointer_y));
    }
}

#[derive(Event, Debug, Clone, Copy)]
#[event_name("pointer_down")]
pub struct PointerDown {
    pub button: &'static str,
}

impl LuaApiTable for PointerDown {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PointerDown {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("button", |_lua, this| Ok(this.button));
    }
}

#[derive(Event, Debug, Clone, Copy)]
#[event_name("pointer_up")]
pub struct PointerUp {
    pub button: &'static str,
}

impl LuaApiTable for PointerUp {
    fn create_api_table<'lua>(lua: &'lua Lua) -> LuaResult<LuaTable<'lua>> {
        let table = lua.create_table()?;

        impl_event_listeners!(lua, table);

        Ok(table)
    }
}

impl LuaUserData for PointerUp {
    fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("button", |_lua, this| Ok(this.button));
    }
}
