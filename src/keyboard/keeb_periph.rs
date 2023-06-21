// use anyhow;
use esp_idf_hal::gpio::{Output, Input};

pub const N_COLS: usize = 14;
pub const N_ROWS: usize = 6;

pub struct KeebPeriph {
    pub columns: [Output; N_COLS],
    pub rows: [Input; N_ROWS],
    pub ble_toggle_pin: Input,
}