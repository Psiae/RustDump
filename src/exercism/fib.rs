/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    return vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(count);
    for _ in 0..count {
        buffer.push(0u8)
    }
    return buffer
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    return vec![1, 1, 2, 3, 5]
}
