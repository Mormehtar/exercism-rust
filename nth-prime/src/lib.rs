struct SievePointer {
    number: u32,
    last_value: usize
}

struct SieveChunk {
    sieve: Vec<bool>,
    offset: usize
}


impl SieveChunk {
    pub fn new(size: usize, offset: usize) -> SieveChunk {
        let mut sieve: Vec<bool> = Vec::with_capacity(size);
        sieve.resize(size, true);
        SieveChunk {sieve, offset}
    }

    pub fn prick_number(&mut self, mut number: &mut SievePointer) {
        loop {
            let new_value = number.last_value + (number.number as usize);
            let pointer = new_value - self.offset;
            if pointer >= self.sieve.len() {
                break;
            }
            self.sieve[pointer] = false;
            number.last_value = new_value;
        }
    }

    pub fn get_next_prime_from(&self, from: usize) -> Option<usize> {
        let mut start: usize = 0;
        if from > self.offset {
            start = from - self.offset;
        }
        for i in start .. self.sieve.len() {
            if self.sieve[i] {
                return Some(i + self.offset);
            }
        }
        return None;
    }
}

pub fn nth(n: u32) -> u32 {
    let prime_index = n as usize;
    let mut found_numbers: Vec<SievePointer> = Vec::with_capacity(prime_index + 1);
    let mut offset = 2;  // 0, 1
    let chunk_size: usize = prime_index.next_power_of_two();

    while found_numbers.len() <= prime_index {
        found_numbers = process_chunk(found_numbers, offset, chunk_size, prime_index + 1);
        offset += chunk_size;
    }
    return found_numbers[prime_index].number;
}

fn process_chunk(
    mut found_numbers: Vec<SievePointer>,
    offset: usize,
    chunk_size: usize,
    limit: usize,
) -> Vec<SievePointer> {
    let mut sieve = SieveChunk::new(chunk_size, offset);
    for prime in &mut found_numbers {
        sieve.prick_number(prime);
    }
    loop {
        let mut last_prime: usize = 0;
        if found_numbers.len() > 0 {
            last_prime = (found_numbers[found_numbers.len() - 1].number + 1) as usize;
        }
        let next_prime = sieve.get_next_prime_from(last_prime);
        match next_prime {
            Some(number) => {
                {
                    let mut new_prime = SievePointer { number: (number) as u32, last_value: number };
                    sieve.prick_number(&mut new_prime);
                    found_numbers.push(new_prime);
                }
            },
            _ => break,
        }
        if found_numbers.len() == limit {
            break;
        }
    }
    return found_numbers
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
    fn test_big_prime() {
        assert_eq!(nth(10_000), 104743);
    }

    #[test]
    fn test_very_big_prime() {
        assert_eq!(nth(100_000), 1299721);
    }

    #[test]
    #[ignore]
    fn test_very_very_big_prime() {
        assert_eq!(nth(1_000_000), 15485867);
    }

}