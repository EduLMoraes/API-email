use actix_web::{HttpRequest, web, get};

#[path="./valid_email.rs"]
mod valid_email;

#[path ="./struct_json.rs"]
mod struct_json;
use struct_json::Email;

/// Rota para verificar email
#[get("/{email}")]
async fn is_email(email: HttpRequest) -> web::Json<Email>{
    let mut email: Email = Email { 
        address: email.match_info().get("email").unwrap().to_owned(),
        valid: false,
    };

    println!("Validando email...");
    email.valid = valid_email::validate(&email.address);
    println!("Email validado...");

    web::Json(email)
}

#[allow(dead_code)]
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("")
            .service(is_email)
    );
}