use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dbus_benches::make_dbus_bytestream_message;
use dbus_benches::make_dbus_message_parser_message;
use dbus_benches::make_dbus_native_message;
use dbus_benches::make_dbus_pure_message;
use dbus_benches::make_dbusrs_message;
use dbus_benches::make_rustbus_message;
use dbus_benches::make_zvariant_derive_message;
use dbus_benches::make_zvariant_message;
use dbus_benches::MessageParts;
use dbus_benches::ZVField;
use dbus_benches::ZVStruct;

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
            black_box(make_rustbus_message(parts, false));
        })
    });
    group.bench_function("marshal_dbusrs", |b| {
        b.iter(|| {
            black_box(make_dbusrs_message(parts, false));
        })
    });
    group.bench_function("marshal_dbus_native", |b| {
        b.iter(|| {
            black_box(make_dbus_native_message(parts, false));
        })
    });
    group.bench_function("marshal_dbus_bytestream", |b| {
        b.iter(|| {
            black_box(make_dbus_bytestream_message(parts, false));
        })
    });
    group.bench_function("marshal_dbus_msg_parser", |b| {
        b.iter(|| {
            black_box(make_dbus_message_parser_message(parts, false));
        })
    });
    group.bench_function("marshal_dbus_pure", |b| {
        b.iter(|| {
            black_box(make_dbus_pure_message(parts, false));
        })
    });
    group.bench_function("marshal_zvariant", |b| {
        b.iter(|| {
            black_box(make_zvariant_message(parts, false));
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
            black_box(make_zvariant_derive_message(parts, &elements, false));
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
            black_box(make_rustbus_message(&mixed_parts, true));
        })
    });
    group.bench_function("send_dbusrs", |b| {
        b.iter(|| {
            black_box(make_dbusrs_message(&mixed_parts, true));
        })
    });
    group.bench_function("send_dbusnative", |b| {
        b.iter(|| {
            black_box(make_dbus_native_message(&mixed_parts, true));
        })
    });
    group.bench_function("send_dbus_bytestream", |b| {
        b.iter(|| {
            black_box(make_dbus_bytestream_message(&mixed_parts, true));
        })
    });
    group.bench_function("send_dbus_pure", |b| {
        b.iter(|| {
            black_box(make_dbus_pure_message(&mixed_parts, true));
        })
    });
    group.bench_function("send_zvariant", |b| {
        b.iter(|| {
            black_box(make_zvariant_message(&&mixed_parts, true));
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
