# Output Key/Value Example

This is an example of sending records with a `key` and a `value` for a fluvio topic. We've defined a function that reads records from a topic, divides them into a key and a value, and sends the result to a new topic.

* See the [dataflow.yaml](./dataflow.yaml) to see how we've configured the dataflow.
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run --ephemeral
```

### Test DataFlow

We've created a sample data file to load records into a fluvio topic. Let's view the file:

```bash
cat ./sample-data/users.txt
```

```bash
{"age":23,"id":"0001","name":"Alice"}
{"age":21,"id":"0002","name":"Bob"}
{"age":34,"id":"0003","name":"Charlie"}
```

Produce the data to the `user` topic:

```bash
fluvio produce user -f ./sample-data/users.txt
```

Checkout the data in `user` topic:

```bash
fluvio consume user -Bd -k
```

```bash
[null] {"age":23,"id":"0001","name":"Alice"}
[null] {"age":21,"id":"0002","name":"Bob"}
[null] {"age":34,"id":"0003","name":"Charlie"}
```

Consume from `users` to retrieve the result:

```bash
fluvio consume user-kv -Bd -k
```

```bash
[0001] {"age":23,"name":"Alice"}
[0002] {"age":21,"name":"Bob"}
[0003] {"age":34,"name":"Charlie"}
```

Note that the `id` field was moved from the `value` into the `key`.

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state user-to-kv/make-kv-user/metrics
```

```bash
Key    Window  succeeded  failed
stats  *       3          0
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites