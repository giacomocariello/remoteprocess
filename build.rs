use std::env;

fn main() {
    // We only support native unwinding on some platforms
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match target_arch.as_str() {
        "x86_64" | "arm" => {}
        _ => return,
    };
    let target = env::var("TARGET").unwrap();

    match env::var("CARGO_CFG_TARGET_OS").unwrap().as_ref() {
        "linux" => {
            // statically link libunwind if compiling for musl, dynamically link otherwise
            if env::var("CARGO_FEATURE_UNWIND").is_ok() {
                println!("cargo:rustc-cfg=use_libunwind");
                let libunwind_libdir = env::var("LIBUNWIND_LIBDIR").unwrap_or(format!("/usr/local/musl/{}/lib", target));
                let libz_libdir = env::var("LIBZ_LIBDIR").unwrap_or(format!("/usr/local/musl/{}/lib", target));
                let liblzma_libdir = env::var("LIBLZMA_LIBDIR").unwrap_or(format!("/usr/local/musl/{}/lib", target));
                if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "musl"
                    && env::var("CARGO_CFG_TARGET_VENDOR").unwrap() != "alpine"
                {
                    println!("cargo:rustc-link-search=native=/usr/local/lib");

                    let out_dir = env::var("OUT_DIR").unwrap();
                    std::fs::copy(
                        format!("{}/libunwind.a", libunwind_libdir),
                        format!("{}/libunwind-remoteprocess.a", out_dir),
                    )
                    .unwrap();
                    std::fs::copy(
                        format!("{}/libunwind-ptrace.a", libunwind_libdir),
                        format!("{}/libunwind-ptrace.a", out_dir),
                    )
                    .unwrap();
                    std::fs::copy(
                        format!("{}/libunwind-{}.a", libunwind_libdir, target_arch),
                        format!("{}/libunwind-{}.a", out_dir, target_arch),
                    )
                    .unwrap();
                    std::fs::copy(
                        format!("{}/libz.a", libz_libdir),
                        format!("{}/libz.a", out_dir),
                    )
                    .unwrap();
                    std::fs::copy(
                        format!("{}/liblzma.a", liblzma_libdir),
                        format!("{}/liblzma.a", out_dir),
                    )
                    .unwrap();
                    println!("cargo:rustc-link-search=native={}", out_dir);
                    println!("cargo:rustc-link-lib=static=unwind-remoteprocess");
                    println!("cargo:rustc-link-lib=static=unwind-ptrace");
                    println!("cargo:rustc-link-lib=static=unwind-{}", target_arch);
                    println!("cargo:rustc-link-lib=static=z");
                    println!("cargo:rustc-link-lib=static=lzma");
                } else {
                    println!("cargo:rustc-link-lib=dylib=unwind");
                    println!("cargo:rustc-link-lib=dylib=unwind-ptrace");
                    println!("cargo:rustc-link-lib=dylib=unwind-{}", target_arch);
                }
            }
        }
        _ => {}
    }
}
