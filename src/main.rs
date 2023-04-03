pub mod tracker;

use sqlx::{migrate, sqlite::SqlitePoolOptions};

#[tokio::main]
async fn main() {
    env_logger::init();

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect("sqlite:test.db")
        .await
        .unwrap();

    migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();

    let duration_monitor = tracker::Monitor::new(pool);
    duration_monitor.run();
}
