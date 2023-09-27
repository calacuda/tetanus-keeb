use esp_idf_hal::gpio::{AnyIOPin, AnyInputPin, AnyOutputPin, Input, Output, PinDriver};

pub const N_COLS: usize = 15;
pub const N_ROWS: usize = 6;

pub struct KeebPeriph<'a> {
    pub columns: Vec<PinDriver<'a, AnyOutputPin, Output>>,
    pub rows: Vec<PinDriver<'a, AnyIOPin, Input>>,
    pub ble_toggle_pin: PinDriver<'a, AnyInputPin, Input>,
    pub led_toggle_pin: PinDriver<'a, AnyInputPin, Input>,
}
