use std::collections::hash_set::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .map(move |x| *x)
        .filter(|x| {*x != 0 as u32})
        .map(|x| (x..limit).step_by(x as usize))
        .flatten()
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
