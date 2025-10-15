use crate::app_state::AppState;
use axum::{routing::post, serve::Serve, Router};
use std::error::Error;
use std::sync::Arc;
use tower_http::services::ServeDir;

pub mod app_state;
mod domain;
pub mod routes;
pub mod services;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/logout", post(routes::logout))
            .route("/verify-token", post(routes::verify_token))
            .with_state(Arc::new(app_state));

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}
