use esp_idf_svc::hal::gpio::{Gpio39, Input, PinDriver, Pull};

pub struct Button {
    button: PinDriver<'static, Input>,
    last_frame_pressed: bool,
    currently_pressed: bool,
}

impl Button {
    pub fn new(pin: Gpio39<'static>) -> Button {
        Button {
            button: PinDriver::input(pin, Pull::Floating).unwrap(),
            last_frame_pressed: false,
            currently_pressed: false,
        }
    }

    pub fn is_button_just_pressed(&mut self) -> bool {
        self.currently_pressed = self.button.is_low();
        let just_pressed = self.currently_pressed && !self.last_frame_pressed;
        self.last_frame_pressed = self.currently_pressed;
        just_pressed
    }
}
