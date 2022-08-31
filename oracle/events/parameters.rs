
use std::collections::HashMap;

use diesel::pg::Pg;
use diesel::serialize::ToSql;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::sql_types::Text;
use serde::Deserialize;
use serde::Serialize;
use web3::ethabi::Token;
use web3::types::H256;

type EventParams = HashMap<String, Token>;

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchCreatedParams {
    pub game_id: H256,
    pub expire_time: u64,
    pub external_game_id: H256
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BetCreatedParams {
    pub game_id: H256
}

#[derive(Debug, AsExpression, Deserialize, Serialize)]
#[diesel(sql_type = Text)]
pub enum Args {
    MatchCreated(MatchCreatedParams),
    BetCreated(BetCreatedParams)
}

impl FromSql<Text, Pg> for Args
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        let value = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_str(&value)?)
    }
}

impl ToSql<Text, Pg> for Args
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let v = serde_json::to_string(&self)?;
        
        <String as ToSql<Text, Pg>>::to_sql(&v, &mut out.reborrow())
    }
}

impl From<&EventParams> for MatchCreatedParams {
    fn from(x: &EventParams) -> Self {
        let game_id: [u8; 32] = x.get("gameId")
            .unwrap()
            .clone()
            .into_fixed_bytes()
            .unwrap()
            .try_into()
            .unwrap();

        let expire_time = x.get("expireTime")
            .unwrap()
            .clone()
            .into_uint()
            .unwrap()
            .as_u64();

        let mut external_game_id: [u8; 32] = [0; 32];

        external_game_id.copy_from_slice(
            &x.get("externalGameId")
                .unwrap()
                .clone()
                .into_fixed_bytes()
                .unwrap()[0..32]
        );

        Self {
            game_id: game_id.into(),
            expire_time,
            external_game_id: external_game_id.into(),
        }
    }
}

impl From<&EventParams> for BetCreatedParams {
    fn from(x: &EventParams) -> Self {
        let game_id: [u8; 32] = x.get("gameId")
            .unwrap()
            .clone()
            .into_fixed_bytes()
            .unwrap()
            .try_into()
            .unwrap();
        
        Self {
            game_id: game_id.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_event_params_to_match_created_params() {
        let params = EventParams::new();
    }
}