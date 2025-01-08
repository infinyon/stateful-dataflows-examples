# Stateful Dataflows Examples

This repository offers a comprehensive range of dataflow examples from basic to advanced levels, designed to familiarize you with the concepts and techniques essential for deploying Stateful Dataflows.

## Prerequisites

* Install and Updated Rust

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh rustup update
  ```

* Install Fluvio and SDF

  ```bash
  curl -fsS https://hub.infinyon.cloud/install/install.sh | bash fvm install sdf-beta5
  ```

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
* [mask-user-pii](/dataflows/mask-user-pii/) 
* [split-sentence](/dataflows/split-sentence/)
* [word-counter](/dataflows/word-counter/)
* [word-probe](/dataflows/word-probe/)

## Callouts

Dataflows that make external calls to send or receive data from external services:

* [http-callout](/callouts/http-callout/)
* [openai-callout](/callouts/openai-callout/)
* [clichouse-callout](/callouts/clichouse-callout/)

## Primitives

Dataflows centered around primitive concepts and operations.

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
