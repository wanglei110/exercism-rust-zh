pub fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..n + 1).map(|x| x * x).fold(0, |accum, x| accum + x)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
