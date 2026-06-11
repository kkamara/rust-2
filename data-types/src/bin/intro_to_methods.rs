fn main() {
    let val: i32 = -15;
    println!("{}", val.abs()); // abs is the distance of a number from 0, so it will return 15

    let empty_space = "        my content       ";
    println!("{}", empty_space.trim());

    println!("{}", val.pow(2)); // pow is the power function, so it will return 225
    println!("{}", val.pow(3)); // pow is the power function, so it will return -3375
}
