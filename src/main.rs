use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::process::Command;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(processor())
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
    .service(hello)
    .service(echo)   

}).bind(("127.0.0.1", 9000))?
.run()
.await

    }

fn processor() ->  Vec<u8> {
    let output = Command::new("ipconfig")
    .output()
    .expect("Can't execute command");

    output.stdout
}