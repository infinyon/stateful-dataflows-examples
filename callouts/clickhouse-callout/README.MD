# Clickhouse Callout Example

Callout example on sending and retrieving Information from Clickhouse using SDF operators.

### Overview

In this example, we will perform the following operations:

1. Provision Clickhouse locally and create a table.
2. Start a Fluvio HTTP connector to read events from the New York transit dataset and send them to a topic.
3. Start a dataflow that synchronizes the events with Clickhouse.

### Dataflow Services

The dataflow includes the following services:

1. Read events from the transit topic and create an SDF table that aggregates the events in 20-second intervals.
2. Send the aggregated table to Clickhouse.
3. Read historical aggregates from Clickhouse and publish trends.


