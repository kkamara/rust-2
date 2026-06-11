fn main() {
    let purchased_ticket = false;
    let plane_on_time = false;
    // If the first condition is false, the second condition is not evaluated
    // because the result is already known to be false.
    let making_event = purchased_ticket && plane_on_time;
    println!("It is {} that I will arrive as expected", making_event);
}
