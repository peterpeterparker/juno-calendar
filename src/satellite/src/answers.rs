use ic_cdk::id;
use junobuild_satellite::{count_docs_store, get_doc_store, OnSetDocContext, set_doc_store, SetDoc};
use junobuild_shared::types::list::{ListMatcher, ListParams};
use junobuild_utils::{decode_doc_data, encode_doc_data};
use crate::types::EventsData;

pub fn count_event_answers(context: &OnSetDocContext) -> Result<(), String> {
    let event_key = context.data.data.after.description.clone();

    match event_key {
        Some(event_key) => count_and_save_answers(&event_key),
        None => Err("This is unexpected".to_string()),
    }
}

fn count_and_save_answers(event_key: &String) -> Result<(), String> {
    let count = count_answers(event_key)?;

    // TODO:
    // If context target collection is "answers" then
    // Count answers for "description" field
    // Save count in "events" collection

    let _ = save_count_answers(event_key, &count)?;

    ic_cdk::print(format!("----___ ANSWERS ___----> on_set_doc {} {}", event_key, count));

    Ok(())
}

fn count_answers(event_key: &String) -> Result<usize, String> {
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

    count_docs_store(id(), "answers".to_string(), &params)
}

fn save_count_answers(event_key: &String, count: &usize) -> Result<(), String> {
    let collection = "events".to_string();
    let event = get_doc_store(id(), collection.clone(), event_key.clone())?;

    match event {
        Some(event) => {
            let mut data: EventsData = decode_doc_data(&event.data)?;
            data.count_answers = Some(count.clone());

            // match data.count_answers {
            //     None => data.count_answers = Some(1),
            //     Some(counter) => data.count_answers = Some(counter + 1),
            // }

            let encode_data = encode_doc_data(&data)?;

            let doc: SetDoc = SetDoc {
                data: encode_data,
                description: event.description,
                version: event.version,
            };

            set_doc_store(
                id(),
                collection,
                event_key.clone(),
                doc,
            )?;

            Ok(())
        },
        None => {
            Err("No document found, that is weird!".to_string())
        }
    }

}