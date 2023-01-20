pub fn raindrops(n: u32) -> String {
    let mut result = "".to_owned();
    if n % 3 == 0 {
        result.push_str("Pling")
    }
    if n % 5 == 0 {
        result.push_str("Plang")
    }
    if n % 7 == 0 {
        result.push_str("Plong")
    }
    return if result == "" {
        n.to_string()
    } else {
        result
    }
}
