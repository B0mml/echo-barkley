#![allow(deprecated)]

use esp_idf_svc::hal::{gpio::Gpio27, rmt::CHANNEL0};
use esp_idf_svc::sys::esp_random;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

pub struct Led {
    led: Ws2812Esp32Rmt<'static>,
    pub is_flashing: bool,
    pub color_hue: u8,
}

impl Led {
    pub fn new(pin: Gpio27<'static>, channel: CHANNEL0<'static>) -> Led {
        Led {
            led: Ws2812Esp32Rmt::new(channel, pin).unwrap(),
            is_flashing: false,
            color_hue: unsafe { esp_random() } as u8,
        }
    }

    pub fn loop_colors(&mut self) {
        let pixels = std::iter::repeat_n(
            hsv2rgb(Hsv {
                hue: self.color_hue,
                sat: 255,
                val: 8,
            }),
            25,
        );

        self.led.write(pixels).unwrap();

        self.color_hue = self.color_hue.wrapping_add(10);
    }

    pub fn turn_off(&mut self) {
        self.led
            .write([RGB8::new(0, 0, 0)].iter().cloned())
            .unwrap();
    }
}
