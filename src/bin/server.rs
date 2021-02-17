use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
extern crate database;
use crate::database::{elastic, mongo, postgres, Connectable, Load, Push};
use dotenv::dotenv;
use std::env;
use std::time::{Duration, Instant};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hey)
            .service(elastic_handler)
            .service(mongo_hanler)
            .route("/", web::get().to(|| HttpResponse::Ok().body("coucou")))
    })
    .bind("127.0.0.1:9293")?
    .run()
    .await
}

#[get("/hey")]
async fn hey() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/elastic")]
async fn elastic_handler() -> impl Responder {
    dotenv().ok();
    let start = Instant::now();
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let elasticsearch_url = env::var("ELASTICSEARCH_URL").expect("ELASTICSEARCH_URL must be set");
    let data_limit = env::var("LIMIT").expect("LIMIT must be set");

    let postgres_duration = start.elapsed();
    let limit: i64 = data_limit.parse().unwrap();
    let postgres_database = postgres::new(&postgres_url);
    println!("size is {}", postgres_database.get_size());
    let postgres_load_duration = Instant::now();
    println!(
        "postgres connection is {}",
        postgres_database.check_connection()
    );
    let data = postgres_database.load(limit);
    let postgres_load_duration = postgres_load_duration.elapsed();

    let elastic_load_duration = Instant::now();
    println!(
        "load data into ElastiSearch in {} bulk requests",
        data.len()
    );
    let elastic = elastic::new(&elasticsearch_url);
    match elastic.bulky(&postgres_database).await {
        Ok(a) => a,
        _ => unreachable!(),
    };
    let elastic_load_duration: Duration = elastic_load_duration.elapsed();

    let full_duration: Duration = start.elapsed();
    println!("lancement postgres {:?}", postgres_duration);
    println!("chargement postgres {:?}", postgres_load_duration);
    println!("chargement elastic {:?}", elastic_load_duration);
    println!("chargement global {:?}", full_duration);
    HttpResponse::Ok().body("Load lancé")
}

#[get("/mongo")]
async fn mongo_hanler() -> impl Responder {
    dotenv().ok();
    let start = Instant::now();
    let postgres_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let data_limit = env::var("LIMIT").expect("LIMIT must be set");

    let postgres_duration = start.elapsed();
    let limit: i64 = data_limit.parse().unwrap();
    let postgres_database = postgres::new(&postgres_url);
    println!("size is {}", postgres_database.get_size());
    let postgres_load_duration = Instant::now();
    println!(
        "postgres connection is {}",
        postgres_database.check_connection()
    );
    let data = postgres_database.load(limit);
    let postgres_load_duration = postgres_load_duration.elapsed();

    let mongo_load_duration = Instant::now();
    println!("load data into MongoDB");
    let mongo_database = mongo::new("10.31.70.236", 27017);
    match mongo_database.push(&postgres_database).await {
        Ok(a) => a,
        _ => unreachable!(),
    };
    let mongo_load_duration: Duration = mongo_load_duration.elapsed();

    let full_duration: Duration = start.elapsed();
    println!("lancement postgres {:?}", postgres_duration);
    println!("chargement postgres {:?}", postgres_load_duration);
    println!("chargement mongo {:?}", mongo_load_duration);
    println!("chargement global {:?}", full_duration);
    HttpResponse::Ok().body("Load mongo effectué")
}
