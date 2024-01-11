use futures_util::{FutureExt, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use tokio::fs::{create_dir_all, read_dir, File};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use toml::Value as TomlValue;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Env error")]
    Env(#[from] std::env::VarError),
    #[error("Other")]
    Other(String),
		#[error("Parse error")]
		TomlDe(#[from] toml::de::Error),
		#[error("Render error")]
		TomlSer(#[from] toml::ser::Error),
    #[error("Parse error")]
    Parse(String),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_id = env::var("REGION_ID").unwrap_or("cn-north-4".to_string());
    let client = Client::new();
    let list_products_resp = list_products(&client).await?;
    let out_dir = env::var("OUT_DIR")?;
    let gen_dir = Path::new(&out_dir);
    create_dir_all(&gen_dir).await?;

    let mut product_names = Vec::new();
    for group in list_products_resp.groups.iter() {
        for product in group.products.iter() {
            let productshort = &product.productshort;
            let normalize_productshort = normalize_identifier(productshort);
            let list_apis_resp = list_apis(&client, productshort).await;
            let list_apis_resp = match list_apis_resp {
                Ok(list_apis_resp) => list_apis_resp,
                Err(e) => {
                    println!("Skip {} because of error {}", productshort, e);
                    continue;
                }
            };
            let enabled = list_apis_resp
                .api_basic_infos
                .iter()
                .collect::<Vec<&ApiBasicInfo>>();
            if enabled.len() == 0 {
                println!("Skip {} because of no apis", productshort);
                continue;
            }

            let gen_dir = gen_dir.join(&normalize_productshort).join("src");
            create_dir_all(&gen_dir).await?;
            product_names.push(normalize_productshort.clone());

            let api_stream = tokio_stream::iter(enabled);
            let api_names = api_stream
                .flat_map_unordered(1, |api_basic_info| {
                    let gen_dir = gen_dir.clone();
                    let productshort = productshort.clone();
                    let client = client.clone();
                    let region_id = region_id.clone();
                    let fut = async move {
                        let api_detail_resp = get_api_detail(
                            &client,
                            &productshort,
                            &api_basic_info.name,
                            &region_id,
                        )
                        .await;
                        match api_detail_resp {
                            Ok(api_detail) => {
                                let path = format!(
                                    "{}/{}.json",
                                    (&gen_dir).display(),
                                    api_basic_info.name
                                );
                                let url =
                                    api_detail_url(&productshort, &api_basic_info.name, &region_id);
                                typify_gen_definition(
                                    path,
                                    &api_basic_info.name,
                                    &url,
                                    api_detail.to_string().as_bytes(),
                                )
                                .await
                                .ok()?;
                                Some(api_basic_info.name.clone())
                            }
                            Err(e) => {
                                println!(
                                    "Skip {}-{} because of error {}",
                                    productshort, api_basic_info.name, e
                                );
                                None
                            }
                        }
                    };
                    let fut = Box::pin(fut);
                    fut.into_stream()
                })
                .filter_map(|x| async { x })
                .collect::<Vec<String>>()
                .await;
            write_file(
                format!("{}/lib.rs", gen_dir.display()),
                feature_mod_gen( &api_names).as_bytes(),
            )
            .await?;
						generate_cargo_toml(
							gen_dir.clone().parent().ok_or(Error::Other("no parent".to_string()))?,
							&productshort,
							&api_names,
						).await?;
        }
    }

    println!("cargo:rustc-env=GENERATED_ENV={}", gen_dir.display());

    Ok(())
}

async fn write_file<P: AsRef<Path>>(path: P, content: &[u8]) -> Result<(), Error> {
    let parent = path
        .as_ref()
        .parent()
        .ok_or(Error::Other("no parent".to_string()))?;
    create_dir_all(parent).await?;
    let mut file = match tokio::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.as_ref())
        .await
    {
        Err(_e) => tokio::fs::File::create(path.as_ref()).await?,
        Ok(file) => file,
    };
    file.write_all(content).await?;
    Ok(())
}

async fn typify_gen_definition<P: AsRef<Path>>(
    path: P,
    name: &str,
    reference: &str,
    content: &[u8],
) -> Result<(), Error> {
    let schema_file = path.as_ref();
    write_file(schema_file, content).await?;

    let companie_rs = path
        .as_ref()
        .parent()
        .ok_or(Error::Other("no parent".to_string()))?
        .join(format!("{}.rs", name));
    write_file(
        companie_rs,
        companie_rs_gen(path.as_ref(), reference).as_bytes(),
    )
    .await?;
    Ok(())
}

fn companie_rs_gen<P: AsRef<Path>>(schema: P, reference: &str) -> String {
    format!(
        r#"
		// This file is generated by hw-cloud-schema/build.rs
		// Definition: {}
		#![allow(dead_code)]
		#![allow(unused_imports)]
		#![allow(non_snake_case)]
		use std::fmt::Debug;
		use serde::{{Serialize, Deserialize}};
		use typify::import_types;
		import_types!(schema="{}", struct_builder = true);
	"#,
        reference,
        schema.as_ref().display()
    )
}

fn feature_mod_gen(name_list: &[String]) -> String {
    let mut prefix = String::new();
    let mut mod_gen = String::new();
    for name in name_list {
        mod_gen.push_str(&format!("#[cfg(feature = \"{}\")]\n", name));
        mod_gen.push_str(&format!("pub mod {};\n", name));
        println!("cargo:rustc-cfg={}", name);
    }
    mod_gen
}

#[derive(Debug, Serialize, Deserialize)]
struct ListProductsResponse {
    groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    id: String,
    name: String,
    products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    // api_count: u32,
    // attributive_product: String,
    // description: String,
    // has_data: bool,
    // icon: String,
    // is_global: bool,
    // is_recommend: bool,
    // link: String,
    name: String,
    productshort: String,
}

// https://console.huaweicloud.com/apiexplorer/new/v5/products
async fn list_products(client: &Client) -> Result<ListProductsResponse, reqwest::Error> {
    let products = client
        .get("https://console.huaweicloud.com/apiexplorer/new/v5/products")
        .send()
        .await?
        .json::<ListProductsResponse>()
        .await?;

    Ok(products)
}

#[derive(Debug, Serialize, Deserialize)]
struct ListApisResponse {
    count: u32,
    api_basic_infos: Vec<ApiBasicInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiBasicInfo {
    alias_name: Option<String>,
    id: String,
    info_version: Option<String>,
    method: String,
    name: String,
    product_short: String,
    summary: Option<String>,
    tags: Option<String>,
}

// https://console.huaweicloud.com/apiexplorer/new/v3/apis?offset=0&limit=100&product_short=MetaStudio
async fn list_apis(
    client: &Client,
    product_short: &str,
) -> Result<ListApisResponse, reqwest::Error> {
    for round in 0..10 {
        let url = format!("https://console.huaweicloud.com/apiexplorer/new/v3/apis?offset=0&limit=100&product_short={}", product_short);
        let apis = client.get(url).send().await?.json::<Value>().await?;

        if let Ok(err) = serde_json::from_value::<ErrorResponse>(apis.clone()) {
            if err.error_code == "APIGW.0308" {
                println!("APIGW.0308, sleep 1s and retry");
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
            println!(
                "failed {} x {} because of error {}",
                product_short, round, err.error_msg
            );
            continue;
        }

        return serde_json::from_value::<ListApisResponse>(apis).or_else(|e| {
            println!(
                "failed {} x {} because of error {}",
                product_short, round, e
            );
            Ok(ListApisResponse {
                count: 0,
                api_basic_infos: Vec::new(),
            })
        });
    }

    panic!("retry too many times");
}

fn normalize_identifier(name: &str) -> String {
    let mut name = name.to_string();
    name = name.replace("-", "_").replace(" ", "_");
    name
}

// {"error_msg":"The throttling threshold has been reached: policy ip over ratelimit,limit:5,time:5 second","error_code":"APIGW.0308","request_id":"c50619de20b0e70590f57a99dbed044f"}
// {"error_code":"APIEXPLORER.1055","error_msg":"api信息为空"}
#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error_msg: String,
    error_code: String,
}

// https://console.huaweicloud.com/apiexplorer/new/v4/apis/detail?product_short=MetaStudio&name=CreateSmartLiveRoom&region_id=cn-north-4
async fn get_api_detail(
    client: &Client,
    product_short: &str,
    api_name: &str,
    region_id: &str,
) -> Result<Value, reqwest::Error> {
    for round in 0..10 {
        let url = api_detail_url(product_short, api_name, region_id);
        let api_detail = client.get(url).send().await?.json::<Value>().await?;

        if let Ok(err) = serde_json::from_value::<ErrorResponse>(api_detail.clone()) {
            if err.error_code == "APIGW.0308" {
                println!(
                    "failed {}-{} x {} APIGW.0308, sleep 1s and retry",
                    product_short, api_name, round
                );
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                continue;
            }
            return Ok(serde_json::json!("{}"));
        }

        return Ok(api_detail);
    }
    panic!("retry too many times");
}

fn api_detail_url(product_short: &str, api_name: &str, region_id: &str) -> String {
    format!("https://console.huaweicloud.com/apiexplorer/new/v4/apis/detail?product_short={}&name={}&region_id={}", product_short, api_name, region_id)
}

#[allow(dead_code)]
async fn get_features<P: AsRef<Path>>(schema_dir: P) -> Result<Vec<String>, Error> {
    // let schema_dir = env::var("OUT_DIR").map(|x| x.to_string()).map_err(|e| Error::Parse(e.to_string()))?;
    let mut dir = read_dir(schema_dir.as_ref()).await?;

    let mut apis = Vec::new();
    while let Some(api_entry) = dir.next_entry().await? {
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

    let mut api_features = apis
        .iter()
        .map(|api| format!("\"{}\" = []", api))
        .collect::<Vec<String>>();

    let api_names = apis
        .into_iter()
        .map(|api| format!("\"{}\"", api))
        .collect::<Vec<String>>()
        .join(", ");

    api_features.push(format!("full = [{}]", api_names));
    api_features.sort();

    Ok(api_features)
}

async fn parse_cargo_toml() -> Result<TomlValue, Error> {
	let mut cargo_toml = File::open("Cargo.template.toml").await?;
	let mut content = String::new();
	cargo_toml.read_to_string(&mut content).await?;
	let cargo_toml = content.parse::<TomlValue>()?;
	Ok(cargo_toml)
}

async fn render_cargo_toml(name: &str, features: &[String]) -> Result<String, Error> {
	let cargo_toml = parse_cargo_toml().await?;
	let mut cargo_toml = cargo_toml
		.as_table()
		.ok_or(Error::Parse("Failed to parse Cargo.toml".to_string()))?
		.clone();

	// update package.name with base and product name
	let pkg_name = cargo_toml
		.get_mut("package")
		.ok_or(Error::Parse("Failed to find 'package' in Cargo.toml".to_string()))?
		.as_table_mut()
		.ok_or(Error::Parse("Failed to parse 'package' in Cargo.toml".to_string()))?
		.get_mut("name")
		.ok_or(Error::Parse("Failed to find 'name' in 'package' in Cargo.toml".to_string()))?;

	let common_name = pkg_name.as_str().ok_or(Error::Parse("Failed to parse 'name' in 'package' in Cargo.toml".to_string()))?;
	let new_name = format!("{}-{}", common_name, name);
	*pkg_name = TomlValue::String(new_name.to_string());

	// update lib.name
	let lib_name = cargo_toml
		.get_mut("lib")
		.ok_or(Error::Parse("Failed to find 'lib' in Cargo.toml".to_string()))?
		.as_table_mut()
		.ok_or(Error::Parse("Failed to parse 'lib' in Cargo.toml".to_string()))?
		.get_mut("name")
		.ok_or(Error::Parse("Failed to find 'name' in 'lib' in Cargo.toml".to_string()))?;

	let common_name = lib_name.as_str().ok_or(Error::Parse("Failed to parse 'name' in 'package' in Cargo.toml".to_string()))?;
	let new_name = format!("{}_{}", common_name, name);
	*lib_name = TomlValue::String(new_name);

	let mut features_table = cargo_toml
		.get_mut("features")
		.ok_or(Error::Parse("Failed to find 'features' in Cargo.toml".to_string()))?
		.as_table_mut()
		.ok_or(Error::Parse("Failed to parse 'features' in Cargo.toml".to_string()))?
		.clone();

	for feature in features {
		features_table.insert(feature.clone(), TomlValue::Array(Vec::new()));
	}

	let all_features = TomlValue::Array(features.iter().map(|x| TomlValue::String(x.clone())).collect());
	features_table.insert("full".to_string(), all_features);

	cargo_toml.insert("features".to_string(), TomlValue::Table(features_table));

	let new_cargo_toml_content = toml::to_string(&cargo_toml)?;
	Ok(new_cargo_toml_content)
}

async fn generate_cargo_toml<P: AsRef<Path>>(path: P, name: &str, features: &[String]) -> Result<(), Error> {
	let new_cargo_toml_content = render_cargo_toml(name, features).await?;
	let mut cargo_toml = File::create(path.as_ref().join("Cargo.toml")).await?;
	cargo_toml.write_all(new_cargo_toml_content.as_bytes()).await?;
	Ok(())
}