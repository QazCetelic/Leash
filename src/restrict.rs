use std::process::Command;

pub(crate) fn set_max_freq_ghz(freq: f64) -> Result<(), String> {
    Command::new("pkexec")
        .arg("cpupower")
        .arg("frequency-set")
        .arg("--max")
        .arg(format!("{}GHz", freq))
        .stdout(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("cpupower couldn't start: {}", e))?
        .wait()
        .map_err(|e| format!("cpupower process exited with an error: {}", e))?;

    return Ok(());
}

pub(crate) fn ram_restrict(restrict: bool) -> Result<(), String> {
    let overcommit_memory = if restrict { 2 } else { 0 };
    let overcommit_ratio = if restrict { 95 } else { 50 };
    Command::new("pkexec")
        .arg("sysctl")
        .arg(format!("vm.overcommit_memory={}", overcommit_memory, ))
        .arg(format!("vm.overcommit_ratio={}", overcommit_ratio))
        .stdout(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("sysctl couldn't start: {}", e))?
        .wait()
        .map_err(|e| format!("sysctl process exited with an error: {}", e))?;

    return Ok(());
}

pub(crate) fn ram_is_restricted() -> bool {
    let overcommit_memory = std::fs::read_to_string("/proc/sys/vm/overcommit_memory").expect("Failed to read overcommit_memory");
    overcommit_memory.trim() == "2"
}

#[allow(dead_code)]
pub(crate) fn set_cpu_governor(governor: String) -> Result<(), String> {
    Command::new("pkexec")
        .arg("cpupower")
        .arg("frequency-set")
        .arg("--governor")
        .arg(governor)
        .stdout(std::process::Stdio::null())
        .spawn()
        .map_err(|e| format!("cpupower couldn't start: {}", e))?
        .wait()
        .map_err(|e| format!("cpupower process exited with an error: {}", e))?;

    return Ok(());
}
