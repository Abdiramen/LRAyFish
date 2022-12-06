extern crate diesel;
extern crate cartilage;

use self::cartilage::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use cartilage::schema::parcels::dsl::*;

    let connection = &mut establish_connection();
    let results = parcels.limit(5)
        .load::<Parcel>(connection)
        .expect("Error loading parcels");
    println!("Displaying Parcels");
    for r in results {
        println!("id: {}", r.id);
        println!("address: {} {}", r.address, r.street_name);
    }
}
