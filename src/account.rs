use std::fs;
use std::path::Path;
use actix_web::{HttpResponse, post, web};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::database::{Account, Database};
use crate::token::Token;

#[derive(Serialize, Deserialize)]
struct AccountInfo {
    name: String,
    discord_id: String
}

#[post("/account/create")]
pub async fn create_account(account_info: Json<AccountInfo>, db: web::Data<Database>) -> HttpResponse {
    let discord_id = &account_info.discord_id;
    let name = &account_info.name;
    let token = Token::new(7).await.generate().await;
    let account = Account::new(discord_id.to_string(), name.to_string(), token.to_string()).await;

    match db.insert_account(account.clone()).await {
        Ok(_) => {
            let dir_location = format!("./upload/{}", account._id);

            let dir = Path::new(&dir_location);

            if !dir.exists() {
                fs::create_dir(dir).expect("Something went wrong while creating file");
            }

            HttpResponse::Ok().body(format!("Your token: {token}. Please save it wherever and remember, do NOT share it anyone!"))
        },
        Err(_) => HttpResponse::InternalServerError().body("Something went wrong! Maybe account actually exists or server is dead!"),
    }
}
