# Word Counter dataflow

Word Count is a dataflow for window processing that analyzes sentences and computes the three most frequently used words in 20-second intervals.

<p align="center">
 <img width="700" src="img/word-counter.jpg">
</p>

## Dataflow Primitives

The dataflow uses the following primitives:
* _window_
  * _assign-timestamp_
  * _assign-key_
  * _update-state_
  * _flush_
* _flat-map_
* _states_


## Step-by-step

Take a look at the [dataflow.yaml](./dataflow.yaml) to get an idea of what we're doing.

Make sure to [Install SDF and start a Fluvio cluster].

### 1. Run the Dataflow

Use `sdf` command line tool to run the dataflow:

Run the dataflow:

```bash
sdf run --ui
```

Use `--ui` to open the Studio.


### 2. Test the Dataflow

#### Produce to Source Topic

Produce sentences to in `sentence` topic:

```bash
echo "behind every great man is a woman rolling her eyes" | fluvio produce sentence
```

```bash
echo "the eyes reflect what is in the heart and soul" | fluvio produce sentence
```

```bash
echo "keep your eyes on the stars and your feet on the ground" | fluvio produce sentence
```

```bash
echo "obstacles are those frightful things you see when you take your eyes off your goal" | fluvio produce sentence
```

**Note:**

> The watermark closes the window on event. Hence, if you stop adding events before the watermark triggers, the window stays open, and nothing gets produced. This is the expected behavior as, in most cases, external entities will continuously produce data, and you won’t be left with an open window. To force the window to flush, produce one more record after the 20-second mark.

In subsequent releases, we’ll add an idle watermark trigger to cover manual testing.

#### Consume from Sink Topic

Consume from `most-used-words` topic:

```bash
fluvio consume most-used-words -B -O json
```

Depending when the 20 second window triggered, you'll see the top 3 words:

```bash
[
  {
    "count": 2,
    "word": "is"
  },
  {
    "count": 2,
    "word": "the"
  },
  {
    "count": 2,
    "word": "eyes"
  }
]
```

#### Check the state

To watch how the window is gradually populated:

```bash
show state
```

Then show the window state:

```bash
show state word-processing-window/count-per-word/state
```

**Note:** The dataflow stops processing records when you close the intractive editor. To resume processing, run `sdf run` again.

Congratulations! You've successfully built and run a dataflow!


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```


#### Produce Continuous Data (optional)

You may also start an http-source connector that continuously feed data to the `sentence` topic. For instructions, checkout [connectors](./connectors/).



[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
