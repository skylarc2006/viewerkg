use std::io::Read;

pub mod byte_handler;
pub mod ctgp_metadata;
pub mod header;
pub mod input_data;

/*
 * TODO:
 * Unfinished/unimplemented functionality
 * ----------------------------------------------
 * Country ID enum
 * State ID enum
 * Location ID enum
 * Read CTGP pause times
 * Handle older CTGP footer versions
 * Create Ghost struct that brings everything together
 * Add CRC validation functions
 * Be able to modify variables in ghost files
 * Implement TryFrom<_> for T where T: Into<ByteHandler>, relies on https://github.com/rust-lang/rust/issues/31844 currently
 */

#[cfg(test)]
mod tests;

struct Ghost {
    header: header::Header,
    input_data: input_data::InputData,
    ctgp_metadata: Option<ctgp_metadata::CTGPMetadata>,
}

impl Ghost {
    fn new_from_file<T: AsRef<std::path::Path>>(path: T) -> Self {
        let mut buf = Vec::with_capacity(0x100);
        std::fs::File::open(path)
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        Self::new(&buf)
    }

    fn new(_bytes: &[u8]) -> Self {
        todo!()
    }
}
