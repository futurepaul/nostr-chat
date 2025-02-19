use druid::{Env, EventCtx};

use crate::{
    data::{
        app_state::AppState,
        contact::Contact,
        conversation::{ChatMsg, Conversation, Msg},
    },
    delegate::{SELECT_CONV, SEND_MSG, START_CHAT},
};

pub struct ChatController {}

impl ChatController {
    pub fn click_send_msg(ctx: &mut EventCtx, data: &mut AppState, _env: &Env) {
        if let Some(conv) = data.selected_conv.clone() {
            let contact_pk = &conv.contact.pk;
            let new_msg = ChatMsg::new(contact_pk, &data.msg_to_send);
            data.push_conv_msg(
                &Msg::new(
                    &data
                        .user
                        .keys
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .public_key_as_str(),
                    &data.msg_to_send,
                ),
                contact_pk,
            );
            ctx.submit_command(SEND_MSG.with(new_msg));
            data.msg_to_send = "".into();
        }
    }
    pub fn click_start_chat(ctx: &mut EventCtx, data: &mut Contact, _env: &Env) {
        ctx.submit_command(START_CHAT.with(data.pk.clone()));
    }
    pub fn click_select_conv(ctx: &mut EventCtx, data: &mut Contact, _env: &Env) {
        ctx.submit_command(SELECT_CONV.with(data.pk.clone()));
    }
}
