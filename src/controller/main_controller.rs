use actix_web::{get, web::{self, Json}, Responder, HttpResponse};
use crate::helper::{response::ResponseJSON, transformer::{UserDB, Transform}};
use serde_json::json;

#[get("/")]
async fn hello() -> impl Responder {
    // Simulasi data dari database
    let user = UserDB {
        id: 1,
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
        created_at: "2025-06-05".to_string(),
    };

    // Transform data menggunakan trait Transform
    let transformed_user = user.transform();

    let obj = ResponseJSON {
        message: "Hello, World!".to_string(),
        values: json!({
            "user": transformed_user,
            "metadata": {
                "version": "1.0",
                "timestamp": "2025-06-05"
            }
        })
    };

    HttpResponse::Ok().json(obj)
}