### Compile and Run

* Build Environment:

  ```bash
  docker build -t fluvio:sdf .
  ```

* Deploy:

  ```bash
  docker compose up -d
  ```

* Remove (including volumes)

  ```bash
  docker compose down --volumes
  ```


### Miscellaneous

See volume mappings via:

```bash
docker volume ls
```
