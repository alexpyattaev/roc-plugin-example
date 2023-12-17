

fn main() {
    println!("Rust main is starting");
    std::process::exit(host::rust_main() as _);
    println!("Rust main is done");
}
