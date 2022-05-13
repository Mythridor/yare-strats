#![allow(clashing_extern_declarations)]

use std::ffi::CString;
struct Spirit {
    index: usize,
    alive: bool,
    friendly: bool,
}

impl Spirit {
    fn goto(&self, x: f32, y: f32) {
        unsafe { bindings::spirit::goto(self.index, x, y) }
    }

    fn shout(&self, string: &str) {
        let c_string = CString::new(string).unwrap();
        unsafe { bindings::spirit::shout(self.index, c_string.as_ptr()) }
    }
}

fn get_spirits() -> Vec<Spirit> {
    unsafe {
        let me = bindings::player::me();
        let count = bindings::spirit::count();
        let mut spirits = Vec::with_capacity(count);
        for index in 0..count {
            spirits.push(Spirit {
                index,
                alive: bindings::spirit::hp(index) > 0,
                friendly: bindings::spirit::id(index).number == me,
            });
        }
        spirits
    }
}

#[no_mangle]
pub extern "C" fn tick(_tick: u32) {
    let all_spirits = get_spirits();
    for spirit in all_spirits {
        if spirit.friendly && spirit.alive {
            spirit.goto(123., 456.);
            spirit.shout("Hello, world!");
        }
    }
}

#[cfg(not(feature = "headless"))]
pub mod bindings;

#[cfg(feature = "headless")]
pub mod headless;


