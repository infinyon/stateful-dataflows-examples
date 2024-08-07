# Update-State Operator Example

In this example, we use the `update-state` operator to collect the `temperature` values from multiple sensors into a table. The dataflow uses `arrow-row` for the table representation.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run --ephemeral
```

### Test DataFlow

The sample data file has the following records:

```json
{"id": 0, "sensor": "SF", "temperature": 54}
{"id": 1, "sensor": "LA", "temperature": 38}
{"id": 2, "sensor": "MO", "temperature": 65}
{"id": 0, "sensor": "SF", "temperature": 47}
```

Produce the data to the `temp-sensor` topic:

```bash
fluvio produce temp-sensor -f ./sample-data/data.txt
```

Checkout the data in `temp-sensor` topic:

```bash
fluvio consume temp-sensor -Bd
```

### Run SDF commands

Use the `sdf` runtime `>>` terminal to display the state variable:

```bash
show state save-temperature/temperature/state --table
```

```bash
 Key  Window  sensor  temperature
 0    *       SF      47
 1    *       LA      38
 2    *       MO      65
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites