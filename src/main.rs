use std::time::SystemTime;
use std::env;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, SeekFrom, prelude::*};
use std::path::Path;

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
    already_found: usize
}

impl Primes {
    fn new() -> Primes {
        Primes {
            primes: vec![2, 3],
            already_found: 0
        }
    }

    fn read_file(&mut self, path: &Path) {
        match File::open(&path) {
            Ok(mut file) => {
                let mut s = String::new();
                match file.read_to_string(&mut s) {
                    Err(why) => panic!("Could not read from file: {}.", why),
                    Ok(_)    => ()
                };
                let (_, s) = s.split_at(11);        // remove "primes = { "
                let (s, _) = s.split_at(s.len()-3); // remove " }\n"
                let primes_string: Vec<&str> = s.split(",").collect();

                self.primes = vec![];
                self.primes.reserve(primes_string.len());
                for prime_string in primes_string {
                    self.primes.push(prime_string.parse().unwrap());
                }
                self.already_found = self.primes.len();
                println!("{} primes have already been found.", self.already_found);
            },
            Err(why) => {
                println!("Could not open file: {}.", why);
            }
        };
    }

    fn write_file(&self, path: &Path) {
        let mut file: File;
        let mut output: String;
        if self.already_found > 0 {
            file = OpenOptions::new().write(true).open(&path).unwrap();
            file.seek(SeekFrom::End(-3)).unwrap();
            output = String::from(",");
        }
        else {
            file = File::create(&path).unwrap();
            output = String::from("primes = { ");
        }
        
        for i in self.already_found..self.primes.len()-1 {
            output.push_str(&self.primes[i].to_string());
            output.push(',');
        }
        output.push_str(&self.primes[self.primes.len()-1].to_string());
        output.push_str(" }\n");

        let mut exit = false;
        while !exit {
            match file.write_all(output.as_bytes()) {
                Err(why) => {
                    print!("Could not write to file: {}. Try again? [Y/n]: ", why);
                    let mut input = String::new();
                    match io::stdin().read_to_string(&mut input) {
                        Err(why) => println!("Could not read input: {}. Please try again ...", why),
                        Ok(_)    => ()
                    };
                }
                Ok(_) => {
                    exit = true;
                    println!("File written successfully.");
                }
            }
        }
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

    fn find_primes_num(&mut self, num_primes: u32) -> Vec<u32> {
        self.primes.reserve(num_primes as usize);
        let mut last_elem: u32 = *self.primes.last().unwrap();
        while self.primes.len() < num_primes as usize {
            print!("\r{}", self.primes.len() as f32 / num_primes as f32); io::stdout().flush().unwrap();
            self.primes.append(&mut self.find_primes_in_range(last_elem + 2, last_elem * 2));
            last_elem = *self.primes.last().unwrap();
        }
        self.primes.clone()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide the upper limit as argument.");
    }
    let num: u32 = match args[1].parse() {
        Ok(n)  => n,
        Err(_) => panic!("What you entered is not a number!")
    };
    
    let path    = Path::new("primes.js");

    let mut primes_finder = Primes::new();
    primes_finder.read_file(&path);
    if primes_finder.already_found >= num as usize {
        println!("This many or more primes have already been found.\nCheck {}.", path.display());
    }
    else {
        println!("Looking for primes ...");
        let start_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n)  => n.as_millis(),
            Err(_) => 0
        };
        let primes = primes_finder.find_primes_num(num);
        let end_time   = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n)  => n.as_millis(),
            Err(_) => 0
        };
        println!("Found {} primes.\nTook {} seconds.\nWriting primes to {} ...", 
        primes.len(), (end_time - start_time) as f32 / 1000.0, path.display());
        
        primes_finder.write_file(&path);
    }
}