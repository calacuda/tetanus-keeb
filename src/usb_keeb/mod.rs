use crate::keyboard::keyboard::{Key, Keyboard};
use anyhow::Ok;
use log::{info, warn};

mod consumer_control;
mod keyboard;

extern "C" {
    fn usb_util_init();
    fn usb_util_keyboard_report(modifier: u8, keycode: *const u8);
    fn usb_util_consumer_report(code: u16);
}

use consumer_control::ConsumerControlReport;
use keyboard::KeyboardReport;

use crate::keycodes::KeyCode;

pub enum HidReportType {
    KeyPress { key_code: KeyCode },
    KeyRelease { key_code: KeyCode },
}

pub struct HidReport {
    pub keyboard: KeyboardReport<6>,
    consumer_control: ConsumerControlReport,
}

impl HidReport {
    pub fn new() -> Self {
        Self {
            keyboard: KeyboardReport::new(),
            consumer_control: ConsumerControlReport::new(),
        }
    }

    pub fn send(&mut self, report: HidReportType) {
        match report {
            HidReportType::KeyPress { key_code } => match key_code {
                KeyCode::None => (),
                KeyCode::Consumer(code) => self.consumer_control.press(code),
                KeyCode::Key(hid_key) => self.keyboard.press(hid_key),
            },
            HidReportType::KeyRelease { key_code } => match key_code {
                KeyCode::None => (),
                KeyCode::Consumer(_) => self.consumer_control.release(),
                KeyCode::Key(hid_key) => self.keyboard.release(hid_key),
            },
        }
    }

    // pub fn clear(&mut self) {
    //     self.keyboard.clear();
    //     self.consumer_control.clear();
    // }

    pub fn type_char(&mut self, key_code: u8) {
        let dlay = 500;
        self.send(HidReportType::KeyPress {
            key_code: KeyCode::Key(key_code),
        });
        esp_idf_hal::delay::FreeRtos::delay_ms(dlay);
        self.send(HidReportType::KeyRelease {
            key_code: KeyCode::Key(key_code),
        });
    }
}

impl Keyboard for HidReport {
    fn init(&mut self) -> anyhow::Result<()> {
        info!("initializing usb keyboard");
        if crate::INIT_USB {
            unsafe { usb_util_init() }
        } else {
            warn!("Skipping USB  init")
        }

        esp_idf_hal::delay::FreeRtos::delay_ms(1100);
        info!("usb keyboard initialized");

        Ok(())
    }

    fn press(&mut self, key: Key) -> anyhow::Result<()> {
        self.send(HidReportType::KeyPress {
            key_code: KeyCode::Key(key),
        });
        Ok(())
    }

    fn release(&mut self, key: Key) -> anyhow::Result<()> {
        self.send(HidReportType::KeyRelease {
            key_code: KeyCode::Key(key),
        });
        Ok(())
    }
}
