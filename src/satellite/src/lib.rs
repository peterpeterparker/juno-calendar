mod answers;
mod images;
mod templates;
mod types;

use candid::CandidType;
use ic_cdk_macros::{query};
use crate::answers::count_event_answers;
use crate::images::generate_social_image;
use junobuild_macros::{on_set_doc};
use junobuild_satellite::{
    include_satellite, OnSetDocContext
};

#[on_set_doc(collections = ["answers", "events"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    match context.data.collection.as_str() {
        "events" => generate_social_image(&context),
        "answers" => count_event_answers(&context),
        _ => Err("This is not supported".to_string()),
    }
}

#[query]
fn hello(text: String) -> String {
    format!("Hello: {}", text)
}

include_satellite!();
