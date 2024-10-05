mod answers;
mod assert;
mod delete;
mod http;
mod images;
mod templates;
mod types;

use crate::answers::count_event_answers;
use crate::assert::assert_no_events;
use crate::delete::delete_answers;
use crate::http::notify::send_email;
use crate::http::response::transform_response;
use crate::images::generate_social_image;
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_cdk_macros::query;
use junobuild_macros::{assert_delete_doc, on_delete_doc, on_set_doc};
use junobuild_satellite::{
    include_satellite, AssertDeleteDocContext, OnDeleteDocContext, OnSetDocContext,
};

#[on_set_doc(collections = ["answers", "events"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {
    match context.data.collection.as_str() {
        "events" => generate_social_image(&context),
        "answers" => {
            count_event_answers(&context)?;
            send_email(&context).await?;

            Ok(())
        }
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

#[query]
fn transform(raw: TransformArgs) -> HttpResponse {
    transform_response(raw)
}

include_satellite!();
