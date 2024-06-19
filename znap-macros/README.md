<div align="center">
  <h1>znap-macros</h1>

  <p>
    <strong>Macros for creating Solana actions</strong>
  </p>
</div>

The `znap-macros` module is a collection of macros designed to facilitate metaprogramming in Rust. This module relies on `znap-syn` for code parsing and generation, allowing developers to dynamically and efficiently manipulate and extend code.

Macros in Rust allow reading the current code and adding extra code, which is essential for automatic generation of functions, structures and other entities. `znap-macros` uses the `znap-syn` capability to parse the source code and transform it into manipulable objects, applying specific logic and then generating the necessary code.

### Solana Actions Macros

Macros in `znap-macros` allow you to transform Rust code snippets into `znap-syn`-defined objects such as `CollectionMod`, `ActionStruct`, `QueryStruct` and `ErrorEnum`. These macros facilitate the creation and manipulation of these objects to generate Solana actions.

## Package

| Package                 | Description                                              | Version                                                                                                                          | Docs                                                                                                            |
| :---------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `znap-macros`           | Macros for creating Solana actions           | [![Crates.io](https://img.shields.io/crates/v/anchor-lang?color=blue)](https://crates.io/crates/znap-macros)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-macros/latest/znap_macros/)                                |