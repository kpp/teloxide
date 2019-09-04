use crate::core::network;
use crate::core::requests::{
    Request, RequestFuture, RequestContext, ResponseResult,
};
use crate::core::types::User;

#[derive(Debug)]
pub struct GetMe<'a> {
    info: RequestContext<'a>,
}

impl<'a> Request<'a> for GetMe<'a> {
    type ReturnValue = User;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request(self.info.client, self.info.token, "getMe", None)
                .await
        })
    }
}

impl<'a> GetMe<'a> {
    pub(crate) fn new(info: RequestContext<'a>) -> Self {
        GetMe { info }
    }
}
