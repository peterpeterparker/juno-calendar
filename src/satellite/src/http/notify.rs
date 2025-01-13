use crate::http::request::post_email;
use crate::types::{AnswerData, AnswersData, EnvData, SettingData};
use ic_cdk::id;
use junobuild_collections::types::core::CollectionKey;
use junobuild_satellite::{get_doc_store, set_doc_store, OnSetDocContext, SetDoc};
use junobuild_shared::types::core::Key;
use junobuild_utils::{decode_doc_data, encode_doc_data};
use serde::de::DeserializeOwned;

pub async fn send_email(context: &OnSetDocContext) -> Result<(), String> {
    let event_key = context.data.data.after.description.clone();
    let answer_key = context.data.key.clone();
    let answer_data: AnswersData = decode_doc_data(&context.data.data.after.data)?;

    match event_key {
        Some(event_key) => notify_email(&event_key, &answer_key, &answer_data).await,
        None => Err("This is unexpected".to_string()),
    }
}

pub async fn notify_email(
    event_key: &Key,
    answer_key: &Key,
    answer_data: &AnswersData,
) -> Result<(), String> {
    // A utility used once to save the authorization token locally
    // save_env_notifications()?;

    let env = get_doc_data(&"env".to_string(), &"notifications".to_string())?;
    let settings = get_settings(event_key)?;

    post_email(&env, &settings, answer_key, answer_data).await
}

fn save_env_notifications() -> Result<(), String> {
    let collection = "env".to_string();

    let data: EnvData = EnvData {
        token: "<REPLACE_WITH_BEARER_TOKEN>".to_string(),
    };

    let encode_data = encode_doc_data(&data)?;

    // If you ever need to update the token you have saved locally
    // let env = get_doc_store(id(), collection.clone(), "notifications".to_string())?;
    //
    // if env.is_none() {
    //    return Err("Unexpected".to_string());
    // }

    let doc: SetDoc = SetDoc {
        data: encode_data,
        description: None,
        version: None, // env.unwrap().version.clone(),
    };

    set_doc_store(id(), collection, "notifications".to_string(), doc)?;

    Ok(())
}

fn get_settings(event_key: &Key) -> Result<SettingData, String> {
    let event = get_doc_store(id(), "events".to_string(), event_key.clone())?;

    match event {
        Some(event) => {
            let settings = get_doc_data(&"settings".to_string(), &event.owner.to_text())?;
            Ok(settings)
        }
        None => Err("No event found".to_string()),
    }
}

fn get_doc_data<T>(collection: &CollectionKey, key: &Key) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let doc = get_doc_store(id(), collection.clone(), key.clone())?;

    match doc {
        Some(doc) => {
            let data: T = decode_doc_data(&doc.data)?;
            Ok(data)
        }
        None => Err("No document found".to_string()),
    }
}
