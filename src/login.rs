use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// A struct that represents a login request.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Request<'a> {
    /// The users email address.
    email_address: Cow<'a, str>,

    /// The users password.
    password: Cow<'a, str>,

    /// Unique device id to track the users device, as this is normally called
    /// from a mobile app I assume this is generated per installation.
    /// For our usage it can be anything you want.
    device_id: Cow<'a, str>,
}

/// A struct that represents the response from the login call.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Response<'a> {
    /// The bearer token used for authentication.
    token: Cow<'a, str>,
}
