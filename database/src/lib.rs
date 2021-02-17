#[macro_use]
extern crate diesel;
pub mod models;
pub mod schema;
use async_trait::async_trait;

pub mod elastic;
pub mod mongo;
pub mod postgres;
pub mod utils;

use elasticsearch::http::request::JsonBody;
use models::Prospect;
use serde_json::{Result as ResultJ, Value};
pub struct Database<Connection> {
    pub connection: Connection,
}
pub struct Data<Connection> {
    pub data: ResultJ<()>,
    pub database: Database<Connection>,
}
pub trait Connectable {
    fn check_connection(&self) -> bool;
    fn new(connection_string: &str) -> Self;
}

pub trait Load {
    fn load(&self, limit: i64) -> Vec<Vec<JsonBody<Value>>>;
    fn full_load(&self, limit: i64) -> Vec<Prospect>;
    fn get_size(&self) -> i64;
}
#[async_trait(?Send)]
pub trait Push {
    async fn push_data(
        &self,
        prospects: &Vec<Vec<JsonBody<Value>>>,
    ) -> Result<(), mongodb::bson::extjson::de::Error>;
    async fn push(&self, data: &dyn Load) -> Result<(), mongodb::bson::extjson::de::Error>;
}
