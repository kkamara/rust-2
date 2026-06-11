fn main() {
    let one_million = 1_000_000;
    let credit_card_number: i64 = 1234_5678_9012_3456;
    let social_security_number = 999_99_9999;
    let hex_bytes: i64 = 0xFF_EC_DE_5E;
    let bytes: i64 = 0b1111_1111_1001_1100_1101_1110_0101_1110;

    println!("One million: {}", one_million);
    println!("Credit card number: {}", credit_card_number);
    println!("Social security number: {}", social_security_number);
    println!("Hex bytes: {:#X}", hex_bytes);
    println!("Bytes: {:#b}", bytes);
}
