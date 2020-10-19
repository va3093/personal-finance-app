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
        let mut base: FinancialStateChanges;
        let ref_changes: &FinancialStateChanges;
        if self.cashflow.len() > other.cashflow.len() {
            base = self.clone();
            ref_changes = other;
        } else {
            ref_changes = self;
            base = other.clone();
        }
        for i in 0..ref_changes.cashflow.len() {
            base.cashflow[i] += ref_changes.cashflow[i];
            base.assets_delta[i] += ref_changes.assets_delta[i];
            base.liabilities_delta[i] += ref_changes.liabilities_delta[i];
        }
        return base;
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
    let merged_changes_3 = FinancialStateChanges::merge_changes(&vec![fin_changes_1, fin_changes_2]);
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
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct SpendingGoal {}

impl FinancialStateInfluencer for SpendingGoal {
    fn generate_financial_state_changes(&self) -> FinancialStateChanges {
        return FinancialStateChanges::new();
    }
}

/// A processed spending goal is a goal that has been processed in light of the overall financial situation
/// and has been marked as beeing achievable or not
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Copy, Clone)]
pub struct ProcessedSpendingGoal {
    pub is_achieveable: bool,
}

impl ProcessedSpendingGoal {
    pub fn from_spending_goal(spending_goal: &SpendingGoal, cash_balance_vec: &Vec<f64>) -> Self {
        return ProcessedSpendingGoal {
            is_achieveable: true,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct MonthlyExpense {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct MonthlyIncome {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct OnceOffIncome {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Savings {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Asset {}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub struct Liability {}

/// A FinancialStateInfluencer is anything that will impact our financial state in the future.
/// Financial state in this sense refers to our available cash as well as assets and liabilities.
/// This includes anything from expected incomes to expected expenses, but also includes things
pub trait FinancialStateInfluencer {
    fn generate_financial_state_changes(&self) -> FinancialStateChanges;
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
    pub fn VECTOR_LENGTH() -> i16 {
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
        return vec![];
    }

    /// These are a list of `FinancialStateChanges` that result from `FinancialStateInfluencers` that are
    /// not able to change in the algorithm. For example, grocery `MonthlyExpenses` are seen as fixed and
    /// will not be changed by the algorithm. In contrast, the `SpendingGoals` are seen as dynamic and will
    /// only impact the final `FinancialForecast` if we can afford it at the time we wish to buy it.
    pub fn financial_state_changes_from_fixed_sources(
        &self,
    ) -> Vec<Box<dyn FinancialStateInfluencer>> {
        return vec![];
    }

    /// Income from sources that we engage in because we need the money are marked as such. This function
    /// returns the `FinancialStateChanges` from those incomes.
    pub fn income_from_undesirable_sources(&self) -> Vec<FinancialStateChanges> {
        return vec![];
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
    pub financial_independence_age: NaiveDate,
    pub all_spending_goals_achievable: bool,
}

impl FinancialForecast {
    pub fn was_improved_by(&self, other: &FinancialForecast) -> bool {
        return false;
    }
}
