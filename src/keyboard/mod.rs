pub mod keyboard;
pub mod lights;
pub mod keeb_periph;
pub mod blt_keeb;

/// holds the state of teh entire keyboard, i.e the actual keys and the LEDs.
pub struct KeyboardState {
    pub keyboard: keyboard::KeysState,
    pub leds: lights::LEDsState,
}



impl KeyboardState {
    pub fn new(periphs: keeb_periph::KeebPeriph) -> Self {
        Self { 
            keyboard: keyboard::KeysState::new(periphs),
            leds: lights::LEDsState::new()
        }
    }

    pub fn init(&mut self) {
        self.keyboard.init();
    }

    pub fn step(&mut self) {
        // TODO: Update keyboard state
        // let col
        self.keyboard.step();
        self.leds.step();
    }
}