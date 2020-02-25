use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rustbus::message::Container;
use rustbus::message::DictMap;
use rustbus::message::Param;
use rustbus::wire::marshal::marshal;

fn marsh(msg: &rustbus::Message, buf: &mut Vec<u8>) {
    marshal(msg, rustbus::message::ByteOrder::LittleEndian, &[], buf).unwrap();
}

fn make_rustbus_message() -> rustbus::Message {
    let mut params: Vec<Param> = Vec::new();

    let mut dict = DictMap::new();
    dict.insert("A".to_owned().into(), 1234567i32.into());
    dict.insert("B".to_owned().into(), 1234567i32.into());
    dict.insert("C".to_owned().into(), 1234567i32.into());
    dict.insert("D".to_owned().into(), 1234567i32.into());
    dict.insert("E".to_owned().into(), 1234567i32.into());

    use std::convert::TryFrom;
    let dict: Param = Container::try_from(dict).unwrap().into();

    let array: Param = Container::try_from(vec![
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
        0xFFFFFFFFFFFFFFFFu64.into(),
    ])
    .unwrap()
    .into();

    for _ in 0..10 {
        params.push("TesttestTesttest".to_owned().into());
        params.push(0xFFFFFFFFFFFFFFFFu64.into());
        params.push(
            Container::Struct(vec![
                0xFFFFFFFFFFFFFFFFu64.into(),
                "TesttestTesttest".to_owned().into(),
            ])
            .into(),
        );
        params.push(dict.clone());
        params.push(array.clone());
    }

    let mut msg = rustbus::message_builder::MessageBuilder::new()
        .signal(
            "io.killing.spark".into(),
            "TestSignal".into(),
            "/io/killing/spark".into(),
        )
        .with_params(params)
        .build();
    msg.serial = Some(1);
    msg
}

fn make_dbusrs_message() -> dbus::Message {
    let mut msg = dbus::message::Message::signal(
        &dbus::strings::Path::from("/io/killing/spark"),
        &dbus::strings::Interface::from("io.killing.spark"),
        &dbus::strings::Member::from("TestSignal"),
    );

    let dict = dbus::arg::Dict::new(
        vec![
            ("A".to_owned(), 1234567i32),
            ("B".to_owned(), 1234567i32),
            ("C".to_owned(), 1234567i32),
            ("D".to_owned(), 1234567i32),
            ("E".to_owned(), 1234567i32),
        ]
        .into_iter(),
    );

    let array = dbus::arg::Array::new(vec![
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
    ]);

    for _ in 0..10 {
        msg = msg.append3(
            "TesttestTesttest",
            0xFFFFFFFFFFFFFFFFu64,
            (0xFFFFFFFFFFFFFFFFu64, "TesttestTesttest"),
        );
        msg = msg.append2(dict.clone(), array.clone());
    }
    msg
}

fn make_zvariant_message() -> zvariant::Structure {
    let mut body = zvariant::Structure::new();
    let mut struct_field = zvariant::Structure::new();

    let mut map = std::collections::HashMap::new();
    map.insert("A".to_owned(), 1234567i32);
    map.insert("B".to_owned(), 1234567i32);
    map.insert("C".to_owned(), 1234567i32);
    map.insert("D".to_owned(), 1234567i32);
    map.insert("E".to_owned(), 1234567i32);

    let dict = zvariant::Dict::from(map);
    use std::convert::TryFrom;
    let dict_arr = zvariant::Array::try_from(dict).unwrap();

    let array = zvariant::Array::from(vec![
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
        0xFFFFFFFFFFFFFFFFu64,
    ]);

    struct_field = struct_field.add_field(0xFFFFFFFFFFFFFFFFu64);
    struct_field = struct_field.add_field("TesttestTesttest");

    for _ in 0..10 {
        body = body.add_field("TesttestTesttest");
        body = body.add_field(0xFFFFFFFFFFFFFFFFu64);
        body = body.add_field(struct_field.clone());
        // TODO is this really the most efficient way?
        body = body.add_field(dict_arr.clone());
        body = body.add_field(array.clone());
    }

    body
}

fn criterion_benchmark(c: &mut Criterion) {
    //
    // This tests only marshalling speed
    // I think that libdbus and by that dbus-rs marshal values as they are added
    // so just creating the message is equivalent to create+marshal in rustbus?
    //
    c.bench_function("marshal_rustbus", |b| {
        b.iter(|| {
            let msg = make_rustbus_message();
            let mut buf = Vec::new();
            buf.clear();
            marsh(black_box(&msg), &mut buf);
            return msg;
        })
    });
    c.bench_function("marshal_dbusrs", |b| {
        b.iter(|| {
            let msg = make_dbusrs_message();
            return msg;
        })
    });
    c.bench_function("marshal_zvariant", |b| {
        b.iter(|| {
            let body = make_zvariant_message();
            let msg = zbus::Message::method(
                Some("io.killing.spark"),
                "/io/killing/spark",
                Some("io.killing.spark"),
                "TestSignal",
                Some(body),
            )
            .unwrap();
            return msg;
        })
    });

    //
    // This tests the flow of:
    // 1. Connect to the session bus (which needs a hello message, which is implicit for dbus-rs)
    // 2. Create a signal message
    // 3. Send the signal to the bus
    //
    c.bench_function("send_rustbus", |b| {
        b.iter(|| {
            let mut rustbus_con = rustbus::client_conn::Conn::connect_to_bus(
                rustbus::get_session_bus_path().unwrap(),
                false,
            )
            .unwrap();
            let msg = make_rustbus_message();
            rustbus_con
                .send_message(rustbus::standard_messages::hello(), None)
                .unwrap();
            rustbus_con.send_message(msg, None).unwrap();
        })
    });
    c.bench_function("send_dbusrs", |b| {
        b.iter(|| {
            use dbus::channel::Sender;
            let conn = dbus::blocking::Connection::new_session().unwrap();
            let msg = make_dbusrs_message();
            conn.send(msg).unwrap();
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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
