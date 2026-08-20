use app::App;
use axum::routing::get;

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.with_route("/health", get("OK"));

    println!("server started on port 8080");
    app.serve(8080).await.expect("failed to serve app");
}
