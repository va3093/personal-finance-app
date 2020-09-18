
class SpendingGoal(FinancialStateInfluencer):
    pass

class ProcessedSpendingGoal(SpendingGoal):
    achievable: bool
    # records how this spending goal impacted cashflow balance
    delta_cashflow: t.List[float]


class FinancialStateChanges:
    cashflow: list
    assets_delta: list
    liability_delta: list

    def merge(self, other: FinancialStateChanges) -> FinancialStateChanges:
        pass


class FinancialStateInfluencer:

    def generate_financial_state_changes(self) -> FinancialStateChanges:
        pass



class FinancialStateInfluencers:
    spending_goals: t.List[SpendingGoal]
    montly_expenses: t.List[MonthlyExpense]
    montly_income: t.List[MonthlyIncome]
    once_off_income: t.List[OnceOffIncome]
    current_savings: Savings
    current_assets: t.List[Assets]
    current_liabilities: t.List[Liability]

    def prioritiesed_optional_spending_goals(self, reverse=False) -> t.List[SpendingGoal]:
        pass

    def income_from_undesirable_sources(self) -> t.List[FinancialStateInfluencer]:
        fin_state_influencers_from_undesirable_source = []
        fin_state_influencers_from_desirable_source = []
        for fin_state_influencer in self.monthly_income:
            if fin_state_influencer.source_is_undesirable:
                fin_state_influencers_from_undesirable_source.append(fin_state_influencer)
            else:
                fin_state_influencers_from_desirable_source.append(fin_state_influencer)

    def financial_state_changes_from_fixed_sources(self)-> t.List[FinancialStateChanges]:
        pass





class FinancialForecast:
    spending_goals: t.List[ProcessedSpendingGoals]
    monthly_cashflow_deltas: t.List[float]
    montly_networth: t.List[float]
    financial_independence_date: int
    all_spending_goals_achievable: bool

# processing.py

def find_overdraft(balance) -> t.Optional[Float]:
    # find first negative balance
    pass

def is_balance_valid(balance) -> bool:
    first_overdraft = find_overdraft(balance)
    return first_overdraft if first_overdraft is not None else None

def bisect_vectors_from_undesirable_income(income_from_undesirable_sources: t.List[FinancialStateInfluencer], bisection_point: int) -> FinancialStateChanges:
    # merge fin_state_changes and replace all values after bisection_point with zeros
    pass

def get_next_bisection_point(current_bisection_point, last_bisection_point, curent_forecast, last_forecast) -> Int:
    ## Return bisection_point else raise FullyBisected
    pass

def process_financial_state_influencers(fin_state_infls: FinancialStateInfluencers) -> FinancialForecast:
    spending_goals_fin_state_impacts = []
    spending_goals = []

    fixed_financial_state_changes = fin_state_infls.financial_state_changes_from_fixed_sources()
    prioritiesed_optional_spending_goals = fin_state_infls.prioritiesed_optional_spending_goals()


    last_successful_forecast = None

    last_bisection_point = 0
    bisection_point = FinancialStateInfluencers.VECTOR_LENGTH:
    while True:

        # Create base fin state changes without spending goals
        income_from_undesirable_sources = bisect_vectors_from_undesirable_income(fin_state_infls.income_from_undesirable_sources, bisection_point)

        # We will update this with the spending goals as the are processed and are valid
        all_fin_state_changes = income_from_undesirable_sources.merge(fixed_financial_state_changes)

        # Intial cash balance does not include spending goals
        cash_balance = all_fin_state_changes.cashflow.cum_sum()

        if !is_balance_valid(cash_balance):
            raise MonthlySpendingExceedsIncome

        all_spending_goals_achievable = True
        for fin_state_infl in prioritiesed_optional_spending_goals:
            # Process the spending goal
            fin_state_change = fin_state_infl.generate_financial_state_changes()
            # temp holder of the merged cash balance vector. Will set the base
            # vector once we check that this spending goal does not cause an overdraft
            _cash_balance = cashflow + fin_state_change.cashflow.cum_sum()
            processed_spending_goal = ProcessedSpendingGoal(..., _cash_balance, is_achievable=!is_balance_valid(_cash_balance))
            spending_goals.append(processed_spending_goal)

            # If the spending goal doesn't cause an overdraft then update the cash balance vector
            if processed_spending_goal.is_achievable:
                cash_balance = _cash_balance
                all_fin_state_changes.merge(fin_state_change)
            else:
                all_spending_goals_achievable = False



        fin_forecast = FinancialForecast(
            all_spending_goals_achievable=all_spending_goals_achievable,
            financial_independence_date=fin_state_infls.age_at_bisection_point(bisection_point),
            monthly_cashflow_deltas=monthly_cashflow_deltas,
            montly_networth=all_fin_state_changes.assets_delta.cum_sum() - all_fin_state_changes.liability_delta.cum_sum(),
            spending_goals=spending_goals
        )

        # If we failed to acheive all the spending goals after the first attempt there is no point trying to bisect
        if not all_spending_goals_achievable and bisection_point == FinancialStateInfluencers.VECTOR_LENGTH:
            break

        # Update parameters of the while loop
        _bisection_point = bisection_point
        try:
            bisection_point = get_next_bisection_point(_bisection_point, last_bisection_point, last_successful_forecast, fin_forecast)
        except FullyBisected:
            break

        last_bisection_point = _bisection_point
        if last_successful_forecast.improved_by(fin_forecast):
            last_successful_forecast = fin_forecast

    return last_successful_forecast
