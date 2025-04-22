use actix_web::{web, App, HttpServer, HttpResponse, Responder, post};
use serde_json::json;
use std::env;
use dotenvy::dotenv;
use base64::{engine::general_purpose, Engine as _};
use actix_files::Files;

const SYNAPSE_API: &str = "https://api.connectome.fr";

type QueryType = web::Query<std::collections::HashMap<String, String>>;

// main route for Synapse authorization
#[post("/synapse/token")]
async fn synapse_token(query: QueryType) -> impl Responder {


    let client_id = env::var("SYNAPSE_ID").unwrap_or_default();
    let client_secret = env::var("SYNAPSE_SECRET").unwrap_or_default();

    let signature = general_purpose::STANDARD.encode(
        format!("{}:{}", client_id, client_secret));

    let code = query.get("code").cloned().unwrap_or_default();
    let url = format!(
        "{}/oauth/token?code={}&grant_type=authorization_code", 
        SYNAPSE_API, code
    );

    let client = reqwest::Client::new();
    let res = client.post(&url)
        .header("Authorization", format!("Basic {}", signature))
        .send()
        .await;


    // a JSON object is returned with token or error
    match res {
        Ok(response) => {
            let json: serde_json::Value = response.json().await.unwrap_or_else(
                |_| json!({"error": "JSON invalide"})
            );
            
            return HttpResponse::Ok().json(json)
        },
        Err(_) => HttpResponse::InternalServerError().json(
            json!({"error": "Contact avec Synapse échoué"})
        ),
    }
}

// static files (html, css, ...) served from /client
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .service(synapse_token)
            .service(Files::new("/", "./client").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}