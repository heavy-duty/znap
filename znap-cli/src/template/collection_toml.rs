use heck::ToKebabCase;

pub fn template(name: &String) -> String {
    format!("[package]\n\
        name = \"{}\"\n\
        version = \"0.1.0\"\n\
        edition = \"2021\"\n\
        \n[dependencies]\n\
        axum = \"0.7.5\"\n\
        serde = \"1.0.203\"\n\
        solana-sdk = \"1.18.16\"\n\
        spl-associated-token-account = \"3.0.2\"\n\
        spl-token = \"4.0.1\"\n\
        znap = \"0.1.1\"\n\
    ", name.to_kebab_case())
}

