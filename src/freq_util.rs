pub fn mhz_to_ghz(mhz: u32) -> f64 {
    mhz as f64 / 1000.0
}

pub fn round_to_100mhz(frequency: u32) -> u32 {
    let remainder = frequency % 100;
    if remainder > 50 {
        frequency + (100 - remainder)
    }
    else {
        frequency - remainder
    }
}