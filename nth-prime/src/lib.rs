struct SievePointer {
    number: usize,
    last_value: usize,
}

struct FoundPrimes {
    primes: Vec<SievePointer>,
}

impl FoundPrimes {
    pub fn new(size: usize) -> FoundPrimes {
        FoundPrimes {
            primes: Vec::with_capacity(size),
        }
    }

    pub fn get_last_found_prime(&self) -> usize {
        if self.primes.is_empty() {
            return 0;
        }
        (self.primes[self.primes.len() - 1].number) as usize
    }

    pub fn get_nth_prime(&self, n: usize) -> usize {
        self.primes[n].number
    }

    pub fn len(&self) -> usize {
        self.primes.len()
    }

    pub fn push(&mut self, found_prime: SievePointer) {
        self.primes.push(found_prime);
    }
}

struct SieveChunk {
    sieve: Vec<bool>,
    offset: usize,
}

impl SieveChunk {
    pub fn new(size: usize, offset: usize) -> SieveChunk {
        let mut sieve: Vec<bool> = Vec::with_capacity(size);
        sieve.resize(size, true);
        SieveChunk { sieve, offset }
    }

    pub fn prickle_number(&mut self, mut number: &mut SievePointer) {
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
        for i in start..self.sieve.len() {
            if self.sieve[i] {
                return Some(i + self.offset);
            }
        }
        None
    }

    pub fn process(&mut self, found_primes: &mut FoundPrimes, limit: usize) {
        self.prickle_known_primes(found_primes);
        self.look_for_new_primes(found_primes, limit);
    }

    fn prickle_known_primes(&mut self, found_primes: &mut FoundPrimes) {
        for prime in &mut found_primes.primes {
            self.prickle_number(prime);
        }
    }

    fn look_for_new_primes(&mut self, found_primes: &mut FoundPrimes, limit: usize) {
        loop {
            let next_prime_candidate: usize = found_primes.get_last_found_prime() + 1;
            let next_prime = self.get_next_prime_from(next_prime_candidate);
            match next_prime {
                Some(number) => self.add_prime(number, found_primes),
                _ => break,
            }
            if found_primes.len() > limit {
                break;
            }
        }
    }

    fn add_prime(&mut self, prime: usize, found_primes: &mut FoundPrimes) {
        let mut new_prime = SievePointer {
            number: prime,
            last_value: prime,
        };
        self.prickle_number(&mut new_prime);
        found_primes.push(new_prime);
    }
}

struct Sieve {
    found_primes: FoundPrimes,
    processed_part: usize,
    chunk_size: usize,
    limit: usize,
    processed: bool,
}

impl Sieve {
    pub fn new(limit: usize) -> Sieve {
        let chunk_size = limit.next_power_of_two();
        Sieve {
            chunk_size,
            limit,
            processed: false,
            processed_part: 2,
            found_primes: FoundPrimes::new(limit),
        }
    }

    pub fn get_nth_prime(&mut self, n: usize) -> usize {
        if n > self.limit {
            panic!("Getting more then limit!");
        }
        if !self.processed {
            panic!("Should be processed!");
        }
        self.found_primes.get_nth_prime(n)
    }

    pub fn ensure_sieve_pricked(&mut self) {
        while self.found_primes.len() <= self.limit {
            self.find_chunk_of_primes();
        }
        self.processed = true;
    }

    fn find_chunk_of_primes(&mut self) {
        let mut sieve_chunk: SieveChunk = SieveChunk::new(self.chunk_size, self.processed_part);
        sieve_chunk.process(&mut self.found_primes, self.limit);
        self.processed_part += self.chunk_size;
    }
}

pub fn nth(n: u32) -> u32 {
    let mut sieve = Sieve::new(n as usize);
    sieve.ensure_sieve_pricked();
    sieve.get_nth_prime(n as usize) as u32
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
