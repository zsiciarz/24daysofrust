# Day 21 - rust-crypto

> Relevancy: 1.6 stable

The [rust-crypto](https://crates.io/crates/rust-crypto) crate is a collection of a lot of cryptography primitives and algorithms. There are tools for calculating hashes, verifying data integrity, encryption  etc. One disclaimer - it hasn't had a proper security audit yet and although the algorithms are well known and researched, the library itself *might* have security bugs. But [which](http://heartbleed.com/) [one](https://www.mozilla.org/en-US/security/advisories/mfsa2014-73/) [doesn't](http://www.gnutls.org/security.html)?

Cryptographic hashes
--------------------

Let's start with a simple task of computing a [cryptographic hash](http://en.wikipedia.org/wiki/Cryptographic_hash_function) of some value. We'll use the SHA-256 algorithm to demonstrate.

```rust
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
    let input = "Hello world!";
    let mut sha = Sha256::new();
    sha.input_str(input);
    println!("{}", sha.result_str());
}
```

All hash algorithms in rust-crypto implement the `Digest` trait, which defines the low level methods such as `input()` and `result()` operating on bytes, but also convenience string-based methods as shown above. We can use those low level methods for example to represent our hash as a base64-encoded string:

```rust
extern crate rustc_serialize;

use rustc_serialize::base64::{STANDARD, ToBase64};
use std::iter::repeat;

let mut bytes: Vec<u8> = repeat(0u8).take(sha.output_bytes()).collect();
sha.result(&mut bytes[..]);
println!("{}", bytes.to_base64(STANDARD));
```

Here's the output:

```sh
$ cargo run
c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a
wFNeS+K3n/2TKRMFQ2v4iTFOSj+uwF7P/Lt98xrZ5Ro=
```

Ciphers
-------

To actually encrypt some data in a way that we can decrypt it back later, we need a **cipher**. A few of these are provided by the `rust-crypto` crate, here's an example of [AES](http://en.wikipedia.org/wiki/Advanced_Encryption_Standard) encryption in CTR mode:

```rust
use crypto::aes::{self, KeySize};
use crypto::symmetriccipher::SynchronousStreamCipher;

use rand::{OsRng, Rng};

let mut gen = OsRng::new().ok().expect("Failed to get OS random generator");
let mut key: Vec<u8> = repeat(0u8).take(16).collect();
gen.fill_bytes(&mut key[..]);
let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
gen.fill_bytes(&mut nonce[..]);
println!("Key: {}", key.to_base64(STANDARD));
println!("Nonce: {}", nonce.to_base64(STANDARD));
let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
let secret = "I like Nickelback";
let mut output: Vec<u8> = repeat(0u8).take(secret.len()).collect();
cipher.process(secret.as_bytes(), &mut output[..]);
println!("Ciphertext: {}", output.to_base64(STANDARD));
```

We generate the secret key and a nonce value with a secure random generator - `OsRng`. Then we need to call the `ctr()` function which returns a *best possible* implementation of AES-CTR cipher (taking into consideration CPU architecture etc.). What we get back is a trait object - a `SynchronousStreamCipher` value in a `Box`. Calling `process()` should encrypt our secret message and store the result bytes in the `output` vector.

Here's the output:

```sh
$ cargo run
Key: NvDy+u51EfMC+amJzoJO+w==
Nonce: d5+SLyfPGUSeug50nK1WGA==
Ciphertext: vTVxjyUms4Z4jex/OcMcQlY=
```

We can decrypt this with Python (who said it would be Rust all the way?):

```python
import base64
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend

key = base64.decodebytes(b'NvDy+u51EfMC+amJzoJO+w==')
nonce = base64.decodebytes(b'd5+SLyfPGUSeug50nK1WGA==')
ct = base64.decodebytes(b'vTVxjyUms4Z4jex/OcMcQlY=')
backend = default_backend()
cipher = Cipher(algorithms.AES(key), modes.CTR(nonce), backend=backend)
decryptor = cipher.decryptor()
print(decryptor.update(ct) + decryptor.finalize())
```

```sh
$ python3 decrypt.py
b'I like Nickelback'
```

HMAC
----

Message authentication algorithms such as [HMAC](http://en.wikipedia.org/wiki/Hash-based_message_authentication_code) verify **both** data integrity and authenticity. Let's see how to generate a MAC for a given message:

```rust
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use rustc_serialize::hex::ToHex;

let mut hmac_key: Vec<u8> = repeat(0u8).take(32).collect();
gen.fill_bytes(&mut hmac_key);
let message = "Ceterum censeo Carthaginem esse delendam";
println!("Message: {}", message);
println!("HMAC key: {}", hmac_key.to_base64(STANDARD));
let mut hmac = Hmac::new(Sha256::new(), &hmac_key);
hmac.input(message.as_bytes());
println!("HMAC digest: {}", hmac.result().code().to_hex());
```

As with the AES cipher example, we generate a random secret key. Then we create the `Hmac` value, passing a cryptographic hash function object (anything that implements `Digest`). Since `Hmac` doesn't have these convenience methods for working with strings, we manually feed the bytes and encode the digest to hexadecimal string. The program outputs a key (we should give it to the recipient of the message through some other secure channel) and an HMAC digest.

```sh
$ cargo run
Message: Ceterum censeo Carthaginem esse delendam
HMAC key: esU5jdGCbM7E/ME5WBECJ+BdX3kt7bcQ3HkeEK+W6ZQ=
HMAC digest: b3240371a17e1e9755b89b23449f0d85c4c361e94e081c7adbe5a89c2d901aaa
```

And here's a simple Python program to verify validity of the message using the [hmac module](https://docs.python.org/3.4/library/hmac.html):

```python
import base64
import hashlib
import hmac

key = base64.decodebytes(b'esU5jdGCbM7E/ME5WBECJ+BdX3kt7bcQ3HkeEK+W6ZQ=')
message = b'Ceterum censeo Carthaginem esse delendam'
expected = 'b3240371a17e1e9755b89b23449f0d85c4c361e94e081c7adbe5a89c2d901aaa'
h = hmac.new(key, message, hashlib.sha256)
print(hmac.compare_digest(h.hexdigest(), expected))
```

So... is the message valid and authentic?

```sh
$ python3 verify.py
True
```

See also
--------

 * [Thoughts on Rust cryptography](https://speakerdeck.com/tarcieri/thoughts-on-rust-cryptography)
 * [Cryptography 1](https://www.coursera.org/course/crypto) - an introductory cryptography course - recommended!
 * [extensive comment](https://github.com/DaGenix/rust-crypto/blob/340cc5f142601077d6838eb6aa0c3b29b7f67358/src/rust-crypto/aessafe.rs#L9) in rust-crypto about AES timing issues
 * [Should we MAC-then-encrypt or encrypt-then-MAC?](http://crypto.stackexchange.com/questions/202/should-we-mac-then-encrypt-or-encrypt-then-mac) (short answer: **encrypt-then-MAC**)
