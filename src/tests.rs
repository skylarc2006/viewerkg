// TODO: Once RKG struct is defined in lib.rs, move this to tests/ in the root directory
// TODO: Once more test files are gathered, write more tests

use crate::header::{Header, slot_id::SlotId};
use std::io::Read;

#[test]
fn test_rkg_header() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let header: Header = Header::new(&rkg_data);

    assert_eq!(header.rkgd(), "RKGD");
    assert_eq!(header.finish_time().minutes(), 1);
    assert_eq!(header.finish_time().seconds(), 3);
    assert_eq!(header.finish_time().milliseconds(), 904);
    assert_eq!(header.finish_time().to_string(), "01:03.904");
    assert_eq!(header.slot_id(), SlotId::LuigiCircuit);
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
    assert_eq!(header.lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(header.lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(header.lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(header.country_code(), 0xFF);
    assert_eq!(header.state_code(), 0xFF);
    assert_eq!(header.location_code(), 0xFFFF);
    // TODO: implement Mii data comparison
    assert_eq!(header.mii_data().is_girl(), false);
    assert_eq!(header.mii_data().month(), 1);
    assert_eq!(header.mii_data().day(), 1);
    assert_eq!(header.mii_data().favorite_color(), 4);
    assert_eq!(header.mii_data().name(), "JC");
    assert_eq!(header.mii_data().height(), 127);
    assert_eq!(header.mii_data().weight(), 127);

    /*
    Mii ID: 893EF2FB
    Console ID: 689EC992
    Face Shape: 3
    Skin Tone: 1
    Face Features: 0
    Can Mingle? true
    Source Type: 0
    Hair Type: 33
    Hair Color: 2
    Hair Flipped? false
    Eyebrow Type: 23
    Eyebrow Rotation: 5
    Eyebrow Color: 1
    Eyebrow Size: 4
    Eyebrow Vertical: 10
    Eyebrow Horizontal: 2
    Eye Type: 5
    Eye Rotation: 4
    Eye Vertical: 9
    Eye Color: 0
    Eye Size: 6
    Eye Horizontal: 1
    Nose Type: 2
    Nose Size: 0
    Nose Vertical: 8
    Mouth Type: 12
    Mouth Color: 0
    Mouth Size: 7
    Mouth Vertical: 6
    Glasses Type: 0
    Glasses Color: 0
    Glasses Size: 4
    Glasses Vertical: 10
    Facial Hair Mustache: 0
    Facial Hair Beard: 0
    Facial Hair Color: 0
    Facial Hair Size: 4
    Facial Hair Vertical: 10
    Mole Type: false
    Mole Size: 4
    Mole Vertical: 20
    Mole Horizontal: 2
    Creator Name: "JC"
    */
    assert_eq!(header.mii_crc16(), 1780);
}
