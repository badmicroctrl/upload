mod client;

use axum::{
    extract::Multipart,
    response::Html,
    routing::{get, post},
    Router,
};
use std::{fs::File, io::Write, net::SocketAddr};
use tokio_util::io::ReaderStream;
use uuid::Uuid;

async fn upload_handler(mut multipart: Multipart) -> Html<&'static str> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().unwrap_or("default.mp3").to_string();
        let unique_filename = format!("uploads/{}.mp3", Uuid::new_v4());

        let data = ReaderStream::new(field);
        let mut file = File::create(unique_filename).unwrap();

        tokio::pin!(data);

        while let Some(chunk) = data.next().await {
            let chunk = chunk.unwrap();
            file.write_all(&chunk).unwrap();
        }
    }

    Html("<h1>File uploaded successfully!</h1>")
}

#[tokio::main]
async fn main() {
    std::fs::create_dir_all("uploads").unwrap();

    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Upload Page</h1>") }))
        .route("/upload", post(upload_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
