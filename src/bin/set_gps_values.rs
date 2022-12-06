extern crate diesel;
extern crate cartilage;

use self::cartilage::*;
use self::models::*;
use self::diesel::prelude::*;

use clap::Parser;
use geocoding::{Forward, Point};

#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    count: i64,
}

fn main() {
    use cartilage::schema::parcels::dsl::*;

    let args = Args::parse();
    let parcel_count = args.count;

    //TODO: rename to establish_open_cage_connection and establish_diesel_connection
    let gps_connection = get_open_cage_connection();
    let db_connection = &mut establish_connection();
    let results = parcels
        .filter(lattitude.is_null())
        .limit(parcel_count)
        .load::<Parcel>(db_connection)
        .expect("Error loading parcels");

    println!("Displaying Parcels");
    for r in results {
        let _address: String = format!("{} {}, St. Louis Mo", r.address, r.street_name);
        let res: Vec<Point<f64>> = gps_connection.forward(&_address).unwrap();
        let first_result = &res[0];

        let updated_row = diesel::update(&r)
            .set(
                &GPSForm {
                    longitude: Some(first_result.x()),
                    lattitude: Some(first_result.y()),
                })
            .execute(db_connection);

        match updated_row {
            Ok(_) => {
                println!("id: {}", r.id);
                println!("address: {} {}", r.address, r.street_name);
                println!("long: {}", first_result.x());
                println!("lat: {}", first_result.y());
                println!("==============================")
            },
            Err(e) => println!("Error updating parcel: {e:?}"),

        }
    }
}
