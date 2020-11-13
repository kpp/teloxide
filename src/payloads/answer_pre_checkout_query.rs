// This file is auto generated by `cg` <https://github.com/teloxide/cg> (455146e).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::True;

impl_payload! {
    /// Once the user has confirmed their payment and shipping details, the Bot API sends the final confirmation in the form of an [`Update`] with the field pre\_checkout\_query. Use this method to respond to such pre-checkout queries. On success, True is returned. **Note:** The Bot API must receive an answer within 10 seconds after the pre-checkout query was sent.
    ///
    /// [`Update`]: crate::types::Update
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub AnswerPreCheckoutQuery (AnswerPreCheckoutQuerySetters) => True {
        required {
            /// Unique identifier for the query to be answered
            pub pre_checkout_query_id: String [into],
            /// Specify True if everything is alright (goods are available, etc.) and the bot is ready to proceed with the order. Use False if there are any problems.
            pub ok: bool,
        }
        optional {
            /// Required if ok is False. Error message in human readable form that explains the reason for failure to proceed with the checkout (e.g. "Sorry, somebody just bought the last of our amazing black T-shirts while you were busy filling out your payment details. Please choose a different color or garment!"). Telegram will display this message to the user.
            pub error_message: String [into],
        }
    }
}
