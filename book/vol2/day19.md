# Day 19 - leftpad

Do you remember the time when
[one developer broke half of the Internet](http://www.theregister.co.uk/2016/03/23/npm_left_pad_chaos/)?
Earlier this year a package called
[`left-pad`](https://www.npmjs.com/package/left-pad) was pulled from the NPM
registry (a counterpart of crates.io for the JavaScript folks). This caused...
some drama, to say the least. Shall we try this in Rust? Not sure about the
drama, maybe just padding a string on the left is enough. There's a crate for
that - [`leftpad`](https://crates.io/crates/leftpad)!

Left pad all the things!
------------------------

Look at this beauty.

```rust
extern crate leftpad;

use leftpad::{left_pad, left_pad_char};

fn main() {
    println!("{}", left_pad("pad me", 20));
    println!("{}", left_pad("pad me again", 20));
}
```

```text
$ cargo run
              pad me
        pad me again
```

Just look.

```rust
println!("{}", left_pad_char("tick", 20, '✓'));
```

```text
$ cargo run
✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓✓tick
```

Wow. Such characters. Very padding. Wow.

So what would happen if Hubert decided to yank `leftpad` from crates.io?
Fortunately it's not a big deal. Existing projects using `leftpad` would
still work. We only wouldn't be able to use it for any new applications.

Let me quote an important fragment of Cargo docs:

> The semantics of a yanked version are that no new dependencies can be created
> against that version, but all existing dependencies continue to work. One of
> the major goals of crates.io is to act as a permanent archive of crates that
> does not change over time, and allowing deletion of a version would go
> against this goal. Essentially a yank means that all projects with a
> Cargo.lock will not break, while any future Cargo.lock files generated will
> not list the yanked version.

Thankfully, `leftpad` is licensed under a
[permissive license](https://github.com/hfiguiere/leftpad-rs/blob/3be4768d7d697654184389fcf527f8750ec8596d/LICENSE),
which means I can get away with posting here the entire two functions
(and the original copyright notice to comply with the BSD license terms).

```rust
pub fn left_pad(s: &str, pad: usize) -> String
{
    left_pad_char(s, pad, ' ')
}

pub fn left_pad_char(s: &str, pad: usize, padchar: char) -> String
{
    let mut out = String::new();

    let len = s.len();
    if pad > len {
        for _ in 0..pad-len {
            out.push(padchar);
        }
    }
    out.push_str(s);

    out
}
```

```text
Copyright (c) 2016, Hubert Figuière
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

1. Redistributions of source code must retain the above copyright
notice, this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright
notice, this list of conditions and the following disclaimer in the
documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

So if anything catastrophic ever happens to GitHub and crates.io, feel free
to come here for your left-padding needs!

Further reading
---------------

 - [left-pad as a service](http://left-pad.io/)
 - [Could Rust have a left-pad incident?](http://edunham.net/2016/03/24/could_rust_have_a_left_pad_incident.html)
 - [docs for `cargo yank`](http://doc.crates.io/crates-io.html#cargo-yank)