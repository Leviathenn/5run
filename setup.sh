#!/bin/bash
#@author Leviathenn
#@purpose Installing rust on a codespace.

install_cargo() {
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/rsins
    chmod +x /tmp/rsins
    sh /tmp/rsins -y
    rm -rf /tmp/rsins
    . "$HOME/.cargo/env"    
}

if [ -f "$HOME/.cargo/bin/cargo" ]; then
    echo "Rust already installed."
    exit 0
fi

if [ -f "/home/codespace/.config/fish" ]; then
    if [ -f "/home/codespace/.config/fish/conf.d" ]; then
        if [ -f "$HOME/.cargo/bin/cargo" ]; then
            echo "Rust already installed."
            exit 0
        else
            install_cargo
        fi

    else
        mkdir "/home/codespace/.config/fish/conf.d"   
    fi
else
    mkdir "/home/codespace/.config/fish"
fi
install_cargos