use axum::{response::Html, routing::get, Router};
use metacall::{loaders, metacall, switch};
use tokio::runtime::Runtime;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx").unwrap();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();

    println!(
        "listening on {}",
        listener.local_addr().unwrap().to_string()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    Html(
        match metacall::<String>("Hello", ["Metacall!".to_string()]) {
            Err(e) => {
                println!("{:?}", e);
                panic!();
            }
            Ok(ret) => match ret {
                message => message,
                _ => "<h1>Not a Valid HTML</h1>".to_string(),
            },
        },
    )
}
