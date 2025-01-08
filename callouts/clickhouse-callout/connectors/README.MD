# Connectors to Generate NY Transit Events

Use http-source connector to read NY transit data from the InfinyOn data server.
The transit data has been converted from parquet to JSON for readability. 

The source of the data and the schema are available at:
* https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page
* https://www.nyc.gov/assets/tlc/downloads/pdf/data_dictionary_trip_records_hvfhs.pdf

### Prerequisites

Download HTTP Source Connector

```bash
cd ./connectors
cdk hub download infinyon/http-source@0.4.3
```

### Start the connector

Checkout the connector configuration file [ny-trips-connector.yaml](ny-trips-connector.yaml) for context.

```bash
cdk deploy start --ipkg infinyon-http-source-0.4.3.ipkg -c ny-trips-connector.yaml
```

The NY transit connector receives 10 events per second. Use fluvio to see the events streaming in real-time:

```bash
fluvio consume ny-trips
```

Use <Ctrl-C> to exit


### Clean-up

Stop the connector:

```bash
cdk deploy shutdown --name ny-trips-connector
```
