// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `roc glue` CLI command

#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]


#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, )]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}



#[repr(C)]
#[derive(Debug)]
pub struct RocFunction_bounce {
    closure_data: Vec<u8>,
}

impl RocFunction_bounce {
    pub fn force_thunk(mut self, arg0: Vec2, arg1: Vec2) -> Vec2 {
        extern "C" {
            fn roc__mainForHost_0_caller(arg0: *const Vec2, arg1: *const Vec2, closure_data: *mut u8, output: *mut Vec2);
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_0_caller(&arg0, &arg1, self.closure_data.as_mut_ptr(), output.as_mut_ptr());

            output.assume_init()
        }
    }
}#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, )]
#[repr(u8)]
pub enum ColoredThings {
    Background = 0,
    Ball = 1,
    Brick = 2,
    Paddle = 3,
    Wall = 4,
}

impl core::fmt::Debug for ColoredThings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Background => f.write_str("ColoredThings::Background"),
            Self::Ball => f.write_str("ColoredThings::Ball"),
            Self::Brick => f.write_str("ColoredThings::Brick"),
            Self::Paddle => f.write_str("ColoredThings::Paddle"),
            Self::Wall => f.write_str("ColoredThings::Wall"),
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, )]
#[repr(C)]
pub struct RGBA {
    pub a: u8,
    pub b: u8,
    pub g: u8,
    pub r: u8,
}



#[repr(C)]
#[derive(Debug)]
pub struct RocFunction_colors {
    closure_data: Vec<u8>,
}

impl RocFunction_colors {
    pub fn force_thunk(mut self, arg0: ColoredThings) -> RGBA {
        extern "C" {
            fn roc__mainForHost_1_caller(arg0: *const ColoredThings, closure_data: *mut u8, output: *mut RGBA);
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_1_caller(&arg0, self.closure_data.as_mut_ptr(), output.as_mut_ptr());

            output.assume_init()
        }
    }
}#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, )]
#[repr(transparent)]
pub struct PluginState {
    pub placeholder: u64,
}



#[repr(C)]
#[derive(Debug)]
pub struct RocFunction_reset {
    closure_data: Vec<u8>,
}

impl RocFunction_reset {
    pub fn force_thunk(mut self, arg0: u64) -> PluginState {
        extern "C" {
            fn roc__mainForHost_2_caller(arg0: *const u64, closure_data: *mut u8, output: *mut PluginState);
        }

        let mut output = core::mem::MaybeUninit::uninit();

        unsafe {
            roc__mainForHost_2_caller(&arg0, self.closure_data.as_mut_ptr(), output.as_mut_ptr());

            output.assume_init()
        }
    }
}
#[derive(Debug, )]
#[repr(C)]
pub struct EngineCallins {
    pub bounce: RocFunction_bounce,
    pub colors: RocFunction_colors,
    pub reset: RocFunction_reset,
}



pub fn mainForHost(arg0: u64) -> EngineCallins {
    
    extern "C" {
        fn roc__mainForHost_1_exposed_size() -> isize;
        fn roc__mainForHost_1_exposed_generic(_: *mut u8, _: u64);
        fn roc__mainForHost_0_size() -> isize;
        fn roc__mainForHost_1_size() -> isize;
        fn roc__mainForHost_2_size() -> isize;
        
    }
    //figure out size of captures 
    let size = unsafe { roc__mainForHost_1_exposed_size() } as usize;
    let mut captures = Vec::with_capacity(size);
    captures.resize(size, 0);

    unsafe {
        roc__mainForHost_1_exposed_generic(captures.as_mut_ptr(), arg0);    
    }
    dbg!(&captures);
    let bounce_size = unsafe { roc__mainForHost_0_size() } as usize;
    let colors_size = unsafe { roc__mainForHost_1_size() } as usize;
    let reset_size = unsafe { roc__mainForHost_2_size() } as usize;
    dbg!(bounce_size);
    dbg!(colors_size);
    dbg!(reset_size);
    let mut ret = EngineCallins {
        bounce: RocFunction_bounce {
            closure_data: Vec::with_capacity(bounce_size),
        },
        colors: RocFunction_colors {
            closure_data: Vec::with_capacity(colors_size),
        },
        reset: RocFunction_reset {
            closure_data: Vec::with_capacity(reset_size),
        },
    };
    //let mut ret = core::mem::MaybeUninit::uninit();

    let mut data_slice = captures.as_slice();
    ret.bounce.closure_data.extend(&data_slice[..bounce_size]);
    data_slice = &data_slice[bounce_size..];
    ret.colors.closure_data.extend(&data_slice[..colors_size]);
    data_slice = &data_slice[colors_size..];
    ret.reset.closure_data.extend(&data_slice[..reset_size]);
    
   ret
}
