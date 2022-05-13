
#[cfg(feature = "headless")]
mod headless {
    #![allow(missing_docs)]
    #![allow(clippy::missing_safety_doc)]
    pub mod base;
    pub mod console;
    pub use crate::bindings::game;
    pub mod graphics;
    pub use crate::bindings::id;
    pub mod outpost;
    pub mod player;
    pub mod spirit;
    pub use crate::bindings::position;
    pub mod random;
    pub mod star;
    pub mod yare_impl;
}