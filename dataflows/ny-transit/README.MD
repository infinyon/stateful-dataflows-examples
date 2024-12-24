# NY Transit dataflow

New York transit dataflow shows how join and query data sets for stream analytics use cases.

The source of the data and the schemas are available at:

* https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page
* https://www.nyc.gov/assets/tlc/downloads/pdf/data_dictionary_trip_records_hvfhs.pdf


<p align="center">
 <img width="700" src="img/xyz.jpg">
</p>


## Step-by-step

Take a look at the [dataflow.yaml](./dataflow.yaml) to get an idea of what we're doing.

Make sure to [Install SDF and start a Fluvio cluster].

### 1. Run the Dataflow

Use `sdf` command line tool to run the dataflow:

```bash
sdf run --ui
```

* Use `--ui` to generate the graphical representation and run the Studio.
* Using `sdf run` as opposed to `sdf deploy` will run the dataflow with an ephemeral worker, which will be cleaned up on exit.

**Note:** It is  important to run the dataflow first, as it creates the topics for you.

### 2. Read the locations & the zones:

Read locations from the data generator and add them to the `locations` topic:

```bash
curl -s https://demo-data.infinyon.com/api/ny-transit/locations | fluvio produce locations
```

Read vendors from the data generator and add them to the `vendors` topic:

```bash
curl -s https://demo-data.infinyon.com/api/ny-transit/vendors | fluvio produce vendors
```

### 2. Start the http connector:

In a new terminal change directory to `./connectors`, download the connector binary, and start connector:

```bash
cd ./connectors
cdk hub download infinyon/http-source@0.4.3
cdk deploy start --ipkg infinyon-http-source-0.4.3.ipkg -c trips-connector.yaml
```

The NY transit connector receives 10 events per second. Use fluvio to see the events streaming in real-time:

```bash
fluvio consume trips
```

Use <Ctrl-C> to exit


### Test SQL

SQL commands:

```sql
select count(*) as rides, sum(`base-passenger-fare`), sum(tips), sum(tolls), sum(`congestion-surcharge`), sum(`driver-pay`), sum(`airport-fee`), sum(`sales-tax`) from trips_tbl t
```

SQL for congestion surcharge:

```sql
select count(*) as rides, sum(`base-passenger-fare`), sum(tips), sum(tolls), sum(`congestion-surcharge`), sum(`driver-pay`), sum(`airport-fee`), sum(`sales-tax`) from trips_tbl t where t.`congestion-surcharge` > 0
```


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

Stop the connector:

```bash
cdk deploy shutdown --name trips-connector
```


[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
