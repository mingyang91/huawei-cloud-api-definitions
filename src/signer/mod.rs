#[cfg(feature = "reqwest")]
pub mod reqwest_signable;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex;
use chrono;

const ALGORITHM: &str = "SDK-HMAC-SHA256";
const HEADER_X_DATE: &str = "X-Sdk-Date";
const HEADER_AUTHORIZATION: &str = "Authorization";
#[allow(dead_code)]
const HEADER_CONTENT_SHA256: &str = "x-sdk-content-sha256";

pub trait Clock {
  fn now() -> chrono::DateTime<chrono::Utc>;
}

pub struct Live {}
impl Clock for Live {
  fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
  }
}

pub struct Signer<C: Clock = Live> {
  key: String,
  secret: String,
  _phantom: std::marker::PhantomData<C>,
}

pub trait SignableRequest {
  type Headers: Iterator<Item = (String, String)>;
  type Body<'a>: AsRef<[u8]> where Self: 'a;
  fn host(&self) -> &str;
  fn method(&self) -> &str;
  fn path(&self) -> &str;
  fn headers(&self) -> Self::Headers;
  fn header(&self, key: &str) -> &str;
  fn set_header(&mut self, key: String, value: String);
  fn query(&self) -> &str;
  fn body(&self) -> Option<Self::Body<'_>>;
  fn sign_with<C: Clock>(mut self, signer: &Signer<C>) -> Self
  where Self: Sized {
    signer.sign(&mut self);
    self
  }
}

impl <C: Clock> Signer<C> {
  pub fn new(key: &str, secret: &str) -> Self {
    Signer {
      key: key.to_string(),
      secret: secret.to_string(),
      _phantom: std::marker::PhantomData,
    }
  }

  pub fn sign<Req>(&self, req: &mut Req) 
  where Req: SignableRequest {
    let mut header_time = req.header(HEADER_X_DATE).to_string();
    if header_time.is_empty() {
      header_time = C::now().format("%Y%m%dT%H%M%SZ").to_string();
      req.set_header(HEADER_X_DATE.to_string(), header_time.to_string());
    }

    if req.header("Host").is_empty() {
      req.set_header("Host".to_string(), req.host().to_string());
    }

    let mut signed_headers: Vec<(String, String)> = req.headers()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    signed_headers.sort_by(|a, b| a.0.cmp(&b.0));

    let canonical_request = self.canonical_request(req, &signed_headers);
    let string_to_sign = Self::string_to_sign(&canonical_request, &header_time);
    let signature = self.sign_string_to_sign(&string_to_sign);
    req.set_header(HEADER_AUTHORIZATION.to_string(), self.auth_header_value(&signature, &signed_headers));
  }

  fn canonical_headers(headers: &Vec<(String, String)>) -> String {
    let mut canonical_headers: Vec<String> = Vec::new();
    for (key, value) in headers {
      canonical_headers.push(format!("{}:{}\n", key.to_lowercase(), value.trim()));
    }
    canonical_headers.join("")
  }

  fn canonical_request<Req>(
    &self, 
    req: &mut Req,
    signed_headers: &Vec<(String, String)>,
  ) -> String 
  where Req: SignableRequest {
    let content_sha256 = 
      if let Some(has_body) = req.body() {
        Self::content_sha256(has_body.as_ref())
      } else {
        Self::content_sha256(&[])
      };
    let mut path = req.path().to_string();
    if !path.ends_with('/') {
      path.push('/');
    }
    let canonical_request = vec![
      req.method().to_string(),
      path,
      req.query().to_string(),
      Self::canonical_headers(signed_headers),
      signed_headers.iter().map(|(key, _)| key.to_lowercase()).collect::<Vec<_>>().join(";"),
      content_sha256
    ];
    canonical_request.join("\n")
  }

  fn content_sha256(body: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(body);
    hex::encode(hasher.finalize())
  }

  fn string_to_sign(canonical_request: &str, header_time: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(canonical_request.as_bytes());
    let hashed_canonical_request = hex::encode(hasher.finalize());
    let string_to_sign = vec![
      ALGORITHM.to_string(),
      header_time.to_string(),
      hashed_canonical_request
    ];
    string_to_sign.join("\n")
  }

  fn sign_string_to_sign(&self, string_to_sign: &str) -> String {
    self.hmac_sha256(self.secret.as_bytes(), string_to_sign.as_bytes())
  }

