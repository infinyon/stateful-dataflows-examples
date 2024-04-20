# Update-State Operator Example

In this example, we use the `update-state` operator to collect the `temperature` values from multiple sensors into a table. The dataflow uses `arrow-row` for the table representation.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

Produce temperature data from sensors at different locations:

```bash
fluvio produce temp-sensor
```

```json
{"id": 0, "sensor": "SF", "temperature": 54}
{"id": 1, "sensor": "LA", "temperature": 38}
{"id": 2, "sensor": "MO", "temperature": 65}
{"id": 0, "sensor": "SF", "temperature": 47}
```

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

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

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete temp-sensor
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites