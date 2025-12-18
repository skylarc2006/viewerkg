// http://wiibrew.org/wiki/Mii_Data#Mii_format

use crate::byte_handler::ByteHandler;

#[derive(thiserror::Error, Debug)]
pub enum MiiError {
    #[error("FromUtf16Error: {0}")]
    FromUtf16Error(#[from] std::string::FromUtf16Error),
    #[error("Invalid data length")]
    InvalidLength,
}

pub struct Mii {
    is_girl: bool,
    month: Option<u8>,
    day: Option<u8>,
    favorite_color: u8,
    is_favorite: bool,
    name: String,
    height: u8,
    weight: u8,
    mii_id: u32,
    system_id: u32,
    face_shape: u8,
    skin_color: u8,
    facial_feature: u8,
    mingle_off: bool,
    downloaded: bool,
    hair_type: u8,
    hair_color: u8,
    hair_part_reversed: bool,
    eyebrow_type: u8,
    eyebrow_rotation: u8,
    eyebrow_color: u8,
    eyebrow_size: u8,
    eyebrow_vertical_pos: u8,
    eyebrow_horizontal_spacing: u8,
    eye_type: u8,
    eye_rotation: u8,
    eye_vertical_pos: u8,
    eye_color: u8,
    eye_size: u8,
    eye_horizontal_spacing: u8,
    nose_type: u8,
    nose_size: u8,
    nose_vertical_pos: u8,
    lip_type: u8,
    lip_color: u8,
    lip_size: u8,
    lip_vertical_pos: u8,
    glasses_type: u8,
    glasses_color: u8,
    glasses_size: u8,
    glasses_vertical_pos: u8,
    mustache_type: u8,
    beard_type: u8,
    facial_hair_color: u8,
    mustache_size: u8,
    mustache_vertical_pos: u8,
    has_mole: bool,
    mole_size: u8,
    mole_vertical_pos: u8,
    mole_horizontal_pos: u8,
    creator_name: String,
}

impl Mii {
    pub fn new(mii_data: impl TryInto<[u8; 0x4A]>) -> Result<Self, MiiError> {
        let mii_data = mii_data.try_into().map_err(|_| MiiError::InvalidLength)?;

        let is_girl = ByteHandler::from(mii_data[0]).read_bool(6);

        let month = (mii_data[0] >> 2) & 0x0F;
        let month = match month == 0 {
            true => None,
            false => Some(month),
        };

        let day = ByteHandler::try_from(&mii_data[0..=1]).unwrap();
        let day = (day.copy_byte(3) >> 5) & 0x1F;
        let day = match day == 0 {
            true => None,
            false => Some(day),
        };

        let favorite_color = (mii_data[1] >> 1) & 0x0F;
        let is_favorite = !mii_data[1].is_multiple_of(2);

        // TODO: somehow make this work (sadly it doesn't)
        // let name = String::from_utf16(unsafe { std::mem::transmute(&mii_data[0x02..=0x15])  }).unwrap();

        let name = utf16be_to_string(&mii_data[0x02..=0x15])?;

        let height = mii_data[0x16] & 0x7F;
        let weight = mii_data[0x17] & 0x7F;

        let mii_id = ByteHandler::try_from(&mii_data[0x18..=0x1B]).unwrap().copy_dword();
        let system_id = ByteHandler::try_from(&mii_data[0x1C..=0x1F]).unwrap().copy_dword();
        
        let face_shape = mii_data[0x20] >> 5;
        let skin_color = (mii_data[0x20] >> 2) & 0x03;
        let mut facial_feature = ByteHandler::try_from(&mii_data[0x20..=0x21]).unwrap();
        facial_feature.shift_right(6);
        let facial_feature = facial_feature.copy_byte(3) & 0x0F;

        let bools = ByteHandler::from(mii_data[0x21]);
        let mingle_off = bools.read_bool(2);
        let downloaded = bools.read_bool(0);

        let mut hair_data = ByteHandler::try_from(&mii_data[0x22..=0x23]).unwrap();
        let hair_type = hair_data.copy_byte(2) >> 1;
        let hair_part_reversed = hair_data.read_bool(4);
        let hair_color = hair_data.copy_byte(3) >> 6;

        let mut eyebrow_data = ByteHandler::try_from(&mii_data[0x24..=0x27]).unwrap();
        let eyebrow_horizontal_spacing = eyebrow_data.copy_byte(3) & 0x0F;
        eyebrow_data.shift_right(3);
        let eyebrow_type = eyebrow_data.copy_byte(0);
        let eyebrow_rotation = eyebrow_data.copy_byte(1) >> 3;
        eyebrow_data.shift_right(1);
        let eyebrow_vertical_pos = eyebrow_data.copy_byte(3) & 0x1F;
        eyebrow_data.shift_right(5);
        let eyebrow_size = eyebrow_data.copy_byte(3) & 0x0F;
        let eyebrow_color = (eyebrow_data.copy_byte(3) >> 4) & 0x03;

        let mut eye_data = ByteHandler::try_from(&mii_data[0x28..=0x2B]).unwrap();
        let eye_vertical_pos = eye_data.copy_byte(1) & 0x1F;
        eye_data.shift_right(1);
        let eye_size = eye_data.copy_byte(2) & 0x0F;
        eye_data.shift_right(1);
        let eye_type = eye_data.copy_byte(0);
        eye_data.shift_right(3);
        let eye_rotation = eye_data.copy_byte(1) & 0x1F;
        let eye_color = eye_data.copy_byte(2) & 0x07;
        let eye_horizontal_spacing = eye_data.copy_byte(3) & 0x0F;

        let mut nose_and_lips_data = ByteHandler::try_from(&mii_data[0x2C..=0x2F]).unwrap();
        let nose_type = nose_and_lips_data.copy_byte(0) >> 4;
        let nose_size = nose_and_lips_data.copy_byte(0) & 0x0F;
        let lip_vertical_pos = nose_and_lips_data.copy_byte(3) & 0x1F;
        nose_and_lips_data.shift_right(1);
        let lip_color = nose_and_lips_data.copy_byte(2) & 0x03;
        nose_and_lips_data.shift_right(2);
        let nose_vertical_pos = nose_and_lips_data.copy_byte(1) & 0x1F;
        let lip_type = nose_and_lips_data.copy_byte(2) & 0x1F;
        nose_and_lips_data.shift_right(2);
        let lip_size = nose_and_lips_data.copy_byte(3) & 0x0F;

        let mut glasses_and_facial_hair_data =
            ByteHandler::try_from(&mii_data[0x30..=0x33]).unwrap();
        let glasses_vertical_pos = glasses_and_facial_hair_data.copy_byte(1) & 0x1F;
        let mustache_vertical_pos = glasses_and_facial_hair_data.copy_byte(3) & 0x1F;
        glasses_and_facial_hair_data.shift_right(1);
        let glasses_color = glasses_and_facial_hair_data.copy_byte(0) & 0x07;
        let facial_hair_color = glasses_and_facial_hair_data.copy_byte(2) & 0x07;
        glasses_and_facial_hair_data.shift_right(3);
        let glasses_type = glasses_and_facial_hair_data.copy_byte(0);
        let beard_type = glasses_and_facial_hair_data.copy_byte(3) & 0x03;
        glasses_and_facial_hair_data.shift_right(1);
        let glasses_size = glasses_and_facial_hair_data.copy_byte(1) & 0x0F;
        let mustache_type = (glasses_and_facial_hair_data.copy_byte(2) >> 1) & 0x03;
        let mustache_size = glasses_and_facial_hair_data.copy_byte(3) & 0x0F;

        let mut mole_data = ByteHandler::try_from(&mii_data[0x34..=0x35]).unwrap();
        mole_data.shift_right(1);
        let has_mole = mole_data.read_bool(14);
        let mole_horizontal_pos = mole_data.copy_byte(3) & 0x1F;
        mole_data.shift_right(2);
        let mole_size = mole_data.copy_byte(2) & 0x04;
        let mole_vertical_pos = mole_data.copy_byte(3) >> 3;

        let creator_name = utf16be_to_string(&mii_data[0x36..=0x49])?;

        Ok(Self {
            is_girl,
            month,
            day,
            favorite_color,
            is_favorite,
            name,
            height,
            weight,
            mii_id,
            system_id,
            face_shape,
            skin_color,
            facial_feature,
            mingle_off,
            downloaded,
            hair_type,
            hair_color,
            hair_part_reversed,
            eyebrow_type,
            eyebrow_rotation,
            eyebrow_color,
            eyebrow_size,
            eyebrow_vertical_pos,
            eyebrow_horizontal_spacing,
            eye_type,
            eye_rotation,
            eye_vertical_pos,
            eye_color,
            eye_size,
            eye_horizontal_spacing,
            nose_type,
            nose_size,
            nose_vertical_pos,
            lip_type,
            lip_color,
            lip_size,
            lip_vertical_pos,
            glasses_type,
            glasses_color,
            glasses_size,
            glasses_vertical_pos,
            mustache_type,
            beard_type,
            facial_hair_color,
            mustache_size,
            mustache_vertical_pos,
            has_mole,
            mole_size,
            mole_vertical_pos,
            mole_horizontal_pos,
            creator_name,
        })
    }

