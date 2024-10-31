# Split (with filter operator)

In this example, we'll read `person` records from a `user` topic split them into `child` and `adult` topics based on age.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

The sample data file used to run this test `./sample-data/data.txt` has the following records:

```bash
{"name":"Andrew","age":16}
{"name":"Jackson","age":17}
{"name":"Randy","age":32}
{"name":"Alice","age":28}
{"name":"Linda","age":15}
```

Produce the data to the `user` topic:

```bash
fluvio produce user -f ./sample-data/data.txt
```

Checkout the data in `user` topic:

```bash
fluvio consume user -Bd
```

Consume from `child` and `adult` to see the result:

```bash
fluvio consume child -Bd
fluvio consume adult -Bd
```

```bash
# child
{"age":16,"name":"Andrew"}
{"age":17,"name":"Jackson"}
{"age":15,"name":"Linda"}

# adult
{"age":32,"name":"Randy"}
{"age":28,"name":"Alice"}
```

**Note:** the data from `user` topic is split into `child` and `adult` topics based on age.

### Run SDF commands

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show split-service/user/metrics
```

```bash
 Key    Window  succeeded  failed 
 stats  *       5          0   
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
