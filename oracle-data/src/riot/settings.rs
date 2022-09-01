use std::env;

pub fn get_riot_key() -> String {
    env::var("RIOT_API_KEY").expect("Failed to retrieve 'RIOT_API_KEY'.")
}
