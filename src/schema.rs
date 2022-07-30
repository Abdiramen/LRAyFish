table! {
    use diesel::sql_types::*;
    use crate::models::exports::*;

    parcels (id) {
        id -> Int8,
        address -> Varchar,
        streetname -> Varchar,
        front_length -> Int8,
        side_length -> Int8,
        square_feet -> Int8,
        usage -> Usage,
    }
}
