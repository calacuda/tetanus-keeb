use crate::keyboard::keeb_periph::{N_COLS, N_ROWS};
use crate::keyboard::keyboard::Key;
use crate::keycodes::*;

pub trait Layout {
    /// takes a list of [row, column] coordinates and returns the keycode to send to the computer.
    fn get_key(&mut self, keys: &[(usize, usize)]) -> Vec<Key>;

    /// returns true if the "fn" key is pressed.
    fn fn_key_pressed(&mut self, pressed: Vec<(usize, usize)>) -> bool;
}

pub struct DefaultLayout {
    layout: Vec<[[Option<Key>; N_COLS]; N_ROWS]>,
    fn_loc: Vec<(usize, usize)>, // all locations of "fn" keys
}

impl DefaultLayout {
    pub fn new() -> Self {
        Self {
            layout: vec![
                [
                    [
                        Some(HID_KEY_ESCAPE),
                        Some(HID_KEY_F1),
                        Some(HID_KEY_F2),
                        Some(HID_KEY_F3),
                        Some(HID_KEY_F4),
                        Some(HID_KEY_F5),
                        Some(HID_KEY_F6),
                        Some(HID_KEY_F7),
                        Some(HID_KEY_F8),
                        Some(HID_KEY_F9),
                        Some(HID_KEY_F10),
                        Some(HID_KEY_F11),
                        Some(HID_KEY_F12),
                        Some(HID_KEY_PRINT_SCREEN),
                        None,
                    ],
                    [
                        Some(HID_KEY_GRAVE),
                        Some(HID_KEY_1),
                        Some(HID_KEY_2),
                        Some(HID_KEY_3),
                        Some(HID_KEY_4),
                        Some(HID_KEY_5),
                        Some(HID_KEY_6),
                        Some(HID_KEY_7),
                        Some(HID_KEY_8),
                        Some(HID_KEY_9),
                        Some(HID_KEY_0),
                        Some(HID_KEY_MINUS),
                        Some(HID_KEY_EQUAL),
                        Some(HID_KEY_BACKSPACE),
                        Some(HID_KEY_DELETE),
                    ],
                    [
                        Some(HID_KEY_TAB),
                        Some(HID_KEY_Q),
                        Some(HID_KEY_W),
                        Some(HID_KEY_E),
                        Some(HID_KEY_R),
                        Some(HID_KEY_T),
                        Some(HID_KEY_Y),
                        Some(HID_KEY_U),
                        Some(HID_KEY_I),
                        Some(HID_KEY_O),
                        Some(HID_KEY_P),
                        Some(HID_KEY_BRACKET_LEFT),
                        Some(HID_KEY_BRACKET_RIGHT),
                        Some(HID_KEY_BACKSLASH),
                        Some(HID_KEY_F13),
                    ],
                    [
                        Some(HID_KEY_GUI_RIGHT),
                        Some(HID_KEY_A),
                        Some(HID_KEY_S),
                        Some(HID_KEY_D),
                        Some(HID_KEY_F),
                        Some(HID_KEY_G),
                        Some(HID_KEY_H),
                        Some(HID_KEY_J),
                        Some(HID_KEY_K),
                        Some(HID_KEY_L),
                        Some(HID_KEY_SEMICOLON),
                        Some(HID_KEY_APOSTROPHE),
                        Some(HID_KEY_ENTER),
                        Some(HID_KEY_F14),
                        None,
                    ],
                    [
                        Some(HID_KEY_SHIFT_LEFT),
                        Some(HID_KEY_Z),
                        Some(HID_KEY_X),
                        Some(HID_KEY_C),
                        Some(HID_KEY_V),
                        Some(HID_KEY_B),
                        Some(HID_KEY_N),
                        Some(HID_KEY_M),
                        Some(HID_KEY_COMMA),
                        Some(HID_KEY_PERIOD),
                        Some(HID_KEY_SLASH),
                        Some(HID_KEY_SHIFT_RIGHT),
                        Some(HID_KEY_ARROW_UP),
                        Some(HID_KEY_F15),
                        None,
                    ],
                    [
                        Some(HID_KEY_CONTROL_LEFT),
                        None,
                        Some(HID_KEY_GUI_LEFT),
                        Some(HID_KEY_ALT_LEFT),
                        None,
                        Some(HID_KEY_SPACE),
                        None,
                        None,
                        None,
                        Some(HID_KEY_ALT_RIGHT),
                        None,
                        Some(HID_KEY_CONTROL_RIGHT),
                        Some(HID_KEY_ARROW_LEFT),
                        Some(HID_KEY_ARROW_DOWN),
                        Some(HID_KEY_ARROW_RIGHT),
                    ],
                ],
                [
                    [
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(HID_KEY_NUM_LOCK),
                        None,
                    ],
                    [
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(HID_KEY_INSERT),
                    ],
                    [
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, None,
                    ],
                    [
                        None, None, None, None, None, None, None, None, None, None, None, None,
                        None, None, None,
                    ],
                    [
                        None,
                        None,
                        None,
                        Some(HID_KEY_LOCKING_CAPS_LOCK), // FIXME: may need to be just capslock
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(HID_KEY_PAGE_UP),
                        None,
                        None,
                    ],
                    [
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(HID_KEY_HOME),
                        Some(HID_KEY_PAGE_DOWN),
                        Some(HID_KEY_END),
                    ],
                ],
            ],
            fn_loc: vec![(5, 1), (5, 10)],
        }
    }

    fn get_layer(&mut self, pressed: &[(usize, usize)]) -> usize {
        pressed
            .into_iter()
            .map(|loc| if self.fn_loc.contains(&loc) { 1 } else { 0 })
            .sum()
    }
}

impl Layout for DefaultLayout {
    fn get_key(&mut self, keys: &[(usize, usize)]) -> Vec<Key> {
        let layer = if self.get_layer(keys) > 0 { 1 } else { 0 };

        keys.iter()
            .filter_map(|(ri, ci)| self.layout[layer][*ri][*ci])
            .collect()
    }

    fn fn_key_pressed(&mut self, pressed: Vec<(usize, usize)>) -> bool {
        self.get_layer(&pressed) > 0
    }
}
