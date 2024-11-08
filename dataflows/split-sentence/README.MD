# Split Sentence

The split-sentence `dataflow` breaks sentences into words and returns two streams, one with word events and the other with the number of characters for each word. The following diagram is a visual representation of these operations:

<p align="center">
 <img width="650" src="img/split-sentence.jpg">
</p>


#### Package Variant

If you prefer to run this dataflow using packages, run the [package-variant](./package-variant/README.md) instead.


## Step-by-step

Take a look at the [dataflow.yaml](./dataflow.yaml) to see how we've implemented it.


### Run the Dataflow

Use `sdf` command line tool to run the dataflow:

```bash
sdf run --ui
```

Use `--ui` to generate the graphical representation and run the Studio.


### Test the Dataflow

The sample data file used to run this test `./sample-data/data.txt` has the following records:

```bash
Hello world
Hi there
```

Produce the data to the `sentences` topic:

```bash
fluvio produce sentences -f ./sample-data/data.txt
```

Checkout the data in `sentences` topic:

```bash
fluvio consume sentences -Bd
```

Consume from `words` to retrieve the result:

```bash
fluvio consume words -Bd
```

```bash
Hello
world
Hi
there
```

Consume from `word-counts` to retrieve the result:

```bash
fluvio consume word-counts -Bd
```

```bash
5
5
2
5
```

### Show State

Check the counters `sdf` terminal:

```bash
show state sentence-words/sentence-to-words/metrics
```

```bash
show state word-counts/word-length/metrics
```


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```


### References

* [Deploy a Dataflow in Production](https://www.fluvio.io/sdf/deployment)
