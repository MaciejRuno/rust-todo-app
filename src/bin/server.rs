use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::PgPool;

#[path = "../config.rs"]
mod config;
#[path = "../db.rs"]
mod db;

use config::Config;

#[derive(serde::Deserialize)]
struct AddRequest {
    text: String,
    parent_id: Option<i32>,
}

#[derive(serde::Deserialize)]
struct MarkRequest {
    mark: bool,
}

async fn list_todos(pool: web::Data<PgPool>) -> impl Responder {
    match db::list_items(&pool).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn add_todo(pool: web::Data<PgPool>, req: web::Json<AddRequest>) -> impl Responder {
    match db::create_item(&pool, &req.text, req.parent_id).await {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn mark_todo(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    req: web::Json<MarkRequest>,
) -> impl Responder {
    match db::mark_item(&pool, path.into_inner(), req.mark).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_todo(pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    match db::delete_item(&pool, path.into_inner()).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let pool = db::init_pool(&config.database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/todos", web::get().to(list_todos))
            .route("/todos", web::post().to(add_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/todos/{id}/mark", web::post().to(mark_todo))
            .service(Files::new("/", "static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
