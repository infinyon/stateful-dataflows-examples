# Connectors to Generate Licenses & Car Events

Use http-source connector to install license plates and read cars from the InfinyOn synthetic data server.

### Prerequisites

Download HTTP Source Connector

```bash
cd ./connectors
cdk hub download infinyon/http-source@0.3.6
```

### Start the licenses connector

Checkout the connector configuration file [license-connector.yaml](license-connector.yaml) for context.

Start the connector:

```bash
cdk deploy start --ipkg infinyon-http-source-0.3.6.ipkg -c license-connector.yaml
```

This connector refreshes the licenses every hour. User fluvio to see the license numbers downloaded from the server:

```bash
fluvio consume licenses -Bd
```


### Start the car connector

Checkout the connector configuration file [car-connector.yaml](car-connector.yaml) for context.

```bash
cdk deploy start --ipkg infinyon-http-source-0.3.6.ipkg -c car-connector.yaml
```

The cars connector receives two car events every second. Use fluvio to see the events streaming in real-time:

```bash
fluvio consume cars
```

Use <Ctrl-C> to exit


### Clean-up

Stop the connectors:

```bash
cdk deploy shutdown --name car-connector
cdk deploy shutdown --name license-connector
```
