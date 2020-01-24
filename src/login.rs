//! Builder and struct for representing login request and response.
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

/// A builder to help with the creation of a login Request.
/// # Examples
///
/// ```
/// use sure_petcare::login;
/// let mut builder = login::RequestBuilder::new();
/// builder.with_email_address("email@example.com");
/// builder.with_password("qwerty123");
/// builder.with_device_id("xxx-xxx-xxx-xxx");
/// let item = builder.build();
/// let serialized = serde_json::to_string(&item).unwrap();
/// assert_eq!(
///     "{\"email_address\":\"email@example.com\",\"password\":\"qwerty123\",\"device_id\":\"xxx-xxx-xxx-xxx\"}".to_string(),
///     serialized
/// );
/// ```
///
/// ```
/// use sure_petcare::login;
/// let item = login::RequestBuilder::new()
///     .with_email_address("email@example.com")
///     .with_password("qwerty123")
///     .with_device_id("xxx-xxx-xxx-xxx")
///     .build();
/// let serialized = serde_json::to_string(&item).unwrap();
/// assert_eq!(
///     "{\"email_address\":\"email@example.com\",\"password\":\"qwerty123\",\"device_id\":\"xxx-xxx-xxx-xxx\"}".to_string(),
///     serialized
/// );
/// ```
#[derive(Clone, Debug, Default)]
pub struct RequestBuilder<'a> {
    email_address: Cow<'a, str>,
    password: Cow<'a, str>,
    device_id: Cow<'a, str>,
}

impl<'a> RequestBuilder<'a> {
    /// Create a new empty builder.
    pub fn new() -> Self {
        RequestBuilder::default()
    }

    /// Sets the email address.
    pub fn with_email_address<T: Into<Cow<'a, str>>>(&mut self, email_address: T) -> &mut Self {
        self.email_address = email_address.into();
        self
    }

    /// Sets the password.
    pub fn with_password<T: Into<Cow<'a, str>>>(&mut self, password: T) -> &mut Self {
        self.password = password.into();
        self
    }

    /// Sets the device id.
    pub fn with_device_id<T: Into<Cow<'a, str>>>(&mut self, device_id: T) -> &mut Self {
        self.device_id = device_id.into();
        self
    }

    /// Builds the request.
    pub fn build(&self) -> Request<'a> {
        Request {
            email_address: self.email_address.to_owned(),
            password: self.password.to_owned(),
            device_id: self.device_id.to_owned(),
        }
    }
}

/// A struct that represents the response from the login call.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Response<'a> {
    /// The bearer token used for authentication.
    token: Cow<'a, str>,
}

impl<'a> Response<'a> {
    pub fn access_token(self) -> Cow<'a, str> {
        self.token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_build_chain() {
        let item = RequestBuilder::new()
            .with_email_address("email@example.com")
            .with_password("qwerty123")
            .with_device_id("xxx-xxx-xxx-xxx")
            .build();
        let serialized = serde_json::to_string(&item).unwrap();
        assert_eq!(
         "{\"email_address\":\"email@example.com\",\"password\":\"qwerty123\",\"device_id\":\"xxx-xxx-xxx-xxx\"}".to_string(),
         serialized
        );
    }

    #[test]
    fn should_build_parts() {
        let mut builder = RequestBuilder::new();
        builder.with_email_address("email@example.com");
        builder.with_password("qwerty123");
        builder.with_device_id("xxx-xxx-xxx-xxx");
        let item = builder.build();
        let serialized = serde_json::to_string(&item).unwrap();
        assert_eq!(
            "{\"email_address\":\"email@example.com\",\"password\":\"qwerty123\",\"device_id\":\"xxx-xxx-xxx-xxx\"}".to_string(),
            serialized
        );
    }
}
