# Mask Social Security Number (SSN) Package

This is an example of building and testing a package that masks Social Security Numbers (SSNs) in a record.


### Package Updates

We've used the [sdf-package.yaml](./sdf-package.yaml) to generate the project, and change the following files.

* [rust/mask-ssn-pkg/src/mask_ssn.rs](./rust/mask-ssn-pkg/src/mask_ssn.rs)
* [rust/mask-ssn-pkg/src/Cargo.thml](./rust/mask-ssn-pkg/Cargo.toml)


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
>> test function mask-ssn --value "My SSN is 534-34-3443."
My SSN is ***-**-****.
```

### Test the code with `Cargo`

Navigate to the `rust` and run the tests using Rust:

```bash
cd rust/mask-ssn-pkg
cargo test
```

### Package Ready

After building the package, it is ready to use in [dataflows](../dataflow).


[Install SDF]: /README.MD#prerequisites
