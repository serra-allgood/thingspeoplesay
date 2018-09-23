use diesel::{
    deserialize::{FromSql, Result},
    pg::Pg,
    sql_types::{Array, Bigint, Jsonb, Text, VarChar},
};
use lib::schema::*;
use serde_json;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct Message {
    pub id: i64,
    pub message: String,
    pub created_at: SystemTime,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub message: &'a str,
}

#[derive(Queryable)]
pub struct Color {
    pub id: i64,
    pub hexcode: String,
}

#[derive(Insertable)]
#[table_name = "colors"]
pub struct NewColor<'a> {
    pub hexcode: &'a str,
}

#[derive(Insertable)]
#[table_name = "gradients"]
pub struct NewGradient {
    pub message_id: i64,
    pub color_id: i64,
    pub position: i32,
}

#[derive(Deserialize, Serialize, QueryableByName, FromSqlRow)]
pub struct Speech {
    #[sql_type = "Text"]
    pub message: String,
    #[sql_type = "Array<VarChar>"]
    pub hexcodes: Vec<String>,
}

impl FromSql<Jsonb, Pg> for Speech {
    fn from_sql(bytes: Option<&[u8]>) -> Result<Self> {
        let bytes = not_none!(bytes);
        if bytes[0] != 1 {
            return Err("Unsupported JSONB encoding version".into());
        }
        let speech: Speech = serde_json::from_slice(&bytes[1..])?;

        Ok(speech)
    }
}

#[derive(Deserialize, Serialize, QueryableByName)]
pub struct SpeechData {
    #[sql_type = "Bigint"]
    pub total: i64,
    #[sql_type = "Array<Jsonb>"]
    pub speeches: Vec<Speech>,
}
