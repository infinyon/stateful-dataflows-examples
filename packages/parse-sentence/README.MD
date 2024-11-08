# Prase Sentence Package

Example package that implements multiple functions for sentence and word processing operations.


### Package Implementation

We used the `sdf generate` command with the [sdf-package.yaml](./sdf-package.yaml) file to generate the project and then applied code changes to the following files:

* [rust/parse-sentence/src/sentence_to_words.rs](./rust/parse-sentence/src/sentence_to_words.rs)
* [rust/parse-sentence/src/word_length.rs](./rust/parse-sentence/src/word_length.rs)


### Build the Package

Generate package binary:

```bash
sdf build
```

### Test the Code with SDF

Test the code with:

```bash
sdf test
```

```bash
>> test function sentence-to-words --value "\"Hello, World!"\"
Hello,
World!
```

```bash
>> test function word-length --value "\"Hello,"\"
6
```

### Test the code with `Cargo`

The following command will test the code with `cargo`:

```bash
(cd rust/parse-sentence && cargo test)
```

### Package Ready

After building the package, it is ready for use in dataflows:
* [split-sentence](../../dataflows/split-sentence/package-variant/README.MD)

[Install SDF]: /README.MD#prerequisites
