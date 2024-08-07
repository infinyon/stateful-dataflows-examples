# Regex Example

In this example, we'll import the `regex` crate to demonstrate adding external `dependencies` for inline functions. We'll use the regex function to identify generation-z based on their year of birth.

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
{"name": "John", "born": 1996}
{"name": "Alice", "born": 1998}
{"name": "Bob", "born": 2011}
{"name": "Michael", "born": 2018}
```

Produce the data to the `person` topic:

```bash
fluvio produce person -f ./sample-data/data.txt
```

Checkout the data in `person` topic:

```bash
fluvio consume person -Bd
```

Consume from `gen-z` to retrieve the result:

```bash
fluvio consume gen-z -Bd
```

```json
{"name": "Alice", "born": 1998}
{"name": "Bob", "born": 2011}
```

**Note:** the regex filter removes people born outside of `gen-z` age group.


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites