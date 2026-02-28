#!/bin/bash

API="http://127.0.0.1:8181/api/v1/dev/trade-approved"

ROUNDS=5
DELAY=0.5

echo "Starting visible stress test"
echo ""

symbols=(
'{"symbol":"BTC","side":"BUY","entry":50000,"stop_loss":49000,"targets":[51000,52000],"timeframe":"1h"}'
'{"symbol":"ETH","side":"SELL","entry":3000,"stop_loss":3100,"targets":[2900,2800],"timeframe":"4h"}'
'{"symbol":"SOL","side":"BUY","entry":100,"stop_loss":95,"targets":[105,110],"timeframe":"30m"}'
)

for i in $(seq 1 $ROUNDS); do
    echo "=============================="
    echo "Batch $i"
    echo "=============================="

    for payload in "${symbols[@]}"; do
        (
            echo "➡ Sending: $payload"

            curl -X POST "$API" \
                -H "Content-Type: application/json" \
                -d "$payload" \
                -w "\nHTTP Status: %{http_code}\n\n"
        ) &
    done

    sleep $DELAY
done

wait

echo ""
echo "Stress test finished"