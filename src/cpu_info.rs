// Seems to be unaffected by cpulimit of cpupower.

pub fn current_freq_avg_mhz(core_count: u32) -> Option<u32> {
    let freq_sum: u32 = (0..core_count)
        .map(|core| current_freq_core_mhz(core))
        .sum::<Option<u32>>()?;
    Some(freq_sum / core_count)
}

pub fn current_freq_max_mhz(core_count: u32) -> Option<u32> {
    (0..core_count)
        .map(|core| current_max_freq_core_mhz(core))
        .max()
        .flatten()
}

pub fn freq_max_mhz(core_count: u32) -> Option<u32> {
    (0..core_count)
        .map(|core| max_freq_core_mhz(core))
        .max()
        .flatten()
}

pub fn freq_min_mhz(core_count: u32) -> Option<u32> {
    (0..core_count)
        .map(|core| min_freq_core_mhz(core))
        .min()
        .flatten()
}

pub fn max_freq_core_mhz(core: u32) -> Option<u32> {
    core_freq_data_mhz(core, "cpuinfo_max_freq")
}

pub fn min_freq_core_mhz(core: u32) -> Option<u32> {
    core_freq_data_mhz(core, "cpuinfo_min_freq")
}

pub fn current_freq_core_mhz(core: u32) -> Option<u32> {
    core_freq_data_mhz(core, "scaling_cur_freq")
}

#[allow(dead_code)]
pub fn current_min_freq_core_mhz(core: u32) -> Option<u32> {
   core_freq_data_mhz(core, "scaling_min_freq")
}

pub fn current_max_freq_core_mhz(core: u32) -> Option<u32> {
   core_freq_data_mhz(core, "scaling_max_freq")
}

fn core_freq_data_mhz(core: u32, data: &'static str) -> Option<u32> {
    let freq_string = core_data(core, data)?;
    let freq = freq_string.parse::<u32>().ok()? / 1000;
    Some(freq)
}

fn core_data(core: u32, data: &'static str) -> Option<String> {
    let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/{}", core, data);
    let data_string = std::fs::read_to_string(path).ok()?;
    let stripped = data_string.strip_suffix("\n")?;
    Some(stripped.to_string())
}

pub fn core_count() -> Option<u32> {
    let paths = std::fs::read_dir("/sys/devices/system/cpu/").ok()?;

    // Counts all files that start with "cpu" and end with a number. (e.g. cpu0, cpu1, cpu2, etc.)
    let count: usize = paths
        .filter_map(|path| path.ok()?.file_name().into_string().ok())
        .filter(|name| name.starts_with("cpu"))
        .filter(|name| name.strip_prefix("cpu").unwrap().parse::<u32>().is_ok() )
        .count();

    Some(count as u32)
}

pub fn temperature() -> Result<u32, &'static str> {
    let thermal_zones_dir = std::fs::read_dir("/sys/class/thermal/")
        .map_err(|_| "'/sys/class/thermal/' directory not found")?;
    for dir_res in thermal_zones_dir {
        if let Ok(dir) = dir_res {
            if let Ok(name) = dir.file_name().into_string() {
                if name.starts_with("thermal_zone") {
                    let temp: u32 = read_thermal_zone(&name).ok_or("Failed to read thermal zone")?;
                    return Ok(temp);
                }
            }
        }
    }
    Err("Thermal zone directory not found")
}

fn read_thermal_zone(name: &str) -> Option<u32> {
    let zone = name.strip_prefix("thermal_zone")?;
    let zone_path = format!("/sys/class/thermal/thermal_zone{}/", zone);
    let temp_string = std::fs::read_to_string(format!("{}temp", zone_path)).ok()?;
    let temp_string_trimmed = temp_string.strip_suffix("\n")?;
    let temp = temp_string_trimmed.parse::<u32>().ok()? / 1000;
    return Some(temp);
}