use crate::{
    ctgp_metadata::CTGPMetadata,
    header::{
        Header,
        combo::{Character, Vehicle},
        controller::Controller,
        date::Date,
        ghost_type::GhostType,
        slot_id::SlotId,
    },
    input_data::InputData,
};
use std::io::Read;

#[test]
fn test_rkg_header() {
    let header = Header::new_from_path("./test_ghosts/JC_LC.rkg").expect("Couldn't read header");

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
    assert!(!header.is_compressed());
    assert_eq!(header.ghost_type(), GhostType::ExpertStaff);
    assert!(header.is_automatic_drift());
    assert_eq!(header.decompressed_input_data_length(), 1856);
    assert_eq!(header.lap_count(), 3);
    assert_eq!(header.lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(header.lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(header.lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(header.country_code(), 0xFF);
    assert_eq!(header.state_code(), 0xFF);
    assert_eq!(header.location_code(), 0xFFFF);

    // Mii Data
    assert!(!header.mii_data().is_girl());
    assert_eq!(header.mii_data().month(), 1);
    assert_eq!(header.mii_data().day(), 1);
    assert_eq!(header.mii_data().favorite_color(), 4);
    assert_eq!(header.mii_data().name(), "JC");
    assert_eq!(header.mii_data().height(), 127);
    assert_eq!(header.mii_data().weight(), 127);

    assert_eq!(header.mii_data().mii_id(), 0x893EF2FB);
    assert_eq!(header.mii_data().system_id(), 0x689EC992);

    assert_eq!(header.mii_data().face_shape(), 3);
    assert_eq!(header.mii_data().skin_color(), 1);
    assert_eq!(header.mii_data().facial_feature(), 0);

    assert!(header.mii_data().mingle_off());
    assert!(!header.mii_data().downloaded());

    assert_eq!(header.mii_data().hair_type(), 33);
    assert_eq!(header.mii_data().hair_color(), 2);
    assert!(!header.mii_data().hair_part_reversed());

    assert_eq!(header.mii_data().eyebrow_type(), 23);
    assert_eq!(header.mii_data().eyebrow_rotation(), 5);
    assert_eq!(header.mii_data().eyebrow_color(), 1);
    assert_eq!(header.mii_data().eyebrow_size(), 4);
    assert_eq!(header.mii_data().eyebrow_vertical_pos(), 10);
    assert_eq!(header.mii_data().eyebrow_horizontal_spacing(), 2);

    assert_eq!(header.mii_data().eye_type(), 5);
    assert_eq!(header.mii_data().eye_rotation(), 4);
    assert_eq!(header.mii_data().eye_vertical_pos(), 9);
    assert_eq!(header.mii_data().eye_color(), 0);
    assert_eq!(header.mii_data().eye_size(), 6);
    assert_eq!(header.mii_data().eye_horizontal_spacing(), 1);

    assert_eq!(header.mii_data().nose_type(), 2);
    assert_eq!(header.mii_data().nose_size(), 0);
    assert_eq!(header.mii_data().nose_vertical_pos(), 8);

    assert_eq!(header.mii_data().lip_type(), 12);
    assert_eq!(header.mii_data().lip_color(), 0);
    assert_eq!(header.mii_data().lip_size(), 7);
    assert_eq!(header.mii_data().lip_vertical_pos(), 6);

    assert_eq!(header.mii_data().glasses_type(), 0);
    assert_eq!(header.mii_data().glasses_color(), 0);
    assert_eq!(header.mii_data().glasses_size(), 4);
    assert_eq!(header.mii_data().glasses_vertical_pos(), 10);

    assert_eq!(header.mii_data().mustache_type(), 0);
    assert_eq!(header.mii_data().beard_type(), 0);
    assert_eq!(header.mii_data().facial_hair_color(), 0);
    assert_eq!(header.mii_data().mustache_size(), 4);
    assert_eq!(header.mii_data().mustache_vertical_pos(), 10);

    assert!(!header.mii_data().has_mole());
    assert_eq!(header.mii_data().mole_size(), 4);
    assert_eq!(header.mii_data().mole_vertical_pos(), 20);
    assert_eq!(header.mii_data().mole_horizontal_pos(), 2);

    assert_eq!(header.mii_data().creator_name(), "JC");

    assert_eq!(header.mii_crc16(), 0x06F4);
}

#[test]
fn test_rkg_input_data() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    // TODO: Handle CKGD
    /* In vanilla ghosts, input data always ends 4 bytes before the end of the file,
     * but with a CTGP ghost the input data would end [CTGP info footer length] bytes
     * before the end of the file.
     */
    let input_data =
        InputData::new(&rkg_data[0x88..rkg_data.len() - 0xE0]).expect("Couldn't read input data");

    assert_eq!(input_data.face_input_count(), 0x18);
    assert_eq!(input_data.stick_input_count(), 0x037B);
    assert_eq!(input_data.dpad_input_count(), 0x09);
}

#[test]
fn test_ctgp_metadata() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_len_offset = rkg_data.len() - 0xC;
    let metadata_length = u32::from_be_bytes([
        rkg_data[ctgp_len_offset],
        rkg_data[ctgp_len_offset + 1],
        rkg_data[ctgp_len_offset + 2],
        rkg_data[ctgp_len_offset + 3],
    ]) as usize;

    let ctgp_metadata =
        CTGPMetadata::new(&rkg_data[&rkg_data.len() - metadata_length..&rkg_data.len() - 0x04])
            .expect("Failed to read CTGP metadata");

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
    let shroomstrat: [u8; 8] = [3, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(ctgp_metadata.shroomstrat(), &shroomstrat);
}
