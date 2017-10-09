#!/bin/bash

rust_install_status=`rustc --version`
rustc_prefix="rustc"

# Checking if Rust is installed
if [[ $rust_install_status == $rustc_prefix* ]];
then
    # Rust is installed
    echo "Rust already installed"
else
    # Rust is not installed
    # Ask for user consent before proceeding with installation
    read -p "Rust is not installed. Do you want to install Rust now ? (necessary for Abrute) [y/n] " Rust_answer

    if [ $Rust_answer == y ];
    then
        # User agreed, proceed with installation
        curl https://sh.rustup.rs -sSf | sh
    else
        # User refused, abort installation
        exit 1
    fi
fi

# Checking if Aescrypt is installed
if [ -f /usr/bin/aescrypt ] && [ -f /usr/bin/aescrypt_keygen ];
then
    # Aescrypt is already installed
    echo "Aescrypt already installed"
else
    # Aescrypt is not installed, proceed with installation
    echo "Installing Aescrypt"
    wget https://www.aescrypt.com/download/v3/linux/aescrypt-3.13.tgz
    tar -xzf aescrypt-3.13.tgz
    cd aescrypt-3.13/src
    make && sudo make install
    cd ../.. && rm -rf aescrypt-3.13 && rm aescrypt-3.13.tgz
    echo "Aescrypt installed"
fi

# Checking for the existence of source files
if [ -d src ] && [ -d .git ];
then
    # The source files are already there. We can build and install Abrute.
    cargo build --release
    sudo cp target/release/abrute /usr/bin
else
    # Downloading the source files
    wget https://github.com/danielpclark/abrute/archive/master.zip
    unzip master.zip
    cd abrute-master
    
    # Building and installing
    cargo build --release
    sudo cp target/release/abrute /usr/bin

    # Cleaning
    cd ..
    rm master.zip
    rm -r abrute-master/
fi

echo "Thank you for installing Abrute. Enjoy !"

