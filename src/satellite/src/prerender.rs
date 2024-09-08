use ic_cdk::{id, print};
use junobuild_satellite::{get_asset_store, get_content_chunks_store, OnSetDocContext, set_asset_handler};
use junobuild_collections::types::rules::Memory;
use junobuild_shared::types::core::Blob;
use junobuild_storage::http::types::HeaderField;
use junobuild_storage::types::store::AssetKey;
use scraper::{Html, Selector};

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
                        let edited_blog = edit_medata(&blob)?;
                        return insert_asset(&key, &edited_blog)
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

fn edit_medata(data: &Blob) -> Result<Blob, String> {
    let html_content = match String::from_utf8(data.clone()) {
        Ok(content) => content,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let document = Html::parse_document(&html_content);

    let title_selector = Selector::parse("title").map_err(|e| e.to_string())?;
    let title_element = document.select(&title_selector).next();

    let title = "Super super".to_string();

    match title_element {
        None => {
            return Err("No title tag found!".to_string());
        },
        Some(title_element) => {
            let new_title_html = format!("<title>{}</title>", &title);
            let result = document.root_element().html().replace(&title_element.html(), &new_title_html);

            Ok(result.into_bytes())
        }
    }
}

fn insert_asset(name: &String, data: &Blob) -> Result<(), String> {
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