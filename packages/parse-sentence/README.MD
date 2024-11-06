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
>> test function sentence-to-words --value "Hello, World!"
["]
```

### Test the code with `Cargo`

Navigate to the `rust` and run the tests using Rust:

```bash
cd rust/mask-ssn-pkg && cargo test
```

### Package Ready

After building the package, it is ready to use in [dataflows](../dataflow).


[Install SDF]: /README.MD#prerequisites
