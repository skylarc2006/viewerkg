use bitreader::BitReader;

pub struct Mii {
    unknown1: bool,
    is_girl: bool,
    month: u8,
    day: u8,
    favorite_color: u8,
    is_favorite: bool,
    name: String,
    height: u8,
    weight: u8,
    mii_id1: u8,
    mii_id2: u8,
    mii_id3: u8,
    mii_id4: u8,
    system_id0: u8,
    system_id1: u8,
    system_id2: u8,
    system_id3: u8,
    face_shape: u8,
    skin_color: u8,
    facial_feature: u8,
    unknown2: u8,
    mingle_off: bool,
    unknown3: bool,
    downloaded: bool,
    hair_type: u8,
    hair_color: u8,
    hair_part_reversed: bool,
    unknown4: u8,
    eyebrow_type: u8,
    unknown5: bool,
    eyebrow_rotation: u8,
    unknown6: u8,
    eyebrow_color: u8,
    eyebrow_size: u8,
    eyebrow_vertical_pos: u8,
    eyebrow_horizontal_spacing: u8,
    eye_type: u8,
    unknown7: u8,
    eye_rotation: u8,
    eye_vertical_pos: u8,
    eye_color: u8,
    unknown8: bool,
    eye_size: u8,
    eye_horizontal_spacing: u8,
    unknown9: u8,
    nose_type: u8,
    nose_size: u8,
    nose_vertical_pos: u8,
    unknown10: u8,
    lip_type: u8,
    lip_color: u8,
    lip_size: u8,
    lip_vertical_pos: u8,
    glasses_type: u8,
    glasses_color: u8,
    unknown11: bool,
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
    unknown12: bool,
    creator_name: String,
}

impl Mii {
    pub fn new(mii_data: &[u8]) -> Result<Self, bitreader::BitReaderError> {
        let mut mii_reader: BitReader<'_> = BitReader::new(mii_data);

        let unknown1 = mii_reader.read_bool()?;
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

        let mii_id1 = mii_reader.read_u8(8)?;
        let mii_id2 = mii_reader.read_u8(8)?;
        let mii_id3 = mii_reader.read_u8(8)?;
        let mii_id4 = mii_reader.read_u8(8)?;
        let system_id0 = mii_reader.read_u8(8)?;
        let system_id1 = mii_reader.read_u8(8)?;
        let system_id2 = mii_reader.read_u8(8)?;
        let system_id3 = mii_reader.read_u8(8)?;

        let face_shape = mii_reader.read_u8(3)?;
        let skin_color = mii_reader.read_u8(3)?;
        let facial_feature = mii_reader.read_u8(4)?;

        let unknown2 = mii_reader.read_u8(3)?;
        let mingle_off = mii_reader.read_bool()?;
        let unknown3 = mii_reader.read_bool()?;
        let downloaded = mii_reader.read_bool()?;

        let hair_type = mii_reader.read_u8(7)?;
        let hair_color = mii_reader.read_u8(3)?;
        let hair_part_reversed = mii_reader.read_bool()?;

        let unknown4 = mii_reader.read_u8(5)?;

        let eyebrow_type = mii_reader.read_u8(5)?;
        let unknown5 = mii_reader.read_bool()?;
        let eyebrow_rotation = mii_reader.read_u8(4)?;
        let unknown6 = mii_reader.read_u8(6)?;
        let eyebrow_color = mii_reader.read_u8(3)?;
        let eyebrow_size = mii_reader.read_u8(4)?;
        let eyebrow_vertical_pos = mii_reader.read_u8(5)?;
        let eyebrow_horizontal_spacing = mii_reader.read_u8(4)?;

        let eye_type = mii_reader.read_u8(6)?;
        let unknown7 = mii_reader.read_u8(2)?;
        let eye_rotation = mii_reader.read_u8(3)?;
        let eye_vertical_pos = mii_reader.read_u8(5)?;
        let eye_color = mii_reader.read_u8(3)?;
        let unknown8 = mii_reader.read_bool()?;
        let eye_size = mii_reader.read_u8(3)?;
        let eye_horizontal_spacing = mii_reader.read_u8(4)?;
        let unknown9 = mii_reader.read_u8(5)?;

        let nose_type = mii_reader.read_u8(4)?;
        let nose_size = mii_reader.read_u8(4)?;
        let nose_vertical_pos = mii_reader.read_u8(5)?;
        let unknown10 = mii_reader.read_u8(3)?;

        let lip_type = mii_reader.read_u8(5)?;
        let lip_color = mii_reader.read_u8(2)?;
        let lip_size = mii_reader.read_u8(4)?;
        let lip_vertical_pos = mii_reader.read_u8(5)?;

        let glasses_type = mii_reader.read_u8(4)?;
        let glasses_color = mii_reader.read_u8(3)?;
        let unknown11 = mii_reader.read_bool()?;
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
        let unknown12 = mii_reader.read_bool()?;

        let mut name_chars: [u16; 10] = [0; 10];
        for c in name_chars.iter_mut() {
            *c = mii_reader.read_u16(16)?;
        }
        let creator_name: String = get_name(&name_chars);

        Ok(Self {
            unknown1,
            is_girl,
            month,
            day,
            favorite_color,
            is_favorite,
            name,
            height,
            weight,
            mii_id1,
            mii_id2,
            mii_id3,
            mii_id4,
            system_id0,
            system_id1,
            system_id2,
            system_id3,
            face_shape,
            skin_color,
            facial_feature,
            unknown2,
            mingle_off,
            unknown3,
            downloaded,
            hair_type,
            hair_color,
            hair_part_reversed,
            unknown4,
            eyebrow_type,
            unknown5,
            eyebrow_rotation,
            unknown6,
            eyebrow_color,
            eyebrow_size,
            eyebrow_vertical_pos,
            eyebrow_horizontal_spacing,
            eye_type,
            unknown7,
            eye_rotation,
            eye_vertical_pos,
            eye_color,
            unknown8,
            eye_size,
            eye_horizontal_spacing,
            unknown9,
            nose_type,
            nose_size,
            nose_vertical_pos,
            unknown10,
            lip_type,
            lip_color,
            lip_size,
            lip_vertical_pos,
            glasses_type,
            glasses_color,
            unknown11,
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
            unknown12,
            creator_name,
        })
    }

