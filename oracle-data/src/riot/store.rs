
use kv::{Config, Store, Bucket};

use super::Error;

const STORE_PATH: &str = "./players";
const BUCKET_NAME: &str = "players";

pub struct PlayerStore {
    store: Store,
}

impl PlayerStore {
    pub fn new() -> Self {
        let cfg = Config::new(STORE_PATH); 
        let store = Store::new(cfg).expect("failed to open storage");

        Self { store }
    }

    pub fn get_bucket<'a>(&self) -> Result<Bucket<'a, String, String>, Error> {
        self.store.bucket::<String, String>(Some(BUCKET_NAME)).map_err(|_| Error::CacheUnavailable)
    }
}