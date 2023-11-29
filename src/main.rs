use actix_web::{HttpServer, App};
mod router;
use router::config;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>>{
    HttpServer::new(|| {
            App::new()
            .configure(config)
        }
    )
    .bind(("0.0.0.0", 8000))?
    .run()
    .await?;

    Ok(())
}
