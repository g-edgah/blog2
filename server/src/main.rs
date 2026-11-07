use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use rusqlite::Connection;
use std::fs;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    if let Err(_e) = fs::File::open("./data.db3") {
        let con: Connection = rusqlite::Connection::open("./data.db3").unwrap();

        let sql: &str = "CREATE TABLE posts (
            id              INTEGER PRIMARY KEY,
            title           TEXT NOT NULL,
            body            TEXT NOT NULL,
            image_file      TEXT,
            time_created    TIMESTAMP
        )";

        con.execute(sql, []).unwrap();
    }

    let app: Router = Router::new()
        .nest_service("/images", ServeDir::new("images"))
        .route("/get_posts", get(get_posts))
        .route("/create_post", post(create_post))
        .layer(CorsLayer::permissive());

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_posts() -> impl IntoResponse {
    let con = rusqlite::Connection::open("./data.db3").unwrap();

    println!("getting posts");

    let mut stm = con
        .prepare("SELECT id, title, body, image_file, time_created FROM posts ORDER BY id DESC")
        .unwrap();

    let rows = stm
        .query_map([], |row| {
            Ok(Post {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                body: row.get(2).unwrap(),
                image_file: row.get(3).unwrap(),
                time_created: row.get(4).unwrap(),
            })
        })
        .unwrap();

    let mut posts: Vec<Post> = Vec::new();

    println!("gotten them posts");

    for row in rows {
        let post = row.unwrap();
        posts.push(post);
        println!("pushing post")
    }

    println!("after for statement");

    Json(posts)
}

async fn create_post(Json(post): Json<PostEntry>) -> impl IntoResponse {
    println!("boom");
    println!("{}", post.title);

    let con: Connection = Connection::open("./data.db3").unwrap();

    if let Some(image_data) = post.image_data {
        let image_id: uuid::Uuid = uuid::Uuid::new_v4();

        let file_name: String = format!("{}.png", image_id);

        let file_path: String = format!("./images/{}.png", image_id);

        std::fs::write(&file_path, image_data).unwrap();

        let sql = "
            INSERT INTO posts (
                title,
                body,
                image_file,
                time_created
            )
            VALUES (
                ?1,
                ?2,
                ?3,
                DATE('now')
        )";

        con.execute(sql, [&post.title, &post.body, &file_name])
            .unwrap();
    } else {
        let sql = "
            INSERT INTO posts (
                title,
                body,
                time_created
            )
            VALUES (
                ?1,
                ?2,
                DATE('now')
        )";

        con.execute(sql, [&post.title, &post.body]).unwrap();
    }

    StatusCode::OK
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Post {
    id: u32,
    title: String,
    body: String,
    image_file: Option<String>,
    time_created: String,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PostEntry {
    title: String,
    body: String,
    image_data: Option<Vec<u8>>,
}
