use base64::prelude::*;
use gloo_net::http::Request;

use crate::states::app_state::User;

const BASE_URI: &str = "https://eu-west-2.aws.data.mongodb-api.com/app/trainops-ciefkxv/endpoint/";
const AUTH_HEAD: &str = "Authorization";

pub async fn login(username: String, password: String) -> Option<User> {
    
    let logstr = format!("{}:{}", username, password);
    let b64 = BASE64_STANDARD.encode(logstr.as_bytes());
    let auth = format!("Basic {b64}");

    let uri = BASE_URI.to_string() + "login";
    let response = Request::post(&uri)
        .header(AUTH_HEAD, &auth)
        .send()
        .await
        .expect("HTTP request failed");

    if response.status() == 200 {
        let login = response
            .json::<User>()
            .await
            .expect("deserialization failed");
        return Some(login)
    }
    None
}