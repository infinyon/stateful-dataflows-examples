### Compile and Run

Build Environment:
`docker build -t fluvio:sdf .`

Deploy:
`docker compose up` or 
`docker compose up -d`

See volume mappings via:
`docker volumes ls`

remove volumes with compose down:
`docker compose down --volumes` or `docker compose down -v`