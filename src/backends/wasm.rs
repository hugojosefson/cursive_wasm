use cursive_core::backend;
use cursive_core::event::{Event, Key, MouseButton, MouseEvent};
use cursive_core::theme::{Color, ColorPair, Effect};
use cursive_core::Vec2;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(remote = "Key")]
pub enum SerializableKey {
    Enter,
    Tab,
    Backspace,
    Esc,
    Left,
    Right,
    Up,
    Down,
    Ins,
    Del,
    Home,
    End,
    PageUp,
    PageDown,
    PauseBreak,
    NumpadCenter,
    F0,
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
}

#[wasm_bindgen(typescript_custom_section)]
const CURSIVE_BACKEND: &'static str = r#"
interface CursiveBackend {
    print(s: string): void;
    setRaw(raw: boolean): void;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "CursiveBackend")]
    pub type CursiveBackend;

    #[wasm_bindgen(structural, method, js_name = "pollEvent")]
    pub fn poll_event(this: &CursiveBackend) -> String;

    #[wasm_bindgen(structural, method, js_name = "setRaw")]
    pub fn set_raw(this: &CursiveBackend, raw: bool);
}

pub struct Backend {
    // js_backend: CursiveBackend,
}

#[wasm_bindgen]
#[serde(with = "SerializableKey")]
impl Backend {
    // pub fn new(js_backend: CursiveBackend) -> Self {
    //     Self { js_backend }
    // }
    fn hello(&mut self, s: &str) -> Key {
        serde_json::from_str(s).unwrap()
    }
}
//
// impl backend::Backend for Backend {
//     fn poll_event(&mut self) -> Option<Event> {
//         let s = self.js_backend.poll_event();
//         let event: SerializableEvent = serde_json::from_str(&s).unwrap();
//         Some(event.into())
//     }
//
//     fn set_title(&mut self, _title: String) {
//         todo!()
//     }
//
//     fn refresh(&mut self) {
//         todo!()
//     }
//
//     fn has_colors(&self) -> bool {
//         todo!()
//     }
//
//     fn screen_size(&self) -> Vec2 {
//         todo!()
//     }
//
//     fn print_at(&self, _pos: Vec2, _text: &str) {
//         todo!()
//     }
//
//     fn clear(&self, _color: Color) {
//         todo!()
//     }
//
//     fn set_color(&self, color: ColorPair) -> ColorPair {
//         todo!()
//     }
//
//     fn set_effect(&self, _effect: Effect) {
//         todo!()
//     }
//
//     fn unset_effect(&self, _effect: Effect) {
//         todo!()
//     }
//
//     fn name(&self) -> &str {
//         "wasm"
//     }
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(remote = "Event")]
// pub enum SerializableEvent {
//     WindowResize,
//     FocusLost,
//     Refresh,
//     Char(char),
//     CtrlChar(char),
//     AltChar(char),
//     #[serde(with = "SerializableKey")]
//     Key(Key),
//     #[serde(with = "SerializableKey")]
//     Shift(Key),
//     #[serde(with = "SerializableKey")]
//     Alt(Key),
//     #[serde(with = "SerializableKey")]
//     AltShift(Key),
//     #[serde(with = "SerializableKey")]
//     Ctrl(Key),
//     #[serde(with = "SerializableKey")]
//     CtrlShift(Key),
//     #[serde(with = "SerializableKey")]
//     CtrlAlt(Key),
//     Mouse {
//         #[serde(with = "SerializableVec2")]
//         offset: Vec2,
//         #[serde(with = "SerializableVec2")]
//         position: Vec2,
//         #[serde(with = "SerializableMouseEvent")]
//         event: MouseEvent,
//     },
//     Unknown(Vec<u8>),
//     Exit,
// }
//
// impl Into<Event> for SerializableEvent {
//     fn into(self) -> Event {
//         match self {
//             SerializableEvent::WindowResize => Event::WindowResize,
//             SerializableEvent::FocusLost => Event::FocusLost,
//             SerializableEvent::Refresh => Event::Refresh,
//             SerializableEvent::Char(c) => Event::Char(c),
//             SerializableEvent::CtrlChar(c) => Event::CtrlChar(c),
//             SerializableEvent::AltChar(c) => Event::AltChar(c),
//             SerializableEvent::Key(k) => Event::Key(k),
//             SerializableEvent::Shift(k) => Event::Shift(k),
//             SerializableEvent::Alt(k) => Event::Alt(k),
//             SerializableEvent::AltShift(k) => Event::AltShift(k),
//             SerializableEvent::Ctrl(k) => Event::Ctrl(k),
//             SerializableEvent::CtrlShift(k) => Event::CtrlShift(k),
//             SerializableEvent::CtrlAlt(k) => Event::CtrlAlt(k),
//             SerializableEvent::Mouse {
//                 offset,
//                 position,
//                 event,
//             } => Event::Mouse {
//                 offset,
//                 position,
//                 event,
//             },
//             SerializableEvent::Unknown(bytes) => Event::Unknown(bytes),
//             SerializableEvent::Exit => Event::Exit,
//         }
//     }
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(remote = "MouseEvent")]
// enum SerializableMouseEvent {
//     #[serde(with = "SerializableMouseButton")]
//     Press(MouseButton),
//     #[serde(with = "SerializableMouseButton")]
//     Release(MouseButton),
//     #[serde(with = "SerializableMouseButton")]
//     Hold(MouseButton),
//     WheelUp,
//     WheelDown,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(remote = "MouseButton")]
// enum SerializableMouseButton {
//     Left,
//     Middle,
//     Right,
//     Button4,
//     Button5,
//     Other,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(remote = "Vec2")]
// pub struct SerializableVec2 {
//     pub x: usize,
//     pub y: usize,
// }
