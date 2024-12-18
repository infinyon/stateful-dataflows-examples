# SQL (cli & query)

In this example, we'll collect `vehicle-data` records from a `vehicle-sensor` topic, store them in a `vehicle_data` state, and query the data using the SQL CLI interface.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

The sample data file used to run this test `./sample-data/data.txt` has lines following the JSON format:

```json
{"timestamp": "2023-11-22T12:34:56Z", "vehicle_id": "V001", "latitude": 40.7128, "longitude": -74.0060, "speed": 60, "engine_temperature": 90, "engine_rpm": 2000, "fuel_consumption": 10, "sensor_status": "ok" }
```

Produce the data to the `vehicle-sensor` topic:

```bash
fluvio produce vehicle-sensor -f ./sample-data/data.txt
```

Checkout the data in `vehicle-sensor` topic:

```bash
fluvio consume vehicle-sensor -Bd
```

### Run SQL commands


On the `sdf` runtime terminal, first enter the SQL mode:

```bash
>> sql
```

List the tables using `show tables` command:

```sql
sql >> show tables
shape: (1, 1)
┌──────────────┐
│ name         │
│ ---          │
│ str          │
╞══════════════╡
│ vehicle_data │
└──────────────┘
```

Query the `vehicle_data` table:

```sql
sql >> select * from vehicle_data
shape: (5, 6)
┌──────┬────────────────────┬──────────────────┬──────────┬───────────┬───────────────┐
│ _key ┆ engine_temperature ┆ fuel_consumption ┆ latitude ┆ longitude ┆ sensor_status │
│ ---  ┆ ---                ┆ ---              ┆ ---      ┆ ---       ┆ ---           │
│ str  ┆ i32                ┆ u32              ┆ f64      ┆ f64       ┆ str           │
╞══════╪════════════════════╪══════════════════╪══════════╪═══════════╪═══════════════╡
│ V001 ┆ 90                 ┆ 10               ┆ 40.7128  ┆ -74.006   ┆ ok            │
│ V003 ┆ 85                 ┆ 8                ┆ 34.0522  ┆ -118.2437 ┆ failed        │
│ V005 ┆ 85                 ┆ 8                ┆ 34.0522  ┆ -118.2437 ┆ ok            │
│ V004 ┆ 85                 ┆ 8                ┆ 34.0522  ┆ -118.2437 ┆ failed        │
│ V002 ┆ 85                 ┆ 8                ┆ 34.0522  ┆ -118.2437 ┆ failed        │
└──────┴────────────────────┴──────────────────┴──────────┴───────────┴───────────────┘
```

Perform a query to filter the data:

```sql
sql >> select _key from vehicle_data where sensor_status = 'failed';
shape: (3, 1)
┌──────┐
│ _key │
│ ---  │
│ str  │
╞══════╡
│ V003 │
│ V004 │
│ V002 │
└──────┘
```

Exit the SQL mode:

```bash
sql >> .exit
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
