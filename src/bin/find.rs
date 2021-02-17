extern crate database;
use crate::database::utils::get_naf;

#[tokio::main]
async fn main() {
    println!("chargement postgres {:?}", get_naf("4711D").await);
}
