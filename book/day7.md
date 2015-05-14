# Day 7 - itertools

The [itertools crate](https://crates.io/crates/itertools) contains several utility functions and macros inspired by Haskell and [Python itertools](https://docs.python.org/3/library/itertools.html). As you can guess from the name, these have to do with iteration and iterators.  Rust has recently split the `Iterator` trait into `Iterator` and `IteratorExt` for so called **object safety** reasons (see [the RFC](https://github.com/rust-lang/rfcs/blob/master/text/0445-extension-trait-conventions.md) for an explanation). This is mostly irrelevant for today's episode of 24 days of Rust, but worth keeping in mind.

To use `itertools`, add the following dependency declaration to `Cargo.toml`:

    :::toml
    [dependencies]
    itertools = "~0.0.4"

We'll start from the helper functions and cover the macros later.

apply
-----

This and a few other functions live in the `Itertools` trait, so we need to bring them into scope by placing `use itertools::Itertools` in our module.

`apply()` is very simple conceptually. It consumes the (mutable) iterator, calling a closure witch each of the elements. The return type is `()` (unit), meaning that `apply()` usually should be at the end of a call chain, like below:

    :::rust
    let mut words = "hello supercalifragilisticexpialidocious programmer".words();
    words.apply(|word| println!("{} is {} characters long", word, word.len()));

As you can see, `apply()` is similar to the `map()` method from the standard library, however `map` returns another iterator. Therefore it's lazy and allows for further method chaining, while `apply` is eager and has the final word.

interleave and intersperse
--------------------------

`interleave()` is somewhat similar to `zip()`. But when `zip` builds tuples from two iterators, `interleave` yields the values alternating between both iterators.

    :::rust
    let even = range(1, 10u).map(|x| x * 2);
    let odd = range(1, 5u).map(|x| x * 2 + 1);
    println!("{}", even.interleave(odd).collect::<Vec<_>>());

The result:

    :::sh
    $ cargo run
    [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 16, 18]

As you can see, the iterators don't have to contain the same number of elements. In that case `interleave` continues to emit values from the other iterator, after it consumes the "shorter" one.

In a manner similar to its [Haskell counterpart](http://hackage.haskell.org/package/base-4.7.0.1/docs/Data-List.html#v:intersperse), `intersperse` takes a single value and an iterator (implicitly as the `self` argument) and emits the given value between every element of the wrapped iterator. For example:

    :::rust
    let it = range(1, 10u);
    println!("{}", it.intersperse(15u).collect::<Vec<_>>());

Output:

    :::sh
    $ cargo run
    [1, 15, 2, 15, 3, 15, 4, 15, 5, 15, 6, 15, 7, 15, 8, 15, 9]

iproduct!
---------

Let's now turn our attention to macros provided by `itertools`. Sometimes there is a need to iterate over a cartesian product of some lists/arrays/vectors. Usually it involves two nested loops; however we can use the `iproduct()` macro to simplify it to a single `for` loop.

    :::rust
    let numbers = range(1, 4u);
    let chars = vec!['a', 'b', 'c'];
    for (i, c) in iproduct!(numbers, chars.iter()) {
        println!("{}: {}", i, c);
    }

icompr!
-------

This is possibly the tastiest bit in `itertools`. Those of you coming to Rust from Python will recognize it immediately. [Generator expressions](https://docs.python.org/3.4/reference/expressions.html#generator-expressions) allow creating lazy iterators with a very simple, convenient syntax. The `icompr!()` (*iterator comprehension*) macro brings that syntax to Rust. The following example borrows from David Beazley's [Generators for System Programmers](http://www.dabeaz.com/generators/) tutorial.

    :::rust
    use std::iter::AdditiveIterator;

    let log = "GET / 4096\nGET /home/ 16301\nPOST /home/ 49\nGET / 4096\n";
    let lines = log.lines();
    let rows = icompr!(line.words().collect::<Vec<_>>() for line in lines);
    let bytes = icompr!(row[2] for row in rows if row[0] != "POST");
    let total = icompr!(from_str::<uint>(b).unwrap() for b in bytes).sum();
    println!("Total GET throughput: {} bytes", total);

We parse a very simplified server log, counting a total number of bytes sent in response to GET requests. Just like in Python, we can use conditionals (`if row[0] != "POST") to filter the values directly in the generator expression.

----

<small>
Code examples in this article were built with rustc 0.13.0-nightly and itertools 0.0.4.
</small>
