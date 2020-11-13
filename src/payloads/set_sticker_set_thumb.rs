// This file is auto generated by `cg` <https://github.com/teloxide/cg> (455146e).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{InputFile, True};

impl_payload! {
    /// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for animated sticker sets only. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetStickerSetThumb (SetStickerSetThumbSetters) => True {
        required {
            /// Name of the sticker set
            pub name: String [into],
            /// User identifier of sticker file owner
            pub user_id: u32,
        }
        optional {
            /// A **PNG** image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a **TGS** animation with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/animated_stickers#technical-requirements for animated sticker technical requirements. Pass a _file\_id_ as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. [More info on Sending Files »]. Animated sticker set thumbnail can't be uploaded via HTTP URL.
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub thumb: InputFile,
        }
    }
}
