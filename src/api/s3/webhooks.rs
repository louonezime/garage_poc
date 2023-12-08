use std::sync::Arc;

use garage_model::garage::Garage;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Serialize;

use crate::s3::error::*;

use garage_util::data::FixedBytes32;

// #[derive(Debug, Serialize, PartialEq)]
// enum PayloadType {
//     ObjectHook,
// 	BucketHook,
// 	MultiPartHook,
// 	WebsiteHook
// }

#[derive(Debug, Serialize, PartialEq)]
pub struct ObjectHook {
	pub hook_type: String,
	pub bucket: String,
	pub bucket_id: FixedBytes32,
	pub object: String,
	pub via: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct BucketHook {
	pub hook_type: String,
	pub bucket: String,
	pub bucket_id: FixedBytes32,
	pub via: String,
}

impl BucketHook {
	pub fn new(title: &str, bucket: String, id: FixedBytes32, via: String) -> Self {
		BucketHook {
			hook_type: title.to_string(),
			bucket: bucket,
			bucket_id: id,
			via: via,
		}
	}
}

impl ObjectHook {
	pub fn new(title: &str, bucket: String, id: FixedBytes32, obj: &String, via: String) -> Self {
		ObjectHook {
			hook_type: title.to_string(),
			bucket: bucket,
			bucket_id: id,
			object: obj.to_string(),
			via: via,
		}
	}
}

pub async fn call_hook<T: Serialize>(garage: Arc<Garage>, hook: T) -> Result<(), Error> {
	match garage.config.webhook_uri.as_ref() {
		Some(uri) => {
			let https = HttpsConnector::new();
			let client = Client::builder().build(https);
			let hook_body = serde_json::to_string(&hook).unwrap();

			println!("Connecting to {}", uri);
			let req = Request::builder()
				.method(Method::POST)
				.uri(uri)
				.header("Content-Type", "application/json")
				.body(Body::from(hook_body))?;

			// even if there is an error with the webhook, do not cause an error
			if let Err(result) = client.request(req).await {
				println!("Error processing webhook to {}: {}", uri, result);
				return Ok(());
			}

			Ok(())
		}
		None => Ok(()),
	}
}
