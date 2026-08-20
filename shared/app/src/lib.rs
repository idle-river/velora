use axum::{Router, routing::MethodRouter};
use tokio::net::TcpListener;

type Error = Box<dyn std::error::Error>;

pub struct App {
    srv: axum::Router,
}

impl App {
    /// Create an instance of the app
    pub fn new() -> Self {
        Self { srv: Router::new() }
    }

    // Create a tokio listener and serve the app on the given port
    pub async fn serve(self, port: u16) -> Result<(), Error> {
        let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
        let listener = TcpListener::bind(format!("{}:{}", host, port)).await?;

        axum::serve(listener, self.srv).await?;
        Ok(())
    }

    // This adds a new route to the app
    pub fn with_route(&mut self, path: &str, route: MethodRouter) {
        self.srv = std::mem::take(&mut self.srv).route(path, route);
    }
}
