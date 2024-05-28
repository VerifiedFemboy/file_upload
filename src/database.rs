use mongodb::{Client, Collection, options::ClientOptions};

pub struct Account {
    pub _id: i32,
    pub name: String,
    pub discord_id: String,
    pub upload_location: String,
    pub token: String
}

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

    pub async fn get_all_accounts(&self) -> mongodb::error::Result<Vec<Account>> {
        let cursor = self.account_collection.find(None, None).await?;
        let mut accounts = Vec::new();

        for result in cursor {
            match result {
                Ok(document) => accounts.push(document),
                Err(e) => return Err(e.into()),
            }
        }
        Ok(accounts)
    }
}
