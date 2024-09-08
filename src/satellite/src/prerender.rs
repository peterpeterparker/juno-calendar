use ic_cdk::{id, print};
use junobuild_satellite::{get_asset_store, get_content_chunks_store, OnSetDocContext, set_asset_handler};
use junobuild_collections::types::rules::Memory;
use junobuild_shared::types::core::Blob;
use junobuild_storage::http::types::HeaderField;
use junobuild_storage::types::store::AssetKey;

pub fn prerender_event_page(context: &OnSetDocContext) -> Result<(), String> {
    let key = context.data.key.clone();

    let asset = get_asset_store(id(), &"#dapp".to_string(), "/event/sample.html".to_string())?;

    match asset {
        None => {
            return Err("No sample file found!".to_string());
        }
        Some(asset) => {
            let content: Vec<Option<Blob>> = asset.encodings.iter().enumerate().map(|(i, (_, encoding))| get_content_chunks_store(&encoding, i, &Memory::Heap)).collect();

            let first_content = content.first();

            if first_content.is_none() {
                return Err("No encodings found for sample file!".to_string());
            }

            if let Some(blob) = first_content {
                match blob {
                    Some(blob) => {
                        return insert_asset(&key, blob)
                    }
                    None => {
                        return Err("No content found for sample file!".to_string());
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn insert_asset(name: &String, data: &Blob) -> Result<(), String> {
    let collection = "#dapp".to_string();

    let full_path = format!("/event/{}.html", name.clone()).to_string();

    let key: AssetKey = AssetKey {
        name: name.clone(),
        full_path: full_path.clone(),
        token: None,
        collection,
        owner: id(),
        description: None,
    };

    let headers = vec![HeaderField(
        "content-type".to_string(),
        "text/html".to_string(),
    )];

    set_asset_handler(&key, data, &headers)?;

    print(format!(
        "Event HTML page prerendered to: http://{}.localhost:5987{}",
        id(),
        full_path
    ));

    Ok(())
}