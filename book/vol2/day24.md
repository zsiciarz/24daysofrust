# Day 24 - conclusion

This article marks the end of the second edition of *24 days of Rust*. I hope
you enjoyed it and maybe found inspiration for a project or two. I sure
did :-) Some of the libraries I wrote about are familiar to almost entire
Rust community. Some are fairly obscure but I find them interesting.
Regardless, I learned a lot just by writing, trying to come up with meaningful
code examples and editing my drafts. This was my intention all along - to
learn something for myself while contributing these articles to the community.

A return to writing
-------------------

I started thinking about doing this series again some time around May-June,
when I heard about `diesel`. A few weeks later I attended PolyConf in Poznań
and had the pleasure of speaking to `diesels`'s author - Sean Griffin. That
convinced me to write an early draft of an article on `diesel`.

Before December started, I had compiled a list of crates, mostly doing this:

 - I use X for doing Y in Python, is there anything like that for Rust?
 - Or: that thing was a few weeks ago in Crate of the Week, let me see... ooh,
   it would be fun to use, let's write an example to familiarize myself with
   the API.
 - Or: this is an entirely new territory for me; I can't think about a simple
   example now, but I'll come back to it later.

I ended up with a Trello board full of ideas, partial examples and several
snippets of prose. What was left was just drawing the rest of the effing owl ;-)

TILWW24DOR
----------

