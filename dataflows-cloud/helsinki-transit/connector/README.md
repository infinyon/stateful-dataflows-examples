# Connector for Helsinki Transit Dataflow

Use mqtt-source connector to read the Helkinki Transit live data feed.

### Prerequisites

*Checkout the connector configuration file [mqtt-helsinki.yaml](mqtt-helsinki.yaml) to get context on what we are doing.


* Load Jolt Smartmodule to your cluster:

  ```bash
  fluvio hub smartmodule download infinyon/jaq@0.1.0
  ```

### Start the mqtt connector

Checkout the connector configuration file [mqtt-helsinki.yaml](mqtt-helsinki.yaml) for context.

Start the cloud connector:

```bash
fluvio cloud connector create --config mqtt-helsinki.yaml
```

This connector refreshes the licenses every hour. Use fluvio to see the license numbers downloaded from the server:

```bash
fluvio consume helsinki
```

Use <Ctrl-C> to exit


### Clean-up

Delete connector:

```bash
fluvio cloud connector delete helsinki-mqtt
```
