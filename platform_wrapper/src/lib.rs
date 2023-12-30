

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    let callins = roc_app::mainForHost(42);
    dbg!(callins);
    
    0 // Exit code
}

#[no_mangle]
pub extern "C" fn callin_number_one() -> i32 {
    42
}
