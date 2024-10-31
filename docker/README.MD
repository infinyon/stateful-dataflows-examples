## Docker Scripts

Docker compose start the fluvio cluster and an SDF worker.

### Start

Built a docker image, run the compose and create client profiles:

```bash
./start.sh
```

### Shutdown

Run docker compose down

```bash
./shutdown.sh
```

### Nuke

Run docker compose down and remove volumes:

```bash
./nuke.sh
```