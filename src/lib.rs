//! Nothing in the source
//!
//! This is a benchmark repo, all relevant source is in $REPO/benches

#[cfg(test)]
mod tests {
    use rustbus::params::Container;
    use rustbus::params::DictMap;
    use rustbus::params::Param;
    use rustbus::wire::marshal::marshal;

    fn marsh(msg: &rustbus::message_builder::OutMessage, buf: &mut Vec<u8>) {
        marshal(msg, rustbus::message::ByteOrder::LittleEndian, &[], buf).unwrap();
    }

    struct MessageParts {
        interface: String,
        member: String,
        object: String,

        string1: String,
        string2: String,
        int1: u64,
        int2: u64,

        dict: std::collections::HashMap<String, i32>,
        int_array: Vec<u64>,
        string_array: Vec<String>,

        repeat: usize,
    }

    fn make_rustbus_message<'a, 'e>(parts: &'a MessageParts) -> Vec<u8> {
        let mut params: Vec<Param> = Vec::new();

        let mut dict = DictMap::new();
        for (key, value) in &parts.dict {
            dict.insert(key.as_str().into(), value.into());
        }

        use std::convert::TryFrom;
        let dict: Param = Container::try_from(dict).unwrap().into();

        let intarr: Vec<Param> = parts
            .int_array
            .iter()
            .copied()
            .map(|i| {
                let x: Param = i.into();
                x
            })
            .collect();
        let sig = intarr[0].sig();
        let intarr = rustbus::params::ArrayRef {
            values: &intarr,
            element_sig: sig,
        };
        let intarray: Param = Param::Container(Container::ArrayRef(intarr));

        let stringarr: Vec<Param> = parts
            .string_array
            .iter()
            .map(|i| {
                let x: Param = i.as_str().into();
                x
            })
            .collect();
        let sig = stringarr[0].sig();
        let stringarr = rustbus::params::ArrayRef {
            values: &stringarr,
            element_sig: sig,
        };
        let stringarray: Param = Param::Container(Container::ArrayRef(stringarr));

        for _ in 0..parts.repeat {
            params.push(parts.string1.as_str().into());
            params.push(parts.int1.into());
            params.push(
                Container::Struct(vec![parts.string2.as_str().into(), parts.int2.into()]).into(),
            );
            params.push(dict.clone());
            params.push(intarray.clone());
            params.push(stringarray.clone());
        }

        let mut msg = rustbus::message_builder::MessageBuilder::new()
            .signal(
                parts.interface.clone(),
                parts.member.clone(),
                parts.object.clone(),
            )
            .build();
        msg.body.push_old_params(&params).unwrap();
        msg.serial = Some(1);

        let mut buf = Vec::new();
        marsh(&msg, &mut buf);
        buf
    }

    fn make_dbus_message_parser_message(parts: &MessageParts) -> Vec<u8> {
        let mut signal =
            dbus_message_parser::Message::signal(&parts.object, &parts.interface, &parts.member);

        let dict_content = parts
            .dict
            .iter()
            .map(|(k, v)| {
                dbus_message_parser::Value::DictEntry(Box::new((
                    dbus_message_parser::Value::String(k.clone()),
                    dbus_message_parser::Value::Int32(*v),
                )))
            })
            .collect();
        let dict = dbus_message_parser::Value::Array(dict_content, "{si}".into());

        let array = dbus_message_parser::Value::Array(
            parts
                .int_array
                .iter()
                .copied()
                .map(|i| dbus_message_parser::Value::Uint64(i))
                .collect(),
            "t".to_owned(),
        );
        let stringarray = dbus_message_parser::Value::Array(
            parts
                .string_array
                .iter()
                .cloned()
                .map(|i| dbus_message_parser::Value::String(i))
                .collect(),
            "s".to_owned(),
        );

        for _ in 0..parts.repeat {
            signal.add_value(dbus_message_parser::Value::Uint64(parts.int1));
            signal.add_value(dbus_message_parser::Value::String(parts.string1.clone()));
            signal.add_value(dbus_message_parser::Value::Struct(vec![
                dbus_message_parser::Value::Uint64(parts.int2),
                dbus_message_parser::Value::String(parts.string2.clone()),
            ]));
            signal.add_value(dict.clone());
            signal.add_value(array.clone());
            signal.add_value(stringarray.clone());
        }
        let mut buffer = bytes::BytesMut::new();
        signal.encode(&mut buffer).unwrap();
        buffer.to_vec()
    }

