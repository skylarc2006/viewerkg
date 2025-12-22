use crate::{
    ctgp_metadata::{CTGPMetadata, exact_finish_time::ExactFinishTime},
    header::{
        Header,
        combo::{Character, Vehicle},
        controller::Controller,
        date::Date,
        ghost_type::GhostType,
        location::country::Country,
        slot_id::SlotId,
    },
    input_data::InputData,
};
use std::io::Read;

#[test]
fn test_rkg_header() {
    let header =
        Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").expect("Couldn't read header");

    // General ghost info
    assert_eq!(header.finish_time().minutes(), 1);
    assert_eq!(header.finish_time().seconds(), 3);
    assert_eq!(header.finish_time().milliseconds(), 904);
    assert_eq!(header.finish_time().to_string(), "01:03.904");
    assert_eq!(header.slot_id(), SlotId::LuigiCircuit);
    assert_eq!(header.combo().vehicle(), Vehicle::WarioBike);
    assert_eq!(header.combo().character(), Character::KingBoo);
    assert_eq!(header.date_set(), &Date::new(2025, 11, 12).unwrap());
    assert_eq!(header.controller(), Controller::Classic);
    assert!(header.is_compressed());
    assert_eq!(header.ghost_type(), GhostType::ExpertStaff);
    assert!(header.is_automatic_drift());
    assert_eq!(header.decompressed_input_data_length(), 1856);
    assert_eq!(header.lap_count(), 3);
    assert_eq!(header.lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(header.lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(header.lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(header.country(), Country::NotSet);
    assert_eq!(header.subregion(), 0xFF);
    assert_eq!(header.location_code(), 0xFFFF);

    // Mii Data
    assert!(!header.mii().is_girl());
    assert_eq!(header.mii().month(), Some(1));
    assert_eq!(header.mii().day(), Some(1));
    assert_eq!(header.mii().favorite_color(), 4);
    assert_eq!(header.mii().name(), "JC");
    assert_eq!(header.mii().height(), 127);
    assert_eq!(header.mii().weight(), 127);

    assert_eq!(header.mii().mii_id(), 0x893EF2FB);
    assert_eq!(header.mii().system_id(), 0x689EC992);

    assert_eq!(header.mii().face_shape(), 3);
    assert_eq!(header.mii().skin_color(), 1);
    assert_eq!(header.mii().facial_feature(), 0);

    assert!(header.mii().mingle_off());
    assert!(!header.mii().downloaded());

    assert_eq!(header.mii().hair_type(), 33);
    assert_eq!(header.mii().hair_color(), 2);
    assert!(!header.mii().hair_part_reversed());

    assert_eq!(header.mii().eyebrow_type(), 23);
    assert_eq!(header.mii().eyebrow_rotation(), 5);
    assert_eq!(header.mii().eyebrow_color(), 1);
    assert_eq!(header.mii().eyebrow_size(), 4);
    assert_eq!(header.mii().eyebrow_vertical_pos(), 10);
    assert_eq!(header.mii().eyebrow_horizontal_spacing(), 2);

    assert_eq!(header.mii().eye_type(), 5);
    assert_eq!(header.mii().eye_rotation(), 4);
    assert_eq!(header.mii().eye_vertical_pos(), 9);
    assert_eq!(header.mii().eye_color(), 0);
    assert_eq!(header.mii().eye_size(), 6);
    assert_eq!(header.mii().eye_horizontal_spacing(), 1);

    assert_eq!(header.mii().nose_type(), 2);
    assert_eq!(header.mii().nose_size(), 0);
    assert_eq!(header.mii().nose_vertical_pos(), 8);

    assert_eq!(header.mii().lip_type(), 12);
    assert_eq!(header.mii().lip_color(), 0);
    assert_eq!(header.mii().lip_size(), 7);
    assert_eq!(header.mii().lip_vertical_pos(), 6);

    assert_eq!(header.mii().glasses_type(), 0);
    assert_eq!(header.mii().glasses_color(), 0);
    assert_eq!(header.mii().glasses_size(), 4);
    assert_eq!(header.mii().glasses_vertical_pos(), 10);

    assert_eq!(header.mii().mustache_type(), 0);
    assert_eq!(header.mii().beard_type(), 0);
    assert_eq!(header.mii().facial_hair_color(), 0);
    assert_eq!(header.mii().mustache_size(), 4);
    assert_eq!(header.mii().mustache_vertical_pos(), 10);

    assert!(!header.mii().has_mole());
    assert_eq!(header.mii().mole_size(), 4);
    assert_eq!(header.mii().mole_vertical_pos(), 20);
    assert_eq!(header.mii().mole_horizontal_pos(), 2);

    assert_eq!(header.mii().creator_name(), "JC");

    assert_eq!(header.mii_crc16(), 0x06F4);
    assert!(header.verify_mii_crc16());
}

#[test]
fn test_rkg_input_data() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let input_data =
        InputData::new(&rkg_data[0x88..rkg_data.len() - 0xE0]).expect("Couldn't read input data");

    assert_eq!(input_data.face_input_count(), 0x18);
    assert_eq!(input_data.stick_input_count(), 0x037B);
    assert_eq!(input_data.dpad_input_count(), 0x09);
    assert_eq!(input_data.inputs().len(), 907);
    assert_eq!(input_data.face_inputs().len(), 12);
    assert_eq!(input_data.stick_inputs().len(), 891);
    assert_eq!(input_data.dpad_inputs().len(), 9);
}

#[test]
fn test_ctgp_metadata() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPMetadata::new(&rkg_data).expect("Failed to read CTGP metadata");

    // Some asserts
    assert_eq!(
        ctgp_metadata.track_sha1(),
        [
            0x1A, 0xE1, 0xA7, 0xD8, 0x94, 0x96, 0x0B, 0x38, 0xE0, 0x9E, 0x74, 0x94, 0x37, 0x33,
            0x78, 0xD8, 0x73, 0x05, 0xA1, 0x63
        ]
    );
    assert_eq!(
        ctgp_metadata.player_id().to_be_bytes(),
        [0xFD, 0x31, 0x97, 0xB0, 0x7D, 0x9D, 0x2B, 0x84]
    );
    let shroomstrat: [u8; 3] = [3, 0, 0];
    assert_eq!(ctgp_metadata.shroomstrat(), &shroomstrat);
}

#[test]
fn print_ctgp_metadata() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/00m58s6479888 David .rkg")
        .expect("Couldn't find `./test_ghosts/00m58s6479888 David .rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPMetadata::new(&rkg_data).expect("Failed to read CTGP metadata");

    // Print info
    print!("Track SHA1: ");
    for byte in ctgp_metadata.track_sha1().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    print!("Player ID: ");
    for byte in ctgp_metadata.player_id().to_be_bytes().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    println!("Exact finish time: {}", ctgp_metadata.exact_finish_time());
    println!(
        "CTGP Version (currently hardcoded): {}\n",
        ctgp_metadata.ctgp_version().unwrap()
    );

    for (index, time) in ctgp_metadata.exact_lap_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, time);
    }
    println!();

    println!("RTC Race Begin: {}", ctgp_metadata.rtc_race_begins());
    println!("RTC Race End: {}", ctgp_metadata.rtc_race_end());
    println!(
        "RTC Time Paused: {}ms",
        ctgp_metadata.rtc_time_paused().num_milliseconds()
    );
    println!("List of pause frames: {:#?}", ctgp_metadata.pause_frames());

    println!("\nMy Stuff enabled? {}", ctgp_metadata.my_stuff_enabled());
    println!("My Stuff used? {}", ctgp_metadata.my_stuff_used());
    println!(
        "USB Gamecube enabled? {}",
        ctgp_metadata.usb_gamecube_enabled()
    );
    println!(
        "Final lap dubious intersection? {}",
        ctgp_metadata.final_lap_dubious_intersection()
    );

    println!(
        "\nAll lap dubious intersection bools: {:?}",
        ctgp_metadata.lap_split_dubious_intersections().unwrap()
    );

    println!("\nShroomstrat: {:?}", ctgp_metadata.shroomstrat());
    println!("Category: {:?}", ctgp_metadata.category());
    println!("Cannoned? {}", ctgp_metadata.cannoned());
    println!("Went OOB? {}", ctgp_metadata.went_oob());
    println!("Slowdown suspected? {}", ctgp_metadata.has_slowdown());
    println!("Rapidfire suspected? {}", ctgp_metadata.has_rapidfire());
    println!("Suspicious ghost? {}", ctgp_metadata.dubious_ghost());
    println!(
        "Has Mii data replaced? {}",
        ctgp_metadata.has_mii_data_replaced()
    );
    println!(
        "Has Mii name replaced? {}",
        ctgp_metadata.has_name_replaced()
    );
    println!("Respawns? {}", ctgp_metadata.respawns());
    println!(
        "CTGP metadata version: {}",
        ctgp_metadata.metadata_version()
    );
}

