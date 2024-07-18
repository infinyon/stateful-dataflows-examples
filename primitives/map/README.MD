# Map Operator Example

Example of how to use the `map` operator. In this example, the map operator transforms masks SSN numbers in a user record.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

The sample data file has the following records:

```bash
{"name": "Alice", "ssn": "222-333-5555"}
{"name": "Bob", "ssn": "444-666-7777"}
```

Produce the data to the `users` topic:

```bash
fluvio produce users -f ./sample-data/data.txt
```

Checkout the data in `users` topic:

```bash
fluvio consume users -Bd
```

Consume from `users-public` to see them transformed:

```bash
fluvio consume users-public -Bd
```

```bash
{"name":"Alice","ssn":"***-***-****"}
{"name":"Bob","ssn":"***-***-****"}
```

**Note:** the `ssn` values are masked by the regex command.


### Run SDF commands

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state map-service/mask-ssn/metrics --table
```

```bash
Key    Window  succeeded  failed
stats  *       2          0
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites