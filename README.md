[![Build Status](https://travis-ci.org/danielpclark/abrute.svg?branch=master)](https://travis-ci.org/danielpclark/abrute)

# abrute - Multi-threaded AES Brute Force File Decryption

Rather straight forward.  It works but has much more improvements yet to come.
This will use your CPU to the best of its ability so you may want to use this on
a secondary system.

## Installation

First you need to have aescrypt installed.

```bash
wget https://www.aescrypt.com/download/v3/linux/aescrypt-3.13.tgz
tar -xzf aescrypt-3.13.tgz
cd aescrypt-3.13/src
make && sudo make install
cd ../.. && rm -rf aescrypt-3.13
```

Next you need to have [Rust installed](https://www.rust-lang.org/en-US/install.html).  

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then you can get and compile abrute.

```bash
git clone https://github.com/danielpclark/abrute.git
cd abrute
cargo build --release
sudo cp target/release/abrute /usr/bin/
```

## Usage

```
abrute - AES Brute Force File Decryptor
----------------------------------------
Usage: abrute RANGE CHARACTERS TARGET

 RANGE       Can be a single number or min max pair with a colon.
             eg) abrute 4:6 asdf1234 file.tar.aes

 CHARACTERS  Should not have quotes unless the password may have them.
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