/// CTGP adds a pause mask to frames where a pause is pressed. Actual race inputs should stay the same.
#[test]
fn test_ctgp_pause_vs_vanilla_input_timing() {
    let mut pause_rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/skylar_pause_ghost_compressed.rkg")
        .expect("Couldn't find `./test_ghosts/skylar_pause_ghost_compressed.rkg`")
        .read_to_end(&mut pause_rkg_data)
        .expect("Couldn't read bytes in file");

    let mut vanilla_rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/skylar_pause_ghost_vanilla.rkg")
        .expect("Couldn't find `./test_ghosts/skylar_pause_ghost_vanilla.rkg`")
        .read_to_end(&mut vanilla_rkg_data)
        .expect("Couldn't read bytes in file");

    let pause_inputs = InputData::new(&pause_rkg_data[0x88..pause_rkg_data.len() - 0xE0])
        .expect("Failed to read inputs from pause ghost");
    let vanilla_inputs = InputData::new(&vanilla_rkg_data[0x88..vanilla_rkg_data.len() - 0x04])
        .expect("Failed to read inputs from vanilla ghost");

    assert_eq!(pause_inputs.face_inputs(), vanilla_inputs.face_inputs());
    assert_eq!(pause_inputs.stick_inputs(), vanilla_inputs.stick_inputs());
    assert_eq!(pause_inputs.dpad_inputs(), pause_inputs.dpad_inputs());

    assert_eq!(pause_inputs.inputs(), vanilla_inputs.inputs());
}

#[test]
#[should_panic(expected = "FaceInputError(InvalidButton(IllegalDriftInput))")]
fn illegal_drift_input_test() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/illegal_drift_inputs.rkg")
        .expect("Couldn't find `./test_ghosts/illegal_drift_inputs.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    // This line should always fail
    let _input_data = InputData::new(&rkg_data).expect("Failed to read input data");
}

#[test]
#[should_panic(expected = "FaceInputError(InvalidButton(IllegalDriftInput))")]
fn illegal_brake_input_test() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/illegal_brake_input.rkg")
        .expect("Couldn't find `./test_ghosts/illegal_brake_input.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    // This line should always fail
    let _input_data = InputData::new(&rkg_data).expect("Failed to read input data");
}

#[test]
fn test_nine_laps() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/9laps_test.rkg")
        .expect("Couldn't find `./test_ghosts/9laps_test.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let header = Header::new(&rkg_data[..0x88]).expect("Couldn't read header");

    for (index, lap) in header.lap_split_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, lap.to_string());
    }

    println!("\nTotal time: {}", header.finish_time().to_string());
}
