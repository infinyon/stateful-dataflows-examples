#!/bin/bash
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <flow_id> <source>"
    exit 1
fi

while true; do
    random_string=$(openssl rand -base64 12)
    json_input="{\"id\": \"$1\", \"payload\": \"$random_string\", \"source\":\"$2\"}"

    echo "$json_input" | fluvio produce data-input --batch-size=20000
    fluvio consume data-input -Bd
    fluvio consume data-output -Bd
    sleep 3
done

