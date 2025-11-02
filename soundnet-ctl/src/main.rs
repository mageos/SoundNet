use soundnet_ctl::run;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber for logging.
    tracing_subscriber::fmt::init();

    run().await;
}