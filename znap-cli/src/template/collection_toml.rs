use heck::ToKebabCase;

pub fn template(name: &str) -> String {
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

[[bin]]
name = "{}"
path = "src/bin/deploy.rs"

[[bin]]
name = "serve"
path = "src/bin/serve.rs"
"#,
        name.to_kebab_case()
    )
}
