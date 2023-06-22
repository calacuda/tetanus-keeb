use crate::keyboard::keyboard::Key;
use crate::keyboard::keeb_periph::{N_COLS, N_ROWS};


pub trait Layout {
    /// takes a list of [row, column] coordinates and returns the keycode to send to the computer.
    fn get_key(&mut self, keys: &[(usize, usize)]) -> Vec<Key>;

    /// returns true if the "fn" key is pressed.
    fn fn_key_pressed(&mut self, pressed: Vec<(usize, usize)>) -> bool; 
}

pub struct DefaultLayout {
    layout: Vec<[[Option<Key>; N_COLS]; N_ROWS]>,
    fn_loc: Vec<(usize, usize)>  // all locations of "fn" keys  
}

impl DefaultLayout {
    pub fn new() -> Self {
        // TODO: make layout
        Self {
            layout: vec!(
                [
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None]
                ],
                [
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None],
                    [None, None, None, None, None, None, None, None, None, None, None, None, None, None]
                ]
            ),
            fn_loc: vec![(0, 0)]
        }
    }

    fn get_layer(&mut self, pressed: &[(usize, usize)]) -> usize {
        pressed
            .into_iter()
            .map(|loc| 
                if self.fn_loc.contains(&loc) { 1 } else { 0 }
            )
            .sum()
    }
}

impl Layout for DefaultLayout {
    fn get_key(&mut self, keys: &[(usize, usize)]) -> Vec<Key> {
        let layer = self.get_layer(keys);
        
        let mut pressed = Vec::with_capacity(6);
        
        for (row_i, col_i) in keys {
            if let Some(key_code) = self.layout[layer][*row_i][*col_i] {
                pressed.push(key_code);
            } else if let Some(key_code) = self.layout[0][*row_i][*col_i] {
                pressed.push(key_code);
            }
        }

        pressed
    }

    fn fn_key_pressed(&mut self, pressed: Vec<(usize, usize)>) -> bool {
        self.get_layer(&pressed) > 0
    }
}