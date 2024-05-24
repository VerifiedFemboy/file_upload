use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, post, web};
use futures_util::{StreamExt, TryStreamExt};
use std::fs as fs;
use std::io::Write;

const DIRECTORY: &str = "./upload";

#[post("/upload")]
pub async fn upload_post(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    while let Ok(Some(mut field)) = payload.try_next().await {
        //get data from payload
        let content = field.content_disposition();

        let file_name = content.get_filename().unwrap_or("unknown").to_string();
        let file_path = format!("{DIRECTORY}{}", file_name);

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
