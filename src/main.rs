use anyhow;
use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::{peripherals::Peripherals, gpio::*};
use log::*;

use keyboard::{
    KeyboardState,
};

use crate::keyboard::keeb_periph::{N_COLS, N_ROWS};

mod keycodes;
mod usb_keeb;
mod loop_tick;
mod keyboard;
mod layout;

pub const INIT_USB: bool = true;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("taking peripherals");
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    info!("peripherals taken");

    // columns for the keyboard matrix.
    let cols: [AnyOutputPin; N_COLS] = [
        pins.gpio4.into(),
        pins.gpio5.into(),
        pins.gpio6.into(),
        pins.gpio7.into(),
        pins.gpio15.into(),
        pins.gpio16.into(),
        pins.gpio17.into(),
        pins.gpio18.into(),
        pins.gpio8.into(),
        pins.gpio3.into(),
        pins.gpio46.into(),
        pins.gpio9.into(),
        pins.gpio10.into(),
        pins.gpio11.into(),
    ];

    // rows for the keyboard matrix
    let rows: [AnyInputPin; N_ROWS] = [
        pins.gpio1.into(),
        pins.gpio2.into(),
        pins.gpio42.into(),
        pins.gpio41.into(),
        pins.gpio40.into(),
        pins.gpio39.into(),
    ];

    // the pin for the bluetooth switch
    let ble_pin = pins.gpio14.into();
    // the pin for the LED back light on/off
    let led_pin = pins.gpio13.into();
    
    // let keeb_periphs = keyboard::keeb_periph::KeebPeriph { 
    //     columns: cols, 
    //     rows: rows, 
    //     ble_toggle_pin: led_pin, 
    //     led_toggle_pin: ble_pin
    // };

    info!("making keyboard in usb mode");
    // let mut keyboard = KeyboardState::new(keeb_periphs)?;
    let mut keyboard = KeyboardState::new(cols, rows, led_pin, ble_pin)?;
    // hello_world(&mut hid_keeb);
    keyboard.init()?;

   
    loop {
        if let Err(e) = keyboard.step() {
            error!("[ERROR] failed to step keyboard state. got error: {e}");
        }
        esp_idf_hal::delay::FreeRtos::delay_ms(1);
    }
}

// fn hello_world(keeb: &mut HidReport) {
    // let mut hid_keeb = HidReport::new(); 
    // info!("initializing usb keyboard");
    // hid_keeb.init();
    // info!("usb keyboard initialized");
//     info!("typing: hello world!");

//     let phrase = [
//         HID_KEY_H, HID_KEY_E, HID_KEY_L, HID_KEY_L, HID_KEY_O, HID_KEY_SPACE,
//         HID_KEY_W, HID_KEY_O, HID_KEY_R, HID_KEY_L, HID_KEY_D
//     ];

//     for key_code in phrase {
//         keeb.type_char(key_code);
//         // esp_idf_hal::delay::FreeRtos::delay_ms(100);
//     }

//     info!("done typing");
// }