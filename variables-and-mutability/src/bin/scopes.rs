fn main() {
    let coffee_price = 5.99;

    {
        let cookie_price = 1.99;
        println!("The price of a coffee is: ${}", coffee_price);
    }
    
    println!("The price of a coffee is: ${}", coffee_price);
}
