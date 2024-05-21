# Overview

Simple example of using the OpenAI API to generate text.  It counds the number of attempts by model. The input is a model followed by sentence.

For example, the following input:

```
hello world
```

use default gpt-3.5-turbo model to generate text.

But the following input:

```
gpt-4 hello world
```

use gpt-4 model to generate text.

## Set up

You must have account at [OpenApi](https://platform.openai.com/docs/overview) and setup an API key.  You can then set the API key in the environment variable `OPENAI_API_KEY`.

Before you start:
* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].


## Running

Assume you have an OpenAI API key in the environment variable `OPENAI_API_KEY`

```bash
sdf run -e OPENAI_API_KEY=$OPENAI_API_KEY
```

### Inputing text

Send the sentences you want to enrich to the topic `sentence`:

```bash
fluvio produce sentence
```

```bash
gpt-4 tell me a story about boy and wolf
```

### Reading output

Consume from `output` to see the result:

```bash
fluvio consume output -B
```

```
{"model":"gpt-4","output":"Once upon a time, there was a boy who cried wolf...", "total_attempts": 1}
```

Congratulations! You've successfully built and run a dataflow!

## Clean-up

Exit `sdf` terminal and remove the topics:

```bash
fluvio topic delete sentence
fluvio topic delete output
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites