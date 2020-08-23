use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn make_bits_file(b: usize, out_dir: &Path) {
    let dest_path = out_dir.join("bits.rs");
    fs::write(
        &dest_path,
        format!(
            "pub const FIX_FLAG_BITS: u32 = 2; \
            pub const FIX_FRAC_BITS: u32 = {}; \
            pub const FIX_INT_BITS: u32 = {};",
            60 - b,
            b
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}

fn main() {
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();

    let out_dir_str = env::var("OUT_DIR").unwrap();
    let curr_dir_str = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_dir = Path::new(&out_dir_str);
    let curr_dir = Path::new(&curr_dir_str);
    let sys_dir = curr_dir.join(Path::new("ftfp-sys"));
    let lib_dir = out_dir.join(Path::new("lib"));

    if let Ok(b) = env::var("FTFP_INTBITS") {
        let b: usize = b.parse().expect("FTFP_INTBITS is not a valid integer.");
        if b > 60 {
            panic!("FTFP_INTBITS must be <= 60.")
        }
        make_bits_file(b, out_dir);
    } else {
        make_bits_file(32, out_dir);
    }

    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());

    if target_env == "sgx" && target_vendor == "fortanix" {
        println!("cargo:rustc-link-lib=static={}", "ftfp_sgx");
        Command::new("make")
            .current_dir(sys_dir.clone())
            .arg(format!("BUILD_DIR={}", out_dir.to_str().unwrap()))
            .arg("SGX=1")
            .status()
            .unwrap();
    } else {
        println!("cargo:rustc-link-lib=dylib={}", "ftfp");
        println!(
            "cargo:rustc-env=LD_LIBRARY_PATH={}",
            lib_dir.to_str().unwrap()
        );
        Command::new("make")
            .current_dir(sys_dir.clone())
            .arg(format!("BUILD_DIR={}", out_dir.to_str().unwrap()))
            .status()
            .unwrap();
    };
}
