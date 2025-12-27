use crate::header::location::{country::Country, subregion::Subregion};

pub mod country;
pub mod subregion;

/// Represents the country and subregion of the player. https://docs.google.com/spreadsheets/d/1mSAomO_msfNllNsPeXbgU6UbJaGV5t6NvbZi6ebPFx4/edit?usp=sharing
pub struct Location {
    country: Country,
    subregion: Subregion,
    location_versioning: LocationVersioning,
}

impl Location {
    pub fn new(country_id: u8, subregion_id: u8, known_version: Option<LocationVersioning>) {
        let mut known_version = match known_version {
            None => LocationVersioning::get_min_from_country_id_number(country_id),
            Some(v) => v,
        };

        todo!()
    }

    pub fn change_version(&mut self, version: LocationVersioning) -> Self {
        // TODO: Handle changing version
        self.location_versioning = version;
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LocationVersioning {
    Vanilla,
    ExtendedRegionsV1_0,
    ExtendedRegionsV1_1,
    ExtendedRegionsV1_2,
    ExtendedRegionsV1_3, // This is WIP rn
}

impl LocationVersioning {
    pub fn get_min_from_country_id_number(value: u8) -> Self {
        match value {
            1 | 8..=52 | 64..=121 | 128 | 136 | 144..=145 | 152..=156 | 160 | 168..=177 | 255 => {
                Self::Vanilla
            }
            2..=7
            | 53..=63
            | 122..=127
            | 129..=135
            | 137..=143
            | 146..=151
            | 157..=159
            | 161..=167
            | 178..=254 => Self::ExtendedRegionsV1_0,
            0 => todo!(),
        }
    }

    pub fn next(self) -> Option<Self> {
        match self {
            Self::Vanilla => Some(Self::ExtendedRegionsV1_0),
            Self::ExtendedRegionsV1_0 => Some(Self::ExtendedRegionsV1_1),
            Self::ExtendedRegionsV1_1 => Some(Self::ExtendedRegionsV1_2),
            Self::ExtendedRegionsV1_2 => Some(Self::ExtendedRegionsV1_3),
            Self::ExtendedRegionsV1_3 => None,
        }
    }
}
