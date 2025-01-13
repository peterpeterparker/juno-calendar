use ic_cdk::api::management_canister::http_request::{
    HttpResponse, TransformArgs as TransformArgsCdk, TransformContext, TransformFunc,
};
use ic_cdk::id;

pub fn param_transform() -> Option<TransformContext> {
    Some(TransformContext {
        function: TransformFunc(candid::Func {
            principal: id(),
            method: "transform".to_string(),
        }),
        context: vec![],
    })
}

// Strips all data that is not needed from the original response.
pub fn transform_response(raw: TransformArgsCdk) -> HttpResponse {
    HttpResponse {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers: vec![],
    }
}
