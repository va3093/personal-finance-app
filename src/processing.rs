use super::models;
use super::models::{FinancialForecast, FinancialStateChanges};
use super::utils::{add, are_balances_valid, cum_sum, date_after_months, minus};
use chrono::prelude::*;
use chrono::Utc;
use models::{FinancialStateInfluencer, ProcessedSpendingGoal};

fn bisect_vectors_from_undesirable_income(
    undesirable_income: &FinancialStateChanges,
    bisection_point: i16,
) -> FinancialStateChanges {
    let mut cash_deltas: Vec<f64> = vec![];
    for (index, cash_delta) in undesirable_income.cashflow.iter().enumerate() {
        if index >= bisection_point as usize {
            cash_deltas.push(0.0)
        } else {
            cash_deltas.push(cash_delta.clone())
        }
    }
    return FinancialStateChanges {
        cashflow: cash_deltas,
        assets_delta: undesirable_income.assets_delta.clone(),
        liabilities_delta: undesirable_income.liabilities_delta.clone(),
    };
}

#[test]
fn test_bisect_vectors_from_undesirable_income() {
    let income_fin_changes = FinancialStateChanges {
        cashflow: vec![100.0; 4],
        assets_delta: vec![10.0; 4],
        liabilities_delta: vec![1.0; 4],
    };
    let expected = FinancialStateChanges {
        cashflow: vec![100.0, 100.0, 0.0, 0.0],
        assets_delta: vec![10.0; 4],
        liabilities_delta: vec![1.0; 4],
    };

    assert_eq!(
        bisect_vectors_from_undesirable_income(&income_fin_changes, 2),
        expected
    )
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

#[derive(Debug)]
pub enum ProcessFinancialStateInfluencersErrors {
    MonthlySpendingExceedsIncome,
    UnableToProduceFinancialForecast,
}

#[test]
fn test_date_after_months() {
    let current_date = NaiveDate::from_ymd(2020, 6, 15);
    let expected_date = NaiveDate::from_ymd(2022, 3, 15);
    assert_eq!(date_after_months(&current_date, 21), expected_date);

    let current_date = NaiveDate::from_ymd(2020, 12, 15);
    let expected_date = NaiveDate::from_ymd(2021, 12, 15);
    assert_eq!(date_after_months(&current_date, 12), expected_date);
}

pub fn process_financial_state_influencers(
    fin_state_influencers: models::FinancialStateInfluencers,
    current_date: NaiveDate,
) -> Result<models::FinancialForecast, ProcessFinancialStateInfluencersErrors> {
    let mut spending_goals: Vec<models::ProcessedSpendingGoal> = Vec::new();
    let fixed_financial_state_changes =
        fin_state_influencers.financial_state_changes_from_fixed_sources(&current_date);
    let prioritised_optional_spending_goals =
        fin_state_influencers.prioritised_optional_spending_goals();
    let income_from_undesirable_sources = fin_state_influencers.income_from_undesirable_sources();

    let mut last_successful_forecast: Option<FinancialForecast> = None;

    let mut bisection = BisectionPoints::initial_point(&fin_state_influencers);
    let mut iteration = 0;
    loop {
        // ****
        // **** STEP 1: Set up the params for an iteration
        // ****
        iteration += 1;
        let final_merged_fin_state_changes: FinancialStateChanges =
            fixed_financial_state_changes.clone();

        // Create base fin state changes without spending goals
        let current_income_from_undesirable_sources = bisect_vectors_from_undesirable_income(
            &income_from_undesirable_sources,
            bisection.middle(),
        );
        let base_fin_state_changes =
            current_income_from_undesirable_sources.merge(&fixed_financial_state_changes);

        // This cashflow holds the current cashflow vector as we loop through the spending goals and subtract
        // the their `FinancialStateChanges` from the cashflow
        let mut cash_balance = cum_sum(&base_fin_state_changes.cashflow);

        // If the cash_balance is not valid (i.e has parts of overdraft) then we exist the loop
        if !are_balances_valid(&cash_balance) {
            return Err(ProcessFinancialStateInfluencersErrors::MonthlySpendingExceedsIncome);
        }

        // ****
        // **** STEP 2: Process Spending goals ****
        // ****

        // Here we process the spending goals by looping throught the spending goals and applying them to the current_cashflow.
        let mut all_spending_goals_achievable: bool = true;
        for spending_goal in &prioritised_optional_spending_goals {
            let fin_state_changes = spending_goal.generate_financial_state_changes(&current_date);

            // Temp holder of the merged cash balance vector. Will set the base vector once
            // we check that this sending goal does not cause and overdraft
            let _cash_balance = add(&cash_balance, &cum_sum(&fin_state_changes.cashflow));
            let processed_spending_goal = ProcessedSpendingGoal::from_spending_goal(
                spending_goal,
                &_cash_balance,
                &current_date,
            );

            spending_goals.push(processed_spending_goal.clone());

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

        // If we failed to achieve all the spending goals after the first attempt there is no point trying
        // to bisect
        if !all_spending_goals_achievable && iteration == 1 {
            break;
        }

        let financial_forecast = FinancialForecast {
            all_spending_goals_achievable: all_spending_goals_achievable,
            spending_goals: spending_goals.to_vec(),
            financial_independence_date: date_after_months(&current_date, bisection.middle()),
            monthly_cashflow_deltas: final_merged_fin_state_changes.cashflow.to_vec(),
            monthly_networth: minus(
                &cum_sum(&final_merged_fin_state_changes.assets_delta),
                &cum_sum(&final_merged_fin_state_changes.liabilities_delta),
            ),
        };

        // store the current bisection point
        let _bisection_point = bisection.middle();
        last_successful_forecast = match last_successful_forecast {
            None => Some(financial_forecast),
            Some(last_forecast) => {
                if last_forecast.was_improved_by(&financial_forecast) {
                    bisection = match bisection.new_bisection_point(&fin_state_influencers, true) {
                        Some(bisection) => bisection,
                        None => {
                            last_successful_forecast = Some(last_forecast);
                            break;
                        }
                    };
                    Some(financial_forecast)
                } else {
                    bisection = match bisection.new_bisection_point(&fin_state_influencers, false) {
                        Some(bisection) => bisection,
                        None => {
                            last_successful_forecast = Some(last_forecast);
                            break;
                        }
                    };
                    Some(last_forecast)
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
