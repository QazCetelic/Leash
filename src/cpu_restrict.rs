use std::process::Command;

pub fn set_max_freq_ghz(freq: f64) -> bool {
    let mut child = Command::new("pkexec")
        .arg("cpupower")
        .arg("frequency-set")
        .arg("--max")
        .arg(format!("{}GHz", freq))
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("Failed to set max frequency");

    child.wait().is_ok()
}