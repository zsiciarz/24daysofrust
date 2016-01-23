# Day 8 - racer

> Relevancy: 1.6 stable

Welcome to the second week of 24 days of Rust! Hope you enjoy the articles so far. Today let me introduce you to [racer](https://github.com/phildawes/racer) - a code completion engine for Rust.

As there is no proper Rust IDE (yet), most of us [Rustaceans](http://www.rustaceans.org/) use some code editor such as Vim, Emacs, Atom etc. However the out-of-the-box support for Rust is very limited - if you're lucky, your editor has syntax highlighting, but not much more. We'd love to have some sort of autocompletion to ease typing, and a "go to definition" command to quickly jump around the codebase.

`racer` is a tool (still in development) to do just that. Follow the [instructions in readme](https://github.com/phildawes/racer) to compile it. You'll need to have the source code for Rust compiler and standard libraries on your machine. Fortunately it's just a `git clone` away:

```sh
$ git clone https://github.com/rust-lang/rust.git $HOME/rust
$ export RUST_SRC_PATH=$HOME/rust/src
```

The second line sets an environment variable that `racer` uses to find Rust sources. You can put that line in your `.bashrc`, `.zshrc` or whatever startup file your shell uses.

The `racer` executable has a few subcommands (you can see the full list by running `racer` with no arguments). Let's take a look at the most important ones.

Completion engine
-----------------

The `racer complete` subcommand takes a part of some (fully qualified) Rust name and tries to find completions. For example:

```sh
$ ./target/release/racer complete std::option::
MATCH Option,167,9,/home/zbyszek/Development/Rust/rust/src/libcore/option.rs,Enum,pub enum Option<T> {
MATCH Iter,815,11,/home/zbyszek/Development/Rust/rust/src/libcore/option.rs,Struct,pub struct Iter<'a, A: 'a> { inner: Item<&'a A> }
MATCH IterMut,845,11,/home/zbyszek/Development/Rust/rust/src/libcore/option.rs,Struct,pub struct IterMut<'a, A: 'a> { inner: Item<&'a mut A> }
MATCH IntoIter,868,11,/home/zbyszek/Development/Rust/rust/src/libcore/option.rs,Struct,pub struct IntoIter<A> { inner: Item<A> }</A></A>
</T>
```

The output is intended to be computer-readable, in fact you could possibly parse it as a comma-separated format (see the [chapter on CSV](day3.md)).

![racer completion in Vim](//i.imgur.com/gE3Q7i4.png)

Find definition
---------------

The second important subcommand is `racer find-definition`. It expects three extra arguments: line number, column number and a filename. The first two numbers are cursor coordinates, so from the code editor's point of view `racer` will try to find the definition of the word under cursor. For example when run on `racer`'s own source (the coordinates point to a `Vec` usage):

```sh
$ ./target/release/racer find-definition 56 15 src/main.rs
MATCH Vec,154,11,/home/zbyszek/Development/Rust/rust/src/libcollections/vec.rs,Struct,pub struct Vec<T> {

```

The output format is the same as in the case of `racer complete`. See how that works in Vim when I press `gd` with the cursor on the `Option` word:

![find definition in Vim](//i.imgur.com/fyE80H5.gif)

Editor integrations
-------------------

TODO: write about YouCompleteMe

[Vim](https://github.com/phildawes/racer/blob/master/editors/racer.vim) and [Emacs](https://github.com/phildawes/racer/blob/master/editors/racer.el) plugins are bundled with `racer`'s source. For other editors there are some third-party plugins:

 * [atom-racer](https://github.com/edubkendo/atom-racer) for [Atom](https://atom.io/)
 * [RustAutoComplete](https://sublime.wbond.net/packages/RustAutoComplete) for [Sublime Text](http://www.sublimetext.com/)
