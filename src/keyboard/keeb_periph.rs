// use anyhow;
use esp_idf_hal::gpio::{AnyOutputPin, AnyInputPin};
// use esp_idf_hal::prelude::Peripherals;
// use esp_idf_hal::;

pub const N_COLS: usize = 14;
pub const N_ROWS: usize = 6;

pub struct KeebPeriph {
    pub columns: [AnyOutputPin; N_COLS],
    pub rows: [AnyInputPin; N_ROWS],
    pub ble_toggle_pin: AnyInputPin,
}