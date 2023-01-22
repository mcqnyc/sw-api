use dotenv::dotenv;
use std::{env, process};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    visits: Vec<Visits>,
}

struct ApiParams {
    domain: String,
    start_date: String,
    end_date: String,
    country: String,
}

impl ApiParams {
    fn build(args: &[String]) -> Result<ApiParams, &'static str> {
        if args.len() <  4 {
            return Err("not enough arguments")
        }

        let domain = args[1].clone();
        let start_date = args[2].clone();
        let end_date = args[3].clone();
        let country = args[4].clone();

        Ok(ApiParams { domain, start_date, end_date, country })
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Visits {
    date: String,
    visits: f32,
}

// tokio lets us use "async" on our main function
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let args: Vec<String> = env::args().collect();

    // run(args, api_key);
    // let (domain, start_date, end_date, country) =  parse_api_params(&args);
    let api_params =  ApiParams::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    let request_url = format!("http://api.similarweb.com/v1/website/{}/total-traffic-and-engagement/visits?api_key={}&start_date={}&end_date={}&country={}&granularity=monthly&main_domain_only=false&format=json", api_params.domain, api_key, api_params.start_date, api_params.end_date, api_params.country);
    let response = reqwest::get(&request_url).await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ApiResponse>().await {
                Ok(parsed) => write_data_to_csv(parsed, api_params),
                Err(e) => println!("{}", e),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Invalid API key, most likely");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}

// fn write_data_to_csv(json_data: ApiResponse, domain: &str, country: &str) {
fn write_data_to_csv(json_data: ApiResponse, api_params: ApiParams) {
    // Open a file for writing the CSV data
    let mut wtr = csv::Writer::from_path("output.csv").unwrap();
    wtr.write_record(&["Domain", "Country", "Date", "Visits"]).unwrap();

    let domain = &api_params.domain;
    let country = &api_params.country;

    // Iterate over the JSON data and write it to the CSV file
    for visits in json_data.visits {
        let date = &visits.date;
        let visits = &visits.visits.to_string();

        wtr.serialize(&[domain, country, date, visits]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Job done!")
}
