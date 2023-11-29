use actix_web::{HttpServer, App};
mod router;
use router::config;
use std::error::Error;
use std::env;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>>{

    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    HttpServer::new(|| {
            App::new()
            .configure(config)
        }
    )
    .bind(("0.0.0.0", port.parse().expect("Erro na porta")))?
    .run()
    .await?;

    Ok(())
}
