use std::ffi::CString;
use yareio::spirit;
use yareio::player;

/// Your own Spirit struct with all the information you want.
struct Spirit {
    index: usize,
    alive: bool,
    friendly: bool,
}

impl Spirit {
    /// Safe wrapper for moving a spirit.
    fn goto(&self, x: f32, y: f32) {
        unsafe { spirit::goto(self.index, x, y) }
    }

    /// Safe wrapper for using shout.
    fn shout(&self, string: &str) {
        let c_string = CString::new(string).unwrap();
        unsafe { spirit::shout(self.index, c_string.as_ptr()) }
    }
}

/// Parse all spirits into your own Spirit structs.
fn get_spirits() -> Vec<Spirit> {
    unsafe {
        let me: usize = player::me();
        let count = spirit::count();
        let mut spirits = Vec::with_capacity(count);
        for index in 0..count {
            spirits.push(Spirit {
                index,
                alive: spirit::hp(index) > 0,
                friendly: spirit::player_id(index) == me,
            });
        }
        spirits
    }
}

// No unsafe block needed here!
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