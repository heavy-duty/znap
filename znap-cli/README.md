<div align="center">
  <h1>znap-cli</h1>

  <p>
    <strong>Command Line Interface for Znap</strong>
  </p>
</div>

`znap-cli` is a command line tool designed to simplify and speed up the creation of Solana Actions and Blinks on different operating systems.

## How to install

1. `cargo install znap-cli`

## Command list

| Command                 | Description                                              |
| :---------------------- | :------------------------------------------------------- |
| `znap build`           | Builds all collections from the workspace           |
| `znap serve`           | Serves all collections from the workspace           |
| `znap test`           | Runs the test suite for the workspace           |
| `znap clean`           | Cleans all the temp files           |
| `znap init`           | Initializes a new workspace           |
| `znap new`           | Create a new collection in the workspace           |
| `znap help`           | Print this message or the help of the given subcommand(s)           |

## Package

| Package                 | Description                                              | Version                                                                                                                          | Docs                                                                                                            |
| :---------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `znap-cli`           | Znap CLI to interact with a znap workspace.           | [![Crates.io](https://img.shields.io/crates/v/anchor-lang?color=blue)](https://crates.io/crates/znap-cli)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-cli/latest/znap_cli/)                                |