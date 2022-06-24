#[cfg(feature = "debug")]
extern crate bindgen;
#[cfg(feature = "debug")]
use std::path::PathBuf;

#[cfg(feature = "debug")]
fn generate_binding() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result or panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    #[cfg(feature = "debug")]
    generate_binding();

    // link library
    println!("cargo:rustc-link-lib=onload_zf");
    println!("cargo:rustc-link-lib=ciul1");
}
