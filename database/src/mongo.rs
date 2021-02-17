use crate::{models::unwrapper, utils::get_coordinates, utils::get_naf, Database, Load, Push};
use async_trait::async_trait;
use diesel::IntoSql;
use elasticsearch::http::request::JsonBody;
use mongodb::{
    bson::{doc, extjson::de::Error},
    options::*,
    Client,
};
use std::env;
use serde_json::Value;
type Mongo = Client;

pub fn new(url: &str, port: u16) -> Database<Mongo> {
    let options = ClientOptions::builder()
        .hosts(vec![StreamAddress {
            hostname: url.into(),
            port: Some(port),
        }])
        .build();

    let client = Client::with_options(options).unwrap();
    // let client = Client::with_uri_str(url).await.unwrap();
    Database { connection: client }
}
#[async_trait(?Send)]
impl Push for Database<Mongo> {
    async fn push_data(&self, prospects: &Vec<Vec<JsonBody<Value>>>) -> Result<(), Error> {
        let db = &self.connection.database("some_db");
        for coll_name in db.list_collection_names(None).await.unwrap() {
            println!("collection: {}", coll_name);
        }

        Ok(())
    }

    async fn push(&self, data: &dyn Load) -> Result<(), mongodb::bson::extjson::de::Error> {
        let db = &self.connection.database("hellomaps");
        for coll_name in db.list_collection_names(None).await.unwrap() {
            println!("collection: {}", coll_name);
        }
        let coll = db.collection("prospects2");
        println!("boucle appel mongo");
        for prospect in data.full_load(data.get_size()) {
        let prospect_full = prospect.full();
        let coordinates: Vec<f64> = match get_coordinates(&prospect).await {
                Ok(coordinates) => coordinates,
                Err(e) => {
                    println!("datagouv request error : {} ", e);
                    vec![0.0, 0.0]
                }
            };
            let result = coll
                .insert_one(
                    doc! {
                        "rae": prospect.pds_id_rae,
                    "tdg_nom": prospect_full.tdg_nom,
                    "tdg_adresse": prospect_full.tdg_adresse,
                    "tdg_code_postal": prospect_full.tdg_code_postal,
                    "date_concu": prospect_full.date_concu,
                    "tdg_id_origine": prospect_full.tdg_id_origine,
                    "tdg_commune": prospect_full.tdg_commune,
                    "dcr_com": prospect_full.dcr_com,
                    "energie": prospect_full.energie,
                    "fct_civilite": prospect_full.fct_civilite,
                    "fct_nom": prospect_full.fct_nom,
                    "fct_prenom": prospect_full.fct_prenom,
                    "fct_tel_fixe": prospect_full.fct_tel_fixe,
                    "fct_tel_mobile": prospect_full.fct_tel_mobile,
                    "fct_email": prospect_full.fct_email,
                    "act_date_creation": prospect_full.act_date_creation,
                    "act_support": prospect_full.act_support,
                    "pgr_offre": prospect_full.pgr_offre,
                    "tdg_libelle_naf" : prospect_full.tdg_libelle_naf,
                        "location": {
                            "type": "Point",
                            "coordinates": [prospect_full.lat, prospect_full.lng],
                        },
                    "gps":{ "lng": coordinates[0], "lat":coordinates[1]},

                    },
                    None,
                )
                .await
                .unwrap();
            println!("{:#?}", result);
        }
        Ok(())
    }
}
