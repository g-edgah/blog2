use axum::{
    // http::StatusCode,
    Json,
    Router,
    response::IntoResponse,
    routing::get,
};
use rusqlite::Connection;
use std::fs;

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

    let app = Router::new().route("/get_posts", get(get_posts));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_posts() -> impl IntoResponse {
    let con = rusqlite::Connection::open("./data.db3").unwrap();

    println!("getting posts");

    let mut stm = con
        .prepare("SELECT id, title, body, time_created FROM posts ORDER BY id DESC")
        .unwrap();

    let rows = stm
        .query_map([], |row| {
            Ok(Post {
                id: row.get(0).unwrap(),
                title: row.get(1).unwrap(),
                body: row.get(2).unwrap(),
                time_created: row.get(3).unwrap(),
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

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Post {
    id: u32,
    title: String,
    body: String,
    time_created: String,
}
