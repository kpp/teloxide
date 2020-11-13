// This file is auto generated by `cg` <https://github.com/teloxide/cg> (455146e).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, InlineKeyboardMarkup, Poll};

impl_payload! {
    /// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll with the final results is returned.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub StopPoll (StopPollSetters) => Poll {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
            pub chat_id: ChatId [into],
            /// Identifier of the message to edit
            pub message_id: i64,
        }
        optional {
            /// A JSON-serialized object for an [inline keyboard].
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            pub reply_markup: InlineKeyboardMarkup,
        }
    }
}
