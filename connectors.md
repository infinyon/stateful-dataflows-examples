# How to Use Connectors with Stateful Services

Connectors are system components that link external services with a fluvio cluster. Connectors communicate with stateful streaming via topics.

For instance a source connector may generate records into a topic `http-data`, and the dataflow reads from it. After completing the dataflow operation, it writes the results into another topic `http-data-results`. Finally, a sink connector sends these events to an external service.

In the following examples, we'll use the CDK CLI to download the HTTP sink connector from InfinyOn Hub and generate synthetic data for dataflows.

InfinyOn provides several data sets that you can use to generate synthetic data, which we'll describe below.

### Prerequisites

* [Install Rust](./README.md#install--update-rust)
* [Install CDK - via fluvio CLI](./README.md#install-fluvio--sdf)

### Download the  HTTP-Source Connector

Use `CDK` to download the http-sink connector to your local machine.

```bash
cdk hub download infinyon/http-source@0.3.3
```

### Start HTTP-Source Pull Connector

The following configuration reads a `quote` periodically (every two seconds):

```yaml
# quotes-periodic.yaml
apiVersion: 0.2.0
meta:
  version: 0.3.3
  name: quotes-periodic
  type: http-source
  topic: quotes
http:
  endpoint: "https://demo-data.infinyon.com/api/quote"
  interval: 2s
```

Copy the configuration to a file and star the connector on your local machine:

```bash
cdk deploy start --ipkg infinyon-http-source-0.3.3.ipkg -c quotes-periodic.yaml
```

Checkout the result:

```bash
fluvio consume quotes
```

To stop the connector, run:

```bash
cdk deploy shutdown --name quotes-periodic
```

### Start HTTP-Source Streaming Connector

The following configuration streams from the `stream-quote` endpoint, which is pre-configured to generate 2 quotes per second.

```yaml
# quotes-stream.yaml
apiVersion: 0.2.0
meta:
  version: 0.3.3
  name: quotes-stream
  type: http-source
  topic: quotes
http:
  endpoint: https://demo-data.infinyon.com/api/stream-quote
  method: GET
  stream: true
  delimiter: "\n\n"
```

Copy the configuration to a file and star the connector on your local machine:

```bash
cdk deploy start --ipkg infinyon-http-source-0.3.3.ipkg -c quotes-stream.yaml
```

Checkout the result:

```bash
fluvio consume quotes
```

To stop the connector, run:

```bash
cdk deploy shutdown --name quotes-stream
```
