mod database;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rusqlite::Connection;

// expect("Invalid SQL Query")

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

#[get("/")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body(
        r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <title>Redirect Test</title>
  </head>
  <body>
    <img src="http://localhost:3000/asdf">
  </body>
</html>"#,
    )
}

#[get("/test")]
async fn test() -> impl Responder {
    "sample text"
}

#[get("/{user}/{link}")]
async fn redirect(path: web::Path<(String, String)>) -> HttpResponse {
    let (user, link) = path.into_inner();

    println!("{user} /::/ {link}");

    HttpResponse::Found()
        .insert_header((
            "Location",
            "https://rustacean.net/assets/rustacean-orig-noshadow.png",
        ))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open("/home/watduhhekbro/test.db").unwrap();

    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        )",
        (),
    )
    .unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let person_iter = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
                data: row.get(2)?,
            })
        })
        .unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.expect("Invalid SQL Query"));
    }

    HttpServer::new(|| App::new().service(redirect).service(greet).service(test))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
