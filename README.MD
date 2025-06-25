# Stateful Dataflows Examples

This repository offers a comprehensive range of dataflow examples from basic to advanced levels, designed to familiarize you with the concepts and techniques essential for deploying Stateful Dataflows.

## Prerequisites

* Install and Updated Rust

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh rustup update
  ```

* Install Fluvio and SDF

  ```bash
  curl -fsS https://hub.infinyon.cloud/install/install.sh | bash fvm install sdf-beta11
  ```

* Run SDF setup to install dependencies

```bash
sdf setup
```

You might be prompted to install wasm dependency. Copy paste the command and run.

* Start Cluster

  ```bash
  fluvio cluster start
  ```


## DataFlows

Dataflow examples:

* [bank-processing](/dataflows/bank-processing/)
* [car-processing](/dataflows/car-processing/)
* [hackernews-notify](/dataflows/hackernews-notify/)
* [helsinki-transit](/dataflows/helsinki-transit/)
* [ny-transit](/dataflows/ny-transit/)
* [http-callout](/dataflows/http-callout/)
* [mask-user-pii](/dataflows/mask-user-pii/) 
* [openai-callout](/dataflows/openai-callout/)
* [split-sentence](/dataflows/split-sentence/)
* [word-counter](/dataflows/word-counter/)
* [word-probe](/dataflows/word-probe/)


## Primitives

Examples centered around primitive concepts and operations.

* [map](/primitives/map/)
* [filter](/primitives/filter/)
* [filter-map](/primitives/filter-map/)
* [flat-map](/primitives/flat-map/)
* [split (filter)](/primitives/split/filter)
* [split (filter-map)](/primitives/split/filter-map)
* [merge](/primitives/merge/)
* [regex](/primitives/regex/)
* [sql (cli)](/primitives/sql/cli)
* [sql (join)](/primitives/sql/join)
* [update-state](/primitives/update-state/)
* [key-value (input)](/primitives/key-value/input)
* [key-value (output)](/primitives/key-value/output)
* [key-value (chained)](/primitives/key-value/chained)


## Packages

Step-by-step instructions on how to build your own package:

* [_hello-world_](/packages/_hello-world_/)

Pre-built packages used in dataflows:

* [mask-ssn](/packages/mask-ssn)
* [parse-sentence](/packages/parse-sentence)


## References
* [connectors](connectors.md)

## Troubleshooting
Stateful DataFlow workers continue to run even if you remove a fluvio cluster. If you delete a cluster and start a new one, or if you unintentionally start a dataflow in a cluster and need to clean up the workers, you can use the following command:

```bash
ps aux | grep sdf
kill -9 <PID>
rm -rf ~/.sdf/local/
```
In the above example `local` is the cluster name & profile name. If you have a different cluster name, replace `local` with your cluster name.