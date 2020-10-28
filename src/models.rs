use crate::utils::{add, date_after_months};

use super::utils::cum_sum;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

/// FinancialStateChanges represent a set of delta vectors that describe how the
/// overall vectors will be impacted by some event. For example Buying a house could be represented
/// in the following way as a FinancialStateChanges.
/// ```
/// use personal_finance::models::FinancialStateChanges;
///   FinancialStateChanges {
///       cashflow: vec![-10000.0, -10000.0, -10000.0, -10000.0, -10000.0, -10000.0 , 65000.0],
///       assets_delta: vec![100000.0, 1000.0, 1000.0, 1000.0, 1000.0, 1000.0, -105000.0],
///       liabilities_delta: vec![90000.0, -10000.0, -10000.0, -10000.0, -10000.0, -10000.0, -40000.0 ],
///   };
///```
/// As you can see in this example a person buys a house worth 100000 with a 10% deposit.
/// He then pays of the mortgage at 10000 a month at 0% interest (thereby decreasing the liability).
/// The house increases in value every month by 1000. And at the end he sells the house which pays of
/// the res of the mortage and deposits the rest in his cashflow
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct FinancialStateChanges {
    pub cashflow: Vec<f64>,
    pub assets_delta: Vec<f64>,
    pub liabilities_delta: Vec<f64>,
}

impl FinancialStateChanges {
    pub fn new() -> Self {
        return Self {
            cashflow: vec![],
            assets_delta: vec![],
            liabilities_delta: vec![],
        };
    }

    pub fn empty() -> Self {
        return Self {
            cashflow: vec![],
            assets_delta: vec![],
            liabilities_delta: vec![],
        };
    }

    pub fn merge_changes(changes: &Vec<FinancialStateChanges>) -> FinancialStateChanges {
        let mut merged_changes = FinancialStateChanges::empty();
        for change in changes {
            merged_changes = merged_changes.merge(change);
        }

        return merged_changes;
    }

    pub fn merge(&self, other: &FinancialStateChanges) -> FinancialStateChanges {
        return FinancialStateChanges {
            cashflow: add(&self.cashflow, &other.cashflow),
            assets_delta: add(&self.assets_delta, &other.assets_delta),
            liabilities_delta: add(&self.assets_delta, &other.liabilities_delta),
        };
    }
}

#[test]
fn test_merge() {
    let fin_changes_1 = FinancialStateChanges {
        cashflow: vec![1.0, 2.0, 3.0],
        assets_delta: vec![1.0, 2.0, 3.0],
        liabilities_delta: vec![1.0, 2.0, 3.0],
    };
    let fin_changes_2 = FinancialStateChanges {
        cashflow: vec![1.0, 2.0],
        assets_delta: vec![1.0, 2.0],
        liabilities_delta: vec![1.0, 2.0],
    };
    let merged_changes_1 = fin_changes_1.merge(&fin_changes_2);
    let merged_changes_2 = fin_changes_2.merge(&fin_changes_1);
    let merged_changes_3 =
        FinancialStateChanges::merge_changes(&vec![fin_changes_1, fin_changes_2]);
    let expected = FinancialStateChanges {
        cashflow: vec![2.0, 4.0, 3.0],
        assets_delta: vec![2.0, 4.0, 3.0],
        liabilities_delta: vec![2.0, 4.0, 3.0],
    };

    assert_eq!(merged_changes_1, expected);
    assert_eq!(merged_changes_2, expected);
    assert_eq!(merged_changes_3, expected);
}

/// A spending goal is something that you wish to purchase in the future. These payments are once off.
/// An important thing to note about spending goals are that they are seen as optional. i.e. They are
/// are things you hope you have enough money to buy in the future.
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct SpendingGoal {
    name: String,
    date: NaiveDate,
    item_purchased: Purchase,
}

fn months_to_date(date: &NaiveDate, from_date: &NaiveDate) -> i32 {
    let number_of_months_to_purchase =
        (date.year() - from_date.year()) * 12 + (date.month() as i32 - from_date.month() as i32);
    return number_of_months_to_purchase;
}

