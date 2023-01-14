pub fn is_armstrong_number(num: u64) -> bool {
    let num_str = num.to_string();
    let len = num_str.len() as u32;
    if len == 1 { return true };
    if len == 2 { return false };
    num == num_str.chars()
        .map(|c| (c.to_digit(10).unwrap() as u64).pow(len))
        .sum()
}