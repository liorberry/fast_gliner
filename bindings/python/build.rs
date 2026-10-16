fn main() {
    // onnxruntime's prebuilt aarch64 static lib references __aarch64_cas8_sync,
    // a GCC outline-atomics helper that lives only in libgcc.a (hidden symbol,
    // not exported by libgcc_s.so.1). rustc links with -nodefaultlibs so
    // libgcc.a is never searched; pull it in explicitly for this target only.
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_arch == "aarch64" && target_os == "linux" {
        println!("cargo:rustc-link-arg=-lgcc");
    }
}
