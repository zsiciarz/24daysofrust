# Day 23 - calling Rust from other languages

> Relevancy: 1.1 stable

In this penultimate episode of the **24 days of Rust** article series we will focus on using Rust code from other languages. Since Rust libraries can expose a C API and calling conventions, using them isn't very different from using regular C libraries. A lot of programming languages have some kind of an [FFI](http://en.wikipedia.org/wiki/Foreign_function_interface) mechanism, allowing them to use libraries written in other language(s).  Let's see a few examples!

Our Rust library
----------------

As an example we're going to build a small Rust library called `stringtools` which exposes just one function: `count_substrings`. Let's start with a manifest - here's our `Cargo.toml`:

```ini
[package]
name = "stringtools"
version = "0.0.1"
authors = ["Zbigniew Siciarz <zbigniew@siciarz.net>"]

[lib]
name = "stringtools"
crate-type = ["dylib"]

[dependencies]
libc = "~0.1"
```

The crate type is explicitly declared as a dynamic library. And now the actual implementation:

```rust
extern crate libc;

use std::ffi::CStr;
use std::str;
use libc::c_char;

#[no_mangle]
pub extern "C" fn count_substrings(value: *const c_char, substr: *const c_char) -> i32 {
    let c_value = unsafe { CStr::from_ptr(value).to_bytes() };
    let c_substr = unsafe { CStr::from_ptr(substr).to_bytes() };
    match str::from_utf8(c_value) {
        Ok(value) => match str::from_utf8(c_substr) {
            Ok(substr) => rust_substrings(value, substr),
            Err(_) => -1,
        },
        Err(_) => -1,
    }
}

fn rust_substrings(value: &str, substr: &str) -> i32 {
    let mut count = 0;
    let substr_len = substr.len();
    let upper_bound = value.len() - substr_len + 1;
    for c in 0..upper_bound {
        let possible_match = &value[c..c+substr_len];
        if possible_match == substr {
            count += 1;
        }
    }
    count
}
```

The regular Rust function `rust_substrings` is wrapped in some magical code to be consistent with a C ABI. The `#[no_mangle]` attribute keep our functions names plain and simple, while `pub extern "C"` exports the function to the outside world with the C calling convention (`cdecl`). See the [FFI guide](http://doc.rust-lang.org/guide-ffi.html) or my blogpost about [wrapping a C library](http://siciarz.net/ffi-rust-writing-bindings-libcpuid/) for more information.

**Aside**: there was [a funny bug regarding bananas](http://www.wabbo.org/blog/2014/22aug_on_bananas.html) in the algorithm Rust uses for string matching.

To build the library, run `cargo build` to obtain a .so file and you're good to go - time to move on to other languages.

Python
------

This bit was the easiest to write :-) With a little help from the [ctypes](https://docs.python.org/3.4/library/ctypes.html) module our Python example consists of just three lines of code (excluding import):

```python
import ctypes

library_name = "../target/debug/libstringtools.so"
stringtools = ctypes.CDLL(library_name)
print(stringtools.count_substrings(b"banana", b"na"))
```

Note that Python byte strings are mapped by `ctypes` to native C strings underneath.

We can run it:

```sh
$ python3 main.py
2
```

C
--

We have to redeclare the function signature and that's it. Since we exposed from Rust a C API (and ABI), from this perspective it isn't really a foreign call.

```c
#include <stdint.h>
#include <stdio.h>

int32_t count_substrings(const char* value, const char* substr);

int main() {
    printf("%d\n", count_substrings("banana", "na"));
    return 0;
}
```

Compile and run (we need to link with the library at runtime, hence the `LD_LIBRARY_PATH` prefix):

```sh
$ gcc main.c -L ../target/debug -lstringtools -o main
$ LD_LIBRARY_PATH=../target/debug ./main
2
```

Haskell
-------

The [FFI introduction](https://www.haskell.org/haskellwiki/FFI_Introduction) may not be as great as the Rust guide, but after reading through it and some careful googling I've managed to put together the example below:

```haskell
{-# LANGUAGE ForeignFunctionInterface #-}

import Foreign.C.String (CString, newCString)

foreign import ccall "count_substrings"
    count_substrings :: CString -> CString -> IO Int

main :: IO ()
main = do
    value <- newCString "banana"
    substr <- newCString "na"
    count_substrings value substr >>= print
```

Haskell strings need to be converted to `CString` values before passing them to the foreign function. Apart from that, the code is straightforward.

Compile, link and run in the same manner as C:

```sh
$ ghc --make main.hs -L../target/debug -lstringtools -o main
$ LD_LIBRARY_PATH=../target/debug ./main
2
```

Node.js
-------

This was as straightforward as with Python, thanks to the [ffi npm package](https://www.npmjs.com/package/ffi).

```javascript
var ffi = require('ffi');

var library_name = '../target/debug/libstringtools.so';
var stringtools = ffi.Library(library_name, {
      'count_substrings': ['int', ['string', 'string']]
});

console.log(stringtools.count_substrings("banana", "na"));
```

The first argument of the `Library` function is the location of our dynamic library. The object passed in the second argument defines a list of functions exported by the library, along with their signatures (in the form `[return type, [arguments]]`).

```sh
$ node main.js
2
```

The code for today is in a separate GitHub repository - [rust-ffi-stringtools](https://github.com/zsiciarz/rust-ffi-stringtools).

See also
--------

* [The Rust FFI omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
* [Calling Rust from C (and Python!)](http://harkablog.com/calling-rust-from-c-and-python.html)
* [Embed Rust in Haskell](https://github.com/creichert/haskellrustdemo)
* [A Ruby gem, implemented in Rust](https://github.com/steveklabnik/rust_example)
* [Bending the Curve: Writing Safe & Fast Native Gems With Rust](http://blog.skylight.io/bending-the-curve-writing-safe-fast-native-gems-with-rust/)
* [Using Rust from Perl and Julia](http://paul.woolcock.us/posts/rust-perl-julia-ffi.html)
