use mongodb::{Client, Collection, options::ClientOptions};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub _id: String,
    pub name: String,
    pub upload_location: String,
    pub token: String
}

impl Account {
    pub async fn new(_id: String, name: String, token: String) -> Self {
        let upload = _id.clone();
        Self {_id, name, upload_location: upload, token}
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

    pub async fn token_match(&self, token: String) -> mongodb::error::Result<Option<Account>> {
        let result = self.account_collection.find_one(doc!{"token": token}, None).await?;
        Ok(result)
    }
}
