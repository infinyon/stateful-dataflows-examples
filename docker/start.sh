# Build Environment
docker build  --build-arg FLUVIO_VERSION=sdf-beta12 -t fluvio:sdf . 

# Run SDF in Docker
docker compose up -d

# Download profile to the CLI
fluvio profile add sdf-docker 127.0.0.1:9103 docker

# Create a worker profile
sdf worker register docker-worker sdf-worker