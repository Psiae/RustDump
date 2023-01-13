
pub fn expected_minutes_in_oven() -> i32 {
    return 40
}

pub fn remaining_minutes_in_oven(elapsed: i32) -> i32 {
    let expect = expected_minutes_in_oven() - elapsed;
    return if expect > 0 { expect } else { 0 }
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    return number_of_layers * 2
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    return preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}