    pub fn unknown1(&self) -> bool {
        self.unknown1
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

    pub fn mii_id1(&self) -> u8 {
        self.mii_id1
    }

    pub fn mii_id2(&self) -> u8 {
        self.mii_id2
    }

    pub fn mii_id3(&self) -> u8 {
        self.mii_id3
    }

    pub fn mii_id4(&self) -> u8 {
        self.mii_id4
    }

    pub fn system_id0(&self) -> u8 {
        self.system_id0
    }

    pub fn system_id1(&self) -> u8 {
        self.system_id1
    }

    pub fn system_id2(&self) -> u8 {
        self.system_id2
    }

    pub fn system_id3(&self) -> u8 {
        self.system_id3
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

    pub fn unknown2(&self) -> u8 {
        self.unknown2
    }

    pub fn mingle_off(&self) -> bool {
        self.mingle_off
    }

    pub fn unknown3(&self) -> bool {
        self.unknown3
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

    pub fn unknown4(&self) -> u8 {
        self.unknown4
    }

    pub fn eyebrow_type(&self) -> u8 {
        self.eyebrow_type
    }

    pub fn unknown5(&self) -> bool {
        self.unknown5
    }

    pub fn eyebrow_rotation(&self) -> u8 {
        self.eyebrow_rotation
    }

    pub fn unknown6(&self) -> u8 {
        self.unknown6
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

    pub fn unknown7(&self) -> u8 {
        self.unknown7
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

    pub fn unknown8(&self) -> bool {
        self.unknown8
    }

    pub fn eye_size(&self) -> u8 {
        self.eye_size
    }

    pub fn eye_horizontal_spacing(&self) -> u8 {
        self.eye_horizontal_spacing
    }

    pub fn unknown9(&self) -> u8 {
        self.unknown9
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

    pub fn unknown10(&self) -> u8 {
        self.unknown10
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

    pub fn unknown11(&self) -> bool {
        self.unknown11
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

    pub fn unknown12(&self) -> bool {
        self.unknown12
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
