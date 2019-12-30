#![no_std]

use core::str::FromStr;
use cortex_m_semihosting::hprintln;
use heapless::{
    String,
    Vec,
    consts,
};
use serde::{Deserialize, Serialize};

#[derive(Clone,Debug,Eq,PartialEq,Serialize,Deserialize)]
pub struct Example {
    combo: Vec<String<consts::U8>, consts::U2>,

    #[serde(with = "serde_bytes")]
    field: Vec<u8, consts::U16>,

    // doesn't work
    // #[serde(with = "serde_bytes")]
    fields: Vec<Vec<u8, consts::U16>, consts::U3>,

    #[serde(skip_serializing_if = "Option::is_none")]
    optional_number: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    another_option: Option<u8>,
}

pub fn test() {
    let mut combo: Vec<String<consts::U8>, consts::U2> = Vec::new();
    combo.push(String::from_str("entry").unwrap()).unwrap();

    let mut field: Vec<u8, consts::U16> = Vec::new();
    field.extend_from_slice(&[0x37, 0x42, 0x59]).unwrap();

    let mut fields: Vec<Vec<u8, consts::U16>, consts::U3> = Vec::new();
    fields.push(field.clone()).unwrap();

    let example = Example {
        combo: combo.clone(),
        field: field.clone(),
        fields: fields.clone(),
        optional_number: Some(1023),
        another_option: None,
    };

    let mut buffer = [0u8; 1024];

    let writer = serde_cbor::ser::SliceWrite::new(&mut buffer);
    let mut ser = serde_cbor::Serializer::new(writer);

    example.serialize(&mut ser).unwrap();

    let writer = ser.into_inner();
    let size = writer.bytes_written();

    hprintln!("serialized: {:x?}", &buffer[..size]).ok();

    let mut scratch = [0u8; 128];
    let example: Example = serde_cbor::de::from_slice_with_scratch(
        &buffer[..size], &mut scratch).unwrap();

    // let example: Example = serde_cbor::de::from_mut_slice(
    //     &mut buffer[..size]).unwrap();

    hprintln!("deserialized: {:x?}", &example).ok();

    assert_eq!(field, example.field);
}
