use ic_cdk::{id, print};
use junobuild_satellite::{DelDoc, delete_doc_store, list_docs_store, OnDeleteDocContext};
use junobuild_shared::types::list::{ListMatcher, ListParams};

pub fn delete_answers(context: &OnDeleteDocContext) -> Result<(), String> {
    let key = context.data.key.clone();

    let filter: ListParams = ListParams {
        paginate: None,
        order: None,
        matcher: Some(ListMatcher {
            key: None,
            description: Some(key),
            created_at: None,
            updated_at: None
        }),
        owner:None
    };

    let answers = list_docs_store(id(), "answers".to_string(), &filter)?;

    for (key, doc) in answers.items {
        let del_doc: DelDoc = DelDoc {
            version: doc.version
        };

        delete_doc_store(id(), "answers".to_string(), key.clone(), del_doc)?;

        print(format!("Deleted answer {}", key));
    }

    Ok(())
}