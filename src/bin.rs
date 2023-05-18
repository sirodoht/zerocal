#[cfg(feature = "local")]
use std::net::SocketAddr;

#[cfg(feature = "local")]
use std::env;

#[cfg(feature = "local")]
use anyhow::Result;

#[cfg(feature = "local")]
#[tokio::main]
async fn main() -> Result<()> {
    let router = zerocal::get_router();
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("8000"))
        .parse()
        .expect("PORT must be a number");
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Listening on http://localhost:{}/", port);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

#[cfg(not(feature = "local"))]
fn main() {
    // dummy main function if shuttle is used
}
