use super::data;
use super::models;
use super::models::{FinancialForecast, FinancialStateChanges};
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use models::{FinancialStateInfluencer, ProcessedSpendingGoal};
use serde::{Deserialize, Serialize};
use std::path::Path;

fn bisect_vectors_from_undesirable_income(
    undesirable_income: &Vec<FinancialStateChanges>,
    bisection_point: i16,
) -> Vec<FinancialStateChanges> {
    return vec![];
}

fn cum_sum(vec: &Vec<f64>) -> Vec<f64> {
    let mut res: Vec<f64> = vec![];
    let mut cum_sum: f64 = 0.0;
    for val in vec {
        cum_sum += val;
        res.push(cum_sum)
    }
    return res;
}

fn add(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut res: Vec<f64> = vec![];
    for i in 0..vec1.len() {
        res.push(vec1[i] + vec2[i]);
    }
    return res;
}

fn minus(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut res: Vec<f64> = vec![];
    for i in 0..vec1.len() {
        res.push(vec1[i] - vec2[i]);
    }
    return res;
}

fn are_balances_valid(balances: &Vec<f64>) -> bool {
    return true;
}

struct BisectionPoints {
    start: i16,
    end: i16,
}

impl BisectionPoints {
    fn middle(self: &Self) -> i16 {
        self.start + ((self.end - self.start) / 2)
    }

    fn new_bisection_point(
        self: &Self,
        fin_state_influencers: &models::FinancialStateInfluencers,
        bisect_left: bool,
    ) -> Option<Self> {
        let middle = self.middle();
        if middle == self.start {
            return None;
        }
        if bisect_left {
            return Some(BisectionPoints {
                start: self.start,
                end: middle - 1,
            });
        } else {
            return Some(BisectionPoints {
                start: self.start,
                end: middle - 1,
            });
        }
    }

    fn initial_point(fin_state_influencers: &models::FinancialStateInfluencers) -> Self {
        Self { start: 0, end: 10 }
    }
}

pub enum ProcessFinancialStateInfluencersErrors {
    MonthlySpendingExceedsIncome,
    UnableToProduceFinancialForecast,
}

pub fn process_financial_state_influencers(
    fin_state_influencers: models::FinancialStateInfluencers,
) -> Result<models::FinancialForecast, ProcessFinancialStateInfluencersErrors> {
    let mut spending_goals: Vec<models::ProcessedSpendingGoal> = Vec::new();
    let fixed_financial_state_chagnes =
        fin_state_influencers.financial_state_changes_from_fixed_sources();
    let prioritised_optional_spending_goals =
        fin_state_influencers.prioritised_optional_spending_goals();
    let income_from_undesirable_sources = fin_state_influencers.income_from_undesirable_sources();

    let mut last_successful_forecast: Option<FinancialForecast> = None;

    let mut bisection = BisectionPoints::initial_point(&fin_state_influencers);
    let mut final_merged_fin_state_changes: FinancialStateChanges = FinancialStateChanges::new();

    loop {
        // ****
        // **** STEP 1: Set up the params for an iteration
        // ****

        // Create base fin state changes without spending goals
        let current_income_from_undesirable_sources = bisect_vectors_from_undesirable_income(
            &income_from_undesirable_sources,
            bisection.middle(),
        );
        // let base_fin_state_changes =

        // This cashflow holds the current cashflow vector as we loop through the spending goals and subtract
        // the their `FinancialStateChanges` from the cashflow
        let mut cash_balance = cum_sum(&final_merged_fin_state_changes.cashflow);

        // If the cash_balance is not valid (i.e has parts of overdraft) then we exist the loop
        if !are_balances_valid(&cash_balance) {
            return Err(ProcessFinancialStateInfluencersErrors::MonthlySpendingExceedsIncome);
        }

        // ****
        // **** STEP 2: Process Spending goals ****
        // ****

        // Here we process the spending goals by looping throught the spending goals and applying them to the current_cashflow.
        let mut all_spending_goals_achievable: bool = true;
        for fin_state_influencer in &prioritised_optional_spending_goals {
            let fin_state_changes = fin_state_influencer.generate_financial_state_changes();

            // Temp holder of the merged cash balance vector. Will set the base vector once
            // we check that this spending goal does not cause and overdraft
            let _cash_balance = add(&cash_balance, &cum_sum(&fin_state_changes.cashflow));
            let processed_spending_goal =
                ProcessedSpendingGoal::from_spending_goal(fin_state_influencer, &_cash_balance);

            spending_goals.push(processed_spending_goal);

            // If the spending goal doesn't cause an overdraft then update the cash balance vector and merge the `FinancialStateChanges`
            // from the spending goal (that may contain updates to assets and liabilities) with the floating fin_state_changes tracker
            if processed_spending_goal.is_achieveable {
                cash_balance = _cash_balance;
                final_merged_fin_state_changes.merge(&fin_state_changes);
            } else {
                all_spending_goals_achievable = false;
            }
        }

        // ****
        // **** STEP 3: Optimise independence age ****
        // ****

        let financial_forecast = FinancialForecast {
            all_spending_goals_achievable: all_spending_goals_achievable,
            spending_goals: spending_goals.to_vec(),
            financial_independence_age: fin_state_influencers
                .age_at_bisection_point(bisection.middle()),
            monthly_cashflow_deltas: final_merged_fin_state_changes.cashflow.to_vec(),
            monthly_networth: minus(
                &cum_sum(&final_merged_fin_state_changes.assets_delta),
                &cum_sum(&final_merged_fin_state_changes.liabilities_delta),
            ),
        };

        // If we failed to achieve all the spending goals after the first attempt there is no point trying
        // to bisect
        if !all_spending_goals_achievable
            && bisection.middle() == models::FinancialStateInfluencers::VECTOR_LENGTH()
        {
            break;
        }

        // store the current bisection point
        let _bisection_point = bisection.middle();
        last_successful_forecast = match last_successful_forecast {
            None => Some(financial_forecast),
            Some(forecast) => {
                if forecast.was_improved_by(&financial_forecast) {
                    // Todo: new_bisection_point should raise error when it can no longer bisect
                    bisection = match bisection.new_bisection_point(&fin_state_influencers, true) {
                        Some(bisection) => bisection,
                        None => {
                            last_successful_forecast = Some(forecast);
                            break;
                        }
                    };
                    Some(financial_forecast)
                } else {
                    // Todo: new_bisection_point should raise error when it can no longer bisect
                    bisection = match bisection.new_bisection_point(&fin_state_influencers, false) {
                        Some(bisection) => bisection,
                        None => {
                            last_successful_forecast = Some(forecast);
                            break;
                        }
                    };
                    Some(forecast)
                }
            }
        }
    }

    match last_successful_forecast {
        None => Err(ProcessFinancialStateInfluencersErrors::UnableToProduceFinancialForecast),
        Some(forecast) => {
            return Ok(forecast);
        }
    }
}
