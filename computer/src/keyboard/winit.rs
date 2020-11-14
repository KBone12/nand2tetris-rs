use std::collections::VecDeque;

use winit::event::{ElementState, KeyboardInput, ModifiersState, VirtualKeyCode};

use crate::{chip::mem::Register, keyboard::Keyboard, signal::Word};

pub struct WinitKeyboard {
    register: Register<Word>,
    states: VecDeque<VirtualKeyCode>,
    modifiers: ModifiersState,
}

impl Keyboard for WinitKeyboard {
    type State = KeyboardInput;

    fn new() -> Self {
        Self {
            register: Register::new(),
            states: VecDeque::new(),
            modifiers: ModifiersState::empty(),
        }
    }

    fn get_output(&self) -> Word {
        self.register.get_output()
    }

    fn set_state(&mut self, state: Self::State) {
        if let Some(virtual_keycode) = state.virtual_keycode {
            match state.state {
                ElementState::Pressed => {
                    #[allow(deprecated)]
                    self.modifiers.insert(state.modifiers);
                    if let Some(pos) = self.states.iter().position(|&key| key == virtual_keycode) {
                        self.states.remove(pos);
                    }
                    self.states.push_front(virtual_keycode);
                    if let Some(bits) = code_to_word(virtual_keycode) {
                        self.register.tick(true, bits);
                    }
                }
                ElementState::Released => {
                    #[allow(deprecated)]
                    self.modifiers.remove(state.modifiers);
                    if let Some(pos) = self.states.iter().position(|&key| key == virtual_keycode) {
                        self.states.remove(pos);
                        if pos == 0 {
                            if let Some(code) = self.states.pop_front() {
                                if let Some(bits) = code_to_word(code) {
                                    self.register.tick(true, bits);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn code_to_word(keycode: VirtualKeyCode) -> Option<Word> {
    let code = match keycode {
        VirtualKeyCode::Space => 32,
        // Exclamation mark is not supported
        // Double quotation is not supported
        // Number mark (#) is not supported
        // Percentage is not supported
        // Ampersand is not supported
        // Single quotation is not supported
        // Parentheses is not supported
        VirtualKeyCode::Asterisk => 42,
        VirtualKeyCode::Plus => 43,
        VirtualKeyCode::Comma | VirtualKeyCode::NumpadComma => 44,
        VirtualKeyCode::Minus => 45,
        VirtualKeyCode::Period => 46,
        VirtualKeyCode::Slash => 47,
        VirtualKeyCode::Key0 | VirtualKeyCode::Numpad0 => 48,
        VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => 49,
        VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2 => 50,
        VirtualKeyCode::Key3 | VirtualKeyCode::Numpad3 => 51,
        VirtualKeyCode::Key4 | VirtualKeyCode::Numpad4 => 52,
        VirtualKeyCode::Key5 | VirtualKeyCode::Numpad5 => 53,
        VirtualKeyCode::Key6 | VirtualKeyCode::Numpad6 => 54,
        VirtualKeyCode::Key7 | VirtualKeyCode::Numpad7 => 55,
        VirtualKeyCode::Key8 | VirtualKeyCode::Numpad8 => 56,
        VirtualKeyCode::Key9 | VirtualKeyCode::Numpad9 => 57,
        VirtualKeyCode::Colon => 58,
        // Double colon is not supported
        // Less than (<) is not supported
        VirtualKeyCode::Equals | VirtualKeyCode::NumpadEquals => 61,
        // Greater than (>) is not supported
        // Question mark is not supported
        VirtualKeyCode::At => 64,
        // Capital alphabets are not supported
        VirtualKeyCode::LBracket => 91,
        // code == 92 is not defined
        VirtualKeyCode::RBracket => 93,
        VirtualKeyCode::Caret => 94,
        VirtualKeyCode::Underline => 95,
        VirtualKeyCode::Backslash => 96,
        VirtualKeyCode::A => 97,
        VirtualKeyCode::B => 98,
        VirtualKeyCode::C => 99,
        VirtualKeyCode::D => 100,
        VirtualKeyCode::E => 101,
        VirtualKeyCode::F => 102,
        VirtualKeyCode::G => 103,
        VirtualKeyCode::H => 104,
        VirtualKeyCode::I => 105,
        VirtualKeyCode::J => 106,
        VirtualKeyCode::K => 107,
        VirtualKeyCode::L => 108,
        VirtualKeyCode::M => 109,
        VirtualKeyCode::N => 110,
        VirtualKeyCode::O => 111,
        VirtualKeyCode::P => 112,
        VirtualKeyCode::Q => 113,
        VirtualKeyCode::R => 114,
        VirtualKeyCode::S => 115,
        VirtualKeyCode::T => 116,
        VirtualKeyCode::U => 117,
        VirtualKeyCode::V => 118,
        VirtualKeyCode::W => 119,
        VirtualKeyCode::X => 120,
        VirtualKeyCode::Y => 121,
        VirtualKeyCode::Z => 122,
        // Braces are not supported
        // Virtical line is not supported
        // Tilda is not supported
        VirtualKeyCode::Return => 128,
        VirtualKeyCode::Back => 129,
        VirtualKeyCode::Left => 130,
        VirtualKeyCode::Up => 131,
        VirtualKeyCode::Right => 132,
        VirtualKeyCode::Down => 132,
        VirtualKeyCode::Home => 134,
        VirtualKeyCode::End => 135,
        VirtualKeyCode::PageUp => 136,
        VirtualKeyCode::PageDown => 137,
        VirtualKeyCode::Insert => 138,
        VirtualKeyCode::Delete => 139,
        VirtualKeyCode::Escape => 140,
        VirtualKeyCode::F1 => 141,
        VirtualKeyCode::F2 => 142,
        VirtualKeyCode::F3 => 143,
        VirtualKeyCode::F4 => 144,
        VirtualKeyCode::F5 => 145,
        VirtualKeyCode::F6 => 146,
        VirtualKeyCode::F7 => 147,
        VirtualKeyCode::F8 => 148,
        VirtualKeyCode::F9 => 149,
        VirtualKeyCode::F10 => 150,
        VirtualKeyCode::F11 => 151,
        VirtualKeyCode::F12 => 152,
        _ => 0,
    };
    if code == 0 {
        None
    } else {
        Some(Word::from(code))
    }
}
