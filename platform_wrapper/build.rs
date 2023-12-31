fn main() {
    let src_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    #[cfg(not(windows))]
    println!("cargo:rustc-link-lib=static=app");

    #[cfg(windows)]
    println!("cargo:rustc-link-lib=static=libapp");

    println!("cargo:rustc-link-search={}",src_dir);
//    println!("cargo:rustc-link-search=all=.");
}
