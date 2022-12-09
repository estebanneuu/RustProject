#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_files::Files;
use url::Url;

// We define a custom type for connection pool to use later.
/* macro_rules! embed_migrations {
    () => { ... };
    ($migrations_path : expr) => { ... };
}
embed_migrations!("../migrations/"); */
mod handlers;
mod models;
mod schema;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Loading .env into environment variable.
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // set up database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let conn = pool.get().expect("ERROR: main: DB connection failed");
    //embed_migrations::run(&*conn).expect("ERROR: main: Failed to run database migrations");

    let port = std::env::var("PORT").expect("$PORT is not set.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| async { "PaSympApi" }))
            .service(handlers::get_all_commands)
            .service(handlers::post_command)
            .service(handlers::get_command_id)
            .service(handlers::get_register)
            .service(handlers::post_result)
            .service(handlers::get_all_results)
/*             .service(handlers::update)
            .service(handlers::destroy) */
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
