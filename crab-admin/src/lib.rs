#![forbid(unsafe_code)]

use rbatis::rbatis::Rbatis;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

lazy_static! {
    pub static ref RB: Rbatis = Rbatis::new();
}
