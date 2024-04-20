# FilterMap Operator Example

Example of how to use the `filter-map` operator. In this example, the filter-map operation transforms sentences over 10 characters in length to uppercase and drops all others.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

Produce to short and long sentences:

```bash
fluvio produce sentences
```

```bash
say hello
say hello world
```

Consume from `capitalized` to retrieve the result:

```bash
fluvio consume capitalized -Bd
```

```bash
SAY HELLO WORLD
```

Display stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state filter-map-service/long-sentence-to-uppercase/metrics --table
```

```bash
Key    Window  succeeded  failed 
stats  *       2          0    
```

### Clean-up

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete sentences
fluvio topic delete capitalized
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
