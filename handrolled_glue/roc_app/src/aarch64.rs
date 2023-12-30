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
pub struct RocFunction_92 {
    closure_data: Vec<u8>,
}

impl RocFunction_92 {
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
pub struct RocFunction_91 {
    closure_data: Vec<u8>,
}

impl RocFunction_91 {
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
pub struct RocFunction_93 {
    closure_data: Vec<u8>,
}

impl RocFunction_93 {
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
}#[derive(Clone, Debug, )]
#[repr(C)]
pub struct EngineCallins {
    pub bounce: RocFunction_92,
    pub colors: RocFunction_91,
    pub reset: RocFunction_93,
}



pub fn mainForHost(arg0: u64) -> EngineCallins {
    panic!("I've only fixed the x86_64 version");
    extern "C" {
        fn roc__mainForHost_1_exposed_generic(_: *mut EngineCallins, _: u64);
    }

    let mut ret = core::mem::MaybeUninit::uninit();

    unsafe {
        roc__mainForHost_1_exposed_generic(ret.as_mut_ptr(), arg0);

        ret.assume_init()
    }
}
