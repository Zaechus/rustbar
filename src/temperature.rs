use std::fs;

use crate::{block, colored_block, RED};

pub fn temperature_block() -> String {
    let temperature = temperature();

    if temperature > 80 {
        colored_block(&format!("{}°C", temperature), RED)
    } else {
        block(&format!("{}°C", temperature))
    }
}

fn temperature() -> u32 {
    fs::read_dir("/sys/class/thermal")
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .filter(|x| x.starts_with("thermal_zone"))
        .map(|x| {
            if let Ok(s) = fs::read_to_string(format!("/sys/class/thermal/{}/temp", x)) {
                s.trim().parse::<u32>().unwrap_or_default() / 1000
            } else {
                0
            }
        })
        .max()
        .unwrap()
}
