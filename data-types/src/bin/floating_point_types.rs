fn main() {
    let pi: f64 = 3.1415926535897932384;
    println!("Pi is approximately: {}", pi); // pi rounds at "793" due to the lack of precision of f64

    println!("{}", pi.floor());
    println!("{}", pi.ceil());
    println!("{}", pi.round());
}
