# Hello World Example for Package Generation

The `_hello-world_` example demonstrates how to generate and compile a fully functional package from an `sdf-package.yaml` file. The example will utilize regex to look up and convert the word `good` to `great`.

### Prerequisites

* [Install SDF] if you haven't already.
* Look at the [sdf-package.yaml](./sdf-package.yaml) to see the type and function definitions.


### Generate Package Sandbox

Use the `generate` scaffold to create the package sandbox:

```bash
sdf generate
```

The generator reads the `sdf-package.yaml` file and creates the following Rust project:

```bash
$ tree
.
├── README.MD
├── rust
│   └── hello-world
│       ├── Cargo.toml
│       ├── README.md
│       └── src
│           ├── hello_world.rs
│           └── lib.rs
└── sdf-package.yaml
```

Each function generates a new file. In this case, we only have one function, which generated the `hello_world.rs` file.

### Add Custom Code

Open the `rust/hello-world/src/hello_world.rs` file and add the following code:

```rust
use std::sync::LazyLock;
use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"good").unwrap());

#[sdf(fn_name = "hello-world")]
pub(crate) fn hello_world(input: Good) -> Result<Great> {
    let output = REGEX.replace_all(input.as_str(), "great").to_string();
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hello_world() {
        let input = "This is good.".to_string();
        let output = hello_world(input);
        assert_eq!(output.unwrap(), "This is great.".to_string());
    }
}
```

The test we've included is not strictly necessary but a best practice.

We'll also need to update the `Cargo.toml` file to include the `regex` dependency.

```toml
[dependencies]
...
regex = "1"
```

Add `regex` to the end of the file.

### Build the Package

To build the code, run:

```bash
sdf build
```

The `build` command creates a fully functional package that can be imported by other packages or dataflows.

### Test the Code with SDF

Inside the `sdf-package.yaml` directory, run:

```bash
sdf test
```

```bash
>> test function hello-world --value "This is good."
This is great.
```

### Test the code with Cargo

You also have the option to test your routines using `rust` test utilities. Inside the `rust/hello-world` directory, run the tests using Rust:

```bash
cargo test
```

:tada: Congratulations! You have successfuly built your first package.


[Install SDF]: /README.MD#prerequisites