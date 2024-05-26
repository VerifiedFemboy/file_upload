use std::path::Path;

use actix_files::{Files, NamedFile};
use actix_web::{App, get, HttpServer, web};

use crate::upload_file::{list_files, serve_file};

mod upload_file;
mod manage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Creates an upload path which files will be saved
    if !Path::new("./upload").exists() {
        tokio::fs::create_dir("./upload").await?
    }

    //Runs the server
    HttpServer::new(|| App::new()
        .service(upload_file::upload_post)
        .service(list_files)
        .service(serve_file)
        .service(get_static_file)
        .service(manage::manage_file)
        .service(Files::new("/static", "./static")
            .show_files_listing()))
        .bind("127.0.0.1:8080")?.run().await
}

#[get("/{filename}")]
async fn get_static_file(filename: web::Path<String>) -> actix_web::Result<NamedFile> {
    let filename = format!("./static/{filename}.html");
    let path: &Path = Path::new(&filename);
    Ok(NamedFile::open(path)?)
}

