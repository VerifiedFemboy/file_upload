extern crate actix_files as fs;

use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, post};

const DIRECTORY: &str = "./upload";

#[post("/upload")]
pub async fn upload_post(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    todo!("mr maciek please code this fucking shit brooooooooooooooooo 💀");
    HttpResponse::Ok().into()
}
