<div align="center">
  <h1>znap</h1>

  <p>
    <strong>Znap framework's core library to create Solana actions</strong>
  </p>
</div>

`znap` is the core library that encompasses `znap-macros`, znap-syn and other essential components for Solana actions and Blinks programming.

`znap` is responsible for coordinating the different modules and tools needed to parse, transform and generate code in Rust. By integrating `znap-macros` and `znap-syn`, `znap` allows developers to take full advantage of Rust's capabilities to create Solana actions and Blinks.

## How to import znap

1. `cargo install znap`
2. In your lib.rs file import: `use znap::*`

## How to use

```
#[derive(Action)]
#[action(
    icon = // icon url,
    title = // action title,
    description = // action description,
    label = // action label
)]

#[derive(ErrorCode)]
enum ActionError {
    #[error(msg = "Invalid account PublicKey")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid Transaction")]
    InvalidTransaction,
}
```

## Package

| Package                 | Description                                              | Version                                                                                                                          | Docs                                                                                                            |
| :---------------------- | :------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------- |
| `znap`           | Znap framework's core library to create Solana actions           | [![Crates.io](https://img.shields.io/crates/v/anchor-lang?color=blue)](https://crates.io/crates/znap)                     | [![Docs.rs](https://docs.rs/anchor-lang/badge.svg)](https://docs.rs/znap/latest/znap/)                                |