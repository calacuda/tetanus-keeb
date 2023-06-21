use anyhow;
// use esp_idf_hal::prelude::Peripherals;
use std::collections::HashSet;
use crate::usb_keeb::HidReport;
// use esp_idf_hal::gpio::Gpio29;
use crate::keyboard::blt_keeb::BLKeyboard;
use crate::keyboard::keeb_periph::{
    KeebPeriph,
    N_COLS,
    N_ROWS
};


pub type Key = u8;

pub trait Keyboard {
    /// initializes the keyboard 
    fn init(&mut self) -> anyhow::Result<()>;

    /// sends a keypress
    fn press(&mut self, key: Key) -> anyhow::Result<()>;

    /// release a key that has previously been pressed
    fn release(&mut self, key: Key) -> anyhow::Result<()>;
}

/// returns true if the bluetooth switch is toggled to the bluetooth setting. 
fn bluetooth_switch(periphs: &KeebPeriph) -> bool {
    // TODO: check bluetooth switch status
    false
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum KeyActions {
    Pressed(Key),
    Released(Key),
}

#[derive(PartialEq, Eq, Clone, Hash)]
enum KeebMode {
    USB,
    BLT
}

/// a state machine that stores the state of the keyboards key and can send that data 
pub struct KeysState {
    modifiers: HashSet<Key>,
    pressed: HashSet<Key>,
    mode: KeebMode,
    // blt_keeb: Option<BLKeyboard>,
    // usb_keeb: HidReport
    keeb: Box<dyn Keyboard>,
    periphs: KeebPeriph,
}

impl KeysState {
    pub fn new(periphs: KeebPeriph) -> Self {
        // Self { 
        //     modifiers: HashSet::new(),
        //     pressed: HashSet::new(),
        //     mode: KeebMode::USB,
        //     blt_keeb: None,
        //     usb_keeb: HidReport::new()
        // }
        Self { 
            modifiers: HashSet::new(),
            pressed: HashSet::new(),
            mode: KeebMode::USB,
            keeb: Box::new(HidReport::new()),
            periphs: periphs
        }
    }

    /// Initializes the keyboard including checking the bluetooth. 
    pub fn init(&mut self) {
        self.set_bluetooth();
        let _ = self.keeb.init();        
    }

    /// toggles between bluetooth and wired mode only if necessary. this function is idempotent and should be
    /// called before every key scan
    fn set_bluetooth(&mut self) {
        let bluetooth = bluetooth_switch(&self.periphs);
        
        if bluetooth && self.mode == KeebMode::USB {
            self.mode = KeebMode::BLT;
            self.keeb = Box::new(BLKeyboard::new());
        } else if !bluetooth && self.mode == KeebMode::BLT{
            self.mode = KeebMode::USB;
            self.keeb = Box::new(HidReport::new());
            let _ = self.keeb.init();
        }

        // if bluetooth && self.mode == KeebMode::USB {
        //     self.mode = KeebMode::BLT;
        //     self.blt_keeb = Some(BLKeyboard::new());
        // } else if !bluetooth && self.mode == KeebMode::BLT{
        //     self.mode == KeebMode::USB;
        //     self.blt_keeb = None;
        // }
    }

    pub fn step(&mut self) {
        self.set_bluetooth();
        // TODO: check fn key
        for col in 0..N_COLS {
            // TODO: turn on pin self.periphs.columns[col]
            for row in 0..N_ROWS {
                // TODO: get keycode for key at (row, col)
                // TODO: read val of self.periphs.rows[row]
                // TODO: get key at location
                // TODO: if high && key in self.pressed, keep key in self.pressed
                // TODO: if high && key not in self.pressed, send press for and add key to self.pressed
                // TODO: if low && key in self.pressed, send release for key and remove key from self.pressed
                // TODO: if low && key not in self.pressed, do nothing
            }
        }
    }

    // // done (in theory)
    // pub fn update(&mut self) -> HashSet<KeyActions> {
    //     // let mut changes = Vec::with_capacity(6);
    //     let mut changes: HashSet<KeyActions> = HashSet::with_capacity(6);

    //     let new_pressed = self.get_pressed();
        
    //     changes.extend(self.pressed
    //         .iter()
    //         .filter(|key| !new_pressed.contains(key))
    //         .map(|key| KeyActions::Released(*key))
    //         .collect::<HashSet<KeyActions>>()
    //     );
        
    //     changes.extend(
    //         new_pressed
    //         .iter()
    //         .map(|key| KeyActions::Pressed(*key))
    //         .collect::<HashSet<KeyActions>>()            
    //     );

    //     changes
    // }

    // fn get_pressed(&mut self) -> HashSet<Key> {
    //     let mut pressed = HashSet::new();

    //     // TODO add keyboard matrix scan

    //     pressed
    // }

    // /// sends the stored keys to the connected computer
    // pub fn send_keys(&mut self) -> anyhow::Result<()> {
    //     for modifier in &self.modifiers {
    //         self.keeb.press(*modifier)?;
    //     }

    //     for key in &self.keys {
    //         self.keeb.press(*key)?;
    //     }

    //     Ok(())
    // }
}