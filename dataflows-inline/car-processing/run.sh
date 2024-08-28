fluvio cluster start
fluvio cluster status
cd /workspace/connectors/ 
cdk deploy start --ipkg infinyon-http-source-0.3.8.ipkg -c license-connector.yaml
cdk deploy start --ipkg infinyon-http-source-0.3.8.ipkg -c car-connector.yaml
cd /workspace

sdf clean
initial_command="sdf run --ephemeral"
new_command="fluvio consume speeding"

$initial_command > output.log 2>&1 &
initial_pid=$!

echo "Waiting for SDF to Start...(this will take a long time)"
tail -f output.log | while read line; do
    echo "$line"
    if [[ "$line" == *"Welcome to SDF"* ]]; then
        echo "Running SDF"
        $new_command
        pkill -P $$ tail
        break
    fi
done