#[test]
fn test_months_to_date() {
    let to_date = NaiveDate::from_ymd(2020, 6, 15);
    let from_date = NaiveDate::from_ymd(2020, 1, 15);

    assert_eq!(months_to_date(&to_date, &from_date), 5);
}

impl FinancialStateInfluencer for SpendingGoal {
    fn generate_financial_state_changes(&self, current_date: &NaiveDate) -> FinancialStateChanges {
        return self
            .item_purchased
            .generate_financial_state_changes(current_date);
    }
}

/// A processed spending goal is a goal that has been processed in light of the overall financial situation
/// and has been marked as beeing achievable or not
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub struct ProcessedSpendingGoal {
    pub is_achieveable: bool,
    pub original_spending_goal: SpendingGoal,
}

impl ProcessedSpendingGoal {
    pub fn from_spending_goal(
        spending_goal: &SpendingGoal,
        cash_balance_vec: &Vec<f64>,
        current_date: &NaiveDate,
    ) -> Self {
        let fin_state_changes = spending_goal.generate_financial_state_changes(&current_date);
        let mut is_achievable = true;
        for (index, cash_delta) in fin_state_changes.cashflow.iter().enumerate() {
            if cash_balance_vec.len() > index && cash_balance_vec[index] - cash_delta < 0.0 {
                is_achievable = false;
            }
        }
        return ProcessedSpendingGoal {
            is_achieveable: is_achievable,
            original_spending_goal: spending_goal.clone(),
        };
    }
}

