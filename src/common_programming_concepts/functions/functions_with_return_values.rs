fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn main() {
    let x = five();

    println!("The value if x is: {}", x);

    let y = plus_one(5);

    println!("The value if y is: {}", y);
}
