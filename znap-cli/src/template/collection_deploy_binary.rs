use heck::ToSnekCase;

pub fn template(name: &str) -> String {
    format!(
        r#"
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {{   
    Ok({}::collection_router().into())
}}
"#,
        name.to_snek_case()
    )
}
