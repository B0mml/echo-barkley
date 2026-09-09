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

    // 32ms per block
    let mut buffer = [0u8; 1024];

    driver.rx_enable().unwrap();

    const BARK_THRESHOLD: f32 = 500.0; // TODO! Change back to higher value
    const MIN_BARK_BLOCKS: u32 = 2; // 130ms
    const MAX_BARK_BLOCKS: u32 = 20; // 640ms

    let mut loud_blocks = 0;
    let mut pause_blocks = 0;

    loop {
        println!("loud_blocks {}", loud_blocks);
        driver
            .read(&mut buffer, esp_idf_svc::hal::delay::BLOCK)
            .unwrap();

        let rms = rms(&buffer);
        println!(
            "RMS: {:>5.0} | loud: {:>2} | pause: {:>2}",
            rms, loud_blocks, pause_blocks
        );

        // Stop multi triggering
        if pause_blocks > 0 {
            pause_blocks -= 1;
            continue;
        }

        if rms >= BARK_THRESHOLD {
            loud_blocks += 1;
            continue;
        }
        if loud_blocks >= MIN_BARK_BLOCKS && loud_blocks <= MAX_BARK_BLOCKS {
            println!("BARK DETECTED!!! (DURATION: {} ms)", loud_blocks * 32);
            pause_blocks = 32;
        }

        loud_blocks = 0;
    }
}

fn rms(buffer: &[u8]) -> f32 {
    let sample_count = buffer.len() / 2;
    if sample_count == 0 {
        return 0.0;
    }

    let sum_sq: f32 = buffer
        .chunks_exact(2)
        .map(|chunk| {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]) as f32;
            sample * sample
        })
        .sum();

    (sum_sq / sample_count as f32).sqrt()
}

// fn bark_filter(rms: &[f64]) -> bool {
//     todo!();
// }
