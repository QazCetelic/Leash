use std::collections::HashMap;

pub(crate) fn mem_info() -> Option<HashMap<String, u64>> {
    let meminfo = std::fs::read_to_string("/proc/meminfo").ok()?;
    let mut map = HashMap::new();
    for line in meminfo.lines() {
        let mut split = line.split(":");
        let key = split.next()?;
        let value_and_unit = split.next()?.trim_start();
        let value_str = value_and_unit.trim_end_matches(" kB");
        let value = value_str.parse::<u64>().ok()?;
        map.insert(String::from(key), value);
    }
    return Some(map);
}