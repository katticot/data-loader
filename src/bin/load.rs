extern crate database;
use database::Database;
use diesel::PgConnection;
extern crate dotenv;
use crate::database::{elastic, mongo, postgres, Connectable, Load, Push};
use dotenv::dotenv;
use std::{env, error::Error};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let start = Instant::now();
    dotenv().ok();
    // env::set_var("no_proxy", "localhost");
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let postgres_database = postgres::new(&postgres_url);

    println!("size is {}", postgres_database.get_size());
    let postgres_duration = start.elapsed();
    let postgres_load_duration = Instant::now();

    println!(
        "postgres connection is {}",
        postgres_database.check_connection()
    );
        // let data =resut.await.unwrap();
        // let data = postgres_database.load(limit);
    let postgres_load_duration = postgres_load_duration.elapsed();
    println!("lancement postgres {:?}", postgres_duration);
    println!("chargement postgres {:?}", postgres_load_duration);

    //    env::set_var("no_proxy", "10.31.70.236");
    //  let mongo_database = mongo::new("mongodb://10.31.70.236:27017");
    //elastic.bulky(&postgres_database).await;
    //mongo_database.push(&postgres_database).await;
    load_elastic(&postgres_database).await;
}
// async fn load_mongo() -> Result<(),Error> {

    // to
// }

async fn load_elastic(postgres_database: &Database<PgConnection>) -> Result<(),Box<dyn Error>> {
    let elastic_load_duration = Instant::now();
    env::set_var("no_proxy", "10.31.70.224");
    let elasticsearch_url = env::var("ELASTICSEARCH_URL").expect("ELASTICSEARCH_URL must be set");
    // println!("{} elasticsearch bulk requests", data.len());
    let elastic = elastic::new(&elasticsearch_url);

    elastic.bulky(postgres_database).await?;
    let elastic_load_duration = elastic_load_duration.elapsed();
    println!("chargement elastic {:?}", elastic_load_duration);
    Ok(())
}