docker build -t fluvio:sdf .

See volume mappings via:
`docker volumes ls`

remove volumes with compose down:
`docker compose down --volumes` or `docker compose down -v`
