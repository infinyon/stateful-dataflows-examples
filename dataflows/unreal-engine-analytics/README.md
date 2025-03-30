# Unreal Engine Analytics Dataflow

This dataflow processes real-time analytics data from Unreal Engine 5 games using the InfinyOn Analytics Plugin. The system captures player events and game telemetry, processes them using stateful dataflows, and visualizes the results in real-time dashboards.

## The Dataflow

The system consists of three main dataflows that process game analytics data:

1. **Map Events**: Processes player location data (X, Y coordinates) for visualization
2. **Counters**: Standardizes and counts game events (jumps, faults, etc.)
3. **Table Sink**: Aggregates event counts in 10-second tumbling windows for dashboard updates

<p align="center">
 [Add your dataflow diagram here]
</p>

## Prerequisites

* An InfinyOn Cloud account with a running Fluvio cluster
* A UE5 game configured with the InfinyOn Analytics Plugin
* WebSocket gateway access keys (1 producer key, 2 consumer keys)

## Step-by-step Setup

### 1. Create WebSocket Gateway Keys

Generate the required access keys for your topics:

```bash
fluvio cloud access-key create key-name --topic topic-name --produce
fluvio cloud access-key create key-name --topic topic-name --consume
```

### 2. Deploy the Dataflows

Deploy all three dataflows using the SDF command:

```bash
sdf deploy
```

### 3. Configure Visualization

Navigate to the [visualization](./visualization/) directory and:
1. Open the HTML files in a browser
2. Input the WebSocket consumer keys when prompted
3. The dashboard will update in real-time as events flow from your game

## Dataflow Components

### Map Events Dataflow
Processes raw location events from the game:
```yaml
meta:
  name: map-event
  version: 0.1.0
  namespace: unreal
# ... existing code ...
```

### Counters Dataflow
Tracks and aggregates game events:
```yaml
meta:
  name: counters
  version: 0.1.0
  namespace: unreal
# ... existing code ...
```

### Flush Dataflow
Materializes event counts every 10 seconds:
```yaml
meta:
  name: flush-experiment
  version: 0.1.0
  namespace: unreal
# ... existing code ...
```

## Visualization

The dashboard provides:
- Real-time player movement traces
- Event count summaries
- Session analytics

The visualization files are located in the [visualization](./visualization/) directory.

## Additional Resources

- [UE5 Plugin Repository](https://github.com/fluvio-community/TuDiAnalyticsTest)
- [InfinyOn Cloud Documentation](https://www.fluvio.io/docs/cloud/quickstart)
- [WebSocket Gateway Guide](https://www.fluvio.io/docs/cloud/demos/ws-gateway)

## Clean-up

To stop the dataflows:

```bash
stop df [df-name]
delete df [df-name]
exit
sdf clean
```

Note: This dataflow requires InfinyOn Cloud and cannot be run locally due to WebSocket gateway dependencies.
