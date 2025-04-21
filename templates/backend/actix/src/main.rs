use actix_web::{web, App, HttpServer, HttpResponse, Responder, get, post};
use serde_json::json;
use std::env;
use dotenvy::dotenv;
use base64::{engine::general_purpose, Engine as _};

const SYNAPSE_API: &str = "https://api.connectome.fr";
const CLIENT_ID = env::var("SYNAPSE_ID").unwrap_or_default();
const CLIENT_SECRET = env::var("SYNAPSE_SECRET").unwrap_or_default();

const SIGNATURE = general_purpose::STANDARD.encode(format!("{}:{}", CLIENT_ID, CLIENT_SECRET));

// main route for Synapse authorization
#[post("/synapse/token")]
async fn synapse_token(query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let code = query.get("code").cloned().unwrap_or_default();
    let url = format!("{}/oauth/token?code={}", SYNAPSE_API, code);

    let client = reqwest::Client::new();
    let res = client.post(&url)
        .header("Authorization", format!("Basic {}", signature))
        .send()
        .await;


    // a JSON object is returned with token or error
    match res {
        Ok(response) => {
            let json: serde_json::Value = response.json().await.unwrap_or_else(|_| json!({"error": "JSON invalide"}));
            HttpResponse::Ok().json(json)
        },
        Err(_) => HttpResponse::InternalServerError().json(json!({"error": "Contact avec Synapse échoué"})),
    }
}

// static files (html, css, ...) served from /client
#[get("/{filename:.*}")]
async fn static_files(path: web::Path<String>) -> impl Responder {
    let filename = if path.is_empty() { "index.html" } else { &path };
    let full_path = format!("./client/{}", filename);

    match tokio::fs::read(full_path).await {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(_) => HttpResponse::NotFound().body("Fichier non trouvé"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(synapse_token)
            .service(static_files)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}