<div align="center">
  <h1>Znap</h1>

  <p>
    <strong>Solana Actions/Blinks Framework</strong>
  </p>

  <p>
    <a href="https://github.com/heavy-duty/znap/tree/master/examples/my-actions"><img alt="Tutorials" src="https://img.shields.io/badge/docs-tutorials-blue" /></a>
  </p>
</div>

Znap is an innovative Rust-based framework designed to simplify the creation of Solana Actions and Blinks on the Solana blockchain.

- Rust eDSL for writing Solana actions
- CLI and workspace management for developing complete Solana actions

If you're familiar with developing using the Anchor framework, then the experience will be familiar.

## Getting Started

1. `cargo install znap-cli`
2. `znap init <my-project-name>`
3. `cd <my-project-name>`
4. `znap new <collection-name>`

## Packages

| Package                 | Description                                              | Version                                                                                                                          | Docs                                                                                                            |
| :---------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `znap-syn`           | Parsing and generating code for macros in Rust           | [![Crates.io](https://img.shields.io/crates/v/anchor-lang?color=blue)](https://crates.io/crates/znap-syn)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-syn/latest/znap_syn/)                                |
| `znap-macros`           | Macros for creating Solana actions           | [![Crates.io](https://img.shields.io/crates/v/anchor-lang?color=blue)](https://crates.io/crates/znap-macros)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-macros/latest/znap_macros/)                                |
| `znap-cli`           | Znap CLI for writing Solana actions           | [![Crates.io](https://img.shields.io/crates/v/anchor-lang?color=blue)](https://crates.io/crates/znap-cli)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-cli/latest/znap_cli/)                                |