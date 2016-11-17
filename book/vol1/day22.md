# Day 22 - built with Rust

Today I'm not going to focus on any specific tool or library. Instead this blogpost will be a showcase of cool things built with Rust. Even though the language is still before 1.0 (but [we're getting there](http://blog.rust-lang.org/2014/12/12/1.0-Timeline.html)!), there are already a few interesting projects out in the wild.

And I don't mean only development libraries - I've been writing about them for the last three weeks :-)

Rust
----

This is kinda obvious. Well, maybe not... First versions of the Rust compiler were written in OCaml. Since then every `rustc` release is built with the previous version, and so on. Such an approach is sometimes called [dogfooding](http://en.wikipedia.org/wiki/Eating_your_own_dog_food).

Servo
-----

The compiler and libraries are not the only rusty things coming from Mozilla. [Servo](https://github.com/servo/servo/) is an experimental web browser engine pursuing the same values as Rust does: safe, concurrent and fast. It's still too early to integrate it in a browser, but it [passes ACID2](http://i.imgur.com/CsLkgLl.png) and is improving day by day.

For a much simpler browser rendering engine in Rust check out [robinson](https://github.com/mbrubeck/robinson) by Matt Brubeck, as well as his awesome blogpost series - [Let's build a browser engine!](http://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html).

iota
----

[iota](https://github.com/gchp/iota) is a work-in-progress console text editor. At the moment it's capabilities are really basic and I don't like emacs-based keybindings, but I've already written one of the examples for 24 days of Rust in iota, just to try it out.

wtftw
-----

[wtftw](https://github.com/Kintaro/wtftw) is a tiling window manager similar to [xmonad](http://xmonad.org/). I must confess I don't use a tiling WM (yet) so I can't state my personal opinion, but the [community reaction](http://www.reddit.com/r/rust/comments/2pkx94/wtftw_released_feedback_welcome/) was very positive. The author [started a tutorial](https://kintaro.github.io/rust/window-manager-in-rust-01/) on how to write a window manager. I'm very much looking forward to it.

coreutils
---------

This [coreutils](https://github.com/uutils/coreutils) repository is an attempt to rewrite GNU coreutils in Rust. I find it interesting to dive into the sources of some program, for example [wc](https://github.com/uutils/coreutils/blob/9a281adc1e4db8da60b2aac0c41ba0e789be8f97/src/wc/wc.rs), and read the code. This is sometimes inspiring, sometimes... not so much. But the big win for Rust coreutils is that it should work on Windows, at least these commands that make sense there.

hematite
--------

[hematite](https://github.com/PistonDevelopers/hematite) is a simplistic Minecraft clone in Rust using the [Piston](http://www.piston.rs/) game engine. Even though I know next to nothing about writing 3D games, Piston with SDL2 looks interesting.

More?
-----

I've picked only a handful of Rust projects that caught my attention in the last few months. You can go to [rustkit.io](http://rustkit.io/) or [crates.io](https://crates.io/) to see new and trending libraries or browse the [Rust subreddit](http://www.reddit.com/r/rust) for new project announcements.
