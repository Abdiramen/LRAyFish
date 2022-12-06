extern crate diesel;
extern crate cartilage;

use self::cartilage::*;
use self::models::Parcel;

use clap::Parser;

#[derive(Parser,Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    file_path: String,
}

fn main() {
    let args = Args::parse();
    let entries: Vec<Parcel> = get_csv(args.file_path).unwrap();

    let mut connection = establish_connection();
    batch_save_parcel(&mut connection, entries);

}
