#![feature(core)]

extern crate slow_primes;

use slow_primes::Primes;

fn num_divisors(n: usize, primes: &Primes) -> Option<usize> {
    use std::iter::MultiplicativeIterator;
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().map(|(_, x)| x + 1).product()),
        Err(_) => None,
    }
}

fn main() {
    println!("24 days of Rust - slow_primes (day 2)");
    let sieve = Primes::sieve(10000);
    let suspect = 5273;
    println!("{} is prime: {}", suspect, sieve.is_prime(suspect));
    let not_a_prime = 1024;
    println!("{} is prime: {}", not_a_prime, sieve.is_prime(not_a_prime));
    let n = 1000;
    match sieve.primes().nth(n - 1) {
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("I don't know anything about {}th prime.", n),
    }
    println!("{:?}", sieve.factor(2610));
    println!("{:?}", num_divisors(2610, &sieve));
}
