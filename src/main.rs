mod upload_file;

use std::path::Path;
use actix_files::NamedFile;
use actix_web::{App, HttpServer, get, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    //Creates an upload path which files will be saved
    if !Path::new("./upload").exists() {
        tokio::fs::create_dir("./upload").await?
    }

    //Runs the server
    HttpServer::new(|| App::new()
        .service(upload_file::upload_post)
        .service(get_static_file))
        .bind("127.0.0.1:8080")?.run().await
}

#[get("/{filename}")]
async fn get_static_file(filename: web::Path<String>) -> actix_web::Result<NamedFile> {
    let filename = format!("./static/{filename}.html");
    let path: &Path = Path::new(&filename);
    Ok(NamedFile::open(path)?)
}


