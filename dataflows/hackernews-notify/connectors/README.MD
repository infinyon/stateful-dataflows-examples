# Connector to Read Hackernews Feed

Use an http-source streaming connector to read the Hackernews RSS feed, transform each article into a JSON object, and send it to the topic `hackernews`.

The connector configuration file is located at [hackernews.yaml](hackernews.yaml).

### 1. Download HTTP Source Connector

```bash
cdk hub download infinyon/http-source@0.3.8
```

### 2. Download Smartmodules

Download the smartmodules used by the hackernews source connectors to your cluster:

```bash
fluvio hub sm download infinyon/jolt@0.4.1
fluvio hub sm download infinyon-labs/rss-json@0.1.1
fluvio hub sm download infinyon-labs/array-map-json@0.1.1
```


### 3. Start the connector

```bash
cdk deploy start --ipkg infinyon-http-source-0.3.8.ipkg -c hackernews.yaml
```

### 4. Stop the connector

To stop the connector, run:

```bash
cdk deploy shutdown --name hackernews
```
