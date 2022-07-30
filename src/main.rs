use geocoding::{Coordinate, Forward, Opencage, Point};
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate dotenv_codegen;
fn main() {
    dotenv().ok();

    let oc = Opencage::new(dotenv!("OPEN_CAGE_API_KEY").to_string());
    let address = "2910 Abner Place, Saint Louis, MO 63120, United States of America";
    let result: Vec<Point<f64>> = oc.forward(address).unwrap();
    match result.first() {
        Some(point) => println!("{x}, {y}", x=point.x(), y=point.y()),
        None        => println!("this ain't right"),
        _           => println!("this ain't right"),
    }
}
