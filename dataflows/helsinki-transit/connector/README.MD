# Connector for Helsinki Transit Dataflow

Use mqtt-source connector to read the Helkinki Transit live data feed.

### Prerequisites

*Checkout the connector configuration file [mqtt-helsinki.yaml](mqtt-helsinki.yaml) to get context on what we are doing.

* Download MQTT Source Connector

  ```bash
  cdk hub download infinyon/mqtt-source@0.2.9
  ```

* Load Jolt Smartmodule to your cluster:

  ```bash
  fluvio hub smartmodule download infinyon/jolt@0.4.1 
  ```

### Start the mqtt connector

Checkout the connector configuration file [mqtt-helsinki.yaml](mqtt-helsinki.yaml) for context.

Start the connector:

```bash
cdk deploy start --ipkg infinyon-mqtt-source-0.2.9.ipkg -c mqtt-helsinki.yaml
```

This connector refreshes the licenses every hour. Use fluvio to see the license numbers downloaded from the server:

```bash
fluvio consume helsinki
```

Use <Ctrl-C> to exit


### Clean-up

Stop the connectors:

```bash
cdk deploy shutdown --name helsinki-mqtt
```