    fn make_dbus_pure_message(parts: &MessageParts) -> Vec<u8> {
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

        let dict_content: Vec<dbus_pure::proto::Variant> = parts
            .dict
            .iter()
            .map(|(k, v)| dbus_pure::proto::Variant::DictEntry {
                key: Box::new(dbus_pure::proto::Variant::String(k.as_str().into())).into(),
                value: Box::new(dbus_pure::proto::Variant::I32(*v)).into(),
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
            elements.push(dbus_pure::proto::Variant::U64(parts.int1));
            elements.push(dbus_pure::proto::Variant::String(
                parts.string1.as_str().into(),
            ));
            elements.push(dbus_pure::proto::Variant::Struct {
                fields: vec![
                    dbus_pure::proto::Variant::U64(0xFFFFFFFFFFFFFFFFu64),
                    dbus_pure::proto::Variant::String("TesttestTesttest".into()),
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

        let mut buf = Vec::new();
        dbus_pure::proto::serialize_message(
            &mut header,
            Some(&body),
            &mut buf,
            dbus_pure::proto::Endianness::Little,
        )
        .unwrap();
        buf
    }

    fn make_dbus_bytestream_message(parts: &MessageParts) -> Vec<u8> {
        let mut msg =
            dbus_bytestream::message::create_signal(&parts.interface, &parts.member, &parts.object);

        let map: std::collections::HashMap<_, _> = parts
            .dict
            .iter()
            .map(|(k, v)| {
                (
                    dbus_serialize::types::BasicValue::String(k.clone()),
                    dbus_serialize::types::Value::BasicValue(
                        dbus_serialize::types::BasicValue::Int32(*v),
                    ),
                )
            })
            .collect();

        let int_array: Vec<_> = parts
            .int_array
            .iter()
            .map(|i| {
                dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
                    *i,
                ))
            })
            .collect();
        let string_array: Vec<_> = parts
            .string_array
            .iter()
            .cloned()
            .map(|i| {
                dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::String(
                    i,
                ))
            })
            .collect();

        let strct = dbus_serialize::types::Struct {
            objects: vec![
                dbus_serialize::types::Value::BasicValue(
                    dbus_serialize::types::BasicValue::Uint64(parts.int2),
                ),
                dbus_serialize::types::Value::BasicValue(
                    dbus_serialize::types::BasicValue::String(parts.string2.clone()),
                ),
            ],
            signature: dbus_serialize::types::Signature("ts".to_owned()),
        };

        for _ in 0..parts.repeat {
            msg = msg.add_arg(&parts.string1);
            msg = msg.add_arg(&parts.int1);
            msg = msg.add_arg(&strct);
            msg = msg.add_arg(&map);
            msg = msg.add_arg(&int_array);
            msg = msg.add_arg(&string_array);
        }

        let mut buf = Vec::new();

        use dbus_bytestream::marshal::Marshal;
        msg.dbus_encode(&mut buf);
        buf
    }

    fn make_mixed_message() -> MessageParts {
        let mut dict = std::collections::HashMap::new();
        dict.insert("A".to_owned(), 1234567i32);
        dict.insert("B".to_owned(), 1234567i32);
        dict.insert("C".to_owned(), 1234567i32);
        dict.insert("D".to_owned(), 1234567i32);
        dict.insert("E".to_owned(), 1234567i32);

        MessageParts {
            string1: "Testtest".to_owned(),
            string2: "TesttestTestest".to_owned(),
            int1: 0xFFFFFFFFFFFFFFFFu64,
            int2: 0xFFFFFFFFFFFFFFFFu64,

            int_array: vec![
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
                0xFFFFFFFFFFFFFFFFu64,
            ],
            string_array: vec!["".into()],
            dict,
            interface: "io.killing.spark".into(),
            member: "TestSignal".into(),
            object: "/io/killing/spark".into(),
            repeat: 10,
        }
    }
    fn make_big_array_message() -> MessageParts {
        let mut dict = std::collections::HashMap::new();
        dict.insert("A".to_owned(), 1234567i32);
        let mut int_array = Vec::new();
        int_array.resize(1024, 0u64);

        MessageParts {
            string1: "Testtest".to_owned(),
            string2: "TesttestTestest".to_owned(),
            int1: 0xFFFFFFFFFFFFFFFFu64,
            int2: 0xFFFFFFFFFFFFFFFFu64,

            int_array,
            string_array: vec!["".into()],
            dict,
            interface: "io.killing.spark".into(),
            member: "TestSignal".into(),
            object: "/io/killing/spark".into(),
            repeat: 1,
        }
    }
    fn make_big_string_array_message() -> MessageParts {
        let mut dict = std::collections::HashMap::new();
        dict.insert("A".to_owned(), 1234567i32);
        let mut string_array = Vec::new();
        for idx in 0..1024 {
            string_array.push(format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}",
                idx, idx, idx, idx, idx, idx, idx, idx, idx, idx, idx, idx
            ))
        }

        MessageParts {
            string1: "Testtest".to_owned(),
            string2: "TesttestTestest".to_owned(),
            int1: 0xFFFFFFFFFFFFFFFFu64,
            int2: 0xFFFFFFFFFFFFFFFFu64,

            string_array,
            int_array: vec![0],
            dict,
            interface: "io.killing.spark".into(),
            member: "TestSignal".into(),
            object: "/io/killing/spark".into(),
            repeat: 1,
        }
    }

    fn make_and_compare(parts: &MessageParts) {
        let rb = make_rustbus_message(parts);
        let bs = make_dbus_bytestream_message(parts);
        let mp = make_dbus_message_parser_message(parts);
        let dp = make_dbus_pure_message(parts);

        let rb_bs = rb.eq(&bs);
        let rb_mp = rb.eq(&mp);
        let rb_dp = rb.eq(&dp);

        let bs_mp = bs.eq(&mp);
        let bs_dp = bs.eq(&dp);

        let dp_mp = dp.eq(&mp);
        assert_eq!(
            [true, true, true, true, true, true],
            [rb_bs, rb_mp, rb_dp, bs_mp, bs_dp, dp_mp]
        );
    }

    #[test]
    fn test_marshalling() {
        let mixed = make_mixed_message();
        make_and_compare(&mixed);
        let array = make_big_string_array_message();
        make_and_compare(&array);
        let array = make_big_array_message();
        make_and_compare(&array);
    }
}
