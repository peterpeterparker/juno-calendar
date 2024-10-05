use ic_cdk::id;
use junobuild_satellite::{get_doc_store, AssertDeleteDocContext};

pub fn assert_no_events(context: &AssertDeleteDocContext) -> Result<(), String> {
    let event = get_doc_store(id(), "events".to_string(), context.data.key.to_string())?;

    if event.is_none() {
        return Ok(());
    }

    Err("The answer has still an event.".to_string())
}
