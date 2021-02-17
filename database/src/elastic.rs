use std::borrow::BorrowMut;

use crate::{models, Connectable, Database, Load};
use async_trait::async_trait;
use elasticsearch::{
    http::request::JsonBody, http::transport::Transport, BulkParts, Elasticsearch, IndexParts,
};
use indicatif::{ProgressBar, ProgressStyle};
use serde_json::{json, to_value, Value};

pub(crate) const ELASTIC_INDEX: &str = "hellomaps_dev";

pub fn new(url: &str) -> Database<Elasticsearch> {
    let transport = Transport::single_node(url).unwrap();
    let client = Elasticsearch::new(transport);
    //    print!("ping sur elasticsearch{:?}", client.ping());
    Database { connection: client }
}
impl Connectable for Database<Elasticsearch> {
    fn check_connection(&self) -> bool {
        true
    }

    fn new(url: &str) -> Self {
        let transport = Transport::single_node(url).unwrap();
        let client = Elasticsearch::new(transport);
        Database { connection: client }
    }
}

impl Database<Elasticsearch> {
    pub fn getdata(&self, data: &dyn Load) -> i64 {
        data.get_size()
    }

    pub async fn bulky(&self, database: &dyn Load) -> Result<(), Box<dyn std::error::Error>> {
        let prospects_size = database.get_size();
        let prospects = database.full_load(prospects_size);
        let result = create_prospect_bulk(prospects, &self.connection, 25000).await?;
        print!("{}", result);
        Ok(())
    }
    pub async fn bulk(&self, data: Vec<JsonBody<Value>>) -> Result<(), Box<dyn std::error::Error>> {
        let json = data;
        let response = self
            .connection
            .bulk(BulkParts::Index(ELASTIC_INDEX))
            .body(json)
            .send()
            .await?;
        let response_body = response.json::<Value>().await?;
        let successful = response_body["errors"].as_bool().unwrap() == false;

        // get the HTTP response status code
        // let status_code = &response.status_code();

        // read fields from the response body
        // print!("status {} ", status_code);
        // print!("response {} ", response_body);
        println!("response success {} ", successful);
        Ok(())
    }
}
async fn create_prospect(
    prospect: models::Prospect,
    client: &Elasticsearch,
) -> Result<(), Box<dyn std::error::Error>> {
    let prospect = prospect.light();
    let response = client
        .index(IndexParts::IndexId(ELASTIC_INDEX, "1"))
        .body(prospect)
        .send()
        .await?;
    let successful = response.status_code().is_success();
    Ok(())
}

async fn create_prospect_bulk(
    prospects: Vec<models::Prospect>,
    client: &Elasticsearch,
    bulk_request_max_size: usize,
) -> Result<bool, Box<dyn std::error::Error>> {
    let bar = ProgressBar::new(prospects.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    let prospects_iterator = prospects.iter();

    let mut limit = 0;
    let mut bulk_request_number: usize = prospects.len()/bulk_request_max_size;
    let mut bodies: Vec<Vec<JsonBody<Value>>> = Vec::with_capacity(bulk_request_number);
    let mut successful = false;
    for n in 0..bulk_request_number+1  {
        bodies.push(Vec::with_capacity(bulk_request_max_size));
        println!("Création de la bulk request : {}",n)
    }
    for (i, prospect) in prospects_iterator.enumerate() {
        bodies[bulk_request_number].push(json!({"index": {"_id": i+1}}).into());
        let prospect_value =
            serde_json::to_value(&prospect.light()).expect("Error on Json to str conversion");
        bodies[bulk_request_number].push(prospect_value.into());
        bar.inc(1);
        limit = limit + 1;
        if limit >= bulk_request_max_size {
            limit = 0;
            let response = client
                .bulk(BulkParts::Index(ELASTIC_INDEX))
                .body(bodies.pop().unwrap())
                .send()
                .await?;
            successful = response.status_code().is_success();
            bulk_request_number=bulk_request_number-1;
            // body.clear();
        }
    }
    bar.finish();
    // let successful = response.status_code().is_success();
    // println!("prospect elastic bulky push {}", successful);
    Ok(successful)
}
