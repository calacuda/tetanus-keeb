// use crate::state_machine;
// use state_machine::KeyboardSate;

pub enum TickAction {
    Nothing,
    UpdateLEDs,
    ScanKeys,
}

pub fn get_tick_action() -> TickAction {
    
    
    TickAction::Nothing
}