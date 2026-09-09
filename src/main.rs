use esp_idf_svc::hal::i2s::config::*;
use esp_idf_svc::hal::i2s::I2sDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let config = Config::default();
    let clock_conf = PdmRxClkConfig::from_sample_rate_hz(16_000);
    let slot_conf =
        PdmRxSlotConfig::from_bits_per_sample_and_slot_mode(DataBitWidth::Bits16, SlotMode::Mono)
            .slot_mode_mask(SlotMode::Mono, PdmSlotMask::Right);
    let gpio_conf = PdmRxGpioConfig::default();
    let rx_conf = PdmRxConfig::new(config, clock_conf, slot_conf, gpio_conf);

    let peripherals = Peripherals::take().unwrap();
    let mut driver = I2sDriver::new_pdm_rx(
        peripherals.i2s0,
        &rx_conf,
        peripherals.pins.gpio33,
        peripherals.pins.gpio23,
    )
    .unwrap();

    println!("Connected to mic!...");

    let mut buffer = [0u8; 1024];

    driver.rx_enable().unwrap();
    loop {
        driver
            .read(&mut buffer, esp_idf_svc::hal::delay::BLOCK)
            .unwrap();

        let samples: Vec<_> = buffer
            .chunks_exact(2)
            .map(|chunk| i16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();

        // let peak = samples.iter().map(|s| s.abs()).max();

        let rms = rms(&samples);
        if rms >= 1000.0 {
            println!("rms: {}", rms);
        }
    }
}

fn rms(samples: &[i16]) -> f64 {
    if samples.is_empty() {
        return 0.0;
    }

    let sum_sq: f64 = samples.iter().map(|&s| (s as f64) * (s as f64)).sum();

    (sum_sq / samples.len() as f64).sqrt()
}
