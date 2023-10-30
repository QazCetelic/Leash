pub fn mhz_to_ghz(mhz: u32) -> f64 {
    mhz as f64 / 1000.0
}

pub fn round_to_100mhz(mhz: u32) -> u32 {
    let remainder = mhz % 100;
    if remainder > 50 {
        mhz + (100 - remainder)
    }
    else {
        mhz - remainder
    }
}

pub fn ghz_to_mhz(ghz: f64) -> u32 {
    (ghz * 1000.0) as u32
}

pub fn khz_to_mhz(khz: u32) -> u32 {
    khz / 1000
}
