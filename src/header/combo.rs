pub struct Combo {
    character: Character,
    vehicle: Vehicle,
}

#[derive(thiserror::Error, Debug)]
pub enum ComboError {
    #[error("The combo has incongruent weight classes")]
    IncongruentWeightClasses,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
    #[error("Invalid Character ID")]
    InvalidCharacterId,
    #[error("Invalid Vehicle ID")]
    InvalidVehicleId,
}

impl Combo {
    pub fn new(vehicle: Vehicle, character: Character) -> Result<Self, ComboError> {
        match character.get_weight_class() == vehicle.get_weight_class() {
            true => Ok(Self { character, vehicle }),
            false => Err(ComboError::IncongruentWeightClasses),
        }
    }

    pub fn character(&self) -> Character {
        self.character
    }

    pub fn vehicle(&self) -> Vehicle {
        self.vehicle
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for Combo {
    type Error = ComboError;
    fn try_from(value: &mut bitreader::BitReader) -> Result<Self, Self::Error> {
        Self::new(
            Vehicle::try_from(value.read_u8(6)?).map_err(|_| ComboError::InvalidVehicleId)?,
            Character::try_from(value.read_u8(6)?).map_err(|_| ComboError::InvalidCharacterId)?,
        )
    }
}

impl GetWeightClass for Combo {
    fn get_weight_class(&self) -> WeightClass {
        self.character.get_weight_class()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeightClass {
    Small,
    Medium,
    Large,
}

pub trait GetWeightClass {
    fn get_weight_class(&self) -> WeightClass;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Character {
    Mario,
    BabyPeach,
    Waluigi,
    Bowser,
    BabyDaisy,
    DryBones,
    BabyMario,
    Luigi,
    Toad,
    DonkeyKong,
    Yoshi,
    Wario,
    BabyLuigi,
    Toadette,
    KoopaTroopa,
    Daisy,
    Peach,
    Birdo,
    DiddyKong,
    KingBoo,
    BowserJr,
    DryBowser,
    FunkyKong,
    Rosalina,
    SmallMiiOutfitAMale,
    SmallMiiOutfitAFemale,
    SmallMiiOutfitBMale,
    SmallMiiOutfitBFemale,
    SmallMiiOutfitCMale,
    SmallMiiOutfitCFemale,
    MediumMiiOutfitAMale,
    MediumMiiOutfitAFemale,
    MediumMiiOutfitBMale,
    MediumMiiOutfitBFemale,
    MediumMiiOutfitCMale,
    MediumMiiOutfitCFemale,
    LargeMiiOutfitAMale,
    LargeMiiOutfitAFemale,
    LargeMiiOutfitBMale,
    LargeMiiOutfitBFemale,
    LargeMiiOutfitCMale,
    LargeMiiOutfitCFemale,
    MediumMii,
    SmallMii,
    LargeMii,
    MenuPeach,
    MenuDaisy,
    MenuRosalina,
}

impl TryFrom<u8> for Character {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::BabyPeach),
            0x02 => Ok(Self::Waluigi),
            0x03 => Ok(Self::Bowser),
            0x04 => Ok(Self::BabyDaisy),
            0x05 => Ok(Self::DryBones),
            0x06 => Ok(Self::BabyMario),
            0x07 => Ok(Self::Luigi),
            0x08 => Ok(Self::Toad),
            0x09 => Ok(Self::DonkeyKong),
            0x0A => Ok(Self::Yoshi),
            0x0B => Ok(Self::Wario),
            0x0C => Ok(Self::BabyLuigi),
            0x0D => Ok(Self::Toadette),
            0x0E => Ok(Self::KoopaTroopa),
            0x0F => Ok(Self::Daisy),
            0x10 => Ok(Self::Peach),
            0x11 => Ok(Self::Birdo),
            0x12 => Ok(Self::DiddyKong),
            0x13 => Ok(Self::KingBoo),
            0x14 => Ok(Self::BowserJr),
            0x15 => Ok(Self::DryBowser),
            0x16 => Ok(Self::FunkyKong),
            0x17 => Ok(Self::Rosalina),
            0x18 => Ok(Self::SmallMiiOutfitAMale),
            0x19 => Ok(Self::SmallMiiOutfitAFemale),
            0x1A => Ok(Self::SmallMiiOutfitBMale),
            0x1B => Ok(Self::SmallMiiOutfitBFemale),
            0x1C => Ok(Self::SmallMiiOutfitCMale),
            0x1D => Ok(Self::SmallMiiOutfitCFemale),
            0x1E => Ok(Self::MediumMiiOutfitAMale),
            0x1F => Ok(Self::MediumMiiOutfitAFemale),
            0x20 => Ok(Self::MediumMiiOutfitBMale),
            0x21 => Ok(Self::MediumMiiOutfitBFemale),
            0x22 => Ok(Self::MediumMiiOutfitCMale),
            0x23 => Ok(Self::MediumMiiOutfitCFemale),
            0x24 => Ok(Self::LargeMiiOutfitAMale),
            0x25 => Ok(Self::LargeMiiOutfitAFemale),
            0x26 => Ok(Self::LargeMiiOutfitBMale),
            0x27 => Ok(Self::LargeMiiOutfitBFemale),
            0x28 => Ok(Self::LargeMiiOutfitCMale),
            0x29 => Ok(Self::LargeMiiOutfitCFemale),
            0x2A => Ok(Self::MediumMii),
            0x2B => Ok(Self::SmallMii),
            0x2C => Ok(Self::LargeMii),
            0x2D => Ok(Self::MenuPeach),
            0x2E => Ok(Self::MenuDaisy),
            0x2F => Ok(Self::MenuRosalina),
            _ => Err(()),
        }
    }
}

impl From<Character> for u8 {
    fn from(value: Character) -> Self {
        match value {
            Character::Mario => 0x00,
            Character::BabyPeach => 0x01,
            Character::Waluigi => 0x02,
            Character::Bowser => 0x03,
            Character::BabyDaisy => 0x04,
            Character::DryBones => 0x05,
            Character::BabyMario => 0x06,
            Character::Luigi => 0x07,
            Character::Toad => 0x08,
            Character::DonkeyKong => 0x09,
            Character::Yoshi => 0x0A,
            Character::Wario => 0x0B,
            Character::BabyLuigi => 0x0C,
            Character::Toadette => 0x0D,
            Character::KoopaTroopa => 0x0E,
            Character::Daisy => 0x0F,
            Character::Peach => 0x10,
            Character::Birdo => 0x11,
            Character::DiddyKong => 0x12,
            Character::KingBoo => 0x13,
            Character::BowserJr => 0x14,
            Character::DryBowser => 0x15,
            Character::FunkyKong => 0x16,
            Character::Rosalina => 0x17,
            Character::SmallMiiOutfitAMale => 0x18,
            Character::SmallMiiOutfitAFemale => 0x19,
            Character::SmallMiiOutfitBMale => 0x1A,
            Character::SmallMiiOutfitBFemale => 0x1B,
            Character::SmallMiiOutfitCMale => 0x1C,
            Character::SmallMiiOutfitCFemale => 0x1D,
            Character::MediumMiiOutfitAMale => 0x1E,
            Character::MediumMiiOutfitAFemale => 0x1F,
            Character::MediumMiiOutfitBMale => 0x20,
            Character::MediumMiiOutfitBFemale => 0x21,
            Character::MediumMiiOutfitCMale => 0x22,
            Character::MediumMiiOutfitCFemale => 0x23,
            Character::LargeMiiOutfitAMale => 0x24,
            Character::LargeMiiOutfitAFemale => 0x25,
            Character::LargeMiiOutfitBMale => 0x26,
            Character::LargeMiiOutfitBFemale => 0x27,
            Character::LargeMiiOutfitCMale => 0x28,
            Character::LargeMiiOutfitCFemale => 0x29,
            Character::MediumMii => 0x2A,
            Character::SmallMii => 0x2B,
            Character::LargeMii => 0x2C,
            Character::MenuPeach => 0x2D,
            Character::MenuDaisy => 0x2E,
            Character::MenuRosalina => 0x2F,
        }
    }
}

impl GetWeightClass for Character {
    fn get_weight_class(&self) -> WeightClass {
        match self {
            Self::BabyDaisy
            | Self::BabyLuigi
            | Self::BabyMario
            | Self::BabyPeach
            | Self::DryBones
            | Self::KoopaTroopa
            | Self::SmallMii
            | Self::SmallMiiOutfitAMale
            | Self::SmallMiiOutfitAFemale
            | Self::SmallMiiOutfitBMale
            | Self::SmallMiiOutfitBFemale
            | Self::SmallMiiOutfitCMale
            | Self::SmallMiiOutfitCFemale
            | Self::Toad
            | Self::Toadette => WeightClass::Small,
            Self::Birdo
            | Self::BowserJr
            | Self::Daisy
            | Self::MenuDaisy
            | Self::DiddyKong
            | Self::Luigi
            | Self::Mario
            | Self::MediumMii
            | Self::MediumMiiOutfitAMale
            | Self::MediumMiiOutfitAFemale
            | Self::MediumMiiOutfitBMale
            | Self::MediumMiiOutfitBFemale
            | Self::MediumMiiOutfitCMale
            | Self::MediumMiiOutfitCFemale
            | Self::Peach
            | Self::MenuPeach
            | Self::Yoshi => WeightClass::Medium,
            Self::Bowser
            | Self::DonkeyKong
            | Self::DryBowser
            | Self::FunkyKong
            | Self::KingBoo
            | Self::LargeMii
            | Self::LargeMiiOutfitAMale
            | Self::LargeMiiOutfitAFemale
            | Self::LargeMiiOutfitBMale
            | Self::LargeMiiOutfitBFemale
            | Self::LargeMiiOutfitCMale
            | Self::LargeMiiOutfitCFemale
            | Self::MenuRosalina
            | Self::Rosalina
            | Self::Waluigi
            | Self::Wario => WeightClass::Large,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Vehicle {
    StandardKartS,
    StandardKartM,
    StandardKartL,
    BoosterSeat,
    ClassicDragster,
    Offroader,
    MiniBeast,
    WildWing,
    FlameFlyer,
    CheepCharger,
    SuperBlooper,
    PiranhaProwler,
    TinyTitan,
    Daytripper,
    Jetsetter,
    BlueFalcon,
    Sprinter,
    Honeycoupe,
    StandardBikeS,
    StandardBikeM,
    StandardBikeL,
    BulletBike,
    MachBike,
    FlameRunner,
    BitBike,
    Sugarscoot,
    WarioBike,
    Quacker,
    ZipZip,
    ShootingStar,
    Magikruiser,
    Sneakster,
    Spear,
    JetBubble,
    DolphinDasher,
    Phantom,
}

impl TryFrom<u8> for Vehicle {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::StandardKartS),
            0x01 => Ok(Self::StandardKartM),
            0x02 => Ok(Self::StandardKartL),
            0x03 => Ok(Self::BoosterSeat),
            0x04 => Ok(Self::ClassicDragster),
            0x05 => Ok(Self::Offroader),
            0x06 => Ok(Self::MiniBeast),
            0x07 => Ok(Self::WildWing),
            0x08 => Ok(Self::FlameFlyer),
            0x09 => Ok(Self::CheepCharger),
            0x0A => Ok(Self::SuperBlooper),
            0x0B => Ok(Self::PiranhaProwler),
            0x0C => Ok(Self::TinyTitan),
            0x0D => Ok(Self::Daytripper),
            0x0E => Ok(Self::Jetsetter),
            0x0F => Ok(Self::BlueFalcon),
            0x10 => Ok(Self::Sprinter),
            0x11 => Ok(Self::Honeycoupe),
            0x12 => Ok(Self::StandardBikeS),
            0x13 => Ok(Self::StandardBikeM),
            0x14 => Ok(Self::StandardBikeL),
            0x15 => Ok(Self::BulletBike),
            0x16 => Ok(Self::MachBike),
            0x17 => Ok(Self::FlameRunner),
            0x18 => Ok(Self::BitBike),
            0x19 => Ok(Self::Sugarscoot),
            0x1A => Ok(Self::WarioBike),
            0x1B => Ok(Self::Quacker),
            0x1C => Ok(Self::ZipZip),
            0x1D => Ok(Self::ShootingStar),
            0x1E => Ok(Self::Magikruiser),
            0x1F => Ok(Self::Sneakster),
            0x20 => Ok(Self::Spear),
            0x21 => Ok(Self::JetBubble),
            0x22 => Ok(Self::DolphinDasher),
            0x23 => Ok(Self::Phantom),
            _ => Err(()),
        }
    }
}

impl From<Vehicle> for u8 {
    fn from(value: Vehicle) -> Self {
        match value {
            Vehicle::StandardKartS => 0x00,
            Vehicle::StandardKartM => 0x01,
            Vehicle::StandardKartL => 0x02,
            Vehicle::BoosterSeat => 0x03,
            Vehicle::ClassicDragster => 0x04,
            Vehicle::Offroader => 0x05,
            Vehicle::MiniBeast => 0x06,
            Vehicle::WildWing => 0x07,
            Vehicle::FlameFlyer => 0x08,
            Vehicle::CheepCharger => 0x09,
            Vehicle::SuperBlooper => 0x0A,
            Vehicle::PiranhaProwler => 0x0B,
            Vehicle::TinyTitan => 0x0C,
            Vehicle::Daytripper => 0x0D,
            Vehicle::Jetsetter => 0x0E,
            Vehicle::BlueFalcon => 0x0F,
            Vehicle::Sprinter => 0x10,
            Vehicle::Honeycoupe => 0x11,
            Vehicle::StandardBikeS => 0x12,
            Vehicle::StandardBikeM => 0x13,
            Vehicle::StandardBikeL => 0x14,
            Vehicle::BulletBike => 0x15,
            Vehicle::MachBike => 0x16,
            Vehicle::FlameRunner => 0x17,
            Vehicle::BitBike => 0x18,
            Vehicle::Sugarscoot => 0x19,
            Vehicle::WarioBike => 0x1A,
            Vehicle::Quacker => 0x1B,
            Vehicle::ZipZip => 0x1C,
            Vehicle::ShootingStar => 0x1D,
            Vehicle::Magikruiser => 0x1E,
            Vehicle::Sneakster => 0x1F,
            Vehicle::Spear => 0x20,
            Vehicle::JetBubble => 0x21,
            Vehicle::DolphinDasher => 0x22,
            Vehicle::Phantom => 0x23,
        }
    }
}

impl GetWeightClass for Vehicle {
    fn get_weight_class(&self) -> WeightClass {
        match self {
            Self::StandardKartS
            | Self::BoosterSeat
            | Self::MiniBeast
            | Self::CheepCharger
            | Self::TinyTitan
            | Self::BlueFalcon
            | Self::StandardBikeS
            | Self::BulletBike
            | Self::BitBike
            | Self::Quacker
            | Self::Magikruiser
            | Self::JetBubble => WeightClass::Small,
            Self::StandardKartM
            | Self::ClassicDragster
            | Self::WildWing
            | Self::SuperBlooper
            | Self::Daytripper
            | Self::Sprinter
            | Self::StandardBikeM
            | Self::MachBike
            | Self::Sugarscoot
            | Self::ZipZip
            | Self::Sneakster
            | Self::DolphinDasher => WeightClass::Medium,
            Self::Offroader
            | Self::StandardKartL
            | Self::FlameFlyer
            | Self::PiranhaProwler
            | Self::Jetsetter
            | Self::Honeycoupe
            | Self::StandardBikeL
            | Self::FlameRunner
            | Self::WarioBike
            | Self::ShootingStar
            | Self::Spear
            | Self::Phantom => WeightClass::Large,
        }
    }
}
