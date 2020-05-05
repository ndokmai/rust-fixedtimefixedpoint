use std::process::Command;
use std::path::Path;
use std::env;

const LIB: &str = "deps/libfixedtimefixedpoint";

fn main() {
    let curr_dir_str = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let curr_dir = Path::new(&curr_dir_str);
    let lib_dir = curr_dir.join(Path::new(LIB));

    Command::new("git").arg("submodule").arg("update").arg("--init")
        .current_dir(&curr_dir)
        .status()
        .unwrap();
    
    Command::new("make")
        .args(&["-C", lib_dir.to_str().unwrap()])
        .arg("SGX=1")
        .status()
        .unwrap();

    println!("cargo:rustc-link-lib=static={}", "ftfp_sgx");
    println!("cargo:rustc-link-search=native={}", lib_dir.to_str().unwrap());

    let header_file = lib_dir.join(Path::new("ftfp.h"));
    let bindings = bindgen::Builder::default()
        .header(header_file.to_str().unwrap())
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
