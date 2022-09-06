use std::str::FromStr;

use diesel::pg::Pg;
use diesel::serialize::ToSql;
use diesel::deserialize::FromSql;
use diesel::expression::AsExpression;
use diesel::sql_types::VarChar;

use serde::{Deserialize, Serialize};
use web3::types::H256;

#[derive(Debug, AsExpression, Deserialize, Serialize)]
#[diesel(sql_type = VarChar)]
pub struct F256(H256);

impl FromSql<VarChar, Pg> for F256
{
    fn from_sql(bytes: diesel::backend::RawValue<'_, Pg>) -> diesel::deserialize::Result<Self> {
        let value = <String as FromSql<VarChar, Pg>>::from_sql(bytes)?;
        Ok(F256(H256::from_str(&value).unwrap()))
    }
}

impl ToSql<VarChar, Pg> for F256
{
    fn to_sql<'b>(&'b self, out: &mut diesel::serialize::Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let v = self.0.to_string();

        <String as ToSql<VarChar, Pg>>::to_sql(&v, &mut out.reborrow())
    }
}