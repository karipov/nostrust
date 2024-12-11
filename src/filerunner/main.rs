use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::fs;

const DB_PATH: &str = "db.blob";
const SEALDATA_PATH: &str = "sealdata.blob";
const FILERUNNER_SERVER: &str = "0.0.0.0:5555";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/set-db", post(set_db))
        .route("/get-db", get(get_db))
        .route("/set-sealdata", post(set_sealdata))
        .route("/get-sealdata", get(get_sealdata));

    println!("Running filerunner on {}", FILERUNNER_SERVER);
    let listener = tokio::net::TcpListener::bind(FILERUNNER_SERVER)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn set_db(body: String) -> Result<StatusCode, StatusCode> {
    println!("Setting db");
    fs::write(DB_PATH, body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

async fn get_db() -> Result<String, StatusCode> {
    println!("Getting db");
    let content = fs::read_to_string(DB_PATH).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(content)
}

async fn set_sealdata(body: String) -> Result<StatusCode, StatusCode> {
    println!("Setting sealdata");
    fs::write(SEALDATA_PATH, body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

async fn get_sealdata() -> Result<String, StatusCode> {
    println!("Getting sealdata");
    let content =
        fs::read_to_string(SEALDATA_PATH).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(content)
}
