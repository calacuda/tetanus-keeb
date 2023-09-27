use crate::keyboard::keeb_periph::{N_COLS, N_ROWS};
use crate::keycodes::*;
use crate::usb_keeb::HidReport;
use anyhow;
use esp_idf_hal::{gpio::*, peripherals::Peripherals};
use esp_idf_sys::{self as _}; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use keyboard::KeyboardState;
use log::*;

pub const INIT_USB: bool = true;

mod keyboard;
mod keycodes;
mod layout;
mod loop_tick;
mod usb_keeb;

// pub const INIT_USB: bool = true;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("disabling Watch Dog");
    // WDT OFF
    unsafe {
        esp_idf_sys::esp_task_wdt_delete(esp_idf_sys::xTaskGetIdleTaskHandleForCPU(
            esp_idf_hal::cpu::core() as u32,
        ));
    };
    info!("watch dog disabled");

    info!("taking peripherals");
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    info!("peripherals taken");

    // columns for the keyboard matrix.
    let cols: [AnyOutputPin; N_COLS] = [
        pins.gpio35.into(),
        pins.gpio36.into(),
        pins.gpio37.into(),
        pins.gpio38.into(),
        pins.gpio39.into(),
        pins.gpio40.into(),
        pins.gpio41.into(),
        pins.gpio42.into(),
        pins.gpio2.into(),
        pins.gpio1.into(),
        pins.gpio17.into(),
        pins.gpio18.into(),
        pins.gpio8.into(),
        pins.gpio3.into(),
        pins.gpio48.into(),
    ];

    // rows for the keyboard matrix
    let rows: [AnyIOPin; N_ROWS] = [
        pins.gpio16.downgrade(),
        pins.gpio15.downgrade(),
        pins.gpio7.downgrade(),
        pins.gpio6.downgrade(),
        pins.gpio5.downgrade(),
        pins.gpio4.downgrade(),
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
    let mut keeb = KeyboardState::new(cols, rows, led_pin, ble_pin)?;
    // hello_world();
    keeb.init(false)?;

    for mut pin in keeb.keyboard.periphs.columns {
        pin.set_high()?;
    }

    loop {
        // if let Err(e) = keeb.step() {
        //     error!("[ERROR] failed to step keyboard state. got error: {e}");
        // }
        esp_idf_hal::delay::FreeRtos::delay_ms(1);
        // esp_idf_hal::delay::FreeRtos::delay_us(10); // i BELIEVE that this is less time then
        // "..::delay_ms(1)" bc I think the "_us" means microseconds.
    }
}

// fn hello_world() {
//     use crate::keyboard::keyboard::Keyboard;
//
//     let mut keeb = HidReport::new();
//     info!("initializing usb keyboard");
//     error!("{:?}", keeb.init());
//     info!("usb keyboard initialized");
//     info!("typing: hello world!");
//
//     let phrase = [
//         HID_KEY_H,
//         HID_KEY_E,
//         HID_KEY_L,
//         HID_KEY_L,
//         HID_KEY_O,
//         HID_KEY_SPACE,
//         HID_KEY_W,
//         HID_KEY_O,
//         HID_KEY_R,
//         HID_KEY_L,
//         HID_KEY_D,
//     ];
//
//     for key_code in phrase {
//         // info!("{:?}", key_code);
//         keeb.type_char(key_code);
//         esp_idf_hal::delay::FreeRtos::delay_ms(100);
//     }
//
//     info!("done typing");
// }
