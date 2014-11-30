extern crate slow_primes;

use slow_primes::Primes;

fn num_divisors(n: uint, primes: &Primes) -> Option<uint> {
    use std::iter::MultiplicativeIterator;
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().map(|(_, x)| x + 1).product()),
        Err(_) => None,
    }
}

fn main() {
    let sieve = Primes::sieve(10000u);
    let suspect = 5273u;
    println!("{} is prime: {}", suspect, sieve.is_prime(suspect));
    let not_a_prime = 1024u;
    println!("{} is prime: {}", not_a_prime, sieve.is_prime(not_a_prime));
    let n = 1000u;
    match sieve.primes().nth(n - 1) {
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("I don't know anything about {}th prime.", n),
    }
    println!("{}", sieve.factor(2610));
    println!("{}", num_divisors(2610, &sieve));
}
