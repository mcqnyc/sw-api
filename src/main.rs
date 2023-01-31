use dotenv::dotenv;
use std::{env, process};

use sw_api::{run, ApiParams};

// tokio lets us use "async" on our main function
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let args: Vec<String> = env::args().collect();

    // let api_params = ApiParams::build(&args).unwrap_or_else(|err| {
    let api_params = ApiParams::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    // run(api_params, &api_key).await.expect("TODO: panic message");
    if let Err(e) = run(api_params, &api_key).await {
        println!("Application error: {e}");
        process::exit(1);
    }
}
