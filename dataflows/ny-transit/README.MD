# NY Transit dataflow

New York transit dataflow shows how join and query data sets for stream analytics use cases.


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


[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
