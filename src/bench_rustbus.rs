use crate::MessageParts;
use rustbus::wire::marshal::marshal;

pub fn make_rustbus_message(parts: &MessageParts, send_it: bool) -> Option<Vec<u8>> {
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
        msg.body.push_param(parts.int_array.as_slice()).unwrap();
        msg.body.push_param(parts.string_array.as_slice()).unwrap();
    }

    if send_it {
        let mut rustbus_con = rustbus::connection::rpc_conn::RpcConn::session_conn(
            rustbus::connection::Timeout::Infinite,
        )
        .unwrap();
        let serial = rustbus_con
            .send_message(&mut rustbus::standard_messages::hello())
            .unwrap()
            .write_all()
            .unwrap();
        let _name_resp = rustbus_con
            .wait_response(serial, rustbus::connection::Timeout::Infinite)
            .unwrap();
        let _serial = rustbus_con
            .send_message(&mut msg)
            .unwrap()
            .write_all()
            .unwrap();
        None
    } else {
        let mut buf = Vec::new();
        marshal(&msg, 1, &mut buf).unwrap();
        Some(buf)
    }
}
