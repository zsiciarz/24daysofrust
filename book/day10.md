# Day 10 - the glorious tau

> Relevancy: 1.6 stable

τ (*tau*) is one of the most important mathematical constants (if not *the* most important), relating circle's circumference to it's radius. See the [tau manifesto](http://www.tauday.com/tau-manifesto) for the long explanation if you're still an unbeliever. Can we use it in Rust? Of course, there's a [crate for that](https://crates.io/crates/tau)!

Let's see this amazing number in full glory:

[include:2-2](../src/day10.rs)
[include:8-8](../src/day10.rs)

```sh
$ cargo run
τ = 6.283185307179586
```

We can do all sorts of important calculations with τ, just look:

[include:9-13](../src/day10.rs)

And if someone really, I mean *really* needs to refer to that other mathematical constant, it is (regrettably) possible as well.

[include:14-14](../src/day10.rs)

Here's the output:

```sh
$ cargo run
circle circumference = τ * r = 94.24777960769379
Euler's identity: exp(i * τ) = 1-0.00000000000000024492935982947064i
Trigonometry: sin(τ) = -0.00000000000000024492935982947064, cos(τ) = 1
That other constant = 3.141592653589793
```
