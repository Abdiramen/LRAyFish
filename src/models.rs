#[derive(SqlType)]
#[diesel(postgres_type(name = "Usage"))]
pub struct UsageType;

#[derive(Debug, FromSqlRow, Clone, Copy, AsExpression)]
#[diesel(sql_type = UsageType)]
pub enum Usage {
    Commercial,
    Condominium,
    Garage,
    Industrial,
    Mixed,
    Residential,
    VacantLot
}

use std::str::FromStr;
impl FromStr for Usage {
    type Err = ();

    fn from_str(input: &str) -> Result<Usage, Self::Err> {
        match input {
            "Commercial" => Ok(Usage::Commercial),
            "Condominium" => Ok(Usage::Condominium),
            "Garage" => Ok(Usage::Garage),
            "Industrial" => Ok(Usage::Industrial),
            "Mixed" => Ok(Usage::Mixed),
            "Residential" => Ok(Usage::Residential),
            "VacantLot" => Ok(Usage::VacantLot),
            _ => Err(()),
        }
    }
}

use std::io::Write;

use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};

impl ToSql<UsageType, Pg> for Usage {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match self {
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

use diesel::backend;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::sql_types::Text;

impl<Db> FromSql<UsageType, Db> for Usage
where
    Db: Backend,
    String: FromSql<Text, Db>,
{
    fn from_sql(bytes: backend::RawValue<Db>) -> deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str() {
            "Commercial" => Ok(Usage::Commercial),
            "Condominium" => Ok(Usage::Condominium),
            "Garage" => Ok(Usage::Garage),
            "Industrial" => Ok(Usage::Industrial),
            "Mixed" => Ok(Usage::Mixed),
            "Residential" => Ok(Usage::Residential),
            "VacantLot" => Ok(Usage::VacantLot),
            _ => Err("Unrecognize enum variant".into()),
        }
    }
}

#[derive(Queryable,Identifiable,Insertable,AsChangeset,Debug)]
pub struct Parcel {
    pub id: i64,
    pub address: String,
    pub street_name: String,
    pub front_length: i64,
    pub side_length: i64,
    pub square_feet: i64,
    pub usage: Usage,
    pub lattitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(AsChangeset)]
#[diesel(table_name = parcels)]
pub struct GPSForm {
    pub longitude: Option<f64>,
    pub lattitude: Option<f64>,
}


use super::schema::parcels;

pub mod exports {
    pub use super::UsageType as Usage;
}
