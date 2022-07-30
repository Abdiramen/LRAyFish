#[derive(SqlType)]
#[postgres(type_name = "Usage")]
pub struct UsageType;

#[derive(Debug, FromSqlRow, AsExpression)]
#[sql_type = "UsageType"]
pub enum Usage {
    Commercial,
    Condominium,
    Garage,
    Industrial,
    Mixed,
    Residential,
    VacantLot
}

use std::io::Write;

use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};

impl<Db: Backend> ToSql<UsageType, Db> for Usage {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            Usage::Commercial => out.write_all(b"Commercial")?,
            Usage::Condominium => out.write_all(b"Condominium")?,
            Usage::Garage => out.write_all(b"Garage")?,
            Usage::Industrial => out.write_all(b"Industrial")?,
            Usage::Mixed => out.write_all(b"Mixed")?,
            Usage::Residential => out.write_all(b"Residential")?,
            Usage::VacantLot => out.write_all(b"VacantLot")?,
        }
        Ok(IsNull::No)
    }
}

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

impl FromSql<UsageType, Pg> for Usage {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Commercial" => Ok(Usage::Commercial),
            b"Condominium" => Ok(Usage::Condominium),
            b"Garage" => Ok(Usage::Garage),
            b"Industrial" => Ok(Usage::Industrial),
            b"Mixed" => Ok(Usage::Mixed),
            b"Residential" => Ok(Usage::Residential),
            b"VacantLot" => Ok(Usage::VacantLot),
            _ => Err("Unrecognize enum variant".into()),
        }
    }
}

#[derive(Queryable)]
pub struct Parcel {
    pub id: i64,
    pub address: String,
    pub streetname: String,
    pub front_length: i64,
    pub side_length: i64,
    pub square_feet: i64,
    pub usage: Usage
}

pub mod exports {
    pub use super::UsageType as Usage;
}
