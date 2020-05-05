use std::process::Command;

const LIB: &str = "deps/libfixedtimefixedpoint";

fn main() {
    Command::new("make")
        .args(&["-C", LIB, "SGX=1"])
        .status()
        .unwrap();
    println!("cargo:rustc-link-lib=static={}", "ftfp_sgx");
    println!("cargo:rustc-link-search=native={}", LIB);
    let header_file = format!("{}/ftfp.h", LIB);
    let bindings = bindgen::Builder::default()
        .header(header_file)
        .clang_arg("-DSGX")
        .whitelist_function("fix_.*")
        .whitelist_var("FIX_.*")
        .blacklist_item("FIX_INTERN.*")
        .blacklist_item("FIX_PRINT.*")
        .rustfmt_bindings(true)
        .raw_line("#![allow(non_camel_case_types)]")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/binding.rs")
        .expect("Couldn't write bindings!");
}
