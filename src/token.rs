use rand::distributions::Alphanumeric;
use rand::Rng;

pub struct Token {
    length: usize,
}

impl Token {
    pub async fn new(length: usize) -> Self {
        Self { length }
    }

    pub async fn generate(&self) -> String {
        rand::thread_rng().sample_iter(&Alphanumeric).take(self.length).map(char::from).collect()
    }
}