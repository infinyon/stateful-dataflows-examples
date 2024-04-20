# FlatMap Operator Example

Example of how to use the `flat-map` operator. In this example, the flat-map operator converts sentences into words.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SSDK and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
ssdk run
```

### Test DataFlow

Produce sentences:

```bash
fluvio produce sentences
```

```bash
This is a test
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

Display the stateful service stats in the ssdk `>>` terminal:

```bash
show state flat-map-service/split-sentence/metrics --table
```

```bash
Key    Window  succeeded  failed 
stats  *       1          0   
```

### Clean-up

Exit `ssdk` terminal and remove the topics:

```bash
fluvio topic delete sentences
fluvio topic delete words
```

[Install SSDK and start a Fluvio cluster]: /README.MD#prerequisites