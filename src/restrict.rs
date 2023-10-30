use std::process::Command;

pub(crate) fn set_max_freq_ghz(freq: f64) -> bool {
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

pub(crate) fn ram_restrict(restrict: bool) -> bool {
    let overcommit_memory = if restrict { 2 } else { 0 };
    let overcommit_ratio = if restrict { 95 } else { 50 };
    let mut child = Command::new("pkexec")
        .arg("sysctl")
        .arg(format!("vm.overcommit_memory={}", overcommit_memory, ))
        .arg(format!("vm.overcommit_ratio={}", overcommit_ratio))
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("Failed to set RAM restriction");

    child.wait().is_ok()
}

pub(crate) fn ram_is_restricted() -> bool {
    let overcommit_memory = std::fs::read_to_string("/proc/sys/vm/overcommit_memory").expect("Failed to read overcommit_memory");
    overcommit_memory.trim() == "2"
}

#[allow(dead_code)]
pub(crate) fn set_cpu_governor(governor: String) -> bool {
    let mut child = Command::new("pkexec")
        .arg("cpupower")
        .arg("frequency-set")
        .arg("--governor")
        .arg(governor)
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("Failed to set governor");

    child.wait().is_ok()
}
