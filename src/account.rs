use actix_web::{HttpResponse, post, web};
use actix_web::web::Json;
use serde::{Deserialize, Serialize};

use crate::database::{Account, Database};
use crate::token::Token;

#[derive(Serialize, Deserialize)]
struct AccountInfo {
    name: String,
}

#[post("/account/create")]
pub async fn create(account_info: Json<AccountInfo>, db: web::Data<Database>) -> HttpResponse {
    let name = &account_info.name;
    let token = Token::new(7).await.generate().await;

    let account = Account::new(name.to_string(), token.to_string()).await;

    match db.insert_account(account).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
