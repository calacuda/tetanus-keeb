use esp_idf_hal::gpio::{AnyIOPin, AnyInputPin, AnyOutputPin};

pub mod blt_keeb;
pub mod keeb_periph;
pub mod keyboard;
pub mod lights;

use keeb_periph::{N_COLS, N_ROWS};

/// holds the state of the entire keyboard, i.e the state of the keys
/// along with any and all LEDs.
pub struct KeyboardState<'a> {
    pub keyboard: keyboard::KeysState<'a>,
    pub leds: lights::LEDsState,
}

impl KeyboardState<'_> {
    pub fn new(
        cols: [AnyOutputPin; N_COLS],
        rows: [AnyIOPin; N_ROWS],
        led_switch: AnyInputPin,
        ble_switch: AnyInputPin,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            keyboard: keyboard::KeysState::new(cols, rows, led_switch, ble_switch)?,
            leds: lights::LEDsState::new(),
        })
    }

    pub fn init(&mut self, bluetooth: bool) -> anyhow::Result<()> {
        self.keyboard.init(bluetooth)?;

        Ok(())
    }

    pub fn step(&mut self) -> anyhow::Result<()> {
        self.keyboard.step()?;
        self.leds.step();

        Ok(())
    }
}
