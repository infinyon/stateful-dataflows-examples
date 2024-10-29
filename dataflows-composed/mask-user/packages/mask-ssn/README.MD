# Mask Social Security Number (SSN) Package

This is an example of building and testing a package that masks Social Security Numbers (SSNs) in a record.

We've used the [sdf-package.yaml](./sdf-package.yaml) to generate the package and updated the following files:

* [rust/mask-ssn-pkg/src/mask_ssn.rs](./rust/mask-ssn-pkg/src/mask_ssn.rs)
* [rust/mask-ssn-pkg/src/Cargo.thml](./rust/mask-ssn-pkg/src/Cargo.toml)



### Build the Package

To build the code, run:

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
cd rust
cargo test
```

### Package Ready for use

The package is ready to use in the [dataflow](../../).


[Install SDF]: /README.MD#prerequisites
