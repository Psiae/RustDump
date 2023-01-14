pub fn is_leap_year(year: u64) -> bool {
    match year {
        y if y % 400 == 0 => true,
        y if y % 100 == 0 => false,
        y if y % 4 == 0 => true,
        _ => false
    }
}
