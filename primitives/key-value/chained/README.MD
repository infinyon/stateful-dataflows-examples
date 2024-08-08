# Chained Key/Value Example

This example demonstrates the engine behavior for passing keys between chained operators.

For a source topic with a key, the engine has the following behavior:

* A function may read or ignore the incoming key.
* If the function returns a new key, the following function receives the new key.
* If any function in the middle of the chain returns a value (without a key), the chain is not broken; the following function reading the key will receive the last updated key.


In this example, we've defined a couple of functions. The first function `update_location` takes the key and uses the first digit to identify the region and populate the value. Note that the first function does not pass down the key, but the second function still has access. The second function `add_new_key` reads the key, adds to the value, and returns a new key with the person's label.

* See the [dataflow.yaml](./dataflow.yaml) to see how we've configured the dataflow.
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

We've created a sample data file to load records into a fluvio topic. Let's view the file:

```bash
cat ./sample-data/users.txt
```

```bash
1001>{"name":"Alice", "age":10}
1002>{"name":"Bob", "age":17}
2003>{"name":"Charlie", "age":24}
```

Produce the data to the `user` topic:

```bash
fluvio produce user-input --key-separator ">" -f ./sample-data/users.txt
```

Checkout the data in `user` topic:

```bash
fluvio consume user-input -Bd -k
```

```bash
[1001] {"name":"Alice", "age":10}
[1002] {"name":"Bob", "age":17}
[2003] {"name":"Charlie", "age":24}
```

Consume from `user-output` to retrieve the result:

```bash
fluvio consume user-output -Bd -k
```

```bash
[child] {"age":10,"id":"1001","location":"New York","name":"Alice"}
[child] {"age":17,"id":"1002","location":"New York","name":"Bob"}
[adult] {"age":24,"id":"2003","location":"California","name":"Charlie"}
```

The existing `key` was merged into the `value` and a new `key` was assigned.

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state kv-to-user/update-location/metrics --table
show state kv-to-user/add-new-key/metrics --table
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites