use bitreader::BitReader;

#[derive(thiserror::Error, Debug)]
pub enum MiiError {
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

pub struct Mii {
    is_girl: bool,
    month: u8,
    day: u8,
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
    pub fn new(mii_data: &[u8]) -> Result<Self, MiiError> {
        let mut mii_reader: BitReader<'_> = BitReader::new(mii_data);
        TryFrom::try_from(&mut mii_reader)
    }

    pub fn is_girl(&self) -> bool {
        self.is_girl
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
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

fn get_name(name_chars: &[u16; 10]) -> String {
    let mut name = String::new();
    for c in name_chars {
        if *c != '\0' as u16 {
            name.push(std::char::from_u32(*c as u32).unwrap());
        }
    }
    name
}

impl TryFrom<&mut bitreader::BitReader<'_>> for Mii {
    type Error = MiiError;
    fn try_from(value: &mut bitreader::BitReader) -> Result<Self, Self::Error> {
        let mii_reader = value;

        mii_reader.skip(1)?;
        let is_girl = mii_reader.read_bool()?;
        let month = mii_reader.read_u8(4)?;
        let day = mii_reader.read_u8(5)?;

        let favorite_color = mii_reader.read_u8(4)?;

        let is_favorite = mii_reader.read_bool()?;

        let mut name_chars: [u16; 10] = [0; 10];
        for c in name_chars.iter_mut() {
            *c = mii_reader.read_u16(16)?;
        }
        let name = get_name(&name_chars);

        let height = mii_reader.read_u8(8)?;

        let weight = mii_reader.read_u8(8)?;

        let mii_id = mii_reader.read_u32(32)?;
        let system_id = mii_reader.read_u32(32)?;
        let face_shape = mii_reader.read_u8(3)?;
        let skin_color = mii_reader.read_u8(3)?;
        let facial_feature = mii_reader.read_u8(4)?;

        mii_reader.skip(3)?;
        let mingle_off = mii_reader.read_bool()?;
        mii_reader.skip(1)?;
        let downloaded = mii_reader.read_bool()?;

        let hair_type = mii_reader.read_u8(7)?;
        let hair_color = mii_reader.read_u8(3)?;
        let hair_part_reversed = mii_reader.read_bool()?;

        mii_reader.skip(5)?;

        let eyebrow_type = mii_reader.read_u8(5)?;
        mii_reader.skip(1)?;
        let eyebrow_rotation = mii_reader.read_u8(4)?;
        mii_reader.skip(6)?;
        let eyebrow_color = mii_reader.read_u8(3)?;
        let eyebrow_size = mii_reader.read_u8(4)?;
        let eyebrow_vertical_pos = mii_reader.read_u8(5)?;
        let eyebrow_horizontal_spacing = mii_reader.read_u8(4)?;

        let eye_type = mii_reader.read_u8(6)?;
        mii_reader.skip(2)?;
        let eye_rotation = mii_reader.read_u8(3)?;
        let eye_vertical_pos = mii_reader.read_u8(5)?;
        let eye_color = mii_reader.read_u8(3)?;
        mii_reader.skip(1)?;
        let eye_size = mii_reader.read_u8(3)?;
        let eye_horizontal_spacing = mii_reader.read_u8(4)?;
        mii_reader.skip(5)?;

        let nose_type = mii_reader.read_u8(4)?;
        let nose_size = mii_reader.read_u8(4)?;
        let nose_vertical_pos = mii_reader.read_u8(5)?;
        mii_reader.skip(3)?;

        let lip_type = mii_reader.read_u8(5)?;
        let lip_color = mii_reader.read_u8(2)?;
        let lip_size = mii_reader.read_u8(4)?;
        let lip_vertical_pos = mii_reader.read_u8(5)?;

        let glasses_type = mii_reader.read_u8(4)?;
        let glasses_color = mii_reader.read_u8(3)?;
        mii_reader.skip(1)?;
        let glasses_size = mii_reader.read_u8(3)?;
        let glasses_vertical_pos = mii_reader.read_u8(5)?;

        let mustache_type = mii_reader.read_u8(2)?;
        let beard_type = mii_reader.read_u8(2)?;
        let facial_hair_color = mii_reader.read_u8(3)?;
        let mustache_size = mii_reader.read_u8(4)?;
        let mustache_vertical_pos = mii_reader.read_u8(5)?;

        let has_mole = mii_reader.read_bool()?;
        let mole_size = mii_reader.read_u8(4)?;
        let mole_vertical_pos = mii_reader.read_u8(5)?;
        let mole_horizontal_pos = mii_reader.read_u8(5)?;
        mii_reader.skip(1)?;

        let mut name_chars: [u16; 10] = [0; 10];
        for c in name_chars.iter_mut() {
            *c = mii_reader.read_u16(16)?;
        }
        let creator_name: String = get_name(&name_chars);

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
}
