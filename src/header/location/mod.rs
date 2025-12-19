use crate::header::location::{country::Country, subregion::Subregion};

pub mod country;
pub mod subregion;

// TODO: implement this properly (there are ~3000 total subregions so this is a huge task and probably not worth it)
/// Represents the country and subregion of the player. https://docs.google.com/spreadsheets/d/1mSAomO_msfNllNsPeXbgU6UbJaGV5t6NvbZi6ebPFx4/edit?usp=sharing
pub struct Location {
    country: Country,
    subregion: Subregion,
}
