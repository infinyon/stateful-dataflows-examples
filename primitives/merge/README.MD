# Merge Example

Here's an example of how to merge events from multiple topics into a single topic.

The example reads events from the truck and sedan topics, converts them to a string format and sends the result to the licenses topic.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

Produce to `trucks`:

```bash
fluvio produce trucks
```

```bash
{"truck":"Ford f-150","license":"6FAF434"}
{"truck":"Dodge RAM","license":"6DUA684"}
```

Produce to `sedans`:

```bash
fluvio produce sedans
```

```bash
{"sedan":"Honda Accord","license":"4GCF223"}
{"sedan":"BMW 330","license":"5JAC844"}
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

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete trucks
fluvio topic delete sedans
fluvio topic delete licenses
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites