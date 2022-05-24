// handler for the /data route

use axum::response::Html;

pub async fn data() -> Html<&'static str> {
    Html("<h1>Some data is available soon...</h1>")
}