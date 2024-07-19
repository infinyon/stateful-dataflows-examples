# Input Key/Value Example

This is an example of sourcing records with `key` and `value` from a fluvio topic. We've defined a function that reads key-value records from a topic, collapses them into a single value, and sends the result to a new topic.

* See the [dataflow.yaml](./dataflow.yaml) to see how we've configured the dataflow.
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

We've created a sample data file to load key-value records into a fluvio topic. Let's view the file:

```bash
cat ./sample-data/users.txt 
```

```bash
0001>{"name":"Alice", "age":23}
0002>{"name":"Bob", "age":21}
0003>{"name":"Charlie", "age":34}
```

Produce the data to the `user-kv` topic:

```bash
fluvio produce user-kv --key-separator ">" -f ./sample-data/users.txt
```

Checkout the data in `user-kv` topic:

```bash
fluvio consume user-kv -Bd -k
```

```bash
[0001] {"name":"Alice", "age":23}
[0002] {"name":"Bob", "age":21}
[0003] {"name":"Charlie", "age":34}
```

Consume from `users` to retrieve the result:

```bash
fluvio consume user -Bd -k
```

```bash
[null] {"age":23,"id":"0001","name":"Alice"}
[null] {"age":21,"id":"0002","name":"Bob"}
[null] {"age":34,"id":"0003","name":"Charlie"}
```

Note that the `id` field was moved from the `key` into the `value`.


Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state kv-to-user/user-kv/metrics --table
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