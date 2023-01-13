pub fn production_rate_per_hour(speed: u8) -> f64 {
    return f64::from(speed) * 221f64 * match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => panic!()
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60.0) as u32
}
