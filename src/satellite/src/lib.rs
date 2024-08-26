mod types;

use junobuild_shared::types::list::{ListMatcher, ListParams};
use ic_cdk::id;
use junobuild_macros::{
    assert_delete_asset, assert_delete_doc, assert_set_doc, assert_upload_asset, on_delete_asset,
    on_delete_doc, on_delete_many_assets, on_delete_many_docs, on_set_doc, on_set_many_docs,
    on_upload_asset,
};
use junobuild_satellite::{include_satellite, AssertDeleteAssetContext, AssertDeleteDocContext, AssertSetDocContext, AssertUploadAssetContext, OnDeleteAssetContext, OnDeleteDocContext, OnDeleteManyAssetsContext, OnDeleteManyDocsContext, OnSetDocContext, OnSetManyDocsContext, OnUploadAssetContext, count_docs_store};

#[on_set_doc(collections = ["answers"])]
async fn on_set_doc(context: OnSetDocContext) -> Result<(), String> {

    let collection = context.data.collection;
    let event_key = context.data.data.after.description;

    match event_key {
        Some(event_key) => {
            // TODO:
            // If context target collection is "answers" then
            // Count answers for "description" field
            // Save count in "events" collection

            let params: ListParams = ListParams {
                owner: None,
                matcher: Some(ListMatcher {
                    description: Some(event_key.clone()),
                    key: None,
                    updated_at: None,
                    created_at: None,
                }),
                order: None,
                paginate: None,
            };

            let count = count_docs_store(id(), collection, &params)?;

            ic_cdk::print(format!("----___ ANSWERS ___----> on_set_doc {} {}", event_key, count));

            Ok(())
        },
        None => {
            Err("This is unexpected".to_string())
        }
    }
}

#[on_set_many_docs]
async fn on_set_many_docs(_context: OnSetManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_doc]
async fn on_delete_doc(_context: OnDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_docs]
async fn on_delete_many_docs(_context: OnDeleteManyDocsContext) -> Result<(), String> {
    Ok(())
}

#[on_upload_asset]
async fn on_upload_asset(_context: OnUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_asset]
async fn on_delete_asset(_context: OnDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

#[on_delete_many_assets]
async fn on_delete_many_assets(_context: OnDeleteManyAssetsContext) -> Result<(), String> {
    Ok(())
}

#[assert_set_doc]
fn assert_set_doc(_context: AssertSetDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_doc]
fn assert_delete_doc(_context: AssertDeleteDocContext) -> Result<(), String> {
    Ok(())
}

#[assert_upload_asset]
fn assert_upload_asset(_context: AssertUploadAssetContext) -> Result<(), String> {
    Ok(())
}

#[assert_delete_asset]
fn assert_delete_asset(_context: AssertDeleteAssetContext) -> Result<(), String> {
    Ok(())
}

include_satellite!();
