pub mod rkg;

use rkg::header::Header;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

fn main() {
    // TODO: gather several more test ghosts and data to test data reading
    // TODO: create structs for input data and CTGP metadata footer

    // get the path of the current executable and then go 3 directories up (since .exe is in target/debug/build)
    let mut ghost_file_path: PathBuf = env::current_exe().expect("Failed to get current exe path");
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
    println!("RKGD matches!");

    assert_eq!(header.finish_time().minutes(), 1);
    println!("Finish time minutes matches!");

    assert_eq!(header.finish_time().seconds(), 3);
    println!("Finish time seconds matches!");

    assert_eq!(header.finish_time().milliseconds(), 904);
    println!("Finish time milliseconds matches!");

    assert_eq!(header.finish_time().string(), "01:03.904");
    println!("Finish time string matches!");

    assert_eq!(header.track_id(), 0x08);
    println!("Track ID matches!");

    assert_eq!(header.vehicle_id(), 0x1A);
    println!("Vehicle ID matches!");

    assert_eq!(header.character_id(), 0x13);
    println!("Character ID matches!");

    assert_eq!(header.year_set(), 2025);
    println!("Year set matches!");

    assert_eq!(header.month_set(), 11);
    println!("Month set matches!");

    assert_eq!(header.day_set(), 12);
    println!("Day set matches!");

    assert_eq!(header.controller_id(), 2);
    println!("Controller ID matches!");

    assert_eq!(header.is_compressed(), true);
    println!("Compressed flag matches!");

    assert_eq!(header.ghost_type(), 0x26);
    println!("Ghost type matches!");

    assert_eq!(header.is_automatic_drift(), true);
    println!("Automatic drift flag matches!");

    assert_eq!(header.decompressed_input_data_length(), 1856);
    println!("Decompressed input data length matches!");

    assert_eq!(header.lap_count(), 3);
    println!("Lap count matches!");

    assert_eq!(header.lap_split_times()[0].string(), "00:25.540");
    println!("Lap 1 split matches!");

    assert_eq!(header.lap_split_times()[1].string(), "00:19.127");
    println!("Lap 2 split matches!");

    assert_eq!(header.lap_split_times()[2].string(), "00:19.237");
    println!("Lap 3 split matches!");

    assert_eq!(header.country_code(), 0xFF);
    println!("Country code matches!");

    assert_eq!(header.state_code(), 0xFF);
    println!("State code matches!");

    assert_eq!(header.location_code(), 0xFFFF);
    println!("Location code matches!");

    println!("Mii data matching not implemented yet!");

    assert_eq!(header.mii_crc16(), 1780);
    println!("Mii CRC16 matches!\n");

    println!("All tests passed!");
}
