mod answers;
mod images;
mod templates;
mod types;
mod assert;
mod delete;

use ic_cdk_macros::{query};
use crate::answers::count_event_answers;
use crate::images::generate_social_image;
use junobuild_macros::{assert_delete_doc, on_delete_doc, on_set_doc};
use junobuild_satellite::{AssertDeleteDocContext, include_satellite, OnDeleteDocContext, OnSetDocContext};
use crate::assert::assert_no_events;
use crate::delete::delete_answers;

#[on_set_doc(collections = ["answers", "events"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    match context.data.collection.as_str() {
        "events" => generate_social_image(&context),
        "answers" => count_event_answers(&context),
        _ => Err("This is not supported".to_string()),
    }
}

#[assert_delete_doc(collections = ["answers"])]
fn assert_delete_doc(context: AssertDeleteDocContext) -> Result<(), String> {
    assert_no_events(&context)
}

#[on_delete_doc(collections = ["events"])]
async fn on_delete_doc(context: OnDeleteDocContext) -> Result<(), String> {
    delete_answers(&context)
}

#[query]
fn hello(text: String) -> String {
    format!("Hello: {}", text)
}

include_satellite!();
