use actix_web::{test, web, App, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::Migrator, PgPool};

#[path = "../src/db.rs"]
mod db;

#[derive(Deserialize, Serialize)]
struct AddRequest {
    text: String,
    parent_id: Option<i32>,
}

#[derive(Deserialize, Serialize)]
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

use std::process::Command;
use tempfile::TempDir;

#[actix_rt::test]
async fn test_endpoints() {
    // initialize temporary postgres instance
    let output = Command::new("sudo")
        .args(["-u", "postgres", "mktemp", "-d"])
        .output()
        .expect("mktemp failed");
    let dir_str = String::from_utf8_lossy(&output.stdout);
    let dir = dir_str.trim();
    Command::new("sudo")
        .args([
            "-u",
            "postgres",
            "/usr/lib/postgresql/16/bin/initdb",
            "-D",
            dir,
        ])
        .status()
        .expect("initdb failed");
    Command::new("sudo")
        .args([
            "-u",
            "postgres",
            "/usr/lib/postgresql/16/bin/pg_ctl",
            "-D",
            dir,
        ])
        .args(["-o", "-F -p 5433", "-w", "start"])
        .status()
        .expect("pg_ctl start failed");
    Command::new("sudo")
        .args([
            "-u",
            "postgres",
            "/usr/lib/postgresql/16/bin/createdb",
            "-p",
            "5433",
            "testdb",
        ])
        .status()
        .expect("createdb failed");

    let db_url = "postgres://postgres@localhost:5433/testdb".to_string();
    let pool = db::init_pool(&db_url).await.unwrap();
    let migrator = Migrator::new(std::path::Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/migrations"
    )))
    .await
    .unwrap();
    migrator.run(&pool).await.unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/todos", web::get().to(list_todos))
            .route("/todos", web::post().to(add_todo))
            .route("/todos/{id}/mark", web::post().to(mark_todo)),
    )
    .await;

    // add item
    let add_req = test::TestRequest::post()
        .uri("/todos")
        .set_json(&AddRequest {
            text: "first".into(),
            parent_id: None,
        })
        .to_request();
    let added: db::TodoItem = test::call_and_read_body_json(&app, add_req).await;
    assert_eq!(added.text, "first");

    // list items
    let list_req = test::TestRequest::get().uri("/todos").to_request();
    let list: Vec<db::TodoItem> = test::call_and_read_body_json(&app, list_req).await;
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].text, "first");

    // mark item
    let mark_req = test::TestRequest::post()
        .uri(&format!("/todos/{}/mark", added.id))
        .set_json(&MarkRequest { mark: true })
        .to_request();
    let _ = test::call_service(&app, mark_req).await;

    // verify database state
    let items = db::list_items(&pool).await.unwrap();
    assert_eq!(items[0].mark, true);

    Command::new("sudo")
        .args(["-u", "postgres", "/usr/lib/postgresql/16/bin/pg_ctl", "-D"])
        .arg(dir)
        .args(["-m", "fast", "stop"])
        .status()
        .expect("pg_ctl stop failed");
    std::fs::remove_dir_all(dir).ok();
}
