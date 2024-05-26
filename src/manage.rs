use std::fs::remove_file;

use actix_web::{HttpResponse, post, web};
use actix_web::web::Json;
use serde::Deserialize;

const DIRECTORY: &str = "./upload";

#[derive(Deserialize)]
struct FileInfo {
    filename: String
}

#[post("/manage/{method}")]
pub async fn manage_file(method: web::Path<String>, file_info: Json<FileInfo>) -> HttpResponse {
    let filename = file_info.filename.to_string();
    let file_path = format!("{DIRECTORY}/{filename}");
    match method.as_str() {
        "delete" => {
            match remove_file(&file_path) {
                Ok(_) => HttpResponse::Ok().body("File deleted successfully"),
                Err(err) => HttpResponse::InternalServerError().body(format!("Something went wrong, may file doesn't exist\n{}", err))
            }
        },
        _ => {
            HttpResponse::InternalServerError().body("Wrong method")
        },
    }
}