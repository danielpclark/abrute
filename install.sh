#!/bin/bash -l

# Checking if Rust is installed
if [ `which rustc` ];
then
  # Rust is installed
  echo "Rust already installed"
else
  # Rust is not installed
  # Ask for user consent before proceeding with installation
  read -p "Rust is not installed. Do you want to install Rust now ? (necessary for Abrute) [y/n] " Rust_answer

  if [ "$Rust_answer" == "y" ];
  then
    # User agreed, proceed with installation
    curl https://sh.rustup.rs -sSf | sh -s -- -y
  else
    # User refused, abort installation
    exit 1
  fi
fi

# AES Crypt decryption is handled in-process by the `aescry` crate, so no
# external `aescrypt` binary is required any more.

# Checking for the existence of source files
if [ -d src ] && [ -d .git ];
then
  # The source files are already there. We can build and install Abrute.
  cargo build --release
  sudo cp target/release/abrute /usr/bin

  if [ -d /usr/local/share/man/man1 ];
  then
    sudo cp abrute.1 /usr/local/share/man/man1/
    sudo mandb -pq
  fi
else
  # Downloading the source files
  wget https://github.com/danielpclark/abrute/archive/master.zip
  unzip master.zip
  cd abrute-master

  # Building and installing
  bash -lc "cargo build --release"
  sudo cp target/release/abrute /usr/bin

  # Cleaning
  cd ..
  rm master.zip
  rm -r abrute-master/
fi

echo "Thank you for installing Abrute. Enjoy!"

