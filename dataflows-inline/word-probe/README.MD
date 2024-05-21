# Word Probe dataflow

Word Probe is a dataflow that shows how to use state across services. This example has two services. The first service reads `sentences`, divides them into words, and stores their count in an aggregate. The second service reads `words,` looks them up in the aggregate, and returns their count.


<p align="center">
 <img width="700" src="img/word-probe.jpg">
</p>

## Pipeline Primitives

The pipeline uses the following primitives:
* _map_
* _flat-map_
* _assign-key_
* _update-state_


## Step-by-step

Take a look at the [dataflow.yaml](./dataflow.yaml) to get an idea of what we're doing.

Make sure to [Install SDF and start a Fluvio cluster].

### Run the Pipeline

Run the pipeline:

```bash
sdf run --ui
```

Use `--ui` to open the Studio.

**Note:** The `lookup-word` service uses Polars, a library that has not yet been optimized, and will it make take a while the compile. We'll optimize this in a future release.


### Test the Pipeline

1. Produce sentences to in `sentences` topic:

```bash
echo "behind every great man is a woman rolling her eyes" | fluvio produce sentences
echo "the eyes reflect what is in the heart and soul" | fluvio produce sentences
echo "keep your eyes on the stars and your feet on the ground" | fluvio produce sentences
```

2. Produce words in `words` topic:

```bash
echo "eyes" | fluvio produce words
echo "stars" | fluvio produce words
echo "the" | fluvio produce words
```

3. Consume from `word-count` to check the result:

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
show state count-words/count-per-word/state --table
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

## Clean-up

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete sentences
fluvio topic delete words
fluvio topic delete word-counts
```


[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
