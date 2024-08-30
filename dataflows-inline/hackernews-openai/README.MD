# HackerNews Word Match Notifications 

Hackernews Notify is a dataflow that watches HackerNews articles against a database of words and the email of the interested parties. When a match is found, the dataflow generates an event with the word, article, and email address of the interested party.

<p align="center">
 <img width="700" src="img/hackernews.jpg">
</p>


## Step-by-step

Take a look at the [dataflow.yaml](./dataflow.yaml) to get an idea of what we're doing.

Make sure to [Install SDF and start a Fluvio cluster].

### 1. Run the Dataflow

Use `sdf` command line tool to run the dataflow:

Run the dataflow:

```bash
sdf run --ephemeral --ui
```

Use `--ui` to open the Studio.


### 2. Start the connector:


In a new terminal change direcotry to `./connectors`, download the connector binary, and deploy it:

```bash
cd ./connectors
cdk hub download infinyon/http-source@0.3.8
cdk deploy start --ipkg infinyon-http-source-0.3.8.ipkg --config ./hackernews.yaml
```

To see the events, run `fluvio consume hackernews -B`. The connector reads the RSS every 60 seconds.

For additional context, checkout [connectors](./connectors/).

### 3. Test the Dataflow

#### Produce Requests to `user-request` topic

Let's add some generic words that we hope to find in the HackerNews article title.


Produce the following entries in `user-request` topic:

```bash
echo '{"word": "from", "email":"alice@test.com"}' | fluvio produce user-request
echo '{"word": "from", "email":"bob@test.com"}' | fluvio produce user-request
echo '{"word": "the", "email":"bob@test.com"}' | fluvio produce user-request
echo '{"word": "AI", "email":"bob@test.com"}' | fluvio produce user-request
```

#### Consume from Hacker new events

If any of the words match, you should see records in the `notify-event` topic:

```bash
fluvio consume notify-event -B -O json
```

In this example `from` matched:

```bash
{
  "email": "alice@test.com",
  "link": "https://arxiv.org/abs/2407.20292",
  "title": From pixels to planning: scale-free active inference",
  "word": "from"
}
{
  "email": "bob@test.com",
  "link": "https://arxiv.org/abs/2407.20292",
  "title": "From pixels to planning: scale-free active inference",
  "word": "from"
}
```

### Consuming from Smart Events

```
fluvio consume smart-event -O json
```

#### Check the state

To watch how the window is gradually populated:

```bash
show state
```

See the notify-table state to see the words and associated emails:

```bash
show state process-user-request/notify-table/state
```

Congratulations! You've successfully built and run a dataflow!


### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

Stop the connector:

```bash
cdk deploy shutdown --name hackernews
```
