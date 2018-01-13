use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

/// Use this method to send answers to callback queries sent from inline keyboards. The answer will
/// be displayed to the user as a notification at the top of the chat screen or as an alert. On
/// success, True is returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct AnswerCallbackQuery<'s> {
    callback_query_id: CallbackQueryId,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'s, str>>,
    #[serde(skip_serializing_if = "Not::not")]
    show_alert: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'s, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
}

impl<'c, 's> Request for AnswerCallbackQuery<'s> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("answerCallbackQuery"), self)
    }
}

impl<'s> AnswerCallbackQuery<'s> {
    pub fn new(callback_query_id: CallbackQueryId) -> Self {
        AnswerCallbackQuery {
            callback_query_id: callback_query_id,
            text: None,
            show_alert: false,
            url: None,
            cache_time: None
        }
    }

    pub fn text<T>(&mut self, text: T) -> &mut Self
        where T: Into<Cow<'s, str>> {
        self.text = Some(text.into());
        self
    }

    pub fn show_alert(&mut self, show_alert: bool) -> &mut Self {
        self.show_alert = show_alert;
        self
    }

    pub fn url<T>(&mut self, url: T) -> &mut Self where T: Into<Cow<'s, str>> {
        self.url = Some(url.into());
        self
    }

    pub fn cache_time(&mut self, cache_time: Integer) -> &mut Self {
        self.cache_time = Some(cache_time);
        self
    }
}

/// Anwer Callback query.
pub trait CanAnswerCallbackQuery {
    fn text<'s, T>(&self, text: T) -> SendMessage<'s> where T: Into<Cow<'s, str>>;
}
