use heck::ToKebabCase;

pub fn template(name: &str) -> String {
    format!(
        "[package]\n\
        name = \"{}\"\n\
        version = \"0.1.0\"\n\
        edition = \"2021\"\n\
        \n[dependencies]\n\
        axum = \"0.7.5\"\n\
        serde = \"1.0.203\"\n\
        solana-sdk = \"2.0.1\"\n\
        znap = \"0.1.36\"\n\
    ",
        name.to_kebab_case()
    )
}
