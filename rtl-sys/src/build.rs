pub fn main() {
    // if we're on Mac OS X we'll kindly add DYLD_FALLBACK_LIBRARY_PATH to rustc's
    // linker search path
    if let Some(dyld_fallback_paths) = option_env!("DYLD_FALLBACK_LIBRARY_PATH") {
        for path in dyld_fallback_paths.split(':') {
            println!("cargo:rustc-link-search=native={}", path)
        }
    }

    println!("cargo:rustc-link-search=native=/opt/local/lib");
}
