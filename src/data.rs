use super::models::FinancialStateInfluencers;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_financial_state_influencers_json(
    path: &Path,
) -> Result<FinancialStateInfluencers, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let financial_state_influencers: FinancialStateInfluencers = serde_json::from_reader(reader)?;
    return Ok(financial_state_influencers);
}
