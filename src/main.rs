use std::env;
use axum::{
    Router,
    response::{IntoResponse, Redirect},
    routing::get,
    http::{StatusCode, Response},
    body::Body,
    extract::{Path, Query},
};
use std::collections::HashMap;
use url::Url;
use serde_json::{json, Value};

mod search;
use search::search_product;
mod product;
use product::product_details;

// Remove duplicate handler definitions - either use these OR the ones from handlers module
async fn search_handler(query: Option<Path<String>>) -> Response<Body> {
    let Path(query) = query.unwrap_or(Path("".to_string()));
    let data = search_product(query).await;
    if let Err(err) = data {
        return Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"error": err.to_string()}).to_string()))
            .unwrap_or_else(|_| Response::new(Body::empty()));
    }

    let data = data.unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

async fn product_handler(
    Path(url): Path<String>,
    q: Option<Query<HashMap<String, String>>>,
) -> Response<Body> {
    let url = if let Some(Query(q)) = q {
        Url::parse_with_params(&format!("https://www.flipkart.com/{}", url), q.iter())
    } else {
        Url::parse(&format!("https://www.flipkart.com/{}", url))
    };
    if let Err(e) = url {
        return Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"error": e.to_string()}).to_string()))
            .unwrap_or_else(|_| Response::new(Body::empty()));
    }
    let url = url.unwrap();
    let data = product_details(url).await;

    if let Err(e) = data {
        return Response::builder()
            .status(StatusCode::BAD_GATEWAY)
            .header("Content-Type", "application/json")
            .body(Body::from(json!({"error": e.to_string()}).to_string()))
            .unwrap_or_else(|_| Response::new(Body::empty()));
    }

    let data = data.unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

#[tokio::main]
async fn main() {
    // Get port from environment variable for Render compatibility
    let port = env::var("PORT").unwrap_or_else(|_| "10000".to_string());
    let host = "0.0.0.0";
    let deployment_url = format!("{}:{}", host, port);
    
    let base_url = env::var("RENDER_EXTERNAL_URL")
        .or_else(|_| env::var("DEPLOYMENT_URL"))
        .unwrap_or_else(|_| format!("http://{}:{}", host, port));
    
    let description: Value = json!({
        "name": env!("CARGO_PKG_NAME"),
        "description": env!("CARGO_PKG_DESCRIPTION"),
        "version": env!("CARGO_PKG_VERSION"),
        "authors": env!("CARGO_PKG_AUTHORS"),
        "repository": env!("CARGO_PKG_REPOSITORY"),
        "license": env!("CARGO_PKG_LICENSE"),
        "usage": {
            "search_api": format!("{}/search/{{product_name}}", base_url),
            "product_api": format!("{}/product/{{product_link_argument}}", base_url),
        }
    });

    // Rest of your code remains the same
    let app = Router::new()
        // ... existing routes

    println!("Starting server on {}", deployment_url);
    let listener = tokio::net::TcpListener::bind(&deployment_url)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to bind to {}: {}", deployment_url, e);
            std::process::exit(1);
        });
    
    println!("Server running at http://{}", deployment_url);
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    });
}