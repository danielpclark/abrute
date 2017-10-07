#!/bin/bash

# installing aescrypt
wget https://www.aescrypt.com/download/v3/linux/aescrypt-3.13.tgz
tar -xzf aescrypt-3.13.tgz
cd aescrypt-3.13/src
make && sudo make install
cd ../.. && rm -rf aescrypt-3.13

# installing Rust
curl https://sh.rustup.rs -sSf | sh