Or, Things I Learned While Writing 24 Days Of Rust:

 - I missed a few cool Cargo subcommands, for example: `cargo edit`, `cargo check` or `cargo modules`
 - it's possible to run benchmarks on stable Rust with [`bencher`](https://crates.io/crates/bencher)
 - I actually understood [what `lazy_static` does](https://siciarz.net/24-days-rust-static-initialization/)
 - you can [pattern match in function arguments](http://words.steveklabnik.com/four-years-with-rust#syntax-changes_2)
 - bow ties are cool
 - there's a crate for building arbitrary binary data - [`test-assembler`](https://crates.io/crates/test-assembler)
 - [habitat](https://www.habitat.sh/) is built with Rust

This is only a partial selection of valuable information I picked from replies
at [/r/rust](https://www.reddit.com/r/rust/), Twitter messages etc.
I'm putting it here in hope it may be useful to someone else as well.

Feedback
--------

The articles received a lot of positive feedback from the Rust community and
I'm very grateful for that. **You are fantastic**! Here are some replies from
Reddit, Twitter etc.:

> Thanks for these great blog posts. Each one was extremely clear and a very
> good intro to the crate and its purpose. As a beginner, this was a boon to
> discover some bits of the Rust ecosystem.

---

> Hey, I just wanted to say thanks for you taking the time to write these up.
> They're quite fun to read, and I've even learned a thing or two :)

---

> It seems that your parsing logic is a little bit broken – the extendend
> length and mask fields are optional, and you're parsing them unconditionally –
> check out the examples from the RFC. Correctness aside, having an example on
> conditional parsing in nom would be a nice addition to the post also!
> Anyway, thanks for great article!

---

> This series is incredible. ❤

---

> Now I'm thinking about how I could write some Rust code to automate my
> various workflows... Awesome blog 👍

---

> I really like that this series shows how to use some less-talked about Rust
> libraries to write actual, potentially useful software.


**Thank you all!** This motivates me to write more.
(And yes, I'll try to fix the parsing example...)

24 days of Rust 2016 - archive
------------------------------

 - [Day 1 - cargo subcommands](https://siciarz.net/24-days-rust-cargo-subcommands/)
 - [Day 2 - hound](https://siciarz.net/24-days-rust-hound/)
 - [Day 3 - rayon](https://siciarz.net/24-days-rust-rayon/)
 - [Day 4 - structured logging](https://siciarz.net/24-days-rust-structured-logging/)
 - [Day 5 - environment variables](https://siciarz.net/24-days-rust-environment-variables/)
 - [Day 6 - derive_builder](https://siciarz.net/24-days-rust-derive_builder/)
 - [Day 7 - static initialization](https://siciarz.net/24-days-rust-static-initialization/)
 - [Day 8 - serde](https://siciarz.net/24-days-rust-serde/)
 - [Day 9 - winreg](https://siciarz.net/24-days-rust-winreg/)
 - [Day 10 - nom, part 1](https://siciarz.net/24-days-rust-nom-part-1/)
 - [Day 11 - nom, part 2](https://siciarz.net/24-days-rust-nom-part-2/)
 - [Day 12 - clap](https://siciarz.net/24-days-rust-clap/)
 - [Day 13 - zip and lzma compression](https://siciarz.net/24-days-rust-zip-and-lzma-compression/)
 - [Day 14 - Cursive](https://siciarz.net/24-days-rust-cursive/)
 - [Day 15 - tera](https://siciarz.net/24-days-rust-tera/)
 - [Day 16 - git2](https://siciarz.net/24-days-rust-git2/)
 - [Day 17 - diesel](https://siciarz.net/24-days-rust-diesel/)
 - [Day 18 - error_chain](https://siciarz.net/24-days-rust-error_chain/)
 - [Day 19 - leftpad](https://siciarz.net/24-days-rust-leftpad/)
 - [Day 20 - reqwest](https://siciarz.net/24-days-rust-reqwest/)
 - [Day 21 - app_dirs and preferences](https://siciarz.net/24-days-rust-app_dirs-and-preferences/)
 - [Day 22 - lettre](https://siciarz.net/24-days-rust-lettre/)
 - [Day 23 - built with Rust](https://siciarz.net/24-days-rust-built-rust-2016/)
 - [Day 24 - conclusion](https://siciarz.net/24-days-rust-conclusion-2016/)

What's next?
------------

Hopefully I'll get back to more regular writing. Now that I'm more comfortable
with Rust, I hope to use it for some serious project and not just
an article's worth of example. The [Rocket](https://rocket.rs/) web framework
was just announced and it looks great, I would like to try it out soon.
There's also a growing ecosystem of
[Machine Learning libraries](https://medium.com/@autumn_eng/about-rust-s-machine-learning-community-4cda5ec8a790#.y8xrysmt8)
and this is something I'm interested in as well. Or maybe play with
[`tokio`](https://github.com/tokio-rs/tokio) for async stuff? There's a lot of
ideas!

Learning resources
------------------

The *24 days of Rust* series may be over for this year, but the Rust journey
continues. It's dangerous to go alone, so here are a few *link*s:

 - [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) -
   the official book by the Rust team
 - [Second edition of the above book](http://rust-lang.github.io/book/) -
   in progress as of December 2016, but *a lot* has changed. Worth reading even
   if you're familiar with the first edition
 - [Rust by Example](http://rustbyexample.com/) - you can read it alongside the
   official book if you like more example-oriented approach
 - [Rust Essentials](https://www.packtpub.com/application-development/rust-essentials) -
   a short book from Packt about basic concepts of Rust. It even mentions *24 days of Rust*!
 - [users.rust-lang.org](https://users.rust-lang.org/) - a discussion forum for
   Rust programmers
 - [The Rust Community](https://www.rust-lang.org/en-US/community.html)
 - [This Week in Rust](https://this-week-in-rust.org/)
 - [into_rust()](http://intorust.com/) - screencasts for learning Rust
 - [rust-learning](https://github.com/ctjhoa/rust-learning) which mentions
   probably all of the above

And here's a selection of Rust-related blogs worth following:

 - [Andrew Gallant's Blog](http://blog.burntsushi.net/)
 - [Featherweight Musings](http://featherweightmusings.blogspot.fr/)
 - [In Pursuit of Laziness](https://manishearth.github.io/)
 - [Jeff Parsons](https://jeffparsons.github.io/)
 - [Huon Wilson](https://huonw.github.io/blog/)
 - [Llogiq on stuff](https://llogiq.github.io/)
 - [Mozilla Hacks](https://hacks.mozilla.org/) and the [Servo blog](https://blog.servo.org/)
 - [Niko Matsakis](http://smallcultfollowing.com/babysteps/)
 - [Steve Klabnik](http://words.steveklabnik.com/)

And with that, it's time to really finish writing for today.

**Rusty Holidays to you all!**