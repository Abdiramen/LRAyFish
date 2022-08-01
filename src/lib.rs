#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

use self::models::{Parcel, Usage};
pub fn batch_save_parcel(conn: PgConnection, entries: Vec<Parcel>) -> usize {
    use schema::parcels::dsl::*;

    diesel::insert_into(parcels)
        .values(&entries)
        .execute(&conn)
        .expect("Error batch saving posts")
}

extern crate csv;

use std::error::Error;
use csv::Reader;
use std::collections::HashSet;
use std::str::FromStr;

pub fn get_csv(lra_file: String) -> Result<Vec<Parcel>, Box<dyn Error>> {
    let mut reader = Reader::from_path(lra_file)?;
    let mut records: Vec<Parcel> = Vec::new();
    let mut dups: HashSet<i64> = HashSet::new();
    for result in reader.records() {
        let record = result?;
        let id = record[0].parse::<i64>().unwrap();
        if !dups.contains(&id) {
            dups.insert(id);
            let new_parcel = Parcel{
                    id: record[0].parse::<i64>().unwrap(),
                    address: record[1].to_string(),
                    street_name: record[2].to_string(),
                    front_length: record[3].parse::<i64>().unwrap(),
                    side_length: record[4].parse::<i64>().unwrap(),
                    square_feet: record[5].parse::<i64>().unwrap(),
                    usage: Usage::from_str(&record[6]).unwrap(),
                    lattitude: None,
                    longitude: None,
            };
            records.push(
                new_parcel
            );
        }
    }
    for e in dups {
        println!("{:?}", e);
    }
    Ok(records)
}
