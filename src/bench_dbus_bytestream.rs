use super::MessageParts;

pub fn make_dbus_bytestream_message(parts: &MessageParts, send_it: bool) -> Option<Vec<u8>> {
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
        None
    } else {
        let mut buf = Vec::new();
        use dbus_bytestream::marshal::Marshal;
        msg.dbus_encode(&mut buf);
        Some(buf)
    }
}
