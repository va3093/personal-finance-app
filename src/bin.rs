#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate plotly;
extern crate serde;
extern crate serde_json;
mod data;
mod models;
mod processing;
mod utils;
use chrono::prelude::*;
use rocket_contrib::json::Json;
use std::{error::Error, path::Path};

#[post("/forecast")]
fn index() -> Option<Json<models::FinancialForecast>> {
    let fin_state_influencers = data::read_financial_state_influencers_json(&Path::new(
        "tests/data/financial_state_influencers_base.json",
    ))
    .unwrap();
    let current_date = NaiveDate::from_ymd(2013, 12, 15);
    let result =
        processing::process_financial_state_influencers(fin_state_influencers, current_date);
    println!("{:?}", result);
    match result {
        Ok(forecast) => return Some(Json(forecast)),
        Err(e) => {
            println!("{:?}", e);
            return None;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    rocket::ignite()
        .mount("/", routes![index])
        // .attach(cors)
        .attach(rocket_cors::CorsOptions::default().to_cors().unwrap())
        .launch();
    Ok(())
}
