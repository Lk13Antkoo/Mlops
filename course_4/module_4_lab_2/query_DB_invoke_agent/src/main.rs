mod generic_handler;
use generic_handler::function_handler;
use lambda_runtime::{run, service_fn, Error};
use tracing_subscriber;
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    run(service_fn(function_handler)).await?;
    Ok(())
}
