use super::processing;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_financial_state_influencers_json(
    path: &Path,
) -> Result<processing::FinancialStateInfluencers, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let financial_state_influencers: processing::FinancialStateInfluencers =
        serde_json::from_reader(reader)?;
    return Ok(financial_state_influencers);
}

#[test]
fn test_read_financial_state_influencers_json() {
    use super::models::*;
    use chrono::prelude::*;
    use std::io::Write;
    use std::string::String;

    let expected = processing::FinancialStateInfluencers {
        once_off_expenses: vec![OnceOffExpense {
            name: String::from("x"),
            date: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            value: 100.10,
        }],
        monthly_expenses: vec![MonthlyExpense {
            name: String::from("x"),
            end_date: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            start_date: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            value: 100.10,
        }],
        once_off_incomes: vec![],
        monthly_incomes: vec![MonthlyIncome {
            name: String::from("x"),
            end_date: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            start_date: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
            value: 100.10,
        }],
        investments: vec![],
    };
    let data = r#"
        {
            "once_off_expenses": [{"name": "x", "date": "2014-07-08T09:10:11Z", "value":100.10}],
            "monthly_expenses": [{"name": "x", "end_date": "2014-07-08T09:10:11Z", "start_date": "2014-07-08T09:10:11Z", "value":100.10}],
            "once_off_incomes": [],
            "monthly_incomes": [{"name": "x", "end_date": "2014-07-08T09:10:11Z", "start_date": "2014-07-08T09:10:11Z", "value":100.10}],
            "investments": []
        }"#;
    let path_str = "/tmp/financial_state_influencers_json.json";
    let mut file = File::create(path_str).unwrap();
    file.write_all(data.as_bytes()).unwrap();

    let res: processing::FinancialStateInfluencers =
        read_financial_state_influencers_json(Path::new(path_str)).unwrap();

    assert_eq!(res, expected)
}
