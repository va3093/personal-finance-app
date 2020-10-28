use std::path::Path;

extern crate personal_finance;
use chrono::prelude::*;
use personal_finance::processing;

#[test]
fn test_generate_future_financial_status() {
    let fin_state_influencers = personal_finance::data::read_financial_state_influencers_json(
        &Path::new("tests/data/financial_state_influencers_base.json"),
    )
    .unwrap();
    let current_date = NaiveDate::from_ymd(2013, 12, 15);
    println!(
        "{:?}",
        processing::process_financial_state_influencers(fin_state_influencers, current_date)
    );
}
