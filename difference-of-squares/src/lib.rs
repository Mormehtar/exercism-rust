fn square(x: u32) -> u32 {
    x * x
}

pub fn square_of_sum(n: u32) -> u32 {
    square((1..).take(n as usize).sum())
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..).take(n as usize).map(square).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
