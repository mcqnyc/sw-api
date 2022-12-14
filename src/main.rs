use dotenv::dotenv;
use std::env;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponse {
    visits: Vec<Visits>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Visits {
    date: String,
    visits: f32,
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = std::env::var("API_KEY").expect("API_KEY must be set");

    let args: Vec<String> = env::args().collect();

    let domain = &args[1];
    let start_date = &args[2];
    let end_date = &args[3];

    let request_url = format!("http://api.similarweb.com/v1/website/{}/total-traffic-and-engagement/visits?api_key={}&start_date={}&end_date={}&country=gb&granularity=monthly&main_domain_only=false&format=json", domain, api_key, start_date, end_date);
    let response = reqwest::get(&request_url).await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ApiResponse>().await {
                Ok(parsed) => write_data_to_csv(parsed, domain),
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

fn write_data_to_csv(json_data: ApiResponse, domain: &str) {
    // Open a file for writing the CSV data
    let mut wtr = csv::Writer::from_path("output.csv").unwrap();
    wtr.write_record(&["Domain", "Date", "Visits"]).unwrap();

    // Iterate over the JSON data and write it to the CSV file
    for item in json_data.visits {
        let date = &item.date;
        let visits = &item.visits.to_string();

        wtr.serialize(&[domain, date, visits]).unwrap();
    }

    wtr.flush().unwrap();
    println!("Job done!")
}
