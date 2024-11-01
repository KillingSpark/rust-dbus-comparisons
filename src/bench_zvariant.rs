use super::MessageParts;

pub fn make_zvariant_message(parts: &MessageParts, send_it: bool) -> Option<zbus::Message> {
    let struct_field = (parts.int2, &parts.string2);

    let mut elements = vec![];

    for _ in 0..parts.repeat {
        let element = (
            parts.string1.as_str(),
            parts.int1,
            &struct_field,
            &parts.dict,
            &parts.int_array,
            &parts.string_array,
        );
        elements.push(element);
    }
    let msg = zbus::Message::signal(
        parts.object.as_str(),
        parts.interface.as_str(),
        parts.member.as_str(),
    )
    .unwrap()
    .build(&elements)
    .unwrap();

    if send_it {
        let con = zbus::blocking::Connection::session().unwrap();
        con.send(&msg).unwrap();
        None
    } else {
        Some(msg)
    }
}

use serde::{Deserialize, Serialize};
use zbus::zvariant::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
pub struct ZVField {
    pub int2: u64,
    pub string2: String,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct ZVStruct {
    pub string1: String,
    pub int1: u64,
    pub field: ZVField,
    pub dict: std::collections::HashMap<String, i32>,
    pub int_array: Vec<u64>,
    pub string_array: Vec<String>,
}

pub fn make_zvariant_derive_message(
    parts: &MessageParts,
    elements: &[ZVStruct],
    send_it: bool,
) -> Option<zbus::Message> {
    let msg = zbus::Message::signal(
        parts.object.as_str(),
        parts.interface.as_str(),
        parts.member.as_str(),
    )
    .unwrap()
    .build(&elements)
    .unwrap();
    if send_it {
        let con = zbus::blocking::Connection::session().unwrap();
        con.send(&msg).unwrap();
        None
    } else {
        Some(msg)
    }
}
