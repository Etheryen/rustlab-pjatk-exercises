use isahc::ReadResponseExt;

pub fn api_request(url: &str) -> (u32, String) {
    let mut response = isahc::get(url).unwrap();
    (
        response
            .status()
            .to_string()
            .split_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap(),
        response.text().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{Method, MockServer};

    #[test]
    fn it_works() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(Method::GET).path("/");
            then.status(200)
                .header("content-type", "text/html")
                .body("API_RESPONSE");
        });

        let (status, text) = api_request(&server.url("/"));

        mock.assert();

        assert_eq!(status, 200);
        assert_eq!(text, "API_RESPONSE");
    }
}
