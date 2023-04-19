use actix_web::web::Form;
use actix_web::HttpResponse;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