    pub fn is_girl(&self) -> bool {
        self.is_girl
    }

    pub fn month(&self) -> Option<u8> {
        self.month
    }

    pub fn day(&self) -> Option<u8> {
        self.day
    }

    pub fn favorite_color(&self) -> u8 {
        self.favorite_color
    }

    pub fn is_favorite(&self) -> bool {
        self.is_favorite
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn weight(&self) -> u8 {
        self.weight
    }

    pub fn mii_id(&self) -> u32 {
        self.mii_id
    }

    pub fn system_id(&self) -> u32 {
        self.system_id
    }

    pub fn face_shape(&self) -> u8 {
        self.face_shape
    }

    pub fn skin_color(&self) -> u8 {
        self.skin_color
    }

    pub fn facial_feature(&self) -> u8 {
        self.facial_feature
    }

    pub fn mingle_off(&self) -> bool {
        self.mingle_off
    }

    pub fn downloaded(&self) -> bool {
        self.downloaded
    }

    pub fn hair_type(&self) -> u8 {
        self.hair_type
    }

    pub fn hair_color(&self) -> u8 {
        self.hair_color
    }

    pub fn hair_part_reversed(&self) -> bool {
        self.hair_part_reversed
    }

    pub fn eyebrow_type(&self) -> u8 {
        self.eyebrow_type
    }

    pub fn eyebrow_rotation(&self) -> u8 {
        self.eyebrow_rotation
    }

    pub fn eyebrow_color(&self) -> u8 {
        self.eyebrow_color
    }

    pub fn eyebrow_size(&self) -> u8 {
        self.eyebrow_size
    }

    pub fn eyebrow_vertical_pos(&self) -> u8 {
        self.eyebrow_vertical_pos
    }

    pub fn eyebrow_horizontal_spacing(&self) -> u8 {
        self.eyebrow_horizontal_spacing
    }

    pub fn eye_type(&self) -> u8 {
        self.eye_type
    }

    pub fn eye_rotation(&self) -> u8 {
        self.eye_rotation
    }

    pub fn eye_vertical_pos(&self) -> u8 {
        self.eye_vertical_pos
    }

    pub fn eye_color(&self) -> u8 {
        self.eye_color
    }

    pub fn eye_size(&self) -> u8 {
        self.eye_size
    }

    pub fn eye_horizontal_spacing(&self) -> u8 {
        self.eye_horizontal_spacing
    }

    pub fn nose_type(&self) -> u8 {
        self.nose_type
    }

    pub fn nose_size(&self) -> u8 {
        self.nose_size
    }

    pub fn nose_vertical_pos(&self) -> u8 {
        self.nose_vertical_pos
    }

    pub fn lip_type(&self) -> u8 {
        self.lip_type
    }

    pub fn lip_color(&self) -> u8 {
        self.lip_color
    }

    pub fn lip_size(&self) -> u8 {
        self.lip_size
    }

    pub fn lip_vertical_pos(&self) -> u8 {
        self.lip_vertical_pos
    }

    pub fn glasses_type(&self) -> u8 {
        self.glasses_type
    }

    pub fn glasses_color(&self) -> u8 {
        self.glasses_color
    }

    pub fn glasses_size(&self) -> u8 {
        self.glasses_size
    }

    pub fn glasses_vertical_pos(&self) -> u8 {
        self.glasses_vertical_pos
    }

    pub fn mustache_type(&self) -> u8 {
        self.mustache_type
    }

    pub fn beard_type(&self) -> u8 {
        self.beard_type
    }

    pub fn facial_hair_color(&self) -> u8 {
        self.facial_hair_color
    }

    pub fn mustache_size(&self) -> u8 {
        self.mustache_size
    }

    pub fn mustache_vertical_pos(&self) -> u8 {
        self.mustache_vertical_pos
    }

    pub fn has_mole(&self) -> bool {
        self.has_mole
    }

    pub fn mole_size(&self) -> u8 {
        self.mole_size
    }

    pub fn mole_vertical_pos(&self) -> u8 {
        self.mole_vertical_pos
    }

    pub fn mole_horizontal_pos(&self) -> u8 {
        self.mole_horizontal_pos
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
