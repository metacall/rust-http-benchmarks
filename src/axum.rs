use axum::{extract::Path, response::Html, routing::get, Router};
use metacall::{loaders, metacall, switch};
use metacall_ssr_benchmark::fib;
use tokio::{
    self,
    signal::unix::{signal, SignalKind},
};

async fn fib_route(Path(num): Path<u64>) -> Html<String> {
    Html(metacall::<String>("Fibonacci", [num as i64, fib(num) as i64]).unwrap())
}

async fn root(Path(name): Path<String>) -> Html<String> {
    Html(metacall::<String>("Hello", [name]).unwrap())
}

#[tokio::main]
async fn main() {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx").unwrap();
    let app = Router::new()
        .route("/hello/:name", get(root))
        .route("/fib/:num", get(fib_route));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!(
        "listening on http://{}",
        listener.local_addr().unwrap().to_string()
    );

    axum::serve(listener, app)
        // Actix has listeners for these three signals. We need to synchronize the implementations.
        .with_graceful_shutdown(async move {
            let mut sigquit = signal(SignalKind::quit()).unwrap();
            let mut sigint = signal(SignalKind::interrupt()).unwrap();
            let mut sigterm = signal(SignalKind::terminate()).unwrap();
            tokio::select! {
                _ = sigint.recv() => {}
                _ = sigquit.recv() => {}
                _ = sigterm.recv() => {}
            }
        })
        .await
        .unwrap();
}
