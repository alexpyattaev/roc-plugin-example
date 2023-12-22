mod roc_ffi;

use libloading::{Library, Symbol};
use roc_app::*;
//use roc_ffi::force_export;
use roc_std::RocStr;

const ROC_MAIN_MAGIC:&[u8]=b"roc__mainForHost_1_exposed_generic";
fn load_plugin(name: &str) {
    unsafe {
        // roc app built into a dylib using `roc build --lib color.roc`
        let lib = Library::new(name).unwrap();

        println!("Library loaded");

        // I expect the following signature based on what is in roc glue

        //type mainforhost_fn =
        //    extern "C" fn(*mut RGBA, &mut core::mem::ManuallyDrop<roc_std::RocStr>);
        type MainForHost = extern "C" fn(*mut EngineCallins,  u64);

        let func: Symbol<MainForHost> = lib.get(ROC_MAIN_MAGIC).unwrap();

        println!("Function extracted");
        // can I now simply call roc?? is it that easy
        let mut ret = core::mem::MaybeUninit::<EngineCallins>::uninit();
        //let arg = RocStr::from("BACKGROUND_COLOR");
        println!("Time to call!");
        func(ret.as_mut_ptr(), 0);
        println!("Call successful!");
        let ret = ret.assume_init();
        dbg!(ret);
    }
}

fn main() {
    //force_export();

    let mut args = std::env::args().skip(1);
    let dllname = args
        .next()
        .expect("Expeceted absolute path to library to load");

    println!("Will load {}", dllname);

    load_plugin(&dllname);
    let rs = RocStr::from("Hello World exaple works sorta");
    dbg!(rs);
}
