use async_trait::async_trait;
use riven::{
    RiotApi,
    models::match_v5::Match,
    models::spectator_v4::CurrentGameInfo,
    consts::Region
};

use super::types::{Error, MatchData, MatchInfo, State};

mod settings;
mod store;

pub struct LeagueOfLegends {
    api: riven::RiotApi,
    store: store::PlayerStore,
}

impl LeagueOfLegends {

    pub fn new(credentials: String) -> Self {

        Self {
            api: RiotApi::with_key(credentials),
            store: store::PlayerStore::new()
        }
    }

    async fn get_request_player_id(&self, name: &String) -> Result<String, Error> {
        let summoner = self.api
            .summoner_v4()
            .get_by_summoner_name(Region::BR, name.as_str())
            .await
            .map_err(|x| Error::ResponseForbidden(x.to_string()))?;

        match summoner {
            Some(s) => {
                let bucket = self.store.get_bucket()?;
                bucket.set(name, &s.puuid).map_err(|_| Error::CacheSetFailed)?;
                Ok(s.puuid)
            },
            None => Err(Error::NotFound)
        }
    }

    fn get_kv_player_id(&self, name: &String) -> Result<Option<String>, Error> {
        let player = self.store.get_bucket()?
            .get(name)
            .map_err(|_| Error::CacheUnavailable)?;

        println!("player cache: {:?}", player);

        Ok(player)
    }

    async fn get_player_id(&self, name: &String) -> Result<String, Error> {
        let cached_puuid = self.get_kv_player_id(name)?;

        if let Some(puuid) = cached_puuid {
            Ok(puuid)
        } else {
            Ok(self.get_request_player_id(name).await?)
        }
    }

    async fn get_spectator_by_player_id(&self, puuid: String) -> Result<CurrentGameInfo, Error> {
        self.api
            .spectator_v4()
            .get_current_game_info_by_summoner(Region::BR, &puuid)
            .await
            .map_err(|x| Error::ResponseForbidden(x.to_string()))?
            .ok_or_else(|| Error::NotFound)
    }

    async fn get_match_data_by_id(&self, match_id: &String) -> Result<Match, Error> {
        self.api
            .match_v5()
            .get_match(Region::BR, match_id.as_str())
            .await
            .map_err(|x| Error::ResponseForbidden(x.to_string()))?
            .ok_or_else(|| Error::NotFound)
    }
}

#[async_trait]
impl MatchData<String> for LeagueOfLegends {

    async fn get_match_by_player_id(&self, player_name: String) -> Result<MatchInfo<String>, Error>  {
        let puuid = self.get_player_id(&player_name).await?;
        let m = self.get_spectator_by_player_id(puuid).await?;

        Ok(MatchInfo {
            id: m.game_id.to_string(),
            created_at: m.game_start_time,
            players: vec![],
            state: State::Running
        })
    }

    async fn get_match_by_id(&self, id: &String) -> Result<MatchInfo<String>, Error> {
        let m = self.get_match_data_by_id(id).await?;

        let state = if let Some(_) = m.info.game_end_timestamp {
            State::Finished
        } else {
            State::Running
        };

        Ok(MatchInfo {
            id: id.clone(),
            created_at: m.info.game_start_timestamp,
            players: vec![],
            state
        })
    }

}

#[cfg(test)]
mod test {
    use super::settings;
    use super::LeagueOfLegends;

    const EXPECTED_RESULT: &str = "vu6eKwzeVA7a03NDKEvR81MfNoZI6UXW-L5FP3Sk-iRcnz4YCi-aw7tJVCodlCtYsyPEK-A4xfO3iA";

    #[tokio::test]
    async fn test_get_summoner_player_id() {
        dotenv::dotenv().ok();

        let key = settings::get_riot_key();
        let riot = LeagueOfLegends::new(key);
        let player_id = riot.get_request_player_id(&String::from("aroncds")).await;

        assert!(player_id.is_ok());
        assert_eq!(EXPECTED_RESULT, player_id.unwrap());
    }
}