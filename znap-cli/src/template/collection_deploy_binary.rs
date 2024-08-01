use heck::ToSnekCase;

pub fn template(name: &str) -> String {
    format!(
        r#"
#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {{
    let identity_keypair = secrets.get("IDENTITY_KEYPAIR").context("IDENTITY_KEYPAIR was not found")?;
    std::env::set_var("IDENTITY_KEYPAIR", identity_keypair);

    Ok({}::collection_router().into())
}}
"#,
        name.to_snek_case()
    )
}
