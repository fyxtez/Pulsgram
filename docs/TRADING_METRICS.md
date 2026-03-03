# Core Trade Metrics (Per Timeframe)

These describe raw behavior.
Total Trades
Resolved Trades
Unresolved Trades
TP1 Hit Rate
TP2 Hit Rate
TP3 Hit Rate
SL Rate
Hard SL Before TP1
Expectancy (TP1-first model)
Total R
Win Streak (max)
Loss Streak (max)
Average Win Streak

# Volatility & Stability Metrics

Mean R (μ)
Standard Deviation of R (σ)
Stability Ratio (μ / σ)
Resolved N (sample size of R values)

# Equity Curve Metrics

Final Equity (R)
Peak Equity (R)
Maximum Drawdown (R)

# Symbol-Level Diagnostics

Symbol Total Trades
Symbol TP1%
Symbol SL%
Symbol Expectancy
P(TP2 | TP1)
P(TP3 | TP2)
P(SL | TP1)
SL After TP1 %
Trade Share %
R Share Signed %
R Share Absolute %
Wilson Lower Bound (95%)
Gap vs Break-even (95% bound delta)
SAFE Flag
Symbol μ
Symbol σ
Symbol μ/σ



# Concentration & Structure

These describe system architecture risk.
HHI (Trade Concentration Index)
Symbol contribution dominance
Signed vs Absolute R skew

# Rolling / Cohort Analysis

These describe time-window sensitivity.
Last 2-day performance
Last 3-day performance
Last 4-day performance
Last 5-day performance
Last 6-day performance
Full dataset baseline
Cohort filter logic
Rolling TP1 rate
Rolling SL-before-TP1
Rolling expectancy
Rolling drawdown
Rolling stability


# Structural System Identity

These describe what the strategy actually is.
Continuation-based edge
Breakout follow-through dependency
Momentum sensitivity
Trend-following bias
Regime sensitivity
Compression vulnerability
Mean-reversion weakness
