# Day 7 - itertools

> Relevancy: 1.6 stable

The [itertools crate](https://crates.io/crates/itertools) contains several utility functions and macros inspired by Haskell and [Python itertools](https://docs.python.org/3/library/itertools.html). As you can guess from the name, these have to do with iteration and iterators.

To use `itertools`, add the following dependency declaration to `Cargo.toml`:

```ini
[dependencies]
itertools = "~0.0.4"
```

We'll start from the helper functions and cover the macros later.

foreach
-----

This and a few other functions live in the `Itertools` trait, so we need to bring them into scope by placing `use itertools::Itertools` in our module.

`foreach()` is very simple conceptually. It consumes the (mutable) iterator, calling a closure witch each of the elements. The return type is `()` (unit), meaning that `foreach()` usually should be at the end of a call chain, like below:

[include:8-9](../src/day7.rs)

As you can see, `foreach()` is similar to the `map()` method from the standard library, however `map` returns another iterator. Therefore it's lazy and allows for further method chaining, while `foreach` is eager and has the final word.

interleave and intersperse
--------------------------

`interleave()` is somewhat similar to `zip()`. But when `zip` builds tuples from two iterators, `interleave` yields the values alternating between both iterators.

[include:10-12](../src/day7.rs)

The result:

```sh
$ cargo run
[2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 16, 18]
```

As you can see, the iterators don't have to contain the same number of elements. In that case `interleave` continues to emit values from the other iterator, after it consumes the "shorter" one.

In a manner similar to its [Haskell counterpart](http://hackage.haskell.org/package/base-4.7.0.1/docs/Data-List.html#v:intersperse), `intersperse` takes a single value and an iterator (implicitly as the `self` argument) and emits the given value between every element of the wrapped iterator. For example:

[include:13-13](../src/day7.rs)

Output:

```sh
$ cargo run
[1, 15, 2, 15, 3, 15, 4, 15, 5, 15, 6, 15, 7, 15, 8, 15, 9]
```

iproduct!
---------

Let's now turn our attention to macros provided by `itertools`. Sometimes there is a need to iterate over a cartesian product of some lists/arrays/vectors. Usually it involves two nested loops; however we can use the `iproduct()` macro to simplify it to a single `for` loop.

[include:14-18](../src/day7.rs)
