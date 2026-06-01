fn main() {
    let apples = 5;
    let oranges = 14 + 6;
    let fruits = apples + oranges;
    println!("This year, I have {apples} apples and {oranges} oranges.");
    println!(
        "This year, I have {0} apples and {1} oranges. I can't believe I have {0} apples.",
        apples, oranges
    );
}
