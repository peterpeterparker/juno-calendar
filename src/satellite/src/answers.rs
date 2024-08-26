use ic_cdk::id;
use junobuild_satellite::count_docs_store;
use junobuild_shared::types::list::{ListMatcher, ListParams};

pub fn count_and_save_answers(event_key: &String) -> Result<(), String> {
    let count = count_answers(event_key)?;

    // TODO:
    // If context target collection is "answers" then
    // Count answers for "description" field
    // Save count in "events" collection

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