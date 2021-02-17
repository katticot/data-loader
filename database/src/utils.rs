use crate::models::Prospect;
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{env};

use mongodb::{bson::doc, options::FindOneOptions};

const GEO_API_U_R_L: &str = "http://10.31.70.224:7878/search/";

pub async fn datagouv_geo_reqwest(
    adresse: &str,
    postcode: &str,
) -> Result<Root, serde_json::Error> {
    let geo_api_url = env::var("GEO_API_URL").expect("GEO_API_URL must be set");
    let parse_url =
        reqwest::Url::parse_with_params(GEO_API_U_R_L, &[("q", &adresse), ("postcode", &postcode)])
            .expect("cant parse url");
    println!("url {:?}", parse_url);
    let res = reqwest::get(parse_url).await.unwrap();
    println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await.unwrap();
    let json: Root = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(e) => return Err(e),
    };

    Ok(json)
}

pub async fn get_coordinates(prospect: &Prospect) -> Result<Vec<f64>, serde_json::Error> {
    println!("appel coordinates");
    let request_duration: Instant = Instant::now();
    let postcode: &str = match prospect.tdg_code_postal.as_ref() {
        Some(postcode) => postcode,
        None => "",
    };
    let adresse = match prospect.tdg_adresse.as_ref() {
        Some(adresse) => adresse,
        None => "",
    };
    println!("appel coordinates{}", postcode);
    let mut lat = 0.0;
    let mut lng = 0.0;
    match datagouv_geo_reqwest(adresse, postcode).await {
        Ok(data) => {
            if data.features.len() > 0 {
                lat = data.features[0].geometry.coordinates[0];
                lng = data.features[0].geometry.coordinates[1];
            }
        }
        Err(e) => return Err(e),
        // Err(e) => println!("datagouv request error : {} -------- ", e),
    };
    let request_duration = request_duration.elapsed();
    println!("durée de la requete {:?}", request_duration);

    Ok(vec![lat, lng])
}

pub async fn get_naf(code_naf: &str) -> String {
    let client_options = ClientOptions::parse("mongodb://10.31.70.236:27017")
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();

    let db = client.database("hellomaps");

    let collection = db.collection("NAF");
    let filter = doc! { "fields.code_naf": code_naf};
    let filter2 = doc! { "fields":"1","_id": "0"};
    let find_options = FindOneOptions::builder().projection(filter2).build();

    match collection.find_one(filter, find_options).await {
        Err(e) => {
            println!("Error {}", e);
            "Non défini".to_string()
        }
        Ok(document) => document
            .unwrap()
            .get_document("fields")
            .unwrap()
            .get_str("intitule_naf_65")
            .unwrap_or_default()
            .to_string(),
    }
}
    pub fn contat_address(address : &str, city: &str) -> String {
        let mut full_address: String = "".to_owned();
        full_address.push_str(address);
        full_address.push_str(city);
        full_address.retain(|c| c != '(');
        full_address.retain(|c| c != ')');
        full_address
    }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: String,
    pub features: Vec<Feature>,
    pub attribution: String,
    pub licence: String,
    pub query: String,
    pub filters: Option<Filters>,
    pub limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    #[serde(rename = "type")]
    pub type_field: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub label: String,
    pub score: f64,
    pub housenumber: Option<String>,
    pub id: String,
    pub name: String,
    pub postcode: String,
    pub citycode: String,
    pub x: f64,
    pub y: f64,
    pub city: String,
    pub context: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub importance: f64,
    pub street: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    pub postcode: String,
}
