# Merge Example

Here's an example of how to merge events from multiple topics into a single topic.

The example reads events from the truck and sedan topics, converts them to a string format and sends the result to the licenses topic.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run --ephemeral
```

### Test DataFlow

For this example, we've created two data files: `trucks` and `sedans`.

```bash
# trucks.txt
{"truck":"Ford f-150","license":"6FAF434"}
{"truck":"Dodge RAM","license":"6DUA684"}

# sedans.txt
{"sedan":"Honda Accord","license":"4GCF223"}
{"sedan":"BMW 330","license":"5JAC844"}
```

Produce the data to the `trucks` and `sedans` topics:

```bash
fluvio produce trucks -f ./sample-data/trucks.txt
fluvio produce sedans -f ./sample-data/sedans.txt
```

Checkout the data in both topics:

```bash
fluvio consume trucks -Bd
fluvio consume sedans -Bd
```

Consume from `licenses` to see their license plates:

```bash
fluvio consume licenses -Bd
```

```bash
"6FAF434"
"6DUA684"
"4GCF223"
"5JAC844"
```

**Note:** the two topics are merged and the resulting values are normalized to the target topic schema.


### Run SDF commands

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state sedans/sedan-license/metrics --table
```

```bash
Key    Window  succeeded  failed
stats  *       2          0
```

```bash
show state trucks/truck-license/metrics --table
```

```bash
Key    Window  succeeded  failed
stats  *       2          0
```

Feel free to checkout the other metrics.


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites