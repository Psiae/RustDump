pub fn factors(n: u64) -> Vec<u64> {
    if n == 0 {
        panic!()
    }
    if n == 1 {
        return vec![]
    }
    let mut factors: Vec<u64> = vec![];
    let mut remain = n;
    let mut div = 2;
    while remain > 1 {
        if remain % div == 0 {
            factors.push(div);
            remain /= div;
            continue
        }
        div += 1
    }
    return factors
}
