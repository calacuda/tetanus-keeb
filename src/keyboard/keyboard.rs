use crate::keyboard::blt_keeb::BLKeyboard;
use crate::keyboard::keeb_periph::{KeebPeriph, N_COLS, N_ROWS};
use crate::layout::{DefaultLayout, Layout};
use crate::usb_keeb::HidReport;
use anyhow::Result;
use esp_idf_hal::gpio::{
    AnyIOPin, AnyInputPin, AnyOutputPin, Input, InputPin, Output, PinDriver, Pull,
};
use esp_idf_sys::EspError;
use log::info;

pub type Key = u8;

pub trait Keyboard {
    /// initializes the keyboard
    fn init(&mut self) -> Result<()>;

    /// sends a keypress
    fn press(&mut self, key: Key) -> anyhow::Result<()>;

    /// release a key that has previously been pressed
    fn release(&mut self, key: Key) -> anyhow::Result<()>;
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub enum KeebMode {
    USB,
    BLT,
}

/// a state machine that stores the state of the keyboards key and can send that data
pub struct KeysState<'a> {
    pressed: Vec<Key>, // the key codes that are currently pressed
    pub mode: KeebMode,
    layout: Box<dyn Layout>,
    keeb: Box<dyn Keyboard>,
    pub periphs: KeebPeriph<'a>,
}

impl KeysState<'_> {
    pub fn new(
        cols: [AnyOutputPin; N_COLS],
        rows: [AnyIOPin; N_ROWS],
        led_switch: AnyInputPin,
        ble_switch: AnyInputPin,
    ) -> anyhow::Result<Self> {
        let col_pins: Result<Vec<PinDriver<'_, AnyOutputPin, Output>>, EspError> =
            cols.map(|pin| PinDriver::output(pin)).into_iter().collect();
        let tmp_row_pins: Result<Vec<PinDriver<'_, AnyIOPin, Input>>, EspError> =
            rows.map(|pin| PinDriver::input(pin)).into_iter().collect();
        let led_pin = PinDriver::input(led_switch)?;
        let ble_pin = PinDriver::input(ble_switch)?;

        let mut row_pins = tmp_row_pins?;
        row_pins
            .iter_mut()
            .map(|driver| driver.set_pull(Pull::Down))
            .collect::<Result<Vec<()>, EspError>>()?;

        Ok(Self {
            pressed: Vec::with_capacity(6),
            mode: KeebMode::USB,
            layout: Box::new(DefaultLayout::new()),
            keeb: Box::new(HidReport::new()),
            periphs: KeebPeriph {
                columns: col_pins?,
                rows: row_pins,
                ble_toggle_pin: led_pin,
                led_toggle_pin: ble_pin,
            },
        })
    }

    /// Initializes the keyboard including checking the bluetooth.
    pub fn init(&mut self, bluetooth: bool) -> anyhow::Result<()> {
        if bluetooth {
            self.set_bluetooth();
        }

        // self.periphs.columns[5].set_high()?;
        self.release_all();
        self.keeb.init()?;

        Ok(())
    }

    /// returns true if the bluetooth switch is toggled to the bluetooth setting.
    fn bluetooth_switch(&mut self) -> bool {
        self.periphs.ble_toggle_pin.is_high()
    }

    /// toggles between bluetooth and wired mode only if necessary. this function is idempotent and should be
    /// called before every key scan
    fn set_bluetooth(&mut self) {
        let bluetooth = self.bluetooth_switch();

        if bluetooth && self.mode == KeebMode::USB {
            self.release_all();
            self.mode = KeebMode::BLT;
            self.keeb = Box::new(BLKeyboard::new());
        } else if !bluetooth && self.mode == KeebMode::BLT {
            self.release_all();
            self.mode = KeebMode::USB;
            self.keeb = Box::new(HidReport::new());
            let _ = self.keeb.init();
        }
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        // self.set_bluetooth();
        let mut switches = Vec::with_capacity(10);

        for (c, col) in self.periphs.columns.iter_mut().enumerate() {
            col.set_high().unwrap();
            esp_idf_hal::delay::FreeRtos::delay_ms(1000);
            // esp_idf_hal::delay::FreeRtos::delay_us(10);
            for (r, row) in self.periphs.rows.iter().enumerate() {
                if row.is_high() {
                    // info!("pin ({r}/{c}) was pressed");
                    switches.push((r, c));
                }
                // esp_idf_hal::delay::FreeRtos::delay_us(10);
            }
            col.set_low().unwrap();
        }

        info!("pressed => {}", switches.len());
        let pressed = if switches.len() >= 6 {
            self.layout.get_key(&switches[0..6])
        } else {
            self.layout.get_key(&switches)
        };

        // info!("pressed => {pressed:?}");
        self.trigger_keys(&pressed);
        self.pressed = pressed;

        Ok(())
    }

    /// triggers the needed key presses and releases
    fn trigger_keys(&mut self, pressed: &[Key]) {
        let mut all = self.pressed.clone();
        all.append(&mut pressed.to_vec());

        // release the no longer pressed keys .
        let _ = all
            .into_iter()
            .filter(|key| self.pressed.contains(key) && !pressed.contains(key))
            .map(|key| self.keeb.release(key));

        // press the pressed keys
        let _ = pressed
            .into_iter()
            .filter(|key| !self.pressed.contains(key))
            .map(|key| self.keeb.press(*key));
    }

    /// releases all pressed keys (just to be safe)
    fn release_all(&mut self) {
        let _ = self.pressed.iter().map(|key| self.keeb.release(*key));

        self.pressed = Vec::with_capacity(6);
    }
}
