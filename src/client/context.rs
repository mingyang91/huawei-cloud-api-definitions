use serde::Serialize;
use serde::de::DeserializeOwned;
use async_trait::async_trait;
use std::fmt::Debug;
use std::error::Error as StdError;

pub struct Profile {
	pub access_key_id: String,
	pub secret_access_key: String,
	pub region: String,
}

impl Profile {
	pub fn new(access_key_id: String, secret_access_key: String, region: String) -> Self {
		Self {
			access_key_id,
			secret_access_key,
			region,
		}
	}

	pub fn from_env() -> Self {
		Self {
			access_key_id: std::env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found"),
			secret_access_key: std::env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found"),
			region: std::env::var("REGION").expect("REGION not found"),
		}
	}
}

#[async_trait]
pub trait Context 
where Self: Send + Sync {
	type Error<T>: Debug + StdError + Send + Sync
		where T: Debug + StdError + Serialize + Send + Sync + 'static,
		      T: DeserializeOwned;

	async fn execute<Param, Resp, DomainError>(
		&self,
		method: &str,
		path: &str,
		req: Param) -> Result<Resp, Self::Error<DomainError>>
	where Param: Serialize + Debug + Send,
		Resp: DeserializeOwned + Debug + Send,
		DomainError: Debug + StdError + DeserializeOwned + Serialize + Send + Sync + 'static;
}
