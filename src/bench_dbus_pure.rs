use super::MessageParts;

pub fn make_dbus_pure_message(parts: &MessageParts, send_it: bool) -> Option<Vec<u8>> {
    let mut header = dbus_pure::proto::MessageHeader {
        r#type: dbus_pure::proto::MessageType::Signal {
            interface: parts.interface.as_str().into(),
            member: parts.member.as_str().into(),
            path: dbus_pure::proto::ObjectPath(parts.object.as_str().into()),
        },
        flags: dbus_pure::proto::message_flags::NO_REPLY_EXPECTED,
        body_len: 0,
        serial: 0,
        fields: (&[][..]).into(),
    };

    let dict_content: Vec<_> = parts
        .dict
        .iter()
        .map(|(k, v)| {
            (
                dbus_pure::proto::Variant::String(k.as_str().into()),
                dbus_pure::proto::Variant::I32(*v),
            )
        })
        .collect();
    let dict_content: Vec<_> = dict_content
        .iter()
        .map(|(k, v)| dbus_pure::proto::Variant::DictEntry {
            key: k.into(),
            value: v.into(),
        })
        .collect();
    let dict = dbus_pure::proto::Variant::Array {
        element_signature: dbus_pure::proto::Signature::DictEntry {
            key: Box::new(dbus_pure::proto::Signature::String),
            value: Box::new(dbus_pure::proto::Signature::I32),
        },
        elements: dict_content.into(),
    };

    let array = dbus_pure::proto::Variant::ArrayU64((&parts.int_array).into());
    let strs = parts
        .string_array
        .iter()
        .map(|s| dbus_pure::proto::Variant::String(s.as_str().into()))
        .collect::<Vec<_>>();
    let strarray = dbus_pure::proto::Variant::Array {
        elements: strs.into(),
        element_signature: dbus_pure::proto::Signature::String,
    };

    let mut elements = vec![];

    for _ in 0..parts.repeat {
        elements.push(dbus_pure::proto::Variant::String(
            parts.string1.as_str().into(),
        ));
        elements.push(dbus_pure::proto::Variant::U64(parts.int1));
        elements.push(dbus_pure::proto::Variant::Struct {
            fields: vec![
                dbus_pure::proto::Variant::U64(parts.int2),
                dbus_pure::proto::Variant::String(parts.string2.as_str().into()),
            ]
            .into(),
        });
        elements.push(dict.clone());
        elements.push(array.clone());
        elements.push(strarray.clone());
    }

    let body = dbus_pure::proto::Variant::Tuple {
        elements: elements.into(),
    };

    if send_it {
        let connection =
            dbus_pure::Connection::new(dbus_pure::BusPath::System, dbus_pure::SaslAuthType::Uid)
                .unwrap();
        let mut dbus_client = dbus_pure::Client::new(connection).unwrap();
        let _ = dbus_client.send(&mut header, Some(&body)).unwrap();
        None
    } else {
        let mut buf = Vec::new();
        dbus_pure::proto::serialize_message(
            &mut header,
            Some(&body),
            &mut buf,
            dbus_pure::proto::Endianness::Little,
        )
        .unwrap();
        Some(buf)
    }
}
