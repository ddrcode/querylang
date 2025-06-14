use std::borrow::Cow;

use serde::Serialize;

#[derive(Serialize)]
pub struct StatusMsg {
    status: Cow<'static, str>,
    message: Cow<'static, str>,
}

impl StatusMsg {
    pub fn new(status: Cow<'static, str>, message: Cow<'static, str>) -> Self {
        Self { status, message }
    }

    pub fn from_str(status: &'static str, message: &'static str) -> Self {
        StatusMsg::new(Cow::Borrowed(status), Cow::Borrowed(message))
    }

    pub fn from_string(status: String, message: String) -> Self {
        StatusMsg::new(Cow::Owned(status), Cow::Owned(message))
    }

    pub fn ok(message: String) -> Self {
        StatusMsg::new(Cow::Borrowed("ok"), Cow::Owned(message))
    }

    pub fn error(message: String) -> Self {
        StatusMsg::new(Cow::Borrowed("error"), Cow::Owned(message))
    }
}

