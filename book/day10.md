# Day 10 - the glorious tau

> Relevancy: 1.1 stable

τ (*tau*) is one of the most important mathematical constants (if not *the* most important), relating circle's circumference to it's radius. See the [tau manifesto](http://www.tauday.com/tau-manifesto) for the long explanation if you're still an unbeliever. Can we use it in Rust? Of course, there's a [crate for that](https://crates.io/crates/tau)!

Let's see this amazing number in full glory:

```rust
extern crate tau;

fn main() {
    println!("τ = {}", tau::TAU);
}
```

```sh
$ cargo run
τ = 6.283185307179586
```

We can do all sorts of important calculations with τ, just look:

```rust
extern crate num;

use num::complex::{Complex, Complex64};

let radius: f64 = 15.0;
println!("circle circumference = τ * r = {}", tau::TAU * radius);
let c: Complex64 = Complex::from_polar(&1.0, &tau::TAU);
println!("Euler's identity: exp(i * τ) = {}", c);
println!("Trigonometry: sin(τ) = {}, cos(τ) = {}", tau::TAU.sin(), tau::TAU.cos());
```

And if someone really, I mean *really* needs to refer to that other mathematical constant, it is (regrettably) possible as well.

```rust
println!("That other constant = {}", tau::TAU / 2.0);
```

Here's the output:

```sh
$ cargo run
circle circumference = τ * r = 94.24777960769379
Euler's identity: exp(i * τ) = 1-0.00000000000000024492935982947064i
Trigonometry: sin(τ) = -0.00000000000000024492935982947064, cos(τ) = 1
That other constant = 3.141592653589793
```
