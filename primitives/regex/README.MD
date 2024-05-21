# Regex Example

In this example, we'll import the `regex` crate to demonstrate adding external `dependencies` for inline functions. We'll use the regex function to identify generation-z based on their year of birth.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

Produce records in the `person` topic:

```bash
fluvio produce person
```

```json
{"name": "John", "born": 1996}
{"name": "Alice", "born": 1998}
{"name": "Bob", "born": 2011}
{"name": "Michael", "born": 2018}
```

Consume from `gen-z` to retrieve the result:

```bash
fluvio consume gen-z -Bd
```

```json
{"name": "Alice", "born": 1998}
{"name": "Bob", "born": 2011}
```

### Clean-up

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete person
fluvio topic delete gen-z
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites