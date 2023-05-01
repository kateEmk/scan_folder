#![feature(iter_advance_by)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
extern crate core;

pub mod utils;
pub mod scanning;

pub mod lib {
    pub use crate::utils::asserts::*;
    pub use crate::utils::errors::*;

    pub use crate::scanning::parsing::*;
    pub use crate::scanning::compare_logic::*;
}