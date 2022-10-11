# SBLOC Financial Modeling Tool

## What does this tool do?
Models the outcomes of a financial strategy that utilizes a SBLOC for all liquidity needs. Outputs graphs of portfolio value, cashflow, and debt over time. 

## Architecture
`FinancialModelingEngine.py` contains the modeling engine. Variables like how many `yearsToRun`, `monthlyAssetAppreciation` and `debtYearlyInterest` can be configured here.
`cashflow.csv`,`portfolioassets.csv`,`events.csv` contain the data for the engine to use.
Cashflow describes our intial cashflow. Income and expenses.
PortfolioAssets describes our initial portfolio conditions.
Events describes the purchases and cashflow events you'll make over time. If it is an asset, it'll be added to your portfolio. Any purchases here will be bought with the SBLOC and the interest payments automatically added to your cashflow.

## What's a SBLOC?
Securities-backed line of credit(SBLOC) is a variable-rate, revolving line of credit backed by your securities portfolio. The interest rate is a spread over SOFR(secured overnight financing rate). The spread starts at around 3% APR for portfolios around $300,000 and can go down to as little as 0.5% APR for $5-10M portfolios. The amount you can withdraw is determined by your release percentage. The release percentage for blue chip stocks is around 50% and up to 90% for bonds.

## Why use a SBLOC?
A SBLOC can be used to meet liquidity needs instead of saving up cash. This means you can be continually investing cash.

## To run:
`bokeh serve --show FinancialModelingEngine.py`