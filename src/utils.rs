use chrono::prelude::*;

pub fn cum_sum(vec: &Vec<f64>) -> Vec<f64> {
    let mut res: Vec<f64> = vec![];
    let mut cum_sum: f64 = 0.0;
    for val in vec {
        cum_sum += val;
        res.push(cum_sum)
    }
    return res;
}

pub fn add(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut base: Vec<f64>;
    let ref_vec: &Vec<f64>;
    if vec1.len() > vec2.len() {
        base = vec1.clone();
        ref_vec = vec2;
    } else {
        ref_vec = vec1;
        base = vec2.clone();
    }
    for i in 0..ref_vec.len() {
        base[i] += ref_vec[i];
    }
    return base;
}

pub fn minus(vec1: &Vec<f64>, vec2: &Vec<f64>) -> Vec<f64> {
    let mut res: Vec<f64> = vec![];
    for i in 0..vec1.len() {
        res.push(vec1[i] - vec2[i]);
    }
    return res;
}

pub fn are_balances_valid(balances: &Vec<f64>) -> bool {
    for balance in balances {
        if balance < &0.0 {
            return false;
        }
    }
    return true;
}

pub fn date_after_months(current_date: &NaiveDate, months: i16) -> NaiveDate {
    let number_of_years = (current_date.month() as i16 + months) / 12;
    let remainder = (current_date.month() as i16 + months) % 12;
    let new_year = current_date.year() + number_of_years as i32;
    let new_month = if remainder != 0 { remainder as u32 } else { 12 };
    return NaiveDate::from_ymd(
        if new_month == 12 {
            new_year - 1
        } else {
            new_year
        },
        new_month,
        current_date.day(),
    );
}
