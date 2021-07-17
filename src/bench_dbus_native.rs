use super::MessageParts;

pub fn make_dbus_native_message(parts: &MessageParts, send_it: bool) -> Option<Vec<u8>> {
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
        writer.write_all(&buf).unwrap();
        writer.flush().unwrap();
        None
    } else {
        let buf = msg
            .marshal(std::num::NonZeroU32::new(1u32).unwrap(), false)
            .unwrap();
        Some(buf)
    }
}
