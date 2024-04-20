# Connector to Generate Sentences

Use an http-source streaming connector to read quotes from the InfinyOn synthetic data server. Checkout [connectors.md](../../connectors.md) for additional information.

The connector configuration file is located at [sentence-connector.yaml](sentence-connector.yaml).

### 1. Download HTTP Source Connector

```bash
cdk hub download infinyon/http-source@0.3.2
```

### 2. Load JOLT Smartmodule to the Cluster

```bash
fluvio hub smartmodule download infinyon/jolt@0.3.0 
```

### 3. Start the connector

```bash
cdk deploy start --ipkg infinyon-http-source-0.3.3.ipkg -c sentence-connector.yaml
```

### 4. Stop the connector

```bash
cdk deploy shutdown --name sentence-connector
```
