use super::MessageParts;

pub fn make_dbusrs_message(parts: &MessageParts, send_it: bool) -> Option<dbus::message::Message> {
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
        None
    } else {
        // no need to marshal, that happend while building
        Some(msg)
    }
}
