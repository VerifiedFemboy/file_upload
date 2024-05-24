use std::fs as fs;
use std::io::Write;
use std::path::Path;
use actix_files::NamedFile;

use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, web};
use futures_util::{StreamExt, TryStreamExt};

const DIRECTORY: &str = "./upload";

#[post("/upload")]
pub async fn upload_post(mut payload: Multipart) -> HttpResponse {
    while let Ok(Some(mut field)) = payload.try_next().await {
        //get data from payload
        let content = field.content_disposition();

        let file_name = content.get_filename().unwrap_or("unknown").to_string();
        let file_path = format!("{DIRECTORY}/{}", file_name);

        //create file
        let mut f = web::block(|| fs::File::create(file_path)).await.unwrap().unwrap();

        //write into the file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap().unwrap();
        }
    }

    HttpResponse::Ok().body("File uploaded successfully")
}

#[get("/uploads")]
pub async fn list_files() -> HttpResponse {
    match fs::read_dir(DIRECTORY) {
        Ok(entries) => {
            let mut file_links = String::new();
            for entry in entries.filter_map(|entry| entry.ok()) {
                if let Some(filename) = entry.path().file_name().and_then(|name| name.to_str()) {
                    file_links.push_str(&format!("<li><a href=\"/uploads/{}\">{}</a></li>", filename, filename));
                }
            }

            let html = format!(
                "<html><head><title>Uploaded Files</title></head><body><h1>Uploaded Files</h1><ul>{}</ul></body></html>",
                file_links
            );

            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        },
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error reading directory: {}", err))
        }
    }
}

#[get("/uploads/{filename:.*}")]
pub async fn serve_file(path: web::Path<String>) -> actix_web::Result<NamedFile> {
    let file_path = Path::new(DIRECTORY).join(&*path);
    Ok(NamedFile::open(file_path)?)
}
