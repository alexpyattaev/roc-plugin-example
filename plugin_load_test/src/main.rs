mod roc_ffi;

use libloading::{library_filename, Library, Symbol};
use roc_std::RocStr;

fn load_plugin() {
    unsafe {
        // roc app built into a dylib using `roc build --lib color.roc`
        let lib = Library::new(library_filename("colors")).unwrap();

        // I expect the following signature
        // pub unsafe extern "C" fn roc__mainForHost_1_exposed_generic(*RocStr, *(u8,u8,u8,u8)) { ... etc }
        let func: Symbol<fn(*const RocStr) -> *const (u8, u8, u8, u8)> =
            lib.get(b"roc__mainForHost_1_exposed_generic").unwrap();

        // can I now simply call roc?? is it that easy
        let rgba: *const (u8, u8, u8, u8) = func(&RocStr::from("Hello World"));
        dbg!(rgba);
    }
}
fn main() {
    println!("Hello, world!");
    let rs = RocStr::from("Hello World");
    load_plugin();
    dbg!(rs);
}
