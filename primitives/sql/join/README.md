# SQL (simple query)

In this example, we'll count the occurrences of words in two topics. We will use the CLI interface to run some SQL queries and then use a JOIN query on a service to find the number of occurrences of a word in both topics.

* Checkout the [dataflow.yaml](./dataflow.yaml).
* Make sure to [Install SDF and start a Fluvio cluster].

### Run DataFlow

With the `dataflow.yaml` file in the current directory, run the following commands:

```bash
sdf run
```

### Test DataFlow

The sample data files used to run this test are in `./sample-data`.

Produce the data to the `source-a` topic:

```bash
fluvio produce source-a -f ./sample-data/source-a.txt
```

Checkout the data in `source-a` topic:

```bash
fluvio consume source-a -Bd
```

Produce the data to the `source-b` topic:

```bash
fluvio produce source-b -f ./sample-data/source-b.txt
```

Checkout the data in `source-b` topic:

```bash
fluvio consume source-b -Bd
```

### Run SQL commands


On the `sdf` runtime terminal, first enter the SQL mode:

```bash
>> sql
```

List the tables using `show tables` command:

```sql
sql >> show tables
shape: (2, 1)
┌──────────────────┐
│ name             │
│ ---              │
│ str              │
╞══════════════════╡
│ count_per_word_a │
│ count_per_word_b │
└──────────────────┘
```

Query the `count_per_word_a` table:

```sql
sql >> select * from count_per_word_a order by occurrences desc limit 6
shape: (6, 2)
┌──────┬─────────────┐
│ _key ┆ occurrences │
│ ---  ┆ ---         │
│ str  ┆ i32         │
╞══════╪═════════════╡
│ the  ┆ 16          │
│ of   ┆ 14          │
│ was  ┆ 13          │
│ it   ┆ 11          │
│ his  ┆ 7           │
│ in   ┆ 4           │
└──────┴─────────────┘
```

Perform a JOIN query using the `count_per_word_a` and `count_per_word_b` tables:

```sql
select a._key, a.occurrences as count_a, b.occurrences as count_b from count_per_word_a a join count_per_word_b b on a._key = b._key order by count_a desc limit 6
shape: (6, 3)
┌──────┬─────────┬─────────┐
│ _key ┆ count_a ┆ count_b │
│ ---  ┆ ---     ┆ ---     │
│ str  ┆ i32     ┆ i32     │
╞══════╪═════════╪═════════╡
│ the  ┆ 16      ┆ 13      │
│ of   ┆ 14      ┆ 8       │
│ it   ┆ 11      ┆ 2       │
│ his  ┆ 7       ┆ 4       │
│ in   ┆ 4       ┆ 2       │
│ to   ┆ 3       ┆ 4       │
└──────┴─────────┴─────────┘
```

Exit the SQL mode:

```bash
sql >> .exit
```

After running the SQL commands, you can see the results of the JOIN query in the `look-up-word` service. The `count_per_word_a` and `count_per_word_b` tables are joined on the `_key` column to find the number of occurrences of a word in both topics.

Produce to `probe-word` topic to see the result:

```bash
fluvio produce probe-word -f ./sample-data/probe-word.txt
```

Check the output in the `word-count` topic:

```bash
$ fluvio consume word-count -Bd
"key: the count: 29"
"key: in count: 6"
"key: into count: 3"
"key: love count: 2"
"key: fluvio count: 0"
```

### Clean-up

Exit `sdf` terminal and clean-up. The `--force` flag removes the topics:

```bash
sdf clean --force
```

[Install SDF and start a Fluvio cluster]: /README.MD#prerequisites
