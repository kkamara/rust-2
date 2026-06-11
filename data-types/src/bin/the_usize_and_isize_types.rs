fn main() {
    // usize: unsigned integer type, used for indexing and pointer offsets
    // isize: signed integer type, used for indexing and pointer offsets
    let days: usize = 55;
    let years: isize = -15_000;
    // usize and isize are architecture-dependent, they can be 32 or 64 bits depending on the platform
    println!("Days: {}, Years: {}", days, years);
}
