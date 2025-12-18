#[derive(thiserror::Error, Debug)]
pub enum CategoryError {
    #[error("Nonexistent Category")]
    NonexistentCategory,
    #[error("BitReader Error: {0}")]
    BitReaderError(#[from] bitreader::BitReaderError),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Category {
    NoGlitch,
    Glitch,
    NoGlitchTAS,
    GlitchTAS,
}

impl TryFrom<u8> for Category {
    type Error = CategoryError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        /*
         * TODO: Test many different TAS ghosts and see if there are any
         * extra category values not listed here.
         *
         * Nearly all TAS CTGP ghosts have category value 0x13 or 0x23, but
         * in a rare instance 0x03 was seen on a No Glitch TAS ghost
         */
        match value {
            0x00 => Ok(Self::NoGlitch),
            0x01 => Ok(Self::Glitch),
            0x13 => Ok(Self::GlitchTAS),
            0x03 | 0x23 => Ok(Self::NoGlitchTAS),
            _ => Err(CategoryError::NonexistentCategory),
        }
    }
}

impl From<Category> for u8 {
    fn from(value: Category) -> Self {
        match value {
            Category::NoGlitch => 0x00,
            Category::Glitch => 0x01,
            Category::GlitchTAS => 0x13,
            Category::NoGlitchTAS => 0x23,
        }
    }
}

impl TryFrom<&mut bitreader::BitReader<'_>> for Category {
    type Error = CategoryError;
    fn try_from(value: &mut bitreader::BitReader) -> Result<Self, Self::Error> {
        Category::try_from(value.read_u8(8)?)
    }
}
