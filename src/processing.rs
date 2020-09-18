use super::data;
use super::models;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct PeriodicFinancialData {
    pub monthly_expenses: Vec<f64>,
    pub monthly_income: Vec<f64>,
    pub monthly_asset_delta: Vec<f64>,
    pub monthly_liability_delta: Vec<f64>,
}

impl PeriodicFinancialData {
    pub fn empty() -> Self {
        return PeriodicFinancialData {
            monthly_expenses: vec![],
            monthly_income: vec![],
            monthly_asset_delta: vec![],
            monthly_liability_delta: vec![],
        };
    }

    pub fn filled(value: f64, size: usize) -> Self {
        return PeriodicFinancialData {
            monthly_expenses: vec![value; size],
            monthly_income: vec![value; size],
            monthly_asset_delta: vec![value; size],
            monthly_liability_delta: vec![value; size],
        };
    }

    fn from_financial_state_influencers(
        financial_state_influencers: FinancialStateInfluencers,
    ) -> PeriodicFinancialData {
        return PeriodicFinancialData::empty();
    }

    pub fn monthly_closing_balance(&self) -> Vec<f64> {
        return vec![];
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct FinancialStateInfluencers {
    pub once_off_expenses: Vec<models::OnceOffExpense>,
    pub monthly_expenses: Vec<models::MonthlyExpense>,
    pub once_off_incomes: Vec<models::OnceOffIncome>,
    pub monthly_incomes: Vec<models::MonthlyIncome>,
    pub investments: Vec<models::Investment>,
}

impl FinancialStateInfluencers {
    pub fn final_relavent_date(&self) -> DateTime<Utc> {
        let mut final_date = Utc.ymd(0, 1, 1).and_hms(0, 0, 0);
        for once_off_expense in &self.once_off_expenses {
            if once_off_expense.date > final_date {
                final_date = once_off_expense.date.clone();
            }
        }
        for monthly_expense in &self.monthly_expenses {
            if monthly_expense.end_date > final_date {
                final_date = monthly_expense.end_date.clone();
            }
        }
        for once_off_income in &self.once_off_incomes {
            if once_off_income.date > final_date {
                final_date = once_off_income.date.clone();
            }
        }
        for monthly_income in &self.monthly_incomes {
            if monthly_income.end_date > final_date {
                final_date = monthly_income.end_date.clone();
            }
        }
        return final_date;
    }
}

pub fn generate_future_finacial_status(
    financial_state_influencers: FinancialStateInfluencers,
    starting_date: DateTime<Utc>,
) -> PeriodicFinancialData {
    let end_date = financial_state_influencers.final_relavent_date();
    // Calculate number of months between starting date and final date
    let vector_length = (end_date.year() - starting_date.year()) * 12 + end_date.month() as i32
        - starting_date.month() as i32
        + 1; // Add one because if the end date and start date are the same we still want to count that month
    println!("{}", vector_length);
    println!("{}", starting_date);
    println!("{}", end_date);
    let mut fin_data = PeriodicFinancialData::filled(0.0, vector_length as usize);
    for month in (0..vector_length) {
        let ref_date = Utc.ymd(
            starting_date.year() + (month / 12) as i32,
            starting_date.month() + (month % 12) as u32,
            starting_date.day(),
        );
        for monthly_expense in &financial_state_influencers.monthly_expenses {
            fin_data.monthly_expenses[month as usize] +=
                monthly_expense.monthly_value_for_date(ref_date);
        }
        for once_off_income in &financial_state_influencers.once_off_incomes {
            fin_data.monthly_income[month as usize] +=
                once_off_income.monthly_value_for_date(ref_date);
        }
        for monthly_income in &financial_state_influencers.monthly_incomes {
            fin_data.monthly_income[month as usize] +=
                monthly_income.monthly_value_for_date(ref_date);
        }
        for once_off_expense in &financial_state_influencers.once_off_expenses {
            fin_data.monthly_expenses[month as usize] +=
                once_off_expense.monthly_value_for_date(ref_date);
        }
    }
    return fin_data;
}

#[cfg(test)]
mod tests {
    fn test_creating_periodid_financial_data_from_influencers() {}
}
