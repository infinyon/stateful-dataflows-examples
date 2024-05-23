# Split Example

In this example, we'll read `person` records from a `user` topic split them into `child` and `adult` topics based on age.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

Produce to `user`:

```bash
fluvio produce user
```

```bash
{"name":"Andrew","age":16}
{"name":"Jackson","age":17}
{"name":"Randy","age":32}
{"name":"Alice","age":28}
{"name":"Linda","age":15}
```

Consume from `child`:

```bash
fluvio consume child -Bd
```

```bash
{"age":16,"name":"Andrew"}
{"age":17,"name":"Jackson"}
{"age":15,"name":"Linda"}
```

Consume from `adult`:

```bash
fluvio consume adult -Bd
```

```bash
{"age":32,"name":"Randy"}
{"age":28,"name":"Alice"}
```

### Clean-up

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete user
fluvio topic delete child
fluvio topic delete adult
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
