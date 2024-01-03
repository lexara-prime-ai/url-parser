use lx_parser::Request;

// Example implementation
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
