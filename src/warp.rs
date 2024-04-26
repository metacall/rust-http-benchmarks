use metacall::{loaders, metacall, switch};
use std::net::SocketAddr;
use warp::{reply::html, Filter};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _metacall = switch::initialize().unwrap();
    loaders::from_single_file("ts", "App.tsx");

    let routes = warp::any().map(|| {
        html(
            match metacall::<String>("Hello", ["MetaCall!".to_string()]) {
                Err(e) => {
                    format!("Cannot load component: {e:?}")
                }
                Ok(ret) => match ret {
                    message => message,
                    _ => "<h1>Not a Valid HTML</h1>".to_string(),
                },
            },
        )
    });
    let addr = "0.0.0.0:8081".to_string();

    println!("running on http://{}", addr);
    warp::serve(routes)
        .run(addr.parse::<SocketAddr>().unwrap())
        .await;
}
