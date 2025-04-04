# HTTP Callout Example

HTTP Callout shows how to call external HTTP servers from inside the operators. In this example, we will read `car` events and send requests to an HTTP server for additional information about the `maker`.

<p align="center">
 <img width="700" src="img/http-callout.jpg">
</p>

The `car` events are generated via an [HTTP-Source connector](connectors).

## Dataflow Primitives

The dataflow uses the following primitives:
* _map_
* _capabilities_
  * _http_


## Step-by-step

* Take a look at the [dataflow.yaml](./dataflow.yaml) to get an idea of what we're doing.

* Make sure to [Install SDF & Start a Cluster].

### 1. Run the Dataflow

Use `sdf` command line tool to run the dataflow:

```bash
sdf run --ui
```

Use `--ui` to open the Studio.

### 2. Test the Dataflow

#### Start the HTTP Sink Connector

In a new terminal change direcotry to `./connectors`, download the connector binary, and start 2 connectors:

```bash
cd ./connectors
cdk hub download infinyon/http-source@0.4.3
cdk deploy start --ipkg infinyon-http-source-0.4.3.ipkg -c car-callout-connector.yaml
```

To see the events, run `fluvio consume cars`.

For additional context, checkout [connectors](./connectors/).


#### Consume form Makers

The http lookup result is sent to a `makers` topic. Consume from `makers` topic:

```bash
fluvio consume makers
```

Hit <Ctrl-C> to exit.

#### Check Metrics

The SDF engine will automatically collect metrics for the dataflow. In the `sdf` terminal, show the number of records processed by `get-car-maker`:

```bash
show state lookup-car-maker/get-car-maker/metrics
```

```bash
Key    Window  succeeded  failed
stats  *       36         0
```

Run `show state` to see the other parameters available.

Congratulations! You've successfully built and run a dataflow!

## Clean-up

Note that your connector will continue to run in the background until you shut them down:

```bash
cdk deploy shutdown --name car-callout
```

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF & Start a Cluster]: /README.MD#prerequisites
