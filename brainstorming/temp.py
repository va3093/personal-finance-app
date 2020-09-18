
class FinancialForecast():

    def save_for_once_off_payment(self, once_off_payment, strategy):
        if self.savings_account_for_once_off_payment(once_off_payment):
            savings_account = self.savings_account_for_once_off_payment(once_off_payment)
        else:
            savings_account = strategy.savings_account_for_once_off_payment(once_off_payment, self.current_month)
        self.savings_accounts.push(savings_account)
        remaining_amount = savings_account.add(self.current_month_surplus)
        self.current_month_surplus = remaining_amount



    def apply_strategy(strategy, fin_events):
        for once_off_payment in fin_events.prioritised_once_off_payments:
            self.save_for_once_off_payment(once_off_payment, strategy)

        for liability in self.liabilities:
            if strategy.reduce_liability(liability, self.current_month)
                self.reduce_liability(liability, strategy)

        for investment in strategy.investments:
            if self.assets.includes(investment):
                if investment.should_liquidate():
                    self.assets[investment.id].liquidate()
            else:
                if investment.should_invest(self.cash_balance_available_for_investment, self.current_month)
                    self.invest(investment)

def calculate_financial_forecast(strategy, fin_events, months_shaved_off):
    let fin_forecast = FinancialForecast()
    let total_number_of_months = fin_events.final_date()
    for month in range(total_months):
        expense = fin_events.expenses_for_month(month)
        if month < total_number_of_months - months_shaved_off:
            income = fin_events.income_for_month(month)
        else:
            income = 0
        fin_forecast.process_income_and_expenses(income, expenses)
        if fin_forecast.closing_balance < 0:
            once_off_expenses_for_month = fin_events.once_off_expenses_for_month(month)
            if once_off_expenses_for_month > 0:
                raise CouldNotPayForExpense(once_off_expenses)
            else:
                raise ProducedDeficit(deficit)

        fin_forecast.apply_strategy(strategy)



def algo() {
    def _calculate_cashflow(strategy, fin_events):
        let total_removed_expenses = []

        try:
            let financial_forecast = calculate_financial_forecast(strategy, financial_events)
        except CouldNotPayForExpense as exp:
            try:
                # Will remove exp as well if the lower priority expenses don't cover the costs of exp
                let (new_fin_events, removed_expenses) = financial_events.remove_minimum_lower_priority_expenses(exp)
                let financial_forecast = _calculate_financial_forecast(strategy, new_fin_events)
            except ProducedDeficit as deficit:
                let (new_fin_events, removed_expenses) = financial_events.remove_minimum_lowest_priority_expenses_before_deficit_occurs(deficit)
                total_removed_expenses += removed_expenses
                if len(removed_expenses) == 0:
                    raise AlgoError("Your income is not high enough to pay your monthly expenses even when removing relevant optional expenses")
                let financial_forecast = _calculate_financial_forecast(strategy, new_fin_events)

        # Now you have a financial_forecast that gets you the most possible OptionalExpenses and runs to the end of your life. Now you want to slowly shave off the months
        # of income produced by labour until your net worth is as close to 0 when you die.

        let final_financial_forecast;
        while True:
            months_shaved_off = 0;
            try:
                final_financial_forecast = calculate_financial_forecast(new_fin_events, months_saved_off)
                months_saved_off += 1;
            except ProducedDeficit as deficit:
                break;
        return final_financial_forecast


    let results_dict = {}
    let financial_events = get_initial_fin_events();
    for strategy in random_strategy_generator():
        let financial_forecast = _calculate_financial_forecast(strategy, financial_events)
        results_dict[strategy] = financial_forecast

    optimimum_financial_forecast = get_best_strategy(results_dict)

    print("""
        Even if you worked all the possible years you have stated you will not be able to afford the following expenses:

        {total_removed_expenses}

        If you apply the following strategy you should be able to stop working at the age of {final_financial_forecast.retirement_age}

    """)
}
