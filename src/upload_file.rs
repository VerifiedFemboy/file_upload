use std::fs as fs;
use std::io::Write;
use std::path::Path;

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, HttpResponse, post, web};
use futures_util::{StreamExt, TryStreamExt};

const DIRECTORY: &str = "./upload";
const STATIC_DIR: &str = "./static";

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
                    if let Some(extension) = entry.path().extension().and_then(|ext| ext.to_str()) {
                        match extension {
                            "png" | "jpg" | "jpeg" | "gif" => {
                                file_links.push_str(&format!(
                                    "<a href=\"/uploads/{}\" target=\"_blank\"><img src=\"/uploads/{}\" alt=\"{}\" style=\"width:500px;height:auto;\" /></a>",
                                    filename, filename, filename
                                ));
                            },
                            "mp4" | "webm" | "ogg" => {
                                file_links.push_str(&format!(
                                    "<video width=\"500\" height=\"auto\" controls><source src=\"/uploads/{}\" type=\"video/{}\">Your browser does not support the video tag.</video>",
                                    filename, extension
                                ));
                            },
                            _ => {
                                file_links.push_str(&format!(
                                    "<a href=\"/uploads/{}\">{}</a>",
                                    filename, filename
                                ));
                            }
                        }
                    } else {
                        file_links.push_str(&format!(
                            "<a href=\"/uploads/{}\">{}</a>",
                            filename, filename
                        ));
                    }
                }
            }

            let file_path = format!("{STATIC_DIR}/uploads.html");
            let html_contents = fs::read_to_string(file_path);

            let html = html_contents.unwrap().replace("{}", &file_links);
            HttpResponse::Ok().content_type("text/html; charset=utf-8").body(html)
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Error reading dir: {}", err))
        }
    }
}

#[get("/uploads/{filename:.*}")]
pub async fn serve_file(path: web::Path<String>) -> actix_web::Result<NamedFile> {
    let file_path = Path::new(DIRECTORY).join(&*path);
    Ok(NamedFile::open(file_path)?)
}
