use std::time::SystemTime;
use std::env;

trait IsPrime {
    fn is_prime(&self, list: &[u32]) -> bool;
}

impl IsPrime for u32 {
    fn is_prime(&self, list: &[u32]) -> bool {
        for prime in list {
            if self % prime == 0 {
                return false;
            }
        }
        true
    }
}

struct Primes {
    primes: Vec<u32>,
}

impl Primes {
    fn new(max: u32) -> Vec<u32> {
        let mut this = Primes {
            primes: vec![2, 3],
        };
        this.find_primes_max(max);
        this.primes
    }

    fn find_primes_in_range(&self, low: u32, high: u32) -> Vec<u32> {
        let mut primes_part: Vec<u32> = vec![];
        for num in low..high {
            if num.is_prime(&self.primes) {
                primes_part.push(num);
            }
        }
        primes_part
    }

    fn find_primes_max(&mut self, max: u32) {
        let mut last_elem: u32 = *self.primes.last().unwrap();
        while last_elem < max {
            self.primes.append(&mut self.find_primes_in_range(last_elem + 2, last_elem * 2));
            last_elem = *self.primes.last().unwrap();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let max: u32 = match args[1].parse() {
        Ok(n)  => n,
        Err(_) => panic!("What you entered is not a number!")
    };
    let start_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n)  => n.as_millis(),
        Err(_) => 0
    };
    let primes = Primes::new(max);
    let end_time   = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n)  => n.as_millis(),
        Err(_) => 0
    };
    println!("Primes:\n
              {:?}\n
              There are {} primes smaller than {}.\n
              Took {} seconds.", primes, primes.len(), max, (end_time - start_time) as f32 / 1000.0);
}