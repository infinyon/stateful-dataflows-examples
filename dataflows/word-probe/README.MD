# Word Probe dataflow

Word Probe is a dataflow that shows how to use state across services. This example has two services. The first service reads `sentences`, divides them into words, and stores their count in an aggregate. The second service reads `words,` looks them up in the aggregate, and returns their count.


<p align="center">
 <img width="700" src="img/word-probe.jpg">
</p>

## Dataflow Primitives

The dataflow uses the following primitives:
* _map_
* _flat-map_
* _assign-key_
* _update-state_


## Step-by-step

Take a look at the [dataflow.yaml](./dataflow.yaml) to get an idea of what we're doing.

Make sure to [Install SDF and start a Fluvio cluster].

### Run the Dataflow

Use `sdf` command line tool to run the dataflow:

```bash
sdf run --ui
```

Use `--ui` to open the Studio.

**Note:** The `lookup-word` service uses Polars, a library that has not yet been optimized, and will it make take a while the compile. We'll optimize this in a future release.


### Test the Dataflow

For this example, we'll use the following data files: `./sample-data/sentences.txt` and `./sample-data/words.txt`.

```bash
# sentences.txt
behind every great man is a woman rolling her eyes
the eyes reflect what is in the heart and soul
keep your eyes on the stars and your feet on the ground

# words.txt
eyes
stars
the
```

Produce the data to the `sentences` and `words` topics:

```bash
fluvio produce sentences -f ./sample-data/sentences.txt
fluvio produce words -f ./sample-data/words.txt
```

Observe the data produced in both topics:

```bash
fluvio consume sentences -Bd
fluvio consume words -Bd
```

Consume from `word-counts` topic to check the result:

```bash
fluvio consume word-counts -Bd -O json
```

You should see something like this:

```json
{
  "count": 3,
  "word": "eyes"
}
{
  "count": 1,
  "word": "stars"
}
{
  "count": 4,
  "word": "the"
}
```

#### Check the state

Use the `show state` in `sdf` terminal to watch the internal state of the windows:

```bash
show state count-words/count-per-word/state
```

You should see something like this:

```text
 Key      count
 a        1
 and      2
 behind   1
 every    1
 eyes     3
 feet     1
 great    1
...
```

**Note:** The dataflow stops processing records when you close the intractive editor. To resume processing, run `sdf run` again.

Congratulations! You've successfully built and run a dataflow!


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