#[test]
fn test_create_processed_spending_goal_from_spending_goal() {
    let cash_balance_vec = vec![10.0, 10.0, 10.0];
    let current_date = NaiveDate::from_ymd(2020, 1, 15);
    let once_off_purchase = Purchase::once_off_purchase {
        purchase_date: NaiveDate::from_ymd(2020, 3, 15),
        value: 1.0,
    };
    let mut spending_goal = SpendingGoal {
        name: "blah".to_string(),
        date: NaiveDate::from_ymd(2020, 3, 15),
        item_purchased: once_off_purchase,
    };
    let achievable_spending_goal =
        ProcessedSpendingGoal::from_spending_goal(&spending_goal, &cash_balance_vec, &current_date);

    // Increase value of item
    spending_goal.item_purchased = Purchase::once_off_purchase {
        purchase_date: NaiveDate::from_ymd(2020, 3, 15),
        value: 100.0,
    };
    let unachievable_spending_goal =
        ProcessedSpendingGoal::from_spending_goal(&spending_goal, &cash_balance_vec, &current_date);

    assert_eq!(achievable_spending_goal.is_achieveable, true);
    assert_eq!(unachievable_spending_goal.is_achieveable, false);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct MonthlyExpense {
    name: String,
    amount: f64,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl FinancialStateInfluencer for MonthlyExpense {
    fn generate_financial_state_changes(&self, current_date: &NaiveDate) -> FinancialStateChanges {
        let vector_size = months_to_date(&self.end_date, &current_date);
        let months_to_start_date = months_to_date(&self.start_date, &current_date);
        let mut cash_delta: Vec<f64> = vec![];
        for i in 0..vector_size {
            if i > months_to_start_date {
                cash_delta.push(-self.amount)
            }
        }
        return FinancialStateChanges {
            cashflow: cash_delta,
            assets_delta: vec![],
            liabilities_delta: vec![],
        };
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct MonthlyIncome {
    undesireable_source: bool,
    name: String,
    amount: f64,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl FinancialStateInfluencer for MonthlyIncome {
    fn generate_financial_state_changes(&self, current_date: &NaiveDate) -> FinancialStateChanges {
        let vector_size = months_to_date(&self.end_date, &current_date);
        let months_to_start_date = months_to_date(&self.start_date, &current_date);
        let mut cash_delta: Vec<f64> = vec![];
        for i in 0..vector_size {
            if i > months_to_start_date {
                cash_delta.push(self.amount)
            }
        }
        return FinancialStateChanges {
            cashflow: cash_delta,
            assets_delta: vec![],
            liabilities_delta: vec![],
        };
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OnceOffIncome {
    name: String,
    amount: f64,
    date: NaiveDate,
}

impl FinancialStateInfluencer for OnceOffIncome {
    fn generate_financial_state_changes(&self, current_date: &NaiveDate) -> FinancialStateChanges {
        let vector_size = months_to_date(&self.date, &current_date);
        let mut cash_delta: Vec<f64> = vec![0.0; vector_size as usize];
        if vector_size > 0 {
            cash_delta[vector_size as usize - 1] = self.amount;
        }
        return FinancialStateChanges {
            cashflow: cash_delta,
            assets_delta: vec![],
            liabilities_delta: vec![],
        };
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Savings {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
pub enum AppreciationType {
    /// Applies some scalar to the value of the asset **per month**
    Linear { scalar: f64 },
}

impl AppreciationType {
    pub fn value_unit_vector(&self, vector_size: i16) -> Vec<f64> {
        match self {
            Self::Linear { scalar } => {
                let mut vec: Vec<f64> = vec![];
                let mut value = 1.0;
                for _ in 0..vector_size {
                    value *= scalar;
                    vec.push(value);
                }
                return vec;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[allow(non_camel_case_types)]
pub enum Purchase {
    once_off_purchase {
        purchase_date: NaiveDate,
        value: f64,
    },
    asset {
        asset: Asset,
    },
}

impl Purchase {
    fn generate_financial_state_changes(&self, current_date: &NaiveDate) -> FinancialStateChanges {
        match self {
            Self::asset { asset } => return asset.generate_financial_state_changes(current_date),
            Self::once_off_purchase {
                purchase_date,
                value,
            } => {
                let number_of_months_to_purchase = months_to_date(purchase_date, &current_date);
                let mut cashflow_from_purchase =
                    vec![0.0; number_of_months_to_purchase as usize + 1];
                let cashflow_from_purchase_size = cashflow_from_purchase.len();
                if cashflow_from_purchase_size > 0 {
                    cashflow_from_purchase[cashflow_from_purchase_size - 1] = -value.clone();
                }
                let fin_changes = FinancialStateChanges {
                    cashflow: cashflow_from_purchase,
                    assets_delta: vec![],
                    liabilities_delta: vec![],
                };
                return fin_changes;
            }
        }
    }
}

#[test]
fn test_once_off_purchase_fin_state_changes() {
    let current_date = NaiveDate::from_ymd(2020, 1, 15);
    let once_off_purchase = Purchase::once_off_purchase {
        purchase_date: NaiveDate::from_ymd(2020, 3, 15),
        value: 100.0,
    };

    let expected_fin_changes = FinancialStateChanges {
        cashflow: vec![0.0, 0.0, -100.0],
        assets_delta: vec![],
        liabilities_delta: vec![],
    };
    assert_eq!(
        once_off_purchase.generate_financial_state_changes(&current_date),
        expected_fin_changes
    )
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Clone)]
#[allow(non_camel_case_types)]
pub enum Asset {
    general_asset {
        purchase_date: NaiveDate,
        name: String,
        initial_value: f64,
        appreciation_type: AppreciationType,
        liquidation_date: NaiveDate,
    },
}

impl Asset {
    fn generate_financial_state_changes(&self, current_date: &NaiveDate) -> FinancialStateChanges {
        match self {
            Self::general_asset {
                purchase_date,
                initial_value,
                appreciation_type,
                liquidation_date,
                ..
            } => {
                let number_of_months_to_purchase = months_to_date(&purchase_date, &current_date);
                let number_of_months_to_liquidation_date =
                    months_to_date(liquidation_date, &current_date);

                // Calculate asset value
                let mut asset_value = vec![0.0; number_of_months_to_liquidation_date as usize];
                let value_unit_vector = appreciation_type.value_unit_vector(
                    number_of_months_to_liquidation_date as i16
                        - number_of_months_to_purchase as i16,
                );

                let mut value_of_asset_at_sale = initial_value.clone();
                for (index, unit_scalar) in value_unit_vector.iter().enumerate() {
                    // Add the diff between last months value and the current value to calculate how much
                    // this month is contributing to the value of the asset
                    if index == 0 {
                        // At the month of the purchase we add the value of the sale and the value that it will appreciate
                        // that month
                        asset_value[number_of_months_to_purchase as usize + index] =
                            initial_value * unit_scalar;
                    } else {
                        asset_value[number_of_months_to_purchase as usize + index] =
                            initial_value * unit_scalar - value_of_asset_at_sale;
                    }

                    value_of_asset_at_sale = initial_value * unit_scalar;
                }

                // on the month of liquidation the value of the asset will be subtracted from the assets
                asset_value[number_of_months_to_liquidation_date as usize - 1] =
                    -value_of_asset_at_sale;

                // Calculate cashflow
                let mut cashflow = vec![0.0; number_of_months_to_liquidation_date as usize];
                cashflow[number_of_months_to_liquidation_date as usize - 1] =
                    value_of_asset_at_sale;
                cashflow[number_of_months_to_purchase as usize] = -initial_value;

                return FinancialStateChanges {
                    cashflow: cashflow,
                    assets_delta: asset_value,
                    liabilities_delta: vec![0.0; number_of_months_to_liquidation_date as usize],
                };
            }
        }
    }
}

#[test]
fn test_general_asset_generate_financial_state_changes() {
    let current_date = NaiveDate::from_ymd(2020, 1, 15);
    let general_asset = Asset::general_asset {
        liquidation_date: NaiveDate::from_ymd(2020, 6, 15),
        purchase_date: NaiveDate::from_ymd(2020, 3, 15),
        initial_value: 100.0,
        name: "test".to_string(),
        appreciation_type: AppreciationType::Linear { scalar: 1.1 },
    };

    let expected_fin_changes = FinancialStateChanges {
        cashflow: vec![0.0, 0.0, -100.0, 0.0, 133.10000000000005],
        assets_delta: vec![0.0, 0.0, 110.000000000000014, 11.0, -133.10000000000005],
        liabilities_delta: vec![0.0; 5],
    };
    assert_eq!(
        general_asset.generate_financial_state_changes(&current_date),
        expected_fin_changes
    )
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Liability {}

/// A FinancialStateInfluencer is anything that will impact our financial state in the future.
/// Financial state in this sense refers to our available cash as well as assets and liabilities.
/// This includes anything from expected incomes to expected expenses, but also includes things
pub trait FinancialStateInfluencer {
    fn generate_financial_state_changes(&self, current_date: &NaiveDate) -> FinancialStateChanges;
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct FinancialStateInfluencers {
    pub max_retirement_age: i8,
    date_of_birth: NaiveDate,
    spending_goals: Vec<SpendingGoal>,
    monthly_expenses: Vec<MonthlyExpense>,
    monthly_income: Vec<MonthlyIncome>,
    once_off_income: Vec<OnceOffIncome>,
    current_savings: Savings,
    current_assets: Vec<Asset>,
    current_liabilities: Vec<Liability>,
}

impl FinancialStateInfluencers {
    #[inline]
    pub fn vector_length() -> i16 {
        12 * 100
    }

    pub fn months_to_max_retirment(self: &Self, today: NaiveDate) -> i16 {
        let months_to_next_birthday =
            (today.month() as i16 - self.date_of_birth.month() as i16) % 12;
        let age = (today.year() - self.date_of_birth.year()) as i16;
        return (self.max_retirement_age as i16 - age) * 12 - months_to_next_birthday;
    }

    pub fn age_at_bisection_point(&self, bisection_point: i16) -> NaiveDate {
        return NaiveDate::from_ymd(2011, 1, 1);
    }

    pub fn prioritised_optional_spending_goals(&self) -> Vec<SpendingGoal> {
        return self.spending_goals.clone();
    }

    /// These are a list of `FinancialStateChanges` that result from `FinancialStateInfluencers` that are
    /// not able to change in the algorithm. For example, grocery `MonthlyExpenses` are seen as fixed and
    /// will not be changed by the algorithm. In contrast, the `SpendingGoals` are seen as dynamic and will
    /// only impact the final `FinancialForecast` if we can afford it at the time we wish to buy it.
    pub fn financial_state_changes_from_fixed_sources(
        &self,
        current_date: &NaiveDate,
    ) -> FinancialStateChanges {
        let mut base = FinancialStateChanges::new();
        for expense in &self.monthly_expenses {
            base = base.merge(&expense.generate_financial_state_changes(&current_date));
        }
        for income in &self.monthly_income {
            if !income.undesireable_source {
                base = base.merge(&income.generate_financial_state_changes(&current_date));
            };
        }
        for income in &self.once_off_income {
            base = base.merge(&income.generate_financial_state_changes(&current_date));
        }
        return base;
    }

    /// Income from sources that we engage in because we need the money are marked as such. This function
    /// returns the `FinancialStateChanges` from those incomes.
    pub fn income_from_undesirable_sources(&self) -> FinancialStateChanges {
        return FinancialStateChanges {
            cashflow: vec![],
            assets_delta: vec![],
            liabilities_delta: vec![],
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn months_to_max_retirment() {
        let fin_state_infl = FinancialStateInfluencers {
            max_retirement_age: 70,
            date_of_birth: NaiveDate::from_ymd(1990, 1, 2),
            spending_goals: vec![],
            monthly_expenses: vec![],
            monthly_income: vec![],
            once_off_income: vec![],
            current_savings: Savings {},
            current_assets: vec![],
            current_liabilities: vec![],
        };

        assert!(fin_state_infl.months_to_max_retirment(NaiveDate::from_ymd(2020, 10, 19)) == 471);
    }
}

/// The FinacialForecast is the result of simulating all your future FinancialStateInfluencers.
/// It shows the fluctuations of your cashflow and networth. It points out which of your financial
/// goals you will not be able to afford and at what point you will have acquired enough wealth to no
// longer need to depend for undesirable sources of income.
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct FinancialForecast {
    pub spending_goals: Vec<ProcessedSpendingGoal>,
    pub monthly_cashflow_deltas: Vec<f64>,
    pub monthly_networth: Vec<f64>,
    pub financial_independence_date: NaiveDate,
    pub all_spending_goals_achievable: bool,
}

impl FinancialForecast {
    pub fn cash_balances(&self) -> Vec<f64> {
        return cum_sum(&self.monthly_cashflow_deltas);
    }

    pub fn dates_series(&self, current_date: &NaiveDate) -> Vec<NaiveDate> {
        let mut dates: Vec<NaiveDate> = vec![current_date.clone()];
        for i in 1..self.monthly_cashflow_deltas.len() {
            dates.push(date_after_months(current_date, i as i16))
        }
        return dates;
    }

    pub fn contains_period_of_overdraft(&self) -> bool {
        let cash_balances = self.cash_balances();

        // If the other FinancialForecast goes into overdraft at any point it is not worth considering
        for balance in cash_balances {
            if balance < 0.0 {
                return true;
            }
        }
        return false;
    }

    pub fn was_improved_by(&self, other: &FinancialForecast) -> bool {
        let current_cash_balances = self.cash_balances();
        let other_cash_balances = other.cash_balances();

        // If the other FinancialForecast goes into overdraft at any point it is not worth considering
        if other.contains_period_of_overdraft() {
            return false;
        }

        // The Financial forecast that produces the lowest positive balance at the end of life is considered
        // to be better

        return other_cash_balances.last() < current_cash_balances.last();
    }
}
