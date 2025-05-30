mod config;
mod features;
mod models;
mod schema;

use crate::features::route_config;
use actix_cors::Cors;
use actix_web::http::header::CONTENT_TYPE;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::env;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::config::openapi::ApiDoc;

// Type alias for the connection pool
type DbPool = Pool<ConnectionManager<PgConnection>>;

const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();

    info!("Starting server...");

    // Set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    info!("Connected to database successfully");
    let server_address = format!("{}:{}", SERVER_HOST, SERVER_PORT);

    info!("Starting HTTP server at {}", server_address);

    HttpServer::new(move || {
        let cors = Cors::default() // TODO: Set Cors correctly
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone())) // Share the pool with the application
            .wrap(cors)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/openapi.json", ApiDoc::openapi())
            )
            .configure(route_config)
    })
    .bind(format!("{}:{}", SERVER_HOST, SERVER_PORT))?
    .run()
    .await
}
