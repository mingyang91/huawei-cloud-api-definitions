use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::env;
use tokio::io::AsyncWriteExt;
use tokio::{fs::{read_dir, File, OpenOptions}, io::AsyncReadExt};

const START_MARK: &str = "# --- GENERATED FEATURES LIST START ---";
const END_MARK: &str = "# --- GENERATED FEATURES LIST END ---";

#[derive(thiserror::Error, Debug)]
enum Error {
	#[error("IO error")]
	FileSystem(#[from] tokio::io::Error),
	#[error("Parse error")]
	Parse(String),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	let mut cargo_toml = File::open("Cargo.toml").await?;
	// read the entire file
	let mut cargo_toml_content = String::new();
	cargo_toml.read_to_string(&mut cargo_toml_content).await?;
	// find the start and end marks
	let start = cargo_toml_content.find(START_MARK).ok_or(Error::Parse("start mark notfound".to_string()))?;
	let end = cargo_toml_content.find(END_MARK).ok_or(Error::Parse("end mark notfound".to_string()))?;

	let features: Vec<String> = get_features().await?;

	let before_begin = &cargo_toml_content[..start + START_MARK.len()];
	let after_end = &cargo_toml_content[end..];


	let new_cargo_toml_content = format!("{}\n{}\n{}\n", before_begin, features.join("\n"), after_end);

	let mut cargo_toml = OpenOptions::new()
		.write(true)
		.truncate(true)
		.open("Cargo.toml")
		.await?;

	cargo_toml
		.write_all(new_cargo_toml_content.as_bytes())
		.await?;

	println!("generated {} features", features.len());
	Ok(())
}

async fn get_features() -> Result<Vec<String>, Error> {
	let schema_dir = env::var("OUT_DIR").map(|x| x.to_string()).map_err(|e| Error::Parse(e.to_string()))?;
	let mut dir = read_dir(schema_dir).await?;
	let mut product_map = HashMap::new();
	while let Some(entry) = dir.next_entry().await? {
		let path = entry.path();
		if !path.is_dir() {
			continue;
		}

		let product_name = path
			.file_name()
			.map(|x| x.to_string_lossy().to_string())
			.ok_or(Error::Parse("illegal product name".to_string()))?;

		let mut product_dir = read_dir(path).await?;
		let mut apis = Vec::new();
		while let Some(api_entry) = product_dir.next_entry().await? {
			let api_path = api_entry.path();
			if !api_path.is_file() {
				continue;
			}

			if api_path.extension() != Some(OsStr::new("rs")) {
				continue;
			}

			let api_name = api_path
				.file_stem()
				.map(|x| x.to_string_lossy().to_string())
				.ok_or(Error::Parse("illegal api file name".to_string()))?;
			if api_name == "mod" {
				continue;
			}
			apis.push(api_name);
		}
		product_map.insert(product_name, apis);
	}

	let full = product_map
		.keys()
		.map(|k| format!("\"{}\"", k))
		.collect::<Vec<String>>();

	let mut gen = product_map
		.into_iter()
		.flat_map(|(product_name, apis)| {
			let mut api_features = apis
				.iter()
				.map(|api| format!("{}_{} = []", product_name, api))
				.collect::<Vec<String>>();

			let api_names = apis
				.into_iter()
				.map(|api| format!("\"{}_{}\"", product_name, api))
				.collect::<Vec<String>>()
				.join(", ");
			let product_feature = format!("{} = [{}]", product_name, api_names);
			api_features.push(product_feature);
			api_features
		})
		.collect::<HashSet<String>>()
		.into_iter()
		.collect::<Vec<String>>();

	gen.push(format!("full = [{}]", full.join(", ")));
	gen.sort();

	Ok(gen)
}