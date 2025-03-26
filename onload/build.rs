#[cfg(feature = "debug")]
extern crate bindgen;
#[cfg(feature = "debug")]
use std::fs::OpenOptions;
#[cfg(feature = "debug")]
use std::io::Write;
#[cfg(feature = "debug")]
use std::path::PathBuf;
#[cfg(feature = "debug")]
fn generate_binding() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings")
        .to_string();
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    println!("{}", out_path.display());
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(&out_path)
        .expect("can't open bindings file");
    file.write(b"#[cfg(windows)]\ncompile_error!(\"onload does not support windows platform\");")
        .unwrap();
    file.write(bindings.as_bytes()).unwrap();
}

fn main() {
    #[cfg(feature = "debug")]
    generate_binding();
    // link library
    println!("cargo:rustc-link-lib=onload_zf");
    println!("cargo:rustc-link-lib=ciul1");
}
