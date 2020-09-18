use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OnceOffExpense {
    pub name: String,
    pub value: f64,
    pub date: DateTime<Utc>,
}

impl OnceOffExpense {
    pub fn generate_expense_vector(&self) -> Vec<f64> {
        return vec![];
    }

    pub fn monthly_value_for_date(&self, date: Date<Utc>) -> f64 {
        if date.year() == self.date.year() && date.month() == self.date.month() {
            return self.value;
        }
        return 0.0;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct MonthlyExpense {
    pub name: String,
    pub value: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl MonthlyExpense {
    pub fn generate_expense_vector(&self) -> Vec<f64> {
        return vec![];
    }

    pub fn monthly_value_for_date(&self, date: Date<Utc>) -> f64 {
        if date.year() >= self.start_date.year() && date.month() >= self.start_date.month() {
            if date.year() <= self.end_date.year() && date.month() <= self.end_date.month() {
                return self.value;
            }
        }
        return 0.0;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OnceOffIncome {
    pub name: String,
    pub value: f64,
    pub date: DateTime<Utc>,
}

impl OnceOffIncome {
    pub fn generate_income_vector(&self) -> Vec<f64> {
        return vec![];
    }

    pub fn monthly_value_for_date(&self, date: Date<Utc>) -> f64 {
        if date.year() == self.date.year() && date.month() == self.date.month() {
            return self.value;
        }
        return 0.0;
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct MonthlyIncome {
    pub name: String,
    pub value: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl MonthlyIncome {
    pub fn generate_income_vector(&self) -> Vec<f64> {
        return vec![];
    }

    pub fn monthly_value_for_date(&self, date: Date<Utc>) -> f64 {
        if date.year() >= self.start_date.year() && date.month() >= self.start_date.month() {
            if date.year() <= self.end_date.year() && date.month() <= self.end_date.month() {
                return self.value;
            }
        }
        return 0.0;
    }
}

pub trait Asset {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct SavingsAccount {
    name: String,
    value: f64,
}

impl Asset for SavingsAccount {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Investment {}
