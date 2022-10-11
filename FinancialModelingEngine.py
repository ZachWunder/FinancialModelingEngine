# Financial Modeling Engine
import pandas as pd
import locale
from bokeh.layouts import row
from bokeh.plotting import figure, output_file, show, curdoc
from bokeh.models import ColumnDataSource, NumeralTickFormatter
from bokeh.models import HoverTool
bokeh_doc = curdoc()
# Loop Conditions
yearsToRun = 50
monthsToRun = yearsToRun * 12
# Initialize Base Conditions
# Read in Exterior Cash Flow
cashflow = pd.read_csv('./cashflow.csv')
cashflow.set_index('From')
# Read in Portfolio Base (assets)
portfolioAssets = pd.read_csv('./portfolioAssets.csv')
portfolioAssets.set_index('Month')
monthlyAppreciation = 1.0083333
# Read in Events
events = pd.read_csv('./events.csv')
debtYearlyInterest = .025
debtMonthlyInterest = debtYearlyInterest / 12

debtDf = pd.DataFrame()

negativeCashFlowCounter = 0
# Main Loop
for month in range(0, monthsToRun):
  # Handle events
  currentEvents = events[(events['Month'] == month)]
  if (currentEvents.empty == False):
    # Update cashflow, debt, assets from Event
    for index, event in currentEvents.iterrows():
      # Cashflow
      if (event.CashFlow != 0):
        newCashFlow = pd.DataFrame([{'Month': month, 'From': event.Name, 'Amount': event.CashFlow}])
        cashflow = pd.concat([cashflow, newCashFlow])
      # Assets
      if (event.Asset == 1):
        newAsset = pd.DataFrame([{'Name': event.Name, 'Value': event.Debt}])
        portfolioAssets = pd.concat([portfolioAssets, newAsset])
      # Debt
      newDebt = int(event.Debt)
      if (newDebt > 0):
        # Add new debt
        debtExpenses = pd.DataFrame([{
          'From': f'{event.name} Debt',
          'Amount': -debtMonthlyInterest * newDebt,
          'Month': month
        }])
        logDebt = pd.DataFrame([{'Month': month, 'Amount': newDebt}])
        debtDf = pd.concat([debtDf, logDebt])
        cashflow = pd.concat([cashflow, debtExpenses])

  # Calculate New Portfolio Values
  # newInterestSeries = portfolioAssets['Value'].multiply(portfolioAssets['MonthlyAppreciation'])
  appreciation = portfolioAssets['Value'].multiply(monthlyAppreciation).sum() - portfolioAssets['Value'].sum()
  a = pd.DataFrame([{
    'Month': month,
    'Name': f'Appreciation {month}',
    'Value': appreciation
  }])
  portfolioAssets = pd.concat([portfolioAssets, a])
  
  # Divert excess cash flow to securities
  cashFlow = cashflow['Amount'].sum()
  if (cashFlow > 0):
    newSecurities = pd.DataFrame([{'Month': month,'Name': f'Securities {month}', 'Value': cashFlow}])
    portfolioAssets = pd.concat([portfolioAssets, newSecurities])
  else:
    negativeCashFlowCounter += 1


# Logging
# print(f'Negative Cash Flows: {negativeCashFlowCounter}')
#   # Throw warning if cash flow goes negative
# print(cashflow)
# print(portfolioAssets)
# locale.setlocale( locale.LC_ALL, 'en_US.UTF-8' )
# print(locale.currency(portfolioAssets['Value'].sum(), grouping=True))

sum_axis = portfolioAssets.groupby(['Month']).sum(numeric_only=True).cumsum()
months_axis = portfolioAssets['Month'].unique()
# Plots
source = ColumnDataSource(portfolioAssets)
# create plot for portfolio value
portfolioValueFigure = figure(title="Portfolio Asset Value", x_axis_label='Months', y_axis_label='Portfolio Value', tools='pan')
crosshair = HoverTool(tooltips=[("Month", "$x"), ("Value", "$y{$0,0}")], formatters={'value': 'numeral'})
portfolioValueFigure.add_tools(crosshair)
portfolioValueFigure.yaxis[0].formatter = NumeralTickFormatter(format="$0,0")
portfolioValueFigure.line(months_axis, sum_axis['Value'], legend_label="Value of Portfolio", line_width=2)

debt_axis = debtDf['Month']
# create plot for debt
debtFigure = figure(title="Debt", x_axis_label='Month', y_axis_label='Debt Amount', tools='pan')
crosshair = HoverTool(tooltips=[("Month", "$x"), ("Value", "$y{$0,0}")], formatters={'value': 'numeral'})
debtFigure.add_tools(crosshair)
debtFigure.yaxis[0].formatter = NumeralTickFormatter(format="$0,0")
debtFigure.line(debt_axis, debtDf['Amount'].cumsum(), legend_label="Debt Amount", line_width=2)

cashflow_axis = cashflow['Month'].unique()
cashflow_y_axis = cashflow.groupby(['Month']).sum(numeric_only=True).cumsum()['Amount']
print(cashflow_y_axis)
# create plot for cashflow
cashflowFigure = figure(title="Pre-investment Cashflow", x_axis_label='Month', y_axis_label='Cashflow', tools='pan')
crosshair = HoverTool(tooltips=[("Month", "$x"), ("Value", "$y{$0,0}")], formatters={'value': 'numeral'})
cashflowFigure.add_tools(crosshair)
cashflowFigure.yaxis[0].formatter = NumeralTickFormatter(format="$0,0")
cashflowFigure.line(cashflow_axis, cashflow_y_axis, legend_label="Cashflow", line_width=2)


bokeh_doc.add_root(row(portfolioValueFigure, debtFigure, cashflowFigure))




