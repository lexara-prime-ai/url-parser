use reqwest;
use std::str::FromStr;
use url::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
struct ToDo {
    userId: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Debug)]
struct Request<'req> {
    protocol: &'req str,
    host: &'req str,
    path: &'req str,
    query: &'req str,
}

impl Request<'_> {
    fn create_url(&self) -> Url {
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

    async fn create_request(url: &str) {
        let result: Result<ToDo, _> = reqwest::get(Url::parse(url).unwrap())
            .await
            .unwrap()
            .json()
            .await;
        println!("{result:#?}");
    }
}

#[tokio::main]
async fn main() {
    let request: Request = Request {
        protocol: "https",
        host: "jsonplaceholder.typicode.com",
        path: "/todos/1",
        query: "",
    };

    let query = Request::create_url(&request);
    Request::create_request(query.as_str()).await;
    println!("{query:?}");
}
