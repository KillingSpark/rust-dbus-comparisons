use super::MessageParts;
use std::convert::TryInto;

pub fn make_dbus_message_parser_message(
    parts: &MessageParts,
    send_it: bool,
) -> Option<bytes::BytesMut> {
    let mut signal = dbus_message_parser::message::Message::signal(
        parts.object.as_str().try_into().unwrap(),
        parts.interface.as_str().try_into().unwrap(),
        parts.member.as_str().try_into().unwrap(),
    );

    let dict_content = parts
        .dict
        .iter()
        .map(|(k, v)| {
            dbus_message_parser::value::Value::DictEntry(Box::new((
                dbus_message_parser::value::Value::String(k.clone()),
                dbus_message_parser::value::Value::Int32(*v),
            )))
        })
        .collect();
    let dict = dbus_message_parser::value::Value::Array(
        dbus_message_parser::value::Array::new(dict_content, "{si}".try_into().unwrap()).unwrap(),
    );

    let array = dbus_message_parser::value::Value::Array(
        dbus_message_parser::value::Array::new(
            parts
                .int_array
                .iter()
                .copied()
                .map(|i| dbus_message_parser::value::Value::Uint64(i))
                .collect(),
            "t".try_into().unwrap(),
        )
        .unwrap(),
    );

    let stringarray = dbus_message_parser::value::Value::Array(
        dbus_message_parser::value::Array::new(
            parts
                .string_array
                .iter()
                .cloned()
                .map(|i| dbus_message_parser::value::Value::String(i))
                .collect(),
            "s".try_into().unwrap(),
        )
        .unwrap(),
    );

    for _ in 0..parts.repeat {
        signal.add_value(dbus_message_parser::value::Value::String(
            parts.string1.clone(),
        ));
        signal.add_value(dbus_message_parser::value::Value::Uint64(parts.int1));
        signal.add_value(dbus_message_parser::value::Value::Struct(
            vec![
                dbus_message_parser::value::Value::Uint64(parts.int2),
                dbus_message_parser::value::Value::String(parts.string2.clone()),
            ]
            .try_into()
            .unwrap(),
        ));
        signal.add_value(dict.clone());
        signal.add_value(array.clone());
        signal.add_value(stringarray.clone());
    }
    if send_it {
        // no send implemented
        None
    } else {
        let buffer = signal.encode().unwrap();
        Some(buffer)
    }
}