  fn auth_header_value(&self, signature: &str, signed_headers: &Vec<(String, String)>) -> String {
    let signed_headers = signed_headers.iter().map(|(key, _)| key.to_lowercase()).collect::<Vec<_>>().join(";");
    let auth_header_value = vec![
      format!("Access={}", self.key),
      format!("SignedHeaders={}", signed_headers),
      format!("Signature={}", signature)
    ];
    ALGORITHM.to_string() + " " + &auth_header_value.join(", ")
  }

  fn hmac_sha256(&self, key: &[u8], message: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)
      .expect("HMAC can take key of any size");
    mac.update(message);
    let result = mac.finalize();

    hex::encode(result.into_bytes())
  }
}


#[cfg(all(test, feature = "reqwest"))]
mod test {
  use serde::Serialize;
  use reqwest::Body;
  use crate::signer::{Signer, SignableRequest, Clock, Live, reqwest_signable::*};

  #[derive(Debug)]
  struct Empty {}
  impl Into<Body> for Empty {
    fn into(self) -> Body {
      vec![].into()
    }
  }

  struct Matrix {}
  impl Clock for Matrix {
    fn now() -> chrono::DateTime<chrono::Utc> {
      // 20120101T000000Z
      chrono::DateTime::parse_from_rfc3339("2012-01-01T00:00:00Z").unwrap().into()
    }
  }
  
  #[test]
  fn sign_get() {
    let client = reqwest::Client::new();
    let signer = Signer::<Matrix>::new("QTWAOYTTINDUT2QVKYUC", "MFyfvK41ba2giqM7**********KGpownRZlmVmHc");
    let request = client.get("http://endpoint.example.com/v1/77b6a44cba5143ab91d13ab9a8ff44fd/vpcs?limie=1")
      .header("Content-Type", "application/json")
      .body(Empty {})
      .build()
      .expect("request builder error")
      .sign_with(&signer);
    let authorization = "SDK-HMAC-SHA256 Access=QTWAOYTTINDUT2QVKYUC, SignedHeaders=content-type;host;x-sdk-date, Signature=486fb116518ebda891aa35f99750ab3fc8f1fa22315e3ba619b016a2261503f4";
    let headers = request.headers();
    assert_eq!(headers.get("Authorization").expect("should have"), authorization);
  }

  #[derive(Serialize)]
  struct Info {
    foo: String,
    bar: i32,
    baz: bool,
  }

  #[test]
  fn sign_post() {
    let client = reqwest::Client::new();
    let body = Info { foo: "foo".to_string(), bar: 114514, baz: true };
    let signer = Signer::<Matrix>::new("QTWAOYTTINDUT2QVKYUC", "MFyfvK41ba2giqM7**********KGpownRZlmVmHc");
    let request = client.post("http://endpoint.example.com/v1/77b6a44cba5143ab91d13ab9a8ff44fd/vpcs?limie=1")
      .header("Content-Type", "application/json")
      .body::<Vec<u8>>(serde_json::to_vec(&body).unwrap().into())
      .build()
      .expect("request builder error")
      .sign_with(&signer);

    let authorization = "SDK-HMAC-SHA256 Access=QTWAOYTTINDUT2QVKYUC, SignedHeaders=content-type;host;x-sdk-date, Signature=166ee46e44d98ca5b0bd55b6a9b69bce88a9938f73725fe7708d9789e34cceb5";
    let headers = request.headers();
    assert_eq!(headers.get("Authorization").expect("should have"), authorization);
  }

  #[tokio::test]
  async fn real_world_should_failed_with_fake_ak_sk() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    // GET https://metastudio.cn-east-3.myhuaweicloud.com/v1/6a6a1f8354f64dd9b9a614def7b59d83/digital-assets
    let signer: Signer<Live> = Signer::new("QTWAOYTTINDUT2QVKYUC", "MFyfvK41ba2giqM7**********KGpownRZlmVmHc");

    let request = client.get("https://metastudio.cn-east-3.myhuaweicloud.com/v1/6a6a1f8354f64dd9b9a614def7b59d83/digital-assets")
      .body(Empty {})
      .build()
      .expect("request builder error")
      .sign_with(&signer);

    if let Ok(resp) = client.execute(request).await {
      assert!(resp.status().is_client_error(), "should be denied with 4xx");
    } else {
      assert!(false, "should have response");
    }
    Ok(())
  }
}