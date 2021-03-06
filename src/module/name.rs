use std::io::Read;
use crate::util::read_u32_from_leb128;

pub(super) type Name = String;

pub(super) fn read_name(reader: &mut impl Read) -> Name {
    let length = read_u32_from_leb128(reader);
    let mut buffer = String::new();
    let mut handle = reader.take(length as u64);
    let _ = handle.read_to_string(&mut buffer);
    buffer
}

#[test]
fn test_read_name() {
    use std::io::BufReader;
    
    let data: [u8; 10] = [
        6,
        0xe3, 0x81, 0x86, // う
        0xe3, 0x81, 0xa9, // ど
        0xe3, 0x82, 0x93, // ん
    ];
    let mut reader = BufReader::new(data.as_ref());
    assert_eq!(read_name(&mut reader), "うど".to_string());
}
