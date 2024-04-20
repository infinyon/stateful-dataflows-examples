# Map Operator Example

Example of how to use the `map` operator. In this example, the map operator transforms masks SSN numbers in a user record.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SSDK and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
ssdk run
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

Display the stateful service stats in the ssdk `>>` terminal:

```bash
show state map-service/mask-ssn/metrics --table
```

```bash
Key    Window  succeeded  failed 
stats  *       2          0   
```

### Clean-up

Exit `ssdk` terminal and remove the topics:

```bash
fluvio topic delete users
fluvio topic delete users-public
```

[Install SSDK and start a Fluvio cluster]: /README.MD#prerequisites