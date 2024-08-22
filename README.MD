# Stateful Dataflows Examples

This repository offers a comprehensive range of dataflow examples from basic to advanced levels, designed to familiarize you with the concepts and techniques essential for deploying Stateful Dataflows.

## Prerequisites

#### Install & Update Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

#### Install Fluvio & SDF

SDF is the abreviation for Stateful Dataflows.

```bash
curl -fsS https://hub.infinyon.cloud/install/install.sh | bash
fvm install sdf-beta1
```

#### Start a cluster

```bash
fluvio cluster start
```

## DataFlows (Inline)

DataFlows defined inline:

* [bank-processing](/dataflows-inline/bank-processing/)
* [car-processing](/dataflows-inline/car-processing/)
* [hackernews-notify](/dataflows-inline/hackernews-notify/)
* [http-callout](/dataflows-inline/http-callout/)
* [openai-callout](/dataflows-inline/openai-callout/)
* [split-sentence](/dataflows-inline/split-sentence/)
* [word-counter](/dataflows-inline/word-counter/)
* [word-probe](/dataflows-inline/word-probe/)

## DataFlows (Composed)

DataFlows defined via composable packages:

* [mask-user-info](/dataflows-composed/mask-user-info/)
* [split-sentence](/dataflows-composed/split-sentence/)

## Primitives

Simple examples centered around primitive concepts and operations.

* [map](/primitives/map/)
* [filter](/primitives/filter/)
* [filter-map](/primitives/filter-map/)
* [flat-map](/primitives/flat-map/)
* [split (filter)](/primitives/split/filter)
* [split (filter-map)](/primitives/split/filter-map)
* [merge](/primitives/merge/)
* [regex](/primitives/regex/)
* [update-state](/primitives/update-state/)
* [key-value (input)](/primitives/key-value/input)
* [key-value (output)](/primitives/key-value/output)
* [key-value (chained)](/primitives/key-value/chained)

## References
* [connectors](connectors.md)
