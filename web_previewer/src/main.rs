use axum::{http::StatusCode, routing::post, Json, Router};
use image::imageops::FilterType;
use previewer::Size;
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/create_preview", post(create_preview))
        .fallback(|| async { StatusCode::NOT_FOUND });

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn create_preview(input: Json<PreviewParams>) -> StatusCode {
    let image = match image::open(&input.image_path) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Failed to open image: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let size = Size {
        width: input.width,
        height: input.height,
    };

    let filter = previewer::str_to_filter(&input.filter).unwrap_or(FilterType::Gaussian);
    
    let Ok(image) = previewer::preview(image, size, filter) else {
        eprintln!("Failed to create preview");
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    if let Err(e) = image.save(format!("preview_{}", input.image_path)) {
        eprintln!("Failed to save preview: {e}");
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    StatusCode::OK
}

#[derive(Debug, Deserialize)]
struct PreviewParams {
    image_path: String,
    width: u32,
    height: u32,
    filter: String,
}