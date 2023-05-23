use crate::http_client::HttpClient;

mod http_client;

#[no_mangle]
pub extern "C" fn fetch_google() -> i32 {
    HttpClient::fetch_google()
}
