#![allow(unused_variables)] // apply to whole crate
#![allow(dead_code)] // apply to whole crate
mod helper;

type Meters = i32;

// #[allow(unused_variables)] // apply to whole function
fn main() {
    // #[allow(unused_variables)] // apply to this variable only
    let mile_race_length: Meters = 16_00;
    // #[allow(unused_variables)]
    let two_mile_race_length: Meters = 32_00;
}
