#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(my_znap_workspace::collection_router().into())
}
