extern crate diesel;
extern crate cartilage;

use self::cartilage::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use cartilage::schema::parcels::dsl::*;

    let connection = establish_connection();
    let results = parcels.limit(5)
        .load::<Parcel>(&connection)
        .expect("Error loading parcels");
}
