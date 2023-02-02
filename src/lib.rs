// use log::{info, error};
#[macro_use] extern crate log;

use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
    visits: Vec<Visits>,
}

pub struct ApiParams {
    domain: String,
    start_date: String,
    end_date: String,
    country: String,
}

impl ApiParams {
    pub fn build(args: &[String]) -> Result<ApiParams, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let domain = args[1].clone();
        let start_date = args[2].clone();
        let end_date = args[3].clone();
        let country = args[4].clone();

        Ok(ApiParams {
            domain,
            start_date,
            end_date,
            country,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Visits {
    date: String,
    visits: f32,
}

pub async fn run(api_params: ApiParams, api_key: &str) -> Result<(), Box<dyn Error>> {
    let request_url = format!("http://api.similarweb.com/v1/website/{}/total-traffic-and-engagement/visits?api_key={}&start_date={}&end_date={}&country={}&granularity=monthly&main_domain_only=false&format=json", api_params.domain, api_key, api_params.start_date, api_params.end_date, api_params.country);
    let response = reqwest::get(&request_url).await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<ApiResponse>().await {
                Ok(parsed) => write_data_to_csv(parsed, api_params),
                Err(e) => error!("{}", e),
            };
            info!("CSV created");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            // println!("Invalid API key, most likely");
            error!("Invalid API key, most likely");
        }
        other => {
            // panic!("Uh oh! Something unexpected happened: {:?}", other);
            error!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };

    Ok(())
}

fn write_data_to_csv(json_data: ApiResponse, api_params: ApiParams) {
    let mut wtr = csv::Writer::from_path("output.csv").unwrap();
    wtr.write_record(&["Domain", "Country", "Date", "Visits"])
        .unwrap();

    let domain = &api_params.domain;
    let country = &api_params.country;

    for visits in json_data.visits {
        let date = &visits.date;
        let visits = &visits.visits.to_string();

        wtr.serialize(&[domain, country, date, visits]).unwrap();
    }

    wtr.flush().unwrap();
    info!("Job done!");
    println!("Job done!");
}
