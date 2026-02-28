#!/bin/bash

API="http://127.0.0.1:8181/api/v1/dev/trade-approved"

echo "Sending test trades..."

symbols=(
'{"symbol":"BTC","side":"BUY","entry":50000,"stop_loss":49000,"targets":[51000,52000],"timeframe":"1h"}'
'{"symbol":"BTC","side":"SELL","entry":50000,"stop_loss":51000,"targets":[49000,48000],"timeframe":"1h"}'
'{"symbol":"ETH","side":"SELL","entry":3000,"stop_loss":3100,"targets":[2900,2800],"timeframe":"4h"}'
'{"symbol":"SOL","side":"BUY","entry":100,"stop_loss":95,"targets":[105,110],"timeframe":"30m"}'
'{"symbol":"XRP","side":"BUY","entry":0.55,"stop_loss":0.5,"targets":[0.6,0.65],"timeframe":"15m"}'
'{"symbol":"BNB","side":"SELL","entry":300,"stop_loss":315,"targets":[280,260],"timeframe":"1h"}'
'{"symbol":"ADA","side":"BUY","entry":0.6,"stop_loss":0.55,"targets":[0.65,0.7],"timeframe":"1h"}'
'{"symbol":"TRX","side":"SELL","entry":0.12,"stop_loss":0.13,"targets":[0.11,0.10],"timeframe":"15m"}'
)

for payload in "${symbols[@]}"; do
    echo "➡ Sending: $payload"

    curl -X POST "$API" \
        -H "Content-Type: application/json" \
        -d "$payload" \
        -w "\nHTTP Status: %{http_code}\n\n"

done

echo "All test trades dispatched."