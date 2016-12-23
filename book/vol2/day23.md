# Day 23 - built with Rust

Today's article is another throwback to the first edition of *24 days of Rust*.
In one of the final posts I
[wrote about interesting projects](http://siciarz.net/24-days-of-rust-built-with-rust/)
built with Rust. That was even before the language hit version 1.0.
Two years later and we're at 1.14. [Servo](https://github.com/servo/servo/),
[iota](https://github.com/gchp/iota) and
[coreutils](https://github.com/uutils/coreutils) are still going strong.
It's surely worth checking them out again, but I'm going to introduce a few
more Rust projects that emerged during the last two years.

Redox
-----

How about starting with an **entire operating system** written in Rust? Because
this is what [Redox](https://www.redox-os.org/) is about. It consists of:

 - a Unix-like microkernel
 - file system (RedoxFS)
 - userspace drivers
 - networking
 - GUI (Orbital)
 - and more...

All built with Rust, free and open source! An impressive achievement for sure.
While I wouldn't replace my Linux installs with Redox just yet, there are
[.iso releases](https://github.com/redox-os/redox/releases) already. It should
work in QEMU or VirtualBox.

Project website: [https://www.redox-os.org/](https://www.redox-os.org/)

ripgrep
-------

[Andrew Gallant](https://github.com/BurntSushi), aka `burntsushi`, is a
*prolific* contributor to the Rust library ecosystem. Regexes, CSV,
byte ordering, docopt, property-based testing - Andrew's your man.
This time he took a stab at filesystem search, with an impressive outcome.
[`ripgrep`](https://github.com/BurntSushi/ripgrep) is a CLI search tool
that aims to surpass `grep`, `ack`, `ag` and others in both usability and
performance. In the aptly titled introductory blog post -
[ripgrep is faster than {grep, ag, git grep, ucg, pt, sift}](http://blog.burntsushi.net/ripgrep/) -
Andrew explains his motivation and presents performance benchmarks.

TL;DR - `ripgrep` is **fast**. Try it out!

Project website: [https://github.com/BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep)

panopticon
----------

I learned about [panopticon](https://panopticon.re/) at
[RustFest EU 2016](http://www.rustfest.eu/talks/#panopticon-a-libre-cross-platform-disassembler-in-rust).
It's a GUI disassembler (for x86, AMD64, AVR and 6502) that aims to be a free,
easy to use and powerful alternative to proprietary tools such as IDA.
Panopticon was rewritten from C++ to Rust and the author expresses his joy
[in a reddit thread](https://www.reddit.com/r/rust/comments/4ihtfa/panopticon_a_libre_crossplatform_disassembler/):

> Programming in Rust is not only more fun, it's definitely easier too.
> Panopticon used alot sum types that were implemented using boost::variant.
> Like everything with Boost it kind of worked but was incredible ugly and
> complex. Replacing them with enums was probably the biggest reason I switched
> to Rust. Also I found iterator invalidation bugs simply by translating C++
> to Rust, thanks Borrow Checker!

Project website: [https://panopticon.re/](https://panopticon.re/)

Mozilla Firefox
---------------

Well, this may be a little stretch ;-) But just a little. Firefox itself wasn't
suddenly rewritten in its entirety from C++ to Rust. However, there are already
[some parts of the browser](https://hacks.mozilla.org/2016/07/shipping-rust-in-firefox/)
built with Rust. [Stylo](https://wiki.mozilla.org/Stylo) is an upcoming
bridge between Firefox and Servo's CSS engine. And with
[Project Quantum](https://medium.com/mozilla-tech/a-quantum-leap-for-the-web-a3b7174b3c12#.kcpq0q16m)
even more good stuff is coming!

Now it's your turn!
-------------------

I'd love to see **you** building awesome Rust projects! Pick a problem that
you'd like to solve and try doing that with Rust. It doesn't have to be an
entire new operating system (that was a hell of an itch to scratch,
Redox folks, right?).

Here are some inspirations:

 - [Awesome Rust](https://github.com/kud1ing/awesome-rust) - a collection of
   programs, libraries and resources
 - [Contributing to Rust — libraries](https://www.rust-lang.org/en-US/contribute-libs.html)
 - [Are we Web yet?](http://www.arewewebyet.org/) - some areas aren't as green
   as I'd like them to be
 - [A list of practical projects that anyone can solve in any programming language](https://github.com/karan/Projects)
 - [Where do you get ideas to build side-projects?](https://news.ycombinator.com/item?id=5234692) -
   a Hacker News discussion

To summarize this article:

> Getting a project featured in *24 days of Rust* is left as an exercise for
> the reader.

Further reading
---------------

 - [Awesome Rust](https://github.com/kud1ing/awesome-rust) - includes applications written in Rust
 - [Friends of Rust](https://www.rust-lang.org/en-US/friends.html) - Organizations running Rust in production
 - [ripgrep code review](http://blog.mbrt.it/2016-12-01-ripgrep-code-review/)
 - [Quantum](https://wiki.mozilla.org/Quantum) at Mozilla Wiki
