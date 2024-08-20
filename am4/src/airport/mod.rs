pub mod db;

use derive_more::{Constructor, Display, Into};
use std::str::FromStr;
use thiserror::Error;

#[cfg(feature = "rkyv")]
use rkyv::{Archive as Ra, Deserialize as Rd, Serialize as Rs};
#[cfg(feature = "serde")]
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "rkyv", derive(Ra, Rd, Rs), archive(check_bytes))]
pub struct Airport {
    pub idx: Id,
    pub name: Name,
    pub fullname: String,
    pub country: String,
    pub continent: String,
    pub iata: Iata,
    pub icao: Icao,
    pub location: Point,
    pub rwy: u16,
    pub market: u8,
    pub hub_cost: u32,
    pub rwy_codes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, Hash, Constructor, Into)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "rkyv", derive(Ra, Rd, Rs), archive(check_bytes))]
pub struct Id(u16);

impl FromStr for Id {
    type Err = AirportError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        id.parse::<u16>().map(Self).map_err(AirportError::InvalidID)
    }
}

#[derive(Debug, Clone, Display, PartialEq, Eq, Hash, Into)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "rkyv", derive(Ra, Rd, Rs), archive(check_bytes))]
pub struct Name(String);

impl FromStr for Name {
    type Err = AirportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            1..=40 => Ok(Self(s.to_string())), // actual 36
            _ => Err(AirportError::InvalidName),
        }
    }
}

#[derive(Debug, Clone, Display, PartialEq, Eq, Hash, Into)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "rkyv", derive(Ra, Rd, Rs), archive(check_bytes))]
pub struct Iata(String);

impl FromStr for Iata {
    type Err = AirportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            3 => Ok(Self(s.to_string())),
            _ => Err(AirportError::InvalidIata),
        }
    }
}

#[derive(Debug, Clone, Display, PartialEq, Eq, Hash, Into)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "rkyv", derive(Ra, Rd, Rs), archive(check_bytes))]
pub struct Icao(String);

impl FromStr for Icao {
    type Err = AirportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            4 => Ok(Self(s.to_string())),
            _ => Err(AirportError::InvalidIcao),
        }
    }
}

#[derive(Debug, Clone, Display, PartialEq)]
#[display("({lng}, {lat})")]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "rkyv", derive(Ra, Rd, Rs), archive(check_bytes))]
pub struct Point {
    pub lng: f32,
    pub lat: f32,
}

#[derive(Debug, Error)]
pub enum AirportError {
    #[error("Invalid airport ID: {0}")]
    InvalidID(#[source] std::num::ParseIntError),
    #[error("Invalid name: must be between 1 and 50 characters")]
    InvalidName,
    #[error("Invalid IATA code: must be 3 characters")]
    InvalidIata,
    #[error("Invalid ICAO code: must be 4 characters")]
    InvalidIcao,
}
