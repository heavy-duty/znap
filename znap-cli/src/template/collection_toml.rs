use heck::ToKebabCase;

pub fn template(name: &String) -> String {
    format!(
        r#"
    
# binaries dependencies
colored = "2.1.0"
console = "0.15.8"

# serve dependencies
tokio = {{ version = "1", features = ["full"] }}

# deploy dependencies
shuttle-axum = "0.46.0"
shuttle-runtime = "0.46.0"

[workspace]
members = []

[[bin]]
name = "{}"
path = "src/bin/deploy.rs"

[[bin]]
name = "serve"
path = "src/bin/serve.rs"

# hack required due to solana dependency on curve25519-dalek@3.2.1
[patch.crates-io]
curve25519-dalek = {{ git = "https://github.com/dalek-cryptography/curve25519-dalek", rev = "8274d5cbb6fc3f38cdc742b4798173895cd2a290" }}
"#,     
        name.to_kebab_case()
    )
}
