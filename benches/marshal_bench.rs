use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustbus::wire::marshal::marshal;

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

fn make_dbus_native_message(parts: &MessageParts, send_it: bool) {
    use dbus_native::marshalled::Marshal;
    use dbus_native::strings::{DBusStr, StringLike};

    let ksig = <&DBusStr>::default().signature().into();
    let vsig = i32::default().signature().into();
    let mut dict = dbus_native::marshalled::DictBuf::new(ksig, vsig).unwrap();
    for (key, value) in &parts.dict {
        let key = dbus_native::strings::DBusStr::new(key).unwrap();
        dict.append(key, value).unwrap();
    }

    let mut intarr = dbus_native::marshalled::ArrayBuf::new(u64::default().signature()).unwrap();
    for x in &parts.int_array {
        intarr.append(x).unwrap();
    }

    let stringarr = dbus_native::marshalled::ArrayBuf::from_iter(
        parts
            .string_array
            .iter()
            .map(|x| dbus_native::strings::DBusStr::new(x).unwrap()),
    )
    .unwrap();

    let string1 = dbus_native::strings::DBusStr::new(&parts.string1).unwrap();
    let string2 = dbus_native::strings::DBusStr::new(&parts.string2).unwrap();
    let mut stru = dbus_native::marshalled::MultiBuf::new();
    stru.append(&parts.int2).unwrap();
    stru.append(string2).unwrap();
    let stru = dbus_native::marshalled::StructBuf::new(stru).unwrap();
    let mut body = dbus_native::marshalled::MultiBuf::new();
    for _ in 0..parts.repeat {
        body.append(string1).unwrap();
        body.append(&parts.int1).unwrap();
        body.append(&stru).unwrap();
        body.append(&dict).unwrap();
        body.append(&intarr).unwrap();
        body.append(&stringarr).unwrap();
    }

    let path = dbus_native::strings::ObjectPath::new(&parts.object).unwrap();
    let interface = dbus_native::strings::InterfaceName::new(&parts.interface).unwrap();
    let member = dbus_native::strings::MemberName::new(&parts.member).unwrap();
    let mut msg =
        dbus_native::message::Message::new_signal(path.into(), interface.into(), member.into())
            .unwrap();
    msg.set_body(body);

    if send_it {
        let addr = dbus_native::address::read_session_address().unwrap();
        let stream = dbus_native::address::connect_blocking(&addr).unwrap();

        let mut reader = std::io::BufReader::new(&stream);
        let mut writer = &stream;
        assert!(!dbus_native::authentication::Authentication::blocking(
            &mut reader,
            &mut writer,
            false
        )
        .unwrap());
        writer.flush().unwrap();

        let hellomsg = dbus_native::message::get_hello_message()
            .marshal(std::num::NonZeroU32::new(1u32).unwrap(), false)
            .unwrap();
        use std::io::Write;
        writer.write_all(&hellomsg).unwrap();
        writer.flush().unwrap();

        let mut mr = dbus_native::message::MessageReader::new();
        let reply = mr.block_until_next_message(&mut reader).unwrap();
        let reply = dbus_native::message::Message::demarshal(&reply)
            .unwrap()
            .unwrap();
        let _our_id = reply
            .read_body()
            .iter()
            .next()
            .unwrap()
            .unwrap()
            .parse()
            .unwrap();

        let buf = msg
            .marshal(std::num::NonZeroU32::new(2u32).unwrap(), false)
            .unwrap();
        black_box(&buf);
    } else {
        let buf = msg
            .marshal(std::num::NonZeroU32::new(1u32).unwrap(), false)
            .unwrap();
        black_box(&buf);
    }
}

fn make_rustbus_message<'a, 'e>(parts: &'a MessageParts, send_it: bool) {
    let mut msg = rustbus::message_builder::MessageBuilder::new()
        .signal(
            parts.interface.clone(),
            parts.member.clone(),
            parts.object.clone(),
        )
        .build();

    for _ in 0..parts.repeat {
        msg.body.push_param(parts.string1.as_str()).unwrap();
        msg.body.push_param(parts.int1).unwrap();
        msg.body
            .push_param((parts.int2, parts.string2.as_str()))
            .unwrap();
        msg.body.push_param(&parts.dict).unwrap();
        msg.body
            .push_param(rustbus::wire::marshal_trait::OptimizedMarshal(
                parts.int_array.as_slice(),
            ))
            .unwrap();
        msg.body.push_param(parts.string_array.as_slice()).unwrap();
    }

    msg.serial = Some(1);

    if send_it {
        let mut rustbus_con = rustbus::client_conn::RpcConn::new(
            rustbus::client_conn::Conn::connect_to_bus(
                rustbus::get_session_bus_path().unwrap(),
                false,
            )
            .unwrap(),
        );
        let serial = rustbus_con
            .send_message(
                &mut rustbus::standard_messages::hello(),
                rustbus::client_conn::Timeout::Infinite,
            )
            .unwrap();
        let _name_resp = rustbus_con
            .wait_response(serial, rustbus::client_conn::Timeout::Infinite)
            .unwrap();
        let _serial = rustbus_con
            .send_message(&mut msg, rustbus::client_conn::Timeout::Infinite)
            .unwrap();
    } else {
        let mut buf = Vec::new();
        marshal(&msg, rustbus::message::ByteOrder::LittleEndian, &[], &mut buf).unwrap();
        black_box(buf);
    }
}

