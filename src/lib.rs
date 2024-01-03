use serde::Deserialize;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ToDo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Request<'req> {
    pub protocol: &'req str,
    pub host: &'req str,
    pub path: &'req str,
    pub query: &'req str,
}

impl Request<'_> {
    pub fn create_url(&self) -> Url {
        let request: Request = Request {
            protocol: self.protocol,
            host: self.host,
            path: self.path,
            query: self.query,
        };

        println!("{request:#?}");

        let url = Url::from_str(
            format!(
                "{}://{}{}{}",
                request.protocol, request.host, request.path, request.query
            )
            .as_str(),
        )
        .unwrap();
        url
    }

    pub async fn create_request(url: &str) {
        let result: Result<ToDo, _> = reqwest::get(Url::parse(url).unwrap())
            .await
            .unwrap()
            .json()
            .await;
        println!("{result:#?}");
    }
}
