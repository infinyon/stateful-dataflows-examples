# Connector to Generate Sentences

Use an http-source streaming connector to read cars from the InfinyOn synthetic data server. 

The connector configuration file is located at [car-callout-connector.yaml](car-callout-connector.yaml).

### 1. Download HTTP Source Connector

```bash
cdk hub download infinyon/http-source@0.3.6
```

### 2. Start the connector

```bash
cdk deploy start --ipkg infinyon-http-source-0.3.6.ipkg -c car-callout-connector.yaml
```

### 3. Checkout  car events

```bash
fluvio consume cars
```

Hit <Ctrl-C> to exit.

### 4. Stop the connector

```bash
cdk deploy shutdown --name car-callout
```
