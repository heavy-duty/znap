# Installation Guide

This guide explains how to set up your development environment to work with Znap.

## Windows (WSL)

For Windows users, you need to install and configure Windows Subsystem for Linux (WSL) to follow this guide.

You can learn how to install and configure WSL [here](https://learn.microsoft.com/en-us/windows/wsl/install).

Be sure to restart your computer when installation is done, then continue this guide.

## Linux and macOS

1. The first thing you need to do is install Rust and Cargo with the following command: `curl https://sh.rustup.rs -sSf | sh`.

2. If the installation was successful, you will see the following message: `Rust is installed now. Great!`. You can find the complete Rust installation guide [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

3. To check if the Rust installation was successful, run: `rustc --version`. You should see a message with the Rust version, commit ID, and date, similar to the following example: `rustc x.y.z (abcabcabc yyyy-mm-dd)`.

4. To check if the Cargo installation was successful, run: `cargo --version`. You should see a message with the Cargo version, commit ID, and date, similar to the following example: `cargo x.y.z (abcabcabc yyyy-mm-dd)`.

5. Now, install `znap-cli` using Cargo with the following command: `cargo install znap-cli`.

6. To check if the Znap installation was successful, run: `znap --version`. You should see a message with the Znap version, similar to the following example: `znap-cli x.y.z`.

### And that's it!

Now you can start creating your projects with Znap. To create your first project, you can follow the tutorial [here](#).