use anyhow;
use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::peripherals::Peripherals;
// use keycodes::HID_KEY_SPACE;
use log::*;

use keyboard::{
    KeyboardState,
    // KeysState,
};

// use crate::usb_keeb::{
//     HidReport,
//     HidReportType,
// };

mod keycodes;
mod usb_keeb;
mod loop_tick;
mod keyboard;

use crate::keyboard::keeb_periph::KeebPeriph;

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
    let keeb_Perph = KeebPeriph {
        columns: [
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
            pins.gpio11.into()
        ],
        rows: [
            pins.gpio1.into(),
            pins.gpio2.into(),
            pins.gpio42.into(),
            pins.gpio41.into(),
            pins.gpio40.into(),
            pins.gpio39.into(),
        ],
        ble_toggle_pin: pins.gpio14.into()
    };
    let led_togle_pin = pins.gpio13;
    info!("making keyboard in usb mode");
    let mut keyboard = KeyboardState::new(keeb_Perph);
    // hello_world(&mut hid_keeb);
    keyboard.init();

   
    loop {
        // press_keys(&mut state.keys, &mut keyboard);
        // step_lighting_effect(&mut state);
        // esp_idf_hal::delay::FreeRtos::delay_ms(100);
        // esp_idf_hal::delay::FreeRtos::delay_ms(1000);

        keyboard.step();
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