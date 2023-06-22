use esp_idf_hal::gpio::{AnyInputPin, AnyOutputPin, PinDriver, Input, Output};

pub const N_COLS: usize = 14;
pub const N_ROWS: usize = 6;

pub struct KeebPeriph<'a> {
    pub columns: Vec<PinDriver<'a, AnyOutputPin, Output>>,
    pub rows: Vec<PinDriver<'a, AnyInputPin, Input>>,
    pub ble_toggle_pin: PinDriver<'a, AnyInputPin, Input>,
    pub led_toggle_pin: PinDriver<'a, AnyInputPin, Input>,
}