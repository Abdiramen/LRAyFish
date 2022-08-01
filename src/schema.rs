table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    parcels (id) {
        id -> Int8,
        address -> Varchar,
        street_name -> Varchar,
        front_length -> Int8,
        side_length -> Int8,
        square_feet -> Int8,
        usage -> Usage,
        lattitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
    }
}
