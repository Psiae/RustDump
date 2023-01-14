pub fn verse(n: u32) -> String {
    match n {
        0 => format!(
            "No more bottles of beer on the wall, no more bottles of beer.
            Go to the store and buy some more, 99 bottles of beer on the wall."
        ),
        1 => format!(
            "1 bottle of beer on the wall, 1 bottle of beer.
            Take it down and pass it around, no more bottles of beer on the wall."
        ),
        2 => format!(
            "2 bottles of beer on the wall, 2 bottles of beer.
            Take one down and pass it around, 1 bottle of beer on the wall."
        ),
        _ => format!(
            "{0} bottles of beer on the wall, {0} bottles of beer.
            Take one down and pass it around, {1} bottles of beer on the wall.",
            n, n - 1
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    if start == end {
     return verse(end)
    }
    let next = sing(start - 1, end);
    return verse(start) + "\n" + next.as_str()
}
