# Split (with filter-map operator)

Here's an example of how to split traffic from one to multiple topics.

The example splits car events into trucks and sedans based on their category while converting the record format to conform to the schema of the receiving topics.

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
{"car":"Honda Accord","category":"sedan","license":"4GCF223"}
{"car":"Ford f-150","category":"truck","license":"6FAF434"}
{"car":"BMW 330","category":"sedan","license":"5JAC844"}
{"car":"Dodge RAM","category":"truck","license":"6DUA684"}
```

Produce the data to the `cars` topic:

```bash
fluvio produce cars -f ./sample-data/data.txt
```

Checkout the data in `cars` topic:

```bash
fluvio consume cars -Bd
```

Consume from `trucks` and `sedans` to see the result:

```bash
fluvio consume trucks -Bd
fluvio consume sedans -Bd
```

```bash
# trucks
{"license":"6FAF434","truck":"Ford f-150"}
{"license":"6DUA684","truck":"Dodge RAM"}

# sedans
{"license":"4GCF223","sedan":"Honda Accord"}
{"license":"5JAC844","sedan":"BMW 330"}
```

**Note:** the split operator divided the `cars` into `trucks` and `sedans`.


### Run SDF commands

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state trucks/to-truck/metrics --table
```

```bash
Key    Window  succeeded  failed
stats  *       4          0
```

Feel free to checkout the other metrics.


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
