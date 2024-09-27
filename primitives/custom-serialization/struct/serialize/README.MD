# Map Operator Example

Example of how to rename the fields of sink structs during serialization. In this example, the `full_name` field on `citizen` is renamed to `fullName`.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

The sample data file used to run this test `./sample-data/data.txt` has the following records:

```bash
{"full_name": "Alice", "age": 17}
{"full_name": "Bob", "age": 32}
```

Produce the data to the `citizens` topic:

```bash
fluvio produce citizens -f ./sample-data/data.txt
```

Checkout the data in `citizens` topic:

```bash
fluvio consume citizens -Bd
```

Consume from `citizens-public` to see them transformed:

```bash
fluvio consume citizens-public -Bd
```

```bash
{"fullName":"Bob","age":32}
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites