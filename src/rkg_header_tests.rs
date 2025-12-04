// TODO: Once RKG struct is defined in lib.rs, move this to tests/ in the root directory

#[cfg(test)]
mod tests {
    use crate::rkg::header::Header;
    use std::env;
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_rkg_header() {
        // TODO: gather several more test ghosts and data to test data reading

        // get the path of the current executable and then go 4 directories up (since .exe is in target/debug/deps)
        let mut ghost_file_path: PathBuf =
            env::current_exe().expect("Failed to get current exe path");
        ghost_file_path.pop();
        ghost_file_path.pop();
        ghost_file_path.pop();
        ghost_file_path.pop();
        ghost_file_path.push("test_ghosts");
        ghost_file_path.push("JC_LC.rkg");

        // Path to rkg
        let path: &Path = Path::new(&ghost_file_path);
        let display: std::path::Display<'_> = path.display();

        let mut rkg_data: Vec<u8> = Vec::new();

        // Open file and extract bytes
        match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => {
                println!("File opened successfully!\n");
                for byte_result in file.bytes() {
                    match byte_result {
                        Err(why) => panic!("Failed to read byte: {}", why),
                        Ok(byte) => rkg_data.push(byte),
                    }
                }
            }
        };

        let header: Header = Header::new(&rkg_data);

        assert_eq!(header.rkgd(), "RKGD");
        assert_eq!(header.finish_time().minutes(), 1);
        assert_eq!(header.finish_time().seconds(), 3);
        assert_eq!(header.finish_time().milliseconds(), 904);
        assert_eq!(header.finish_time().string(), "01:03.904");
        assert_eq!(header.track_id(), 0x08);
        assert_eq!(header.vehicle_id(), 0x1A);
        assert_eq!(header.character_id(), 0x13);
        assert_eq!(header.year_set(), 2025);
        assert_eq!(header.month_set(), 11);
        assert_eq!(header.day_set(), 12);
        assert_eq!(header.controller_id(), 2);
        assert_eq!(header.is_compressed(), true);
        assert_eq!(header.ghost_type(), 0x26);
        assert_eq!(header.is_automatic_drift(), true);
        assert_eq!(header.decompressed_input_data_length(), 1856);
        assert_eq!(header.lap_count(), 3);
        assert_eq!(header.lap_split_times()[0].string(), "00:25.540");
        assert_eq!(header.lap_split_times()[1].string(), "00:19.127");
        assert_eq!(header.lap_split_times()[2].string(), "00:19.237");
        assert_eq!(header.country_code(), 0xFF);
        assert_eq!(header.state_code(), 0xFF);
        assert_eq!(header.location_code(), 0xFFFF);
        // TODO: implement Mii data comparison
        assert_eq!(header.mii_crc16(), 1780);
    }
}
