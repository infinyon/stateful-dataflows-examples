# Map Operator Example

Example of how to use the `map` operator. In this example, the map operator transforms masks SSN numbers in a user record.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

Produce to `users`:

```bash
fluvio produce users
```

```bash
{"name": "Alice", "ssn": "222-333-5555"}
{"name": "Bob", "ssn": "444-666-7777"}
```

Consume from `users-public` to see them transformed:

```bash
fluvio consume users-public -Bd
```

```bash
{"name":"Alice","ssn":"***-***-****"}
{"name":"Bob","ssn":"***-***-****"}
```

Display the stateful dataflow stats in the `sdf` runtime `>>` terminal:

```bash
show state map-service/mask-ssn/metrics --table
```

```bash
Key    Window  succeeded  failed
stats  *       2          0
```

### Clean-up

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete users
fluvio topic delete users-public
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites