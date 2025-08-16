#[tokio::main]
pub async fn main() {
    paleastra::entrypoint::start_server().await;
}
