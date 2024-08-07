# FlatMap Operator Example

Example of how to use the `flat-map` operator. In this example, the flat-map operator converts sentences into words.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run --ephemeral
```

### Test DataFlow

The sample data file has the following records:

```bash
This is a test
```

Produce the data to the `sentences` topic:

```bash
fluvio produce sentences -f ./sample-data/data.txt
```

Checkout the data in `sentences` topic:

```bash
fluvio consume sentences -Bd
```

Consume from `words` to see them transformed:

```bash
fluvio consume words -Bd
```

```bash
This
is
a
test
```

**Note:** the sentence has been divided into a stream of words.


### Run SDF commands

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state flat-map-service/split-sentence/metrics --table
```

```bash
Key    Window  succeeded  failed
stats  *       1          0
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
