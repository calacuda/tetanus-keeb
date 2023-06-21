/// a state machine that stores the state of the keyboards lights
pub struct LEDsState {
    // led_pin: Gpio
    // effect:  // the selected lighting effect. is a state machine that implements the "LightingEffect" trait
}


impl LEDsState {
    pub fn new() -> Self {
        Self { }
    }

    pub fn step(&mut self) {
        // TODO make interchangeable lighting effects and call the step function from here
        // TODO make a lighting effect trait that can be implemented for different state machines, that way the state machine
    }
}