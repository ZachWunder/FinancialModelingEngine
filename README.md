# SBLOC Financial Modeling Tool

## Usage
General Usage:
`fme path/to/inputs/folder {yearly_debt_interest_rate} {yearly_asset_appreciation_rate}`
yearly_debt_interest_rate is optional and defaults to 0.03.
yearly_asset_appreciation_rate is also optional and defaults to 0.10

Example:
`fme inputs/example .04 .08`
`fme inputs/example`

Where the file structure would look like:
```
directory  
│
└───inputs
│   │
│   └───example
│       │   cashflow.csv
│       │   events.csv
│       │   portfolio.csvv
```
See `FME/inputs/example` in this repository for example usage.

## What does this tool do?
Models the outcomes of a financial strategy that utilizes a SBLOC for all liquidity needs. Outputs graphs of portfolio value, cashflow, and debt over time. 

## Architecture
`cashflow.csv`,`portfolioassets.csv`,`events.csv` contain the data for the engine to use.
Cashflow describes intial cashflow. This is income and expenses.
PortfolioAssets describes initial portfolio conditions ie. existing assets.
Events describes the purchases and cashflow events made over time. If it is an asset, it'll be added to the portfolio. Any purchases here will be bought with the SBLOC and the interest payments automatically added to cashflow.

## What's a SBLOC?
Securities-backed line of credit(SBLOC) is a variable-rate, revolving line of credit backed by your securities portfolio. The interest rate is a spread over SOFR(secured overnight financing rate). The spread starts at around 3% APR for portfolios around $300,000 and can go down to as little as 0.5% APR for $5-10M portfolios. The amount available to withdraw is determined by the release percentage. The release percentage for blue chip stocks is around 50% and up to 90% for bonds.

## Why use a SBLOC?
A SBLOC can be used to meet liquidity needs instead of saving up cash. This means excess cashflow can be continually invested and capital gains tax avoided.

## To run the python visualization (deprecated):
`bokeh serve --show FinancialModelingEngine.py`