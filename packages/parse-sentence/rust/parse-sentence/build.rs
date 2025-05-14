fn main() {
    println!("cargo:rerun-if-changed=./../../sdf-package.yaml");
    println!("cargo:rerun-if-env-changed=SDF_PROD_MODE");
}
