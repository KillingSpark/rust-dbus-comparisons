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
    let sender: Option<&str> = None;
    let msg = zbus::Message::signal(
        sender,
        Some(parts.interface.as_str()),
        parts.object.as_str(),
        parts.interface.as_str(),
        parts.member.as_str(),
        &elements,
    )
    .unwrap();

    if send_it {
        async_std::task::block_on(async {
            let con = zbus::Connection::session().await.unwrap();
            con.send_message(msg).await.unwrap();
        });
        None
    } else {
        Some(msg)
    }
}

use serde::{Deserialize, Serialize};
use zvariant_derive::Type;

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
    let sender: Option<&str> = None;
    let msg = zbus::Message::signal(
        sender,
        Some(parts.interface.as_str()),
        parts.object.as_str(),
        parts.interface.as_str(),
        parts.member.as_str(),
        &elements,
    )
    .unwrap();
    if send_it {
        async_std::task::block_on(async {
            let con = zbus::Connection::session().await.unwrap();
            con.send_message(msg).await.unwrap();
        });
        None
    } else {
        Some(msg)
    }
}
