[![Build Status](https://travis-ci.org/danielpclark/abrute.svg?branch=master)](https://travis-ci.org/danielpclark/abrute)

# abrute - AES Brute Force File Decryptor

Multithreaded AES file brute force decryptor.  Rather straight forward.
It works but has much more improvements yet to come.

## Usage

```
abrute - AES Brute Force File Decryptor
----------------------------------------
Usage: abrute RANGE CHARACTERS TARGET

 RANGE       Can be a single number or min max pair with a colon.
             eg) abrute 4:6 asdf1234 file.tar.aes

 CHARACTERS  Should not have quotes unless the password may have them.
```

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
