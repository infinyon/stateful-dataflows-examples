# Filter Operator Example

Example of how to use the `filter` operator. In this example, the filter operation allows all sentences that have a question mark (`?`) and drops all others.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run --ephemeral
```

### Test DataFlow

The sample data file used to run this test `./sample-data/data.txt` has the following records:

```bash
Hi there!
What time is it?
```

Produce the data to the `sentences` topic:

```bash
fluvio produce sentences -f ./sample-data/data.txt
```

Checkout the data in `sentences` topic:

```bash
fluvio consume sentences -Bd
```

Consume from `questions` to retrieve the result:

```bash
fluvio consume questions -Bd
```

```bash
What time is it?
```

**Note:** the sentence with question mark `?` is returned, the other is filtered out.


### Run SDF commands

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state filter-service/filter-questions/metrics
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