fn make_dbus_message_parser_message(parts: &MessageParts, send_it: bool) {
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
        signal.add_value(dbus_message_parser::Value::String(parts.string1.clone()));
        signal.add_value(dbus_message_parser::Value::Uint64(parts.int1));
        signal.add_value(dbus_message_parser::Value::Struct(vec![
            dbus_message_parser::Value::Uint64(parts.int2),
            dbus_message_parser::Value::String(parts.string2.clone()),
        ]));
        signal.add_value(dict.clone());
        signal.add_value(array.clone());
        signal.add_value(stringarray.clone());
    }
    if send_it {
        // no send implemented
    } else {
        let mut buffer = bytes::BytesMut::new();
        signal.encode(&mut buffer).unwrap();
        black_box(buffer);
    }
}

fn make_dbus_pure_message(parts: &MessageParts, send_it: bool) {
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
    } else {
        let mut buf = Vec::new();
        dbus_pure::proto::serialize_message(
            &mut header,
            Some(&body),
            &mut buf,
            dbus_pure::proto::Endianness::Little,
        )
        .unwrap();
        black_box(buf);
    }
}

fn make_dbusrs_message(parts: &MessageParts, send_it: bool) {
    let mut msg = dbus::message::Message::signal(
        &dbus::strings::Path::from(&parts.object),
        &dbus::strings::Interface::from(&parts.interface),
        &dbus::strings::Member::from(&parts.member),
    );

    let dict = dbus::arg::Dict::new(parts.dict.iter().map(|(k, v)| (k, v)));

    for _ in 0..parts.repeat {
        msg = msg.append3(&parts.string1, parts.int1, (parts.int2, &parts.string2));
        msg = msg.append3(&dict, &parts.int_array, &parts.string_array);
    }

    if send_it {
        use dbus::channel::Sender;
        let conn = dbus::blocking::Connection::new_session().unwrap();
        conn.send(msg).unwrap();
    } else {
        // no need to marshal, that happend while building
        black_box(msg);
    }
}

fn make_zvariant_message(parts: &MessageParts, send_it: bool) {
    let struct_field = (
        parts.int2,
        &parts.string2,
    );

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

    if send_it {
        // no send implemented
    } else {
        let msg = zbus::Message::method(
            None,
            Some(&parts.interface),
            &parts.object,
            Some(&parts.interface),
            &parts.member,
            &elements,
        )
        .unwrap();
        black_box(msg);
    }
}

use serde::{Deserialize, Serialize};
use zvariant_derive::Type;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug, Clone)]
struct ZVField {
    int2: u64,
    string2: String,
}

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
struct ZVStruct {
    string1: String,
    int1: u64,
    field: ZVField,
    dict: std::collections::HashMap<String, i32>,
    int_array: Vec<u64>,
    string_array: Vec<String>,
}

fn make_zvariant_derive_message(parts: &MessageParts, elements: &[ZVStruct], send_it: bool) {
    if send_it {
        // no send implemented
    } else {
        let msg = zbus::Message::method(
            None,
            Some(&parts.interface),
            &parts.object,
            Some(&parts.interface),
            &parts.member,
            &elements,
        )
        .unwrap();
        black_box(msg);
    }
}

