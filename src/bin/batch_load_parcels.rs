extern crate diesel;
extern crate cartilage;

use self::cartilage::*;
use self::models::Parcel;
use self::diesel::prelude::*;

use clap::Parser;

#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file_path: String,
}

fn main() {
    use cartilage::schema::parcels::dsl::*;

    let args = Args::parse();
    let entries: Vec<Parcel> = get_csv(args.file_path).unwrap();

    let connection = establish_connection();
    batch_save_parcel(connection, entries);

}
