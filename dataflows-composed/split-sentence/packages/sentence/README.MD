# Sentence Package

The `sentence` package offers a couple of functions to work with sentences:

* sentence-to-workds - splits a sentence into individual words.
* augment-count - augments the sentence with the number of charaters.

### Prerequisites

* Checkout the [sdf-package.yaml](./sdf-package.yaml) to see the functions we'll be creating.
* Make sure to [Install SDF].


### Generate the Package Sandbox

Use the `generate` scaffold to create the package sandbox:

```bash
sdf generate
```

The generator creates all necessary glue logic for a reusable package.


### Add Custom Code

Update the code for the APIs defined.

Replace the `sentence_to_words` function body in the `rust/sentence-to-words/src/lib.rs` with the following code:

```rust
pub(crate) fn sentence_to_words(sentence: String) -> Result<Vec<String>, String> {
    Ok(sentence.split_whitespace().map(String::from).collect())
}

```

In `rust/augment-count/src/lib.rs` replace the function body:

```rust
pub(crate) fn augment_count(word: String) -> Result<String, String> {
    Ok(format!("{}({})", word, word.chars().count()))
}
```

Let's add some tests as well:

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_augment_count() {
        let input = "Hello".to_string();
        let output = augment_count(input);
        assert_eq!(output.unwrap(), "Hello(5)");
    }
}
```

### Build the Package

To build the code, run:

```bash
sdf build
```

### Test the Functions

Test the code with `sdf`:

```bash
sdf test
```

Show the functions avaialble for testing:

```bash
>> show functions
sentence-to-words
augment-count
```

Test `sentence-to-words` function:

```bash
>> test function sentence-to-words --input "Hello World"
Hello
World
```

Test `augment-count` function:

```bash
>> test function augment-count --input "Hello"
Hello(5)
```

To test the `rust` code via Cargo, perform the following steps:

```bash
cd rust
cargo test
```

The package is ready to use in the [dataflow](../../).

[Install SDF]: /README.MD#prerequisites