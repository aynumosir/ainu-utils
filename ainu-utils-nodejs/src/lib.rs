#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use ainu_utils;

#[napi]
pub fn segment(input: String) -> Vec<String> {
    ainu_utils::segmenter::segment(&input)
}
