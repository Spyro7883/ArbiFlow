#!/bin/bash

# Configuration
CONTRACT_ADDRESS="0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"
INITIAL_RATE=1
MAX_RATE=32
THRESHOLD=16
INCREASE_FACTOR=2       # As integer factor
DECREASE_FACTOR=0.5     # Use this to calculate rate separately
TOTAL_TRANSACTIONS=100
FUNCTION_CALL="updateValue(uint256)"
GAS_LIMIT=500000
VALUE=0
CAST_ABI="build/IMonitoringContract.abi"

# Variables
rate=$INITIAL_RATE
sent=0
start_time=$(date +%s)  # Start time of the simulation

echo "Starting TCP-like transaction simulation..."

while (( sent < TOTAL_TRANSACTIONS )); do
  echo "Rate: ${rate} tx/sec, Sent so far: $sent"

  # Calculate delay between transactions (as integer milliseconds)
  delay_ms=$(( 1000 / rate ))

  # Send transactions in bursts
  for (( i=0; i<rate && sent<TOTAL_TRANSACTIONS; i++ )); do
    tx_value=$(( RANDOM % 1000 ))
    cast send $CONTRACT_ADDRESS "$FUNCTION_CALL" $tx_value --private-key $ETH_PRIVATE_KEY --gas-limit $GAS_LIMIT --value $VALUE &> /dev/null &
    (( sent++ ))
    sleep "$((delay_ms))e-3"  # Convert milliseconds to seconds
  done

  # Increase the rate
  rate=$(( rate * INCREASE_FACTOR ))

  # Check for threshold and apply congestion control
  if (( rate >= THRESHOLD )); then
    echo "Congestion detected. Reducing rate."
    rate=$(echo "$rate * $DECREASE_FACTOR" | bc)  # Use bc for the reduction
    rate=${rate%.*}  # Convert to integer by truncating decimal

    # Ensure rate does not go below INITIAL_RATE
    if (( rate < INITIAL_RATE )); then
      rate=$INITIAL_RATE
    fi
  fi

  # Ensure rate doesn't exceed MAX_RATE
  if (( rate > MAX_RATE )); then
    rate=$MAX_RATE
  fi
done

echo "All transactions sent. Waiting for network congestion to clear..."

# Start timing the congestion clearance
congestion_clear_start=$(date +%s)

# Check periodically until the rate drops back to the initial rate
while true; do
  # Simulate a reduction in congestion (rate naturally decreases over time)
  rate=$(echo "$rate * $DECREASE_FACTOR" | bc)
  rate=${rate%.*}  # Convert to integer by truncating decimal

  # If rate has dropped to or below the initial rate, we consider congestion cleared
  if (( rate <= INITIAL_RATE )); then
    break
  fi

  echo "Current simulated network rate: $rate tx/sec"
  sleep 1  # Check again after 1 second
done

# Calculate congestion clearance time
congestion_clear_end=$(date +%s)
congestion_clear_duration=$((congestion_clear_end - congestion_clear_start))

echo "Network congestion cleared after $congestion_clear_duration seconds."
echo "Simulation complete."
