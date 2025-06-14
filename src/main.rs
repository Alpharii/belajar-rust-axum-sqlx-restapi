mod db;
mod handler;
mod model;
mod route;

use dotenvy::dotenv;
use db::create_pg_pool;
use route::create_routes;
use axum::serve;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = create_pg_pool().await.expect("Gagal connect DB");
    let app = create_routes(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server running on http://localhost:3000");

    serve(listener, app).await.unwrap();
}
