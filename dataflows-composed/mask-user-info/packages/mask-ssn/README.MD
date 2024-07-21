# Mask Social Security Number (SSN) Package

This is an example of building and testing a package that masks Social Security Numbers (SSNs) in a record.

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

Let's update the content of `rust/mask-ssn/src/lib.rs` file with following code:

```rust
use sdf_macros::sdf;
use regex::Regex;
use once_cell::sync::Lazy;

static SSN_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d{3}-\d{2}-\d{4}").unwrap());

#[sdf(map, package = "mask-ssn", namespace = "example")]
pub(crate) fn mask_ssn(text: String) -> Result<String, String> {
    let output = SSN_RE.replace_all(text.as_str(), "***-**-****").to_string();
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mask_ssn() {
        let input = "123-45-6789".to_string();
        let output = mask_ssn(input);
        assert_eq!(output.unwrap(), "***-**-****");
    }
}
```

Next, add `once_cell` and `regex` to the dependencies in `Cargo.toml`:

```toml
[dependencies]
...
once_cell = "1"
regex = "1"
```

### Build the Package

To build the code, run:

```bash
sdf build
```


### Test the Code

Test the code with:

```bash
sdf test
```

```bash
>> test function mask-ssn --value "My SSN is 534-34-3443."
My SSN is ***-**-****.
```

The package is ready to use in the [dataflow](../../).


[Install SDF]: /README.MD#prerequisites