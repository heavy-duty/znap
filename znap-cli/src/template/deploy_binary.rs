pub fn template() -> String {
    format!(
        r#"
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {{   
    Ok(collection_router.into())
}}
"#)
}
