use axum::{response::Html, routing::get, Router};
use metacall::{loaders, metacall, switch};
use tokio::{
    self,
    signal::unix::{signal, SignalKind},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx").unwrap();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!(
        "listening on {}",
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

async fn root() -> Html<String> {
    Html(metacall::<String>("Hello", ["Metacall!".to_string()]).unwrap())
}
