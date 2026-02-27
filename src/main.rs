use infosec_webserver_lib::presentation::api::server;

#[tokio::main]
async fn main() {
    server::start().await;
}
