mod util;

use crate::util::port;
//use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;

/*#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    #[cfg(debug_assertions)]
    let addr = "127.0.0.1";
    #[cfg(not(debug_assertions))]
    let addr = "0.0.0.0";

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(manual_hello)
        //.route("/hey", web::get().to(manual_hello))
    })
    .bind((addr, port()))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[cfg(test)]
mod tests {
    #[test]
    fn asdf() {
        //
    }
}*/

use actix::{Actor, StreamHandler};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

/// Define HTTP actor
struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/gateway", web::get().to(index))
    })
    .bind(("127.0.0.1", port()))?
    .run()
    .await
}
