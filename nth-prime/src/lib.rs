struct SievePointer {
    number: u32,
    last_value: usize
}

pub fn nth(n: u32) -> u32 {
    let mut found_numbers: Vec<SievePointer> = vec![
        SievePointer {number: 2, last_value: 2},
        SievePointer {number: 3, last_value: 3}
    ];
    let mut offset = 4;  // 0, 1, 2, 3
    let chunk_size = n.next_power_of_two() as usize;
    while found_numbers.len() <= (n as usize) {
        found_numbers = find_chunk(found_numbers, offset, chunk_size);
        offset += chunk_size;
    }
    return found_numbers[n as usize].number;
}

fn find_chunk(
    mut found_numbers: Vec<SievePointer>,
    offset: usize,
    chunk_size: usize
) -> Vec<SievePointer> {
    let mut sieve: Vec<bool> = Vec::with_capacity(chunk_size);
    sieve.resize(chunk_size, true);
    for prime in &mut found_numbers {
        prick_sieve(prime, &mut sieve, offset);
    }
    for i in 0..sieve.len() {
        if sieve[i] {
            let mut new_prime = SievePointer {number: (i + offset) as u32, last_value: i + offset};
            prick_sieve(&mut new_prime, &mut sieve, offset);
            found_numbers.push(new_prime);
        }
    }
    return found_numbers;
}

fn prick_sieve(mut number: &mut SievePointer, sieve: &mut Vec<bool>, offset: usize) {
    loop {
        let new_value = number.last_value + (number.number as usize);
        let pointer = new_value - offset;
        if pointer >= sieve.len() {
            return;
        }
        sieve[pointer] = false;
        number.last_value = new_value;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_prime() {
        assert_eq!(nth(0), 2);
    }

    #[test]
    fn test_second_prime() {
        assert_eq!(nth(1), 3);
    }

    #[test]
    fn test_sixth_prime() {
        assert_eq!(nth(5), 13);
    }

    #[test]
    fn test_big_prime() { assert_eq!(nth(10000), 104743); }

}