# FilterMap Operator Example

Example of how to use the `filter` operator. In this example, the filter operation allows all sentences that have a question mark (`?`) and drops all others.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

Produce to sentence with and without question mark (`?`):

```bash
fluvio produce sentences
```

```bash
Hi there!
What time is it?
```

Consume from `questions` to retrieve the result:

```bash
fluvio consume questions -Bd
```

```bash
What time is it?
```

Display stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state filter-service/filter-questions/metrics --table
```

```bash
Key    Window  succeeded  failed 
stats  *       2          0    
```

### Clean-up

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete sentences
fluvio topic delete questions
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites