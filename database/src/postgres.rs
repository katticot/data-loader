use crate::models::Prospect;
use crate::schema::hellomaps_export::dsl::*;
use crate::{Connectable, Database, Load};
use async_trait::async_trait;
use diesel::expression::sql_literal::sql;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::Connection;
use elasticsearch::http::request::JsonBody;
use serde_json::{json, Value};

impl Connectable for Database<PgConnection> {
    fn check_connection(&self) -> bool {
        let results = hellomaps_export
            .limit(1)
            .load::<Prospect>(&self.connection)
            .expect("Error loading hellomaps_exports");
        results.len() == 1
    }

    fn new(url: &str) -> Self {
        Database {
            connection: PgConnection::establish(url).expect("Error Connecting to database"),
        }
    }
}

#[async_trait(?Send)]
impl Load for Database<PgConnection> {
    // todo make generic http://siciarz.net/24-days-rust-diesel/di
    fn get_size(&self) -> i64 {
        let results: i64 = sql("select count(*) from hellomaps.\"hellomaps_export\"")
            .get_result(&self.connection)
            .expect("msg");
        results
    }

    fn load(&self, limit: i64) -> Vec<Vec<JsonBody<Value>>> {
        let results = hellomaps_export
            .limit(limit)
            .load::<Prospect>(&self.connection)
            .expect("Error loading hellomaps_exports");
        let iter = results.iter();
        let mut body: Vec<Vec<JsonBody<Value>>> = Vec::with_capacity(results.len());
        let mut bodypart: Vec<JsonBody<Value>> = Vec::with_capacity(25000);
        let mut limit = 0;
        for (i, prospect) in iter.enumerate() {
            let prospect_str =
                serde_json::to_string(&prospect).expect("Error on Json to str conversion");
            if limit >= 25000 {
                limit = 0;
                body.push(bodypart);
                bodypart = Vec::with_capacity(25000);
            }
            bodypart.push(json!({"index": {"_id": i+1}}).into());
            let prospect_json: Value =
                serde_json::from_str(&prospect_str).expect("Error on Json to Value conversion");
            bodypart.push(prospect_json.into());

            limit = limit + 1;
        }
        body.push(bodypart);
        body
    }

    fn full_load(&self, limit: i64) -> Vec<Prospect> {
        let results = hellomaps_export
            .limit(limit)
            .load::<Prospect>(&self.connection)
            .expect("Error loading hellomaps_exports");
        results
    }
}
pub fn new(url: &str) -> Database<PgConnection> {
    Database {
        connection: PgConnection::establish(url).expect("Error Connecting to database"),
    }
}
