// Force disable aws-lc-sys for Windows compatibility
// This prevents C11 compilation issues on Windows

fn main() {
    // Disable aws-lc-sys compilation on Windows
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-env=AWS_LC_SYS_NO_PREFIX=1");
        println!("cargo:rustc-env=AWS_LC_SYS_STATIC=0");
    }
}
