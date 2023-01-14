pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for num in 1..=n {
        sum += num
    }
    return sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum = 0;
    for num in 1..=n {
        sum += (num * num)
    }
    return sum
}

pub fn difference(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n)
}

pub fn square_of_sum_fast(n: u32) -> u32 {
    return (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares_fast(n: u32) -> u32 {
    return n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference_fast(n: u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n)
}





