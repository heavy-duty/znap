use heck::ToSnekCase;

pub fn template(name: &str) -> String {
    format!(
        r#"
use shuttle_runtime::SecretStore;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {{
    let identity_keypair = secrets
        .get("IDENTITY_KEYPAIR")
        .or_else(|| panic!("IDENTITY_KEYPAIR is missing"))
        .unwrap();
    std::env::set_var("IDENTITY_KEYPAIR", identity_keypair);

    Ok({}::collection_router().into())
}}
"#,
        name.to_snek_case()
    )
}
