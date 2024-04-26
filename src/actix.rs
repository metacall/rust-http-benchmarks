use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use metacall::{loaders, metacall, switch};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(metacall::<String>("Hello", [String::from("Metacall!")]).unwrap())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx").unwrap();
    let addr = "0.0.0.0:8080".to_string();

    println!("running on http://{}", addr);

    HttpServer::new(|| App::new().service(hello))
        .bind(addr)?
        .run()
        .await
}
