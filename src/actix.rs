use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use metacall::{loaders, metacall, switch};
use metacall_ssr_benchmark::fib;

#[get("/fib/{num}")]
async fn fib_route(num: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(metacall::<String>("Fibonacci", [num.clone() as i64, fib(num.into_inner()) as i64]).unwrap())
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(metacall::<String>("Hello", [name.into_inner()]).unwrap())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx").unwrap();
    let addr = "0.0.0.0:8080".to_string();

    println!("running on http://{}", addr);

    // Actix has automated listeners for shutdown.
    HttpServer::new(|| App::new().service(hello).service(fib_route))
        .bind(addr)
        .unwrap()
        .run()
        .await
        .unwrap();
}
