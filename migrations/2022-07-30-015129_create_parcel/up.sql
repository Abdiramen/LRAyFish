CREATE TYPE usage AS ENUM ('Commercial', 'Condominium', 'Garage', 'Industrial', 'Mixed', 'Residential', 'Vacant Lot');

CREATE TABLE parcels (
  id bigint PRIMARY KEY,
  address VARCHAR NOT NULL,
  streetname VARCHAR NOT NULL,
  front_length bigint NOT NULL,
  side_length bigint NOT NULL,
  square_feet bigint NOT NULL,
  usage usage NOT NULL
)
