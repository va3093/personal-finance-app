use std::path::Path;

extern crate personal_finance;
use chrono::{TimeZone, Utc};
use personal_finance::processing;

#[test]
fn test_generate_future_financial_status() {
    let fin_state_influencers = personal_finance::data::read_financial_state_influencers_json(
        &Path::new("tests/data/financial_state_influencers_base.json"),
    )
    .unwrap();

    let expected_financial_state = personal_finance::processing::PeriodicFinancialData {
        monthly_expenses: vec![
            1000.0, 2000.0, 1000.0, 2000.0, 1000.0, 1000.0, 400.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ],
        monthly_income: vec![
            2800.0, 3600.0, 3300.0, 2800.0, 2800.0, 2800.0, 2800.0, 2800.0, 2800.0, 2800.0, 2800.0,
            2000.0,
        ],
        monthly_asset_delta: vec![0.0; 12],
        monthly_liability_delta: vec![0.0; 12],
    };
    let financial_state = processing::generate_future_finacial_status(
        fin_state_influencers,
        Utc.ymd(2014, 1, 1).and_hms(0, 0, 0),
    );

    assert_eq!(financial_state, expected_financial_state);
}
