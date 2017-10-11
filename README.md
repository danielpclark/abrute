[![Build Status](https://travis-ci.org/danielpclark/abrute.svg?branch=master)](https://travis-ci.org/danielpclark/abrute)

# abrute - Multi-threaded AES Brute Force File Decryption

Rather straight forward.  It works but has much more improvements yet to come.
This will use your CPU to the best of its ability so you may want to use this on
a secondary system.

## Installation

To use the install script you need to have the following commands available on your
system `curl wget unzip sudo cc` and possibly other build essentials for C.

---

For Ubuntu and Docker images of Ubuntu you can install these prerequisites with:

```bash
apt-get update && apt-get install curl wget unzip sudo build-essential
```

_NOTE: if you are not root you may need to precede `apt-get` with `sudo`_

---

The `install.sh` script will check for Rust and aescrypt and proceed to install them
if they are missing _(it will ask you if you want to install Rust)_.

First download the installation script locally.

```bash
curl https://raw.githubusercontent.com/danielpclark/abrute/master/install.sh > install.sh
```

Then verify that the contents of the installation script is safe to use (a no brainer).  Then run

```bash
bash install.sh
```

This install script will use sudo to copy the binary directly to `/usr/bin` and make the
command `abrute` available to you everywhere.

### Manual Installation

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
  USAGE:  abrute <RANGE> <CHARACTERS> [OPTIONS] -- <TARGET>
  
   <RANGE>         Single digit or a range 4:6 for password length.
   <CHARACTERS>    Characters to use in password attempt. Don't use quotes
                   unless they may be in the password. Backslash may escape
                   characters such as space.
   -a, --adjacent  Set a limit for allowed adjacent characters. Zero will
                   not allow any characters of the same kind to neighbor
                   in the attempts.
   -s, --start     Starting character sequence to begin with.
   <TARGET>        Target file to decrypt.  The target must be preceeded
                   by a double dash: -- target.aes
   -h, --help      Prints help information.
   -v, --version   Prints version information.
```

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](MIT-LICENSE) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
