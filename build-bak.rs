fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let cross_lib_dir = std::env::var_os("PYO3_CROSS_LIB_DIR")
            .expect("PYO3_CROSS_LIB_DIR is not set when cross-compiling");
        let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
        let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

        let libdir = std::path::Path::new(&cross_lib_dir);
        python3_dll_a::generate_implib_for_target(libdir, &arch, &env)
            .expect("python3.dll import library generator failed");
    }
}