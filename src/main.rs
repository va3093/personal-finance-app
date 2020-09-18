use chrono::Utc;
use processing::FinancialStateInfluencers;
use std::path::Path;

extern crate serde;
extern crate serde_json;
mod data;
mod models;
mod processing;

fn main() {
    let fin_state_influencers: FinancialStateInfluencers =
        data::read_financial_state_influencers_json(Path::new(
            "data/financial_state_influencers.json",
        ))
        .unwrap();
    println!(
        "{:?}",
        processing::generate_future_finacial_status(fin_state_influencers, Utc::now())
            .monthly_closing_balance()
    )
}
