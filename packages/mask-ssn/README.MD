# Mask Social Security Number (SSN) Package

Example package that uses a third party library (Regex) to mask social security numbers.


### Package Implementation

We used the `sdf generate` command with the [sdf-package.yaml](./sdf-package.yaml) file to generate the project and then applied code changes to the following files:

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

The following command will test the code with `cargo`:

```bash
(cd rust/mask-ssn-pkg && cargo test)
```

### Package Ready

After building the package, it is ready to use in [dataflows](../dataflow).


[Install SDF]: /README.MD#prerequisites
