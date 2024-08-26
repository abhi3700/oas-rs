use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router};
use mongodb::bson::doc;
use serde_json::json;

/// Health check of API server.
pub(crate) async fn health_check(
	Extension(db_client): Extension<Arc<mongodb::Client>>,
) -> impl IntoResponse {
	match db_client.database("admin").run_command(doc! {"ping": 1}).await {
		Ok(_) => {
			let response = json!({
				"status": StatusCode::OK.to_string(),
				"data": "Health check passed",
			});

			(StatusCode::OK, Json(response)).into_response()
		},
		Err(e) => {
			println!("Error: {:?}", e);
			(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
		},
	}
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> eyre::Result<()> {
	// initialize tracing
	tracing_subscriber::fmt::init();
	dotenv::dotenv().ok();

	let db_client = mongodb::Client::with_uri_str(
		std::env::var("MONGODB_URI").expect("MONGODB_URI is not set"),
	)
	.await?;

	let db_client = Arc::new(db_client);

	let port = std::env::var("PORT").expect("PORT is not set");

	// let cors = CorsLayer::new()
	// 	.allow_origin("https://omnipay-web-app.vercel.app/".parse::<HeaderValue>()?) // Allow requests only from your frontend's origin
	// 	.allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
	// 	.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

	/* Router */
	let app = Router::new().route("/health", get(health_check)).layer(Extension(db_client));

	/* Socket Address = IP Address + Port */
	// NOTE: 0.0.0.0 is the default IP address for the local machine. Doesn't look for URL here. So,
	// API_BASE_URL is NOT used.
	let socket_addr = &format!("0.0.0.0:{}", port);
	let listener = tokio::net::TcpListener::bind(socket_addr).await?;
	tracing::info!("Listening on {}", socket_addr);

	/* Start the server */
	axum::serve(listener, app).await?;

	Ok(())
}
