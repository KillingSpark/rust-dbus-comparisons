use std::num::NonZeroU32;

use crate::MessageParts;
use async_rustbus::rustbus_core;

pub fn make_async_rustbus_message(parts: &MessageParts, send_it: bool) -> Option<Vec<u8>> {
    let mut msg = rustbus_core::message_builder::MessageBuilder::new()
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
        async_std::task::block_on(async {
            let conn = async_rustbus::RpcConn::session_conn(false).await?;
            conn.send_msg_wo_rsp(&msg).await.map(|_| ())
        })
        .unwrap();
        None
    } else {
        let mut buf = Vec::new();
        msg.marshal_header(NonZeroU32::new(1).unwrap(), &mut buf)
            .unwrap();
        Some(buf)
    }
}
