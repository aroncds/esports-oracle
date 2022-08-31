use riven::RiotApi;
use riven::consts::Region;

use crate::matches::{MatchData, MatchInfo, Error};

pub struct LeagueOfLegends {
    api: riven::RiotApi,
}

impl LeagueOfLegends {
    pub fn new(credentials: String) -> Self {
        Self {
            api: RiotApi::with_key(credentials)
        }
    }

    async fn get_player_id(&self, name: String) -> Result<String, Error> {
        let summoner = self.api
            .summoner_v4()
            .get_by_summoner_name(Region::BR, name.as_str())
            .await
            .map_err(|x| Error::ResponseForbidden)?;

        match summoner {
            Some(x) => Ok(x.puuid),
            None => Err(Error::NotFound)
        }
    }
}

#[cfg(test)]
mod test {
    use super::LeagueOfLegends;

    #[tokio::test]
    async fn test_get_summoner_player_id() {
        let key = String::from("?");
        let riot = LeagueOfLegends::new(key);
        let player_id = riot.get_player_id(String::from("aroncds")).await;
        println!("player id: {:?}", player_id);
    }
}