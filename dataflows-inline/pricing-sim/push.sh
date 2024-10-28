#!/bin/bash
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <dataflow_raw_file>"
    exit 1
fi

while true; do
    user_id="user_$((RANDOM % 10000))"
    name="name_$((RANDOM % 100))"

    dataflow_raw=$(cat "$1" | base64)
    json_input="{\"user\": \"$user_id\", \"id\": \"$name\", \"dataflow_raw\": \"$dataflow_raw\"}"

    echo "$json_input" | fluvio produce new-dataflow --batch-size=20000

    # Optional: consume the latest data from Fluvio for validation
    fluvio consume new-dataflow -d -T 1

    # Sleep for a short time to avoid flooding (optional)
    sleep 10
done

