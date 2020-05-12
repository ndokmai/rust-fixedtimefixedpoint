# rust-fixedtimefixedpoint
Constant-time mathematical operations on fixed point numbers. Rust wrapper for libfixedtimefixedpoint.
## Build
``` bash
git submodule update --init --recursive
export FTFP_INTBITS={desired intbits}
cargo build
```

This can also be build to target x86_64-fortanix-unknown-sgx:

``` bash
git submodule update --init --recursive
export FTFP_INTBITS={desired intbits}
cargo build --target x86_64-fortanix-unknown-sgx
```
