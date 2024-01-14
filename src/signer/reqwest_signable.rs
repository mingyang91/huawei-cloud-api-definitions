use std::str::FromStr;
use reqwest::{header::HeaderName, header::HeaderValue, Request};
use crate::signer::SignableRequest;

impl SignableRequest for Request {
  type Headers = std::vec::IntoIter<(String, String)>;
  type Body<'a> = &'a [u8];
  fn host(&self) -> &str {
    self.url().host_str().expect("")
  }

  fn method(&self) -> &str {
    self.method().as_str()
  }

  fn path(&self) -> &str {
    self.url().path()
  }

  fn header(&self, key: &str) -> &str {
    let Some(value) = self.headers().get(key) else {
      return "";
    };

    value.to_str().unwrap_or("")
  }

  fn headers(&self) -> Self::Headers {
    self.headers()
      .into_iter()
      .map(|(key, value)| (key.to_string(), value.to_str().expect("undecodeable header").to_string()))
      .collect::<Vec<_>>()
      .into_iter()
  }

  fn set_header(&mut self, key: String, value: String) {
    self.headers_mut()
      .insert(
        HeaderName::from_str(&key).expect("set header error"),
        HeaderValue::from_str(&value).expect("set header error")
      );
  }

  fn query(&self) -> &str {
    self.url().query().unwrap_or("")
  }

  fn body(&self) -> Option<Self::Body<'_>> {
    self.body()?.as_bytes()
  }
}
