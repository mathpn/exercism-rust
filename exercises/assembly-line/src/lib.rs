// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let float_speed = speed as f64;
    return match speed {
        1..=4 => float_speed * 221.0,
        5..=8 => float_speed * 221.0 * 0.9,
        _ => float_speed * 221.0 * 0.77,
    };
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return production_rate_per_hour(speed) as u32 / 60;
}
