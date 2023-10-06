mod server;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";

    println!("GraphiQL IDE: http://{addr}/graphql");

    server::Server::bind(&addr.parse().unwrap())
        .serve(server::app().into_make_service())
        .await
        .unwrap();
}
