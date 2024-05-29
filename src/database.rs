use mongodb::{Client, Collection, options::ClientOptions};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub discord_id: String,
    pub upload_location: String,
    pub token: String
}

impl Account {
    pub async fn new(name: String, token: String) -> Self {
        let upload= name.clone();
        Self {name, discord_id: "".to_string(), upload_location: upload, token}
    }
}

#[derive(Clone)]
#[warn(dead_code)]
pub struct Database {
    client: Client,
    account_collection: Collection<Account>,
}

impl Database {
    pub async fn new(connection_string: &str, db_name: &str, collection_name: &str) -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(connection_string).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(db_name);
        let collection = database.collection::<Account>(collection_name);

        Ok(Self { client, account_collection: collection })
    }

    pub async fn insert_account(&self, account: Account) -> mongodb::error::Result<()> {
        self.account_collection.insert_one(account, None).await?;
        Ok(())
    }

}
