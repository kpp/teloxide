// This file is auto generated by `cg` <https://github.com/teloxide/cg> (455146e).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, Message, ReplyMarkup};

impl_payload! {
    /// Use this method to send information about a venue. On success, the sent [`Message`] is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Clone, Serialize)]
    pub SendVenue (SendVenueSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// Latitude of new location
            pub latitude: f64,
            /// Longitude of new location
            pub longitude: f64,
            /// Name of the venue
            pub title: String [into],
            /// Address of the venue
            pub address: String [into],
        }
        optional {
            /// Foursquare identifier of the venue
            pub foursquare_id: String [into],
            /// Foursquare type of the venue, if known. (For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/icecream”.)
            pub foursquare_type: String [into],
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i64,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
