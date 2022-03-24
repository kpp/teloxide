use derive_more::{Display, From};
use serde::{Deserialize, Serialize};

use crate::types::ChatId;

/// A unique identifier for the target chat or username of the target channel
/// (in the format `@channelusername`).
#[derive(Clone, PartialEq, Eq, Hash)]
#[derive(Debug, Display, From)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Recipient {
    /// A chat identifier.
    #[display(fmt = "{}", _0)]
    Id(ChatId),

    /// A channel username (in the format @channelusername).
    #[display(fmt = "{}", _0)]
    ChannelUsername(String),
}

impl Recipient {
    #[allow(unused)]
    pub(crate) fn is_channel(&self) -> bool {
        match self {
            Recipient::Id(id) => id.is_channel(),
            Recipient::ChannelUsername(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_id_id_serialization() {
        let expected_json = String::from(r#"123456"#);
        let actual_json = serde_json::to_string(&Recipient::Id(ChatId(123_456))).unwrap();

        assert_eq!(expected_json, actual_json)
    }

    #[test]
    fn chat_id_channel_username_serialization() {
        let expected_json = String::from(r#""@username""#);
        let actual_json =
            serde_json::to_string(&Recipient::ChannelUsername(String::from("@username"))).unwrap();

        assert_eq!(expected_json, actual_json)
    }
}