fn make_dbus_bytestream_message(parts: &MessageParts, send_it: bool) {
    let mut msg =
        dbus_bytestream::message::create_signal(&parts.interface, &parts.member, &parts.object);

    let map: std::collections::HashMap<_, _> = parts
        .dict
        .iter()
        .map(|(k, v)| {
            (
                dbus_serialize::types::BasicValue::String(k.clone()),
                dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Int32(
                    *v,
                )),
            )
        })
        .collect();

    let int_array: Vec<_> = parts
        .int_array
        .iter()
        .map(|i| {
            dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(*i))
        })
        .collect();
    let string_array: Vec<_> = parts
        .string_array
        .iter()
        .cloned()
        .map(|i| {
            dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::String(i))
        })
        .collect();

    let strct = dbus_serialize::types::Struct {
        objects: vec![
            dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::Uint64(
                parts.int2,
            )),
            dbus_serialize::types::Value::BasicValue(dbus_serialize::types::BasicValue::String(
                parts.string2.clone(),
            )),
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

    if send_it {
        let conn = dbus_bytestream::connection::Connection::connect_session().unwrap();
        conn.send(msg).unwrap();
    } else {
        let mut buf = Vec::new();
        use dbus_bytestream::marshal::Marshal;
        msg.dbus_encode(&mut buf);
        black_box(buf);
    }
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
    int_array.resize(1024 * 10, 0u64);

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
    for idx in 0..1024 * 10 {
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

fn run_marshal_benches(group_name: &str, c: &mut Criterion, parts: &MessageParts) {
    let mut group = c.benchmark_group(group_name);
    group.bench_function("marshal_rustbus", |b| {
        b.iter(|| {
            make_rustbus_message(parts, false);
        })
    });
    group.bench_function("marshal_dbusrs", |b| {
        b.iter(|| {
            make_dbusrs_message(parts, false);
        })
    });
    group.bench_function("marshal_dbus_native", |b| {
        b.iter(|| {
            make_dbus_native_message(parts, false);
        })
    });
    group.bench_function("marshal_dbus_bytestream", |b| {
        b.iter(|| {
            make_dbus_bytestream_message(parts, false);
        })
    });
    group.bench_function("marshal_dbus_msg_parser", |b| {
        b.iter(|| {
            make_dbus_message_parser_message(parts, false);
        })
    });
    group.bench_function("marshal_dbus_pure", |b| {
        b.iter(|| {
            make_dbus_pure_message(parts, false);
        })
    });
    group.bench_function("marshal_zvariant", |b| {
        b.iter(|| {
            make_zvariant_message(parts, false);
        })
    });
    group.bench_function("marshal_zvariant_derive", |b| {
        // We don't necessarily need to clone anything here and keep refs in the structs but let's
        // avoid all the lifetimes fun, shall we? :) The struct creation is (intentionally) not
        // part of the benchmark anyway.
        let field = ZVField {
            int2: parts.int2,
            string2: parts.string2.clone(),
        };

        let mut elements = vec![];

        for _ in 0..parts.repeat {
            let element = ZVStruct {
                string1: parts.string1.clone(),
                int1: parts.int1,
                field: field.clone(),
                dict: parts.dict.clone(),
                int_array: parts.int_array.clone(),
                string_array: parts.string_array.clone(),
            };
            elements.push(element);
        }

        b.iter(|| {
            make_zvariant_derive_message(parts, &elements, false);
        })
    });

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    //
    // This tests only marshalling speed
    // I think that libdbus and by that dbus-rs marshal values as they are added
    // so just creating the message is equivalent to create+marshal in rustbus?
    //
    let mixed_parts = make_mixed_message();
    run_marshal_benches("MarshalMixed", c, &mixed_parts);
    let big_array_parts = make_big_array_message();
    run_marshal_benches("MarshalBigArray", c, &big_array_parts);
    let big_str_array_parts = make_big_string_array_message();
    run_marshal_benches("MarshalBigStrArray", c, &big_str_array_parts);
    let mut group = c.benchmark_group("Sending");
    //
    // This tests the flow of:
    // 1. Connect to the session bus (which needs a hello message, which is implicit for dbus-rs)
    // 2. Create a signal message
    // 3. Send the signal to the bus
    //
    group.bench_function("send_rustbus", |b| {
        b.iter(|| {
            make_rustbus_message(&mixed_parts, true);
        })
    });
    group.bench_function("send_dbusrs", |b| {
        b.iter(|| {
            make_dbusrs_message(&mixed_parts, true);
        })
    });
    group.bench_function("send_dbusnative", |b| {
        b.iter(|| {
            make_dbus_native_message(&mixed_parts, true);
        })
    });
    group.bench_function("send_dbus_bytestream", |b| {
        b.iter(|| {
            make_dbus_bytestream_message(&mixed_parts, true);
        })
    });
    group.bench_function("send_dbus_pure", |b| {
        b.iter(|| {
            make_dbus_pure_message(&mixed_parts, true);
        })
    });
    // currently this does a lot of println so it is not a fair comparison
    //c.bench_function("send_zvariant", |b| {
    //    b.iter(|| {
    //        let mut con = zbus::Connection::new_session().unwrap();
    //        let body = make_zvariant_message();
    //        // this crate does not yet support signals so we send a call to a nonexistent service
    //        assert!(con
    //            .call_method(
    //                Some("io.killing.spark"),
    //                "/io/killing/spark",
    //                Some("io.killing.spark"),
    //                "TestSignal",
    //                Some(body),
    //            )
    //            .is_err());
    //    })
    //});

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
