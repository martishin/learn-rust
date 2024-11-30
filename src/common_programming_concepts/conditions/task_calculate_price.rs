pub fn main() {
    println!("{}", calculate_price(20));
}

fn calculate_price(amount: i32) -> i32 {
    if amount >= 40 {
        amount
    } else {
        amount * 2
    }
}
