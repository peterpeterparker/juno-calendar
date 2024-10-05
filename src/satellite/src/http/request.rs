use crate::types::{EnvData, SettingData};
use ic_cdk::api::management_canister::http_request::http_request as http_request_outcall;
use ic_cdk::api::management_canister::http_request::{
    CanisterHttpRequestArgument, HttpHeader, HttpMethod,
};
use ic_cdk::print;
use junobuild_satellite::{error, log};
use junobuild_shared::types::core::Key;
use serde_json::json;

pub async fn post_email(
    env: &EnvData,
    settings: &SettingData,
    answer_key: &Key,
) -> Result<(), String> {
    let request = get_request(env, settings, answer_key)?;

    log(format!(
        "🔫 ---------> Starting the request. {}",
        request.url
    ))?;

    print(format!(
        "🔫 ---------> Starting the request. {}",
        request.url
    ));

    match http_request_outcall(request, 25_000_000_000).await {
        Ok((_response,)) => {
            log("✅ ---------> Request processed.".to_string())?;

            print("✅ ---------> Request processed.".to_string());
        }
        Err((r, m)) => {
            let message = format!("HTTP request error. RejectionCode: {:?}, Error: {}", r, m);

            error(format!("‼️ --> {}.", message))?;

            print(format!("‼️ --> {}.", message));

            // We do not bubble an error in this particular use case because we write the error in the datastore for demo purpose
        }
    }

    Ok(())
}

fn get_request(
    env: &EnvData,
    settings: &SettingData,
    answer_key: &Key,
) -> Result<CanisterHttpRequestArgument, String> {
    let email_notifications_url =
        "https://europe-west6-datepicker-xyz.cloudfunctions.net/datepicker/notifications/email";

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "idempotency-key".to_string(),
            value: answer_key.to_owned(),
        },
        HttpHeader {
            name: "authorization".to_string(),
            value: format!("Bearer {}", env.token),
        },
    ];

    let body = json!({
      "to": settings.email.to_owned(),
    });

    let body_json = serde_json::to_string(&body).map_err(|e| e.to_string())?;

    Ok(CanisterHttpRequestArgument {
        url: email_notifications_url.to_string(),
        method: HttpMethod::POST,
        body: Some(body_json.as_bytes().to_vec()),
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    })
}
