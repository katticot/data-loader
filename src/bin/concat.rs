
extern crate database;
use crate::database::{ utils};
fn main() {
    let address =contat_address("68 RUE PAUL BELLAMY", "NANTES (44109)");
    println!("{}",address)
}
    fn contat_address(address : &str, city: &str) -> String {
        let mut full_address: String = "".to_owned();
        full_address.push_str(address);
        full_address.push_str(city);
        full_address.retain(|c| c != '(');
        full_address.retain(|c| c != ')');
        full_address
    }