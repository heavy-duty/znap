<div align="center">
  <h1>znap-syn</h1>

  <p>
    <strong>Parsing and generating code for macros in Rust</strong>
  </p>
</div>

The `znap-syn` module is a fundamental part of the znap library ecosystem, designed specifically for analyzing and generating code in Rust. This module is essential for the functionality of `znap-macros`, a collection of macros that depends on `znap-syn` for code generation and manipulation.

`znap-syn` allows you to transform Rust code fragments into manipulable objects, which can then be used to apply specific logic and generate new code.

### Main components

- **CollectionMod:** A CollectionMod is a structured representation of a Rust module. This object contains:

    - **A list of `ActionFn`:** functions that define actions within the module. 
    - **A `name`:** the name of the module. 
    - **The module in its `raw` form:** the normal structure of the module before being transformed.

In addition to the `CollectionMod`, `znap-syn` handles other object types such as `ActionStruct`, `QueryStruct`, and `ErrorEnum`. These are also derived from code snippets and used for various transformations and code generation.

## Package

| Package                 | Description                                              | Version                                                                                                                          | Docs                                                                                                            |
| :---------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `znap-syn`           | Parsing and generating code for macros in Rust           | [![Crates.io](https://img.shields.io/crates/v/anchor-lang?color=blue)](https://crates.io/crates/znap-syn)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap-syn/latest/znap_syn/)                                |