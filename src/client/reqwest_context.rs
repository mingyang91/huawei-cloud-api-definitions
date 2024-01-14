use serde::Serialize;
use serde::de::DeserializeOwned;
use async_trait::async_trait;
use std::fmt::Debug;
use std::error::Error as StdError;
use std::sync::Arc;
use std::str::FromStr;
use reqwest::Method;
use crate::client::context::{Context, Profile};
use crate::signer::{SignableRequest, Signer};


#[derive(thiserror::Error, Debug)]
pub enum Error<T>
where T: Debug + StdError + Serialize + Send + Sync,
      T: DeserializeOwned {
	#[error("reqwest error")]
	Reqwest(reqwest::Error),
	#[error("codec error")]
	Codec(#[source] serde_json::Error),
	#[error("method error")]
	InvalidMethod(#[source] http::method::InvalidMethod),
	#[error("domain error")]
	Domain(#[from] T),
}

struct InnerLive {
	profile: Profile,
	signer: Signer,
	client: reqwest::Client,
}

pub struct Live {
	inner: Arc<InnerLive>,
}

impl Live {
	pub fn new(profile: Profile) -> Self {
		let signer = Signer::new(&profile.access_key_id, &profile.secret_access_key);
		Self {
			inner: Arc::new(InnerLive {
				profile,
				signer,
				client: reqwest::Client::new(),
			})
		}
	}
}

#[async_trait]
impl Context for Live {
	type Error<T> = Error<T>
	where T: Debug + StdError + Serialize + Send + Sync + 'static,
	      T: DeserializeOwned;

	async fn execute<Param, Resp, DomainError>(
		&self,
		method: &str,
		path: &str,
		req: Param) -> Result<Resp, Self::Error<DomainError>>
	where Param: Serialize + Debug + Send,
		Resp: DeserializeOwned + Debug + Send,
		DomainError: Debug + StdError + DeserializeOwned + Serialize + Send + Sync + 'static {
		// https://metastudio.cn-north-4.myhuaweicloud.com/v1/70b76xxxxxx34253880af501cdxxxxxx/smart-live-rooms
		let method = Method::from_str(method).map_err(Error::InvalidMethod)?;
		let url = format!("https://metastudio.{}.myhuaweicloud.com{}", self.inner.profile.region, path);
		let body = serde_json::to_vec(&req).map_err(Error::Codec)?;
		let req = self.inner.client.request(method, &url)
			.header("Content-Type", "application/json")
			.body(body)
			.build()
			.map_err(Error::Reqwest)?
			.sign_with(&self.inner.signer);
		let payload = self.inner.client.execute(req)
			.await
			.map_err(Error::Reqwest)?
			.bytes()
			.await
			.map_err(Error::Reqwest)?;

		let Ok(resp) = serde_json::from_slice(&payload[..]) else {
			let domain_error = serde_json::from_slice(&payload[..]).unwrap();
			return Err(Error::Domain(domain_error));
		};

		Ok(resp)
	}
}

