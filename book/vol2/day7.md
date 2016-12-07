# Day 7 - static initialization

[Static variables](https://en.wikipedia.org/wiki/Static_variable) are available
throughout the entire life of a program. They are allocated in a block of
memory known at compile time. Due to that, they tend to represent global
state that the program can access.
It's getting especially tricky if one static variable depends on another.
Some language communities even talk about
[static initialization order fiasco](https://isocpp.org/wiki/faq/ctors#static-init-order)
(looking at you, C++). Other, like C, allow static intialization only with
constant literals/expressions. Rust
[belongs to this group](https://doc.rust-lang.org/beta/book/const-and-static.html)
as well. But there are alternatives...

Looking up colors by name
-------------------------

Suppose we're building a web browser engine. Among thousands of things to take
care of, we should be able to render colorful text. `<p style="color: blue">`
should look like a paragraph set in a blue font. But `blue` is a human-readable
name for a color; computers like numbers. Let's translate it with the help
of a `Color` struct:

```rust
#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
```

We cannot create a static `HashMap` and initialize it with a mapping
from color names to `Color`s. So let's use pattern matching to find
color by name:

```rust
pub fn find_color(name: &str) -> Option<Color> {
    match name.to_lowercase().as_str() {
        "amber" => Some(Color { r: 255, g: 191, b: 0 }),
        // hundreds of other names...
        "zinnwaldite brown" => Some(Color { r: 44, g: 22, b: 8 }),
        _ => None,
    }
}
```

The downside is that matching string slices will probably generate a linear
search. So the more colors we have, the slower `find_color` will be.

Did I just say we cannot have a static `HashMap`?

> *puts on Morpheus sunglasses*

What if I told you...

`lazy_static!`
--------------

[`lazy_static`](https://crates.io/crates/lazy_static/) is a crate that
enables `static` variables that are initialized in a non-trivial way.
For example a precomputed regular epression such as
[the ones used in `docopt`](https://github.com/docopt/docopt.rs/blob/717c26c1924d2c95fac48814c96ac6979fe2f20d/src/parse.rs#L182),
or a static `HashMap`!

```rust
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref COLORS_MAP: HashMap<&'static str, Color> = {
        let mut map = HashMap::new();
        map.insert("amber", Color { r: 255, g: 191, b: 0 });
        // ...
        map.insert("zinnwaldite brown", Color { r: 44, g: 22, b: 8 });
        map
    };
}

pub fn find_color_lazy_static(name: &str) -> Option<Color> {
    COLORS_MAP.get(name.to_lowercase().as_str()).cloned()
}
```

`COLORS_MAP` will be evaluated on first access. We can now safely treat
it as if it was a regular static variable.

`phf`
-----

`HashMap` uses a *somewhat slow hash algorithm* (quoting the documentation)
to avoid DoS attacks. With large enough maps there's also a possibility of
collisions.

On the other hand, [`phf`](https://crates.io/crates/phf) uses perfect hashing
(hashing that guarantees no collisions) to build compile-time maps.
This way we can have efficient constant-time lookups at runtime.

```rust
#![feature(plugin)]
#![plugin(phf_macros)]

#[macro_use]
extern crate phf;

static COLORS: phf::Map<&'static str, Color> = phf_map! {
    "amber" => Color { r: 255, g: 191, b: 0 },
    // ...
    "zinnwaldite brown" => Color { r: 44, g: 22, b: 8 },
};

pub fn find_color_phf(name: &str) -> Option<Color> {
    COLORS.get(name.to_lowercase().as_str()).cloned()
}
```

Benchmarks!
-----------

So what's the difference in speed between these approaches?

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_match_lookup(b: &mut Bencher) {
        b.iter(|| find_color("White"))
    }

    #[bench]
    fn bench_lazy_static_map(b: &mut Bencher) {
        b.iter(|| find_color_lazy_static("White"))
    }

    #[bench]
    fn bench_phf_map(b: &mut Bencher) {
        b.iter(|| find_color_phf("White"))
    }
}
```

```text
$ cargo bench
running 3 tests
test tests::bench_lazy_static_map ... bench:         367 ns/iter (+/- 20)
test tests::bench_match_lookup    ... bench:       3,948 ns/iter (+/- 460)
test tests::bench_phf_map         ... bench:         350 ns/iter (+/- 33)
```

Linear search at runtime in the `match` statement is the slowest, as expected.
Both static `HashMap` and `phf::Map` are about an order of magnitude faster,
with the latter leading by a small amount. I would personally prefer `phf`
for lookups like this, as it is the intended use for static compile-time
maps. `lazy_static` is more general in its intent and initializing maps
is just one of its many potential uses.

Further reading
---------------

 - [Static initializers will murder your family](https://meowni.ca/posts/static-initializers/) (in C++, fortunately not Rust)
 - [Perfect hashing functions](https://en.wikipedia.org/wiki/Perfect_hash_function)
 - [Match statement efficiency?](https://users.rust-lang.org/t/match-statement-efficiency/4488/1)
