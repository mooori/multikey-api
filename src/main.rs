use warp::{http::Response, Filter};

use dummyapi::FooQuery;

#[tokio::main]
async fn main() {
    // Setup tracing.
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).init();

    let hello = warp::any()
        .and(warp::query::<FooQuery>())
        .map(|q: FooQuery| Response::builder().body(format!("{:?}", q)))
        .with(warp::trace::named("hello"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
