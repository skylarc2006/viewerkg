// http://wiibrew.org/wiki/Mii_Data#Mii_format

use crate::{
    byte_handler::{ByteHandler, ByteHandlerError, FromByteHandler},
    header::mii::{
        bday::{Birthday, BirthdayError}, build::{Build, BuildError}, eyebrows::{Eyebrows, EyebrowsError}, eyes::{Eyes, EyesError}, facial_hair::{FacialHair, FacialHairError}, fav_color::{FavColor, FavColorError}, glasses::{Glasses, GlassesError}, hair::{Hair, HairError}, head::{Head, HeadError}, lips::{Lips, LipsError}, mole::{Mole, MoleError}, nose::{Nose, NoseError}
    },
};

pub mod bday;
pub mod build;
pub mod eyebrows;
pub mod eyes;
pub mod facial_hair;
pub mod fav_color;
pub mod glasses;
pub mod hair;
pub mod head;
pub mod lips;
pub mod mole;
pub mod nose;

#[derive(thiserror::Error, Debug)]
pub enum MiiError {
    #[error("FromUtf16Error: {0}")]
    FromUtf16Error(#[from] std::string::FromUtf16Error),
    #[error("Invalid data length")]
    InvalidLength,
    #[error("Birthday Error: {0}")]
    BirthdayError(#[from] BirthdayError),
    #[error("FavColor Error: {0}")]
    FavColorError(#[from] FavColorError),
    #[error("Build Error: {0}")]
    BuildError(#[from] BuildError),
    #[error("Head Error: {0}")]
    HeadError(#[from] HeadError),
    #[error("Hair Error: {0}")]
    HairError(#[from] HairError),
    #[error("Eyebrows Error: {0}")]
    EyebrowsError(#[from] EyebrowsError),
    #[error("Eyes Error: {0}")]
    EyesError(#[from] EyesError),
    #[error("Nose Error: {0}")]
    NoseError(#[from] NoseError),
    #[error("Lips Error: {0}")]
    LipsError(#[from] LipsError),
    #[error("Glasses Error: {0}")]
    GlassesError(#[from] GlassesError),
    #[error("FacialHair Error: {0}")]
    FacialHairError(#[from] FacialHairError),
    #[error("Mole Error: {0}")]
    MoleError(#[from] MoleError),
    #[error("ByteHandler Error: {0}")]
    ByteHandlerError(#[from] ByteHandlerError),
}

pub struct Mii {
    is_girl: bool,
    birthday: Birthday,
    favorite_color: FavColor,
    is_favorite: bool,
    name: String,
    build: Build,
    mii_id: u32,
    system_id: u32,
    head: Head,
    mingle_off: bool,
    downloaded: bool,
    hair: Hair,
    eyebrows: Eyebrows,
    eyes: Eyes,
    nose: Nose,
    lips: Lips,
    glasses: Glasses,
    facial_hair: FacialHair,
    mole: Mole,
    creator_name: String,
}

impl Mii {
    pub fn new(mii_data: impl TryInto<[u8; 0x4A]>) -> Result<Self, MiiError> {
        let mii_data = mii_data.try_into().map_err(|_| MiiError::InvalidLength)?;

        let bytes = ByteHandler::try_from(&mii_data[0..=1])?;
        let is_girl = bytes.read_bool(6);
        let birthday = Birthday::from_byte_handler(bytes)?;

        let favorite_color = FavColor::from_byte_handler(mii_data[1])?;
        let is_favorite = mii_data[1].is_multiple_of(2);

        let name = utf16be_to_string(&mii_data[0x02..=0x15])?;

        let build = Build::from_byte_handler(&mii_data[0x16..=0x17])?;

        let mii_id = ByteHandler::try_from(&mii_data[0x18..=0x1B])?.copy_dword();
        let system_id = ByteHandler::try_from(&mii_data[0x1C..=0x1F])?.copy_dword();

        let bytes = ByteHandler::try_from(&mii_data[0x20..=0x21])?;
        let mingle_off = bytes.read_bool(2);
        let downloaded = bytes.read_bool(0);
        let head = Head::from_byte_handler(bytes)?;
        let hair = Hair::from_byte_handler(&mii_data[0x22..=0x23])?;
        let eyebrows = Eyebrows::from_byte_handler(&mii_data[0x24..=0x27])?;
        let eyes = Eyes::from_byte_handler(&mii_data[0x28..=0x2B])?;
        let nose = Nose::from_byte_handler(&mii_data[0x2C..=0x2D])?;
        let lips = Lips::from_byte_handler(&mii_data[0x2E..=0x2F])?;
        let glasses = Glasses::from_byte_handler(&mii_data[0x30..=0x31])?;
        let facial_hair = FacialHair::from_byte_handler(&mii_data[0x32..=0x33])?;
        let mole = Mole::from_byte_handler(&mii_data[0x34..=0x35])?;

        let creator_name = utf16be_to_string(&mii_data[0x36..=0x49])?;

        Ok(Self {
            is_girl,
            birthday,
            favorite_color,
            is_favorite,
            name,
            build,
            mii_id,
            system_id,
            head,
            mingle_off,
            downloaded,
            hair,
            eyebrows,
            eyes,
            nose,
            lips,
            glasses,
            facial_hair,
            mole,
            creator_name,
        })
    }

    pub fn is_girl(&self) -> bool {
        self.is_girl
    }

    pub fn birthday(&self) -> Birthday {
        self.birthday
    }

    pub fn favorite_color(&self) -> FavColor {
        self.favorite_color
    }

    pub fn is_favorite(&self) -> bool {
        self.is_favorite
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn build(&self) -> Build {
        self.build
    }

    pub fn mii_id(&self) -> u32 {
        self.mii_id
    }

    pub fn system_id(&self) -> u32 {
        self.system_id
    }

    pub fn head(&self) -> Head {
        self.head
    }

    pub fn mingle_off(&self) -> bool {
        self.mingle_off
    }

    pub fn downloaded(&self) -> bool {
        self.downloaded
    }

    pub fn hair(&self) -> Hair {
        self.hair
    }

    pub fn eyebrows(&self) -> &Eyebrows {
        &self.eyebrows
    }

    pub fn eyes(&self) -> &Eyes {
        &self.eyes
    }

    pub fn nose(&self) -> Nose {
        self.nose
    }

    pub fn lips(&self) -> Lips {
        self.lips
    }

    pub fn glasses(&self) -> Glasses {
        self.glasses
    }

    pub fn facial_hair(&self) -> &FacialHair {
        &self.facial_hair
    }

    pub fn mole(&self) -> Mole {
        self.mole
    }

    pub fn creator_name(&self) -> &str {
        &self.creator_name
    }
}

fn utf16be_to_string(bytes: &[u8]) -> Result<String, std::string::FromUtf16Error> {
    let utf16: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|c| u16::from_be_bytes([c[0], c[1]]))
        .take_while(|&u| u != 0)
        .collect();

    String::from_utf16(&utf16)
}
