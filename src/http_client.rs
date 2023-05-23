pub struct HttpClient;

impl HttpClient {
    pub fn fetch_google() -> i32 {
        let body: String = ureq::get("https://google.com")
            .call()
            .expect("did not happen")
            .into_string()
            .expect("did also not happend");
        body.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::http_client::HttpClient;

    #[test]
    fn http_client_calls_google() {
        let answer = HttpClient::fetch_google();
        assert!(answer > 18000);
    }
}
