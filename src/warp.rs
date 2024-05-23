use metacall::{loaders, metacall, switch};
use metacall_ssr_benchmark::fib;
use std::net::SocketAddr;
use tokio::{
    self,
    signal::unix::{signal, SignalKind},
};
use warp::{reply::html, Filter};

#[tokio::main]
async fn main() {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx").unwrap();

    let hello_route = warp::path!("hello" / String)
        .map(|name| html(metacall::<String>("Hello", [name]).unwrap()));

    let fib_noute = warp::path!("fib" / u64)
        .map(|num| html(metacall::<String>("Fibonacci", [num as i64, fib(num) as i64]).unwrap()));

    let routes = warp::any().and(fib_noute.or(hello_route));
    let addr = "0.0.0.0:8080".to_string();

    println!("running on http://{}", addr);
    warp::serve(routes)
        // Actix has listeners for these three signals. We need to synchronize the implementations.
        .bind_with_graceful_shutdown(addr.parse::<SocketAddr>().unwrap(), async move {
            let mut sigquit = signal(SignalKind::quit()).unwrap();
            let mut sigint = signal(SignalKind::interrupt()).unwrap();
            let mut sigterm = signal(SignalKind::terminate()).unwrap();
            tokio::select! {
                _ = sigint.recv() => {}
                _ = sigquit.recv() => {}
                _ = sigterm.recv() => {}
            }
        })
        .1
        .await;
}
