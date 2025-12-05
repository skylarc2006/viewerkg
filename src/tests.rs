// TODO: Once RKG struct is defined in lib.rs, move this to tests/ in the root directory
// TODO: Once more test files are gathered, write more tests

use crate::header::{
    Header,
    combo::{Character, Vehicle},
    controller::Controller,
    date::Date,
    slot_id::SlotId,
};
use std::io::Read;

#[test]
fn test_rkg_header() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let header: Header = Header::new(&rkg_data).expect("Couldn't read header");

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
    assert_eq!(header.is_compressed(), true);
    assert_eq!(header.ghost_type(), 0x26);
    assert_eq!(header.is_automatic_drift(), true);
    assert_eq!(header.decompressed_input_data_length(), 1856);
    assert_eq!(header.lap_count(), 3);
    assert_eq!(header.lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(header.lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(header.lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(header.country_code(), 0xFF);
    assert_eq!(header.state_code(), 0xFF);
    assert_eq!(header.location_code(), 0xFFFF);

    // Mii Data
    assert_eq!(header.mii_data().is_girl(), false);
    assert_eq!(header.mii_data().month(), 1);
    assert_eq!(header.mii_data().day(), 1);
    assert_eq!(header.mii_data().favorite_color(), 4);
    assert_eq!(header.mii_data().name(), "JC");
    assert_eq!(header.mii_data().height(), 127);
    assert_eq!(header.mii_data().weight(), 127);

    assert_eq!(header.mii_data().mii_id1(), 0x89);
    assert_eq!(header.mii_data().mii_id2(), 0x3E);
    assert_eq!(header.mii_data().mii_id3(), 0xF2);
    assert_eq!(header.mii_data().mii_id4(), 0xFB);
    assert_eq!(header.mii_data().system_id0(), 0x68);
    assert_eq!(header.mii_data().system_id1(), 0x9E);
    assert_eq!(header.mii_data().system_id2(), 0xC9);
    assert_eq!(header.mii_data().system_id3(), 0x92);

    assert_eq!(header.mii_data().face_shape(), 3);
    assert_eq!(header.mii_data().skin_color(), 1);
    assert_eq!(header.mii_data().facial_feature(), 0);

    assert_eq!(header.mii_data().mingle_off(), true);
    assert_eq!(header.mii_data().downloaded(), false);

    assert_eq!(header.mii_data().hair_type(), 33);
    assert_eq!(header.mii_data().hair_color(), 2);
    assert_eq!(header.mii_data().hair_part_reversed(), false);

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

    assert_eq!(header.mii_data().has_mole(), false);
    assert_eq!(header.mii_data().mole_size(), 4);
    assert_eq!(header.mii_data().mole_vertical_pos(), 20);
    assert_eq!(header.mii_data().mole_horizontal_pos(), 2);

    assert_eq!(header.mii_data().creator_name(), "JC");

    assert_eq!(header.mii_crc16(), 0x06F4);
}
