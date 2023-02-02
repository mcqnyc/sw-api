use dotenv::dotenv;
use std::{env, process};
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use sw_api::{run, ApiParams};

// tokio lets us use "async" on our main function
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");

    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");
    info!("env information loaded");

    let args: Vec<String> = env::args().collect();

    // let api_params = ApiParams::build(&args).unwrap_or_else(|err| {
    let api_params = ApiParams::build(&args).unwrap_or_else(|err| {
        error!("Problem passing arguments: {err}");
        process::exit(1);
    });

    // run(api_params, &api_key).await.expect("TODO: panic message");
    if let Err(e) = run(api_params, &api_key).await {
        println!("Application error: {e}");
        process::exit(1);
    }
}
