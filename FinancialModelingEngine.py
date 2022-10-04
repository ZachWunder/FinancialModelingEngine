# Financial Modeling Engine
import pandas as pd
import locale
from bokeh.plotting import figure, show
from bokeh.plotting import figure, output_file, show
from bokeh.models import ColumnDataSource
# Loop Conditions
yearsToRun = 70
monthsToRun = yearsToRun * 12
# Initialize Base Conditions
# Read in Exterior Cash Flow
exteriorCashFlow = pd.read_csv('./test_exteriorCashflow.csv')
exteriorCashFlow.set_index('From')
# Read in Portfolio Base (assets)
portfolioAssets = pd.read_csv('./test_portfolioAssets.csv')
portfolioAssets.set_index('Month')
monthlyAppreciation = 1.0083333
# Read in Events
events = pd.read_csv('./test_events.csv')
debtYearlyInterest = .025
debtMonthlyInterest = debtYearlyInterest / 12

negativeCashFlowCounter = 0
# Main Loop
for month in range(0, monthsToRun):
  # Handle events
  currentEvents = events[(events['Month'] == month)]
  if (currentEvents.empty == False):
    # Update cashflow, debt, assets from Event
    for index, event in currentEvents.iterrows():
      # Cashflow
      if (event.ExternalCashFlow != 0):
        newCashFlow = pd.DataFrame([{'Month': month, 'From': event.Name, 'Amount': event.ExternalCashFlow}])
        exteriorCashFlow = pd.concat([exteriorCashFlow, newCashFlow])
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
        pd.concat([exteriorCashFlow, debtExpenses])

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
  cashFlow = exteriorCashFlow['Amount'].sum()
  if (cashFlow > 0):
    newSecurities = pd.DataFrame([{'Month': month,'Name': f'Securities {month}', 'Value': cashFlow}])
    portfolioAssets = pd.concat([portfolioAssets, newSecurities])
  else:
    negativeCashFlowCounter += 1


# Logging
print(f'Negative Cash Flows: {negativeCashFlowCounter}')
  # Throw warning if cash flow goes negative
print(exteriorCashFlow)
print(portfolioAssets)
locale.setlocale( locale.LC_ALL, 'en_US.UTF-8' )
print(locale.currency(portfolioAssets['Value'].sum(), grouping=True))

# Plots
output_file('test.html')
source = ColumnDataSource(portfolioAssets)
# create a new plot with a title and axis labels
p = figure(title="Portfolio Asset Value", x_axis_label='x', y_axis_label='y')
# add a line renderer with legend and line thickness to the plot
p.line(x='Month', y='Value', source=source, legend_label="Temp.", line_width=2)
show(p)
